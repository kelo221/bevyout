#define_import_path bevyout_dynamic_lighting::surface

#import bevy_pbr::{
    mesh_types::MESH_FLAGS_SHADOW_RECEIVER_BIT,
    pbr_deferred_types as deferred_types,
    pbr_types::STANDARD_MATERIAL_FLAGS_UNLIT_BIT,
    utils::octahedral_decode,
}
#import bevy_render::view::View
#import bevyout_dynamic_lighting::{
    spatial::spatial_multiplier,
    types::{DynamicLight, attenuation, bounce_color},
}

const PI: f32 = 3.141592653589793;

struct DynamicLightingSurface {
    world_position: vec3<f32>,
    normal: vec3<f32>,
    base_color: vec3<f32>,
    perceptual_roughness: f32,
    metallic: f32,
    reflectance: f32,
    mesh_flags: u32,
    material_flags: u32,
}

fn reconstruct_surface(
    frag_position: vec4<f32>,
    depth: f32,
    deferred: vec4<u32>,
    view: View,
) -> DynamicLightingSurface {
    let flags = deferred_types::unpack_flags(deferred.a);
    let mesh_material_flags = deferred_types::mesh_material_flags_from_deferred_flags(flags);
    let base_roughness = deferred_types::unpack_unorm4x8_(deferred.r);
    let properties = deferred_types::unpack_unorm4x8_(deferred.b);
    let encoded_normal = deferred_types::unpack_24bit_normal(deferred.a);
    let normal = octahedral_decode(encoded_normal);

    let uv = (frag_position.xy - view.viewport.xy) / view.viewport.zw;
    let ndc = vec3<f32>(uv * vec2<f32>(2.0, -2.0) + vec2<f32>(-1.0, 1.0), depth);
    let homogeneous_world = view.world_from_clip * vec4<f32>(ndc, 1.0);
    let world = homogeneous_world.xyz / homogeneous_world.w;

    return DynamicLightingSurface(
        world,
        normal,
        pow(base_roughness.rgb, vec3<f32>(2.2)),
        base_roughness.a,
        properties.g,
        properties.r,
        mesh_material_flags.x,
        mesh_material_flags.y,
    );
}

fn geometry_schlick_ggx(ndot: f32, roughness: f32) -> f32 {
    let r = roughness + 1.0;
    let k = r * r / 8.0;
    return ndot / (ndot * (1.0 - k) + k);
}

fn fresnel_schlick(cosine: f32, f0: vec3<f32>) -> vec3<f32> {
    return f0 + (vec3<f32>(1.0) - f0) * pow(1.0 - cosine, 5.0);
}

// Mirrors bevy_pbr::pbr_functions::calculate_view. Orthographic cameras have
// a constant view direction; using camera position would fan specular
// highlights across an otherwise parallel projection.
fn view_direction_for_surface(surface: DynamicLightingSurface, view: View) -> vec3<f32> {
    if view.clip_from_view[3].w == 1.0 {
        return normalize(vec3<f32>(
            view.clip_from_world[0].z,
            view.clip_from_world[1].z,
            view.clip_from_world[2].z,
        ));
    }
    return normalize(view.world_position.xyz - surface.world_position);
}

fn material_light_contribution(
    surface: DynamicLightingSurface,
    light: DynamicLight,
    view: View,
    shadow_visibility: f32,
) -> vec3<f32> {
    let light_minus_world = light.position - surface.world_position;
    let distance_sqr = dot(light_minus_world, light_minus_world);
    if distance_sqr > light.radius_sqr || light.radius_sqr <= 0.0 {
        return vec3<f32>(0.0);
    }

    let light_direction = normalize(light_minus_world);
    let spatial = spatial_multiplier(
        light,
        surface.world_position,
        light_minus_world,
        light_direction,
    );
    if spatial <= 0.0 {
        return vec3<f32>(0.0);
    }

    let ndotl = max(dot(surface.normal, light_direction), 0.0);
    let light_scale = attenuation(light, distance_sqr) * spatial * view.exposure;
    let radiance = light.color * light_scale;
    let view_direction = view_direction_for_surface(surface, view);
    let halfway = normalize(view_direction + light_direction);
    let ndotv = max(dot(surface.normal, view_direction), 0.0001);
    let ndoth = max(dot(surface.normal, halfway), 0.0);
    let hdotv = max(dot(halfway, view_direction), 0.0);

    let roughness = max(surface.perceptual_roughness * surface.perceptual_roughness, 0.04);
    let alpha_squared = roughness * roughness;
    let denominator = ndoth * ndoth * (alpha_squared - 1.0) + 1.0;
    let distribution = alpha_squared / max(PI * denominator * denominator, 0.0001);
    let geometry = geometry_schlick_ggx(ndotv, roughness)
        * geometry_schlick_ggx(ndotl, roughness);
    let dielectric_f0 = vec3<f32>(0.16 * surface.reflectance * surface.reflectance);
    let f0 = mix(dielectric_f0, surface.base_color, surface.metallic);
    let fresnel = fresnel_schlick(hdotv, f0);
    let specular = distribution * geometry * fresnel / max(4.0 * ndotv * ndotl, 0.0001);
    let diffuse_weight = (vec3<f32>(1.0) - fresnel) * (1.0 - surface.metallic);
    let diffuse = diffuse_weight * surface.base_color / PI;
    let direct = (diffuse + specular) * radiance * ndotl;

    // The standalone port has no Unity photon texture. This optional local
    // response is an explicitly authored approximation, disabled by default.
    let bounce = surface.base_color
        * (1.0 - surface.metallic)
        * bounce_color(light)
        * light_scale
        * 0.08;
    return (direct + bounce) * shadow_visibility;
}

fn surface_is_unlit(surface: DynamicLightingSurface) -> bool {
    return (surface.material_flags & STANDARD_MATERIAL_FLAGS_UNLIT_BIT) != 0u;
}

fn surface_receives_shadows(surface: DynamicLightingSurface) -> bool {
    return (surface.mesh_flags & MESH_FLAGS_SHADOW_RECEIVER_BIT) != 0u;
}
