#define_import_path bevyout_dynamic_lighting::types

struct DynamicLight {
    position: vec3<f32>,
    radius_sqr: f32,
    channel: u32,
    intensity: f32,
    parameter_a: f32,
    parameter_b: f32,
    color: vec3<f32>,
    parameter_c: f32,
    up: vec3<f32>,
    shimmer_scale: f32,
    forward: vec3<f32>,
    shimmer_modifier: f32,
    volumetric_intensity: f32,
    volumetric_visibility: f32,
    cookie_index: u32,
    shadow_cubemap_index: u32,
    falloff_and_bounce: vec4<f32>,
}

struct DynamicLightMeta {
    count: u32,
    enabled: u32,
    shadow_texel_size: f32,
    shadow_near_z: f32,
}

const LIGHT_TYPE_MASK: u32 = 960u;
const LIGHT_TYPE_POINT: u32 = 0u << 6u;
const LIGHT_TYPE_SPOT: u32 = 1u << 6u;
const LIGHT_TYPE_DISCOBALL: u32 = 2u << 6u;
const LIGHT_TYPE_WAVE: u32 = 3u << 6u;
const LIGHT_TYPE_INTERFERENCE: u32 = 4u << 6u;
const LIGHT_TYPE_ROTOR: u32 = 5u << 6u;
const LIGHT_TYPE_SHOCK: u32 = 6u << 6u;
const LIGHT_TYPE_DISCO: u32 = 7u << 6u;
const LIGHT_FLAG_SHADOW: u32 = 1u << 15u;
const INVALID_LIGHT_INDEX: u32 = 0xffffffffu;

fn light_type(light: DynamicLight) -> u32 {
    return light.channel & LIGHT_TYPE_MASK;
}

fn attenuation(light: DynamicLight, distance_sqr: f32) -> f32 {
    let s = saturate(distance_sqr / light.radius_sqr);
    return light.intensity * pow(1.0 - s, 2.0) / (1.0 + light.falloff_and_bounce.x * s);
}

fn bounce_color(light: DynamicLight) -> vec3<f32> {
    return light.falloff_and_bounce.yzw;
}
