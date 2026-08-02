//! Persistent save foundation for the Bevy runtime.
//!
//! This slice follows OpenMW's ownership model: the save header describes the
//! content set, cell state is keyed by stable FormIDs, and reference changes
//! are stored as deltas. Runtime ECS capture/application will consume this
//! model in a later slice.

use anyhow::{Context, Result, bail};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};

use crate::item_transaction::{
    HolderId, ItemHolderState, ItemInstance, ItemInstanceId, ItemLedgerSnapshot, ItemState,
    TransactionId,
};
use bevyout_core::actor_state::{ActorInstanceState, ActorLifeState, ActorPackageCheckpoint};
use bevyout_core::dialogue::DialogueSnapshot;
use bevyout_core::manifest::exterior::WorldLocation;

mod openmw;

use openmw::{read_records, read_subrecords, tag, write_record, write_subrecord};

pub const CURRENT_SAVE_FORMAT_VERSION: u32 = 7;
pub const MIN_SUPPORTED_SAVE_FORMAT_VERSION: u32 = 1;

#[derive(Debug, Clone)]
pub struct SaveGame {
    pub header: SaveGameHeader,
    pub world: PersistentWorldState,
    /// Optional player record (issue #60, F60.4): `None` round-trips as an
    /// absent PLYR record, so saves written before this record existed --
    /// and readers older than it (unknown records are skipped, see the
    /// forward-compatibility test) -- stay compatible in both directions.
    pub player: Option<PlayerState>,
    pub next_runtime_item_id: u64,
    pub rng_state: u64,
    /// Canonical M3 item-holder state. `None` is retained for v1/v2 callers;
    /// decoding or encoding a legacy save deterministically migrates it.
    pub canonical: Option<ItemLedgerSnapshot>,
    /// Boundary-only narrative state; no Yarn VM or Bevy entity is saved.
    pub dialogue: DialogueSnapshot,
    /// Exact player location, including exterior worldspace or interior cell.
    /// Format v7 writers emit this as a separate WLOC record; older saves
    /// decode with `None` and expose only the identity-only header-cell
    /// fallback through [`SaveGame::legacy_location_fallback_cell`].
    pub location: Option<WorldLocation>,
}

impl PartialEq for SaveGame {
    fn eq(&self, other: &Self) -> bool {
        self.header == other.header
            && self.world == other.world
            && self.player == other.player
            && self.next_runtime_item_id == other.next_runtime_item_id
            && self.rng_state == other.rng_state
            && self.dialogue == other.dialogue
            && self.location == other.location
            && canonical_for_compare(self) == canonical_for_compare(other)
    }
}

fn canonical_for_compare(save: &SaveGame) -> Option<ItemLedgerSnapshot> {
    save.canonical.clone().or_else(|| migrate_legacy(save).ok())
}

impl Default for SaveGame {
    fn default() -> Self {
        Self {
            header: SaveGameHeader::default(),
            world: PersistentWorldState::default(),
            player: None,
            next_runtime_item_id: 1,
            rng_state: 0,
            canonical: None,
            dialogue: DialogueSnapshot::default(),
            location: None,
        }
    }
}

/// Persistent player state (issue #60, F60.4; equipment/hotkeys added by
/// issue #98, F98.4, format v3).
#[derive(Debug, Clone, Default, PartialEq)]
pub struct PlayerState {
    pub inventory: Vec<ItemStack>,
    /// The equipped set (issue #98). Absent (empty) on saves written before
    /// format v3 -- `equipitem`/Pip-Boy equip state simply didn't exist yet.
    /// Sorted by `(kind, base_form_id, condition)`, see `validate_equipped`.
    pub equipped: Vec<EquippedItem>,
    /// Hotkey digit 1-8 bindings (issue #98), each optionally bound to a
    /// carried stack. Absent (all `None`) on saves written before format v3.
    pub hotkeys: [Option<HotkeyBinding>; 8],
}

/// Which equip slot an `EquippedItem` occupies. Carries no slot-specific
/// data (a biped-slot mask, a required ammo form id) -- that is catalog data
/// re-derived at load time via `player::equipment`'s `equip`, exactly the
/// way picking an item up re-derives its condition from the catalog.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum EquippedKind {
    Apparel,
    Weapon,
    Ammo,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EquippedItem {
    pub kind: EquippedKind,
    pub base_form_id: u32,
    pub condition: Option<u32>,
}

/// A hotkey digit's bound stack identity (issue #98). No `kind` needed --
/// pressing the hotkey re-resolves the category from the catalog exactly
/// like an equip toggle from the Pip-Boy would.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HotkeyBinding {
    pub base_form_id: u32,
    pub condition: Option<u32>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SaveGameHeader {
    pub format_version: u32,
    pub content_fingerprint: String,
    pub plugins: Vec<SavePlugin>,
    pub current_cell: u32,
    pub play_time_seconds: f64,
    pub description: String,
}

impl Default for SaveGameHeader {
    fn default() -> Self {
        Self {
            format_version: CURRENT_SAVE_FORMAT_VERSION,
            content_fingerprint: String::new(),
            plugins: Vec::new(),
            current_cell: 0,
            play_time_seconds: 0.0,
            description: String::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SavePlugin {
    pub name: String,
    pub fingerprint: String,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PersistentWorldState {
    pub cells: BTreeMap<u32, PersistentCellState>,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PersistentCellState {
    pub references: BTreeMap<u32, PersistentReferenceDelta>,
    pub dropped_items: BTreeMap<u64, DroppedItemState>,
    /// Format v4 actor mutations keyed by stable ACHR/ACRE reference FormID.
    /// Inventory/equipment remain in the canonical item ledger and transforms/
    /// enabled state remain in `references`.
    pub actors: BTreeMap<u32, ActorInstanceState>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DroppedItemState {
    pub runtime_id: u64,
    pub stack: ItemStack,
    pub transform: SavedTransform,
    pub body: SavedBodyState,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PersistentReferenceDelta {
    pub enabled: Option<bool>,
    pub deleted: bool,
    pub activated: Option<bool>,
    pub lock_level: Option<i8>,
    pub enable_root_form_id: Option<u32>,
    pub transform: Option<SavedTransform>,
    pub inventory: Option<Vec<ItemStack>>,
    /// Issue #76 (F76.1): marks that this container reference's first-open
    /// leveled-list roll already happened, so an emptied container (`Some`
    /// `inventory` with no stacks, or no `inventory` override at all when the
    /// roll produced nothing) is distinguishable from one that was never
    /// opened -- which still rolls on first activation. Rides alongside
    /// `inventory` as an independent optional subrecord (`OBJE.LVLR`); a v1
    /// save that predates this field simply never wrote it, so it decodes as
    /// `None` exactly like `body`/`transform` do for older saves.
    pub leveled_resolved: Option<bool>,
    pub body: Option<SavedBodyState>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SavedTransform {
    pub translation: [f32; 3],
    pub rotation_xyzw: [f32; 4],
    pub scale: [f32; 3],
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SavedBodyState {
    pub linear_velocity: [f32; 3],
    pub angular_velocity: [f32; 3],
    pub sleeping: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ItemStack {
    pub base_form_id: u32,
    pub count: i32,
    pub condition: Option<u32>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SaveSlotSource {
    Primary,
    Backup,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SaveLoadOutcome {
    pub save: SaveGame,
    pub source: SaveSlotSource,
    pub warning: Option<String>,
}

pub fn encode_save(save: &SaveGame) -> Result<Vec<u8>> {
    let mut save = save.clone();
    if save.header.format_version >= 3 && save.canonical.is_none() {
        save.canonical = Some(migrate_legacy(&save)?);
    }
    validate_save(&save)?;
    let mut bytes = Vec::new();
    write_record(&mut bytes, tag("SAVE"), &encode_header(&save.header)?)?;
    if let Some(player) = &save.player {
        write_record(
            &mut bytes,
            tag("PLYR"),
            &encode_player(player, save.header.format_version)?,
        )?;
    }
    if save.header.format_version >= 2 {
        write_record(
            &mut bytes,
            tag("NITM"),
            &save.next_runtime_item_id.to_le_bytes(),
        )?;
    }
    if save.header.format_version >= 3 {
        let canonical = save
            .canonical
            .as_ref()
            .context("v3+ save is missing canonical item state")?;
        write_record(
            &mut bytes,
            tag("ITMS"),
            ron::ser::to_string(canonical)
                .context("encoding canonical item state")?
                .as_bytes(),
        )?;
    }
    if save.header.format_version >= 6 {
        write_record(
            &mut bytes,
            tag("DLOG"),
            ron::ser::to_string(&save.dialogue)
                .context("encoding dialogue snapshot")?
                .as_bytes(),
        )?;
    }
    if save.header.format_version >= 7
        && let Some(location) = &save.location
    {
        write_record(
            &mut bytes,
            tag("WLOC"),
            ron::ser::to_string(location)
                .context("encoding player world location")?
                .as_bytes(),
        )?;
    }
    for (cell_form_id, cell) in &save.world.cells {
        let mut payload = Vec::new();
        write_subrecord(&mut payload, tag("FORM"), &cell_form_id.to_le_bytes())?;
        write_record(&mut bytes, tag("CSTA"), &payload)?;
        for (reference_form_id, delta) in &cell.references {
            write_record(
                &mut bytes,
                tag("OBJE"),
                &encode_reference(
                    *cell_form_id,
                    *reference_form_id,
                    delta,
                    save.header.format_version,
                )?,
            )?;
        }
        if save.header.format_version >= 4 {
            for (reference_form_id, actor) in &cell.actors {
                write_record(
                    &mut bytes,
                    tag("ACTR"),
                    &encode_actor(*cell_form_id, *reference_form_id, actor)?,
                )?;
            }
        }
        if save.header.format_version >= 2 {
            for dropped in cell.dropped_items.values() {
                write_record(
                    &mut bytes,
                    tag("DROP"),
                    &encode_dropped(*cell_form_id, dropped)?,
                )?;
            }
        }
    }
    write_record(&mut bytes, tag("RAND"), &save.rng_state.to_le_bytes())?;
    let checksum: [u8; 32] = Sha256::digest(&bytes).into();
    write_record(&mut bytes, tag("CHKS"), &checksum)?;
    Ok(bytes)
}

pub fn decode_save(bytes: &[u8]) -> Result<SaveGame> {
    let records = read_records(bytes).context("reading save record stream")?;
    let mut save = SaveGame::default();
    let mut saw_header = false;
    let mut saw_rng = false;
    let mut saw_next_runtime_item = false;
    let mut saw_canonical = false;
    let mut saw_dialogue = false;
    let mut saw_location = false;
    let mut checksum = None;
    let mut checksum_offset = None;
    let mut offset = 0usize;

    for record in records {
        if record.tag == tag("CHKS") {
            if checksum.is_some() {
                bail!("save contains duplicate CHKS records");
            }
            if record.payload.len() != 32 {
                bail!("save CHKS record must contain exactly 32 bytes");
            }
            let expected = Sha256::digest(&bytes[..offset]);
            checksum_offset = Some(offset);
            checksum = Some((record.payload, expected.to_vec()));
        } else {
            if checksum.is_some() {
                bail!("save records appear after the CHKS record");
            }
            match &record.tag {
                record_tag if *record_tag == tag("SAVE") => {
                    if saw_header {
                        bail!("save contains duplicate SAVE records");
                    }
                    save.header = decode_header(&record.payload)?;
                    saw_header = true;
                }
                record_tag if *record_tag == tag("PLYR") => {
                    if save.player.is_some() {
                        bail!("save contains duplicate PLYR records");
                    }
                    if !saw_header {
                        bail!("PLYR appears before the SAVE header");
                    }
                    save.player = Some(decode_player(&record.payload, save.header.format_version)?);
                }
                record_tag if *record_tag == tag("NITM") => {
                    if saw_next_runtime_item {
                        bail!("save contains duplicate NITM records");
                    }
                    if record.payload.len() != 8 {
                        bail!("NITM must contain eight bytes");
                    }
                    save.next_runtime_item_id = u64::from_le_bytes(
                        record.payload.as_slice().try_into().expect("checked NITM"),
                    );
                    saw_next_runtime_item = true;
                }
                record_tag if *record_tag == tag("ITMS") => {
                    if saw_canonical {
                        bail!("save contains duplicate ITMS records");
                    }
                    if !saw_header || save.header.format_version < 3 {
                        bail!("ITMS is only valid in save format v3 or newer");
                    }
                    save.canonical = Some(
                        ron::de::from_bytes(&record.payload)
                            .context("decoding canonical item state")?,
                    );
                    saw_canonical = true;
                }
                record_tag if *record_tag == tag("DLOG") => {
                    if saw_dialogue {
                        bail!("save contains duplicate DLOG records");
                    }
                    if !saw_header || save.header.format_version < 6 {
                        bail!("DLOG is only valid in save format v6 or newer");
                    }
                    save.dialogue = ron::de::from_bytes(&record.payload)
                        .context("decoding dialogue snapshot")?;
                    saw_dialogue = true;
                }
                record_tag if *record_tag == tag("WLOC") => {
                    if saw_location {
                        bail!("save contains duplicate WLOC records");
                    }
                    if !saw_header || save.header.format_version < 7 {
                        bail!("WLOC is only valid in save format v7 or newer");
                    }
                    save.location = Some(
                        ron::de::from_bytes(&record.payload)
                            .context("decoding player world location")?,
                    );
                    saw_location = true;
                }
                record_tag if *record_tag == tag("CSTA") => {
                    let cell_form_id = decode_cell_state(&record.payload)?;
                    if save
                        .world
                        .cells
                        .insert(cell_form_id, PersistentCellState::default())
                        .is_some()
                    {
                        bail!("save contains duplicate CSTA for cell {cell_form_id:08x}");
                    }
                }
                record_tag if *record_tag == tag("OBJE") => {
                    let (cell_form_id, reference_form_id, delta) =
                        decode_reference(&record.payload, save.header.format_version)?;
                    save.world
                        .cells
                        .entry(cell_form_id)
                        .or_default()
                        .references
                        .insert_checked(reference_form_id, delta)?;
                }
                record_tag if *record_tag == tag("ACTR") => {
                    if !saw_header || save.header.format_version < 4 {
                        bail!("ACTR is only valid in save format v4 or newer");
                    }
                    let (cell_form_id, reference_form_id, actor) = decode_actor(&record.payload)?;
                    if save
                        .world
                        .cells
                        .entry(cell_form_id)
                        .or_default()
                        .actors
                        .insert(reference_form_id, actor)
                        .is_some()
                    {
                        bail!(
                            "save contains duplicate ACTR state for reference {reference_form_id:08x}"
                        );
                    }
                }
                record_tag if *record_tag == tag("DROP") => {
                    let (cell_form_id, dropped) = decode_dropped(&record.payload)?;
                    if save
                        .world
                        .cells
                        .entry(cell_form_id)
                        .or_default()
                        .dropped_items
                        .insert(dropped.runtime_id, dropped)
                        .is_some()
                    {
                        bail!("save contains duplicate dropped runtime item id");
                    }
                }
                record_tag if *record_tag == tag("RAND") => {
                    if saw_rng || record.payload.len() != 8 {
                        bail!("save RAND record is missing, duplicated, or malformed");
                    }
                    save.rng_state = u64::from_le_bytes(
                        record
                            .payload
                            .as_slice()
                            .try_into()
                            .expect("checked RAND length"),
                    );
                    saw_rng = true;
                }
                _ => {}
            }
        }
        offset += record.raw.len();
    }

    let Some((actual, expected)) = checksum else {
        bail!("save is missing its final CHKS record");
    };
    if checksum_offset != Some(bytes.len() - (8 + 32)) {
        bail!("save CHKS record is not the final record");
    }
    if actual != expected {
        bail!("save checksum mismatch");
    }
    if !saw_header {
        bail!("save is missing its SAVE header");
    }
    if !saw_rng {
        bail!("save is missing its RAND state");
    }
    if save.header.format_version >= 2 && !saw_next_runtime_item {
        bail!("save format v2 is missing NITM");
    }
    if save.header.format_version >= 3 && !saw_canonical {
        save.canonical = Some(migrate_legacy(&save)?);
    }
    validate_save(&save)?;
    Ok(save)
}

/// Deterministic v1/v2 migration. Legacy records are already sorted by
/// holder/FormID/condition, so assigning IDs in that traversal order makes a
/// migration reproducible while preserving every legacy count and condition.
fn migrate_legacy(save: &SaveGame) -> Result<ItemLedgerSnapshot> {
    let mut snapshot = ItemLedgerSnapshot {
        next_item_id: ItemInstanceId(1),
        next_transaction_id: TransactionId(1),
        ..Default::default()
    };
    let mut next_id = 1u64;
    let mut add_stack = |items: &mut Vec<ItemInstance>, stack: &ItemStack| -> Result<()> {
        if stack.count <= 0 {
            bail!("legacy item count must be positive during v3 migration");
        }
        items.push(ItemInstance::new(
            ItemInstanceId(next_id),
            stack.base_form_id,
            u32::try_from(stack.count).context("legacy item count exceeds v3 range")?,
            ItemState {
                condition: stack.condition,
                ..Default::default()
            },
        )?);
        next_id = next_id.saturating_add(1);
        Ok(())
    };

    if let Some(player) = &save.player {
        let mut state = ItemHolderState::default();
        for stack in &player.inventory {
            add_stack(&mut state.items, stack)?;
        }
        snapshot.holders.insert(HolderId::Player, state);
    }
    for (cell_form_id, cell) in &save.world.cells {
        for (reference_form_id, delta) in &cell.references {
            let Some(inventory) = &delta.inventory else {
                continue;
            };
            let holder = HolderId::FixtureContainer {
                reference_form_id: *reference_form_id,
            };
            if snapshot.holders.contains_key(&holder) {
                bail!("legacy migration found duplicate container holder {reference_form_id:08x}");
            }
            let mut state = ItemHolderState::default();
            for stack in inventory {
                add_stack(&mut state.items, stack)?;
            }
            snapshot.holders.insert(holder, state);
        }
        for (runtime_id, dropped) in &cell.dropped_items {
            let holder = HolderId::RuntimeWorld {
                cell_form_id: *cell_form_id,
                runtime_id: *runtime_id,
            };
            let mut state = ItemHolderState::default();
            add_stack(&mut state.items, &dropped.stack)?;
            snapshot.holders.insert(holder, state);
        }
    }
    for holder in snapshot.holders.keys().copied() {
        snapshot.bindings.entry(holder).or_default();
    }
    snapshot.next_item_id = ItemInstanceId(next_id);
    Ok(snapshot)
}

impl SaveGame {
    /// Returns the legacy header-cell fallback only when no exact WLOC was
    /// serialized. This is an identity-only migration seam: it deliberately
    /// carries no authored position or rotation and must not be treated as an
    /// exact travel anchor.
    pub fn legacy_location_fallback_cell(&self) -> Option<u32> {
        self.location.is_none().then_some(self.header.current_cell)
    }

    pub fn ensure_compatible(
        &self,
        content_fingerprint: &str,
        plugins: &[SavePlugin],
    ) -> Result<()> {
        if self.header.content_fingerprint != content_fingerprint {
            bail!(
                "save content fingerprint {} does not match loaded content {}",
                self.header.content_fingerprint,
                content_fingerprint
            );
        }
        if self.header.plugins != plugins {
            bail!("save plugin load order/fingerprints do not match the loaded content");
        }
        Ok(())
    }
}

pub struct SaveStore {
    save_dir: PathBuf,
}

impl SaveStore {
    pub fn new(project_root: impl Into<PathBuf>) -> Self {
        Self {
            save_dir: project_root.into().join(".bevyout").join("saves"),
        }
    }

    pub fn from_save_dir(save_dir: impl Into<PathBuf>) -> Self {
        Self {
            save_dir: save_dir.into(),
        }
    }

    pub fn write_slot(&self, slot: &str, save: &SaveGame) -> Result<()> {
        validate_slot(slot)?;
        let bytes = encode_save(save)?;
        decode_save(&bytes).context("validating serialized save before install")?;
        fs::create_dir_all(&self.save_dir)
            .with_context(|| format!("creating save directory {}", self.save_dir.display()))?;
        let primary = self.primary_path(slot);
        let backup = self.backup_path(slot);
        let temporary = self.temporary_path(slot);
        write_synced(&temporary, &bytes)?;
        install_with_backup(&temporary, &primary, &backup)
    }

    pub fn read_slot(&self, slot: &str) -> Result<SaveLoadOutcome> {
        validate_slot(slot)?;
        let primary = self.primary_path(slot);
        match fs::read(&primary).and_then(|bytes| decode_save(&bytes).map_err(io_error)) {
            Ok(save) => Ok(SaveLoadOutcome {
                save,
                source: SaveSlotSource::Primary,
                warning: None,
            }),
            Err(primary_error) => {
                let backup = self.backup_path(slot);
                let bytes = fs::read(&backup).with_context(|| {
                    format!(
                        "primary save {} failed ({primary_error}); backup {} is unavailable",
                        primary.display(),
                        backup.display()
                    )
                })?;
                let save = decode_save(&bytes).with_context(|| {
                    format!(
                        "primary save {} failed ({primary_error}); backup {} is invalid",
                        primary.display(),
                        backup.display()
                    )
                })?;
                Ok(SaveLoadOutcome {
                    save,
                    source: SaveSlotSource::Backup,
                    warning: Some(format!(
                        "primary save {} was unusable: {primary_error}",
                        primary.display()
                    )),
                })
            }
        }
    }

    pub fn primary_path(&self, slot: &str) -> PathBuf {
        self.save_dir.join(format!("{slot}.bevyoutsave"))
    }

    fn backup_path(&self, slot: &str) -> PathBuf {
        self.save_dir.join(format!("{slot}.bevyoutsave.bak"))
    }

    fn temporary_path(&self, slot: &str) -> PathBuf {
        self.save_dir.join(format!("{slot}.bevyoutsave.tmp"))
    }
}

fn encode_header(header: &SaveGameHeader) -> Result<Vec<u8>> {
    let mut payload = Vec::new();
    write_subrecord(
        &mut payload,
        tag("VERS"),
        &header.format_version.to_le_bytes(),
    )?;
    write_subrecord(
        &mut payload,
        tag("FPRN"),
        header.content_fingerprint.as_bytes(),
    )?;
    write_subrecord(
        &mut payload,
        tag("CELL"),
        &header.current_cell.to_le_bytes(),
    )?;
    write_subrecord(
        &mut payload,
        tag("TIME"),
        &header.play_time_seconds.to_le_bytes(),
    )?;
    write_subrecord(&mut payload, tag("DESC"), header.description.as_bytes())?;
    for plugin in &header.plugins {
        let mut plugin_payload = Vec::new();
        write_length_prefixed(&mut plugin_payload, plugin.name.as_bytes())?;
        write_length_prefixed(&mut plugin_payload, plugin.fingerprint.as_bytes())?;
        write_subrecord(&mut payload, tag("PLUG"), &plugin_payload)?;
    }
    Ok(payload)
}

fn decode_header(payload: &[u8]) -> Result<SaveGameHeader> {
    let mut header = SaveGameHeader::default();
    let mut saw_version = false;
    let mut saw_fingerprint = false;
    let mut saw_cell = false;
    let mut saw_time = false;
    for subrecord in read_subrecords(payload)? {
        match &subrecord.tag {
            record_tag if *record_tag == tag("VERS") => {
                ensure_once(&mut saw_version, "SAVE.VERS")?;
                header.format_version = read_u32(&subrecord.payload, "SAVE.VERS")?;
            }
            record_tag if *record_tag == tag("FPRN") => {
                ensure_once(&mut saw_fingerprint, "SAVE.FPRN")?;
                header.content_fingerprint = read_string(&subrecord.payload, "SAVE.FPRN")?;
            }
            record_tag if *record_tag == tag("CELL") => {
                ensure_once(&mut saw_cell, "SAVE.CELL")?;
                header.current_cell = read_u32(&subrecord.payload, "SAVE.CELL")?;
            }
            record_tag if *record_tag == tag("TIME") => {
                ensure_once(&mut saw_time, "SAVE.TIME")?;
                if subrecord.payload.len() != 8 {
                    bail!("SAVE.TIME must contain eight bytes");
                }
                header.play_time_seconds = f64::from_le_bytes(
                    subrecord
                        .payload
                        .as_slice()
                        .try_into()
                        .expect("checked TIME length"),
                );
            }
            record_tag if *record_tag == tag("DESC") => {
                header.description = read_string(&subrecord.payload, "SAVE.DESC")?;
            }
            record_tag if *record_tag == tag("PLUG") => {
                let mut cursor = 0;
                header.plugins.push(SavePlugin {
                    name: read_length_prefixed(&subrecord.payload, &mut cursor, "SAVE.PLUG.name")?,
                    fingerprint: read_length_prefixed(
                        &subrecord.payload,
                        &mut cursor,
                        "SAVE.PLUG.fingerprint",
                    )?,
                });
                if cursor != subrecord.payload.len() {
                    bail!("SAVE.PLUG has trailing bytes");
                }
            }
            _ => {}
        }
    }
    if !(saw_version && saw_fingerprint && saw_cell && saw_time) {
        bail!("SAVE header is missing a required subrecord");
    }
    Ok(header)
}

fn encode_player(player: &PlayerState, format_version: u32) -> Result<Vec<u8>> {
    let mut payload = Vec::new();
    write_subrecord(
        &mut payload,
        tag("INVT"),
        &encode_inventory_bytes(&player.inventory, format_version)?,
    )?;
    // Issue #98 (F98.4): equipment/hotkeys are new in format v3 -- v1/v2
    // writers never emit these subrecords, so older readers (which simply
    // skip unknown subrecords) and this reader's own absent-defaults path
    // both keep working.
    if format_version >= 3 {
        write_subrecord(
            &mut payload,
            tag("EQIP"),
            &encode_equipped(&player.equipped),
        )?;
        write_subrecord(&mut payload, tag("HOTK"), &encode_hotkeys(&player.hotkeys))?;
    }
    Ok(payload)
}

fn decode_player(payload: &[u8], format_version: u32) -> Result<PlayerState> {
    let mut player = PlayerState::default();
    let mut saw_inventory = false;
    for subrecord in read_subrecords(payload)? {
        if subrecord.tag == tag("INVT") {
            ensure_once(&mut saw_inventory, "PLYR.INVT")?;
            player.inventory = decode_inventory(&subrecord.payload, format_version)?;
        } else if subrecord.tag == tag("EQIP") {
            player.equipped = decode_equipped(&subrecord.payload)?;
        } else if subrecord.tag == tag("HOTK") {
            player.hotkeys = decode_hotkeys(&subrecord.payload)?;
        }
    }
    if !saw_inventory {
        bail!("PLYR record is missing its INVT subrecord");
    }
    Ok(player)
}

fn encode_equipped(equipped: &[EquippedItem]) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(4 + equipped.len() * 10);
    bytes.extend_from_slice(&(equipped.len() as u32).to_le_bytes());
    for item in equipped {
        bytes.push(match item.kind {
            EquippedKind::Apparel => 0,
            EquippedKind::Weapon => 1,
            EquippedKind::Ammo => 2,
        });
        bytes.extend_from_slice(&item.base_form_id.to_le_bytes());
        bytes.push(item.condition.is_some() as u8);
        bytes.extend_from_slice(&item.condition.unwrap_or_default().to_le_bytes());
    }
    bytes
}

fn decode_equipped(bytes: &[u8]) -> Result<Vec<EquippedItem>> {
    if bytes.len() < 4 {
        bail!("PLYR.EQIP is truncated");
    }
    let count = read_u32(&bytes[..4], "PLYR.EQIP")? as usize;
    let mut offset = 4usize;
    let mut equipped = Vec::with_capacity(count);
    for _ in 0..count {
        let entry = bytes
            .get(offset..offset + 10)
            .context("PLYR.EQIP entry is truncated")?;
        let kind = match entry[0] {
            0 => EquippedKind::Apparel,
            1 => EquippedKind::Weapon,
            2 => EquippedKind::Ammo,
            other => bail!("PLYR.EQIP has an unknown kind tag {other}"),
        };
        let base_form_id = read_u32(&entry[1..5], "PLYR.EQIP")?;
        let condition = read_optional_condition(entry[5], &entry[6..10], "PLYR.EQIP")?;
        equipped.push(EquippedItem {
            kind,
            base_form_id,
            condition,
        });
        offset += 10;
    }
    if offset != bytes.len() {
        bail!("PLYR.EQIP has trailing bytes");
    }
    validate_equipped(&equipped)?;
    Ok(equipped)
}

fn encode_hotkeys(hotkeys: &[Option<HotkeyBinding>; 8]) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(8 * 10);
    for slot in hotkeys {
        match slot {
            Some(binding) => {
                bytes.push(1);
                bytes.extend_from_slice(&binding.base_form_id.to_le_bytes());
                bytes.push(binding.condition.is_some() as u8);
                bytes.extend_from_slice(&binding.condition.unwrap_or_default().to_le_bytes());
            }
            None => bytes.extend_from_slice(&[0u8; 10]),
        }
    }
    bytes
}

fn decode_hotkeys(bytes: &[u8]) -> Result<[Option<HotkeyBinding>; 8]> {
    if bytes.len() != 80 {
        bail!("PLYR.HOTK must contain 80 bytes");
    }
    let mut hotkeys: [Option<HotkeyBinding>; 8] = Default::default();
    for (index, slot) in hotkeys.iter_mut().enumerate() {
        let entry = &bytes[index * 10..index * 10 + 10];
        if entry[0] > 1 {
            bail!("PLYR.HOTK present flag must be 0 or 1");
        }
        if entry[0] == 1 {
            let base_form_id = read_u32(&entry[1..5], "PLYR.HOTK")?;
            let condition = read_optional_condition(entry[5], &entry[6..10], "PLYR.HOTK")?;
            *slot = Some(HotkeyBinding {
                base_form_id,
                condition,
            });
        }
    }
    Ok(hotkeys)
}

/// Shared by `decode_equipped`/`decode_hotkeys`: a one-byte "has condition"
/// flag followed by a four-byte condition value, exactly `ItemStack`'s v2+
/// condition layout (see `encode_inventory_bytes`).
fn read_optional_condition(has_condition: u8, bytes: &[u8], label: &str) -> Result<Option<u32>> {
    match has_condition {
        0 => Ok(None),
        1 => Ok(Some(read_u32(bytes, label)?)),
        other => bail!("{label} condition flag must be 0 or 1, got {other}"),
    }
}

fn encode_inventory_bytes(inventory: &[ItemStack], format_version: u32) -> Result<Vec<u8>> {
    let item_bytes = if format_version >= 2 { 13 } else { 8 };
    let mut bytes = Vec::with_capacity(4 + inventory.len() * item_bytes);
    bytes.extend_from_slice(&(inventory.len() as u32).to_le_bytes());
    for item in inventory {
        bytes.extend_from_slice(&item.base_form_id.to_le_bytes());
        bytes.extend_from_slice(&item.count.to_le_bytes());
        if format_version >= 2 {
            bytes.push(item.condition.is_some() as u8);
            bytes.extend_from_slice(&item.condition.unwrap_or_default().to_le_bytes());
        } else if item.condition.is_some() {
            bail!("save format v1 cannot encode item condition");
        }
    }
    Ok(bytes)
}

fn encode_reference(
    cell_form_id: u32,
    reference_form_id: u32,
    delta: &PersistentReferenceDelta,
    format_version: u32,
) -> Result<Vec<u8>> {
    let mut payload = Vec::new();
    write_subrecord(&mut payload, tag("CELL"), &cell_form_id.to_le_bytes())?;
    write_subrecord(&mut payload, tag("REFR"), &reference_form_id.to_le_bytes())?;
    let mut flags = 0u32;
    if delta.enabled.is_some() {
        flags |= 1 << 0;
    }
    if delta.deleted {
        flags |= 1 << 1;
    }
    if delta.activated.is_some() {
        flags |= 1 << 2;
    }
    if delta.lock_level.is_some() {
        flags |= 1 << 3;
    }
    if delta.enable_root_form_id.is_some() {
        flags |= 1 << 4;
    }
    write_subrecord(&mut payload, tag("FLAG"), &flags.to_le_bytes())?;
    if let Some(value) = delta.enabled {
        write_subrecord(&mut payload, tag("ENAB"), &[value as u8])?;
    }
    if let Some(value) = delta.activated {
        write_subrecord(&mut payload, tag("ACTV"), &[value as u8])?;
    }
    if let Some(value) = delta.lock_level {
        write_subrecord(&mut payload, tag("LOCK"), &[value as u8])?;
    }
    if let Some(value) = delta.enable_root_form_id {
        write_subrecord(&mut payload, tag("ROOT"), &value.to_le_bytes())?;
    }
    if let Some(transform) = delta.transform {
        let mut bytes = Vec::with_capacity(40);
        for value in transform
            .translation
            .into_iter()
            .chain(transform.rotation_xyzw)
            .chain(transform.scale)
        {
            bytes.extend_from_slice(&value.to_le_bytes());
        }
        write_subrecord(&mut payload, tag("XFRM"), &bytes)?;
    }
    if let Some(inventory) = &delta.inventory {
        write_subrecord(
            &mut payload,
            tag("INVT"),
            &encode_inventory_bytes(inventory, format_version)?,
        )?;
    }
    if let Some(value) = delta.leveled_resolved {
        write_subrecord(&mut payload, tag("LVLR"), &[value as u8])?;
    }
    if let Some(body) = delta.body {
        let mut bytes = Vec::with_capacity(25);
        for value in body
            .linear_velocity
            .into_iter()
            .chain(body.angular_velocity)
        {
            bytes.extend_from_slice(&value.to_le_bytes());
        }
        bytes.push(body.sleeping as u8);
        write_subrecord(&mut payload, tag("BODY"), &bytes)?;
    }
    Ok(payload)
}

fn decode_reference(
    payload: &[u8],
    format_version: u32,
) -> Result<(u32, u32, PersistentReferenceDelta)> {
    let mut cell_form_id = None;
    let mut reference_form_id = None;
    let mut flags = None;
    let mut delta = PersistentReferenceDelta::default();
    for subrecord in read_subrecords(payload)? {
        match &subrecord.tag {
            record_tag if *record_tag == tag("CELL") => {
                cell_form_id = Some(read_u32(&subrecord.payload, "OBJE.CELL")?)
            }
            record_tag if *record_tag == tag("REFR") => {
                reference_form_id = Some(read_u32(&subrecord.payload, "OBJE.REFR")?)
            }
            record_tag if *record_tag == tag("FLAG") => {
                flags = Some(read_u32(&subrecord.payload, "OBJE.FLAG")?)
            }
            record_tag if *record_tag == tag("ENAB") => {
                delta.enabled = Some(read_bool(&subrecord.payload, "OBJE.ENAB")?)
            }
            record_tag if *record_tag == tag("ACTV") => {
                delta.activated = Some(read_bool(&subrecord.payload, "OBJE.ACTV")?)
            }
            record_tag if *record_tag == tag("LOCK") => {
                if subrecord.payload.len() != 1 {
                    bail!("OBJE.LOCK must contain one byte");
                }
                delta.lock_level = Some(subrecord.payload[0] as i8);
            }
            record_tag if *record_tag == tag("ROOT") => {
                delta.enable_root_form_id = Some(read_u32(&subrecord.payload, "OBJE.ROOT")?)
            }
            record_tag if *record_tag == tag("XFRM") => {
                delta.transform = Some(decode_transform(&subrecord.payload)?)
            }
            record_tag if *record_tag == tag("INVT") => {
                delta.inventory = Some(decode_inventory(&subrecord.payload, format_version)?)
            }
            record_tag if *record_tag == tag("LVLR") => {
                delta.leveled_resolved = Some(read_bool(&subrecord.payload, "OBJE.LVLR")?)
            }
            record_tag if *record_tag == tag("BODY") => {
                delta.body = Some(decode_body(&subrecord.payload)?)
            }
            _ => {}
        }
    }
    let flags = flags.context("OBJE is missing FLAG")?;
    delta.deleted = flags & (1 << 1) != 0;
    require_flag(flags, 0, delta.enabled.is_some(), "ENAB")?;
    require_flag(flags, 2, delta.activated.is_some(), "ACTV")?;
    require_flag(flags, 3, delta.lock_level.is_some(), "LOCK")?;
    require_flag(flags, 4, delta.enable_root_form_id.is_some(), "ROOT")?;
    Ok((
        cell_form_id.context("OBJE is missing CELL")?,
        reference_form_id.context("OBJE is missing REFR")?,
        delta,
    ))
}

fn encode_actor(
    cell_form_id: u32,
    reference_form_id: u32,
    actor: &ActorInstanceState,
) -> Result<Vec<u8>> {
    if actor.reference_form_id != reference_form_id {
        bail!(
            "ACTR reference key {reference_form_id:08x} does not match state {:08x}",
            actor.reference_form_id
        );
    }
    actor
        .validate()
        .map_err(|error| anyhow::anyhow!("invalid ACTR state: {error}"))?;
    let mut payload = Vec::new();
    write_subrecord(&mut payload, tag("CELL"), &cell_form_id.to_le_bytes())?;
    write_subrecord(&mut payload, tag("REFR"), &reference_form_id.to_le_bytes())?;
    let life = match actor.life_state {
        ActorLifeState::Alive => 0,
        ActorLifeState::Dead => 1,
    };
    write_subrecord(&mut payload, tag("LIFE"), &[life])?;
    if !actor.value_mutations.is_empty() {
        write_subrecord(
            &mut payload,
            tag("AVMD"),
            ron::ser::to_string(&actor.value_mutations)
                .context("encoding ACTR actor-value mutations")?
                .as_bytes(),
        )?;
    }
    if let Some(package) = actor.package {
        let mut bytes = Vec::with_capacity(12);
        bytes.extend_from_slice(&package.package_form_id.to_le_bytes());
        bytes.extend_from_slice(&package.procedure_index.to_le_bytes());
        bytes.extend_from_slice(&package.elapsed_seconds.to_le_bytes());
        write_subrecord(&mut payload, tag("PACK"), &bytes)?;
    }
    Ok(payload)
}

fn decode_actor(payload: &[u8]) -> Result<(u32, u32, ActorInstanceState)> {
    let mut cell_form_id = None;
    let mut reference_form_id = None;
    let mut life_state = None;
    let mut value_mutations = None;
    let mut package = None;
    for subrecord in read_subrecords(payload)? {
        match &subrecord.tag {
            record_tag if *record_tag == tag("CELL") => {
                if cell_form_id.is_some() {
                    bail!("ACTR contains duplicate CELL");
                }
                cell_form_id = Some(read_u32(&subrecord.payload, "ACTR.CELL")?);
            }
            record_tag if *record_tag == tag("REFR") => {
                if reference_form_id.is_some() {
                    bail!("ACTR contains duplicate REFR");
                }
                reference_form_id = Some(read_u32(&subrecord.payload, "ACTR.REFR")?);
            }
            record_tag if *record_tag == tag("LIFE") => {
                if life_state.is_some() {
                    bail!("ACTR contains duplicate LIFE");
                }
                life_state = Some(match subrecord.payload.as_slice() {
                    [0] => ActorLifeState::Alive,
                    [1] => ActorLifeState::Dead,
                    _ => bail!("ACTR.LIFE must contain one byte with value 0 or 1"),
                });
            }
            record_tag if *record_tag == tag("AVMD") => {
                if value_mutations.is_some() {
                    bail!("ACTR contains duplicate AVMD");
                }
                value_mutations = Some(
                    ron::de::from_bytes(&subrecord.payload)
                        .context("decoding ACTR actor-value mutations")?,
                );
            }
            record_tag if *record_tag == tag("PACK") => {
                if package.is_some() {
                    bail!("ACTR contains duplicate PACK");
                }
                if subrecord.payload.len() != 12 {
                    bail!("ACTR.PACK must contain twelve bytes");
                }
                package = Some(ActorPackageCheckpoint {
                    package_form_id: read_u32(&subrecord.payload[0..4], "ACTR.PACK form")?,
                    procedure_index: read_u32(&subrecord.payload[4..8], "ACTR.PACK procedure")?,
                    elapsed_seconds: f32::from_le_bytes(
                        subrecord.payload[8..12]
                            .try_into()
                            .expect("checked ACTR.PACK elapsed length"),
                    ),
                });
            }
            _ => {}
        }
    }
    let reference_form_id = reference_form_id.context("ACTR is missing REFR")?;
    let actor = ActorInstanceState {
        reference_form_id,
        life_state: life_state.context("ACTR is missing LIFE")?,
        value_mutations: value_mutations.unwrap_or_default(),
        package,
    };
    actor
        .validate()
        .map_err(|error| anyhow::anyhow!("invalid ACTR state: {error}"))?;
    Ok((
        cell_form_id.context("ACTR is missing CELL")?,
        reference_form_id,
        actor,
    ))
}

fn encode_dropped(cell_form_id: u32, dropped: &DroppedItemState) -> Result<Vec<u8>> {
    let mut payload = Vec::new();
    write_subrecord(&mut payload, tag("CELL"), &cell_form_id.to_le_bytes())?;
    write_subrecord(&mut payload, tag("RID0"), &dropped.runtime_id.to_le_bytes())?;
    write_subrecord(
        &mut payload,
        tag("BASE"),
        &dropped.stack.base_form_id.to_le_bytes(),
    )?;
    write_subrecord(
        &mut payload,
        tag("CNT0"),
        &dropped.stack.count.to_le_bytes(),
    )?;
    if let Some(condition) = dropped.stack.condition {
        write_subrecord(&mut payload, tag("COND"), &condition.to_le_bytes())?;
    }
    let mut transform = Vec::with_capacity(40);
    for value in dropped
        .transform
        .translation
        .into_iter()
        .chain(dropped.transform.rotation_xyzw)
        .chain(dropped.transform.scale)
    {
        transform.extend_from_slice(&value.to_le_bytes());
    }
    write_subrecord(&mut payload, tag("XFRM"), &transform)?;
    let mut body = Vec::with_capacity(25);
    for value in dropped
        .body
        .linear_velocity
        .into_iter()
        .chain(dropped.body.angular_velocity)
    {
        body.extend_from_slice(&value.to_le_bytes());
    }
    body.push(dropped.body.sleeping as u8);
    write_subrecord(&mut payload, tag("BODY"), &body)?;
    Ok(payload)
}

fn decode_dropped(payload: &[u8]) -> Result<(u32, DroppedItemState)> {
    let mut cell = None;
    let mut runtime_id = None;
    let mut base_form_id = None;
    let mut count = None;
    let mut condition = None;
    let mut transform = None;
    let mut body = None;
    for subrecord in read_subrecords(payload)? {
        match &subrecord.tag {
            value if *value == tag("CELL") => {
                ensure_none(&cell, "DROP.CELL")?;
                cell = Some(read_u32(&subrecord.payload, "DROP.CELL")?);
            }
            value if *value == tag("RID0") => {
                ensure_none(&runtime_id, "DROP.RID0")?;
                if subrecord.payload.len() != 8 {
                    bail!("DROP.RID0 must contain eight bytes");
                }
                runtime_id = Some(u64::from_le_bytes(
                    subrecord
                        .payload
                        .as_slice()
                        .try_into()
                        .expect("checked DROP.RID0"),
                ));
            }
            value if *value == tag("BASE") => {
                ensure_none(&base_form_id, "DROP.BASE")?;
                base_form_id = Some(read_u32(&subrecord.payload, "DROP.BASE")?);
            }
            value if *value == tag("CNT0") => {
                ensure_none(&count, "DROP.CNT0")?;
                if subrecord.payload.len() != 4 {
                    bail!("DROP.CNT0 must contain four bytes");
                }
                count = Some(i32::from_le_bytes(
                    subrecord
                        .payload
                        .as_slice()
                        .try_into()
                        .expect("checked count"),
                ));
            }
            value if *value == tag("COND") => {
                ensure_none(&condition, "DROP.COND")?;
                condition = Some(read_u32(&subrecord.payload, "DROP.COND")?);
            }
            value if *value == tag("XFRM") => {
                ensure_none(&transform, "DROP.XFRM")?;
                transform = Some(decode_transform(&subrecord.payload)?);
            }
            value if *value == tag("BODY") => {
                ensure_none(&body, "DROP.BODY")?;
                body = Some(decode_body(&subrecord.payload)?);
            }
            _ => {}
        }
    }
    let runtime_id = runtime_id.context("DROP is missing RID0")?;
    Ok((
        cell.context("DROP is missing CELL")?,
        DroppedItemState {
            runtime_id,
            stack: ItemStack {
                base_form_id: base_form_id.context("DROP is missing BASE")?,
                count: count.context("DROP is missing CNT0")?,
                condition,
            },
            transform: transform.context("DROP is missing XFRM")?,
            body: body.context("DROP is missing BODY")?,
        },
    ))
}

fn ensure_none<T>(value: &Option<T>, label: &str) -> Result<()> {
    if value.is_some() {
        bail!("{label} appears more than once");
    }
    Ok(())
}

fn decode_cell_state(payload: &[u8]) -> Result<u32> {
    let mut cell = None;
    for subrecord in read_subrecords(payload)? {
        if subrecord.tag == tag("FORM") {
            if cell.is_some() {
                bail!("CSTA contains duplicate FORM");
            }
            cell = Some(read_u32(&subrecord.payload, "CSTA.FORM")?);
        }
    }
    cell.context("CSTA is missing FORM")
}

fn decode_transform(bytes: &[u8]) -> Result<SavedTransform> {
    if bytes.len() != 40 {
        bail!("OBJE.XFRM must contain 40 bytes");
    }
    let values = read_f32_array::<10>(bytes, "OBJE.XFRM")?;
    let transform = SavedTransform {
        translation: [values[0], values[1], values[2]],
        rotation_xyzw: [values[3], values[4], values[5], values[6]],
        scale: [values[7], values[8], values[9]],
    };
    validate_transform(&transform)?;
    Ok(transform)
}

fn decode_inventory(bytes: &[u8], format_version: u32) -> Result<Vec<ItemStack>> {
    if bytes.len() < 4 {
        bail!("OBJE.INVT is truncated");
    }
    let count =
        u32::from_le_bytes(bytes[..4].try_into().expect("checked inventory header")) as usize;
    let item_bytes = if format_version >= 2 { 13 } else { 8 };
    let expected = 4usize
        .checked_add(
            count
                .checked_mul(item_bytes)
                .context("inventory count overflow")?,
        )
        .context("inventory length overflow")?;
    if bytes.len() != expected {
        bail!("OBJE.INVT length does not match its item count");
    }
    let mut inventory = Vec::with_capacity(count);
    let mut offset = 4;
    for _ in 0..count {
        inventory.push(ItemStack {
            base_form_id: u32::from_le_bytes(
                bytes[offset..offset + 4]
                    .try_into()
                    .expect("checked item id"),
            ),
            count: i32::from_le_bytes(
                bytes[offset + 4..offset + 8]
                    .try_into()
                    .expect("checked item count"),
            ),
            condition: if format_version >= 2 {
                match bytes[offset + 8] {
                    0 => None,
                    1 => Some(u32::from_le_bytes(
                        bytes[offset + 9..offset + 13]
                            .try_into()
                            .expect("checked item condition"),
                    )),
                    _ => bail!("inventory condition flag must be 0 or 1"),
                }
            } else {
                None
            },
        });
        offset += item_bytes;
    }
    validate_inventory(&inventory)?;
    Ok(inventory)
}

fn decode_body(bytes: &[u8]) -> Result<SavedBodyState> {
    if bytes.len() != 25 {
        bail!("OBJE.BODY must contain 25 bytes");
    }
    let values = read_f32_array::<6>(&bytes[..24], "OBJE.BODY")?;
    if bytes[24] > 1 {
        bail!("OBJE.BODY sleeping flag must be 0 or 1");
    }
    let body = SavedBodyState {
        linear_velocity: [values[0], values[1], values[2]],
        angular_velocity: [values[3], values[4], values[5]],
        sleeping: bytes[24] != 0,
    };
    validate_body(&body)?;
    Ok(body)
}

fn validate_save(save: &SaveGame) -> Result<()> {
    if !(MIN_SUPPORTED_SAVE_FORMAT_VERSION..=CURRENT_SAVE_FORMAT_VERSION)
        .contains(&save.header.format_version)
    {
        bail!(
            "unsupported save format version {}",
            save.header.format_version
        );
    }
    if !save.header.play_time_seconds.is_finite() || save.header.play_time_seconds < 0.0 {
        bail!("save play time must be finite and non-negative");
    }
    if save.next_runtime_item_id == 0 {
        bail!("next runtime item id must be non-zero");
    }
    if save.dialogue.schema_version != bevyout_core::dialogue::DIALOGUE_SNAPSHOT_SCHEMA_VERSION {
        bail!(
            "unsupported dialogue snapshot schema version {}",
            save.dialogue.schema_version
        );
    }
    if let Some(checkpoint) = &save.dialogue.active {
        if checkpoint.dialogue.as_str().is_empty() || checkpoint.node.is_empty() {
            bail!("dialogue checkpoint must identify a dialogue and node");
        }
        if checkpoint
            .completed_actions
            .windows(2)
            .any(|window| window[0] >= window[1])
        {
            bail!("dialogue checkpoint action keys must be sorted and unique");
        }
    }
    let mut plugin_names = Vec::new();
    for plugin in &save.header.plugins {
        if plugin.name.is_empty() || plugin.fingerprint.is_empty() {
            bail!("save plugin names and fingerprints must be non-empty");
        }
        let key = plugin.name.to_ascii_lowercase();
        if plugin_names.contains(&key) {
            bail!("save contains duplicate plugin {}", plugin.name);
        }
        plugin_names.push(key);
    }
    if let Some(player) = &save.player {
        validate_inventory(&player.inventory)?;
        validate_equipped(&player.equipped)?;
    }
    for (cell_form_id, cell) in &save.world.cells {
        if *cell_form_id == 0 && !cell.actors.is_empty() {
            bail!("actor cell FormID must be non-zero");
        }
        for delta in cell.references.values() {
            if let Some(inventory) = &delta.inventory {
                validate_inventory(inventory)?;
            }
            if let Some(transform) = delta.transform {
                validate_transform(&transform)?;
            }
            if let Some(body) = delta.body {
                validate_body(&body)?;
            }
        }
        for (runtime_id, dropped) in &cell.dropped_items {
            if *runtime_id == 0 || dropped.runtime_id != *runtime_id {
                bail!("dropped item runtime id must be non-zero and match its map key");
            }
            if dropped.stack.count <= 0 {
                bail!("dropped item count must be positive");
            }
            validate_transform(&dropped.transform)?;
            validate_body(&dropped.body)?;
            if *runtime_id >= save.next_runtime_item_id {
                bail!("next runtime item id must exceed every dropped item id");
            }
        }
        for (reference_form_id, actor) in &cell.actors {
            if *reference_form_id != actor.reference_form_id {
                bail!(
                    "actor state key {reference_form_id:08x} does not match reference {:08x}",
                    actor.reference_form_id
                );
            }
            actor
                .validate()
                .map_err(|error| anyhow::anyhow!("invalid actor state: {error}"))?;
        }
    }
    if save.header.format_version >= 3 {
        let canonical = save
            .canonical
            .as_ref()
            .context("v3+ save is missing canonical item state")?;
        validate_canonical(canonical)?;
    } else if save.canonical.is_some() {
        bail!("canonical item state is not valid in save format v1/v2");
    }
    if save.header.format_version < 4
        && save
            .world
            .cells
            .values()
            .any(|cell| !cell.actors.is_empty())
    {
        bail!("actor state is not valid before save format v4");
    }
    if save.header.format_version < 7 && save.location.is_some() {
        bail!("player world location is not valid before save format v7");
    }
    if let Some(location) = &save.location
        && !location.is_well_formed()
    {
        bail!("invalid player world location");
    }
    Ok(())
}

fn validate_canonical(snapshot: &ItemLedgerSnapshot) -> Result<()> {
    if snapshot.next_item_id.0 == 0 || snapshot.next_transaction_id.0 == 0 {
        bail!("canonical save counters must be non-zero");
    }
    let mut ids = BTreeSet::new();
    let mut max_id = 0;
    for (holder, state) in &snapshot.holders {
        state.validate().map_err(|error| anyhow::anyhow!(error))?;
        for item in &state.items {
            if !ids.insert(item.id) {
                bail!(
                    "canonical item id {:?} appears in more than one holder",
                    item.id
                );
            }
            max_id = max_id.max(item.id.0);
        }
        if let Some(binding) = snapshot.bindings.get(holder)
            && let Some(item_id) = binding
                .equipped
                .into_iter()
                .chain(binding.equipped_apparel.iter().copied())
                .chain(binding.hotkeys.into_iter().flatten())
                .find(|item_id| state.find(*item_id).is_none())
        {
            bail!(
                "canonical binding for holder {:?} references item {:?} in another holder or no holder",
                holder,
                item_id
            );
        }
    }
    for holder in snapshot.bindings.keys() {
        if !snapshot.holders.contains_key(holder) {
            bail!("canonical bindings reference unknown holder {:?}", holder);
        }
    }
    if snapshot.next_item_id.0 <= max_id {
        bail!("canonical next item id must exceed every item id");
    }
    Ok(())
}

fn validate_inventory(inventory: &[ItemStack]) -> Result<()> {
    let mut previous = None;
    for item in inventory {
        if item.count == 0 {
            bail!("save inventory stacks may not have a zero count");
        }
        let key = (item.base_form_id, item.condition);
        if previous.is_some_and(|previous| previous >= key) {
            bail!("save inventory stacks must be strictly sorted by FormID and condition");
        }
        previous = Some(key);
    }
    Ok(())
}

/// Issue #98 (F98.4): mirrors `validate_inventory`'s strict-sort invariant,
/// keyed by `(kind, base_form_id, condition)` so encode/decode stays
/// deterministic byte-for-byte.
fn validate_equipped(equipped: &[EquippedItem]) -> Result<()> {
    let mut previous: Option<(EquippedKind, u32, Option<u32>)> = None;
    for item in equipped {
        let key = (item.kind, item.base_form_id, item.condition);
        if previous.is_some_and(|previous| previous >= key) {
            bail!("save equipped items must be strictly sorted by kind, FormID, and condition");
        }
        previous = Some(key);
    }
    Ok(())
}

fn validate_transform(transform: &SavedTransform) -> Result<()> {
    if transform
        .translation
        .into_iter()
        .chain(transform.rotation_xyzw)
        .chain(transform.scale)
        .any(|value| !value.is_finite())
    {
        bail!("save transform contains a non-finite value");
    }
    Ok(())
}

fn validate_body(body: &SavedBodyState) -> Result<()> {
    if body
        .linear_velocity
        .into_iter()
        .chain(body.angular_velocity)
        .any(|value| !value.is_finite())
    {
        bail!("save body state contains a non-finite value");
    }
    Ok(())
}

fn read_u32(bytes: &[u8], label: &str) -> Result<u32> {
    if bytes.len() != 4 {
        bail!("{label} must contain four bytes");
    }
    Ok(u32::from_le_bytes(
        bytes.try_into().expect("checked u32 length"),
    ))
}

fn read_bool(bytes: &[u8], label: &str) -> Result<bool> {
    if bytes.len() != 1 || bytes[0] > 1 {
        bail!("{label} must contain one byte with value 0 or 1");
    }
    Ok(bytes[0] != 0)
}

fn read_string(bytes: &[u8], label: &str) -> Result<String> {
    String::from_utf8(bytes.to_vec()).with_context(|| format!("{label} is not valid UTF-8"))
}

fn read_f32_array<const N: usize>(bytes: &[u8], label: &str) -> Result<[f32; N]> {
    if bytes.len() != N * 4 {
        bail!("{label} has the wrong length");
    }
    let mut result = [0.0; N];
    for (index, value) in result.iter_mut().enumerate() {
        *value = f32::from_le_bytes(
            bytes[index * 4..index * 4 + 4]
                .try_into()
                .expect("checked f32 length"),
        );
    }
    Ok(result)
}

fn write_length_prefixed(output: &mut Vec<u8>, bytes: &[u8]) -> Result<()> {
    if bytes.len() > u32::MAX as usize {
        bail!("length-prefixed value is too large");
    }
    output.extend_from_slice(&(bytes.len() as u32).to_le_bytes());
    output.extend_from_slice(bytes);
    Ok(())
}

fn read_length_prefixed(bytes: &[u8], cursor: &mut usize, label: &str) -> Result<String> {
    if bytes.len().saturating_sub(*cursor) < 4 {
        bail!("{label} length is truncated");
    }
    let length = u32::from_le_bytes(
        bytes[*cursor..*cursor + 4]
            .try_into()
            .expect("checked length prefix"),
    ) as usize;
    *cursor += 4;
    let end = cursor
        .checked_add(length)
        .ok_or_else(|| anyhow::anyhow!("{label} length overflows"))?;
    if end > bytes.len() {
        bail!("{label} value is truncated");
    }
    let value = read_string(&bytes[*cursor..end], label)?;
    *cursor = end;
    Ok(value)
}

fn ensure_once(seen: &mut bool, label: &str) -> Result<()> {
    if *seen {
        bail!("duplicate {label} subrecord");
    }
    *seen = true;
    Ok(())
}

fn require_flag(flags: u32, bit: u32, present: bool, label: &str) -> Result<()> {
    if flags & (1 << bit) != 0 && !present {
        bail!("OBJE FLAG requires {label}");
    }
    if flags & (1 << bit) == 0 && present {
        bail!("OBJE {label} is present without its FLAG bit");
    }
    Ok(())
}

fn validate_slot(slot: &str) -> Result<()> {
    if slot.is_empty()
        || slot == "."
        || slot == ".."
        || !slot.chars().all(|character| {
            character.is_ascii_alphanumeric() || matches!(character, '-' | '_' | '.')
        })
    {
        bail!("invalid save slot '{slot}'; use only ASCII letters, digits, '-', '_', or '.'");
    }
    Ok(())
}

fn write_synced(path: &Path, bytes: &[u8]) -> Result<()> {
    let mut file = File::create(path).with_context(|| format!("creating {}", path.display()))?;
    file.write_all(bytes)
        .with_context(|| format!("writing {}", path.display()))?;
    file.sync_all()
        .with_context(|| format!("flushing {}", path.display()))?;
    Ok(())
}

fn install_with_backup(temporary: &Path, primary: &Path, backup: &Path) -> Result<()> {
    if primary.exists() {
        if backup.exists() {
            fs::remove_file(backup).with_context(|| format!("removing {}", backup.display()))?;
        }
        fs::rename(primary, backup).with_context(|| {
            format!(
                "moving previous save {} to {}",
                primary.display(),
                backup.display()
            )
        })?;
    }
    match fs::rename(temporary, primary) {
        Ok(()) => Ok(()),
        Err(error) => {
            if backup.exists() && !primary.exists() {
                let _ = fs::rename(backup, primary);
            }
            Err(error).with_context(|| format!("installing save {}", primary.display()))
        }
    }
}

fn io_error(error: anyhow::Error) -> std::io::Error {
    std::io::Error::other(error.to_string())
}

trait InsertChecked<K, V> {
    fn insert_checked(&mut self, key: K, value: V) -> Result<()>;
}

impl<K: Ord, V> InsertChecked<K, V> for BTreeMap<K, V> {
    fn insert_checked(&mut self, key: K, value: V) -> Result<()> {
        if self.insert(key, value).is_some() {
            bail!("save contains duplicate OBJE reference state");
        }
        Ok(())
    }
}

#[cfg(test)]
#[path = "tests/mod.rs"]
mod tests;

#[cfg(test)]
#[path = "tests/location.rs"]
mod location_tests;
