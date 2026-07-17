//! Runtime state and diagnostics for prepared point-shadow artifacts.

use bevy::pbr::{BakedPointLightShadow, BakedPointShadowGpuStatus, PointLightShadowSamples};
use bevy::prelude::*;
use serde_json::json;

#[derive(Component, Debug, Clone, Copy, Default)]
pub(crate) struct RealtimeShadowCandidate;

#[derive(Resource, Debug, Default)]
pub(crate) struct RealtimeShadowLight(pub(crate) Option<Entity>);

fn strongest_candidate<I>(candidates: I) -> Option<Entity>
where
    I: IntoIterator<Item = (Entity, f32)>,
{
    candidates
        .into_iter()
        .filter(|(_, intensity)| intensity.is_finite())
        .max_by(
            |(left_entity, left_intensity), (right_entity, right_intensity)| {
                left_intensity
                    .total_cmp(right_intensity)
                    .then_with(|| right_entity.cmp(left_entity))
            },
        )
        .map(|(entity, _)| entity)
}

/// Enables runtime point shadows on exactly one strongest startup light.
///
/// The candidate marker is deliberately only added to startup-cell lights;
/// preloaded neighbor-cell lights remain hard/unshadowed until the selection
/// policy is expanded to follow cell activation.
pub(crate) fn apply_realtime_shadow_light(
    mut selected: ResMut<RealtimeShadowLight>,
    mut lights: Query<(Entity, &mut PointLight), With<RealtimeShadowCandidate>>,
) {
    let strongest = strongest_candidate(
        lights
            .iter()
            .map(|(entity, light)| (entity, light.intensity)),
    );
    if selected.0 != strongest {
        selected.0 = strongest;
    }

    for (entity, mut light) in &mut lights {
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
    fn strongest_candidate_uses_intensity_and_stable_entity_tie_breaking() {
        let low = Entity::from_bits(1);
        let high = Entity::from_bits(2);
        assert_eq!(strongest_candidate([(low, 2.0), (high, 8.0)]), Some(high));
        assert_eq!(strongest_candidate([(low, 8.0), (high, 8.0)]), Some(low));
    }

    #[test]
    fn non_finite_candidate_intensities_are_ignored() {
        let entity = Entity::from_bits(7);
        assert_eq!(strongest_candidate([(entity, f32::NAN)]), None);
    }
}
