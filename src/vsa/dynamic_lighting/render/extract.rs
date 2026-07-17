use bevy::{
    ecs::system::lifetimeless::Read, light::PointLightShadowMap, prelude::*, render::Extract,
};

use super::super::bevy_bridge::{
    DynamicLight, DynamicLightOrdinal, DynamicLightRuntime, DynamicLightViewMask,
    DynamicLightingDiagnostics, DynamicLightingSettings,
};
use super::gpu::{ExtractedDynamicLights, GpuDynamicLight, MAX_DYNAMIC_LIGHTS};

type DynamicLightExtractQuery = (
    Entity,
    Read<DynamicLight>,
    Read<DynamicLightRuntime>,
    Read<DynamicLightOrdinal>,
    Read<DynamicLightViewMask>,
    Read<GlobalTransform>,
);

pub(super) fn extract_dynamic_lights(
    mut commands: Commands,
    settings: Extract<Res<DynamicLightingSettings>>,
    diagnostics: Extract<Res<DynamicLightingDiagnostics>>,
    point_shadow_map: Extract<Res<PointLightShadowMap>>,
    lights: Extract<Query<DynamicLightExtractQuery>>,
) {
    let mut sorted = lights
        .iter()
        .filter(|(_, light, _, _, mask, _)| light.config.view_mask & mask.0 != 0)
        .map(|(entity, light, runtime, ordinal, _, transform)| {
            (
                ordinal.0,
                entity,
                GpuDynamicLight::from_main_world(light, runtime, transform),
            )
        })
        .collect::<Vec<_>>();
    sorted.sort_unstable_by_key(|(ordinal, _, _)| *ordinal);
    sorted.truncate(MAX_DYNAMIC_LIGHTS);
    diagnostics.set_extracted_light_count(sorted.len());

    commands.insert_resource(ExtractedDynamicLights {
        values: sorted
            .into_iter()
            .map(|(_, main_entity, light)| super::gpu::ExtractedDynamicLight { main_entity, light })
            .collect(),
        enabled: settings.enabled,
        shadow_texel_size: 2.0 / point_shadow_map.size as f32,
    });
}
