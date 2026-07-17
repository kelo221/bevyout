use std::collections::{BTreeMap, HashMap, HashSet};

use bevy::picking::mesh_picking::ray_cast::{MeshRayCast, MeshRayCastSettings, RayCastVisibility};
use bevy::prelude::*;

use crate::app_state::{AppState, GameplayModal, RequestStateTransition};
use crate::console::{ConsoleSessionStore, RefRegistry};
use crate::item_transaction::{
    HolderId, ItemHolderState, ItemInstanceId, ItemLedger, ItemLedgerSnapshot, ItemState,
    TransactionError, TransactionRequest,
};
use crate::vsa::{
    PreparedDoor, PreparedInventoryEntry, PreparedItemCatalog, PreparedItemCategory,
    PreparedItemDefinition, PreparedItemStats, PreparedLeveledList, PreparedPlacement,
    PreparedSemantic,
};

use super::animation::{self, ClipTransition};
use super::audio::PlaySound;
use super::inventory::{Inventory, InventoryStack, StackKey, TransferResult};
use super::player::equipment::{self, EquipError, EquipKind, EquipOutcome, EquipmentState};
use super::player::{CameraMode, CameraModeState};
use super::world::{ActiveCell, PlaythroughSeed, ResidentCells};

// Pure std/serde-only modules (no Bevy imports), so `tests/features.rs` can
// pull them in verbatim via `#[path]`; see their module doc comments for
// the pattern (issue #74 resolver, issue #75 transfer policy).
pub(crate) mod container_policy;
pub(crate) mod item_rules;
pub(crate) mod item_use;
pub(crate) mod leveled;
mod transfer_ui;

pub(crate) const INTERACTION_DISTANCE_METERS: f32 = 3.0;
/// No leveling system exists yet (later M3+ scope): leveled lists resolve
/// against a fixed player level until one does.
const PLAYER_LEVEL: u16 = 1;
const NOTICE_SECONDS: f32 = 3.0;
const FOCUS_RAYCAST_INTERVAL_SECONDS: f32 = 0.1;
const MAX_PARENT_DEPTH: usize = 64;

/// Issue #52: written when the player opens a door whose `destination` is
/// `Some`, and consumed the same frame by `world::swap`'s eligibility system
/// (ordered `.after(DoorActivationSet)`) to drive either an instant cell
/// swap or a loading-screen fallback. Translation/rotation are already in
/// Bevy coordinates (converted at prepare time), matching
/// `PreparedDoorDestination`.
///
/// Issue #57: `activate_focused_placement` no longer always writes this
/// directly. A door with an `Open` clip stages it in `PendingDoorTravel`
/// instead, and `tick_pending_door_travel` (also `.in_set(DoorActivationSet)`,
/// chained right after) writes it once the open-lead elapses -- possibly
/// several frames later, but always from a system inside this set, so
/// `world::swap`'s same-frame contract holds on the frame the lead expires.
/// A door with no clip (zero lead) still writes it the same frame it
/// activates, bit-for-bit wave-2's behavior.
#[derive(Message, Clone, Copy, Debug)]
pub(crate) struct DoorTravelRequested {
    pub(crate) destination_cell_form_id: u32,
    pub(crate) translation: Vec3,
    pub(crate) rotation_xyzw: [f32; 4],
}

/// Ordering handle for `world::swap`'s door-travel systems: message readers
/// scheduled `.after(DoorActivationSet)` see `DoorTravelRequested` messages
/// written this same frame (Bevy's message double-buffering swaps once per
/// frame in `First`, not between systems), so the eligibility check and any
/// instant swap complete in the same frame as the door activation itself.
#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct DoorActivationSet;

/// F57.3: a travel door's `DoorTravelRequested` write staged behind its
/// `Open` clip's lead (`animation::open_lead_seconds`). Only one travel can
/// be pending at a time -- same constraint `world::swap`'s
/// `PendingInstantSwap`/`PendingFallbackSwap` already enforce for the
/// message itself.
#[derive(Resource, Default)]
struct PendingDoorTravel(Option<PendingTravel>);

struct PendingTravel {
    entity: Entity,
    remaining_seconds: f32,
    request: DoorTravelRequested,
}

/// F57.3: counts a pending travel's open-lead down every frame this set
/// runs (gated the same as door activation itself: `AppState::InGame` and
/// `GameplayModal::None`, so a modal opening mid-lead pauses the countdown
/// exactly like it pauses everything else in this chain) and writes
/// `DoorTravelRequested` once it elapses.
fn tick_pending_door_travel(
    time: Res<Time>,
    mut pending: ResMut<PendingDoorTravel>,
    mut door_travel: MessageWriter<DoorTravelRequested>,
) {
    let Some(travel) = pending.0.as_mut() else {
        return;
    };
    travel.remaining_seconds -= time.delta_secs();
    if travel.remaining_seconds <= 0.0 {
        let request = travel.request;
        pending.0 = None;
        door_travel.write(request);
    }
}

/// Wave-3 shipped amendment: scripted (console/BRP) door activation follows
/// the same Open-clip lead as the player's Enter activation — the door is
/// marked open, its clip plays, and the travel request is staged behind the
/// lead. Zero lead (no clip) writes the message this same frame, exactly the
/// wave-2 `activate` behavior. Returns the lead in milliseconds so the
/// console can report it.
pub(crate) fn scripted_door_travel(
    world: &mut World,
    entity: Entity,
    request: DoorTravelRequested,
) -> f32 {
    let open_clip_seconds = world
        .get::<animation::AnimatedPlacement>(entity)
        .and_then(|animated| animated.clip_seconds("Open"));
    let lead_seconds =
        animation::open_lead_seconds(open_clip_seconds, animation::OPEN_LEAD_CAP_SECONDS);
    world
        .get_resource_or_insert_with(InteractionState::default)
        .open
        .insert(entity);
    world.write_message(animation::PlayPlacementAnimation {
        root: entity,
        transition: ClipTransition::Opening,
        lead_ms: lead_seconds * 1000.0,
    });
    if lead_seconds <= 0.0 {
        world.write_message(request);
    } else {
        world
            .get_resource_or_insert_with(PendingDoorTravel::default)
            .0 = Some(PendingTravel {
            entity,
            remaining_seconds: lead_seconds,
            request,
        });
    }
    lead_seconds * 1000.0
}

/// Wave-4 acceptance seam: scripted (console/BRP) container activation,
/// mirroring `activate_focused_placement`'s container branch (open-set
/// toggle, clip, sound, notice) minus the raycast focus/distance checks.
/// Returns the new open state. Used to drive the #60/#61 gate walk-through
/// ("open a container, revisit, restart") over the agent bridge.
pub(crate) fn scripted_container_toggle(world: &mut World, entity: Entity) -> bool {
    let placement = world
        .get::<PlacementRoot>(entity)
        .expect("caller resolved a placement root")
        .placement()
        .clone();
    let name = placement
        .display_name
        .clone()
        .or_else(|| placement.editor_id.clone())
        .unwrap_or_else(|| format!("{:08x}", placement.reference_form_id));
    let position = world
        .get::<GlobalTransform>(entity)
        .map(|transform| transform.translation())
        .unwrap_or_default();
    let mut state = world.get_resource_or_insert_with(InteractionState::default);
    let opening = !state.open.contains(&entity);
    let (sound, transition) = if opening {
        state.open.insert(entity);
        (placement.audio.open_sound_form_id, ClipTransition::Opening)
    } else {
        state.open.remove(&entity);
        (placement.audio.close_sound_form_id, ClipTransition::Closing)
    };
    if let Some(form_id) = sound {
        world.write_message(PlaySound::container_at(form_id, position));
    }
    world.write_message(animation::PlayPlacementAnimation {
        root: entity,
        transition,
        lead_ms: 0.0,
    });
    let notice_text = if opening {
        format!("{name}: {}", inventory_summary(&placement.inventory))
    } else {
        format!("Closed {name}")
    };
    world
        .get_resource_or_insert_with(InteractionNotice::default)
        .show(notice_text);
    // Wave-2 seam: scripted opens go through the same seed-once container
    // store as the player path, so console/BRP `activate` observes (and
    // triggers) exactly the state the transfer modal and #76's capture see —
    // including the deterministic first-open leveled roll. Stacks are logged
    // for gate evidence (determinism across runs, no re-roll after reload).
    if opening {
        let active_cell = world
            .get_resource::<ActiveCell>()
            .map(|cell| cell.0)
            .unwrap_or_default();
        let playthrough_seed = world
            .get_resource::<PlaythroughSeed>()
            .map(|seed| seed.0)
            .unwrap_or_default();
        let leveled_lists = if placement.inventory.iter().any(|entry| entry.leveled) {
            world
                .get_resource::<ResidentCells>()
                .and_then(|cells| cells.0.get(&active_cell))
                .map(|resident| leveled_lists_from_manifest(&resident.manifest.leveled_lists))
                .unwrap_or_default()
        } else {
            BTreeMap::new()
        };
        let mut states = world.get_resource_or_insert_with(ContainerStates::default);
        let resolved = seed_loot_holder(
            &mut states,
            &placement,
            active_cell,
            playthrough_seed,
            &leveled_lists,
        );
        info!(
            "container {} ({:08x}) opened with {} stacks: {:?}",
            name, placement.reference_form_id, resolved.0, resolved.1
        );
    } else {
        info!(
            "container {} ({:08x}) closed",
            name, placement.reference_form_id
        );
    }
    opening
}

/// M4 wave 3 (#112): scripted intra-cell door open, for the `bevy_landmass`
/// nav-agent door-link spike (`nav::agent`'s door-link system) to request an
/// off-mesh-link door through the same boundary the `activate` console
/// command uses, rather than inventing a parallel door-opening path.
/// `scripted_door_travel` (above) only covers the destination/cell-swap
/// case; the real player path (`activate_focused_placement`'s door branch)
/// is inline in a multi-`Query`/`MessageWriter` system and not directly
/// callable from `&mut World`. This mirrors `scripted_container_toggle`'s
/// shape (state toggle, sound, animation clip) restricted to the "ensure
/// open" direction a traversing agent needs, and -- like
/// `console::activate_reference`'s scripted door branch -- bypasses locks,
/// since this is dev/agent tooling, not the player's raycast-focused `E`
/// activation. Idempotent: a door already open is left open. Returns
/// whether this call transitioned the door from closed to open.
pub(crate) fn scripted_door_open(world: &mut World, entity: Entity) -> bool {
    let placement = world
        .get::<PlacementRoot>(entity)
        .expect("caller resolved a placement root")
        .placement()
        .clone();
    let name = placement_name(&placement);
    let position = world
        .get::<GlobalTransform>(entity)
        .map(|transform| transform.translation())
        .unwrap_or_default();
    let opening = !world
        .get_resource_or_insert_with(InteractionState::default)
        .open
        .contains(&entity);
    if !opening {
        return false;
    }
    world
        .get_resource_or_insert_with(InteractionState::default)
        .open
        .insert(entity);
    if let Some(form_id) = placement.audio.open_sound_form_id {
        world.write_message(PlaySound::at(form_id, position));
    }
    world.write_message(animation::PlayPlacementAnimation {
        root: entity,
        transition: ClipTransition::Opening,
        lead_ms: 0.0,
    });
    info!(
        "door {} ({:08x}) opened (scripted, nav agent)",
        name, placement.reference_form_id
    );
    true
}

/// F118.2: scripted corpse activation enters the same transfer modal as
/// player activation. A corpse is deliberately staged data in this slice;
/// no actor death or AI transition is implied by this function.
pub(crate) fn scripted_corpse_toggle(world: &mut World, entity: Entity) -> bool {
    let placement = world
        .get::<PlacementRoot>(entity)
        .expect("caller resolved a placement root")
        .placement()
        .clone();
    let name = placement_name(&placement);
    let position = world
        .get::<GlobalTransform>(entity)
        .map(|transform| transform.translation())
        .or_else(|| {
            world
                .get::<Transform>(entity)
                .map(|transform| transform.translation)
        })
        .unwrap_or_default();
    let opening = {
        let mut state = world.get_resource_or_insert_with(InteractionState::default);
        if state.open.contains(&entity) {
            state.open.remove(&entity);
            false
        } else {
            state.open.insert(entity);
            true
        }
    };

    if opening {
        let active_cell = world
            .get_resource::<ActiveCell>()
            .map(|cell| cell.0)
            .unwrap_or_default();
        let playthrough_seed = world
            .get_resource::<PlaythroughSeed>()
            .map(|seed| seed.0)
            .unwrap_or_default();
        let leveled_lists = if placement.inventory.iter().any(|entry| entry.leveled) {
            world
                .get_resource::<ResidentCells>()
                .and_then(|cells| cells.0.get(&active_cell))
                .map(|resident| leveled_lists_from_manifest(&resident.manifest.leveled_lists))
                .unwrap_or_default()
        } else {
            BTreeMap::new()
        };
        let resolved = {
            let mut states = world.get_resource_or_insert_with(ContainerStates::default);
            seed_loot_holder(
                &mut states,
                &placement,
                active_cell,
                playthrough_seed,
                &leveled_lists,
            )
        };
        world
            .get_resource_or_insert_with(ActiveContainerTarget::default)
            .0 = Some(ActiveContainer {
            kind: LootHolderKind::Corpse,
            entity,
            reference_form_id: placement.reference_form_id,
            name: name.clone(),
            item_names: container_item_names(&placement.inventory),
            owner_form_id: placement.owner_form_id,
        });
        if world.contains_resource::<Messages<PlaySound>>()
            && let Some(form_id) = placement.audio.open_sound_form_id
        {
            world.write_message(PlaySound::at(form_id, position));
        }
        if world.contains_resource::<Messages<animation::PlayPlacementAnimation>>() {
            world.write_message(animation::PlayPlacementAnimation {
                root: entity,
                transition: ClipTransition::Opening,
                lead_ms: 0.0,
            });
        }
        world
            .get_resource_or_insert_with(InteractionNotice::default)
            .show(format!("Looting {name}"));
        if world.contains_resource::<Messages<RequestStateTransition>>() {
            world.write_message(RequestStateTransition::Modal(GameplayModal::Container));
        }
        info!(
            "corpse {} ({:08x}) opened with {} stacks",
            name, placement.reference_form_id, resolved.0
        );
    } else {
        let active_target_matches = world
            .get_resource::<ActiveContainerTarget>()
            .and_then(|target| target.0.as_ref())
            .is_some_and(|target| target.entity == entity && target.kind == LootHolderKind::Corpse);
        if active_target_matches {
            world.resource_mut::<ActiveContainerTarget>().0 = None;
        }
        if world.contains_resource::<Messages<PlaySound>>()
            && let Some(form_id) = placement.audio.close_sound_form_id
        {
            world.write_message(PlaySound::at(form_id, position));
        }
        if world.contains_resource::<Messages<animation::PlayPlacementAnimation>>() {
            world.write_message(animation::PlayPlacementAnimation {
                root: entity,
                transition: ClipTransition::Closing,
                lead_ms: 0.0,
            });
        }
        world
            .get_resource_or_insert_with(InteractionNotice::default)
            .show(format!("Closed {name}"));
        if world
            .get_resource::<State<GameplayModal>>()
            .is_some_and(|state| *state.get() == GameplayModal::Container)
            && world.contains_resource::<Messages<RequestStateTransition>>()
        {
            world.write_message(RequestStateTransition::Modal(GameplayModal::None));
        }
        info!(
            "corpse {} ({:08x}) closed",
            name, placement.reference_form_id
        );
    }
    opening
}

/// Issue #84 (F84.2): scripted (console/BRP) pickup, mirroring
/// `activate_focused_placement`'s `Pickup` arm minus the raycast focus and
/// distance-state handling -- the runtime-item branch (inventory add plus
/// `ActiveSaveState` dropped-item cleanup), the prepared-item branch
/// (`PreparedItemCatalog` `max_condition` lookup for Weapon/Apparel stats),
/// the #81 steal classification log, the pickup sound, and the notice all
/// carry over verbatim. Returns the number of items added, or an error if a
/// runtime item's persistence resource is not ready.
pub(crate) fn scripted_pickup(
    world: &mut World,
    entity: Entity,
) -> Result<i32, ScriptedPickupError> {
    let placement = world
        .get::<PlacementRoot>(entity)
        .expect("caller resolved a placement root")
        .placement()
        .clone();
    let runtime_item = world
        .get::<super::world_items::RuntimeWorldItem>(entity)
        .copied();
    let position = world
        .get::<GlobalTransform>(entity)
        .map(|transform| transform.translation())
        .unwrap_or_default();
    let name = placement_name(&placement);
    let count = placement.count.max(1);

    if let Some(runtime_item) = runtime_item {
        if !world.contains_resource::<super::world::ActiveSaveState>() {
            return Err(ScriptedPickupError::PersistenceNotReady);
        }
        let stack = runtime_item.stack;
        let before = world.resource::<PlayerInventory>().legacy_snapshot();
        {
            let mut canonical = world.resource_mut::<CanonicalItemLedger>();
            canonical
                .ensure_runtime_item(runtime_item.runtime_id, runtime_item.cell_form_id, stack)
                .map_err(|_| ScriptedPickupError::ItemTransactionFailed)?;
            canonical
                .move_runtime_to_player(&before, runtime_item.runtime_id, runtime_item.cell_form_id)
                .map_err(|_| ScriptedPickupError::ItemTransactionFailed)?;
        }
        let _ = world.resource_mut::<PlayerInventory>().add_stack(stack);
        let mut save_state = world.resource_mut::<super::world::ActiveSaveState>();
        if let Some(cell) = save_state.0.cells.get_mut(&runtime_item.cell_form_id) {
            cell.dropped_items.remove(&runtime_item.runtime_id);
            if cell.references.is_empty() && cell.dropped_items.is_empty() {
                save_state.0.cells.remove(&runtime_item.cell_form_id);
            }
        }
        info!(
            "retrieved runtime item {} from cell {:08x}",
            runtime_item.runtime_id, runtime_item.cell_form_id
        );
    } else {
        let condition = world
            .get_resource::<PreparedItemCatalog>()
            .into_iter()
            .flat_map(|catalog| &catalog.items)
            .find(|item| item.base_form_id == placement.base_form_id)
            .and_then(|item| match &item.stats {
                PreparedItemStats::Weapon { max_condition, .. }
                | PreparedItemStats::Apparel { max_condition, .. } => *max_condition,
                _ => None,
            });
        let stack = InventoryStack {
            base_form_id: placement.base_form_id,
            count,
            condition,
        };
        let before = world.resource::<PlayerInventory>().legacy_snapshot();
        world
            .resource_mut::<CanonicalItemLedger>()
            .add_player_item(&before, stack)
            .map_err(|_| ScriptedPickupError::ItemTransactionFailed)?;
        let _ = world.resource_mut::<PlayerInventory>().add_stack(stack);
    }
    // Issue #81 (F81.4): picking up an owned reference is theft; no
    // crime/karma consequences in M3, only the stable log line.
    if let item_rules::TakeClassification::Steal { owner_form_id } =
        item_rules::classify_take(placement.owner_form_id)
    {
        info!(
            "steal {:08x} owner {:08x}",
            placement.base_form_id, owner_form_id
        );
    }
    if let Some(form_id) = placement.audio.pickup_sound_form_id {
        world.write_message(PlaySound::pickup_at(form_id, position));
    }
    world
        .get_resource_or_insert_with(InteractionNotice::default)
        .show(format!("Picked up {name} x{count}"));
    info!(
        "picked up {} x{} ({:08x}); inventory now has {}",
        name,
        count,
        placement.base_form_id,
        world
            .resource::<PlayerInventory>()
            .count(placement.base_form_id)
    );
    world
        .get_resource_or_insert_with(InteractionState::default)
        .open
        .remove(&entity);
    world.despawn(entity);
    Ok(count)
}

/// Failure mode for [`scripted_pickup`]: a runtime (player-dropped) item was
/// targeted before `ActiveSaveState` exists, so the dropped-item cleanup that
/// keeps persistence consistent cannot run yet.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ScriptedPickupError {
    PersistenceNotReady,
    ItemTransactionFailed,
}

/// Attach this component to the root that owns a prepared placement's scene.
/// Mesh-ray hits are walked through `ChildOf` ancestors until this root is found.
#[derive(Component, Clone, Debug)]
pub(crate) struct PlacementRoot {
    placement: PreparedPlacement,
}

impl PlacementRoot {
    pub(crate) fn new(placement: PreparedPlacement) -> Self {
        Self { placement }
    }

    pub(crate) fn uses_quick_ao(&self) -> bool {
        self.placement.ao_mode == "ao-quick-v1"
    }

    pub(crate) fn placement(&self) -> &PreparedPlacement {
        &self.placement
    }
}

#[derive(Resource, Default, Debug)]
pub(crate) struct PlayerInventory(Inventory);

impl PlayerInventory {
    pub(crate) fn count(&self, form_id: u32) -> i32 {
        self.0.count(form_id)
    }

    pub(crate) fn contains(&self, form_id: u32) -> bool {
        self.0.contains(form_id)
    }

    pub(crate) fn add_stack(&mut self, stack: InventoryStack) -> TransferResult {
        self.0.add(stack)
    }

    pub(crate) fn remove(&mut self, key: StackKey, count: i32) -> TransferResult {
        self.0.remove(key, count)
    }

    pub(crate) fn available(&self, key: StackKey) -> i32 {
        self.0.available(key)
    }

    pub(crate) fn stack_states(&self) -> Vec<InventoryStack> {
        self.0.stacks()
    }

    pub(crate) fn legacy_snapshot(&self) -> Inventory {
        self.0.clone()
    }

    pub(crate) fn total_weight(&self, weight: impl FnMut(u32) -> Option<f32>) -> f32 {
        self.0.total_weight(weight)
    }

    /// Issue #60 (F60.4): rebuilds the inventory from a loaded save's
    /// player record.
    #[cfg(test)]
    pub(crate) fn from_stacks(stacks: impl IntoIterator<Item = (u32, i32)>) -> Self {
        Self(Inventory::from_stacks(stacks.into_iter().map(
            |(base_form_id, count)| InventoryStack {
                base_form_id,
                count,
                condition: None,
            },
        )))
    }

    pub(crate) fn from_stack_states(stacks: impl IntoIterator<Item = InventoryStack>) -> Self {
        Self(Inventory::from_stacks(stacks))
    }
}

/// Runtime adapter for the canonical #95 holder ledger. The legacy inventory
/// resource remains the Pip-Boy-facing projection during this migration; item
/// ownership and mutable state live in this ledger.
#[derive(Resource, Debug, Clone)]
pub(crate) struct CanonicalItemLedger {
    pub(crate) ledger: ItemLedger,
}

impl Default for CanonicalItemLedger {
    fn default() -> Self {
        Self {
            ledger: ItemLedger::new(),
        }
    }
}

/// Issue #98 (F98.2/F98.3): the authoritative equipped set. Thin `Resource`
/// wrapper around the pure `equipment::EquipmentState`, mirroring how
/// `PlayerInventory` wraps `Inventory`.
#[derive(Resource, Default, Debug, Clone)]
pub(crate) struct PlayerEquipment(EquipmentState);

impl PlayerEquipment {
    pub(crate) fn is_equipped(&self, key: StackKey) -> bool {
        self.0.is_equipped(key)
    }

    pub(crate) fn equipped_apparel(
        &self,
    ) -> impl Iterator<Item = (equipment::BipedSlot, StackKey)> + '_ {
        self.0.equipped_apparel()
    }

    pub(crate) fn equipped_weapon(&self) -> Option<StackKey> {
        self.0.equipped_weapon()
    }

    pub(crate) fn equipped_ammo(&self) -> Option<StackKey> {
        self.0.equipped_ammo()
    }

    pub(crate) fn toggle(
        &mut self,
        key: StackKey,
        kind: EquipKind,
    ) -> Result<EquipOutcome, EquipError> {
        self.0.toggle(key, kind)
    }

    pub(crate) fn equip(
        &mut self,
        key: StackKey,
        kind: EquipKind,
    ) -> Result<EquipOutcome, EquipError> {
        self.0.equip(key, kind)
    }

    pub(crate) fn unequip(&mut self, key: StackKey) -> EquipOutcome {
        self.0.unequip(key)
    }

    /// Issue #60/#98 (F98.4): rebuilds equipment from a loaded save's
    /// persisted entries.
    pub(crate) fn restore(
        apparel: impl IntoIterator<Item = (equipment::BipedSlot, StackKey)>,
        weapon: Option<(StackKey, Option<u32>)>,
        ammo: Option<StackKey>,
    ) -> Self {
        Self(EquipmentState::restore(apparel, weapon, ammo))
    }
}

/// F98.3: the biped-slot mask (apparel), required ammo form id (weapon), or
/// plain ammo-slot membership an item's `PreparedItemStats` maps to, if it
/// is equippable at all. Shared by the Pip-Boy equip toggle, hotkeys, and
/// the `player.equipitem` console command so all three equip through the
/// same classification.
pub(crate) fn equip_kind_for(item: &PreparedItemDefinition) -> Option<EquipKind> {
    match item.category {
        PreparedItemCategory::Weapons => {
            let ammo_form_id = match &item.stats {
                PreparedItemStats::Weapon { ammo_form_id, .. } => *ammo_form_id,
                _ => None,
            };
            Some(EquipKind::Weapon { ammo_form_id })
        }
        PreparedItemCategory::Apparel => {
            let biped_slot_mask = match &item.stats {
                PreparedItemStats::Apparel {
                    biped_slot_mask, ..
                } => biped_slot_mask.unwrap_or(0),
                _ => 0,
            };
            Some(EquipKind::Apparel { biped_slot_mask })
        }
        PreparedItemCategory::Ammo => Some(EquipKind::Ammo),
        _ => None,
    }
}

/// F98.3: written by `viewer::pipboy`'s equip-toggle row action and by
/// `viewer::bindings`' outside-Pip-Boy hotkey press; `apply_equip_toggle_requests`
/// is the single choke point that resolves the catalog entry, decides the
/// `EquipKind`, and mutates `PlayerEquipment`, so both callers get identical
/// behavior and notices.
#[derive(Message, Clone, Copy, Debug)]
pub(crate) struct EquipToggleRequested(pub(crate) StackKey);

fn apply_equip_toggle_requests(
    mut requests: MessageReader<EquipToggleRequested>,
    catalog: Res<PreparedItemCatalog>,
    inventory: Res<PlayerInventory>,
    mut equipment_state: ResMut<PlayerEquipment>,
    mut notice: ResMut<InteractionNotice>,
) {
    for request in requests.read() {
        let key = request.0;
        if !inventory
            .stack_states()
            .iter()
            .any(|stack| stack.key() == key)
        {
            notice.show("That item is not in your inventory");
            continue;
        }
        let Some(item) = catalog
            .items
            .iter()
            .find(|item| item.base_form_id == key.base_form_id)
        else {
            notice.show("That item has no prepared definition");
            continue;
        };
        let Some(kind) = equip_kind_for(item) else {
            notice.show("That item cannot be equipped");
            continue;
        };
        let label = item
            .display_name
            .clone()
            .or_else(|| item.editor_id.clone())
            .unwrap_or_else(|| format!("{:08X}", item.base_form_id));
        match equipment_state.toggle(key, kind) {
            Ok(_outcome) => {
                let now_equipped = equipment_state.is_equipped(key);
                notice.show(if now_equipped {
                    format!("Equipped {label}")
                } else {
                    format!("Unequipped {label}")
                });
            }
            Err(EquipError::NotEquippable) => notice.show("That item cannot be equipped"),
            Err(EquipError::IncompatibleAmmo) => {
                notice.show("That ammo does not fit the equipped weapon")
            }
            Err(EquipError::NoWeaponEquipped) => notice.show("Equip a weapon before loading ammo"),
        }
    }
}

impl CanonicalItemLedger {
    pub(crate) fn from_snapshot(snapshot: ItemLedgerSnapshot) -> Result<Self, TransactionError> {
        Ok(Self {
            ledger: ItemLedger::from_snapshot(snapshot)?,
        })
    }

    pub(crate) fn snapshot(&self) -> ItemLedgerSnapshot {
        self.ledger.snapshot()
    }

    pub(crate) fn player_stack_key(&self, item_id: ItemInstanceId) -> Option<StackKey> {
        self.ledger
            .holders()
            .get(&HolderId::Player)
            .and_then(|state| state.find(item_id))
            .map(|item| StackKey {
                base_form_id: item.base_form_id,
                condition: item.state.condition,
            })
    }

    pub(crate) fn player_legacy_snapshot(&self) -> Option<Inventory> {
        let state = self.ledger.holders().get(&HolderId::Player)?;
        let mut stacks = BTreeMap::<(u32, Option<u32>), i32>::new();
        for item in &state.items {
            let count = i32::try_from(item.count).unwrap_or(i32::MAX);
            let entry = stacks
                .entry((item.base_form_id, item.state.condition))
                .or_default();
            *entry = entry.saturating_add(count);
        }
        Some(Inventory::from_stacks(stacks.into_iter().map(
            |((base_form_id, condition), count)| InventoryStack {
                base_form_id,
                count,
                condition,
            },
        )))
    }

    pub(crate) fn sync_player(&mut self, inventory: &Inventory) -> Result<(), TransactionError> {
        if !self.ledger.holders().contains_key(&HolderId::Player) {
            self.ledger
                .insert_holder(HolderId::Player, ItemHolderState::default())?;
        }
        let mut desired = BTreeMap::new();
        for stack in inventory.stacks() {
            if stack.count > 0 {
                desired.insert(
                    (stack.base_form_id, stack.condition),
                    u32::try_from(stack.count).map_err(|_| TransactionError::InvalidCount)?,
                );
            }
        }
        let mut missing = desired.clone();
        if let Some(state) = self.ledger.holders_mut().get_mut(&HolderId::Player) {
            for item in &mut state.items {
                let remaining = missing
                    .entry((item.base_form_id, item.state.condition))
                    .or_default();
                let kept = item.count.min(*remaining);
                item.count = kept;
                *remaining -= kept;
            }
            state.items.retain(|item| item.count > 0);
            state.items.sort_by_key(|item| item.id);
            state.revision = state.revision.saturating_add(1);
        }
        for ((base_form_id, condition), count) in missing {
            if count > 0 {
                self.ledger.insert_new_item(
                    HolderId::Player,
                    base_form_id,
                    count,
                    ItemState {
                        condition,
                        ..Default::default()
                    },
                )?;
            }
        }
        Ok(())
    }

    pub(crate) fn add_player_item(
        &mut self,
        inventory: &Inventory,
        stack: InventoryStack,
    ) -> Result<ItemInstanceId, TransactionError> {
        self.sync_player(inventory)?;
        self.ledger.insert_new_item(
            HolderId::Player,
            stack.base_form_id,
            u32::try_from(stack.count).map_err(|_| TransactionError::InvalidCount)?,
            ItemState {
                condition: stack.condition,
                ..Default::default()
            },
        )
    }

    pub(crate) fn move_player_to_runtime(
        &mut self,
        inventory: &Inventory,
        runtime_id: u64,
        cell_form_id: u32,
        stack: InventoryStack,
    ) -> Result<(), TransactionError> {
        self.sync_player(inventory)?;
        let holder = HolderId::RuntimeWorld {
            cell_form_id,
            runtime_id,
        };
        self.ledger
            .insert_holder(holder, ItemHolderState::default())?;
        let item_id = self
            .ledger
            .holders()
            .get(&HolderId::Player)
            .and_then(|state| {
                state.items.iter().find(|item| {
                    item.base_form_id == stack.base_form_id
                        && item.state.condition == stack.condition
                        && item.count >= u32::try_from(stack.count).unwrap_or_default()
                })
            })
            .map(|item| item.id)
            .ok_or(TransactionError::InsufficientItems)?;
        self.ledger.execute(TransactionRequest::Transfer {
            source: HolderId::Player,
            destination: holder,
            item_id,
            count: u32::try_from(stack.count).map_err(|_| TransactionError::InvalidCount)?,
        })?;
        Ok(())
    }

    pub(crate) fn move_runtime_to_player(
        &mut self,
        inventory: &Inventory,
        runtime_id: u64,
        cell_form_id: u32,
    ) -> Result<i32, TransactionError> {
        self.sync_player(inventory)?;
        let source = HolderId::RuntimeWorld {
            cell_form_id,
            runtime_id,
        };
        let item = self
            .ledger
            .holders()
            .get(&source)
            .and_then(|state| state.items.first())
            .cloned()
            .ok_or(TransactionError::InsufficientItems)?;
        self.ledger.execute(TransactionRequest::Transfer {
            source,
            destination: HolderId::Player,
            item_id: item.id,
            count: item.count,
        })?;
        Ok(i32::try_from(item.count).unwrap_or(i32::MAX))
    }

    pub(crate) fn ensure_runtime_item(
        &mut self,
        runtime_id: u64,
        cell_form_id: u32,
        stack: InventoryStack,
    ) -> Result<(), TransactionError> {
        let holder = HolderId::RuntimeWorld {
            cell_form_id,
            runtime_id,
        };
        if !self.ledger.holders().contains_key(&holder) {
            self.ledger
                .insert_holder(holder, ItemHolderState::default())?;
            self.ledger.insert_new_item(
                holder,
                stack.base_form_id,
                u32::try_from(stack.count).map_err(|_| TransactionError::InvalidCount)?,
                ItemState {
                    condition: stack.condition,
                    ..Default::default()
                },
            )?;
        }
        Ok(())
    }

    pub(crate) fn ensure_container(
        &mut self,
        reference_form_id: u32,
        stacks: &[(u32, i32)],
    ) -> Result<(), TransactionError> {
        let holder = HolderId::FixtureContainer { reference_form_id };
        if self.ledger.holders().contains_key(&holder) {
            return Ok(());
        }
        self.ledger
            .insert_holder(holder, ItemHolderState::default())?;
        for &(base_form_id, count) in stacks {
            if count > 0 {
                self.ledger.insert_new_item(
                    holder,
                    base_form_id,
                    u32::try_from(count).map_err(|_| TransactionError::InvalidCount)?,
                    ItemState::default(),
                )?;
            }
        }
        Ok(())
    }

    pub(crate) fn set_merchant(
        &mut self,
        reference_form_id: u32,
        stacks: &[(u32, i32)],
        caps: u64,
    ) -> Result<(), TransactionError> {
        let holder = HolderId::FixtureMerchant { reference_form_id };
        if self.ledger.holders().contains_key(&holder) {
            return Err(TransactionError::InvalidMerchant);
        }
        self.ledger.insert_holder(
            holder,
            ItemHolderState {
                caps,
                ..Default::default()
            },
        )?;
        for &(base_form_id, count) in stacks {
            if count > 0 {
                self.ledger.insert_new_item(
                    holder,
                    base_form_id,
                    u32::try_from(count).map_err(|_| TransactionError::InvalidCount)?,
                    ItemState::default(),
                )?;
            }
        }
        Ok(())
    }
}

/// `open` is pub(crate) for issues #60/#61: `world::persist` captures
/// door/container open state on the way out of a cell and re-inserts it on
/// apply. Everything else stays private to this module.
#[derive(Resource, Default)]
pub(crate) struct InteractionState {
    focused: Option<Entity>,
    pub(crate) open: HashSet<Entity>,
}

/// Issue #59's notice seam: `world::swap`'s fallback-cancellation and
/// failure-recovery systems reach this same HUD line (`show`, made
/// `pub(crate)` for that) rather than inventing a second notice surface.
#[derive(Resource, Default)]
pub(crate) struct InteractionNotice {
    text: String,
    remaining_seconds: f32,
}

impl InteractionNotice {
    pub(crate) fn show(&mut self, text: impl Into<String>) {
        self.text = text.into();
        self.remaining_seconds = NOTICE_SECONDS;
    }

    /// Read-only view of the currently displayed notice text (issue #59's
    /// swap tests assert on it; nothing outside tests should).
    #[cfg(test)]
    pub(crate) fn text(&self) -> &str {
        &self.text
    }
}

/// Fixed seam (issue #75, wired at wave-2 integration into #76's persistence
/// capture/apply): a container's runtime inventory, keyed by
/// `reference_form_id` -- the same identity the manifest and console/BRP
/// paths already use for containers. `pub(crate)` (and the tuple field with
/// it) so `world::persist` can capture/apply it without this module growing
/// a second, parallel accessor API.
#[derive(Resource, Default)]
pub(crate) struct ContainerStates(pub(crate) HashMap<u32, container_policy::ContainerState>);

impl ContainerStates {
    /// F75.2/T75.2: opens (or reopens) a container's state. `resolve_leveled`
    /// is the #74 resolver seam -- called only when this reference has never
    /// resolved before; a `resolved` state short-circuits straight back
    /// without touching it (never re-rolls).
    fn open(
        &mut self,
        reference_form_id: u32,
        entries: &[container_policy::SeedEntry],
        resolve_leveled: impl FnMut(u32) -> Vec<(u32, i32)>,
    ) -> &container_policy::ContainerState {
        let existing = self.0.remove(&reference_form_id);
        let state = container_policy::open_container(existing, entries, resolve_leveled);
        self.0.entry(reference_form_id).or_insert(state)
    }

    pub(crate) fn get(&self, reference_form_id: u32) -> Option<&container_policy::ContainerState> {
        self.0.get(&reference_form_id)
    }

    fn get_mut(&mut self, reference_form_id: u32) -> Option<&mut container_policy::ContainerState> {
        self.0.get_mut(&reference_form_id)
    }
}

/// The transfer modal's holder kind. Containers and staged corpses share the
/// stack/persistence implementation but retain distinct stable logs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum LootHolderKind {
    Container,
    Corpse,
}

impl LootHolderKind {
    fn label(self) -> &'static str {
        match self {
            Self::Container => "container",
            Self::Corpse => "corpse",
        }
    }
}

/// F75.2/F118.2: which loot holder the transfer modal is showing, and the
/// best-effort item names collected from its prepared inventory at open
/// time (resolved-leveled or player-only items with no known name fall
/// back to their hex form id -- name/count is all this issue promises, see
/// the plan's non-goals).
pub(crate) struct ActiveContainer {
    kind: LootHolderKind,
    entity: Entity,
    reference_form_id: u32,
    name: String,
    item_names: HashMap<u32, String>,
    /// `XOWN` owner of the container reference (issue #81): taking from an
    /// owned container logs as theft.
    owner_form_id: Option<u32>,
}

#[derive(Resource, Default)]
pub(crate) struct ActiveContainerTarget(pub(crate) Option<ActiveContainer>);

#[derive(Component)]
struct InteractionPromptText;

#[derive(Component)]
struct InteractionNoticeText;

pub(crate) fn install(app: &mut App) {
    app.init_resource::<PlayerInventory>()
        .init_resource::<CanonicalItemLedger>()
        .init_resource::<PlayerEquipment>()
        // Issue #98: `apply_equip_toggle_requests` reads the item catalog;
        // `run_view` inserts the real one before installing this plugin
        // (`init_resource` never overwrites an existing resource), and bare
        // test harnesses get an empty default instead of a missing-resource
        // panic.
        .init_resource::<PreparedItemCatalog>()
        .init_resource::<InteractionState>()
        .init_resource::<InteractionNotice>()
        .init_resource::<PendingDoorTravel>()
        .init_resource::<ContainerStates>()
        .init_resource::<ActiveContainerTarget>()
        .add_message::<DoorTravelRequested>()
        .add_message::<EquipToggleRequested>()
        // F75.2: `activate_focused_placement` now also writes
        // `RequestStateTransition` to open the transfer modal.
        // `AppStatePlugin` registers this message too (`add_message` is
        // idempotent -- see `bevy_app::SubApp::add_message` -- so real apps
        // that already install `AppStatePlugin` are unaffected); this keeps
        // `interaction::install` self-sufficient for callers/tests (e.g.
        // `door_travel_animation`'s bare-`App` harness) that don't.
        .add_message::<RequestStateTransition>()
        .add_systems(Startup, spawn_interaction_ui)
        .add_systems(
            Update,
            (
                update_focused_placement,
                activate_focused_placement,
                tick_pending_door_travel,
            )
                .chain()
                .in_set(DoorActivationSet)
                .run_if(in_state(AppState::InGame))
                .run_if(in_state(GameplayModal::None)),
        )
        .add_systems(
            Update,
            probe_center_target
                .run_if(in_state(AppState::InGame))
                .run_if(in_state(GameplayModal::None)),
        )
        .add_systems(
            Update,
            (
                update_interaction_notice,
                cleanup_removed_placements,
                apply_equip_toggle_requests,
            ),
        );
    transfer_ui::install(app);
}

fn spawn_interaction_ui(mut commands: Commands) {
    commands.spawn((
        Text::new(""),
        InteractionPromptText,
        super::console::GameUi,
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            left: Val::Percent(50.0),
            top: Val::Percent(58.0),
            margin: UiRect::left(Val::Px(-140.0)),
            width: Val::Px(280.0),
            justify_content: JustifyContent::Center,
            ..default()
        },
        ZIndex(110),
    ));
    commands.spawn((
        Text::new(""),
        InteractionNoticeText,
        super::console::GameUi,
        TextColor(Color::srgb(1.0, 0.9, 0.5)),
        Node {
            position_type: PositionType::Absolute,
            left: Val::Percent(50.0),
            top: Val::Percent(63.0),
            margin: UiRect::left(Val::Px(-240.0)),
            width: Val::Px(480.0),
            justify_content: JustifyContent::Center,
            ..default()
        },
        ZIndex(110),
    ));
}

#[allow(clippy::too_many_arguments)]
fn update_focused_placement(
    time: Res<Time>,
    mode: Res<CameraModeState>,
    cameras: Query<(&Camera, &GlobalTransform), With<Camera3d>>,
    mut raycast: MeshRayCast,
    parents: Query<&ChildOf>,
    roots: Query<&PlacementRoot>,
    inventory: Res<PlayerInventory>,
    mut state: ResMut<InteractionState>,
    mut prompt: Query<&mut Text, With<InteractionPromptText>>,
    mut raycast_elapsed: Local<f32>,
) {
    if mode.mode != CameraMode::Fps {
        state.focused = None;
        if let Ok(mut prompt) = prompt.single_mut() {
            prompt.0.clear();
        }
        return;
    }
    *raycast_elapsed += time.delta_secs();
    if *raycast_elapsed < FOCUS_RAYCAST_INTERVAL_SECONDS {
        return;
    }
    *raycast_elapsed = 0.0;

    let focused = active_center_ray(&cameras).and_then(|ray| {
        let settings = MeshRayCastSettings {
            visibility: RayCastVisibility::VisibleInView,
            ..default()
        };
        let (hit_entity, hit) = raycast.cast_ray(ray, &settings).first()?;
        if hit.distance > INTERACTION_DISTANCE_METERS {
            return None;
        }
        let root_entity = find_placement_root(*hit_entity, &parents, &roots)?;
        let root = roots.get(root_entity).ok()?;
        interaction_prompt(
            &root.placement,
            state.open.contains(&root_entity),
            &inventory,
        )
        .map(|text| (root_entity, text))
    });

    state.focused = focused.as_ref().map(|(entity, _)| *entity);
    if let Ok(mut prompt) = prompt.single_mut() {
        prompt.0 = focused.map(|(_, text)| text).unwrap_or_default();
    }
}

fn active_center_ray(
    cameras: &Query<(&Camera, &GlobalTransform), With<Camera3d>>,
) -> Option<Ray3d> {
    cameras.iter().find_map(|(camera, transform)| {
        if !camera.is_active {
            return None;
        }
        let viewport = camera.logical_viewport_size()?;
        camera.viewport_to_world(transform, viewport * 0.5).ok()
    })
}

pub(crate) fn find_placement_root(
    mut entity: Entity,
    parents: &Query<&ChildOf>,
    roots: &Query<&PlacementRoot>,
) -> Option<Entity> {
    for _ in 0..MAX_PARENT_DEPTH {
        if roots.contains(entity) {
            return Some(entity);
        }
        entity = parents.get(entity).ok()?.parent();
    }
    warn!("placement hierarchy exceeded {MAX_PARENT_DEPTH} ancestors");
    None
}

fn cleanup_removed_placements(
    mut removed: RemovedComponents<PlacementRoot>,
    mut references: ResMut<RefRegistry>,
    mut sessions: ResMut<ConsoleSessionStore>,
) {
    for entity in removed.read() {
        references.unregister(entity);
        sessions.clear_entity(entity);
    }
}

fn probe_center_target(
    keys: Res<ButtonInput<KeyCode>>,
    cameras: Query<(&Camera, &GlobalTransform), With<Camera3d>>,
    mut raycast: MeshRayCast,
    parents: Query<&ChildOf>,
    roots: Query<&PlacementRoot>,
    references: Res<RefRegistry>,
) {
    if !keys.just_pressed(KeyCode::Enter) {
        return;
    }

    let Some(ray) = active_center_ray(&cameras) else {
        info!("{}", probe_status_message(false, None));
        return;
    };
    let settings = MeshRayCastSettings {
        visibility: RayCastVisibility::VisibleInView,
        ..default()
    };
    let Some((hit_entity, _)) = raycast.cast_ray(ray, &settings).first() else {
        info!("{}", probe_status_message(false, None));
        return;
    };
    let Some(root_entity) = find_placement_root(*hit_entity, &parents, &roots) else {
        info!("{}", probe_status_message(true, None));
        return;
    };
    if references.identity(root_entity).is_none() {
        info!("{}", probe_status_message(true, None));
        return;
    }
    let label = references.label(root_entity);
    info!("{}", probe_status_message(true, Some(&label)));
}

fn probe_status_message(hit: bool, label: Option<&str>) -> String {
    if !hit {
        "probe: no target".into()
    } else if let Some(label) = label {
        format!("probe: {label}")
    } else {
        "probe: NOT_IMPLEMENTED (static-batched geometry)".into()
    }
}

/// The #74 resolver's inputs, bundled so `activate_focused_placement` stays
/// under Bevy's 16-parameter system limit: which cell the roll happens in,
/// that cell's manifest (for the leveled-list bodies), and the playthrough
/// seed the roll derives from. All `Option` because they belong to the
/// `world`/`persist` slices — bare-App interaction tests run without them,
/// and a missing resource just resolves against defaults (cell 0, no list
/// bodies, seed 0).
#[derive(bevy::ecs::system::SystemParam)]
struct LeveledResolveContext<'w> {
    active_cell: Option<Res<'w, ActiveCell>>,
    resident_cells: Option<Res<'w, ResidentCells>>,
    playthrough_seed: Option<Res<'w, PlaythroughSeed>>,
}

/// Bundles the container-open resources so `activate_focused_placement`
/// stays under Bevy's 16-parameter system limit alongside wave-1's dropped
/// item / catalog params (see `PickupContext` and `LeveledResolveContext`
/// above for the same reasoning).
#[derive(bevy::ecs::system::SystemParam)]
struct ContainerActivation<'w> {
    states: ResMut<'w, ContainerStates>,
    active: ResMut<'w, ActiveContainerTarget>,
    modal_requests: MessageWriter<'w, RequestStateTransition>,
}

/// Bundles wave-1's dropped-item retrieval resources (save-state removal,
/// catalog condition lookup) for the same 16-parameter reason as
/// `ContainerActivation`.
#[derive(bevy::ecs::system::SystemParam)]
struct PickupContext<'w> {
    save_state: Option<ResMut<'w, super::world::ActiveSaveState>>,
    catalog: Option<Res<'w, PreparedItemCatalog>>,
}

/// Converts the manifest's leveled-list bodies into `leveled`'s std-only
/// mirror types (the same mirror pattern `persist_policy` uses toward
/// `save`; see `leveled`'s module doc).
fn leveled_lists_from_manifest(
    lists: &BTreeMap<u32, PreparedLeveledList>,
) -> BTreeMap<u32, leveled::PreparedLeveledList> {
    lists
        .iter()
        .map(|(form_id, list)| {
            (
                *form_id,
                leveled::PreparedLeveledList {
                    chance_none: list.chance_none,
                    flags: list.flags,
                    entries: list
                        .entries
                        .iter()
                        .map(|entry| leveled::PreparedLeveledEntry {
                            level: entry.level,
                            base_form_id: entry.base_form_id,
                            count: entry.count,
                        })
                        .collect(),
                },
            )
        })
        .collect()
}

/// Seeds one FormID-keyed loot holder through the existing container policy.
/// The returned pair is (stack_count, stacks) for deterministic scripted logs.
fn seed_loot_holder(
    states: &mut ContainerStates,
    placement: &PreparedPlacement,
    active_cell: u32,
    playthrough_seed: u64,
    leveled_lists: &BTreeMap<u32, leveled::PreparedLeveledList>,
) -> (usize, Vec<(u32, i32)>) {
    let seed_entries: Vec<container_policy::SeedEntry> = placement
        .inventory
        .iter()
        .map(|entry| container_policy::SeedEntry {
            base_form_id: entry.base_form_id,
            count: entry.count,
            leveled: entry.leveled,
        })
        .collect();
    let seed =
        leveled::LeveledSeed::derive(playthrough_seed, active_cell, placement.reference_form_id);
    let resolved = states.open(placement.reference_form_id, &seed_entries, |list_form_id| {
        leveled::resolve_leveled(list_form_id, leveled_lists, seed, PLAYER_LEVEL)
    });
    (resolved.stacks.len(), resolved.stacks.clone())
}
#[allow(clippy::too_many_arguments)]
fn activate_focused_placement(
    keys: Res<ButtonInput<KeyCode>>,
    mode: Res<CameraModeState>,
    mut commands: Commands,
    roots: Query<(
        &PlacementRoot,
        &GlobalTransform,
        Option<&super::world_items::RuntimeWorldItem>,
    )>,
    animated: Query<&animation::AnimatedPlacement>,
    mut inventory: ResMut<PlayerInventory>,
    mut canonical: ResMut<CanonicalItemLedger>,
    mut state: ResMut<InteractionState>,
    mut notice: ResMut<InteractionNotice>,
    mut sounds: MessageWriter<PlaySound>,
    mut door_travel: MessageWriter<DoorTravelRequested>,
    mut animation_playback: MessageWriter<animation::PlayPlacementAnimation>,
    mut pending_travel: ResMut<PendingDoorTravel>,
    mut container_activation: ContainerActivation,
    resolve_context: LeveledResolveContext,
    mut pickup_context: PickupContext,
) {
    if mode.mode != CameraMode::Fps || !keys.just_pressed(KeyCode::KeyE) {
        return;
    }
    let Some(entity) = state.focused else {
        return;
    };
    let Ok((root, transform, runtime_item)) = roots.get(entity) else {
        state.focused = None;
        return;
    };
    let placement = &root.placement;
    let position = transform.translation();
    let name = placement_name(placement);

    match &placement.semantic {
        PreparedSemantic::Pickup(_) => {
            let count = placement.count.max(1);
            if let Some(runtime_item) = runtime_item {
                let Some(save_state) = pickup_context.save_state.as_mut() else {
                    notice.show("Dropped-item persistence is not ready");
                    return;
                };
                let before = inventory.legacy_snapshot();
                if canonical
                    .ensure_runtime_item(
                        runtime_item.runtime_id,
                        runtime_item.cell_form_id,
                        runtime_item.stack,
                    )
                    .and_then(|_| {
                        canonical.move_runtime_to_player(
                            &before,
                            runtime_item.runtime_id,
                            runtime_item.cell_form_id,
                        )
                    })
                    .is_err()
                {
                    notice.show("Item transaction failed while retrieving the dropped item");
                    return;
                }
                let _ = inventory.add_stack(runtime_item.stack);
                if let Some(cell) = save_state.0.cells.get_mut(&runtime_item.cell_form_id) {
                    cell.dropped_items.remove(&runtime_item.runtime_id);
                    if cell.references.is_empty() && cell.dropped_items.is_empty() {
                        save_state.0.cells.remove(&runtime_item.cell_form_id);
                    }
                }
                info!(
                    "retrieved runtime item {} from cell {:08x}",
                    runtime_item.runtime_id, runtime_item.cell_form_id
                );
            } else {
                let condition = pickup_context
                    .catalog
                    .as_deref()
                    .into_iter()
                    .flat_map(|catalog| &catalog.items)
                    .find(|item| item.base_form_id == placement.base_form_id)
                    .and_then(|item| match &item.stats {
                        PreparedItemStats::Weapon { max_condition, .. }
                        | PreparedItemStats::Apparel { max_condition, .. } => *max_condition,
                        _ => None,
                    });
                let stack = InventoryStack {
                    base_form_id: placement.base_form_id,
                    count,
                    condition,
                };
                let before = inventory.legacy_snapshot();
                if canonical.add_player_item(&before, stack).is_err() {
                    notice.show("Item transaction failed while picking up the item");
                    return;
                }
                let _ = inventory.add_stack(stack);
            }
            // Issue #81 (F81.4): picking up an owned reference is theft; no
            // crime/karma consequences in M3, only the stable log line.
            // Player-dropped runtime items carry no owner.
            if let item_rules::TakeClassification::Steal { owner_form_id } =
                item_rules::classify_take(placement.owner_form_id)
            {
                info!(
                    "steal {:08x} owner {:08x}",
                    placement.base_form_id, owner_form_id
                );
            }
            write_pickup_sound(&mut sounds, placement.audio.pickup_sound_form_id, position);
            notice.show(format!("Picked up {name} x{count}"));
            info!(
                "picked up {} x{} ({:08x}); inventory now has {}",
                name,
                count,
                placement.base_form_id,
                inventory.count(placement.base_form_id)
            );
            state.focused = None;
            state.open.remove(&entity);
            commands.entity(entity).despawn();
        }
        PreparedSemantic::Container | PreparedSemantic::Corpse => {
            // Issue #75 (F75.2): activation now opens the paused transfer
            // modal rather than toggling a notice in place. `E` on an
            // already-open container is unreachable in practice --
            // `activate_focused_placement` only runs in `GameplayModal::None`
            // (see `install`), and opening a container leaves that state --
            // but a leftover `open` marker (e.g. restored by a future
            // persistence apply before the modal system observes it) is
            // guarded against rather than trusted.
            if state.open.contains(&entity) {
                return;
            }
            let active_cell = resolve_context
                .active_cell
                .as_ref()
                .map(|cell| cell.0)
                .unwrap_or_default();
            let leveled_lists = if placement.inventory.iter().any(|entry| entry.leveled) {
                resolve_context
                    .resident_cells
                    .as_ref()
                    .and_then(|cells| cells.0.get(&active_cell))
                    .map(|resident| leveled_lists_from_manifest(&resident.manifest.leveled_lists))
                    .unwrap_or_default()
            } else {
                BTreeMap::new()
            };
            let stack_data = seed_loot_holder(
                &mut container_activation.states,
                placement,
                active_cell,
                resolve_context
                    .playthrough_seed
                    .as_ref()
                    .map(|seed| seed.0)
                    .unwrap_or_default(),
                &leveled_lists,
            );
            state.open.insert(entity);
            container_activation.active.0 = Some(ActiveContainer {
                kind: match &placement.semantic {
                    PreparedSemantic::Container => LootHolderKind::Container,
                    PreparedSemantic::Corpse => LootHolderKind::Corpse,
                    _ => unreachable!("guarded loot-holder activation"),
                },
                entity,
                reference_form_id: placement.reference_form_id,
                name: name.clone(),
                item_names: container_item_names(&placement.inventory),
                owner_form_id: placement.owner_form_id,
            });
            write_container_sound(&mut sounds, placement.audio.open_sound_form_id, position);
            animation_playback.write(animation::PlayPlacementAnimation {
                root: entity,
                transition: ClipTransition::Opening,
                lead_ms: 0.0,
            });
            container_activation
                .modal_requests
                .write(RequestStateTransition::Modal(GameplayModal::Container));
            info!(
                "{} {} ({:08x}) opened with {} stacks",
                match &placement.semantic {
                    PreparedSemantic::Container => "container",
                    PreparedSemantic::Corpse => "corpse",
                    _ => unreachable!("guarded loot-holder activation"),
                },
                name,
                placement.reference_form_id,
                stack_data.0
            );
        }
        PreparedSemantic::Door(door) => {
            if door_is_locked(door, &inventory) {
                notice.show(format!("{name} is locked"));
                info!(
                    "door {} ({:08x}) is locked; key {:?}",
                    name, placement.reference_form_id, door.key_form_id
                );
                return;
            }
            let opening = !state.open.contains(&entity);
            let transition = if opening {
                state.open.insert(entity);
                write_sound(&mut sounds, placement.audio.open_sound_form_id, position);
                ClipTransition::Opening
            } else {
                state.open.remove(&entity);
                write_sound(&mut sounds, placement.audio.close_sound_form_id, position);
                // F57.4: closing before this door's own open-lead elapses
                // cancels the still-pending travel rather than letting a
                // stale swap fire after the player has already reversed
                // course.
                if pending_travel
                    .0
                    .as_ref()
                    .is_some_and(|pending| pending.entity == entity)
                {
                    pending_travel.0 = None;
                }
                ClipTransition::Closing
            };
            notice.show(format!(
                "{} {name}",
                if opening { "Opened" } else { "Closed" }
            ));
            info!(
                "door {} ({:08x}) {}{}",
                name,
                placement.reference_form_id,
                if opening { "opened" } else { "closed" },
                if door.destination.is_some() {
                    "; travel requested"
                } else {
                    ""
                }
            );
            // Issue #57: a travel door's Open clip gets a lead -- computed
            // from `AnimatedPlacement`'s discovered "Open" clip duration, if
            // any -- before `DoorTravelRequested` fires, so the door is
            // visibly open before the (already instant) cell swap. No clip
            // means zero lead: `world::swap` sees the message this same
            // frame, exactly like wave 2.
            let lead_seconds = if opening && door.destination.is_some() {
                let open_clip_seconds = animated
                    .get(entity)
                    .ok()
                    .and_then(|animated| animated.clip_seconds("Open"));
                animation::open_lead_seconds(open_clip_seconds, animation::OPEN_LEAD_CAP_SECONDS)
            } else {
                0.0
            };
            animation_playback.write(animation::PlayPlacementAnimation {
                root: entity,
                transition,
                lead_ms: lead_seconds * 1000.0,
            });
            // Issue #52: entering (opening) a door with a resolved
            // destination requests a cell swap; `world::swap` decides
            // instant vs. loading-screen fallback from cell residency.
            if opening && let Some(destination) = &door.destination {
                let request = DoorTravelRequested {
                    destination_cell_form_id: destination.cell_form_id,
                    translation: Vec3::from_array(destination.translation),
                    rotation_xyzw: destination.rotation_xyzw,
                };
                if lead_seconds <= 0.0 {
                    door_travel.write(request);
                } else {
                    pending_travel.0 = Some(PendingTravel {
                        entity,
                        remaining_seconds: lead_seconds,
                        request,
                    });
                }
            }
        }
        PreparedSemantic::Activator => {
            write_sound(
                &mut sounds,
                placement.audio.activate_sound_form_id,
                position,
            );
            notice.show(format!("Activated {name}"));
            info!("activated {} ({:08x})", name, placement.reference_form_id);
            animation_playback.write(animation::PlayPlacementAnimation {
                root: entity,
                transition: ClipTransition::Opening,
                lead_ms: 0.0,
            });
        }
        _ => {}
    }
}

fn write_sound(sounds: &mut MessageWriter<PlaySound>, form_id: Option<u32>, position: Vec3) {
    if let Some(form_id) = form_id {
        sounds.write(PlaySound::at(form_id, position));
    }
}

fn write_pickup_sound(sounds: &mut MessageWriter<PlaySound>, form_id: Option<u32>, position: Vec3) {
    if let Some(form_id) = form_id {
        sounds.write(PlaySound::pickup_at(form_id, position));
    }
}

fn write_container_sound(
    sounds: &mut MessageWriter<PlaySound>,
    form_id: Option<u32>,
    position: Vec3,
) {
    if let Some(form_id) = form_id {
        sounds.write(PlaySound::container_at(form_id, position));
    }
}

fn update_interaction_notice(
    time: Res<Time>,
    mut notice: ResMut<InteractionNotice>,
    mut text: Query<&mut Text, With<InteractionNoticeText>>,
) {
    if notice.remaining_seconds > 0.0 {
        notice.remaining_seconds = (notice.remaining_seconds - time.delta_secs()).max(0.0);
        if notice.remaining_seconds == 0.0 {
            notice.text.clear();
        }
    }
    if let Ok(mut text) = text.single_mut() {
        text.0.clone_from(&notice.text);
    }
}

fn interaction_prompt(
    placement: &PreparedPlacement,
    is_open: bool,
    inventory: &PlayerInventory,
) -> Option<String> {
    let name = placement_name(placement);
    match &placement.semantic {
        PreparedSemantic::Pickup(_) => Some(format!(
            "[E] Take {name}{}",
            if placement.count > 1 {
                format!(" x{}", placement.count)
            } else {
                String::new()
            }
        )),
        PreparedSemantic::Container => Some(format!(
            "[E] {} {name}",
            if is_open { "Close" } else { "Open" }
        )),
        PreparedSemantic::Corpse => Some(format!(
            "[E] {} {name}",
            if is_open { "Close" } else { "Loot" }
        )),
        PreparedSemantic::Door(door) => {
            if door_is_locked(door, inventory) {
                Some(format!("[E] {name} (Locked)"))
            } else {
                Some(format!(
                    "[E] {} {name}",
                    if is_open { "Close" } else { "Open" }
                ))
            }
        }
        PreparedSemantic::Activator => Some(format!("[E] Activate {name}")),
        _ => None,
    }
}

fn placement_name(placement: &PreparedPlacement) -> String {
    placement
        .display_name
        .as_deref()
        .or(placement.editor_id.as_deref())
        .map(str::to_owned)
        .unwrap_or_else(|| format!("{:08x}", placement.base_form_id))
}

/// Whether `door` is currently locked and the player cannot bypass it with
/// a held key. `pub(crate)` (not `fn`) so `viewer::nav` can reuse this exact
/// check for excluding blocked door links from route planning (issue #113,
/// M4 wave 4 feature 4) rather than re-deriving its own copy.
pub(crate) fn door_is_locked(door: &PreparedDoor, inventory: &PlayerInventory) -> bool {
    if door.lock_level.is_none_or(|level| level <= 0) {
        return false;
    }
    door.key_form_id
        .is_none_or(|key_form_id| !inventory.contains(key_form_id))
}

fn entry_display_name(entry: &PreparedInventoryEntry) -> String {
    entry
        .display_name
        .as_deref()
        .or(entry.editor_id.as_deref())
        .map(str::to_owned)
        .unwrap_or_else(|| format!("{:08x}", entry.base_form_id))
}

fn inventory_summary(entries: &[PreparedInventoryEntry]) -> String {
    if entries.is_empty() {
        return "empty".into();
    }
    const DISPLAY_LIMIT: usize = 8;
    let mut summary = entries
        .iter()
        .take(DISPLAY_LIMIT)
        .map(|entry| format!("{} x{}", entry_display_name(entry), entry.count))
        .collect::<Vec<_>>()
        .join(", ");
    if entries.len() > DISPLAY_LIMIT {
        summary.push_str(&format!(", +{} more", entries.len() - DISPLAY_LIMIT));
    }
    summary
}

/// F75.2: best-effort `base_form_id -> name` lookup for the transfer modal,
/// built from a container's fixed (non-leveled) prepared inventory entries
/// at open time. Leveled-resolved items and anything already in the
/// player's inventory from elsewhere have no name source here -- the
/// transfer UI falls back to the hex form id for those, matching this
/// issue's "name + count is enough" scope.
fn container_item_names(entries: &[PreparedInventoryEntry]) -> HashMap<u32, String> {
    entries
        .iter()
        .map(|entry| (entry.base_form_id, entry_display_name(entry)))
        .collect()
}

#[cfg(test)]
#[path = "interaction/tests/mod.rs"]
mod tests;
