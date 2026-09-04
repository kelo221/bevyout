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
use super::crime;
use super::inventory::{Inventory, InventoryStack, StackKey, TransferResult};
use super::player::equipment::{self, EquipError, EquipKind, EquipOutcome, EquipmentState};
use super::player::{CameraMode, CameraModeState};
use super::world::{ActiveCell, PlaythroughSeed, ResidentCells};

// Pure std/serde-only modules (no Bevy imports), so `tests/features.rs` can
// pull them in verbatim via `#[path]`; see their module doc comments for
// the pattern (issue #74 resolver, issue #75 transfer policy).
mod activation;
pub(crate) mod container_policy;
mod door;
mod focus;
pub(crate) mod item_rules;
pub(crate) mod item_use;
mod items;
pub(crate) mod leveled;
mod presentation;
mod scripted;
mod state;
mod transfer_ui;
mod ui;

use activation::{activate_focused_placement, leveled_lists_from_manifest, seed_loot_holder};
pub(crate) use door::{DoorActivationSet, DoorTravelRequested};
use door::{PendingDoorTravel, PendingTravel, tick_pending_door_travel};
pub(crate) use focus::find_placement_root;
#[cfg(test)]
use focus::probe_status_message;
use focus::{cleanup_removed_placements, probe_center_target, update_focused_placement};
pub(crate) use items::{
    CanonicalItemLedger, EquipToggleRequested, PlayerEquipment, PlayerInventory, equip_kind_for,
};
pub(crate) use presentation::door_is_locked;
use presentation::{
    container_item_names, interaction_prompt, inventory_summary, placement_name,
    update_interaction_notice,
};
pub(crate) use scripted::{
    ScriptedPickupError, scripted_activator_toggle, scripted_container_toggle,
    scripted_corpse_toggle, scripted_door_open, scripted_door_toggle, scripted_door_travel,
    scripted_pickup,
};
pub(crate) use state::{
    ActiveContainer, ActiveContainerTarget, ContainerStates, InteractionNotice, InteractionState,
    LootHolderKind, PlacementRoot,
};
use state::{InteractionNoticeText, InteractionPromptText};
use ui::spawn_interaction_ui;

pub(crate) const INTERACTION_DISTANCE_METERS: f32 = 3.0;
/// No leveling system exists yet (later M3+ scope): leveled lists resolve
/// against a fixed player level until one does.
const PLAYER_LEVEL: u16 = 1;
const NOTICE_SECONDS: f32 = 3.0;
const FOCUS_RAYCAST_INTERVAL_SECONDS: f32 = 0.1;
const MAX_PARENT_DEPTH: usize = 64;

pub(crate) struct InteractionPlugin;

impl Plugin for InteractionPlugin {
    fn build(&self, app: &mut App) {
        install(app);
    }
}

fn install(app: &mut App) {
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
            OnEnter(GameplayModal::Dialogue),
            ui::clear_interaction_prompt,
        )
        .add_systems(
            Update,
            (
                update_focused_placement,
                activate_focused_placement,
                tick_pending_door_travel,
            )
                .chain()
                .in_set(DoorActivationSet)
                .in_set(super::plugins::ViewerSet::Interaction)
                .run_if(in_state(AppState::InGame))
                .run_if(in_state(GameplayModal::None)),
        )
        .add_systems(
            Update,
            probe_center_target
                .in_set(super::plugins::ViewerSet::Interaction)
                .run_if(in_state(AppState::InGame))
                .run_if(in_state(GameplayModal::None)),
        )
        .add_systems(
            Update,
            (
                update_interaction_notice,
                cleanup_removed_placements,
                items::apply_equip_toggle_requests,
            )
                .in_set(super::plugins::ViewerSet::Interaction),
        );
    transfer_ui::install(app);
}

#[cfg(test)]
#[path = "interaction/tests/mod.rs"]
mod tests;
