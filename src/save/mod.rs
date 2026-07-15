//! Persistent save foundation for the Bevy runtime.
//!
//! This slice follows OpenMW's ownership model: the save header describes the
//! content set, cell state is keyed by stable FormIDs, and reference changes
//! are stored as deltas. Runtime ECS capture/application will consume this
//! model in a later slice.

use anyhow::{Context, Result, bail};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};

mod openmw;

use openmw::{read_records, read_subrecords, tag, write_record, write_subrecord};

pub const CURRENT_SAVE_FORMAT_VERSION: u32 = 1;
pub const MIN_SUPPORTED_SAVE_FORMAT_VERSION: u32 = 1;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SaveGame {
    pub header: SaveGameHeader,
    pub world: PersistentWorldState,
    /// Optional player record (issue #60, F60.4): `None` round-trips as an
    /// absent PLYR record, so saves written before this record existed --
    /// and readers older than it (unknown records are skipped, see the
    /// forward-compatibility test) -- stay compatible in both directions.
    pub player: Option<PlayerState>,
    pub rng_state: u64,
}

/// Persistent player state (issue #60, F60.4). Currently just the
/// inventory, so picked-up keys still open locked doors after a restart.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct PlayerState {
    pub inventory: Vec<ItemStack>,
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
    validate_save(save)?;
    let mut bytes = Vec::new();
    write_record(&mut bytes, tag("SAVE"), &encode_header(&save.header)?)?;
    if let Some(player) = &save.player {
        write_record(&mut bytes, tag("PLYR"), &encode_player(player)?)?;
    }
    for (cell_form_id, cell) in &save.world.cells {
        let mut payload = Vec::new();
        write_subrecord(&mut payload, tag("FORM"), &cell_form_id.to_le_bytes())?;
        write_record(&mut bytes, tag("CSTA"), &payload)?;
        for (reference_form_id, delta) in &cell.references {
            write_record(
                &mut bytes,
                tag("OBJE"),
                &encode_reference(*cell_form_id, *reference_form_id, delta)?,
            )?;
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
                    save.player = Some(decode_player(&record.payload)?);
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
                        decode_reference(&record.payload)?;
                    save.world
                        .cells
                        .entry(cell_form_id)
                        .or_default()
                        .references
                        .insert_checked(reference_form_id, delta)?;
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
    validate_save(&save)?;
    Ok(save)
}

impl SaveGame {
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

fn encode_player(player: &PlayerState) -> Result<Vec<u8>> {
    let mut payload = Vec::new();
    write_subrecord(
        &mut payload,
        tag("INVT"),
        &encode_inventory_bytes(&player.inventory),
    )?;
    Ok(payload)
}

fn decode_player(payload: &[u8]) -> Result<PlayerState> {
    let mut player = PlayerState::default();
    let mut saw_inventory = false;
    for subrecord in read_subrecords(payload)? {
        if subrecord.tag == tag("INVT") {
            ensure_once(&mut saw_inventory, "PLYR.INVT")?;
            player.inventory = decode_inventory(&subrecord.payload)?;
        }
    }
    if !saw_inventory {
        bail!("PLYR record is missing its INVT subrecord");
    }
    Ok(player)
}

fn encode_inventory_bytes(inventory: &[ItemStack]) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(4 + inventory.len() * 8);
    bytes.extend_from_slice(&(inventory.len() as u32).to_le_bytes());
    for item in inventory {
        bytes.extend_from_slice(&item.base_form_id.to_le_bytes());
        bytes.extend_from_slice(&item.count.to_le_bytes());
    }
    bytes
}

fn encode_reference(
    cell_form_id: u32,
    reference_form_id: u32,
    delta: &PersistentReferenceDelta,
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
        write_subrecord(&mut payload, tag("INVT"), &encode_inventory_bytes(inventory))?;
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

fn decode_reference(payload: &[u8]) -> Result<(u32, u32, PersistentReferenceDelta)> {
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
                delta.inventory = Some(decode_inventory(&subrecord.payload)?)
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

fn decode_inventory(bytes: &[u8]) -> Result<Vec<ItemStack>> {
    if bytes.len() < 4 {
        bail!("OBJE.INVT is truncated");
    }
    let count =
        u32::from_le_bytes(bytes[..4].try_into().expect("checked inventory header")) as usize;
    let expected = 4usize
        .checked_add(count.checked_mul(8).context("inventory count overflow")?)
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
        });
        offset += 8;
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
    }
    for cell in save.world.cells.values() {
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
    }
    Ok(())
}

fn validate_inventory(inventory: &[ItemStack]) -> Result<()> {
    let mut previous = None;
    for item in inventory {
        if item.count == 0 {
            bail!("save inventory stacks may not have a zero count");
        }
        if previous.is_some_and(|form_id| form_id >= item.base_form_id) {
            bail!("save inventory stacks must be strictly sorted by FormID");
        }
        previous = Some(item.base_form_id);
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
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn sample_save() -> SaveGame {
        let mut references = BTreeMap::new();
        references.insert(
            0x0100_0020,
            PersistentReferenceDelta {
                enabled: Some(false),
                deleted: false,
                activated: Some(true),
                lock_level: Some(50),
                enable_root_form_id: Some(0x0100_0010),
                transform: Some(SavedTransform {
                    translation: [1.0, 2.0, 3.0],
                    rotation_xyzw: [0.0, 0.0, 0.0, 1.0],
                    scale: [1.0, 1.0, 1.0],
                }),
                inventory: Some(vec![ItemStack {
                    base_form_id: 0x0000_0042,
                    count: 3,
                }]),
                body: Some(SavedBodyState {
                    linear_velocity: [0.1, 0.2, 0.3],
                    angular_velocity: [0.4, 0.5, 0.6],
                    sleeping: true,
                }),
            },
        );
        let mut cells = BTreeMap::new();
        cells.insert(0x0001_51e3, PersistentCellState { references });
        SaveGame {
            header: SaveGameHeader {
                format_version: CURRENT_SAVE_FORMAT_VERSION,
                content_fingerprint: "content-hash".into(),
                plugins: vec![SavePlugin {
                    name: "Fallout3.esm".into(),
                    fingerprint: "plugin-hash".into(),
                }],
                current_cell: 0x0001_51e3,
                play_time_seconds: 12.5,
                description: "test save".into(),
            },
            world: PersistentWorldState { cells },
            player: Some(PlayerState {
                inventory: vec![
                    ItemStack {
                        base_form_id: 0x0000_0011,
                        count: 1,
                    },
                    ItemStack {
                        base_form_id: 0x0000_0042,
                        count: 3,
                    },
                ],
            }),
            rng_state: 0x0123_4567_89ab_cdef,
        }
    }

    #[test]
    fn round_trip_is_deterministic() {
        let save = sample_save();
        let first = encode_save(&save).unwrap();
        let second = encode_save(&save).unwrap();
        assert_eq!(first, second);
        assert_eq!(decode_save(&first).unwrap(), save);
    }

    #[test]
    fn unknown_records_are_skipped_when_checksum_is_recomputed() {
        let encoded = encode_save(&sample_save()).unwrap();
        let checksum_start = encoded.len() - 40;
        let mut bytes = encoded[..checksum_start].to_vec();
        write_record(&mut bytes, tag("FUTR"), b"future data").unwrap();
        let checksum: [u8; 32] = Sha256::digest(&bytes).into();
        write_record(&mut bytes, tag("CHKS"), &checksum).unwrap();
        assert_eq!(decode_save(&bytes).unwrap(), sample_save());
    }

    // F60.4: a save with no player record round-trips as `None`, so
    // pre-player saves stay loadable.
    #[test]
    fn a_save_without_a_player_record_round_trips_as_none() {
        let mut save = sample_save();
        save.player = None;
        let bytes = encode_save(&save).unwrap();
        let decoded = decode_save(&bytes).unwrap();
        assert_eq!(decoded.player, None);
        assert_eq!(decoded, save);
    }

    // F60.4: the player inventory rides the same validation as reference
    // inventories -- zero counts and unsorted stacks are rejected before
    // encoding.
    #[test]
    fn an_invalid_player_inventory_is_rejected() {
        let mut save = sample_save();
        save.player = Some(PlayerState {
            inventory: vec![ItemStack {
                base_form_id: 0x1,
                count: 0,
            }],
        });
        assert!(encode_save(&save).is_err());
        save.player = Some(PlayerState {
            inventory: vec![
                ItemStack {
                    base_form_id: 0x2,
                    count: 1,
                },
                ItemStack {
                    base_form_id: 0x1,
                    count: 1,
                },
            ],
        });
        assert!(encode_save(&save).is_err());
    }

    #[test]
    fn corruption_is_rejected() {
        let mut bytes = encode_save(&sample_save()).unwrap();
        bytes[12] ^= 0x80;
        assert!(decode_save(&bytes).is_err());
        assert!(decode_save(&bytes[..bytes.len() - 1]).is_err());
    }

    #[test]
    fn content_fingerprint_and_plugins_are_checked_before_apply() {
        let save = sample_save();
        assert!(
            save.ensure_compatible(
                "content-hash",
                &[SavePlugin {
                    name: "Fallout3.esm".into(),
                    fingerprint: "plugin-hash".into(),
                }]
            )
            .is_ok()
        );
        assert!(save.ensure_compatible("other-hash", &[]).is_err());
    }

    #[test]
    fn backup_is_used_when_primary_is_corrupt() {
        let root = std::env::temp_dir().join(format!(
            "bevyout-save-test-{}-{}",
            std::process::id(),
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        let store = SaveStore::new(&root);
        let first = sample_save();
        let mut second = sample_save();
        second.rng_state = 42;
        store.write_slot("slot", &first).unwrap();
        store.write_slot("slot", &second).unwrap();
        fs::write(store.primary_path("slot"), b"corrupt").unwrap();
        let outcome = store.read_slot("slot").unwrap();
        assert_eq!(outcome.source, SaveSlotSource::Backup);
        assert_eq!(outcome.save, first);
        assert!(outcome.warning.is_some());
        let _ = fs::remove_dir_all(root);
    }
}
