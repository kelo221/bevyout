//! Render diagnostics and CSV reports.

use super::controls::{LightsDisabled, UnlitMode};
use super::*;

#[derive(Resource)]
pub(crate) struct RenderReportPath(pub(crate) PathBuf);

#[derive(Resource, Default)]
pub(crate) struct RenderReportBuffer {
    next_sample: u64,
    samples: VecDeque<RenderReportSample>,
}

#[derive(Clone, Copy)]
pub(crate) struct RenderReportSample {
    sample: u64,
    frame_time_ms: f64,
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
    let sample = RenderReportSample {
        sample: report.next_sample,
        frame_time_ms,
    };
    report.next_sample = report.next_sample.saturating_add(1);
    report.samples.push_back(sample);
    while report.samples.len() > RENDER_REPORT_HISTORY {
        report.samples.pop_front();
    }
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
        let manifest = world.resource::<PreparedSceneManifest>();
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
        "sample,frame_time_ms,fps,entity_count,mesh_entities,hidden_meshes,named_gltf_meshes,point_lights,directional_lights,irradiance_volumes,cameras,mesh_assets,material_assets,image_assets,manifest_placements,manifest_lights,bloom_intensity,bloom_threshold,bloom_softness,camera_mode,unlit_mode,lights_disabled,physics_disabled,collider_entities,physics_authored_assets,physics_fallback_assets,physics_bodies,physics_shapes,physics_shape_kinds,physics_packed_triangles,physics_filtered_shapes,physics_dynamic_bodies,physics_cooking_ms,physics_sidecar_bytes\n",
    );
    for sample in &samples {
        let fps = if sample.frame_time_ms > f64::EPSILON {
            1000.0 / sample.frame_time_ms
        } else {
            0.0
        };
        csv.push_str(&format!(
            "{},{:.4},{:.4},{},{},{},{},{},{},{},{},{},{},{},{},{},{:.4},{:.4},{:.4},{},{},{},{},{},{},{},{},{},{},{},{},{},{:.3},{}\n",
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
