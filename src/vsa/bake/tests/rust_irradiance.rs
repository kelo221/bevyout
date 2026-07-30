use super::*;

#[test]
fn hemisphere_samples_are_deterministic_and_inside_the_face() {
    let first = (0..64)
        .map(|sample| cosine_hemisphere_direction(Vec3::Y, 12, sample, 64))
        .collect::<Vec<_>>();
    let second = (0..64)
        .map(|sample| cosine_hemisphere_direction(Vec3::Y, 12, sample, 64))
        .collect::<Vec<_>>();
    assert_eq!(first, second);
    assert!(first.iter().all(|direction| direction.dot(Vec3::Y) >= 0.0));
}

#[test]
fn rgb9e5_black_is_zero_and_bright_values_survive() {
    assert_eq!(pack_rgb9e5(Vec3::ZERO), 0);
    assert_ne!(pack_rgb9e5(Vec3::new(1.0, 2.0, 3.0)), 0);
}

#[test]
fn probe_grid_uses_bevy_xyz_axis_order() {
    let resolution = [3, 2, 4];
    assert_eq!(
        probe_position(0, resolution, Vec3::ZERO, Vec3::ONE),
        Vec3::ZERO
    );
    assert_eq!(
        probe_position(23, resolution, Vec3::ZERO, Vec3::ONE),
        Vec3::ONE
    );
}

#[test]
fn one_bounce_diffuse_is_lit_and_respects_occlusion() {
    fn horizontal_triangle(height: f32) -> IrradianceTriangle {
        IrradianceTriangle {
            vertices: [
                Point3::new(-2.0, height, -2.0),
                Point3::new(0.0, height, 2.0),
                Point3::new(2.0, height, -2.0),
            ],
            normals: [Vec3::Y; 3],
            uvs: [Vec2::ZERO; 3],
            colors: [Vec4::ONE; 3],
            material: 0,
            node_index: 0,
        }
    }

    fn sampled_radiance(mut triangles: Vec<IrradianceTriangle>) -> Vec3 {
        let bvh = Bvh::build(&mut triangles);
        let materials = [TransportMaterial {
            metallic_factor: 0.0,
            ..Default::default()
        }];
        let lights = [JobLight {
            translation: [0.0, 1.0, 0.0],
            rotation_xyzw: [0.0, 0.0, 0.0, 1.0],
            color_rgba: [1.0; 4],
            radius: 4.0,
            intensity_lumens: 0.0,
            kind: "point".into(),
        }];
        trace_radiance(
            &bvh,
            &triangles,
            &materials,
            &lights,
            &DirectionalBakeLight {
                color_rgba: [0.0; 4],
                rotation_xyzw: [0.0, 0.0, 0.0, 1.0],
                illuminance: 0.0,
            },
            Vec3::new(0.0, 0.25, 0.0),
            Vec3::NEG_Y,
            10.0,
        )
    }

    let visible = sampled_radiance(vec![horizontal_triangle(0.0)]);
    let occluded = sampled_radiance(vec![horizontal_triangle(0.0), horizontal_triangle(0.5)]);
    assert!(visible.max_element() > 0.0, "expected a lit diffuse bounce");
    assert_eq!(occluded, Vec3::ZERO);
}
