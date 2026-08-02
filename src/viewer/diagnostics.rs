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

const CONVERGENCE_REPORT_SCHEMA: &str = "m6-convergence-v1";
const CONVERGENCE_FRAME_BUDGET_MS: f64 = 16.6667;

#[derive(Clone, Copy)]
enum ReportStatus {
    Measured,
    NotYetSampled,
    Unsupported,
    NotRun,
}

impl ReportStatus {
    fn as_str(self) -> &'static str {
        match self {
            Self::Measured => "measured",
            Self::NotYetSampled => "not_yet_sampled",
            Self::Unsupported => "unsupported",
            Self::NotRun => "not_run",
        }
    }
}

fn report_domain(name: &str, status: ReportStatus, value: serde_json::Value) -> serde_json::Value {
    serde_json::json!({
        "name": name,
        "status": status.as_str(),
        "value": value,
    })
}

/// Project the existing runtime diagnostics into the deterministic report
/// consumed by the M6 convergence protocol. This function only reads existing
/// authorities; route, preparation, and timing measurements that have not run
/// remain explicit status/value pairs instead of being represented by a
/// fabricated zero.
pub(crate) fn convergence_report(world: &mut World) -> serde_json::Value {
    let has_streaming = world
        .get_resource::<super::world::exterior::ExteriorStreamState>()
        .is_some();
    let streaming = if has_streaming {
        super::world::exterior::exterior_status_json(&mut *world)
    } else {
        serde_json::Value::Null
    };
    let presentation = super::world::exterior::exterior_presentation_json(world);
    let actor_navigation = actor_navigation_domain(world);
    let travel_save = travel_save_domain(world);
    let environment = environment_domain(world);
    let cache_preparation = cache_preparation_domain(world);
    let (frame_timing, legacy_frame) = frame_timing_domain(world);
    let process_memory = process_memory_domain(&streaming, has_streaming);

    let conversion = serde_json::json!({
        "selected_pipeline": "native",
        "assets_built": null,
        "assets_reused": null,
        "lossy_assets": null,
        "cache_bytes": null,
        "cold_seconds": null,
        "warm_seconds": null,
        "runtime_blender_invocations": null,
        "measurement_status": "not_run",
        "offline_measurements_required": true,
    });
    let runtime = serde_json::json!({
        "frame": legacy_frame,
        "transition_ms_p95": null,
        "transition_ms_p95_status": "not_run",
        "nav_path_ms_p95": null,
        "nav_path_ms_p95_status": "not_run",
        "visible_lod_transitions": presentation["terrain"]["lod_transitions"],
        "timing_measurements_required": true,
    });

    serde_json::json!({
        "schema": CONVERGENCE_REPORT_SCHEMA,
        "status": "partial",
        "gate_87_claimed": false,
        "domains": [
            streaming_domain(&streaming, has_streaming),
            actor_navigation,
            travel_save,
            environment,
            report_domain(
                "presentation",
                if has_streaming {
                    ReportStatus::Measured
                } else {
                    ReportStatus::NotRun
                },
                if has_streaming {
                    presentation.clone()
                } else {
                    serde_json::Value::Null
                },
            ),
            cache_preparation,
            frame_timing,
            process_memory,
        ],
        "conversion": conversion,
        "streaming": streaming,
        "presentation": presentation,
        "runtime": runtime,
    })
}

fn streaming_domain(streaming: &serde_json::Value, has_streaming: bool) -> serde_json::Value {
    report_domain(
        "streaming_lifecycle",
        if has_streaming {
            ReportStatus::Measured
        } else {
            ReportStatus::NotRun
        },
        if has_streaming {
            streaming.clone()
        } else {
            serde_json::Value::Null
        },
    )
}

fn actor_navigation_domain(world: &mut World) -> serde_json::Value {
    let Some(manifest) = world.get_resource::<LoadedSceneManifest>() else {
        return report_domain(
            "actor_navigation",
            ReportStatus::NotRun,
            serde_json::Value::Null,
        );
    };

    let prepared_actor_count = manifest
        .placements
        .iter()
        .filter(|placement| super::actor::is_actor_semantic(&placement.semantic))
        .count();
    let prepared_navigation = manifest
        .exterior
        .as_ref()
        .and_then(|package| package.navigation.as_ref())
        .map(|navigation| {
            serde_json::json!({
                "revision": navigation.revision,
                "mesh_count": navigation.mesh_count,
                "polygon_count": navigation.polygon_count,
                "vertex_count": navigation.vertex_count,
                "door_count": navigation.door_count,
                "external_connection_count": navigation.external_connection_count,
                "mesh_merge_count": navigation.mesh_merge_count,
                "clearance_ready": navigation.clearance_ready,
                "border_portals": navigation.border_portals.len(),
            })
        })
        .or_else(|| {
            manifest.nav_graph.as_ref().map(|graph| {
                serde_json::json!({
                    "revision": graph.revision,
                    "mesh_count": graph.mesh_count,
                    "polygon_count": graph.polygon_count,
                    "asset_path": graph.asset_path,
                })
            })
        });
    let runtime_actor_count = {
        let mut query = world.query_filtered::<Entity, With<super::actor::ActorRuntime>>();
        query.iter(world).count()
    };
    let runtime_diagnostic_count = {
        let mut query = world.query::<&super::actor::ActorRuntimeState>();
        query
            .iter(world)
            .map(|state| state.diagnostics.len())
            .sum::<usize>()
    };

    report_domain(
        "actor_navigation",
        ReportStatus::NotRun,
        serde_json::json!({
            "prepared_actor_count": prepared_actor_count,
            "runtime_actor_count": runtime_actor_count,
            "runtime_diagnostic_count": runtime_diagnostic_count,
            "prepared_navigation": prepared_navigation,
            "route_measurement": {
                "status": "not_run",
                "value": null,
            },
        }),
    )
}

fn travel_save_domain(world: &World) -> serde_json::Value {
    let current_location = world
        .get_resource::<super::world::CurrentWorldLocation>()
        .and_then(|location| location.0.as_ref())
        .and_then(|location| serde_json::to_value(location).ok());
    let persisted = world
        .get_resource::<super::world::ActiveSaveState>()
        .map(|state| {
            let references = state
                .0
                .cells
                .values()
                .map(|cell| cell.references.len())
                .sum::<usize>();
            let dropped_items = state
                .0
                .cells
                .values()
                .map(|cell| cell.dropped_items.len())
                .sum::<usize>();
            let actors = state
                .0
                .cells
                .values()
                .map(|cell| cell.actors.len())
                .sum::<usize>();
            serde_json::json!({
                "cells": state.0.cells.len(),
                "references": references,
                "dropped_items": dropped_items,
                "actors": actors,
            })
        });

    if current_location.is_none() && persisted.is_none() {
        return report_domain("travel_save", ReportStatus::NotRun, serde_json::Value::Null);
    }

    report_domain(
        "travel_save",
        ReportStatus::NotRun,
        serde_json::json!({
            "runtime_snapshot": {
                "status": "measured",
                "value": {
                    "current_location": current_location,
                    "persisted_state": persisted,
                },
            },
            "travel_save_measurement": {
                "status": "not_run",
                "value": null,
            },
        }),
    )
}

fn environment_domain(world: &World) -> serde_json::Value {
    let manifest = world.get_resource::<LoadedSceneManifest>();
    let streamed = world
        .get_resource::<super::world::exterior::ExteriorStreamState>()
        .and_then(|state| state.cells.get(&state.current_grid));
    let exterior_environment = streamed
        .and_then(|cell| cell.package.as_ref())
        .map(|package| &package.environment)
        .or_else(|| manifest.and_then(|value| value.exterior.as_ref().map(|e| &e.environment)));
    let clock = world.get_resource::<super::day_night::GameClock>().copied();
    let transition = world
        .get_resource::<super::day_night::WeatherTransition>()
        .copied();
    let water = world
        .get_resource::<super::world::exterior::ExteriorWaterState>()
        .and_then(|state| state.contact);
    let swimming = world
        .get_resource::<super::world::exterior::SwimmingState>()
        .copied();

    if clock.is_none()
        && transition.is_none()
        && exterior_environment.is_none()
        && water.is_none()
        && swimming.is_none()
    {
        return report_domain("environment", ReportStatus::NotRun, serde_json::Value::Null);
    }

    report_domain(
        "environment",
        ReportStatus::Measured,
        serde_json::json!({
            "hour": clock.map(|clock| clock.hour),
            "timescale": clock.map(|clock| clock.timescale),
            "cell_form_id": streamed
                .map(|cell| format!("{:08x}", cell.state.cell_form_id))
                .or_else(|| manifest.map(|value| format!("{:08x}", value.cell.form_id))),
            "worldspace_form_id": manifest
                .and_then(|value| value.exterior.as_ref())
                .map(|value| format!("{:08x}", value.worldspace_form_id)),
            "climate_form_id": exterior_environment.and_then(|value| value.climate_form_id),
            "weather_form_id": exterior_environment.and_then(|value| value.weather_form_id),
            "image_space_form_id": exterior_environment.and_then(|value| value.image_space_form_id),
            "dynamic_lighting_allowed": exterior_environment
                .map(|value| value.dynamic_lighting_allowed),
            "fog_near": exterior_environment.map(|value| value.fog_near),
            "fog_far": exterior_environment.map(|value| value.fog_far),
            "water": water,
            "swimming": swimming,
            "weather_transition": transition,
        }),
    )
}

fn cache_preparation_domain(world: &mut World) -> serde_json::Value {
    let manifest = world.get_resource::<LoadedSceneManifest>().map(|manifest| {
        serde_json::json!({
            "cell_form_id": format!("{:08x}", manifest.cell.form_id),
            "schema_version": manifest.schema_version,
            "prepare_revision": manifest.prepare_revision,
            "converter_revision": manifest.converter_revision,
            "physics_schema_version": manifest.physics_schema_version,
            "source_fingerprint": manifest.source_fingerprint,
            "placements": manifest.placements.len(),
            "lights": manifest.lights.len(),
            "nav_graph_present": manifest.nav_graph.is_some()
                || manifest
                    .exterior
                    .as_ref()
                    .and_then(|package| package.navigation.as_ref())
                    .is_some(),
            "static_point_shadows_present": manifest.static_point_shadows.is_some(),
        })
    });
    let shadow_runtime_present = world
        .get_resource::<super::lighting::PreparedPointShadowRuntime>()
        .is_some();
    let shadow = shadow_runtime_present.then(|| super::lighting::shadow_cache_status(world));
    let preparation_measurements = serde_json::json!({
        "status": "not_run",
        "value": null,
        "required": [
            "assets_built",
            "assets_reused",
            "lossy_assets",
            "cache_bytes",
            "cold_seconds",
            "warm_seconds",
        ],
    });

    if manifest.is_none() && shadow.is_none() {
        return report_domain(
            "cache_preparation",
            ReportStatus::NotRun,
            serde_json::Value::Null,
        );
    }

    report_domain(
        "cache_preparation",
        if shadow
            .as_ref()
            .and_then(|shadow| shadow["artifact_present"].as_bool())
            .unwrap_or(false)
        {
            ReportStatus::Measured
        } else {
            ReportStatus::NotYetSampled
        },
        serde_json::json!({
            "prepared_manifest": manifest,
            "runtime_shadow_cache": shadow,
            "preparation_measurements": preparation_measurements,
        }),
    )
}

fn frame_timing_domain(world: &World) -> (serde_json::Value, serde_json::Value) {
    let Some(report) = world.get_resource::<RenderReportBuffer>() else {
        return (
            report_domain(
                "frame_timing",
                ReportStatus::NotRun,
                serde_json::Value::Null,
            ),
            serde_json::json!({
                "frame_ms_p50": null,
                "frame_ms_p95": null,
                "frame_ms_max": null,
                "frame_samples": 0,
            }),
        );
    };
    let summary = summarize_render_samples(
        report,
        None,
        RENDER_REPORT_HISTORY,
        CONVERGENCE_FRAME_BUDGET_MS,
    );
    let value = serde_json::to_value(&summary).expect("frame summary is JSON serializable");
    let legacy = serde_json::json!({
        "frame_ms_p50": summary.p50_ms,
        "frame_ms_p95": summary.p95_ms,
        "frame_ms_max": summary.max_ms,
        "frame_samples": summary.sample_count,
    });
    (
        report_domain(
            "frame_timing",
            if summary.sample_count == 0 {
                ReportStatus::NotYetSampled
            } else {
                ReportStatus::Measured
            },
            value,
        ),
        legacy,
    )
}

fn process_memory_domain(streaming: &serde_json::Value, has_streaming: bool) -> serde_json::Value {
    if !has_streaming {
        return report_domain(
            "process_memory",
            ReportStatus::NotRun,
            serde_json::Value::Null,
        );
    }
    let status = match streaming["memory_measurement_status"].as_str() {
        Some("supported") => ReportStatus::Measured,
        Some("not_yet_sampled") => ReportStatus::NotYetSampled,
        Some("unsupported") => ReportStatus::Unsupported,
        _ => ReportStatus::NotRun,
    };
    let value = (matches!(status, ReportStatus::Measured)).then(|| {
        serde_json::json!({
            "resident_bytes": streaming["resident_bytes"],
            "peak_bytes": streaming["peak_memory"],
            "ending_bytes": streaming["ending_memory"],
        })
    });
    let value = value.unwrap_or(serde_json::Value::Null);
    let mut domain = report_domain("process_memory", status, value);
    if let serde_json::Value::Object(fields) = &mut domain {
        fields.insert(
            "method".into(),
            streaming["memory_measurement_method"].clone(),
        );
        fields.insert(
            "metric".into(),
            streaming["memory_measurement_metric"].clone(),
        );
        fields.insert(
            "platform".into(),
            streaming["memory_measurement_platform"].clone(),
        );
        fields.insert(
            "sample_count".into(),
            streaming["memory_sample_count"].clone(),
        );
        fields.insert(
            "trace_active".into(),
            streaming["memory_trace_active"].clone(),
        );
    }
    domain
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
