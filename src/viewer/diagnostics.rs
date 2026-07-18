//! Render diagnostics and CSV reports.

use super::controls::{LightsDisabled, UnlitMode};
use super::performance_policy::{FrameProbeSummary, FrameSample, summarize_frame_window};
use super::*;

#[derive(Resource)]
pub(crate) struct RenderReportPath(pub(crate) PathBuf);

#[derive(Resource, Default)]
pub(crate) struct RenderReportBuffer {
    next_sample: u64,
    samples: VecDeque<FrameSample>,
}

pub(crate) fn render_diagnostics_report_path(report_path: &Path) -> PathBuf {
    report_path.with_file_name("render_diagnostics.csv")
}

pub(crate) fn render_report_path(manifest_path: &Path) -> PathBuf {
    manifest_path
        .ancestors()
        .find(|path| {
            path.file_name()
                .and_then(|name| name.to_str())
                .is_some_and(|name| name.eq_ignore_ascii_case(".bevyout"))
        })
        .and_then(Path::parent)
        .map(|path| path.join("render_timings.csv"))
        .unwrap_or_else(|| {
            std::env::current_dir()
                .unwrap_or_else(|_| PathBuf::from("."))
                .join("render_timings.csv")
        })
}

pub(crate) fn record_render_sample(
    diagnostics: Res<DiagnosticsStore>,
    mut report: ResMut<RenderReportBuffer>,
) {
    let Some(frame_time_ms) = diagnostics
        .get(&FrameTimeDiagnosticsPlugin::FRAME_TIME)
        .and_then(|diagnostic| diagnostic.value())
    else {
        return;
    };
    if !frame_time_ms.is_finite() || frame_time_ms < 0.0 {
        return;
    }
    let sample = FrameSample {
        sample: report.next_sample,
        frame_time_ms,
    };
    report.next_sample = report.next_sample.saturating_add(1);
    report.samples.push_back(sample);
    while report.samples.len() > RENDER_REPORT_HISTORY {
        report.samples.pop_front();
    }
}

pub(crate) fn latest_render_sample(report: &RenderReportBuffer) -> Option<u64> {
    report.samples.back().map(|sample| sample.sample)
}

pub(crate) fn summarize_render_samples(
    report: &RenderReportBuffer,
    after_sample: Option<u64>,
    latest_limit: usize,
    budget_ms: f64,
) -> FrameProbeSummary {
    let samples = report.samples.iter().copied().collect::<Vec<_>>();
    summarize_frame_window(&samples, after_sample, latest_limit, budget_ms)
}

pub(crate) fn save_render_report_now(world: &mut World) -> std::io::Result<PathBuf> {
    let report_path = world.resource::<RenderReportPath>().0.clone();
    let samples = world
        .resource::<RenderReportBuffer>()
        .samples
        .iter()
        .copied()
        .collect::<Vec<_>>();
    let (mesh_assets, material_assets, image_assets) = (
        world.resource::<Assets<Mesh>>().len(),
        world.resource::<Assets<StandardMaterial>>().len(),
        world.resource::<Assets<Image>>().len(),
    );
    let (manifest_placements, manifest_lights) = {
        let manifest = world.resource::<crate::viewer::LoadedSceneManifest>();
        (manifest.placements.len(), manifest.lights.len())
    };
    let camera_mode = format!("{:?}", world.resource::<player::CameraModeState>().mode);
    let physics_disabled = world.resource::<player::PhysicsDisabled>().0;
    let unlit_mode = world.resource::<UnlitMode>().0;
    let lights_disabled = world.resource::<LightsDisabled>().0;
    let physics = world.resource::<player::CollisionRuntimeStats>().clone();

    let entity_count = {
        let mut query = world.query::<Entity>();
        query.iter(world).count()
    };
    let (mesh_entity_count, hidden_mesh_count) = {
        let mut query = world.query_filtered::<Option<&Visibility>, With<Mesh3d>>();
        let visibilities = query.iter(world).collect::<Vec<_>>();
        (
            visibilities.len(),
            visibilities
                .iter()
                .filter(|visibility| matches!(visibility, Some(Visibility::Hidden)))
                .count(),
        )
    };
    let named_mesh_count = {
        let mut query = world.query::<&GltfMeshName>();
        query.iter(world).count()
    };
    let blooms = {
        let mut query = world.query_filtered::<&Bloom, With<Camera3d>>();
        query
            .iter(world)
            .map(|bloom| {
                (
                    bloom.intensity,
                    bloom.prefilter.threshold,
                    bloom.prefilter.threshold_softness,
                )
            })
            .collect::<Vec<_>>()
    };
    let camera_count = blooms.len();
    let (bloom_intensity, bloom_threshold, bloom_softness) =
        blooms.first().copied().unwrap_or_default();
    let point_light_count = {
        let mut query = world.query_filtered::<Entity, With<PointLight>>();
        query.iter(world).count()
    };
    let directional_light_count = {
        let mut query = world.query_filtered::<Entity, With<DirectionalLight>>();
        query.iter(world).count()
    };
    let irradiance_volume_count = {
        let mut query = world.query_filtered::<Entity, With<IrradianceVolume>>();
        query.iter(world).count()
    };
    let collider_count = {
        let mut query = world.query_filtered::<Entity, With<player::PhysicsCollider>>();
        query.iter(world).count()
    };
    let mut shape_kinds = physics.shape_kinds.iter().collect::<Vec<_>>();
    shape_kinds.sort_unstable_by_key(|(kind, _)| **kind);
    let shape_kinds = shape_kinds
        .into_iter()
        .map(|(kind, count)| format!("{kind}:{count}"))
        .collect::<Vec<_>>()
        .join(";");

    let mut csv = String::from(
        "sample,frame_time_ms,fps,entity_count,mesh_entities,hidden_meshes,named_gltf_meshes,point_lights,directional_lights,irradiance_volumes,cameras,mesh_assets,material_assets,image_assets,manifest_placements,manifest_lights,bloom_intensity,bloom_threshold,bloom_softness,camera_mode,unlit_mode,lights_disabled,physics_disabled,collider_entities,physics_authored_assets,physics_fallback_assets,physics_bodies,physics_shapes,physics_shape_kinds,physics_packed_triangles,physics_filtered_shapes,physics_dynamic_bodies,physics_awake_dynamic_bodies,physics_sleeping_dynamic_bodies,physics_dynamic_transform_updates,physics_cooking_ms,physics_sidecar_bytes\n",
    );
    for sample in &samples {
        let fps = if sample.frame_time_ms > f64::EPSILON {
            1000.0 / sample.frame_time_ms
        } else {
            0.0
        };
        csv.push_str(&format!(
            "{},{:.4},{:.4},{},{},{},{},{},{},{},{},{},{},{},{},{},{:.4},{:.4},{:.4},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{:.3},{}\n",
            sample.sample,
            sample.frame_time_ms,
            fps,
            entity_count,
            mesh_entity_count,
            hidden_mesh_count,
            named_mesh_count,
            point_light_count,
            directional_light_count,
            irradiance_volume_count,
            camera_count,
            mesh_assets,
            material_assets,
            image_assets,
            manifest_placements,
            manifest_lights,
            bloom_intensity,
            bloom_threshold,
            bloom_softness,
            camera_mode,
            u8::from(unlit_mode),
            u8::from(lights_disabled),
            u8::from(physics_disabled),
            collider_count,
            physics.authored_assets,
            physics.fallback_assets,
            physics.bodies,
            physics.shapes,
            csv_field(&shape_kinds),
            physics.packed_triangles,
            physics.filtered_shapes,
            physics.dynamic_bodies,
            physics.awake_dynamic_bodies,
            physics.sleeping_dynamic_bodies,
            physics.dynamic_transform_updates,
            physics.cooking_millis,
            physics.sidecar_bytes,
        ));
    }
    fs::write(&report_path, csv)?;
    let mut render_diagnostics = world
        .resource::<DiagnosticsStore>()
        .iter()
        .filter(|diagnostic| diagnostic.path().as_str().starts_with("render/"))
        .filter_map(|diagnostic| {
            let value = diagnostic.value()?;
            if !value.is_finite() {
                return None;
            }
            Some((
                diagnostic.path().as_str().to_owned(),
                diagnostic.suffix.as_ref().to_owned(),
                value,
                diagnostic.average().unwrap_or(value),
                diagnostic.history_len(),
            ))
        })
        .collect::<Vec<_>>();
    render_diagnostics.sort_unstable_by(|left, right| left.0.cmp(&right.0));

    let diagnostics_path = render_diagnostics_report_path(&report_path);
    let mut diagnostics_csv = String::from("diagnostic_path,suffix,value,average,history_len\n");
    for (path, suffix, value, average, history_len) in render_diagnostics {
        diagnostics_csv.push_str(&format!(
            "{},{},{value:.6},{average:.6},{history_len}\n",
            csv_field(&path),
            csv_field(&suffix),
        ));
    }
    fs::write(&diagnostics_path, diagnostics_csv)?;
    info!(
        "wrote render diagnostics report to {}",
        diagnostics_path.display()
    );
    info!(
        "wrote render report with {} samples to {}",
        samples.len(),
        report_path.display()
    );
    Ok(report_path)
}

pub(crate) fn csv_field(value: &str) -> String {
    if value
        .chars()
        .any(|character| matches!(character, ',' | '"' | '\n' | '\r'))
    {
        format!("\"{}\"", value.replace('"', "\"\""))
    } else {
        value.to_owned()
    }
}

// -- Issue #151: console-toggleable debug info HUD --------------------------
//
// Follows the same pattern as `player::mod`'s `ColliderDebugHud`/
// `StepDebugHud`: a marker component spawned once under `console::
// DiagnosticUi` (so `tdt` still folds it into the rest of the diagnostic
// HUD), a plain toggle `Resource` the console command flips, and an
// `Update` system that rewrites the `Text` every frame. Unlike those two
// booleans, this block reports live state (position/cell/agents) so its
// update system is exclusive (`&mut World`) to reach `player::FpsPlayer`'s
// `Transform` and `nav::agent::hud_agent_status_lines` without widening
// either module's public surface just for the HUD.

/// Toggled by the `tdi` console command (`viewer::console`). A plain global
/// resource, so it survives cell swaps untouched -- `world::swap` never
/// resets viewer console resources -- and the HUD simply keeps reporting
/// whatever is true of the new active cell once a swap completes.
#[derive(Resource, Default)]
pub(crate) struct DebugInfoState {
    pub(crate) enabled: bool,
}

#[derive(Component)]
pub(crate) struct DebugInfoHud;

const DEBUG_INFO_OFF_LINE: &str = "Debug info: Off";

// Anchor corner note (post-merge fix, real-data smoke test): the existing
// HUD occupies top-right (`FpsText`) and bottom-right (`ColliderDebugHud`
// at `bottom: 10`, `StepDebugHud` at `bottom: 34`). This block's line count
// varies at runtime (grows with live test nav agents), so anchoring it to
// any of those same edges risks it growing into a row already in use --
// exactly what happened bottom-left/bottom-right sharing the bottom row.
// Top-left is the only corner nothing else uses, so pinning there rules out
// the collision by construction regardless of how many lines this block
// ever renders, instead of relying on padding-based spacing that a future
// line addition could silently outgrow.
pub(crate) fn spawn_debug_info_hud(mut commands: Commands) {
    commands.spawn((
        Text::new(DEBUG_INFO_OFF_LINE),
        DebugInfoHud,
        console::DiagnosticUi,
        TextColor(Color::srgb(0.7, 0.9, 1.0)),
        Node {
            position_type: PositionType::Absolute,
            left: px(10),
            top: px(8),
            ..default()
        },
        ZIndex(120),
    ));
}

/// Pure formatting (issue #151's "deterministic text formatting" requirement,
/// so tests can assert stable lines without spinning up a full `App`): one
/// line for the toggle state, one for player position, one for the active
/// cell's identity, then one per live test nav agent (already formatted by
/// `nav::agent::hud_agent_status_lines`).
pub(crate) fn format_debug_info_lines(
    player_position: Option<Vec3>,
    active_cell: Option<(u32, Option<&str>, Option<&str>)>,
    nav_agent_lines: &[String],
) -> Vec<String> {
    let mut lines = vec!["Debug info: On".to_string()];
    lines.push(match player_position {
        Some(position) => format!(
            "player pos=({:.2},{:.2},{:.2})",
            position.x, position.y, position.z
        ),
        None => "player pos=unavailable".to_string(),
    });
    lines.push(match active_cell {
        Some((form_id, editor_id, name)) => format!(
            "cell={form_id:08x} editor_id={} name={}",
            editor_id.unwrap_or("none"),
            name.unwrap_or("none"),
        ),
        None => "cell=unavailable".to_string(),
    });
    lines.extend(nav_agent_lines.iter().cloned());
    lines
}

pub(crate) fn update_debug_info_hud(world: &mut World) {
    let enabled = world
        .get_resource::<DebugInfoState>()
        .is_some_and(|state| state.enabled);
    let text = if enabled {
        let player_position = {
            let mut query = world.query_filtered::<&Transform, With<player::FpsPlayer>>();
            query
                .iter(world)
                .next()
                .map(|transform| transform.translation)
        };
        let active_cell = world
            .get_resource::<PreparedSceneManifest>()
            .map(|manifest| {
                (
                    manifest.cell.form_id,
                    manifest.cell.editor_id.clone(),
                    manifest.cell.name.clone(),
                )
            });
        let active_cell_refs = active_cell
            .as_ref()
            .map(|(form_id, editor_id, name)| (*form_id, editor_id.as_deref(), name.as_deref()));
        let nav_agent_lines = nav::agent::hud_agent_status_lines(world);
        format_debug_info_lines(player_position, active_cell_refs, &nav_agent_lines).join("\n")
    } else {
        DEBUG_INFO_OFF_LINE.to_string()
    };
    let mut query = world.query_filtered::<&mut Text, With<DebugInfoHud>>();
    if let Some(mut hud_text) = query.iter_mut(world).next() {
        hud_text.0 = text;
    }
}

#[cfg(test)]
mod debug_info_tests {
    use bevy::ecs::system::RunSystemOnce;

    use super::*;

    #[test]
    fn off_line_is_stable_and_alone() {
        assert_eq!(DEBUG_INFO_OFF_LINE, "Debug info: Off");
    }

    // Real-data smoke-test fix: with the block anchored bottom-left, a
    // multi-line render (a live test nav agent widens it to 4+ lines) grew
    // upward into the same bottom row `ColliderDebugHud`/`StepDebugHud`
    // occupy bottom-right, and the two texts garbled each other on screen.
    // Top-left is the only screen corner nothing else in the HUD uses
    // (`FpsText` is top-right, the collider/step HUDs are bottom-right), so
    // pinning there rules out that collision by construction -- asserting
    // `bottom`/`right` stay `Val::Auto` here is exactly the guarantee that
    // no line count can ever grow this block into the existing bottom row.
    #[test]
    fn hud_is_anchored_top_left_never_the_existing_bottom_right_row() {
        let mut world = World::new();
        world.run_system_once(spawn_debug_info_hud).unwrap();
        let mut query = world.query_filtered::<&Node, With<DebugInfoHud>>();
        let node = query.single(&world).unwrap();
        assert_eq!(node.position_type, PositionType::Absolute);
        assert_eq!(node.top, Val::Px(8.0));
        assert_eq!(node.left, Val::Px(10.0));
        assert_eq!(
            node.bottom,
            Val::Auto,
            "must not share the existing bottom-right HUD row"
        );
        assert_eq!(
            node.right,
            Val::Auto,
            "must not share the existing bottom-right HUD row"
        );
    }

    #[test]
    fn on_reports_unavailable_player_and_cell_when_absent() {
        let lines = format_debug_info_lines(None, None, &[]);
        assert_eq!(
            lines,
            vec![
                "Debug info: On".to_string(),
                "player pos=unavailable".to_string(),
                "cell=unavailable".to_string(),
            ]
        );
    }

    #[test]
    fn on_reports_player_position_and_cell_identity() {
        let lines = format_debug_info_lines(
            Some(Vec3::new(1.0, 2.5, -3.25)),
            Some((0x0002_8579, Some("VaultAtrium"), Some("Vault 101 Atrium"))),
            &[],
        );
        assert_eq!(
            lines,
            vec![
                "Debug info: On".to_string(),
                "player pos=(1.00,2.50,-3.25)".to_string(),
                "cell=00028579 editor_id=VaultAtrium name=Vault 101 Atrium".to_string(),
            ]
        );
    }

    #[test]
    fn on_reports_cell_with_no_editor_id_or_name_as_none() {
        let lines = format_debug_info_lines(None, Some((0x10, None, None)), &[]);
        assert_eq!(lines[2], "cell=00000010 editor_id=none name=none");
    }

    #[test]
    fn nav_agent_lines_are_appended_verbatim_after_cell() {
        let lines = format_debug_info_lines(
            None,
            None,
            &["nav agent 0 status=Idle position=(0.00,0.00,0.00) grounded=true stuck=false blocked=false".to_string()],
        );
        assert_eq!(lines.len(), 4);
        assert_eq!(
            lines[3],
            "nav agent 0 status=Idle position=(0.00,0.00,0.00) grounded=true stuck=false blocked=false"
        );
    }
}
