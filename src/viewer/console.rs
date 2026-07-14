//! Viewer-dependent console commands and UI visibility markers.

use std::path::PathBuf;

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
    ConsoleRegistry,
};

use super::controls::{
    AmbientScale, AoStrength, FogStrength, IrradianceIntensity, LightingScale, LightsDisabled,
    UnlitMode,
};
#[cfg(test)]
use super::lighting::PreparedPointShadowRuntime;
use super::lighting::shadow_cache_status;
use super::{diagnostics, player};

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
        .add_systems(Update, sync_ui_visibility);
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

const RENDER_SETTINGS: [&str; 9] = [
    "lighting",
    "irradiance",
    "ambient",
    "bloom_intensity",
    "bloom_threshold",
    "bloom_softness",
    "fog",
    "ao",
    "shadow_samples",
];

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
            .insert_resource(player::StepDebugSettings::default());
        app.init_state::<GameplayModal>();
        let camera = player::CameraModeState {
            collision_build_complete: true,
            collisions_ready: true,
            ..default()
        };
        app.insert_resource(camera);
        app.world_mut().spawn((
            Camera3d::default(),
            Bloom::default(),
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

    #[test]
    fn developer_commands_and_aliases_are_registered_and_structured() {
        let mut app = test_app();
        assert!(exec(&mut app, "help toggleflycam").ok);
        assert!(exec(&mut app, "help togglecollisiongeometry").ok);
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
            9
        );
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
}
