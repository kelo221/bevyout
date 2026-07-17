//! Runtime state and diagnostics for prepared point-shadow artifacts.

use bevy::pbr::{BakedPointShadowGpuStatus, PointLightShadowSamples};
use bevy::prelude::*;
use serde_json::json;

use crate::vsa::DynamicLightPreparedSource;
use crate::vsa::{DynamicLight, DynamicLightShadowProxy, DynamicLightingView};

#[derive(Resource, Debug, Default)]
pub(crate) struct RealtimeShadowLight(pub(crate) Option<Entity>);

fn strongest_camera_source<I>(camera: Vec3, sources: I) -> Option<Entity>
where
    I: IntoIterator<Item = (Entity, u32, Vec3, f32, f32)>,
{
    let mut strongest_containing = None;
    let mut nearest_visible = None;
    for (entity, reference_form_id, position, radius, intensity) in sources {
        let distance_squared = position.distance_squared(camera);
        let radius_squared = radius * radius;
        if !distance_squared.is_finite()
            || !radius_squared.is_finite()
            || radius_squared <= 0.0
            || !intensity.is_finite()
        {
            continue;
        }
        if distance_squared <= radius_squared {
            let normalized = (distance_squared / radius_squared).clamp(0.0, 1.0);
            let smooth_factor = (1.0 - normalized * normalized).clamp(0.0, 1.0);
            let score = intensity * smooth_factor * smooth_factor / distance_squared.max(0.0001);
            if strongest_containing.is_none_or(
                |(_, incumbent_form, incumbent_score): (Entity, u32, f32)| {
                    score > incumbent_score
                        || (score == incumbent_score && reference_form_id < incumbent_form)
                },
            ) {
                strongest_containing = Some((entity, reference_form_id, score));
            }
        }

        // The camera can stand just outside a lamp's authored sphere while
        // still seeing its lit receivers. Keep the one bounded cubemap on
        // the nearest sphere in that case instead of leaving it pinned to
        // an unrelated startup light.
        let distance_to_sphere = distance_squared.sqrt() - radius;
        if nearest_visible.is_none_or(
            |(_, incumbent_form, incumbent_distance): (Entity, u32, f32)| {
                distance_to_sphere < incumbent_distance
                    || (distance_to_sphere == incumbent_distance
                        && reference_form_id < incumbent_form)
            },
        ) {
            nearest_visible = Some((entity, reference_form_id, distance_to_sphere));
        }
    }
    strongest_containing
        .or(nearest_visible)
        .map(|(entity, _, _)| entity)
}

/// Keeps the single realtime cubemap on the strongest prepared light that
/// affects the current camera, or the nearest light sphere when the camera is
/// just outside all of them. Equal-strength startup lights are common in
/// Fallout interiors; selecting only by authored intensity can pin the shadow
/// pass to an arbitrary lamp in a distant room.
pub(crate) fn retarget_realtime_shadow_proxy(
    camera: Single<&GlobalTransform, With<DynamicLightingView>>,
    sources: Query<(
        Entity,
        &DynamicLightPreparedSource,
        &DynamicLight,
        &GlobalTransform,
    )>,
    mut proxies: Query<&mut DynamicLightShadowProxy>,
) {
    let selected = strongest_camera_source(
        camera.translation(),
        sources.iter().map(|(entity, prepared, light, transform)| {
            (
                entity,
                prepared.reference_form_id,
                transform.translation(),
                light.config.radius,
                light.config.intensity,
            )
        }),
    );
    let Some(selected) = selected else {
        return;
    };
    for mut proxy in &mut proxies {
        if proxy.dynamic_light != selected {
            proxy.dynamic_light = selected;
        }
    }
}

#[derive(Resource, Clone, Debug, Default)]
pub(crate) struct PreparedPointShadowRuntime {
    pub(crate) revision: Option<String>,
    pub(crate) fingerprint: Option<String>,
    pub(crate) asset_path: Option<String>,
    pub(crate) resolution: u32,
    pub(crate) near_z: f32,
    pub(crate) layers: u32,
    pub(crate) attached_lights: u32,
    pub(crate) cpu_loaded: bool,
    pub(crate) load_error: Option<String>,
}

pub(crate) fn shadow_cache_status(world: &mut World) -> serde_json::Value {
    let runtime = world
        .get_resource::<PreparedPointShadowRuntime>()
        .cloned()
        .unwrap_or_default();
    let samples = world
        .get_resource::<PointLightShadowSamples>()
        .map_or(1, |samples| samples.0);
    let gpu = world
        .get_resource::<BakedPointShadowGpuStatus>()
        .map(BakedPointShadowGpuStatus::snapshot)
        .unwrap_or_default();
    let attached_now = {
        let mut query = world.query::<&DynamicLightPreparedSource>();
        query
            .iter(world)
            .filter(|source| source.baked_shadow.is_some())
            .count() as u32
    };
    let runtime_shadow_passes = world
        .get_resource::<RealtimeShadowLight>()
        .and_then(|selected| selected.0)
        .map_or(0, |_| 1);
    let estimated_bytes = u64::from(runtime.resolution)
        .saturating_mul(u64::from(runtime.resolution))
        .saturating_mul(u64::from(runtime.layers))
        .saturating_mul(6)
        .saturating_mul(4);

    json!({
        "artifact_present": runtime.asset_path.is_some(),
        "asset_load_state": if runtime.cpu_loaded { "cpu-loaded" } else if runtime.load_error.is_some() { "failed" } else { "not-configured" },
        "load_error": runtime.load_error,
        "gpu_ready": gpu.ready,
        "gpu_supported": gpu.supported,
        "gpu_capacity_cubemaps": gpu.max_cubemaps,
        "revision": runtime.revision,
        "fingerprint": runtime.fingerprint,
        "asset_path": runtime.asset_path,
        "resolution": runtime.resolution,
        "near_z": runtime.near_z,
        "layers": runtime.layers,
        "attached_lights": attached_now.max(runtime.attached_lights),
        "estimated_memory_bytes": estimated_bytes,
        "shadow_samples_per_pixel": samples,
        "runtime_shadow_passes": runtime_shadow_passes,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn status_reports_prepared_memory_without_a_runtime_cache() {
        let mut world = World::new();
        world.insert_resource(PreparedPointShadowRuntime {
            resolution: 256,
            layers: 11,
            ..default()
        });
        world.insert_resource(PointLightShadowSamples(1));

        let status = shadow_cache_status(&mut world);
        assert_eq!(status["estimated_memory_bytes"], 17_301_504_u64);
        assert_eq!(status["runtime_shadow_passes"], 0);
    }

    #[test]
    fn realtime_source_selection_uses_camera_contribution_not_spawn_order() {
        let far = Entity::from_bits(1);
        let near = Entity::from_bits(2);
        let outside = Entity::from_bits(3);
        assert_eq!(
            strongest_camera_source(
                Vec3::ZERO,
                [
                    (far, 10, Vec3::new(0.0, 0.0, 3.0), 4.0, 8.0),
                    (near, 20, Vec3::new(0.0, 0.0, 1.0), 4.0, 8.0),
                    (outside, 5, Vec3::new(0.0, 0.0, 5.0), 4.0, 100.0),
                ],
            ),
            Some(near)
        );
    }

    #[test]
    fn realtime_source_selection_is_stable_for_equal_contribution() {
        let low_form = Entity::from_bits(7);
        let high_form = Entity::from_bits(8);
        assert_eq!(
            strongest_camera_source(
                Vec3::ZERO,
                [
                    (high_form, 20, Vec3::Z, 4.0, 8.0),
                    (low_form, 10, Vec3::Z, 4.0, 8.0),
                ],
            ),
            Some(low_form)
        );
    }

    #[test]
    fn realtime_source_selection_falls_back_to_nearest_light_sphere() {
        let near = Entity::from_bits(9);
        let far = Entity::from_bits(10);
        assert_eq!(
            strongest_camera_source(
                Vec3::ZERO,
                [
                    (far, 10, Vec3::new(0.0, 0.0, 12.0), 2.0, 100.0),
                    (near, 20, Vec3::new(0.0, 0.0, 5.0), 3.0, 1.0),
                ],
            ),
            Some(near)
        );
    }
}
