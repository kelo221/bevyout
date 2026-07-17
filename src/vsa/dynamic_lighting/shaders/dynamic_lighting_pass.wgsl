#import bevy_render::view::View
#import bevyout_dynamic_lighting::{
    surface::{DynamicLightingSurface, material_light_contribution, reconstruct_surface, surface_is_unlit, surface_receives_shadows},
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

fn shadow_visibility(surface: DynamicLightingSurface, light: DynamicLight, shadow: DynamicLightShadow) -> f32 {
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
    for (var index = 0u; index < dynamic_light_meta.count; index += 1u) {
        let light = dynamic_lights[index];
        added_light += material_light_contribution(
            surface,
            light,
            view,
            shadow_visibility(surface, light, dynamic_light_shadows[index]),
        );
    }
    return vec4<f32>(source.rgb + added_light, source.a);
}
