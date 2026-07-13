//! Viewer-dependent console commands and UI visibility markers.

use std::path::PathBuf;

use bevy::prelude::*;
use bevy::render::view::screenshot::{Screenshot, save_to_disk};
use bevy::window::PrimaryWindow;
use serde_json::json;

use crate::console::{
    ConsoleCommand, ConsoleCommandResult, ConsoleEntityHooks, ConsoleError, ConsoleInvocation,
    ConsoleRegistry,
};

use super::player;

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
        .init_resource::<DiagnosticUiState>();
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
    ] {
        registry
            .register(command)
            .expect("viewer console command is unique");
    }
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

fn toggle_collision(
    world: &mut World,
    invocation: &ConsoleInvocation,
) -> Result<ConsoleCommandResult, ConsoleError> {
    no_args(invocation)?;
    let enabled = {
        let mut no_clip = world.resource_mut::<player::PlayerNoClip>();
        no_clip.0 = !no_clip.0;
        no_clip.0
    };
    Ok(ConsoleCommandResult::new(
        json!({ "no_clip": enabled }),
        vec![format!(
            "collision {}",
            if enabled { "disabled" } else { "enabled" }
        )],
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
    let mut query = world.query_filtered::<&mut Visibility, With<GameUi>>();
    for mut visibility in query.iter_mut(world) {
        *visibility = if visible {
            Visibility::Inherited
        } else {
            Visibility::Hidden
        };
    }
    Ok(ConsoleCommandResult::value(json!({ "visible": visible })))
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
    let mut query = world.query_filtered::<&mut Visibility, With<DiagnosticUi>>();
    for mut visibility in query.iter_mut(world) {
        *visibility = if visible {
            Visibility::Inherited
        } else {
            Visibility::Hidden
        };
    }
    Ok(ConsoleCommandResult::value(json!({ "visible": visible })))
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
    Ok(ConsoleCommandResult::value(
        json!({ "relative_speed": multiplier }),
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
        vec![format!("screenshot queued at {}", path.display())],
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::console::{ConsoleExecutor, ConsolePlugin, ConsoleRequest, ConsoleSessionId};

    fn test_app() -> App {
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, ConsolePlugin))
            .insert_resource(player::PlayerNoClip::default());
        install(&mut app);
        app.update();
        app
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
}
