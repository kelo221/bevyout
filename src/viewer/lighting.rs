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

/// Runtime opt-in for the native point-shadow pass. Prepared point shadows
/// remain available regardless of this setting.
#[derive(Resource, Debug, Clone, Copy, Default)]
pub(crate) struct RealtimeShadowSettings {
    pub(crate) enabled: bool,
}

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
    settings: Res<RealtimeShadowSettings>,
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
    if !settings.enabled {
        selected.0 = None;
        for (_, _, _, mut light) in &mut lights {
            light.shadow_maps_enabled = false;
        }
        return;
    }

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
        .filter(|_| {
            world
                .get_resource::<RealtimeShadowSettings>()
                .is_none_or(|settings| settings.enabled)
        })
        .map_or(0, |_| 1);
    let realtime_shadows_enabled = world
        .get_resource::<RealtimeShadowSettings>()
        .is_some_and(|settings| settings.enabled);
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
        "realtime_shadows_enabled": realtime_shadows_enabled,
        "runtime_shadow_passes": runtime_shadow_passes,
    })
}

#[cfg(test)]
#[path = "tests/lighting.rs"]
mod tests;
