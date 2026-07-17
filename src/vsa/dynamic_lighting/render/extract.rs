use bevy::{
    camera::primitives::{Frustum, Sphere},
    ecs::system::lifetimeless::Read,
    math::Vec3A,
    prelude::*,
    render::Extract,
};

use super::super::bevy_bridge::{
    DynamicLight, DynamicLightLayerMask, DynamicLightOrdinal, DynamicLightPreparedSource,
    DynamicLightRuntime, DynamicLightingDiagnostics, DynamicLightingSettings,
};
use super::super::core::volumetric_bounding_radius;
use super::DynamicLightingView;
use super::gpu::{
    ExtractedDynamicLight, ExtractedDynamicLights, GpuDynamicLight, GpuDynamicShadow,
    MAX_DYNAMIC_LIGHTS,
};

// The volumetric pass is full-screen, so its cost scales with every submitted
// volume even when most cover the same pixels. Keep the fog configuration on
// every source, but submit only the strongest local contributors for the view.
const MAX_ACTIVE_VOLUMETRIC_LIGHTS: usize = 2;

type DynamicLightExtractQuery = (
    Entity,
    Read<DynamicLight>,
    Read<DynamicLightRuntime>,
    Read<DynamicLightOrdinal>,
    Read<DynamicLightLayerMask>,
    Read<GlobalTransform>,
    Option<Read<InheritedVisibility>>,
    Option<Read<DynamicLightPreparedSource>>,
);

pub(super) fn extract_dynamic_lights(
    mut commands: Commands,
    settings: Extract<Res<DynamicLightingSettings>>,
    diagnostics: Extract<Res<DynamicLightingDiagnostics>>,
    lights: Extract<Query<DynamicLightExtractQuery>>,
    views: Extract<Query<(&Frustum, &GlobalTransform), With<DynamicLightingView>>>,
) {
    commands.insert_resource((*diagnostics).clone());
    let mut sorted = lights
        .iter()
        .filter(|(_, light, _, _, layers, _, inherited, _)| {
            light.config.layer_mask & layers.0 != 0
                && inherited.is_none_or(|visibility| visibility.get())
        })
        .map(
            |(entity, light, runtime, ordinal, _, transform, _, prepared)| {
                let mut volumetric =
                    GpuDynamicLight::from_volumetric_main_world(light, runtime, transform);
                if volumetric.is_some() && !views.is_empty() {
                    let (scale, _, translation) = transform.to_scale_rotation_translation();
                    let bounds = Sphere {
                        center: Vec3A::from(translation),
                        radius: volumetric_bounding_radius(
                            light.config.volumetric,
                            scale.to_array(),
                            light.config.spatial.outer_cutoff_degrees,
                        ),
                    };
                    if !views
                        .iter()
                        .any(|(frustum, _)| frustum.intersects_sphere(&bounds, true))
                    {
                        volumetric = None;
                    }
                }
                let volumetric_influence = volumetric.map_or(0.0, |volume| {
                    views
                        .iter()
                        .map(|(_, view_transform)| {
                            volume_influence(
                                volume.volumetric_intensity,
                                volume.radius_sqr,
                                transform.translation(),
                                view_transform.translation(),
                            )
                        })
                        .fold(0.0, f32::max)
                });
                (
                    ordinal.0,
                    entity,
                    prepared.map(|source| source.reference_form_id),
                    GpuDynamicLight::from_main_world(light, runtime, transform, prepared.is_some()),
                    volumetric,
                    volumetric_influence,
                    prepared
                        .and_then(|source| source.baked_shadow)
                        .map_or_else(GpuDynamicShadow::default, GpuDynamicShadow::from),
                )
            },
        )
        .collect::<Vec<_>>();
    sorted.sort_unstable_by_key(|(ordinal, _, _, _, _, _, _)| *ordinal);
    let source_count = sorted.len();
    let truncated_count = source_count.saturating_sub(MAX_DYNAMIC_LIGHTS);
    let truncation_changed = diagnostics.set_truncated_light_count(truncated_count);
    if truncated_count > 0 && truncation_changed {
        warn!(
            "dynamic lighting truncated {truncated_count} source(s): {source_count} active exceeds GPU limit {MAX_DYNAMIC_LIGHTS}"
        );
    }
    sorted.truncate(MAX_DYNAMIC_LIGHTS);
    diagnostics.set_extracted_light_count(sorted.len());
    let mut volumetric_values = sorted
        .iter()
        .filter_map(|(ordinal, _, _, _, light, influence, _)| {
            light.map(|light| (*influence, *ordinal, light))
        })
        .collect::<Vec<_>>();
    volumetric_values.sort_unstable_by(|left, right| {
        right
            .0
            .total_cmp(&left.0)
            .then_with(|| left.1.cmp(&right.1))
    });
    volumetric_values.truncate(MAX_ACTIVE_VOLUMETRIC_LIGHTS);
    let volumetric_values = volumetric_values
        .into_iter()
        .map(|(_, _, light)| light)
        .collect::<Vec<_>>();
    diagnostics.set_extracted_volumetric_light_count(volumetric_values.len());

    commands.insert_resource(ExtractedDynamicLights {
        values: sorted
            .into_iter()
            .map(
                |(_, main_entity, prepared_reference_form_id, light, _, _, prepared_shadow)| {
                    ExtractedDynamicLight {
                        main_entity,
                        prepared_reference_form_id,
                        light,
                        prepared_shadow,
                    }
                },
            )
            .collect(),
        enabled: settings.enabled,
        volumetric_values,
        volumetric_enabled: settings.enabled && settings.volumetric_enabled,
    });
}

fn volume_influence(intensity: f32, radius: f32, light_position: Vec3, view_position: Vec3) -> f32 {
    let radius_sqr = radius * radius;
    let distance_sqr = light_position.distance_squared(view_position);
    intensity.max(0.0) * radius_sqr / distance_sqr.max(radius_sqr).max(f32::EPSILON)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn volume_influence_prefers_near_strong_sources_and_clamps_inside() {
        let center = Vec3::ZERO;
        assert_eq!(volume_influence(0.5, 4.0, center, center), 0.5);
        assert_eq!(volume_influence(0.5, 4.0, center, Vec3::X * 2.0), 0.5);
        assert!(
            volume_influence(0.5, 4.0, center, Vec3::X * 8.0)
                < volume_influence(0.25, 4.0, center, Vec3::X * 2.0)
        );
    }
}
