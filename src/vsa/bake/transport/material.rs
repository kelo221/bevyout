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
    let base = srgb_to_linear_vec3(material.base_color_factor.truncate())
        * srgb_to_linear_vec3(sampled_base.truncate())
        * srgb_to_linear_vec3(vertex_color.truncate());
    let diffuse = base * (1.0 - material.metallic_factor).clamp(0.0, 1.0);
    let emissive_sample = material
        .emissive_texture
        .as_ref()
        .map_or(Vec4::ONE, |texture| texture.sample(uv));
    let emissive = srgb_to_linear_vec3(material.emissive_factor)
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
