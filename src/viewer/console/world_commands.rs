//! World-reference activation and actor-debug console commands.

use super::*;

pub(super) struct WorldCommandProvider;

impl ConsoleCommandProvider for WorldCommandProvider {
    fn register_commands(&self, registry: &mut ConsoleRegistry) -> Result<(), ConsoleError> {
        for command in [
            ConsoleCommand::new("ragdoll", "ragdoll <actor-reference> [on|off|reset]", "Toggle a prepared NPC/creature's developer ragdoll body; actors stay locked in T-pose by default.", ragdoll).mutating(),
            ConsoleCommand::new("activate", "activate <reference>", "Activate a door, container, corpse, or pickup reference; a door with a destination requests cell travel (locks bypassed).", activate_reference).mutating(),
            ConsoleCommand::new(
                "tp",
                "tp <x> <y> <z> [<cell-formid>]",
                "Atomically teleport the player to (x, y, z) in metres, optionally after swapping to a prepared cell first.",
                teleport_player,
            )
            .mutating(),
        ] {
            registry.register(command)?;
        }
        Ok(())
    }
}

/// Scripted door activation for the agent bridge (M2 wave 2 acceptance):
/// resolves a door reference and requests the same `DoorTravelRequested`
/// cell travel that the player's Enter activation produces, skipping the
/// raycast focus, distance, and lock checks (this is a developer command).
pub(super) fn activate_reference(
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

pub(super) fn ragdoll(
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

/// Mirrors `world::preload::scene_manifest_path` (private to that module).
/// Duplicated here only for `tp`'s synchronous prepared-cell existence
/// check -- the actual swap/loader logic itself is not duplicated; `tp`
/// drives the exact same `DoorTravelRequested` message and `world::swap`
/// systems door travel and `activate` already use (see `teleport_player`
/// below).
pub(super) fn tp_scene_manifest_path(asset_root: &std::path::Path, form_id: u32) -> PathBuf {
    asset_root
        .join("scenes")
        .join(format!("{form_id:08x}"))
        .join("scene.ron")
}

pub(super) fn finite_tp_coordinate(value: &str) -> Result<f32, ConsoleError> {
    let parsed = value
        .parse::<f32>()
        .map_err(|_| ConsoleError::new("bad_type", "tp coordinates must be finite numbers"))?;
    if parsed.is_finite() {
        Ok(parsed)
    } else {
        Err(ConsoleError::new(
            "bad_type",
            "tp coordinates must be finite numbers",
        ))
    }
}

/// Issue #152: sets all three axes of the player's `Transform` in one
/// write -- the same mechanism `[player.]setpos` uses
/// (`console::commands::set_position`/`notify_transform_mutated`), just
/// without the intermediate single-axis states that let a mid-sequence
/// physics tick see the player poking through geometry. `player::
/// console_transform_mutated` is the only hook `viewer::console::install`
/// registers with `ConsoleEntityHooks`, so calling it directly here is
/// equivalent to the generic `notify_transform_mutated` dispatch `setpos`
/// goes through (that dispatcher itself is private to `crate::console` and
/// not re-exported for callers outside it).
pub(super) fn teleport_player_in_place(
    world: &mut World,
    position: Vec3,
) -> Result<ConsoleCommandResult, ConsoleError> {
    let entity = resolve_reference(world, "player")?;
    {
        let mut transform = world
            .get_mut::<Transform>(entity)
            .ok_or_else(|| ConsoleError::new("component_missing", "reference has no Transform"))?;
        transform.translation = position;
    }
    player::console_transform_mutated(world, entity);
    Ok(ConsoleCommandResult::new(
        json!({
            "x": position.x,
            "y": position.y,
            "z": position.z,
            "unit": "metres",
        }),
        vec![format!(
            "tp: teleported player to ({:.3}, {:.3}, {:.3}).",
            position.x, position.y, position.z
        )],
    ))
}

/// Issue #152: `tp <x> <y> <z> [<cell-formid>]`. The 3-arg form is an
/// atomic `setpos` (see `teleport_player_in_place`). The 4-arg form reuses
/// the exact same instant-swap/cell-travel machinery door activation and
/// `activate` already drive -- a `DoorTravelRequested` message consumed by
/// `world::swap`'s `evaluate_door_travel_requests` /
/// `apply_pending_instant_swap` (or the loading-screen fallback for a
/// prepared-but-not-resident cell) -- rather than a new loader. Its
/// `translation` is the requested (x, y, z) directly, so `world::swap`'s
/// `activate_resident_cell` places the player there via `player::
/// teleport_active_player` as part of the very same swap a door uses; a
/// synthetic `door_form_id: 0` (never a real FormID) marks it as not a real
/// door for `nav::agent::note_player_swap_door`'s follow-through/freeze
/// bookkeeping. A destination already equal to the active cell skips the
/// swap and falls back to the plain in-place teleport above. An unprepared
/// destination cell fails synchronously and deterministically, matching
/// `activate`'s unknown-reference error style, instead of silently kicking
/// off a fallback load that would only fail later.
pub(super) fn teleport_player(
    world: &mut World,
    invocation: &ConsoleInvocation,
) -> Result<ConsoleCommandResult, ConsoleError> {
    let (coordinates, cell_arg) = match invocation.args.as_slice() {
        [x, y, z] => ([x, y, z], None),
        [x, y, z, cell] => ([x, y, z], Some(cell)),
        _ => {
            return Err(ConsoleError::new(
                "bad_arity",
                "tp requires <x> <y> <z> and an optional cell FormID",
            ));
        }
    };
    let position = Vec3::new(
        finite_tp_coordinate(coordinates[0])?,
        finite_tp_coordinate(coordinates[1])?,
        finite_tp_coordinate(coordinates[2])?,
    );
    let Some(cell_value) = cell_arg else {
        return teleport_player_in_place(world, position);
    };
    let destination_cell = parse_item_form_id(cell_value).ok_or_else(|| {
        ConsoleError::new(
            "bad_type",
            "tp cell FormID must be 1-8 hex digits, e.g. f, 0x1f, or 0000000f",
        )
    })?;
    if world
        .get_resource::<super::super::world::ActiveCell>()
        .is_some_and(|active| active.0 == destination_cell)
    {
        return teleport_player_in_place(world, position);
    }
    let asset_root = world
        .get_resource::<crate::viewer::LoadedSceneManifest>()
        .map(|manifest| PathBuf::from(&manifest.0.asset_root))
        .ok_or_else(|| {
            ConsoleError::new("cell_unavailable", "no active cell manifest is loaded")
        })?;
    if !tp_scene_manifest_path(&asset_root, destination_cell).is_file() {
        return Err(ConsoleError::new(
            "cell_not_found",
            format!("cell {destination_cell:08x} is not a prepared cell"),
        ));
    }
    let rotation_xyzw = resolve_reference(world, "player")
        .ok()
        .and_then(|entity| world.get::<Transform>(entity))
        .map(|transform| transform.rotation.to_array())
        .unwrap_or([0.0, 0.0, 0.0, 1.0]);
    world.write_message(interaction::DoorTravelRequested {
        destination_cell_form_id: destination_cell,
        translation: position,
        rotation_xyzw,
        door_form_id: 0,
    });
    Ok(ConsoleCommandResult::new(
        json!({
            "x": position.x,
            "y": position.y,
            "z": position.z,
            "cell_form_id": destination_cell,
            "unit": "metres",
        }),
        vec![format!(
            "tp: cell travel requested to {destination_cell:08x}; player will be placed at ({:.3}, {:.3}, {:.3}).",
            position.x, position.y, position.z
        )],
    ))
}
