//! Viewer-dependent console commands and UI visibility markers.

use std::path::PathBuf;

use bevy::core_pipeline::tonemapping::Tonemapping;
use bevy::pbr::PointLightShadowSamples;
use bevy::post_process::bloom::Bloom;
use bevy::prelude::*;
use bevy::render::view::screenshot::{Screenshot, save_to_disk};
use bevy::window::PrimaryWindow;
use bevy_boxddd::prelude::BoxdddDebugDrawSettings;
use serde_json::{Map, Value, json};

use crate::app_state::GameplayModal;
use crate::console::{
    ConsoleCommand, ConsoleCommandResult, ConsoleEntityHooks, ConsoleError, ConsoleInvocation,
    ConsoleRegistry, resolve_reference,
};
use crate::item_transaction::{HolderId, ItemInstanceId, TransactionRequest};
use crate::vsa::{PreparedItemCatalog, PreparedItemStats, PreparedSemantic};

use super::controls::{
    AmbientScale, AoStrength, FogStrength, HorizontalFov, IrradianceIntensity, LightingScale,
    LightsDisabled, MAX_HORIZONTAL_FOV_DEGREES, MIN_HORIZONTAL_FOV_DEGREES, UnlitMode,
    horizontal_to_vertical_fov,
};
use super::inventory::{InventoryStack, StackKey};
#[cfg(test)]
use super::lighting::PreparedPointShadowRuntime;
use super::lighting::{RealtimeShadowSettings, shadow_cache_status};
use super::{diagnostics, interaction, nav, nav_overlay, player};

#[derive(Component)]
pub(crate) struct GameUi;

#[derive(Component)]
pub(crate) struct DiagnosticUi;

#[derive(Resource, Clone, Copy, Debug)]
struct GameUiState {
    visible: bool,
}

impl Default for GameUiState {
    fn default() -> Self {
        Self { visible: true }
    }
}

#[derive(Resource, Clone, Copy, Debug)]
struct DiagnosticUiState {
    visible: bool,
}

impl Default for DiagnosticUiState {
    fn default() -> Self {
        Self { visible: true }
    }
}

pub(crate) fn install(app: &mut App) {
    app.init_resource::<GameUiState>()
        .init_resource::<DiagnosticUiState>()
        .init_resource::<RealtimeShadowSettings>()
        .init_resource::<nav_overlay::NavMeshOverlayState>()
        .add_systems(
            Update,
            (sync_ui_visibility, nav_overlay::despawn_stale_nav_overlay),
        );
    {
        let mut hooks = app.world_mut().resource_mut::<ConsoleEntityHooks>();
        hooks.register_transform_mutated(player::console_transform_mutated);
        hooks.register_angle_adapter(player::console_get_angles, player::console_set_angles);
    }
    let mut registry = app.world_mut().resource_mut::<ConsoleRegistry>();
    for command in [
        ConsoleCommand::new(
            "tcl",
            "tcl",
            "Toggle FPS-player collision and gravity while preserving movement.",
            toggle_collision,
        )
        .mutating(),
        ConsoleCommand::new(
            "tfc",
            "tfc",
            "Toggle between the FPS player and free-fly camera.",
            toggle_fly_camera,
        )
        .aliases(&["toggleflycam"])
        .mutating(),
        ConsoleCommand::new(
            "fov",
            "fov [10..170]",
            "Get or set the horizontal camera field of view in degrees.",
            field_of_view,
        )
        .mutating(),
        ConsoleCommand::new(
            "tlights",
            "tlights",
            "Toggle all runtime scene lights.",
            toggle_lights,
        )
        .mutating(),
        ConsoleCommand::new(
            "tcg",
            "tcg",
            "Toggle collision geometry diagnostics.",
            toggle_collision_geometry,
        )
        .aliases(&["togglecollisiongeometry"])
        .mutating(),
        ConsoleCommand::new(
            "tnm",
            "tnm",
            "Toggle navigation-mesh triangle visualization.",
            nav_overlay::toggle_nav_mesh,
        )
        .aliases(&["togglenavmesh"])
        .mutating(),
        ConsoleCommand::new(
            "tna",
            "tna spawn|goto <x> <y> <z>|goto player|travel <door-formid>|status|despawn",
            "Test nav agent (issues #112/#134): spawn/goto/travel/status/despawn a bevy_landmass-driven agent with intercell handoff.",
            nav::agent::tna_command,
        )
        .mutating(),
        ConsoleCommand::new(
            "ragdoll",
            "ragdoll <actor-reference> [on|off|reset]",
            "Toggle a prepared NPC/creature's developer ragdoll body; actors stay locked in T-pose by default.",
            ragdoll,
        )
        .mutating(),
        ConsoleCommand::new(
            "stairdebug",
            "stairdebug",
            "Toggle stair-step rejection logging.",
            toggle_stair_debug,
        )
        .mutating(),
        ConsoleCommand::new(
            "tunlit",
            "tunlit",
            "Toggle unlit material diagnostics.",
            toggle_unlit,
        )
        .mutating(),
        ConsoleCommand::new(
            "getrender",
            "getrender [setting]",
            "Get one render setting or all render settings.",
            get_render,
        ),
        ConsoleCommand::new(
            "setrender",
            "setrender <setting> <value>",
            "Set a validated render setting.",
            set_render,
        )
        .mutating(),
        ConsoleCommand::new(
            "tonemap",
            "tonemap [mode]",
            "Get or set the active camera tonemapper.",
            tonemap,
        )
        .mutating(),
        ConsoleCommand::new(
            "renderreport",
            "renderreport",
            "Write the configured render timing and diagnostic reports immediately.",
            render_report,
        )
        .mutating(),
        ConsoleCommand::new(
            "shadowcache",
            "shadowcache <status|rebuild>",
            "Inspect the prepared point-shadow artifact or show rebuild instructions.",
            shadow_cache,
        )
        .mutating(),
        ConsoleCommand::new(
            "tm",
            "tm",
            "Toggle gameplay UI; the open console remains visible.",
            toggle_game_ui,
        )
        .mutating(),
        ConsoleCommand::new(
            "tdt",
            "tdt",
            "Toggle diagnostic UI entities.",
            toggle_diagnostic_ui,
        )
        .mutating(),
        ConsoleCommand::new(
            "sgtm",
            "sgtm <0.01..100>",
            "Set Time<Virtual> relative speed without changing pause state.",
            set_global_time_multiplier,
        )
        .mutating(),
        ConsoleCommand::new(
            "screenshot",
            "screenshot [name]",
            "Save the primary window under .bevyout/screenshots using a sanitized name.",
            screenshot,
        )
        .mutating(),
        ConsoleCommand::new(
            "activate",
            "activate <reference>",
            "Activate a door, container, corpse, or pickup reference; a door with a destination requests cell travel (locks bypassed).",
            activate_reference,
        )
        .mutating(),
        ConsoleCommand::new(
            "additem",
            "[player.]additem <FormID> [count]",
            "Add count (default 1) of an item FormID to the player inventory.",
            add_item,
        )
        .reference_callable(false)
        .mutating(),
        ConsoleCommand::new(
            "equip",
            "[player.]equip <ItemInstanceId>",
            "Equip a canonical player item by stable instance id.",
            equip_item,
        )
        .reference_callable(false)
        .mutating(),
        ConsoleCommand::new(
            "equipitem",
            "[player.]equipitem <FormID>",
            "Equip (or unequip, if already equipped) an item FormID already in the player inventory.",
            equip_item_formid,
        )
        .reference_callable(false)
        .mutating(),
        ConsoleCommand::new(
            "unequip",
            "[player.]unequip",
            "Unequip the canonical player item.",
            unequip_item,
        )
        .reference_callable(false)
        .mutating(),
        ConsoleCommand::new(
            "hotkey",
            "[player.]hotkey <0..7> <ItemInstanceId>",
            "Bind a stable canonical item id to a player hotkey slot.",
            bind_hotkey,
        )
        .reference_callable(false)
        .mutating(),
        ConsoleCommand::new(
            "useitem",
            "[player.]useitem <ItemInstanceId>",
            "Consume one unit through the canonical item-use seam.",
            use_item,
        )
        .reference_callable(false)
        .mutating(),
        ConsoleCommand::new(
            "setmerchant",
            "setmerchant <container-reference> <caps>",
            "Mark a prepared static container as a merchant with fixed caps.",
            set_merchant,
        )
        .mutating(),
        ConsoleCommand::new(
            "buy",
            "buy <merchant-reference> <ItemInstanceId> [count]",
            "Buy a fixed-price item from a prepared static merchant.",
            buy_item,
        )
        .mutating(),
        ConsoleCommand::new(
            "sell",
            "sell <merchant-reference> <ItemInstanceId> [count]",
            "Sell a fixed-price item to a prepared static merchant.",
            sell_item,
        )
        .mutating(),
        ConsoleCommand::new(
            "save",
            "save <slot>",
            "Capture the active cell state and write it to a named save slot.",
            save_slot,
        )
        .mutating(),
    ] {
        registry
            .register(command)
            .expect("viewer console command is unique");
    }
}

/// Scripted door activation for the agent bridge (M2 wave 2 acceptance):
/// resolves a door reference and requests the same `DoorTravelRequested`
/// cell travel that the player's Enter activation produces, skipping the
/// raycast focus, distance, and lock checks (this is a developer command).
fn activate_reference(
    world: &mut World,
    invocation: &ConsoleInvocation,
) -> Result<ConsoleCommandResult, ConsoleError> {
    let [selector] = invocation.args.as_slice() else {
        return Err(ConsoleError::new(
            "bad_arity",
            "activate requires exactly one reference selector",
        ));
    };
    let entity = resolve_reference(world, selector)?;
    let placement = world
        .get::<interaction::PlacementRoot>(entity)
        .ok_or_else(|| ConsoleError::new("not_activatable", "reference has no placement root"))?
        .placement()
        .clone();
    // Wave-4 amendment: containers toggle through the same open-state and
    // clip path as player activation, so the #60/#61 persistence gate can
    // be driven over the agent bridge.
    if matches!(placement.semantic, PreparedSemantic::Corpse) {
        let opened = interaction::scripted_corpse_toggle(world, entity);
        return Ok(ConsoleCommandResult::new(
            json!({
                "reference_form_id": placement.reference_form_id,
                "kind": "corpse",
                "opened": opened,
            }),
            vec![format!(
                "corpse {:08x} {}",
                placement.reference_form_id,
                if opened { "opened" } else { "closed" }
            )],
        ));
    }
    if matches!(placement.semantic, PreparedSemantic::Container) {
        let opened = interaction::scripted_container_toggle(world, entity);
        return Ok(ConsoleCommandResult::new(
            json!({
                "reference_form_id": placement.reference_form_id,
                "opened": opened,
            }),
            vec![format!(
                "container {:08x} {}",
                placement.reference_form_id,
                if opened { "opened" } else { "closed" }
            )],
        ));
    }
    // Issue #84 (F84.2): pickups activate through the same seam as door and
    // container references, mirroring the player's `E` pickup minus the
    // raycast focus and distance-state handling.
    if matches!(placement.semantic, PreparedSemantic::Pickup(_)) {
        let count = interaction::scripted_pickup(world, entity).map_err(|error| match error {
            interaction::ScriptedPickupError::PersistenceNotReady => ConsoleError::new(
                "persistence_not_ready",
                "dropped-item persistence is not ready",
            ),
            interaction::ScriptedPickupError::ItemTransactionFailed => ConsoleError::new(
                "item_transaction_failed",
                "canonical item transaction could not be committed",
            ),
        })?;
        return Ok(ConsoleCommandResult::new(
            json!({
                "reference_form_id": placement.reference_form_id,
                "base_form_id": placement.base_form_id,
                "count": count,
            }),
            vec![format!("picked up {:08x} x{count}", placement.base_form_id)],
        ));
    }
    let PreparedSemantic::Door(door) = &placement.semantic else {
        return Err(ConsoleError::new(
            "not_a_door",
            "activate supports only door, container, corpse, and pickup references",
        ));
    };
    let Some(destination) = &door.destination else {
        return Err(ConsoleError::new(
            "no_destination",
            "door has no travel destination",
        ));
    };
    // Wave-3 amendment: route through the same Open-clip lead as player
    // activation so BRP-driven travel animates instead of teleporting.
    let open_lead_ms = interaction::scripted_door_travel(
        world,
        entity,
        interaction::DoorTravelRequested {
            destination_cell_form_id: destination.cell_form_id,
            translation: Vec3::from_array(destination.translation),
            rotation_xyzw: destination.rotation_xyzw,
            door_form_id: placement.reference_form_id,
        },
    );
    Ok(ConsoleCommandResult::new(
        json!({
            "reference_form_id": placement.reference_form_id,
            "destination_cell_form_id": destination.cell_form_id,
            "open_lead_ms": open_lead_ms,
        }),
        vec![format!(
            "travel requested to cell {:08x} (open lead {open_lead_ms:.0} ms)",
            destination.cell_form_id
        )],
    ))
}

fn ragdoll(
    world: &mut World,
    invocation: &ConsoleInvocation,
) -> Result<ConsoleCommandResult, ConsoleError> {
    if !(1..=2).contains(&invocation.args.len()) {
        return Err(ConsoleError::new(
            "bad_arity",
            "ragdoll requires an actor reference and optional on, off, or reset",
        ));
    }
    let entity = resolve_reference(world, &invocation.args[0])?;
    let placement = world
        .get::<interaction::PlacementRoot>(entity)
        .ok_or_else(|| ConsoleError::new("not_actor", "reference has no placement root"))?
        .placement()
        .clone();
    if !matches!(
        placement.semantic,
        PreparedSemantic::Npc(_) | PreparedSemantic::Creature(_)
    ) {
        return Err(ConsoleError::new(
            "not_actor",
            "ragdoll only accepts NPC or creature references",
        ));
    }
    let operation = invocation
        .args
        .get(1)
        .map(|value| value.to_ascii_lowercase())
        .unwrap_or_else(|| "toggle".into());
    match operation.as_str() {
        "on" => {
            world.entity_mut(entity).insert(player::RagdollToggle(true));
        }
        "off" => {
            world
                .entity_mut(entity)
                .insert(player::RagdollToggle(false));
        }
        "reset" => {
            world.entity_mut(entity).remove::<player::RagdollToggle>();
            if let Some(mut transform) = world.get_mut::<Transform>(entity) {
                transform.translation = Vec3::from_array(placement.translation);
                transform.rotation = Quat::from_xyzw(
                    placement.rotation_xyzw[0],
                    placement.rotation_xyzw[1],
                    placement.rotation_xyzw[2],
                    placement.rotation_xyzw[3],
                );
                transform.scale = Vec3::splat(placement.scale);
            }
        }
        "toggle" => {
            let enabled = world
                .get::<player::RagdollToggle>(entity)
                .is_some_and(|toggle| toggle.0);
            world
                .entity_mut(entity)
                .insert(player::RagdollToggle(!enabled));
        }
        _ => {
            return Err(ConsoleError::new(
                "bad_value",
                "ragdoll mode must be on, off, or reset",
            ));
        }
    }
    let enabled = world
        .get::<player::RagdollToggle>(entity)
        .is_some_and(|toggle| toggle.0);
    Ok(ConsoleCommandResult::new(
        json!({
            "reference_form_id": placement.reference_form_id,
            "enabled": enabled,
            "mode": operation,
        }),
        vec![format!(
            "ragdoll {:08x} {}",
            placement.reference_form_id,
            if enabled { "enabled" } else { "disabled" }
        )],
    ))
}

/// Parses a raw item FormID argument: 1..=8 hex digits, with or without a
/// `0x` prefix -- the Bethesda console accepts bare short hex like
/// `player.additem f 100`. Deliberately looser than `console::executor`'s
/// private reference-selector parser (which requires a prefix or exactly 8
/// digits to disambiguate from EditorIDs): `additem` targets a catalog item
/// FormID rather than a live reference, so there is no EditorID form to
/// collide with and no `RefRegistry` resolution.
fn parse_item_form_id(value: &str) -> Option<u32> {
    let digits = value
        .strip_prefix("0x")
        .or_else(|| value.strip_prefix("0X"))
        .unwrap_or(value);
    ((1..=8).contains(&digits.len()) && digits.bytes().all(|byte| byte.is_ascii_hexdigit()))
        .then(|| u32::from_str_radix(digits, 16).ok())
        .flatten()
}

/// Issue #84 (F84.1): `additem`/`player.additem <FormID> [count]` always
/// targets the player inventory -- the Bethesda-console `player.` prefix is
/// accepted (`.reference_callable(false)`) but never used to pick a
/// container, matching real `additem`'s player-only scope. Condition is
/// seeded from `PreparedItemCatalog`'s `max_condition` for Weapon/Apparel
/// stats, exactly as picking the item up in the world would; an absent or
/// uncataloged item still adds, with condition `None`.
fn add_item(
    world: &mut World,
    invocation: &ConsoleInvocation,
) -> Result<ConsoleCommandResult, ConsoleError> {
    if invocation.args.is_empty() || invocation.args.len() > 2 {
        return Err(ConsoleError::new(
            "bad_arity",
            "additem expects a FormID and an optional count",
        ));
    }
    let form_id = parse_item_form_id(&invocation.args[0]).ok_or_else(|| {
        ConsoleError::new(
            "bad_type",
            "additem FormID must be 1-8 hex digits, e.g. f, 0x1f, or 0000000f",
        )
    })?;
    let count = invocation
        .args
        .get(1)
        .map(|value| {
            value
                .parse::<i32>()
                .map_err(|_| ConsoleError::new("bad_type", "count must be a whole number"))
        })
        .transpose()?
        .unwrap_or(1);
    if count < 1 {
        return Err(ConsoleError::new("bad_count", "count must be at least 1"));
    }
    let condition = world
        .get_resource::<PreparedItemCatalog>()
        .into_iter()
        .flat_map(|catalog| &catalog.items)
        .find(|item| item.base_form_id == form_id)
        .and_then(|item| match &item.stats {
            PreparedItemStats::Weapon { max_condition, .. }
            | PreparedItemStats::Apparel { max_condition, .. } => *max_condition,
            _ => None,
        });
    let stack = InventoryStack {
        base_form_id: form_id,
        count,
        condition,
    };
    let before = world
        .resource::<interaction::PlayerInventory>()
        .legacy_snapshot();
    world
        .resource_mut::<interaction::CanonicalItemLedger>()
        .add_player_item(&before, stack)
        .map_err(|error| ConsoleError::new("item_transaction_failed", error.to_string()))?;
    let _ = world
        .resource_mut::<interaction::PlayerInventory>()
        .add_stack(stack);
    let total = world
        .resource::<interaction::PlayerInventory>()
        .count(form_id);
    Ok(ConsoleCommandResult::new(
        json!({
            "form_id": form_id,
            "count": count,
            "total": total,
        }),
        vec![format!(
            "additem {form_id:08x} x{count}; inventory now has {total}"
        )],
    ))
}

fn parse_item_instance_id(value: &str) -> Option<ItemInstanceId> {
    let digits = value
        .strip_prefix("0x")
        .or_else(|| value.strip_prefix("0X"))
        .unwrap_or(value);
    u64::from_str_radix(digits, 16)
        .ok()
        .filter(|id| *id != 0)
        .map(ItemInstanceId)
}

fn parse_positive_count(value: Option<&String>) -> Result<u32, ConsoleError> {
    value
        .map(|value| {
            value
                .parse::<u32>()
                .map_err(|_| ConsoleError::new("bad_type", "count must be a whole number"))
        })
        .transpose()
        .map(|count| count.unwrap_or(1))
        .and_then(|count| {
            (count > 0)
                .then_some(count)
                .ok_or_else(|| ConsoleError::new("bad_count", "count must be at least 1"))
        })
}

fn canonical_equip_target(
    world: &World,
    item_id: ItemInstanceId,
) -> Result<(StackKey, player::equipment::EquipKind), ConsoleError> {
    let key = world
        .resource::<interaction::CanonicalItemLedger>()
        .player_stack_key(item_id)
        .ok_or_else(|| {
            ConsoleError::new(
                "item_not_found",
                "item instance is not in the player inventory",
            )
        })?;
    let item = world
        .get_resource::<PreparedItemCatalog>()
        .and_then(|catalog| {
            catalog
                .items
                .iter()
                .find(|item| item.base_form_id == key.base_form_id)
        })
        .ok_or_else(|| {
            ConsoleError::new(
                "no_catalog_entry",
                "item instance has no prepared item definition",
            )
        })?;
    let kind = interaction::equip_kind_for(item)
        .ok_or_else(|| ConsoleError::new("not_equippable", "item instance cannot be equipped"))?;
    Ok((key, kind))
}

fn equip_error(error: player::equipment::EquipError) -> ConsoleError {
    match error {
        player::equipment::EquipError::NotEquippable => {
            ConsoleError::new("not_equippable", "item instance cannot be equipped")
        }
        player::equipment::EquipError::IncompatibleAmmo => ConsoleError::new(
            "incompatible_ammo",
            "item instance ammo does not match the equipped weapon",
        ),
        player::equipment::EquipError::NoWeaponEquipped => ConsoleError::new(
            "no_weapon_equipped",
            "item instance ammo requires a weapon to be equipped first",
        ),
    }
}

/// Equip a canonical player item by stable instance id.
fn equip_item(
    world: &mut World,
    invocation: &ConsoleInvocation,
) -> Result<ConsoleCommandResult, ConsoleError> {
    let [value] = invocation.args.as_slice() else {
        return Err(ConsoleError::new(
            "bad_arity",
            "equip requires an item instance id",
        ));
    };
    let item_id = parse_item_instance_id(value)
        .ok_or_else(|| ConsoleError::new("bad_type", "item instance id must be hexadecimal"))?;
    let (key, kind) = canonical_equip_target(world, item_id)?;
    let previous_equipment = world.resource::<interaction::PlayerEquipment>().clone();
    world
        .resource_mut::<interaction::PlayerEquipment>()
        .equip(key, kind)
        .map_err(equip_error)?;
    if let Err(error) = world
        .resource_mut::<interaction::CanonicalItemLedger>()
        .ledger
        .equip(HolderId::Player, item_id)
    {
        world
            .resource_mut::<interaction::PlayerEquipment>()
            .clone_from(&previous_equipment);
        return Err(ConsoleError::new("equip_failed", error.to_string()));
    }
    Ok(ConsoleCommandResult::new(
        json!({ "item_id": item_id.0, "equipped": true }),
        vec![format!("equipped item {:016x}", item_id.0)],
    ))
}

/// Issue #98 (F98.4): `player.equipitem <FormID>` toggles equip/unequip for
/// an item already in the player inventory, through the same
/// `interaction::equip_kind_for`/`PlayerEquipment::toggle` seam the Pip-Boy
/// and hotkeys use, so all three equip paths behave identically.
fn equip_item_formid(
    world: &mut World,
    invocation: &ConsoleInvocation,
) -> Result<ConsoleCommandResult, ConsoleError> {
    let [raw_form_id] = invocation.args.as_slice() else {
        return Err(ConsoleError::new(
            "bad_arity",
            "equipitem expects exactly one FormID",
        ));
    };
    let form_id = parse_item_form_id(raw_form_id).ok_or_else(|| {
        ConsoleError::new(
            "bad_type",
            "equipitem FormID must be 1-8 hex digits, e.g. f, 0x1f, or 0000000f",
        )
    })?;
    let key = world
        .resource::<interaction::PlayerInventory>()
        .stack_states()
        .into_iter()
        .find(|stack| stack.base_form_id == form_id)
        .map(|stack| stack.key())
        .ok_or_else(|| {
            ConsoleError::new(
                "not_in_inventory",
                "equipitem target is not in the player inventory",
            )
        })?;
    let item = world
        .get_resource::<PreparedItemCatalog>()
        .and_then(|catalog| {
            catalog
                .items
                .iter()
                .find(|item| item.base_form_id == form_id)
                .cloned()
        })
        .ok_or_else(|| {
            ConsoleError::new(
                "no_catalog_entry",
                "equipitem target has no prepared item definition",
            )
        })?;
    let kind = interaction::equip_kind_for(&item).ok_or_else(|| {
        ConsoleError::new("not_equippable", "equipitem target cannot be equipped")
    })?;
    let outcome = world
        .resource_mut::<interaction::PlayerEquipment>()
        .toggle(key, kind)
        .map_err(|error| match error {
            player::equipment::EquipError::NotEquippable => {
                ConsoleError::new("not_equippable", "equipitem target cannot be equipped")
            }
            player::equipment::EquipError::IncompatibleAmmo => ConsoleError::new(
                "incompatible_ammo",
                "equipitem ammo does not match the equipped weapon",
            ),
            player::equipment::EquipError::NoWeaponEquipped => ConsoleError::new(
                "no_weapon_equipped",
                "equipitem ammo requires a weapon to be equipped first",
            ),
        })?;
    let equipped = world
        .resource::<interaction::PlayerEquipment>()
        .is_equipped(key);
    Ok(ConsoleCommandResult::new(
        json!({
            "form_id": form_id,
            "equipped": equipped,
            "evicted": outcome
                .evicted
                .iter()
                .map(|key| format!("{:08x}", key.base_form_id))
                .collect::<Vec<_>>(),
        }),
        vec![format!(
            "equipitem {form_id:08x} {}",
            if equipped { "equipped" } else { "unequipped" }
        )],
    ))
}

fn unequip_item(
    world: &mut World,
    invocation: &ConsoleInvocation,
) -> Result<ConsoleCommandResult, ConsoleError> {
    if !invocation.args.is_empty() {
        return Err(ConsoleError::new(
            "bad_arity",
            "unequip accepts no arguments",
        ));
    }
    let item_id = world
        .resource::<interaction::CanonicalItemLedger>()
        .ledger
        .bindings()
        .get(&HolderId::Player)
        .and_then(|bindings| bindings.equipped);
    let key = item_id.and_then(|item_id| {
        world
            .resource::<interaction::CanonicalItemLedger>()
            .player_stack_key(item_id)
    });
    if item_id.is_some() && key.is_none() {
        return Err(ConsoleError::new(
            "unequip_failed",
            "canonical equipped item is not in the player inventory",
        ));
    }
    let item_id = world
        .resource_mut::<interaction::CanonicalItemLedger>()
        .ledger
        .unequip(HolderId::Player)
        .map_err(|error| ConsoleError::new("unequip_failed", error.to_string()))?;
    if let Some(key) = key {
        world
            .resource_mut::<interaction::PlayerEquipment>()
            .unequip(key);
    }
    Ok(ConsoleCommandResult::new(
        json!({ "item_id": item_id.map(|id| id.0), "equipped": false }),
        vec!["player item unequipped".into()],
    ))
}

fn bind_hotkey(
    world: &mut World,
    invocation: &ConsoleInvocation,
) -> Result<ConsoleCommandResult, ConsoleError> {
    let [slot, value] = invocation.args.as_slice() else {
        return Err(ConsoleError::new(
            "bad_arity",
            "hotkey requires a slot and item instance id",
        ));
    };
    let slot = slot
        .parse::<usize>()
        .map_err(|_| ConsoleError::new("bad_type", "hotkey slot must be 0..7"))?;
    if slot >= 8 {
        return Err(ConsoleError::new(
            "hotkey_failed",
            "hotkey slot must be 0..7",
        ));
    }
    let item_id = parse_item_instance_id(value)
        .ok_or_else(|| ConsoleError::new("bad_type", "item instance id must be hexadecimal"))?;
    let key = world
        .resource::<interaction::CanonicalItemLedger>()
        .player_stack_key(item_id)
        .ok_or_else(|| {
            ConsoleError::new(
                "item_not_found",
                "item instance is not in the player inventory",
            )
        })?;
    world
        .resource_mut::<interaction::CanonicalItemLedger>()
        .ledger
        .bind_hotkey(HolderId::Player, slot, item_id)
        .map_err(|error| ConsoleError::new("hotkey_failed", error.to_string()))?;
    world
        .get_resource_or_insert_with(super::bindings::HotkeyBindings::default)
        .assign((slot + 1) as u8, key);
    Ok(ConsoleCommandResult::new(
        json!({ "slot": slot, "item_id": item_id.0 }),
        vec![format!("hotkey {slot} bound to {:016x}", item_id.0)],
    ))
}

fn use_item(
    world: &mut World,
    invocation: &ConsoleInvocation,
) -> Result<ConsoleCommandResult, ConsoleError> {
    let [value] = invocation.args.as_slice() else {
        return Err(ConsoleError::new(
            "bad_arity",
            "useitem requires an item instance id",
        ));
    };
    let item_id = parse_item_instance_id(value)
        .ok_or_else(|| ConsoleError::new("bad_type", "item instance id must be hexadecimal"))?;
    let used = world
        .resource_mut::<interaction::CanonicalItemLedger>()
        .ledger
        .use_item(HolderId::Player, item_id)
        .map_err(|error| ConsoleError::new("useitem_failed", error.to_string()))?;
    Ok(ConsoleCommandResult::new(
        json!({ "item_id": item_id.0, "base_form_id": used.base_form_id, "count": 1 }),
        vec![format!("used item {:016x}", item_id.0)],
    ))
}

fn merchant_stacks(placement: &interaction::PlacementRoot) -> Vec<(u32, i32)> {
    placement
        .placement()
        .inventory
        .iter()
        .filter(|entry| !entry.leveled)
        .map(|entry| (entry.base_form_id, entry.count))
        .collect()
}

fn set_merchant(
    world: &mut World,
    invocation: &ConsoleInvocation,
) -> Result<ConsoleCommandResult, ConsoleError> {
    let [selector, caps] = invocation.args.as_slice() else {
        return Err(ConsoleError::new(
            "bad_arity",
            "setmerchant requires a container reference and caps",
        ));
    };
    let entity = resolve_reference(world, selector)?;
    let (reference_form_id, stacks) = {
        let placement = world
            .get::<interaction::PlacementRoot>(entity)
            .ok_or_else(|| {
                ConsoleError::new("not_a_container", "reference has no placement root")
            })?;
        if !matches!(placement.placement().semantic, PreparedSemantic::Container) {
            return Err(ConsoleError::new(
                "not_a_container",
                "merchant reference is not a container",
            ));
        }
        (
            placement.placement().reference_form_id,
            merchant_stacks(placement),
        )
    };
    let caps = caps
        .parse::<u64>()
        .map_err(|_| ConsoleError::new("bad_type", "merchant caps must be a whole number"))?;
    world
        .resource_mut::<interaction::CanonicalItemLedger>()
        .set_merchant(reference_form_id, &stacks, caps)
        .map_err(|error| ConsoleError::new("merchant_setup_failed", error.to_string()))?;
    Ok(ConsoleCommandResult::new(
        json!({ "reference_form_id": reference_form_id, "caps": caps }),
        vec![format!(
            "merchant {:08x} configured with {caps} caps",
            reference_form_id
        )],
    ))
}

fn merchant_transaction(
    world: &mut World,
    invocation: &ConsoleInvocation,
    buying: bool,
) -> Result<ConsoleCommandResult, ConsoleError> {
    let [selector, item_value, count_value @ ..] = invocation.args.as_slice() else {
        return Err(ConsoleError::new(
            "bad_arity",
            "merchant transfer requires reference, item instance id, and optional count",
        ));
    };
    if count_value.len() > 1 {
        return Err(ConsoleError::new(
            "bad_arity",
            "merchant transfer accepts one optional count",
        ));
    }
    let count = parse_positive_count(count_value.first())?;
    let merchant_entity = resolve_reference(world, selector)?;
    let reference_form_id = world
        .get::<interaction::PlacementRoot>(merchant_entity)
        .ok_or_else(|| ConsoleError::new("not_a_merchant", "reference has no placement root"))?
        .placement()
        .reference_form_id;
    let item_id = parse_item_instance_id(item_value)
        .ok_or_else(|| ConsoleError::new("bad_type", "item instance id must be hexadecimal"))?;
    let merchant = HolderId::FixtureMerchant { reference_form_id };
    let item = world
        .resource::<interaction::CanonicalItemLedger>()
        .ledger
        .holders()
        .get(&merchant)
        .and_then(|state| state.find(item_id))
        .or_else(|| {
            world
                .resource::<interaction::CanonicalItemLedger>()
                .ledger
                .holders()
                .get(&HolderId::Player)
                .and_then(|state| state.find(item_id))
        })
        .ok_or_else(|| ConsoleError::new("item_not_found", "item instance is not available"))?;
    let value = world
        .resource::<PreparedItemCatalog>()
        .items
        .iter()
        .find(|definition| definition.base_form_id == item.base_form_id)
        .and_then(|definition| definition.value)
        .filter(|value| *value >= 0)
        .ok_or_else(|| {
            ConsoleError::new("invalid_price", "item has no non-negative catalog value")
        })? as u64;
    if item.base_form_id == interaction::item_rules::CAPS_FORM_ID
        || world
            .resource::<PreparedItemCatalog>()
            .items
            .iter()
            .find(|definition| definition.base_form_id == item.base_form_id)
            .is_some_and(|definition| definition.quest_item)
    {
        return Err(ConsoleError::new(
            "item_not_tradeable",
            "caps and quest items cannot be traded",
        ));
    }
    let request = if buying {
        TransactionRequest::Buy {
            merchant,
            player: HolderId::Player,
            item_id,
            count,
            unit_price: value,
        }
    } else {
        TransactionRequest::Sell {
            player: HolderId::Player,
            merchant,
            item_id,
            count,
            unit_price: value,
        }
    };
    let receipt = world
        .resource_mut::<interaction::CanonicalItemLedger>()
        .ledger
        .execute(request)
        .map_err(|error| ConsoleError::new("merchant_transfer_failed", error.to_string()))?;
    Ok(ConsoleCommandResult::new(
        json!({ "merchant": reference_form_id, "item_id": item_id.0, "count": count, "unit_price": value, "transaction_id": receipt.id.0 }),
        vec![format!(
            "{} {:016x} x{} at {} caps",
            if buying { "bought" } else { "sold" },
            item_id.0,
            count,
            value
        )],
    ))
}

fn buy_item(
    world: &mut World,
    invocation: &ConsoleInvocation,
) -> Result<ConsoleCommandResult, ConsoleError> {
    merchant_transaction(world, invocation, true)
}

fn sell_item(
    world: &mut World,
    invocation: &ConsoleInvocation,
) -> Result<ConsoleCommandResult, ConsoleError> {
    merchant_transaction(world, invocation, false)
}

/// Issue #60 (F60.3): captures the active cell into `ActiveSaveState` and
/// writes the named slot through `SaveStore`, responding deterministically
/// with the written path.
fn save_slot(
    world: &mut World,
    invocation: &ConsoleInvocation,
) -> Result<ConsoleCommandResult, ConsoleError> {
    let [slot] = invocation.args.as_slice() else {
        return Err(ConsoleError::new(
            "bad_arity",
            "save requires exactly one slot name",
        ));
    };
    let path = super::world::write_save_slot(world, slot)
        .map_err(|error| ConsoleError::new("save_failed", format!("{error:#}")))?;
    Ok(ConsoleCommandResult::new(
        json!({ "slot": slot, "path": path }),
        vec![format!("Save written to {}.", path.display())],
    ))
}

fn no_args(invocation: &ConsoleInvocation) -> Result<(), ConsoleError> {
    if invocation.args.is_empty() {
        Ok(())
    } else {
        Err(ConsoleError::new(
            "bad_arity",
            format!("{} does not accept arguments", invocation.command),
        ))
    }
}

fn toggle_result(value: Value, label: &str, enabled: bool) -> ConsoleCommandResult {
    ConsoleCommandResult::new(
        value,
        vec![format!(
            "{label} {}.",
            if enabled { "enabled" } else { "disabled" }
        )],
    )
}

fn toggle_collision(
    world: &mut World,
    invocation: &ConsoleInvocation,
) -> Result<ConsoleCommandResult, ConsoleError> {
    no_args(invocation)?;
    if world.resource::<player::CameraModeState>().mode != player::CameraMode::Fps {
        return Err(ConsoleError::new(
            "player_unavailable",
            "tcl requires the FPS player",
        ));
    }
    let currently_no_clip = world.resource::<player::PlayerNoClip>().0;
    if currently_no_clip {
        if world.resource::<player::PhysicsDisabled>().0 {
            return Err(ConsoleError::new(
                "physics_disabled",
                "collision cannot be enabled because physics is disabled",
            ));
        }
        if !world.resource::<player::CameraModeState>().collisions_ready {
            return Err(ConsoleError::new(
                "collision_unavailable",
                "collision cannot be enabled because the scene has no usable collision geometry",
            ));
        }
    }
    let enabled = !currently_no_clip;
    world.resource_mut::<player::PlayerNoClip>().0 = enabled;
    Ok(toggle_result(
        json!({ "no_clip": enabled }),
        "Collision",
        !enabled,
    ))
}

fn toggle_fly_camera(
    world: &mut World,
    invocation: &ConsoleInvocation,
) -> Result<ConsoleCommandResult, ConsoleError> {
    no_args(invocation)?;
    let mode = player::toggle_camera_mode_now(world).map_err(|error| match error {
        player::CameraModeError::CameraUnavailable => ConsoleError::new(
            "camera_unavailable",
            "expected exactly one active 3D camera",
        ),
        player::CameraModeError::HierarchyInvalid => ConsoleError::new(
            "camera_hierarchy_invalid",
            "camera and FPS-player hierarchy is inconsistent",
        ),
        player::CameraModeError::PlayerUnavailable => {
            ConsoleError::new("player_unavailable", "the FPS player does not exist")
        }
    })?;
    let mode = match mode {
        player::CameraMode::Free => "free",
        player::CameraMode::Fps => "fps",
    };
    Ok(toggle_result(
        json!({ "camera_mode": mode }),
        "Free camera",
        mode == "free",
    ))
}

fn field_of_view(
    world: &mut World,
    invocation: &ConsoleInvocation,
) -> Result<ConsoleCommandResult, ConsoleError> {
    if invocation.args.len() > 1 {
        return Err(ConsoleError::new(
            "bad_arity",
            "fov accepts at most one value",
        ));
    }
    let camera = {
        let mut query = world.query_filtered::<Entity, (With<Camera3d>, With<HorizontalFov>)>();
        let mut cameras = query.iter(world);
        let Some(camera) = cameras.next() else {
            return Err(ConsoleError::new(
                "camera_unavailable",
                "the active 3D camera does not expose an FOV setting",
            ));
        };
        if cameras.next().is_some() {
            return Err(ConsoleError::new(
                "camera_unavailable",
                "expected exactly one active 3D camera",
            ));
        }
        camera
    };

    let requested = invocation
        .args
        .first()
        .map(|value| {
            value
                .parse::<f32>()
                .map_err(|_| ConsoleError::new("bad_type", "fov must be a finite number"))
        })
        .transpose()?;
    if let Some(requested) = requested {
        if !requested.is_finite()
            || !(MIN_HORIZONTAL_FOV_DEGREES..=MAX_HORIZONTAL_FOV_DEGREES).contains(&requested)
        {
            return Err(ConsoleError::new(
                "out_of_range",
                format!(
                    "fov must be between {MIN_HORIZONTAL_FOV_DEGREES} and {MAX_HORIZONTAL_FOV_DEGREES} degrees"
                ),
            ));
        }
        let aspect_ratio = match world.get::<Projection>(camera) {
            Some(Projection::Perspective(perspective)) => perspective.aspect_ratio,
            _ => {
                return Err(ConsoleError::new(
                    "camera_unavailable",
                    "the active 3D camera is not perspective",
                ));
            }
        };
        world
            .get_mut::<HorizontalFov>(camera)
            .expect("camera query required HorizontalFov")
            .0 = requested;
        let mut projection = world
            .get_mut::<Projection>(camera)
            .expect("Camera3d requires Projection");
        let Projection::Perspective(perspective) = &mut *projection else {
            unreachable!("perspective projection checked above");
        };
        perspective.fov = horizontal_to_vertical_fov(requested, aspect_ratio);
    }

    let degrees = world
        .get::<HorizontalFov>(camera)
        .expect("camera query required HorizontalFov")
        .0;
    Ok(ConsoleCommandResult::new(
        json!({
            "degrees": degrees,
            "axis": "horizontal",
        }),
        vec![if requested.is_some() {
            format!("Horizontal FOV set to {degrees} degrees.")
        } else {
            format!("Horizontal FOV is {degrees} degrees.")
        }],
    ))
}

fn toggle_lights(
    world: &mut World,
    invocation: &ConsoleInvocation,
) -> Result<ConsoleCommandResult, ConsoleError> {
    no_args(invocation)?;
    let disabled = {
        let mut disabled = world.resource_mut::<LightsDisabled>();
        disabled.0 = !disabled.0;
        disabled.0
    };
    Ok(toggle_result(
        json!({ "lights_enabled": !disabled }),
        "Lights",
        !disabled,
    ))
}

fn toggle_collision_geometry(
    world: &mut World,
    invocation: &ConsoleInvocation,
) -> Result<ConsoleCommandResult, ConsoleError> {
    no_args(invocation)?;
    let enabled = {
        let mut settings = world.resource_mut::<BoxdddDebugDrawSettings>();
        player::flip_collider_debug(&mut settings);
        settings.enabled
    };
    Ok(toggle_result(
        json!({ "enabled": enabled }),
        "Collision geometry",
        enabled,
    ))
}

fn toggle_stair_debug(
    world: &mut World,
    invocation: &ConsoleInvocation,
) -> Result<ConsoleCommandResult, ConsoleError> {
    no_args(invocation)?;
    let enabled = {
        let mut settings = world.resource_mut::<player::StepDebugSettings>();
        player::flip_step_debug(&mut settings);
        player::step_debug_enabled(&settings)
    };
    Ok(toggle_result(
        json!({ "enabled": enabled }),
        "Stair debugging",
        enabled,
    ))
}

fn toggle_unlit(
    world: &mut World,
    invocation: &ConsoleInvocation,
) -> Result<ConsoleCommandResult, ConsoleError> {
    no_args(invocation)?;
    let enabled = {
        let mut mode = world.resource_mut::<UnlitMode>();
        mode.0 = !mode.0;
        mode.0
    };
    Ok(toggle_result(
        json!({ "enabled": enabled }),
        "Unlit mode",
        enabled,
    ))
}

fn toggle_game_ui(
    world: &mut World,
    invocation: &ConsoleInvocation,
) -> Result<ConsoleCommandResult, ConsoleError> {
    no_args(invocation)?;
    let visible = {
        let mut state = world.resource_mut::<GameUiState>();
        state.visible = !state.visible;
        state.visible
    };
    apply_game_ui_visibility(world);
    Ok(toggle_result(
        json!({ "visible": visible }),
        "Game UI",
        visible,
    ))
}

fn toggle_diagnostic_ui(
    world: &mut World,
    invocation: &ConsoleInvocation,
) -> Result<ConsoleCommandResult, ConsoleError> {
    no_args(invocation)?;
    let visible = {
        let mut state = world.resource_mut::<DiagnosticUiState>();
        state.visible = !state.visible;
        state.visible
    };
    apply_diagnostic_ui_visibility(world);
    Ok(toggle_result(
        json!({ "visible": visible }),
        "Diagnostic UI",
        visible,
    ))
}

fn console_open(world: &World) -> bool {
    world
        .get_resource::<State<GameplayModal>>()
        .is_some_and(|modal| *modal.get() == GameplayModal::Console)
}

fn apply_game_ui_visibility(world: &mut World) {
    let visible = world.resource::<GameUiState>().visible && !console_open(world);
    let mut query = world.query_filtered::<&mut Visibility, With<GameUi>>();
    for mut visibility in query.iter_mut(world) {
        *visibility = if visible {
            Visibility::Inherited
        } else {
            Visibility::Hidden
        };
    }
}

fn apply_diagnostic_ui_visibility(world: &mut World) {
    let visible = world.resource::<DiagnosticUiState>().visible && !console_open(world);
    let mut query = world.query_filtered::<&mut Visibility, With<DiagnosticUi>>();
    for mut visibility in query.iter_mut(world) {
        *visibility = if visible {
            Visibility::Inherited
        } else {
            Visibility::Hidden
        };
    }
}

type UiVisibilityQuery<'w, 's> = Query<
    'w,
    's,
    (
        Option<&'static GameUi>,
        Option<&'static DiagnosticUi>,
        &'static mut Visibility,
    ),
    Or<(With<GameUi>, With<DiagnosticUi>)>,
>;

fn sync_ui_visibility(
    modal: Option<Res<State<GameplayModal>>>,
    game: Res<GameUiState>,
    diagnostic: Res<DiagnosticUiState>,
    mut entities: UiVisibilityQuery<'_, '_>,
) {
    let console_open = modal.is_some_and(|modal| *modal.get() == GameplayModal::Console);
    for (game_marker, diagnostic_marker, mut visibility) in &mut entities {
        let visible = !console_open
            && game_marker.is_none_or(|_| game.visible)
            && diagnostic_marker.is_none_or(|_| diagnostic.visible);
        *visibility = if visible {
            Visibility::Inherited
        } else {
            Visibility::Hidden
        };
    }
}

const RENDER_SETTINGS: [&str; 10] = [
    "lighting",
    "irradiance",
    "ambient",
    "bloom_intensity",
    "bloom_threshold",
    "bloom_softness",
    "fog",
    "ao",
    "shadow_samples",
    "realtime_shadows",
];

const TONEMAPPER_NAMES: [&str; 9] = [
    "none",
    "reinhard",
    "reinhard_luminance",
    "aces_fitted",
    "agx",
    "somewhat_boring_display_transform",
    "tony_mc_mapface",
    "blender_filmic",
    "khronos_pbr_neutral",
];

fn tonemapper_name(tonemapper: Tonemapping) -> &'static str {
    match tonemapper {
        Tonemapping::None => "none",
        Tonemapping::Reinhard => "reinhard",
        Tonemapping::ReinhardLuminance => "reinhard_luminance",
        Tonemapping::AcesFitted => "aces_fitted",
        Tonemapping::AgX => "agx",
        Tonemapping::SomewhatBoringDisplayTransform => "somewhat_boring_display_transform",
        Tonemapping::TonyMcMapface => "tony_mc_mapface",
        Tonemapping::BlenderFilmic => "blender_filmic",
        Tonemapping::KhronosPbrNeutral => "khronos_pbr_neutral",
    }
}

fn tonemapper_display_name(tonemapper: Tonemapping) -> &'static str {
    match tonemapper {
        Tonemapping::None => "None",
        Tonemapping::Reinhard => "Reinhard",
        Tonemapping::ReinhardLuminance => "Reinhard Luminance",
        Tonemapping::AcesFitted => "ACES Fitted",
        Tonemapping::AgX => "AgX",
        Tonemapping::SomewhatBoringDisplayTransform => "Somewhat Boring Display Transform",
        Tonemapping::TonyMcMapface => "Tony McMapface",
        Tonemapping::BlenderFilmic => "Blender Filmic",
        Tonemapping::KhronosPbrNeutral => "Khronos PBR Neutral",
    }
}

fn parse_tonemapper(value: &str) -> Option<Tonemapping> {
    match value.to_ascii_lowercase().as_str() {
        "none" => Some(Tonemapping::None),
        "reinhard" => Some(Tonemapping::Reinhard),
        "reinhard_luminance" => Some(Tonemapping::ReinhardLuminance),
        "aces_fitted" => Some(Tonemapping::AcesFitted),
        "agx" => Some(Tonemapping::AgX),
        "somewhat_boring_display_transform" => Some(Tonemapping::SomewhatBoringDisplayTransform),
        "tony_mc_mapface" => Some(Tonemapping::TonyMcMapface),
        "blender_filmic" => Some(Tonemapping::BlenderFilmic),
        "khronos_pbr_neutral" => Some(Tonemapping::KhronosPbrNeutral),
        _ => None,
    }
}

fn tonemapper_camera(world: &mut World) -> Result<Entity, ConsoleError> {
    let mut query = world.query_filtered::<Entity, (With<Camera3d>, With<Tonemapping>)>();
    let mut cameras = query.iter(world);
    let Some(camera) = cameras.next() else {
        return Err(ConsoleError::new(
            "camera_unavailable",
            "expected exactly one active 3D camera with tonemapping",
        ));
    };
    if cameras.next().is_some() {
        return Err(ConsoleError::new(
            "camera_unavailable",
            "expected exactly one active 3D camera with tonemapping",
        ));
    }
    Ok(camera)
}

fn tonemap(
    world: &mut World,
    invocation: &ConsoleInvocation,
) -> Result<ConsoleCommandResult, ConsoleError> {
    if invocation.args.len() > 1 {
        return Err(ConsoleError::new(
            "bad_arity",
            "tonemap accepts at most one mode",
        ));
    }
    let camera = tonemapper_camera(world)?;
    let requested = invocation
        .args
        .first()
        .map(|value| {
            parse_tonemapper(value).ok_or_else(|| {
                ConsoleError::new(
                    "unknown_tonemapper",
                    format!(
                        "unknown tonemapper '{value}'; expected one of: {}",
                        TONEMAPPER_NAMES.join(", ")
                    ),
                )
            })
        })
        .transpose()?;
    if let Some(requested) = requested {
        *world
            .get_mut::<Tonemapping>(camera)
            .expect("tonemapper camera query required Tonemapping") = requested;
    }
    let current = *world
        .get::<Tonemapping>(camera)
        .expect("tonemapper camera query required Tonemapping");
    let value = tonemapper_name(current);
    let display = tonemapper_display_name(current);
    let message = if requested.is_some() {
        format!("Tonemapper set to {display}.")
    } else {
        format!("Tonemapper is {display}.")
    };
    Ok(ConsoleCommandResult::new(
        json!({ "tonemapper": value }),
        vec![message],
    ))
}

fn bloom_values(world: &mut World) -> Result<(f32, f32, f32), ConsoleError> {
    let mut query = world.query_filtered::<&Bloom, With<Camera3d>>();
    let mut blooms = query.iter(world);
    let Some(bloom) = blooms.next() else {
        return Err(ConsoleError::new(
            "camera_unavailable",
            "the active camera does not have bloom settings",
        ));
    };
    let values = (
        bloom.intensity,
        bloom.prefilter.threshold,
        bloom.prefilter.threshold_softness,
    );
    if blooms.next().is_some() {
        return Err(ConsoleError::new(
            "camera_unavailable",
            "expected exactly one camera with bloom settings",
        ));
    }
    Ok(values)
}

fn render_values(world: &mut World) -> Result<Map<String, Value>, ConsoleError> {
    let (bloom_intensity, bloom_threshold, bloom_softness) = bloom_values(world)?;
    let mut values = Map::new();
    values.insert(
        "lighting".into(),
        json!(world.resource::<LightingScale>().0),
    );
    values.insert(
        "irradiance".into(),
        json!(world.resource::<IrradianceIntensity>().0),
    );
    values.insert("ambient".into(), json!(world.resource::<AmbientScale>().0));
    values.insert("bloom_intensity".into(), json!(bloom_intensity));
    values.insert("bloom_threshold".into(), json!(bloom_threshold));
    values.insert("bloom_softness".into(), json!(bloom_softness));
    values.insert("fog".into(), json!(world.resource::<FogStrength>().0));
    values.insert("ao".into(), json!(world.resource::<AoStrength>().0));
    values.insert(
        "shadow_samples".into(),
        json!(world.resource::<PointLightShadowSamples>().0),
    );
    values.insert(
        "realtime_shadows".into(),
        json!(world.resource::<RealtimeShadowSettings>().enabled as u8),
    );
    Ok(values)
}

fn validate_render_setting(setting: &str) -> Result<(), ConsoleError> {
    if RENDER_SETTINGS.contains(&setting) {
        Ok(())
    } else {
        Err(ConsoleError::new(
            "unknown_setting",
            format!("unknown render setting '{setting}'"),
        ))
    }
}

fn render_setting_label(setting: &str) -> &'static str {
    match setting {
        "lighting" => "Lighting",
        "irradiance" => "Irradiance",
        "ambient" => "Ambient light",
        "bloom_intensity" => "Bloom intensity",
        "bloom_threshold" => "Bloom threshold",
        "bloom_softness" => "Bloom softness",
        "fog" => "Fog",
        "ao" => "Ambient occlusion",
        "shadow_samples" => "Point-shadow samples per pixel",
        "realtime_shadows" => "Realtime point shadows",
        _ => "Render setting",
    }
}

fn get_render(
    world: &mut World,
    invocation: &ConsoleInvocation,
) -> Result<ConsoleCommandResult, ConsoleError> {
    if invocation.args.len() > 1 {
        return Err(ConsoleError::new(
            "bad_arity",
            "getrender accepts at most one setting",
        ));
    }
    let values = render_values(world)?;
    if let Some(setting) = invocation.args.first() {
        let setting = setting.to_ascii_lowercase();
        validate_render_setting(&setting)?;
        let value = values[&setting].clone();
        let message = format!("{}: {value}.", render_setting_label(&setting));
        Ok(ConsoleCommandResult::new(
            json!({
                "setting": setting,
                "value": value
            }),
            vec![message],
        ))
    } else {
        let log = RENDER_SETTINGS
            .iter()
            .map(|setting| format!("{}: {}.", render_setting_label(setting), values[*setting]))
            .collect();
        Ok(ConsoleCommandResult::new(Value::Object(values), log))
    }
}

fn set_render(
    world: &mut World,
    invocation: &ConsoleInvocation,
) -> Result<ConsoleCommandResult, ConsoleError> {
    if invocation.args.len() != 2 {
        return Err(ConsoleError::new(
            "bad_arity",
            "setrender expects a setting and value",
        ));
    }
    let setting = invocation.args[0].to_ascii_lowercase();
    validate_render_setting(&setting)?;
    let value = invocation.args[1]
        .parse::<f32>()
        .map_err(|_| ConsoleError::new("bad_type", "render value must be a number"))?;
    if !value.is_finite() {
        return Err(ConsoleError::new(
            "non_finite",
            "render value must be finite",
        ));
    }
    let valid = match setting.as_str() {
        "lighting" => (0.0001..=262_144.0).contains(&value),
        "irradiance" => (0.0..=4096.0).contains(&value),
        "ambient" => (0.0001..=4096.0).contains(&value),
        "bloom_intensity" | "bloom_softness" | "fog" | "ao" => (0.0..=1.0).contains(&value),
        "bloom_threshold" => value >= 0.0,
        "shadow_samples" => value == 0.0 || value == 1.0,
        "realtime_shadows" => value == 0.0 || value == 1.0,
        _ => unreachable!(),
    };
    if !valid {
        return Err(ConsoleError::new(
            "out_of_range",
            format!("value {value} is outside the supported range for {setting}"),
        ));
    }

    match setting.as_str() {
        "lighting" => world.resource_mut::<LightingScale>().0 = value,
        "irradiance" => world.resource_mut::<IrradianceIntensity>().0 = value,
        "ambient" => world.resource_mut::<AmbientScale>().0 = value,
        "fog" => world.resource_mut::<FogStrength>().0 = value,
        "ao" => world.resource_mut::<AoStrength>().0 = value,
        "shadow_samples" => world.resource_mut::<PointLightShadowSamples>().0 = value as u32,
        "realtime_shadows" => {
            world.resource_mut::<RealtimeShadowSettings>().enabled = value == 1.0;
        }
        "bloom_intensity" | "bloom_threshold" | "bloom_softness" => {
            let camera = {
                let mut query = world.query_filtered::<Entity, (With<Camera3d>, With<Bloom>)>();
                let mut cameras = query.iter(world);
                let Some(camera) = cameras.next() else {
                    return Err(ConsoleError::new(
                        "camera_unavailable",
                        "the active camera does not have bloom settings",
                    ));
                };
                if cameras.next().is_some() {
                    return Err(ConsoleError::new(
                        "camera_unavailable",
                        "expected exactly one camera with bloom settings",
                    ));
                }
                camera
            };
            let mut bloom = world
                .get_mut::<Bloom>(camera)
                .ok_or_else(|| ConsoleError::new("camera_unavailable", "bloom is unavailable"))?;
            match setting.as_str() {
                "bloom_intensity" => bloom.intensity = value,
                "bloom_threshold" => bloom.prefilter.threshold = value,
                "bloom_softness" => bloom.prefilter.threshold_softness = value,
                _ => unreachable!(),
            }
        }
        _ => unreachable!(),
    }
    let message = format!("{} set to {value}.", render_setting_label(&setting));
    Ok(ConsoleCommandResult::new(
        json!({
            "setting": setting,
            "value": value
        }),
        vec![message],
    ))
}

fn shadow_cache(
    world: &mut World,
    invocation: &ConsoleInvocation,
) -> Result<ConsoleCommandResult, ConsoleError> {
    if invocation.args.len() > 1 {
        return Err(ConsoleError::new(
            "bad_arity",
            "shadowcache accepts status or rebuild",
        ));
    }
    match invocation
        .args
        .first()
        .map(String::as_str)
        .unwrap_or("status")
    {
        "status" => Ok(ConsoleCommandResult::new(
            shadow_cache_status(world),
            vec!["Shadow cache status reported.".into()],
        )),
        "rebuild" => Err(ConsoleError::new(
            "prepare_required",
            "prepared shadows cannot be rebuilt in the viewer; run `prepare --rebuild-shadows` for this cell, then restart render",
        )),
        _ => Err(ConsoleError::new(
            "bad_value",
            "shadowcache expects status or rebuild",
        )),
    }
}

fn render_report(
    world: &mut World,
    invocation: &ConsoleInvocation,
) -> Result<ConsoleCommandResult, ConsoleError> {
    no_args(invocation)?;
    let path = diagnostics::save_render_report_now(world).map_err(ConsoleError::internal)?;
    Ok(ConsoleCommandResult::new(
        json!({ "path": path }),
        vec![format!("Render report written to {}.", path.display())],
    ))
}

fn set_global_time_multiplier(
    world: &mut World,
    invocation: &ConsoleInvocation,
) -> Result<ConsoleCommandResult, ConsoleError> {
    if invocation.args.len() != 1 {
        return Err(ConsoleError::new(
            "bad_arity",
            "sgtm expects exactly one multiplier",
        ));
    }
    let multiplier = invocation.args[0]
        .parse::<f32>()
        .map_err(|_| ConsoleError::new("bad_type", "time multiplier must be a number"))?;
    if !multiplier.is_finite() || !(0.01..=100.0).contains(&multiplier) {
        return Err(ConsoleError::new(
            "out_of_range",
            "time multiplier must be between 0.01 and 100",
        ));
    }
    world
        .resource_mut::<Time<Virtual>>()
        .set_relative_speed(multiplier);
    Ok(ConsoleCommandResult::new(
        json!({ "relative_speed": multiplier }),
        vec![format!("Time multiplier set to {multiplier}.")],
    ))
}

fn screenshot(
    world: &mut World,
    invocation: &ConsoleInvocation,
) -> Result<ConsoleCommandResult, ConsoleError> {
    if invocation.args.len() > 1 {
        return Err(ConsoleError::new(
            "bad_arity",
            "screenshot accepts at most one name",
        ));
    }
    let default_name = format!("frame-{:08}", invocation.frame);
    let supplied = invocation
        .args
        .first()
        .map_or(default_name.as_str(), String::as_str);
    let name = supplied.strip_suffix(".png").unwrap_or(supplied);
    if name.is_empty()
        || name.len() > 96
        || !name
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_'))
    {
        return Err(ConsoleError::new(
            "invalid_path",
            "screenshot name may contain only letters, numbers, '-' and '_'",
        ));
    }
    let has_window = {
        let mut windows = world.query_filtered::<Entity, With<PrimaryWindow>>();
        windows.iter(world).next().is_some()
    };
    if !has_window {
        return Err(ConsoleError::new(
            "unsupported",
            "screenshots require a primary window",
        ));
    }
    let directory = PathBuf::from(".bevyout/screenshots");
    std::fs::create_dir_all(&directory).map_err(ConsoleError::internal)?;
    let path = directory.join(format!("{name}.png"));
    world
        .spawn(Screenshot::primary_window())
        .observe(save_to_disk(path.clone()));
    Ok(ConsoleCommandResult::new(
        json!({ "path": path, "persistence": "runtime_capture" }),
        vec![format!("Screenshot queued at {}.", path.display())],
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::console::{ConsoleExecutor, ConsolePlugin, ConsoleRequest, ConsoleSessionId};
    use crate::vsa::{PreparedItemCategory, PreparedItemDefinition};
    use bevy::state::app::StatesPlugin;

    fn test_app() -> App {
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, StatesPlugin, ConsolePlugin))
            .insert_resource(player::PlayerNoClip::default())
            .insert_resource(player::PhysicsDisabled(false))
            .insert_resource(LightingScale(1.0))
            .insert_resource(IrradianceIntensity(1.0))
            .insert_resource(AmbientScale(1.0))
            .insert_resource(FogStrength(1.0))
            .insert_resource(AoStrength(1.0))
            .insert_resource(UnlitMode(false))
            .insert_resource(LightsDisabled(false))
            .insert_resource(PreparedPointShadowRuntime::default())
            .insert_resource(PointLightShadowSamples::default())
            .insert_resource(BoxdddDebugDrawSettings::default())
            .insert_resource(player::StepDebugSettings::default())
            .insert_resource(interaction::PlayerInventory::default())
            .insert_resource(interaction::PlayerEquipment::default());
        app.init_resource::<interaction::CanonicalItemLedger>();
        app.init_state::<GameplayModal>();
        let camera = player::CameraModeState {
            collision_build_complete: true,
            collisions_ready: true,
            ..default()
        };
        app.insert_resource(camera);
        app.world_mut().spawn((
            Camera3d::default(),
            Projection::Perspective(super::super::default_perspective_projection()),
            HorizontalFov::default(),
            Bloom::default(),
            Tonemapping::AgX,
            Transform::from_xyz(0.0, 2.0, 0.0),
            super::super::FlyCamera {
                yaw: 0.0,
                pitch: 0.0,
                speed: 8.0,
            },
        ));
        install(&mut app);
        player::set_camera_mode(app.world_mut(), player::CameraMode::Fps).unwrap();
        app.update();
        app
    }

    fn current_tonemapper(app: &mut App) -> Tonemapping {
        let world = app.world_mut();
        let mut query = world.query_filtered::<&Tonemapping, With<Camera3d>>();
        *query.single(world).unwrap()
    }

    fn exec(app: &mut App, line: &str) -> crate::console::ConsoleOutput {
        ConsoleExecutor::execute(
            app.world_mut(),
            ConsoleRequest {
                session: ConsoleSessionId::new("test"),
                line: line.into(),
            },
        )
    }

    #[test]
    fn toggles_and_time_multiplier_change_focused_state() {
        let mut app = test_app();
        let game_ui = app.world_mut().spawn((GameUi, Visibility::Inherited)).id();
        let diagnostics = app
            .world_mut()
            .spawn((DiagnosticUi, Visibility::Inherited))
            .id();
        assert!(exec(&mut app, "tcl").value["no_clip"].as_bool().unwrap());
        assert!(exec(&mut app, "tm").ok);
        assert_eq!(
            app.world().get::<Visibility>(game_ui),
            Some(&Visibility::Hidden)
        );
        assert!(exec(&mut app, "tdt").ok);
        assert_eq!(
            app.world().get::<Visibility>(diagnostics),
            Some(&Visibility::Hidden)
        );
        assert!(exec(&mut app, "sgtm 2").ok);
        assert_eq!(
            app.world().resource::<Time<Virtual>>().relative_speed(),
            2.0
        );
    }

    #[test]
    fn fov_reports_sets_and_validates_horizontal_degrees() {
        let mut app = test_app();

        let current = exec(&mut app, "fov");
        assert_eq!(current.value["degrees"], 90.0);
        assert_eq!(current.value["axis"], "horizontal");

        let changed = exec(&mut app, "fov 110");
        assert!(changed.ok);
        assert_eq!(changed.value["degrees"], 110.0);
        let mut query = app
            .world_mut()
            .query_filtered::<&Projection, With<Camera3d>>();
        let Projection::Perspective(perspective) = query.single(app.world()).unwrap() else {
            panic!("expected a perspective camera");
        };
        let expected = horizontal_to_vertical_fov(110.0, perspective.aspect_ratio);
        assert!((perspective.fov - expected).abs() < 1e-6);

        assert_eq!(exec(&mut app, "fov 9").error.unwrap().code, "out_of_range");
        assert_eq!(
            exec(&mut app, "fov 171").error.unwrap().code,
            "out_of_range"
        );
        assert_eq!(exec(&mut app, "fov nope").error.unwrap().code, "bad_type");
        assert_eq!(
            exec(&mut app, "fov 90 extra").error.unwrap().code,
            "bad_arity"
        );
    }

    #[test]
    fn screenshot_rejects_headless_and_unsafe_names() {
        let mut app = test_app();
        assert_eq!(
            exec(&mut app, "screenshot").error.unwrap().code,
            "unsupported"
        );
        assert_eq!(
            exec(&mut app, "screenshot ../escape").error.unwrap().code,
            "invalid_path"
        );
    }

    #[test]
    fn developer_commands_and_aliases_are_registered_and_structured() {
        let mut app = test_app();
        assert!(exec(&mut app, "help toggleflycam").ok);
        assert!(exec(&mut app, "help togglecollisiongeometry").ok);
        assert!(exec(&mut app, "help fov").ok);
        let free_camera = exec(&mut app, "toggleflycam");
        assert_eq!(free_camera.value["camera_mode"], "free");
        assert_eq!(free_camera.log, ["Free camera enabled."]);
        let fps_camera = exec(&mut app, "tfc");
        assert_eq!(fps_camera.value["camera_mode"], "fps");
        assert_eq!(fps_camera.log, ["Free camera disabled."]);

        let collision_geometry = exec(&mut app, "togglecollisiongeometry");
        assert_eq!(collision_geometry.value["enabled"], true);
        assert_eq!(collision_geometry.log, ["Collision geometry enabled."]);
        let stair_debug = exec(&mut app, "stairdebug");
        assert_eq!(stair_debug.value["enabled"], true);
        assert_eq!(stair_debug.log, ["Stair debugging enabled."]);
        let unlit = exec(&mut app, "tunlit");
        assert_eq!(unlit.value["enabled"], true);
        assert_eq!(unlit.log, ["Unlit mode enabled."]);
        let lights = exec(&mut app, "tlights");
        assert_eq!(lights.value["lights_enabled"], false);
        assert_eq!(lights.log, ["Lights disabled."]);
    }

    #[test]
    fn render_settings_validate_boundaries_before_mutation() {
        let mut app = test_app();
        for (setting, low, high) in [
            ("lighting", 0.0001, 262_144.0),
            ("irradiance", 0.0, 4096.0),
            ("ambient", 0.0001, 4096.0),
            ("bloom_intensity", 0.0, 1.0),
            ("bloom_softness", 0.0, 1.0),
            ("fog", 0.0, 1.0),
            ("ao", 0.0, 1.0),
        ] {
            assert!(exec(&mut app, &format!("setrender {setting} {low}")).ok);
            assert!(exec(&mut app, &format!("setrender {setting} {high}")).ok);
        }
        assert!(exec(&mut app, "setrender shadow_samples 0").ok);
        assert_eq!(app.world().resource::<PointLightShadowSamples>().0, 0);
        assert!(exec(&mut app, "setrender shadow_samples 1").ok);
        assert_eq!(app.world().resource::<PointLightShadowSamples>().0, 1);
        assert!(!app.world().resource::<RealtimeShadowSettings>().enabled);
        assert!(exec(&mut app, "setrender realtime_shadows 1").ok);
        assert!(app.world().resource::<RealtimeShadowSettings>().enabled);
        assert!(exec(&mut app, "setrender realtime_shadows 0").ok);
        assert!(!app.world().resource::<RealtimeShadowSettings>().enabled);
        assert_eq!(
            exec(&mut app, "getrender realtime_shadows").value["value"],
            0
        );
        assert!(exec(&mut app, "setrender bloom_threshold 5000").ok);
        let before = app.world().resource::<LightingScale>().0;
        assert_eq!(
            exec(&mut app, "setrender lighting 0").error.unwrap().code,
            "out_of_range"
        );
        assert_eq!(app.world().resource::<LightingScale>().0, before);
        assert_eq!(
            exec(&mut app, "setrender lighting NaN").error.unwrap().code,
            "non_finite"
        );
        assert_eq!(app.world().resource::<LightingScale>().0, before);
        assert_eq!(
            exec(&mut app, "setrender unknown 1").error.unwrap().code,
            "unknown_setting"
        );
        assert_eq!(
            exec(&mut app, "setrender shadow_samples 2")
                .error
                .unwrap()
                .code,
            "out_of_range"
        );
        assert!(exec(&mut app, "shadowcache status").ok);
        assert_eq!(
            exec(&mut app, "shadowcache rebuild").error.unwrap().code,
            "prepare_required"
        );
        assert_eq!(
            exec(&mut app, "getrender").value.as_object().unwrap().len(),
            10
        );
    }

    #[test]
    fn tonemap_reports_changes_and_rejects_invalid_requests_without_mutation() {
        let mut app = test_app();
        let initial = exec(&mut app, "tonemap");
        assert_eq!(initial.value["tonemapper"], "agx");
        assert_eq!(initial.log, ["Tonemapper is AgX."]);

        for (input, expected, display) in [
            ("none", "none", "None"),
            ("reinhard", "reinhard", "Reinhard"),
            (
                "reinhard_luminance",
                "reinhard_luminance",
                "Reinhard Luminance",
            ),
            ("aces_fitted", "aces_fitted", "ACES Fitted"),
            ("agx", "agx", "AgX"),
            (
                "somewhat_boring_display_transform",
                "somewhat_boring_display_transform",
                "Somewhat Boring Display Transform",
            ),
            ("tony_mc_mapface", "tony_mc_mapface", "Tony McMapface"),
            ("blender_filmic", "blender_filmic", "Blender Filmic"),
            (
                "khronos_pbr_neutral",
                "khronos_pbr_neutral",
                "Khronos PBR Neutral",
            ),
        ] {
            let changed = exec(&mut app, &format!("tonemap {input}"));
            assert_eq!(changed.value["tonemapper"], expected);
            assert_eq!(changed.log, [format!("Tonemapper set to {display}.")]);
            assert_eq!(
                current_tonemapper(&mut app),
                parse_tonemapper(expected).unwrap()
            );
        }

        let case_insensitive = exec(&mut app, "tonemap AGX");
        assert_eq!(case_insensitive.value["tonemapper"], "agx");
        assert_eq!(current_tonemapper(&mut app), Tonemapping::AgX);

        let before_invalid = current_tonemapper(&mut app);
        assert_eq!(
            exec(&mut app, "tonemap unsupported").error.unwrap().code,
            "unknown_tonemapper"
        );
        assert_eq!(current_tonemapper(&mut app), before_invalid);
        assert_eq!(
            exec(&mut app, "tonemap agx extra").error.unwrap().code,
            "bad_arity"
        );
        assert_eq!(current_tonemapper(&mut app), before_invalid);
    }

    #[test]
    fn tonemap_rejects_missing_or_ambiguous_cameras() {
        let mut missing = test_app();
        let camera = {
            let world = missing.world_mut();
            let mut query = world.query_filtered::<Entity, With<Camera3d>>();
            query.single(world).unwrap()
        };
        missing.world_mut().despawn(camera);
        assert_eq!(
            exec(&mut missing, "tonemap agx").error.unwrap().code,
            "camera_unavailable"
        );

        let mut ambiguous = test_app();
        ambiguous
            .world_mut()
            .spawn((Camera3d::default(), Tonemapping::TonyMcMapface));
        let before = {
            let world = ambiguous.world_mut();
            let mut query = world.query_filtered::<&Tonemapping, With<Camera3d>>();
            *query.iter(world).next().unwrap()
        };
        assert_eq!(
            exec(&mut ambiguous, "tonemap none").error.unwrap().code,
            "camera_unavailable"
        );
        let after = {
            let world = ambiguous.world_mut();
            let mut query = world.query_filtered::<&Tonemapping, With<Camera3d>>();
            *query.iter(world).next().unwrap()
        };
        assert_eq!(after, before);
    }

    #[test]
    fn forced_no_clip_cannot_enable_unavailable_collision() {
        let mut app = test_app();
        app.world_mut().resource_mut::<player::PlayerNoClip>().0 = true;
        app.world_mut().resource_mut::<player::PhysicsDisabled>().0 = true;
        assert_eq!(
            exec(&mut app, "tcl").error.unwrap().code,
            "physics_disabled"
        );
        assert!(app.world().resource::<player::PlayerNoClip>().0);

        app.world_mut().resource_mut::<player::PhysicsDisabled>().0 = false;
        app.world_mut()
            .resource_mut::<player::CameraModeState>()
            .collisions_ready = false;
        assert_eq!(
            exec(&mut app, "tcl").error.unwrap().code,
            "collision_unavailable"
        );
        assert!(app.world().resource::<player::PlayerNoClip>().0);
    }

    #[test]
    fn console_suppression_preserves_tm_and_tdt_state() {
        let mut app = test_app();
        let game_ui = app.world_mut().spawn((GameUi, Visibility::Inherited)).id();
        let diagnostic_ui = app
            .world_mut()
            .spawn((DiagnosticUi, Visibility::Inherited))
            .id();

        app.world_mut()
            .resource_mut::<NextState<GameplayModal>>()
            .set(GameplayModal::Console);
        app.update();
        assert_eq!(
            app.world().get::<Visibility>(game_ui),
            Some(&Visibility::Hidden)
        );
        assert_eq!(
            app.world().get::<Visibility>(diagnostic_ui),
            Some(&Visibility::Hidden)
        );

        assert!(exec(&mut app, "tm").ok);
        app.world_mut()
            .resource_mut::<NextState<GameplayModal>>()
            .set(GameplayModal::None);
        app.update();
        assert_eq!(
            app.world().get::<Visibility>(game_ui),
            Some(&Visibility::Hidden)
        );
        assert_eq!(
            app.world().get::<Visibility>(diagnostic_ui),
            Some(&Visibility::Inherited)
        );

        assert!(exec(&mut app, "tm").ok);
        assert!(exec(&mut app, "tdt").ok);
        app.world_mut()
            .resource_mut::<NextState<GameplayModal>>()
            .set(GameplayModal::Console);
        app.update();
        app.world_mut()
            .resource_mut::<NextState<GameplayModal>>()
            .set(GameplayModal::None);
        app.update();
        assert_eq!(
            app.world().get::<Visibility>(game_ui),
            Some(&Visibility::Inherited)
        );
        assert_eq!(
            app.world().get::<Visibility>(diagnostic_ui),
            Some(&Visibility::Hidden)
        );
    }

    // -- activate (scripted door travel, M2 wave 2) -----------------------

    /// Registers a placement built from minimal RON (avoids widening the
    /// `vsa` re-export surface just for test constructors) under FormID
    /// 0x10 / EditorID "TestRef".
    fn register_placement(app: &mut App, semantic_ron: &str) {
        let ron = format!(
            "(
                reference_form_id: 16,
                base_form_id: 1,
                asset_path: None,
                translation: (0.0, 0.0, 0.0),
                rotation_xyzw: (0.0, 0.0, 0.0, 1.0),
                scale: 1.0,
                error: None,
                semantic: {semantic_ron},
            )"
        );
        let placement: crate::vsa::PreparedPlacement = ron::de::from_str(&ron).unwrap();
        let entity = app
            .world_mut()
            .spawn((
                Transform::default(),
                interaction::PlacementRoot::new(placement),
            ))
            .id();
        app.world_mut()
            .resource_mut::<crate::console::RefRegistry>()
            .register(entity, 0x10, Some("TestRef"));
    }

    const DOOR_WITH_DESTINATION: &str = "Door((
        lock_level: None,
        key_form_id: None,
        destination: Some((
            door_reference_form_id: 32,
            cell_form_id: 148753,
            translation: (1.0, 2.0, 3.0),
            rotation_xyzw: (0.0, 0.0, 0.0, 1.0),
        )),
    ))";

    fn error_code(output: &crate::console::ConsoleOutput) -> String {
        output
            .error
            .as_ref()
            .expect("expected an error")
            .code
            .clone()
    }

    #[test]
    fn activate_requires_exactly_one_reference() {
        let mut app = test_app();
        assert_eq!(error_code(&exec(&mut app, "activate")), "bad_arity");
        assert_eq!(error_code(&exec(&mut app, "activate a b")), "bad_arity");
    }

    #[test]
    fn activate_rejects_non_door_and_destination_less_references() {
        let mut app = test_app();
        register_placement(&mut app, "Static");
        assert_eq!(
            error_code(&exec(&mut app, "activate 00000010")),
            "not_a_door"
        );

        let mut app = test_app();
        register_placement(
            &mut app,
            "Door((lock_level: None, key_form_id: None, destination: None))",
        );
        assert_eq!(
            error_code(&exec(&mut app, "activate TestRef")),
            "no_destination"
        );
    }

    // Wave-4 amendment: containers toggle their open state through the
    // console, so the persistence gate can be driven over the agent bridge.
    #[test]
    fn activate_toggles_a_container_open_and_closed() {
        let mut app = test_app();
        app.add_message::<super::super::audio::PlaySound>();
        app.add_message::<super::super::animation::PlayPlacementAnimation>();
        register_placement(&mut app, "Container");
        let output = exec(&mut app, "activate 00000010");
        assert!(output.ok, "activate failed: {:?}", output.error);
        assert_eq!(output.value["opened"], true);
        let output = exec(&mut app, "activate 00000010");
        assert!(output.ok, "activate failed: {:?}", output.error);
        assert_eq!(output.value["opened"], false);
    }

    fn register_corpse_placement(app: &mut App) {
        let ron = r#"(
            reference_form_id: 16,
            base_form_id: 1,
            asset_path: None,
            translation: (0.0, 0.0, 0.0),
            rotation_xyzw: (0.0, 0.0, 0.0, 1.0),
            scale: 1.0,
            error: None,
            semantic: Corpse,
            inventory: [
                (base_form_id: 2, count: 3, record_kind: "MISC", editor_id: Some("CorpseItem"), display_name: Some("Corpse Item"), leveled: false),
            ],
        )"#;
        let placement: crate::vsa::PreparedPlacement = ron::de::from_str(ron).unwrap();
        let entity = app
            .world_mut()
            .spawn((
                Transform::default(),
                interaction::PlacementRoot::new(placement),
            ))
            .id();
        app.world_mut()
            .resource_mut::<crate::console::RefRegistry>()
            .register(entity, 0x10, Some("TestCorpse"));
    }

    // F118.2: scripted corpse activation uses the loot-holder transfer seam,
    // seeds the stable FormID-keyed state, and requests the existing modal.
    #[test]
    fn activate_opens_a_corpse_holder_with_stable_console_output() {
        let mut app = test_app();
        app.add_message::<super::super::audio::PlaySound>();
        app.add_message::<super::super::animation::PlayPlacementAnimation>();
        app.add_message::<crate::app_state::RequestStateTransition>();
        register_corpse_placement(&mut app);

        let output = exec(&mut app, "activate TestCorpse");
        assert!(output.ok, "activate failed: {:?}", output.error);
        assert_eq!(output.value["reference_form_id"], 16);
        assert_eq!(output.value["kind"], "corpse");
        assert_eq!(output.value["opened"], true);
        assert_eq!(output.log, ["corpse 00000010 opened"]);
        assert_eq!(
            app.world().resource::<interaction::ContainerStates>().0[&0x10].stacks,
            vec![(0x2, 3)]
        );
        let requests = app
            .world()
            .resource::<Messages<crate::app_state::RequestStateTransition>>();
        assert!(requests.iter_current_update_messages().any(|request| {
            *request == crate::app_state::RequestStateTransition::Modal(GameplayModal::Container)
        }));
    }

    // F118.2: non-corpse actor references remain unsupported until the actor
    // simulation/death slice exists; the error is deterministic.
    #[test]
    fn activate_rejects_a_live_actor_as_a_corpse_holder() {
        let mut app = test_app();
        register_placement(&mut app, "Npc((base_template_form_id: None))");
        assert_eq!(
            error_code(&exec(&mut app, "activate TestRef")),
            "not_a_door"
        );
    }

    // -- activate pickup (issue #84, F84.2) --------------------------------

    const PICKUP_SEMANTIC: &str = "Pickup((category: \"Misc\", value: None, weight: None))";

    #[test]
    fn activate_picks_up_a_prepared_item_and_despawns_the_reference() {
        let mut app = test_app();
        register_placement(&mut app, PICKUP_SEMANTIC);
        let entity = app
            .world_mut()
            .query_filtered::<Entity, With<interaction::PlacementRoot>>()
            .single(app.world())
            .unwrap();
        let output = exec(&mut app, "activate 00000010");
        assert!(output.ok, "activate failed: {:?}", output.error);
        assert_eq!(output.value["reference_form_id"], 16);
        assert_eq!(output.value["base_form_id"], 1);
        assert_eq!(output.value["count"], 1);
        assert_eq!(output.log, ["picked up 00000001 x1"]);
        assert_eq!(
            app.world()
                .resource::<interaction::PlayerInventory>()
                .count(1),
            1
        );
        assert!(!app.world().entities().contains(entity));
    }

    // The pickup route only accepts `PreparedSemantic::Pickup`; every other
    // unsupported semantic (Furniture here, matching the door/container
    // routes rejecting `Static` above) still errors deterministically.
    #[test]
    fn activate_rejects_unsupported_semantics_for_pickup() {
        let mut app = test_app();
        register_placement(&mut app, "Furniture");
        assert_eq!(
            error_code(&exec(&mut app, "activate 00000010")),
            "not_a_door"
        );
    }

    #[test]
    fn activate_door_with_destination_writes_a_travel_request() {
        let mut app = test_app();
        app.add_message::<interaction::DoorTravelRequested>();
        register_placement(&mut app, DOOR_WITH_DESTINATION);
        let output = exec(&mut app, "activate 00000010");
        assert!(output.ok, "activate failed: {:?}", output.error);
        assert_eq!(output.value["destination_cell_form_id"], 148753);
        let requests = app
            .world()
            .resource::<Messages<interaction::DoorTravelRequested>>();
        let request = requests
            .iter_current_update_messages()
            .next()
            .expect("expected a DoorTravelRequested message");
        assert_eq!(request.destination_cell_form_id, 148753);
        assert_eq!(request.translation, Vec3::new(1.0, 2.0, 3.0));
    }

    // -- save (issue #60, F60.3) ------------------------------------------

    #[test]
    fn save_validates_arity_and_fails_deterministically_without_a_world() {
        let mut app = test_app();
        assert_eq!(error_code(&exec(&mut app, "save")), "bad_arity");
        assert_eq!(error_code(&exec(&mut app, "save a b")), "bad_arity");
        // The console harness has no active cell, so the failure is a
        // structured error rather than a panic.
        assert_eq!(error_code(&exec(&mut app, "save slot1")), "save_failed");
    }

    /// Wave-3 amendment: a door with a discovered Open clip stages its travel
    /// behind the open lead instead of firing it the same frame, matching the
    /// player's Enter activation.
    #[test]
    fn activate_animated_door_defers_travel_by_the_open_lead() {
        let mut app = test_app();
        app.add_message::<interaction::DoorTravelRequested>();
        register_placement(&mut app, DOOR_WITH_DESTINATION);
        let root = app
            .world_mut()
            .query_filtered::<Entity, With<interaction::PlacementRoot>>()
            .single(app.world())
            .unwrap();
        let player = app.world_mut().spawn_empty().id();
        app.world_mut().entity_mut(root).insert(
            super::super::animation::AnimatedPlacement::for_test(player, &[("Open", 1.33)]),
        );
        let output = exec(&mut app, "activate 00000010");
        assert!(output.ok, "activate failed: {:?}", output.error);
        let lead = output.value["open_lead_ms"].as_f64().unwrap();
        assert!(lead > 0.0, "expected a nonzero open lead, got {lead}");
        let requests = app
            .world()
            .resource::<Messages<interaction::DoorTravelRequested>>();
        assert_eq!(
            requests.iter_current_update_messages().count(),
            0,
            "travel must be staged behind the open lead, not fired same-frame"
        );
    }

    // -- additem (issue #84, F84.1) -----------------------------------------

    #[test]
    fn additem_rejects_bad_arity_bad_count_and_bad_form_id() {
        let mut app = test_app();
        assert_eq!(error_code(&exec(&mut app, "additem")), "bad_arity");
        assert_eq!(error_code(&exec(&mut app, "additem 1 2 3")), "bad_arity");
        assert_eq!(
            error_code(&exec(&mut app, "additem 00000001 0")),
            "bad_count"
        );
        assert_eq!(
            error_code(&exec(&mut app, "additem 00000001 -1")),
            "bad_count"
        );
        // Bare short hex is a valid FormID, so an invalid count is reported
        // as bad_count, not masked by the FormID parse.
        assert_eq!(error_code(&exec(&mut app, "additem f -5")), "bad_count");
        assert_eq!(error_code(&exec(&mut app, "additem zz")), "bad_type");
        assert_eq!(
            error_code(&exec(&mut app, "additem 123456789 1")),
            "bad_type"
        );
        assert_eq!(
            app.world()
                .resource::<interaction::PlayerInventory>()
                .count(1),
            0
        );
        assert_eq!(
            app.world()
                .resource::<interaction::PlayerInventory>()
                .count(0xf),
            0
        );
    }

    // Issue #84's gate evidence line is literally `player.additem f 100`:
    // the Bethesda console accepts bare short hex without a 0x prefix, and
    // the player. prefix is accepted without selecting a container.
    #[test]
    fn additem_accepts_bare_short_hex_and_the_player_prefix() {
        let mut app = test_app();
        let player = app.world_mut().spawn(Transform::default()).id();
        app.world_mut()
            .resource_mut::<crate::console::RefRegistry>()
            .set_player(player);
        let output = exec(&mut app, "player.additem f 100");
        assert!(output.ok, "additem failed: {:?}", output.error);
        assert_eq!(output.value["form_id"], 15);
        assert_eq!(output.value["count"], 100);
        assert_eq!(output.value["total"], 100);
        assert_eq!(output.log, ["additem 0000000f x100; inventory now has 100"]);
        assert_eq!(
            app.world()
                .resource::<interaction::PlayerInventory>()
                .count(0xf),
            100
        );
    }

    #[test]
    fn additem_adds_the_default_count_of_one() {
        let mut app = test_app();
        let output = exec(&mut app, "additem 00000005");
        assert!(output.ok, "additem failed: {:?}", output.error);
        assert_eq!(output.value["form_id"], 5);
        assert_eq!(output.value["count"], 1);
        assert_eq!(output.value["total"], 1);
        assert_eq!(output.log, ["additem 00000005 x1; inventory now has 1"]);
        assert_eq!(
            app.world()
                .resource::<interaction::PlayerInventory>()
                .count(5),
            1
        );
    }

    #[test]
    fn additem_adds_a_requested_count_seeded_with_catalog_condition() {
        let mut app = test_app();
        app.insert_resource(PreparedItemCatalog {
            revision: "test".into(),
            source_fingerprint: "test".into(),
            items: vec![PreparedItemDefinition {
                base_form_id: 0x42,
                record_kind: "WEAP".into(),
                category: PreparedItemCategory::Weapons,
                editor_id: None,
                display_name: None,
                source_model_path: None,
                icon_asset_path: None,
                world_asset_path: None,
                physics_asset_path: None,
                drop_collider: Default::default(),
                value: None,
                weight: None,
                quest_item: false,
                stats: PreparedItemStats::Weapon {
                    damage: None,
                    max_condition: Some(255),
                    clip_size: None,
                    speed: None,
                    reach: None,
                    ammo_form_id: None,
                },
                audio: Default::default(),
            }],
        });
        let output = exec(&mut app, "additem 00000042 3");
        assert!(output.ok, "additem failed: {:?}", output.error);
        assert_eq!(output.value["count"], 3);
        assert_eq!(output.value["total"], 3);
        assert_eq!(output.log, ["additem 00000042 x3; inventory now has 3"]);
        let stack = app
            .world()
            .resource::<interaction::PlayerInventory>()
            .stack_states()
            .into_iter()
            .find(|stack| stack.base_form_id == 0x42)
            .expect("expected the added stack");
        assert_eq!(stack.count, 3);
        assert_eq!(stack.condition, Some(255));
    }

    #[test]
    fn additem_without_a_catalog_entry_adds_with_no_condition() {
        let mut app = test_app();
        let output = exec(&mut app, "additem 000000aa 2");
        assert!(output.ok, "additem failed: {:?}", output.error);
        assert_eq!(output.value["total"], 2);
        let stack = app
            .world()
            .resource::<interaction::PlayerInventory>()
            .stack_states()
            .into_iter()
            .find(|stack| stack.base_form_id == 0xaa)
            .expect("expected the added stack");
        assert_eq!(stack.count, 2);
        assert_eq!(stack.condition, None);
    }

    // -- equipitem (issue #98, F98.4) --------------------------------------

    fn apparel_item(base_form_id: u32, biped_slot_mask: u32) -> PreparedItemDefinition {
        PreparedItemDefinition {
            base_form_id,
            record_kind: "ARMO".into(),
            category: PreparedItemCategory::Apparel,
            editor_id: None,
            display_name: None,
            source_model_path: None,
            icon_asset_path: None,
            world_asset_path: None,
            physics_asset_path: None,
            drop_collider: Default::default(),
            value: None,
            weight: None,
            quest_item: false,
            stats: PreparedItemStats::Apparel {
                armor_rating: None,
                max_condition: None,
                biped_slot_mask: Some(biped_slot_mask),
            },
            audio: Default::default(),
        }
    }

    #[test]
    fn equipitem_rejects_bad_arity_and_bad_form_id() {
        let mut app = test_app();
        assert_eq!(error_code(&exec(&mut app, "equipitem")), "bad_arity");
        assert_eq!(error_code(&exec(&mut app, "equipitem 1 2")), "bad_arity");
        assert_eq!(error_code(&exec(&mut app, "equipitem zz")), "bad_type");
    }

    #[test]
    fn equipitem_rejects_a_form_id_not_in_the_inventory() {
        let mut app = test_app();
        assert_eq!(
            error_code(&exec(&mut app, "equipitem 00000001")),
            "not_in_inventory"
        );
    }

    #[test]
    fn equipitem_rejects_a_form_id_with_no_catalog_entry() {
        let mut app = test_app();
        exec(&mut app, "additem 00000001");
        assert_eq!(
            error_code(&exec(&mut app, "equipitem 00000001")),
            "no_catalog_entry"
        );
    }

    #[test]
    fn equipitem_equips_and_toggling_again_unequips() {
        let mut app = test_app();
        app.insert_resource(PreparedItemCatalog {
            revision: "test".into(),
            source_fingerprint: "test".into(),
            items: vec![apparel_item(0x10, 0x1)],
        });
        exec(&mut app, "additem 00000010");
        let output = exec(&mut app, "player.equipitem 00000010");
        assert!(output.ok, "equipitem failed: {:?}", output.error);
        assert_eq!(output.value["equipped"], true);
        assert_eq!(output.log, ["equipitem 00000010 equipped"]);
        assert!(
            app.world()
                .resource::<interaction::PlayerEquipment>()
                .is_equipped(super::super::inventory::StackKey {
                    base_form_id: 0x10,
                    condition: None,
                })
        );

        let output = exec(&mut app, "equipitem 00000010");
        assert!(output.ok, "equipitem failed: {:?}", output.error);
        assert_eq!(output.value["equipped"], false);
        assert_eq!(output.log, ["equipitem 00000010 unequipped"]);
        assert!(
            !app.world()
                .resource::<interaction::PlayerEquipment>()
                .is_equipped(super::super::inventory::StackKey {
                    base_form_id: 0x10,
                    condition: None,
                })
        );
    }

    #[test]
    fn equipitem_evicts_the_previous_occupant_of_a_shared_slot() {
        let mut app = test_app();
        app.insert_resource(PreparedItemCatalog {
            revision: "test".into(),
            source_fingerprint: "test".into(),
            items: vec![apparel_item(0x10, 0x1), apparel_item(0x11, 0x1)],
        });
        exec(&mut app, "additem 00000010");
        exec(&mut app, "additem 00000011");
        exec(&mut app, "equipitem 00000010");
        let output = exec(&mut app, "equipitem 00000011");
        assert!(output.ok, "equipitem failed: {:?}", output.error);
        assert_eq!(output.value["evicted"], json!(["00000010"]));
    }

    #[test]
    fn equipitem_rejects_ammo_with_no_weapon_equipped() {
        let mut app = test_app();
        app.insert_resource(PreparedItemCatalog {
            revision: "test".into(),
            source_fingerprint: "test".into(),
            items: vec![PreparedItemDefinition {
                base_form_id: 0x20,
                record_kind: "AMMO".into(),
                category: PreparedItemCategory::Ammo,
                editor_id: None,
                display_name: None,
                source_model_path: None,
                icon_asset_path: None,
                world_asset_path: None,
                physics_asset_path: None,
                drop_collider: Default::default(),
                value: None,
                weight: None,
                quest_item: false,
                stats: PreparedItemStats::Ammo {
                    damage: None,
                    speed: None,
                },
                audio: Default::default(),
            }],
        });
        exec(&mut app, "additem 00000020");
        assert_eq!(
            error_code(&exec(&mut app, "equipitem 00000020")),
            "no_weapon_equipped"
        );
    }

    #[test]
    fn canonical_instance_commands_update_ledger_and_runtime_projections() {
        let mut app = test_app();
        app.insert_resource(PreparedItemCatalog {
            revision: "test".into(),
            source_fingerprint: "test".into(),
            items: vec![apparel_item(0x10, 0x1)],
        });
        assert!(exec(&mut app, "additem 00000010").ok);
        let item_id = app
            .world()
            .resource::<interaction::CanonicalItemLedger>()
            .ledger
            .holders()[&HolderId::Player]
            .items[0]
            .id;
        let key = super::super::inventory::StackKey {
            base_form_id: 0x10,
            condition: None,
        };

        let equipped = exec(&mut app, &format!("equip {:016x}", item_id.0));
        assert!(equipped.ok, "equip failed: {:?}", equipped.error);
        assert_eq!(
            app.world()
                .resource::<interaction::CanonicalItemLedger>()
                .ledger
                .bindings()[&HolderId::Player]
                .equipped,
            Some(item_id)
        );
        assert!(
            app.world()
                .resource::<interaction::PlayerEquipment>()
                .is_equipped(key)
        );

        let hotkey = exec(&mut app, &format!("hotkey 0 {:016x}", item_id.0));
        assert!(hotkey.ok, "hotkey failed: {:?}", hotkey.error);
        assert_eq!(
            app.world()
                .resource::<interaction::CanonicalItemLedger>()
                .ledger
                .bindings()[&HolderId::Player]
                .hotkeys[0],
            Some(item_id)
        );
        assert_eq!(
            app.world()
                .resource::<super::super::bindings::HotkeyBindings>()
                .get(1),
            Some(key)
        );

        let unequipped = exec(&mut app, "unequip");
        assert!(unequipped.ok, "unequip failed: {:?}", unequipped.error);
        assert_eq!(
            app.world()
                .resource::<interaction::CanonicalItemLedger>()
                .ledger
                .bindings()[&HolderId::Player]
                .equipped,
            None
        );
        assert!(
            !app.world()
                .resource::<interaction::PlayerEquipment>()
                .is_equipped(key)
        );
    }

    #[test]
    fn canonical_instance_commands_fail_without_partial_mutation() {
        let mut app = test_app();
        assert!(exec(&mut app, "additem 00000010").ok);
        let item_id = app
            .world()
            .resource::<interaction::CanonicalItemLedger>()
            .ledger
            .holders()[&HolderId::Player]
            .items[0]
            .id;
        let before_ledger = app
            .world()
            .resource::<interaction::CanonicalItemLedger>()
            .snapshot();
        let before_equipped = app
            .world()
            .resource::<interaction::PlayerEquipment>()
            .is_equipped(super::super::inventory::StackKey {
                base_form_id: 0x10,
                condition: None,
            });
        assert_eq!(
            error_code(&exec(&mut app, &format!("equip {:016x}", item_id.0))),
            "no_catalog_entry"
        );
        assert_eq!(
            app.world()
                .resource::<interaction::CanonicalItemLedger>()
                .snapshot(),
            before_ledger
        );
        assert_eq!(
            app.world()
                .resource::<interaction::PlayerEquipment>()
                .is_equipped(super::super::inventory::StackKey {
                    base_form_id: 0x10,
                    condition: None,
                }),
            before_equipped
        );

        assert_eq!(
            error_code(&exec(&mut app, "hotkey 0 deadbeef")),
            "item_not_found"
        );
        assert_eq!(
            app.world()
                .resource::<interaction::CanonicalItemLedger>()
                .snapshot(),
            before_ledger
        );
        assert!(
            app.world()
                .get_resource::<super::super::bindings::HotkeyBindings>()
                .is_none()
        );
    }
}
