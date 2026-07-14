use super::*;

pub(crate) fn run_view(
    manifest_path: PathBuf,
    disable_physics: bool,
    trace_seconds: Option<f32>,
    agent_port: Option<u16>,
) -> Result<()> {
    let manifest_path = fs::canonicalize(&manifest_path).context("manifest does not exist")?;
    let text = fs::read_to_string(&manifest_path)?;
    let manifest: PreparedSceneManifest = from_str(&text).context("invalid scene manifest")?;
    ensure_prepared_manifest_compatible(
        &manifest,
        NIF_CONVERTER_REVISION,
        PHYSICS_ASSET_SCHEMA_VERSION,
    )?;
    ensure_baked_scene_compatible(&manifest)?;
    let asset_root = PathBuf::from(&manifest.asset_root);
    let physics_assets = player::load_prepared_physics_assets(&manifest, &asset_root)?;
    let report_path = render_report_path(&manifest_path);
    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(default_primary_window()),
                ..default()
            })
            .set(AssetPlugin {
                file_path: asset_root.to_string_lossy().to_string(),
                ..default()
            }),
        FrameTimeDiagnosticsPlugin::new(RENDER_REPORT_HISTORY),
        RenderDiagnosticsPlugin,
        AutoExposurePlugin,
    ));
    app.add_plugins(crate::console::ConsolePlugin);
    app.insert_resource(physics_assets);
    if let Some(port) = agent_port {
        agent_bridge::install(&mut app, port);
    }
    app.add_plugins(AppStatePlugin);
    app.insert_resource(LoadingTarget::NewGame {
        manifest: manifest_path.clone(),
    });

    player::install(&mut app, disable_physics);
    audio::install(&mut app);
    interaction::install(&mut app);
    console::install(&mut app);
    console_ui::install(&mut app);
    app.insert_resource(manifest)
        .insert_resource(UnlitMode(false))
        .insert_resource(LightingScale(DEFAULT_LIGHTING_SCALE))
        .insert_resource(IrradianceIntensity(1.0))
        .insert_resource(AmbientScale(0.05))
        .insert_resource(FogStrength(DEFAULT_FOG_STRENGTH))
        .insert_resource(AoStrength(1.0))
        .insert_resource(AoMeshBases::default())
        .insert_resource(RenderReportPath(report_path))
        .insert_resource(RenderReportBuffer::default())
        .insert_resource(LightsDisabled(false))
        .insert_resource(PreparedPointShadowRuntime::default())
        .insert_resource(PointLightShadowSamples::default())
        // F35.6: the CLI's view/render flow auto-advances Boot -> Loading ->
        // InGame with no menu stop; MainMenu remains reachable in the state
        // graph but the CLI never observes it (LoadingTarget is always set).
        .add_systems(Update, (auto_advance_from_boot, auto_advance_from_loading))
        .add_systems(
            OnEnter(AppState::InGame),
            (
                capture_cursor,
                spawn_prepared_scene,
                player::build_prepared_colliders,
                spawn_reticle,
            )
                .chain(),
        )
        .add_systems(Update, apply_lighting_scale)
        .add_systems(
            Update,
            (
                apply_fog_strength,
                apply_ao_strength,
                apply_irradiance_intensity,
                apply_horizontal_fov,
                update_fps_text,
                apply_unlit_mode,
                configure_glow_cards,
            ),
        )
        .add_systems(Update, record_render_sample)
        .add_systems(
            Update,
            (
                capture_cursor_input,
                free_fly_camera,
                player::fps_mouse_look,
            )
                .chain()
                .run_if(in_state(AppState::InGame)),
        );
    if let Some(seconds) = trace_seconds {
        if !seconds.is_finite() || seconds <= 0.0 {
            anyhow::bail!("--trace-seconds must be finite and greater than zero");
        }
        app.insert_resource(TraceCaptureLimit { remaining: seconds })
            .add_systems(Update, stop_trace_capture);
    }
    app.run();
    Ok(())
}

fn default_primary_window() -> Window {
    Window {
        resolution: (DEFAULT_WINDOW_WIDTH, DEFAULT_WINDOW_HEIGHT).into(),
        focused: true,
        present_mode: PresentMode::AutoNoVsync,
        ..default()
    }
}

#[derive(Resource)]
struct TraceCaptureLimit {
    remaining: f32,
}

fn stop_trace_capture(
    time: Res<Time>,
    mut limit: ResMut<TraceCaptureLimit>,
    mut app_exit: MessageWriter<AppExit>,
) {
    limit.remaining -= time.delta_secs();
    if limit.remaining <= 0.0 {
        app_exit.write(AppExit::Success);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn primary_window_defaults_to_1080p() {
        let window = default_primary_window();
        assert_eq!(window.resolution.width(), 1920.0);
        assert_eq!(window.resolution.height(), 1080.0);
    }
}
