use bevy::prelude::*;
use serde_json::{Value, json};

use super::executor::{
    adapted_angles, notify_transform_mutated, resolve_reference, set_adapted_angles,
};
use super::{
    ConsoleCommand, ConsoleCommandResult, ConsoleError, ConsoleInvocation, ConsoleRegistry,
    ConsoleSessionStore, RefRegistry,
};

pub(crate) fn register_engine_commands(registry: &mut ConsoleRegistry) {
    let commands = [
        ConsoleCommand::new(
            "help",
            "help [command]",
            "List commands or describe one command.",
            help,
        ),
        ConsoleCommand::new(
            "prid",
            "prid <player|FormID|EditorID>",
            "Select a live reference for this console session.",
            prid,
        )
        .mutating(),
        ConsoleCommand::new(
            "getformid",
            "[reference.]getformid",
            "Return the selected reference FormID.",
            get_form_id,
        )
        .reference_callable(true),
        ConsoleCommand::new(
            "getedid",
            "[reference.]getedid",
            "Return the selected reference EditorID.",
            get_editor_id,
        )
        .reference_callable(true),
        ConsoleCommand::new(
            "getpos",
            "[reference.]getpos [x|y|z]",
            "Return position in Bevy metres.",
            get_position,
        )
        .reference_callable(true),
        ConsoleCommand::new(
            "setpos",
            "[reference.]setpos <x|y|z> <metres>",
            "Set one position axis in Bevy metres.",
            set_position,
        )
        .reference_callable(true)
        .mutating(),
        ConsoleCommand::new(
            "getangle",
            "[reference.]getangle [x|y|z]",
            "Return Gamebryo-style Euler angles in degrees.",
            get_angle,
        )
        .reference_callable(true),
        ConsoleCommand::new(
            "setangle",
            "[reference.]setangle <x|y|z> <degrees>",
            "Set one Gamebryo-style Euler angle in degrees.",
            set_angle,
        )
        .reference_callable(true)
        .mutating(),
        ConsoleCommand::new(
            "moveto",
            "[reference.]moveto <player|FormID|EditorID>",
            "Move the selected reference to another live reference.",
            move_to,
        )
        .reference_callable(true)
        .mutating(),
        ConsoleCommand::new(
            "dump",
            "[reference.]dump [player|FormID|EditorID]",
            "Return stable identity, transform, and component information.",
            dump,
        )
        .reference_callable(false),
    ];
    for command in commands {
        registry
            .register(command)
            .expect("built-in command is unique");
    }
}

fn require_arity(
    invocation: &ConsoleInvocation,
    min: usize,
    max: usize,
) -> Result<(), ConsoleError> {
    if (min..=max).contains(&invocation.args.len()) {
        Ok(())
    } else {
        Err(ConsoleError::new(
            "bad_arity",
            format!(
                "{} expects {} argument(s); received {}",
                invocation.command,
                if min == max {
                    min.to_string()
                } else {
                    format!("{min}..={max}")
                },
                invocation.args.len()
            ),
        ))
    }
}

fn axis(value: &str) -> Result<usize, ConsoleError> {
    match value.to_ascii_lowercase().as_str() {
        "x" => Ok(0),
        "y" => Ok(1),
        "z" => Ok(2),
        _ => Err(ConsoleError::new(
            "bad_type",
            format!("'{value}' is not an axis; expected x, y, or z"),
        )),
    }
}

fn finite_f32(value: &str, label: &str) -> Result<f32, ConsoleError> {
    let parsed = value
        .parse::<f32>()
        .map_err(|_| ConsoleError::new("bad_type", format!("{label} must be a finite number")))?;
    if parsed.is_finite() {
        Ok(parsed)
    } else {
        Err(ConsoleError::new(
            "bad_type",
            format!("{label} must be a finite number"),
        ))
    }
}

fn target(invocation: &ConsoleInvocation) -> Result<Entity, ConsoleError> {
    invocation.target.ok_or_else(|| {
        ConsoleError::new(
            "no_selection",
            "select a reference with prid or use reference.command",
        )
    })
}

fn help(
    world: &mut World,
    invocation: &ConsoleInvocation,
) -> Result<ConsoleCommandResult, ConsoleError> {
    require_arity(invocation, 0, 1)?;
    let registry = world.resource::<ConsoleRegistry>();
    if let Some(name) = invocation.args.first() {
        let name = name.to_ascii_lowercase();
        let command = registry.resolve(&name).ok_or_else(|| {
            ConsoleError::new("unknown_command", format!("unknown command '{name}'"))
        })?;
        let metadata = command.metadata.clone();
        return Ok(ConsoleCommandResult::new(
            serde_json::to_value(&metadata).map_err(ConsoleError::internal)?,
            vec![format!("{} — {}", metadata.signature, metadata.help)],
        ));
    }
    let metadata = registry.metadata();
    let log = metadata
        .iter()
        .map(|command| format!("{} — {}", command.signature, command.help))
        .collect();
    Ok(ConsoleCommandResult::new(
        serde_json::to_value(metadata).map_err(ConsoleError::internal)?,
        log,
    ))
}

fn prid(
    world: &mut World,
    invocation: &ConsoleInvocation,
) -> Result<ConsoleCommandResult, ConsoleError> {
    require_arity(invocation, 1, 1)?;
    let entity = resolve_reference(world, &invocation.args[0])?;
    let references = world.resource::<RefRegistry>();
    let label = references.label(entity);
    let identity = references.identity(entity).cloned();
    world
        .resource_mut::<ConsoleSessionStore>()
        .select(invocation.session.clone(), entity);
    Ok(ConsoleCommandResult::new(
        json!({
            "label": label,
            "form_id": identity.as_ref().and_then(|value| value.form_id).map(|form_id| format!("{form_id:08x}")),
            "editor_id": identity.and_then(|value| value.editor_id),
        }),
        vec![format!("selected {label}")],
    ))
}

fn get_form_id(
    world: &mut World,
    invocation: &ConsoleInvocation,
) -> Result<ConsoleCommandResult, ConsoleError> {
    require_arity(invocation, 0, 0)?;
    let entity = target(invocation)?;
    let form_id = world
        .resource::<RefRegistry>()
        .identity(entity)
        .and_then(|identity| identity.form_id)
        .ok_or_else(|| ConsoleError::new("form_id_unavailable", "reference has no FormID"))?;
    Ok(ConsoleCommandResult::value(json!({
        "form_id": format!("{form_id:08x}"),
        "form_id_u32": form_id,
    })))
}

fn get_editor_id(
    world: &mut World,
    invocation: &ConsoleInvocation,
) -> Result<ConsoleCommandResult, ConsoleError> {
    require_arity(invocation, 0, 0)?;
    let entity = target(invocation)?;
    let editor_id = world
        .resource::<RefRegistry>()
        .identity(entity)
        .and_then(|identity| identity.editor_id.clone())
        .ok_or_else(|| ConsoleError::new("editor_id_unavailable", "reference has no EditorID"))?;
    Ok(ConsoleCommandResult::value(
        json!({ "editor_id": editor_id }),
    ))
}

fn get_position(
    world: &mut World,
    invocation: &ConsoleInvocation,
) -> Result<ConsoleCommandResult, ConsoleError> {
    require_arity(invocation, 0, 1)?;
    let entity = target(invocation)?;
    let position = world
        .get::<Transform>(entity)
        .map(|transform| transform.translation)
        .ok_or_else(|| ConsoleError::new("component_missing", "reference has no Transform"))?;
    match invocation.args.first() {
        Some(value) => {
            let index = axis(value)?;
            Ok(ConsoleCommandResult::value(json!(position[index])))
        }
        None => Ok(ConsoleCommandResult::value(json!({
            "x": position.x,
            "y": position.y,
            "z": position.z,
            "unit": "metres",
        }))),
    }
}

fn set_position(
    world: &mut World,
    invocation: &ConsoleInvocation,
) -> Result<ConsoleCommandResult, ConsoleError> {
    require_arity(invocation, 2, 2)?;
    let entity = target(invocation)?;
    let index = axis(&invocation.args[0])?;
    let value = finite_f32(&invocation.args[1], "position")?;
    let position = {
        let mut transform = world
            .get_mut::<Transform>(entity)
            .ok_or_else(|| ConsoleError::new("component_missing", "reference has no Transform"))?;
        transform.translation[index] = value;
        transform.translation
    };
    notify_transform_mutated(world, entity);
    Ok(ConsoleCommandResult::value(json!({
        "x": position.x,
        "y": position.y,
        "z": position.z,
        "unit": "metres",
    })))
}

fn entity_angles(world: &World, entity: Entity) -> Result<Vec3, ConsoleError> {
    if let Some(angles) = adapted_angles(world, entity) {
        return Ok(angles);
    }
    let rotation = world
        .get::<Transform>(entity)
        .map(|transform| transform.rotation)
        .ok_or_else(|| ConsoleError::new("component_missing", "reference has no Transform"))?;
    let (yaw, pitch, roll) = rotation.to_euler(EulerRot::YXZ);
    Ok(Vec3::new(
        clean_angle(pitch.to_degrees()),
        clean_angle(yaw.to_degrees()),
        clean_angle(roll.to_degrees()),
    ))
}

fn clean_angle(value: f32) -> f32 {
    if value.abs() < 0.000_001 { 0.0 } else { value }
}

fn get_angle(
    world: &mut World,
    invocation: &ConsoleInvocation,
) -> Result<ConsoleCommandResult, ConsoleError> {
    require_arity(invocation, 0, 1)?;
    let angles = entity_angles(world, target(invocation)?)?;
    match invocation.args.first() {
        Some(value) => Ok(ConsoleCommandResult::value(json!(angles[axis(value)?]))),
        None => Ok(ConsoleCommandResult::value(json!({
            "x": angles.x,
            "y": angles.y,
            "z": angles.z,
            "unit": "degrees",
        }))),
    }
}

fn set_angle(
    world: &mut World,
    invocation: &ConsoleInvocation,
) -> Result<ConsoleCommandResult, ConsoleError> {
    require_arity(invocation, 2, 2)?;
    let entity = target(invocation)?;
    let index = axis(&invocation.args[0])?;
    let value = finite_f32(&invocation.args[1], "angle")?;
    let mut angles = entity_angles(world, entity)?;
    angles[index] = value;
    if !set_adapted_angles(world, entity, angles) {
        let rotation = Quat::from_euler(
            EulerRot::YXZ,
            angles.y.to_radians(),
            angles.x.to_radians(),
            angles.z.to_radians(),
        );
        world
            .get_mut::<Transform>(entity)
            .ok_or_else(|| ConsoleError::new("component_missing", "reference has no Transform"))?
            .rotation = rotation;
    }
    notify_transform_mutated(world, entity);
    Ok(ConsoleCommandResult::value(json!({
        "x": angles.x,
        "y": angles.y,
        "z": angles.z,
        "unit": "degrees",
    })))
}

fn move_to(
    world: &mut World,
    invocation: &ConsoleInvocation,
) -> Result<ConsoleCommandResult, ConsoleError> {
    require_arity(invocation, 1, 1)?;
    let source = target(invocation)?;
    let destination = resolve_reference(world, &invocation.args[0])?;
    let destination_position = world
        .get::<Transform>(destination)
        .map(|transform| transform.translation)
        .ok_or_else(|| ConsoleError::new("component_missing", "destination has no Transform"))?;
    world
        .get_mut::<Transform>(source)
        .ok_or_else(|| ConsoleError::new("component_missing", "reference has no Transform"))?
        .translation = destination_position;
    notify_transform_mutated(world, source);
    Ok(ConsoleCommandResult::value(json!({
        "x": destination_position.x,
        "y": destination_position.y,
        "z": destination_position.z,
        "unit": "metres",
    })))
}

fn dump(
    world: &mut World,
    invocation: &ConsoleInvocation,
) -> Result<ConsoleCommandResult, ConsoleError> {
    require_arity(invocation, 0, 1)?;
    let entity = match invocation.args.first() {
        Some(selector) => resolve_reference(world, selector)?,
        None => target(invocation)?,
    };
    let identity = world.resource::<RefRegistry>().identity(entity).cloned();
    let name = world
        .get::<Name>(entity)
        .map(|name| name.as_str().to_string());
    let transform = world.get::<Transform>(entity).map(|transform| {
        let (yaw, pitch, roll) = transform.rotation.to_euler(EulerRot::YXZ);
        json!({
            "translation": [transform.translation.x, transform.translation.y, transform.translation.z],
            "rotation_degrees": [pitch.to_degrees(), yaw.to_degrees(), roll.to_degrees()],
            "scale": [transform.scale.x, transform.scale.y, transform.scale.z],
        })
    });
    let mut components = world
        .inspect_entity(entity)
        .into_iter()
        .flatten()
        .map(|component| component.name().to_string())
        .collect::<Vec<_>>();
    components.sort();
    let value = json!({
        "entity": entity.to_bits(),
        "name": name,
        "reference": identity.map(|identity| json!({
            "form_id": identity.form_id.map(|form_id| format!("{form_id:08x}")),
            "editor_id": identity.editor_id,
        })),
        "transform": transform,
        "components": components,
    });
    Ok(ConsoleCommandResult::new(
        value,
        vec![format!("dumped entity {}", entity.to_bits())],
    ))
}

#[allow(dead_code)]
fn _value_is_stable(value: &Value) -> bool {
    !value.is_null()
}
