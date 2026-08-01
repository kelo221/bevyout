//! Render diagnostics and CSV reports.

use super::controls::{LightsDisabled, UnlitMode};
use super::performance_policy::{FrameProbeSummary, FrameSample, summarize_frame_window};
use super::*;
use std::time::Duration;

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
    let presentation = super::world::exterior::exterior_presentation_json(world);
    let terrain_near = presentation["terrain"]["near"].as_u64().unwrap_or(0);
    let terrain_middle = presentation["terrain"]["middle"].as_u64().unwrap_or(0);
    let terrain_distant = presentation["terrain"]["distant"].as_u64().unwrap_or(0);
    let terrain_lod_transitions = presentation["terrain"]["lod_transitions"]
        .as_u64()
        .unwrap_or(0);
    let distance_culled = presentation["objects"]["distance_culled"]
        .as_u64()
        .unwrap_or(0);
    let occlusion_enabled = presentation["culling"]["occlusion"]["enabled"]
        .as_bool()
        .unwrap_or(false);

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
    let reflection_probe_count = {
        let mut query =
            world.query_filtered::<Entity, With<super::scene::PreparedReflectionProbe>>();
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
        "sample,frame_time_ms,fps,entity_count,mesh_entities,hidden_meshes,named_gltf_meshes,point_lights,directional_lights,irradiance_volumes,reflection_probes,cameras,mesh_assets,material_assets,image_assets,manifest_placements,manifest_lights,bloom_intensity,bloom_threshold,bloom_softness,camera_mode,unlit_mode,lights_disabled,physics_disabled,collider_entities,physics_authored_assets,physics_fallback_assets,physics_bodies,physics_shapes,physics_shape_kinds,physics_packed_triangles,physics_filtered_shapes,physics_dynamic_bodies,physics_awake_dynamic_bodies,physics_sleeping_dynamic_bodies,physics_dynamic_transform_updates,physics_cooking_ms,physics_sidecar_bytes,terrain_near,terrain_middle,terrain_distant,terrain_lod_transitions,distance_culled,occlusion_enabled\n",
    );
    for sample in &samples {
        let fps = if sample.frame_time_ms > f64::EPSILON {
            1000.0 / sample.frame_time_ms
        } else {
            0.0
        };
        csv.push_str(&format!(
            "{},{:.4},{:.4},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{:.4},{:.4},{:.4},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{:.3},{},{},{},{},{},{},{}\n",
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
            reflection_probe_count,
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
            terrain_near,
            terrain_middle,
            terrain_distant,
            terrain_lod_transitions,
            distance_culled,
            u8::from(occlusion_enabled),
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
// `Update` system that owns its `Text`. Unlike those two booleans, this
// block reports live state (position/cell/agents); issue #268 therefore
// made it change-driven and non-exclusive: the off line is written only on
// toggle transitions, enabled content refreshes on a bounded timer
// (`DEBUG_INFO_REFRESH_INTERVAL`), and the composed string is only ever
// assigned when it actually changed. `player::FpsPlayer`'s `Transform` is
// an ordinary `Query` read and the agent lines come from
// `nav::agent::HudAgentProjection`, a read-only projection introduced so no
// module's public surface has to widen for the HUD.

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
const DEBUG_INFO_HUD_TOP_PX: f32 = 56.0;

/// Issue #268: enabled-state refresh cadence for the live player/cell/agent
/// lines. 125 ms = 8 Hz -- inside the required 5-10 Hz band, coarse enough
/// that HUD relayout noise vanishes from steady-state frame times.
const DEBUG_INFO_REFRESH_INTERVAL: Duration = Duration::from_millis(125);

// Anchor corner note (post-merge fix, real-data smoke test): the existing
// HUD occupies top-right (`FpsText`) and bottom-right (`ColliderDebugHud`
// at `bottom: 10`, `StepDebugHud` at `bottom: 34`). This block's line count
// varies at runtime (grows with live test nav agents), so anchoring it to
// any of those same edges risks it growing into a row already in use --
// exactly what happened bottom-left/bottom-right sharing the bottom row.
// Top-left is the diagnostic corner; the player transform HUD owns the first
// row and this block starts below it. Keeping this block independently
// anchored prevents its variable line count from colliding with the other
// fixed-corner diagnostics.
pub(crate) fn spawn_debug_info_hud(mut commands: Commands) {
    commands.spawn((
        Text::new(DEBUG_INFO_OFF_LINE),
        DebugInfoHud,
        console::DiagnosticUi,
        TextColor(Color::srgb(0.7, 0.9, 1.0)),
        Node {
            position_type: PositionType::Absolute,
            left: px(10),
            top: px(DEBUG_INFO_HUD_TOP_PX),
            ..default()
        },
        ZIndex(120),
    ));
}

/// Pure formatting (issue #151's "deterministic text formatting" requirement,
/// so tests can assert stable lines without spinning up a full `App`): one
/// line for the toggle state, one for player position, one for the active
/// cell's identity, then one per live test nav agent (already formatted by
/// `nav::agent::HudAgentProjection`).
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

/// Issue #268: ordinary (non-exclusive) change-driven update system.
///
/// Gates, in order: nothing runs past the two resource reads while the HUD
/// is steadily off; the off line is written only when `DebugInfoState` was
/// just toggled; while enabled, composition happens on toggle (immediate)
/// or when the 8 Hz refresh timer finishes. The final compare-guard means
/// NO frame assigns `Text` with the string it already holds, which is what
/// keeps Bevy's text relayout out of steady-state frames.
pub(crate) fn update_debug_info_hud(
    state: Res<DebugInfoState>,
    time: Res<Time>,
    mut refresh: Local<Option<Timer>>,
    players: Query<&Transform, With<player::FpsPlayer>>,
    manifest: Option<Res<crate::viewer::LoadedSceneManifest>>,
    agents: nav::agent::HudAgentProjection,
    mut hud_text: Query<&mut Text, With<DebugInfoHud>>,
) {
    let toggled = state.is_changed();
    let text = if state.enabled {
        let timer = refresh
            .get_or_insert_with(|| Timer::new(DEBUG_INFO_REFRESH_INTERVAL, TimerMode::Repeating));
        let due = timer.tick(time.delta()).just_finished();
        if toggled {
            // Toggle writes immediately; restart the cadence from here.
            timer.reset();
        } else if !due {
            return;
        }
        let player_position = players.iter().next().map(|transform| transform.translation);
        let active_cell = manifest.as_deref().map(|manifest| {
            (
                manifest.0.cell.form_id,
                manifest.0.cell.editor_id.as_deref(),
                manifest.0.cell.name.as_deref(),
            )
        });
        format_debug_info_lines(player_position, active_cell, &agents.status_lines()).join("\n")
    } else {
        if !toggled {
            return;
        }
        DEBUG_INFO_OFF_LINE.to_string()
    };
    if let Some(mut hud_text) = hud_text.iter_mut().next()
        && hud_text.0 != text
    {
        hud_text.0 = text;
    }
}

#[cfg(test)]
#[path = "tests/diagnostics.rs"]
mod tests;
