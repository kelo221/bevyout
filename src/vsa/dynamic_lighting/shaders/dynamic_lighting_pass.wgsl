#import bevy_render::view::View
#import bevyout_dynamic_lighting::{
    surface::{DynamicLightingSurface, material_light_contribution, reconstruct_surface, surface_is_unlit, surface_receives_prepared_shadows, surface_receives_shadows},
    types::{DynamicLight, DynamicLightMeta, DynamicLightShadow, INVALID_LIGHT_INDEX, LIGHT_FLAG_SHADOW},
}

@group(0) @binding(0) var source_hdr: texture_2d<f32>;
@group(0) @binding(1) var source_sampler: sampler;
@group(0) @binding(2) var depth_texture: texture_depth_2d;
@group(0) @binding(3) var deferred_texture: texture_2d<u32>;
@group(0) @binding(4) var<uniform> view: View;
@group(0) @binding(5) var<storage, read> dynamic_lights: array<DynamicLight>;
@group(0) @binding(6) var<uniform> dynamic_light_meta: DynamicLightMeta;
@group(0) @binding(7) var point_shadow_textures: texture_depth_cube_array;
@group(0) @binding(8) var point_shadow_sampler: sampler_comparison;
@group(0) @binding(9) var<storage, read> dynamic_light_shadows: array<DynamicLightShadow>;
@group(0) @binding(10) var<storage, read> prepared_light_shadows: array<DynamicLightShadow>;
@group(0) @binding(11) var prepared_shadow_textures: texture_depth_cube_array;

fn realtime_shadow_visibility(surface: DynamicLightingSurface, light: DynamicLight, shadow: DynamicLightShadow) -> f32 {
    if !surface_receives_shadows(surface) ||
            (light.channel & LIGHT_FLAG_SHADOW) == 0u ||
            shadow.cubemap_index == INVALID_LIGHT_INDEX {
        return 1.0;
    }

    let surface_to_light = light.position - surface.world_position;
    let distance_to_light = max(max(abs(surface_to_light.x), abs(surface_to_light.y)), abs(surface_to_light.z));
    let normal_offset = shadow.normal_bias * distance_to_light * surface.normal;
    let depth_offset = shadow.depth_bias * normalize(surface_to_light);
    let offset_position = surface.world_position + normal_offset + depth_offset;
    let light_local = offset_position - light.position;
    let major_axis = max(max(abs(light_local.x), abs(light_local.y)), abs(light_local.z));
    let comparison_depth = shadow.near_z / max(major_axis, shadow.near_z);
    return textureSampleCompareLevel(
        point_shadow_textures,
        point_shadow_sampler,
        light_local * vec3<f32>(1.0, 1.0, -1.0),
        i32(shadow.cubemap_index),
        comparison_depth,
    );
}

fn prepared_shadow_visibility(surface: DynamicLightingSurface, light: DynamicLight, shadow: DynamicLightShadow) -> f32 {
    if !surface_receives_prepared_shadows(surface) ||
            (light.channel & LIGHT_FLAG_SHADOW) == 0u ||
            shadow.cubemap_index == INVALID_LIGHT_INDEX {
        return 1.0;
    }

    let surface_to_light = light.position - surface.world_position;
    let distance_to_light = max(max(abs(surface_to_light.x), abs(surface_to_light.y)), abs(surface_to_light.z));
    let normal_offset = shadow.normal_bias * distance_to_light * surface.normal;
    let depth_offset = shadow.depth_bias * normalize(surface_to_light);
    let offset_position = surface.world_position + normal_offset + depth_offset;
    let light_local = offset_position - light.position;
    let major_axis = max(max(abs(light_local.x), abs(light_local.y)), abs(light_local.z));
    let comparison_depth = shadow.near_z / max(major_axis, shadow.near_z);
    return textureSampleCompareLevel(
        prepared_shadow_textures,
        point_shadow_sampler,
        light_local * vec3<f32>(1.0, 1.0, -1.0),
        i32(shadow.cubemap_index),
        comparison_depth,
    );
}

@fragment
fn fragment(@builtin(position) frag_position: vec4<f32>) -> @location(0) vec4<f32> {
    let pixel = vec2<i32>(frag_position.xy);
    let source = textureLoad(source_hdr, pixel, 0);
    if dynamic_light_meta.enabled == 0u || dynamic_light_meta.count == 0u ||
            dynamic_light_meta.padding_a != 0.0 || dynamic_light_meta.padding_b != 0.0 {
        return source;
    }
    let depth = textureLoad(depth_texture, pixel, 0);
    if depth <= 0.0 {
        return source;
    }
    let deferred = textureLoad(deferred_texture, pixel, 0);
    let surface = reconstruct_surface(frag_position, depth, deferred, view);
    if surface_is_unlit(surface) {
        return source;
    }

    var added_light = vec3<f32>(0.0);
    var dominant_shadow_index = INVALID_LIGHT_INDEX;
    var dominant_shadow_score = -1.0;
    var dominant_shadow_contribution = vec3<f32>(0.0);
    for (var index = 0u; index < dynamic_light_meta.count; index += 1u) {
        let light = dynamic_lights[index];
        let contribution = material_light_contribution(surface, light, view);
        added_light += contribution;
        let realtime_capable = surface_receives_shadows(surface) &&
            dynamic_light_shadows[index].cubemap_index != INVALID_LIGHT_INDEX;
        let prepared_capable = surface_receives_prepared_shadows(surface) &&
            prepared_light_shadows[index].cubemap_index != INVALID_LIGHT_INDEX;
        let shadow_capable = (light.channel & LIGHT_FLAG_SHADOW) != 0u &&
            (realtime_capable || prepared_capable);
        let score = dot(max(contribution, vec3<f32>(0.0)), vec3<f32>(0.2126, 0.7152, 0.0722));
        if shadow_capable && score > dominant_shadow_score {
            dominant_shadow_index = index;
            dominant_shadow_score = score;
            dominant_shadow_contribution = contribution;
        }
    }
    if dominant_shadow_index != INVALID_LIGHT_INDEX {
        let dominant_light = dynamic_lights[dominant_shadow_index];
        let realtime_visibility = realtime_shadow_visibility(
            surface,
            dominant_light,
            dynamic_light_shadows[dominant_shadow_index],
        );
        let prepared_visibility = prepared_shadow_visibility(
            surface,
            dominant_light,
            prepared_light_shadows[dominant_shadow_index],
        );
        let combined_visibility = min(realtime_visibility, prepared_visibility);
        added_light -= dominant_shadow_contribution * (1.0 - combined_visibility);
    }
    return vec4<f32>(source.rgb + added_light, source.a);
}
