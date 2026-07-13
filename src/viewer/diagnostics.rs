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

#[derive(SystemParam)]
pub(crate) struct RenderReportParams<'w, 's> {
    keys: Res<'w, ButtonInput<KeyCode>>,
    report: Res<'w, RenderReportBuffer>,
    report_path: Res<'w, RenderReportPath>,
    diagnostics: Res<'w, DiagnosticsStore>,
    meshes: Res<'w, Assets<Mesh>>,
    materials: Res<'w, Assets<StandardMaterial>>,
    images: Res<'w, Assets<Image>>,
    entities: Query<'w, 's, Entity>,
    mesh_entities: Query<'w, 's, (Entity, Option<&'static Visibility>), With<Mesh3d>>,
    named_meshes: Query<'w, 's, &'static GltfMeshName>,
    cameras: Query<'w, 's, &'static Bloom, With<Camera3d>>,
    irradiance_volumes: Query<'w, 's, Entity, With<IrradianceVolume>>,
    point_lights: Query<'w, 's, Entity, With<PointLight>>,
    directional_lights: Query<'w, 's, Entity, With<DirectionalLight>>,
    manifest: Res<'w, PreparedSceneManifest>,
    camera_mode: Res<'w, player::CameraModeState>,
    physics_disabled: Res<'w, player::PhysicsDisabled>,
    unlit_mode: Res<'w, UnlitMode>,
    lights_disabled: Res<'w, LightsDisabled>,
    colliders: Query<'w, 's, Entity, With<player::PhysicsCollider>>,
    physics: Res<'w, player::CollisionRuntimeStats>,
}

pub(crate) fn save_render_report(params: RenderReportParams) {
    if !params.keys.just_pressed(KeyCode::F10) {
        return;
    }

    let mesh_entity_count = params.mesh_entities.iter().count();
    let entity_count = params.entities.iter().count();
    let named_mesh_count = params.named_meshes.iter().count();
    let camera_count = params.cameras.iter().count();
    let hidden_mesh_count = params
        .mesh_entities
        .iter()
        .filter(|(_, visibility)| matches!(visibility, Some(Visibility::Hidden)))
        .count();
    let bloom = params.cameras.single().ok();
    let bloom_intensity = bloom.map_or(0.0, |value| value.intensity);
    let bloom_threshold = bloom.map_or(0.0, |value| value.prefilter.threshold);
    let bloom_softness = bloom.map_or(0.0, |value| value.prefilter.threshold_softness);
    let point_light_count = params.point_lights.iter().count();
    let directional_light_count = params.directional_lights.iter().count();
    let irradiance_volume_count = params.irradiance_volumes.iter().count();
    let camera_mode = format!("{:?}", params.camera_mode.mode);
    let collider_count = params.colliders.iter().count();
    let mut shape_kinds = params.physics.shape_kinds.iter().collect::<Vec<_>>();
    shape_kinds.sort_unstable_by_key(|(kind, _)| **kind);
    let shape_kinds = shape_kinds
        .into_iter()
        .map(|(kind, count)| format!("{kind}:{count}"))
        .collect::<Vec<_>>()
        .join(";");

    let mut csv = String::from(
        "sample,frame_time_ms,fps,entity_count,mesh_entities,hidden_meshes,named_gltf_meshes,point_lights,directional_lights,irradiance_volumes,cameras,mesh_assets,material_assets,image_assets,manifest_placements,manifest_lights,bloom_intensity,bloom_threshold,bloom_softness,camera_mode,unlit_mode,lights_disabled,physics_disabled,collider_entities,physics_authored_assets,physics_fallback_assets,physics_bodies,physics_shapes,physics_shape_kinds,physics_packed_triangles,physics_filtered_shapes,physics_dynamic_bodies,physics_cooking_ms,physics_sidecar_bytes\n",
    );
    for sample in &params.report.samples {
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
            params.meshes.len(),
            params.materials.len(),
            params.images.len(),
            params.manifest.placements.len(),
            params.manifest.lights.len(),
            bloom_intensity,
            bloom_threshold,
            bloom_softness,
            camera_mode,
            u8::from(params.unlit_mode.0),
            u8::from(params.lights_disabled.0),
            u8::from(params.physics_disabled.0),
            collider_count,
            params.physics.authored_assets,
            params.physics.fallback_assets,
            params.physics.bodies,
            params.physics.shapes,
            csv_field(&shape_kinds),
            params.physics.packed_triangles,
            params.physics.filtered_shapes,
            params.physics.dynamic_bodies,
            params.physics.cooking_millis,
            params.physics.sidecar_bytes,
        ));
    }
    if let Err(error) = fs::write(&params.report_path.0, csv) {
        warn!(
            "failed to write render report {}: {error}",
            params.report_path.0.display()
        );
    } else {
        let mut render_diagnostics = params
            .diagnostics
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

        let diagnostics_path = render_diagnostics_report_path(&params.report_path.0);
        let mut diagnostics_csv =
            String::from("diagnostic_path,suffix,value,average,history_len\n");
        for (path, suffix, value, average, history_len) in render_diagnostics {
            diagnostics_csv.push_str(&format!(
                "{},{},{value:.6},{average:.6},{history_len}\n",
                csv_field(&path),
                csv_field(&suffix),
            ));
        }
        if let Err(error) = fs::write(&diagnostics_path, diagnostics_csv) {
            warn!(
                "failed to write render diagnostics report {}: {error}",
                diagnostics_path.display()
            );
        } else {
            info!(
                "wrote render diagnostics report to {}",
                diagnostics_path.display()
            );
        }
        info!(
            "wrote render report with {} samples to {}",
            params.report.samples.len(),
            params.report_path.0.display()
        );
    }
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

pub(crate) fn inspect_center_hit(
    keys: Res<ButtonInput<KeyCode>>,
    cameras: Query<(&Camera, &GlobalTransform), With<Camera3d>>,
    mut raycast: MeshRayCast,
    names: Query<&GltfMeshName>,
) {
    if !keys.just_pressed(KeyCode::Enter) {
        return;
    }
    let Ok((camera, transform)) = cameras.single() else {
        return;
    };
    let Some(viewport) = camera.logical_viewport_size() else {
        return;
    };
    let Ok(ray) = camera.viewport_to_world(transform, viewport * 0.5) else {
        return;
    };
    let settings = MeshRayCastSettings {
        visibility: RayCastVisibility::VisibleInView,
        ..default()
    };
    if let Some((entity, hit)) = raycast.cast_ray(ray, &settings).first() {
        let name = names
            .get(*entity)
            .map(|name| name.0.as_str())
            .unwrap_or("unnamed");
        info!(
            "center hit: entity {entity:?}, mesh {name:?}, distance {:.2}",
            hit.distance
        );
    } else {
        info!("center hit: none");
    }
}
