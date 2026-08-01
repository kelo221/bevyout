//! Console controls for prepared IMAD screen feedback.

use super::*;
use crate::viewer::screen_fx::policy::{
    ScreenFxClearReason, ScreenFxRequest, ScreenFxSettings, ScreenFxSource, ScreenFxStart,
};
use crate::viewer::screen_fx::{
    ScreenFxCatalog, ScreenFxRequested, ScreenFxRuntime, definition_from_modifier, status_json,
};

pub(super) struct ScreenFxCommandProvider;

impl ConsoleCommandProvider for ScreenFxCommandProvider {
    fn register_commands(&self, registry: &mut ConsoleRegistry) -> Result<(), ConsoleError> {
        registry.register(
            ConsoleCommand::new(
                "screenfx",
                "screenfx status | start <form-id> [priority] | stop <form-id> | clear [reason] | settings <overall> <blood> <flashes> <motion-distortion>",
                "Inspect, start, stop, or clear prepared IMAD screen feedback.",
                screen_fx,
            )
            .mutating(),
        )
    }
}

fn screen_fx(
    world: &mut World,
    invocation: &ConsoleInvocation,
) -> Result<ConsoleCommandResult, ConsoleError> {
    match invocation.args.as_slice() {
        [] => screen_fx_status(world),
        [command] if command.eq_ignore_ascii_case("status") => screen_fx_status(world),
        [command] if command.eq_ignore_ascii_case("clear") => {
            clear_screen_fx(world, ScreenFxClearReason::Teardown)
        }
        [command, reason] if command.eq_ignore_ascii_case("clear") => {
            let reason = match reason.to_ascii_lowercase().as_str() {
                "death" => ScreenFxClearReason::Death,
                "save_load" | "saveload" => ScreenFxClearReason::SaveLoad,
                "camera_mode" | "cameramode" => ScreenFxClearReason::CameraMode,
                "cell_transition" | "cell" => ScreenFxClearReason::CellTransition,
                "teardown" => ScreenFxClearReason::Teardown,
                _ => {
                    return Err(ConsoleError::new(
                        "invalid_clear_reason",
                        "screenfx clear reason must be death, save_load, camera_mode, cell_transition, or teardown",
                    ));
                }
            };
            clear_screen_fx(world, reason)
        }
        [command, raw_form_id] if command.eq_ignore_ascii_case("stop") => {
            let form_id = parse_item_form_id(raw_form_id).ok_or_else(|| {
                ConsoleError::new(
                    "invalid_form_id",
                    "screenfx stop requires a hexadecimal FormID",
                )
            })?;
            write_request(
                world,
                ScreenFxRequest::Stop {
                    modifier_form_id: form_id,
                },
            )?;
            Ok(ConsoleCommandResult::new(
                json!({
                    "schema": "bevyout.m5.screen_fx",
                    "schema_version": 1,
                    "queued": true,
                    "action": "stop",
                    "form_id": format!("{form_id:08x}"),
                }),
                vec![format!("Screen modifier {form_id:08x} stop queued.")],
            ))
        }
        [command, raw_form_id] | [command, raw_form_id, _]
            if command.eq_ignore_ascii_case("start") =>
        {
            let form_id = parse_item_form_id(raw_form_id).ok_or_else(|| {
                ConsoleError::new(
                    "invalid_form_id",
                    "screenfx start requires a hexadecimal FormID",
                )
            })?;
            let priority = match invocation.args.as_slice() {
                [_, _, raw_priority] => raw_priority.parse::<i32>().map_err(|_| {
                    ConsoleError::new("invalid_priority", "screenfx priority must be an integer")
                })?,
                _ => 0,
            };
            let modifier = world
                .resource::<ScreenFxCatalog>()
                .modifiers
                .get(&form_id)
                .cloned()
                .ok_or_else(|| {
                    ConsoleError::new(
                        "unknown_modifier",
                        format!(
                            "prepared IMAD modifier {form_id:08x} is not in the active catalog"
                        ),
                    )
                })?;
            let definition = definition_from_modifier(&modifier);
            write_request(
                world,
                ScreenFxRequest::Start(ScreenFxStart {
                    source: ScreenFxSource::Developer,
                    modifier_form_id: form_id,
                    priority,
                    start_ms: u64::MAX,
                    intensity: 1.0,
                    definition,
                }),
            )?;
            Ok(ConsoleCommandResult::new(
                json!({
                    "schema": "bevyout.m5.screen_fx",
                    "schema_version": 1,
                    "queued": true,
                    "action": "start",
                    "form_id": format!("{form_id:08x}"),
                    "priority": priority,
                    "duration_ms": modifier.duration_ms,
                    "editor_id": modifier.editor_id,
                }),
                vec![format!("Screen modifier {form_id:08x} start queued.")],
            ))
        }
        [command, overall, blood, flashes, distortion]
            if command.eq_ignore_ascii_case("settings") =>
        {
            let settings = ScreenFxSettings {
                overall_intensity: parse_setting(overall, "overall")?,
                screen_blood: parse_setting(blood, "screen_blood")?,
                flashes: parse_setting(flashes, "flashes")?,
                motion_and_distortion: parse_setting(distortion, "motion_distortion")?,
            };
            world
                .resource_mut::<ScreenFxRuntime>()
                .policy
                .set_settings(settings);
            Ok(ConsoleCommandResult::new(
                json!({
                    "schema": "bevyout.m5.screen_fx",
                    "schema_version": 1,
                    "action": "settings",
                    "overall_intensity": settings.overall_intensity,
                    "screen_blood": settings.screen_blood,
                    "flashes": settings.flashes,
                    "motion_and_distortion": settings.motion_and_distortion,
                }),
                vec!["Screen feedback settings updated.".into()],
            ))
        }
        _ => Err(ConsoleError::new(
            "bad_arguments",
            "screenfx accepts status, start <form-id> [priority], stop <form-id>, clear [reason], or settings <overall> <blood> <flashes> <motion-distortion>",
        )),
    }
}

fn clear_screen_fx(
    world: &mut World,
    reason: ScreenFxClearReason,
) -> Result<ConsoleCommandResult, ConsoleError> {
    write_request(world, ScreenFxRequest::Clear { reason })?;
    Ok(ConsoleCommandResult::new(
        json!({
            "schema": "bevyout.m5.screen_fx",
            "schema_version": 1,
            "queued": true,
            "action": "clear",
            "reason": format!("{reason:?}"),
        }),
        vec![format!("Screen feedback clear ({reason:?}) queued.")],
    ))
}

fn parse_setting(value: &str, name: &str) -> Result<f32, ConsoleError> {
    let value = value.parse::<f32>().map_err(|_| {
        ConsoleError::new(
            "invalid_setting",
            format!("{name} must be a finite number in 0..1"),
        )
    })?;
    if value.is_finite() && (0.0..=1.0).contains(&value) {
        Ok(value)
    } else {
        Err(ConsoleError::new(
            "invalid_setting",
            format!("{name} must be a finite number in 0..1"),
        ))
    }
}

fn screen_fx_status(world: &World) -> Result<ConsoleCommandResult, ConsoleError> {
    let runtime = world.get_resource::<ScreenFxRuntime>().ok_or_else(|| {
        ConsoleError::new("screen_fx_unavailable", "screen feedback runtime is absent")
    })?;
    let catalog = world.get_resource::<ScreenFxCatalog>().ok_or_else(|| {
        ConsoleError::new("screen_fx_unavailable", "screen feedback catalog is absent")
    })?;
    Ok(ConsoleCommandResult::value(status_json(runtime, catalog)))
}

fn write_request(world: &mut World, request: ScreenFxRequest) -> Result<(), ConsoleError> {
    let Some(mut messages) = world.get_resource_mut::<Messages<ScreenFxRequested>>() else {
        return Err(ConsoleError::new(
            "screen_fx_unavailable",
            "screen feedback message channel is absent",
        ));
    };
    messages.write(ScreenFxRequested(request));
    Ok(())
}
