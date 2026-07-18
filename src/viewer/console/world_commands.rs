//! World-reference activation and actor-debug console commands.

use super::*;

pub(super) struct WorldCommandProvider;

impl ConsoleCommandProvider for WorldCommandProvider {
    fn register_commands(&self, registry: &mut ConsoleRegistry) -> Result<(), ConsoleError> {
        for command in [
            ConsoleCommand::new("ragdoll", "ragdoll <actor-reference> [on|off|reset]", "Toggle a prepared NPC/creature's developer ragdoll body; actors stay locked in T-pose by default.", ragdoll).mutating(),
            ConsoleCommand::new("ragdollprobe", "ragdollprobe <actor-reference>", "Report live ragdoll constraint, velocity, sleep, and visual-node errors without changing the actor.", ragdoll_probe),
            ConsoleCommand::new("activate", "activate <reference>", "Activate a door, container, corpse, or pickup reference; a door with a destination requests cell travel (locks bypassed).", activate_reference).mutating(),
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

pub(super) fn ragdoll_probe(
    world: &mut World,
    invocation: &ConsoleInvocation,
) -> Result<ConsoleCommandResult, ConsoleError> {
    if invocation.args.len() != 1 {
        return Err(ConsoleError::new(
            "bad_arity",
            "ragdollprobe requires exactly one actor reference",
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
            "ragdollprobe only accepts NPC or creature references",
        ));
    }
    let enabled = world
        .get::<player::RagdollToggle>(entity)
        .is_some_and(|toggle| toggle.0);
    let probe = player::probe_ragdoll(world, entity);
    let summary = format!(
        "ragdollprobe {:08x} enabled={} instantiated={} bodies={} awake={} sleep_disabled={} joints={} max_joint_linear={:.5}m max_joint_angular={:.5}rad max_node_position={:.5}m max_node_angle={:.5}rad max_linear_speed={:.5}m/s max_angular_speed={:.5}rad/s",
        placement.reference_form_id,
        enabled,
        probe.instantiated,
        probe.body_count,
        probe.awake_count,
        probe.sleep_disabled_count,
        probe.joint_count,
        probe.max_joint_linear_separation,
        probe.max_joint_angular_separation,
        probe.max_node_position_error,
        probe.max_node_angle_error,
        probe.max_linear_speed,
        probe.max_angular_speed,
    );
    let value = serde_json::to_value(&probe).map_err(|error| {
        ConsoleError::new("serialization_failed", format!("ragdoll probe: {error}"))
    })?;
    Ok(ConsoleCommandResult::new(
        json!({
            "reference_form_id": placement.reference_form_id,
            "enabled": enabled,
            "probe": value,
        }),
        vec![summary],
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
