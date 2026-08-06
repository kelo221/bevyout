use super::super::lightmap_uv::*;
use super::super::rust_scene::ComposedPrimitive;
use bevy::math::{Vec2, Vec3, Vec4};

#[test]
fn unwrapping_remaps_every_vertex_attribute_and_assigns_stable_binding() {
    let mut primitive = ComposedPrimitive {
        name: "quad".into(),
        primitive_key: "cell:fixture/quad".into(),
        reference_form_ids: vec![1],
        material: 0,
        positions: vec![Vec3::ZERO, Vec3::X, Vec3::ONE, Vec3::Y],
        normals: vec![Vec3::Z; 4],
        uvs: vec![Vec2::ZERO, Vec2::X, Vec2::ONE, Vec2::Y],
        colors: vec![Vec4::ONE; 4],
        transport_colors: vec![Vec4::ONE; 4],
        indices: vec![0, 1, 2, 0, 2, 3],
        uv1: Vec::new(),
        uv1_chart_ids: Vec::new(),
        lightmap_texels_per_meter: 16.0,
        lightmap_dimensions: [0, 0],
        lightmap_binding_id: None,
    };
    let source = primitive.clone();
    unwrap_primitive(&mut primitive).unwrap();
    assert_eq!(primitive.positions.len(), primitive.uv1.len());
    assert_eq!(primitive.positions.len(), primitive.uv1_chart_ids.len());
    assert_eq!(primitive.normals.len(), primitive.colors.len());
    assert_eq!(primitive.indices.len(), source.indices.len());
    assert!(primitive.uv1.iter().all(|uv| {
        uv.x.is_finite()
            && uv.y.is_finite()
            && (0.0..=1.0).contains(&uv.x)
            && (0.0..=1.0).contains(&uv.y)
    }));
    assert!(primitive.lightmap_binding_id.is_some());
    assert!(primitive.lightmap_dimensions[0] > 0);
    assert!(primitive.lightmap_dimensions[1] > 0);
}
