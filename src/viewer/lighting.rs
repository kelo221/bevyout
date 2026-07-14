//! Runtime state and diagnostics for prepared point-shadow artifacts.

use bevy::pbr::{BakedPointLightShadow, BakedPointShadowGpuStatus, PointLightShadowSamples};
use bevy::prelude::*;
use serde_json::json;

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
        "runtime_shadow_passes": 0,
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
}
