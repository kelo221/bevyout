//! Runtime state and diagnostics for prepared point-shadow artifacts.

use bevy::pbr::{BakedPointLightShadow, BakedPointShadowGpuStatus, PointLightShadowSamples};
use bevy::prelude::*;
use serde_json::json;

/// A startup-cell light eligible for the single bounded realtime shadow pass.
/// The stable reference id keeps selection deterministic when authored lights
/// have equal strength and avoids depending on ECS spawn order.
#[derive(Component, Debug, Clone, Copy)]
pub(crate) struct RealtimeShadowCandidate {
    pub(crate) reference_form_id: u32,
}

#[derive(Resource, Debug, Default)]
pub(crate) struct RealtimeShadowLight(pub(crate) Option<Entity>);

fn strongest_camera_candidate<I>(camera: Vec3, candidates: I) -> Option<Entity>
where
    I: IntoIterator<Item = (Entity, u32, Vec3, f32, f32)>,
{
    let mut strongest_containing = None;
    let mut nearest_visible = None;
    for (entity, reference_form_id, position, radius, intensity) in candidates {
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
            let replace =
                strongest_containing.is_none_or(|(_, incumbent_form, incumbent_score)| {
                    score > incumbent_score
                        || (score == incumbent_score && reference_form_id < incumbent_form)
                });
            if replace {
                strongest_containing = Some((entity, reference_form_id, score));
            }
        }

        let distance_to_sphere = distance_squared.sqrt() - radius;
        let replace = nearest_visible.is_none_or(|(_, incumbent_form, incumbent_distance)| {
            distance_to_sphere < incumbent_distance
                || (distance_to_sphere == incumbent_distance && reference_form_id < incumbent_form)
        });
        if replace {
            nearest_visible = Some((entity, reference_form_id, distance_to_sphere));
        }
    }

    strongest_containing
        .or(nearest_visible)
        .map(|(entity, _, _)| entity)
}

/// Enables runtime point shadows on exactly one camera-relevant startup light.
///
/// The candidate marker is deliberately only added to startup-cell lights;
/// preloaded neighbor-cell lights remain hard/unshadowed until the selection
/// policy is expanded to follow cell activation.
pub(crate) fn apply_realtime_shadow_light(
    camera: Single<&GlobalTransform, With<Camera3d>>,
    mut selected: ResMut<RealtimeShadowLight>,
    mut lights: Query<
        (
            Entity,
            &RealtimeShadowCandidate,
            &GlobalTransform,
            &mut PointLight,
        ),
        With<RealtimeShadowCandidate>,
    >,
) {
    let strongest = strongest_camera_candidate(
        camera.translation(),
        lights.iter().map(|(entity, candidate, transform, light)| {
            (
                entity,
                candidate.reference_form_id,
                transform.translation(),
                light.range,
                light.intensity,
            )
        }),
    );
    if selected.0 != strongest {
        selected.0 = strongest;
    }

    for (entity, _, _, mut light) in &mut lights {
        let should_enable = Some(entity) == strongest;
        if light.shadow_maps_enabled != should_enable {
            light.shadow_maps_enabled = should_enable;
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
        let mut query = world.query::<&BakedPointLightShadow>();
        query.iter(world).count() as u32
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
    fn strongest_camera_candidate_uses_local_contribution_not_spawn_order() {
        let far = Entity::from_bits(1);
        let near = Entity::from_bits(2);
        let outside = Entity::from_bits(3);
        assert_eq!(
            strongest_camera_candidate(
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
    fn strongest_camera_candidate_tie_breaks_by_reference_form_id() {
        let high_form = Entity::from_bits(7);
        let low_form = Entity::from_bits(8);
        assert_eq!(
            strongest_camera_candidate(
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
    fn strongest_camera_candidate_falls_back_to_nearest_light_sphere() {
        let near = Entity::from_bits(9);
        let far = Entity::from_bits(10);
        assert_eq!(
            strongest_camera_candidate(
                Vec3::ZERO,
                [
                    (far, 10, Vec3::new(0.0, 0.0, 12.0), 2.0, 100.0),
                    (near, 20, Vec3::new(0.0, 0.0, 5.0), 3.0, 1.0),
                ],
            ),
            Some(near)
        );
    }

    #[test]
    fn non_finite_candidate_intensities_are_ignored() {
        let entity = Entity::from_bits(7);
        assert_eq!(
            strongest_camera_candidate(Vec3::ZERO, [(entity, 1, Vec3::ZERO, 4.0, f32::NAN)],),
            None
        );
    }
}
