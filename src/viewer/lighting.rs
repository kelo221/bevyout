//! Runtime state and diagnostics for prepared point-shadow artifacts.

use bevy::pbr::{BakedPointLightShadow, BakedPointShadowGpuStatus, PointLightShadowSamples};
use bevy::prelude::*;
use serde_json::json;

use super::lighting_policy::{
    DEFAULT_SOURCE_RADIUS_METERS, PreparedPointShadowFilter, filter_for_source_radius,
};

/// Apparent source radius used by prepared point lights.
///
/// This is a presentation control only: prepared depth cubemaps remain the
/// source of occlusion data, while Bevy's PCSS filter derives the penumbra
/// from this radius. Dynamic and unprepared lights do not receive this value.
#[derive(Resource, Clone, Copy, Debug, PartialEq)]
pub(crate) struct PointShadowSourceRadius(pub(crate) f32);

impl Default for PointShadowSourceRadius {
    fn default() -> Self {
        Self(DEFAULT_SOURCE_RADIUS_METERS)
    }
}

/// Applies a changed source radius only to prepared lights.
///
/// The change guard is intentional: Bevy's point-light extractor watches
/// `Changed<PointLight>`, so unchanged console state must not rewrite every
/// prepared component each frame. This does not avoid baked texture uploads
/// or clustered-buffer preparation, which are separate renderer stages.
pub(crate) fn apply_point_shadow_source_radius(
    radius: Res<PointShadowSourceRadius>,
    mut prepared_lights: Query<(&mut PointLight, &BakedPointLightShadow)>,
) {
    if !radius.is_changed() {
        return;
    }
    let filter = filter_for_source_radius(radius.0);
    for (mut light, _) in &mut prepared_lights {
        light.radius = radius.0;
        light.soft_shadows_enabled = matches!(filter, PreparedPointShadowFilter::Pcss);
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
        "shadow_radius": world
            .get_resource::<PointShadowSourceRadius>()
            .map_or(DEFAULT_SOURCE_RADIUS_METERS, |radius| radius.0),
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
        world.insert_resource(PointShadowSourceRadius::default());

        let status = shadow_cache_status(&mut world);
        assert_eq!(status["estimated_memory_bytes"], 17_301_504_u64);
        assert_eq!(status["runtime_shadow_passes"], 0);
        assert!((status["shadow_radius"].as_f64().unwrap() - 0.05).abs() < 1e-6);
    }

    #[test]
    fn changed_radius_updates_prepared_lights_only() {
        let mut app = App::new();
        app.insert_resource(PointShadowSourceRadius::default())
            .add_systems(Update, apply_point_shadow_source_radius);
        let prepared = app
            .world_mut()
            .spawn((
                PointLight::default(),
                BakedPointLightShadow {
                    layer: 0,
                    baked_translation: Vec3::ZERO,
                    baked_range: 10.0,
                    near_z: 0.1,
                },
            ))
            .id();
        let dynamic = app.world_mut().spawn(PointLight::default()).id();

        app.update();
        assert_eq!(
            app.world().get::<PointLight>(prepared).unwrap().radius,
            0.05
        );
        assert!(
            app.world()
                .get::<PointLight>(prepared)
                .unwrap()
                .soft_shadows_enabled
        );
        let dynamic_light = app.world().get::<PointLight>(dynamic).unwrap();
        assert_eq!(dynamic_light.radius, 0.0);
        assert!(!dynamic_light.soft_shadows_enabled);

        app.update();
        let mut changed = app
            .world_mut()
            .query_filtered::<Entity, Changed<PointLight>>();
        assert_eq!(changed.iter(app.world()).count(), 0);

        app.world_mut().resource_mut::<PointShadowSourceRadius>().0 = 0.0;
        app.update();
        let light = app.world().get::<PointLight>(prepared).unwrap();
        assert_eq!(light.radius, 0.0);
        assert!(!light.soft_shadows_enabled);
    }
}
