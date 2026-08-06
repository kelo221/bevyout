use bevy::math::{Vec2, Vec3, Vec4};

use super::super::rust_scene::TransportMaterial;

#[derive(Clone, Copy, Debug)]
pub(crate) struct MaterialSample {
    pub(crate) base_color: Vec3,
    pub(crate) alpha: f32,
    pub(crate) emissive: Vec3,
}

pub(crate) fn sample_material(
    material: &TransportMaterial,
    uv: Vec2,
    vertex_color: Vec4,
) -> MaterialSample {
    let sampled_base = material
        .base_color_texture
        .as_ref()
        .map_or(Vec4::ONE, |texture| texture.sample(uv));
    let alpha = (material.base_color_factor.w * sampled_base.w * vertex_color.w).clamp(0.0, 1.0);
    // glTF baseColorFactor and COLOR_0 are already linear multipliers. Only
    // the sampled texture carries sRGB encoding.
    let base = material.base_color_factor.truncate()
        * srgb_to_linear_vec3(sampled_base.truncate())
        * vertex_color.truncate();
    let diffuse = base * (1.0 - material.metallic_factor).clamp(0.0, 1.0);
    let emissive_sample = material
        .emissive_texture
        .as_ref()
        .map_or(Vec4::ONE, |texture| texture.sample(uv));
    // emissiveFactor is also authored in linear space; the emissive texture
    // is the only value decoded here.
    let emissive = material.emissive_factor
        * srgb_to_linear_vec3(emissive_sample.truncate())
        * bevyout_core::lighting::EMISSION_SCALE;
    MaterialSample {
        base_color: diffuse,
        alpha,
        emissive,
    }
}

fn srgb_to_linear_vec3(value: Vec3) -> Vec3 {
    Vec3::from_array(bevyout_core::lighting::srgb_to_linear_rgb(value.to_array()))
}
