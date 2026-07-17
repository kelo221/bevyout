use bevy::{ecs::system::lifetimeless::Read, prelude::*, render::Extract};

use super::super::bevy_bridge::{
    DynamicLight, DynamicLightLayerMask, DynamicLightOrdinal, DynamicLightRuntime,
    DynamicLightingDiagnostics, DynamicLightingSettings,
};
use super::gpu::{ExtractedDynamicLights, GpuDynamicLight, MAX_DYNAMIC_LIGHTS};

type DynamicLightExtractQuery = (
    Entity,
    Read<DynamicLight>,
    Read<DynamicLightRuntime>,
    Read<DynamicLightOrdinal>,
    Read<DynamicLightLayerMask>,
    Read<GlobalTransform>,
);

pub(super) fn extract_dynamic_lights(
    mut commands: Commands,
    settings: Extract<Res<DynamicLightingSettings>>,
    diagnostics: Extract<Res<DynamicLightingDiagnostics>>,
    lights: Extract<Query<DynamicLightExtractQuery>>,
) {
    let mut sorted = lights
        .iter()
        .filter(|(_, light, _, _, layers, _)| light.config.layer_mask & layers.0 != 0)
        .map(|(entity, light, runtime, ordinal, _, transform)| {
            (
                ordinal.0,
                entity,
                GpuDynamicLight::from_main_world(light, runtime, transform),
                GpuDynamicLight::from_volumetric_main_world(light, runtime, transform),
            )
        })
        .collect::<Vec<_>>();
    sorted.sort_unstable_by_key(|(ordinal, _, _, _)| *ordinal);
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
    let volumetric_values = sorted
        .iter()
        .filter_map(|(_, _, _, light)| *light)
        .collect::<Vec<_>>();
    diagnostics.set_extracted_volumetric_light_count(volumetric_values.len());

    commands.insert_resource(ExtractedDynamicLights {
        values: sorted
            .into_iter()
            .map(
                |(_, main_entity, light, _)| super::gpu::ExtractedDynamicLight {
                    main_entity,
                    light,
                },
            )
            .collect(),
        enabled: settings.enabled,
        volumetric_values,
        volumetric_enabled: settings.enabled && settings.volumetric_enabled,
    });
}
