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
fn spotlight_transport_uses_cone_orientation_and_falloff() {
    let mut light = JobLight {
        translation: [0.0, 0.0, 0.0],
        rotation_xyzw: Quat::IDENTITY.to_array(),
        color_rgba: [1.0; 4],
        radius: 10.0,
        intensity_lumens: 1.0,
        kind: "spot".into(),
        flags: 0,
        spot_fov_radians: std::f32::consts::FRAC_PI_2,
        spot_falloff_exponent: 2.0,
    };

    assert_eq!(spot_angular_factor(&light, Vec3::NEG_Z), 1.0);
    assert_eq!(spot_angular_factor(&light, Vec3::X), 0.0);

    let rotated = Quat::from_rotation_y(std::f32::consts::FRAC_PI_2);
    light.rotation_xyzw = rotated.to_array();
    let rotated_axis = rotated * Vec3::NEG_Z;
    assert_eq!(spot_angular_factor(&light, rotated_axis), 1.0);
    assert_eq!(spot_angular_factor(&light, -rotated_axis), 0.0);

    light.rotation_xyzw = Quat::IDENTITY.to_array();
    let intermediate = Vec3::new(0.0, 0.9, -1.0).normalize();
    let factor = spot_angular_factor(&light, intermediate);
    assert!(factor > 0.0 && factor < 1.0);
}

#[test]
fn point_lights_and_legacy_zero_cones_remain_omnidirectional() {
    let point = JobLight {
        translation: [0.0, 0.0, 0.0],
        rotation_xyzw: Quat::IDENTITY.to_array(),
        color_rgba: [1.0; 4],
        radius: 10.0,
        intensity_lumens: 1.0,
        kind: "point".into(),
        flags: 0,
        spot_fov_radians: 0.0,
        spot_falloff_exponent: 0.0,
    };
    assert_eq!(spot_angular_factor(&point, Vec3::X), 1.0);

    let mut legacy_spot = point;
    legacy_spot.kind = "spot".into();
    assert_eq!(spot_angular_factor(&legacy_spot, Vec3::X), 1.0);
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
            flags: 0,
            spot_fov_radians: 0.0,
            spot_falloff_exponent: 0.0,
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
            Vec3::ZERO,
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

#[test]
fn point_light_response_matches_explicit_and_fallback_contract() {
    let triangle = IrradianceTriangle {
        vertices: [
            Point3::new(-2.0, 0.0, -2.0),
            Point3::new(0.0, 0.0, 2.0),
            Point3::new(2.0, 0.0, -2.0),
        ],
        normals: [Vec3::Y; 3],
        uvs: [Vec2::ZERO; 3],
        colors: [Vec4::ONE; 3],
        material: 0,
        node_index: 0,
    };

    fn radiance(triangle: IrradianceTriangle, intensity_lumens: f32) -> Vec3 {
        let mut triangles = vec![triangle];
        let bvh = Bvh::build(&mut triangles);
        trace_radiance(
            &bvh,
            &triangles,
            &[TransportMaterial {
                metallic_factor: 0.0,
                ..Default::default()
            }],
            &[JobLight {
                translation: [0.0, 1.0, 0.0],
                rotation_xyzw: [0.0, 0.0, 0.0, 1.0],
                color_rgba: [1.0; 4],
                radius: 4.0,
                intensity_lumens,
                kind: "point".into(),
                flags: 0,
                spot_fov_radians: 0.0,
                spot_falloff_exponent: 0.0,
            }],
            &DirectionalBakeLight {
                color_rgba: [0.0; 4],
                rotation_xyzw: [0.0, 0.0, 0.0, 1.0],
                illuminance: 0.0,
            },
            Vec3::ZERO,
            Vec3::new(0.0, 0.25, 0.0),
            Vec3::NEG_Y,
            10.0,
        )
    }

    let explicit = radiance(
        triangle.clone(),
        bevyout_core::lighting::AUTHORED_LIGHTING_SCALE,
    );
    let fallback = radiance(triangle, 0.0);
    assert!(explicit.x > 0.0);
    let expected_ratio = bevyout_core::lighting::DEFAULT_LIGHTING_SCALE
        / bevyout_core::lighting::AUTHORED_LIGHTING_SCALE
        * (4.0 * 4.0 * 2.0 * bevyout_core::lighting::AUTHORED_LIGHTING_SCALE)
        / bevyout_core::lighting::DEFAULT_LIGHTING_SCALE;
    assert!((fallback.x / explicit.x - expected_ratio).abs() < 1e-4);
}

#[test]
fn cornell_box_fixture_preserves_colored_wall_response() {
    fn triangle(
        a: [f32; 3],
        b: [f32; 3],
        c: [f32; 3],
        normal: Vec3,
        material: usize,
    ) -> IrradianceTriangle {
        IrradianceTriangle {
            vertices: [Point3::from(a), Point3::from(b), Point3::from(c)],
            normals: [normal; 3],
            uvs: [Vec2::ZERO; 3],
            colors: [Vec4::ONE; 3],
            material,
            node_index: 0,
        }
    }

    fn quad(corners: [[f32; 3]; 4], normal: Vec3, material: usize) -> [IrradianceTriangle; 2] {
        [
            triangle(corners[0], corners[1], corners[2], normal, material),
            triangle(corners[0], corners[2], corners[3], normal, material),
        ]
    }

    let mut triangles = Vec::new();
    triangles.extend(quad(
        [
            [-1.0, 0.0, -1.0],
            [1.0, 0.0, -1.0],
            [1.0, 0.0, 1.0],
            [-1.0, 0.0, 1.0],
        ],
        Vec3::Y,
        0,
    ));
    triangles.extend(quad(
        [
            [-1.0, 2.0, 1.0],
            [1.0, 2.0, 1.0],
            [1.0, 2.0, -1.0],
            [-1.0, 2.0, -1.0],
        ],
        Vec3::NEG_Y,
        0,
    ));
    triangles.extend(quad(
        [
            [-1.0, 0.0, 1.0],
            [1.0, 0.0, 1.0],
            [1.0, 2.0, 1.0],
            [-1.0, 2.0, 1.0],
        ],
        Vec3::NEG_Z,
        0,
    ));
    triangles.extend(quad(
        [
            [-1.0, 0.0, -1.0],
            [-1.0, 2.0, -1.0],
            [1.0, 2.0, -1.0],
            [1.0, 0.0, -1.0],
        ],
        Vec3::Z,
        0,
    ));
    triangles.extend(quad(
        [
            [-1.0, 0.0, -1.0],
            [-1.0, 0.0, 1.0],
            [-1.0, 2.0, 1.0],
            [-1.0, 2.0, -1.0],
        ],
        Vec3::X,
        1,
    ));
    triangles.extend(quad(
        [
            [1.0, 0.0, 1.0],
            [1.0, 0.0, -1.0],
            [1.0, 2.0, -1.0],
            [1.0, 2.0, 1.0],
        ],
        Vec3::NEG_X,
        2,
    ));

    let bvh = Bvh::build(&mut triangles);
    let radiance = trace_radiance(
        &bvh,
        &triangles,
        &[
            TransportMaterial::default(),
            TransportMaterial {
                base_color_factor: Vec4::new(0.8, 0.0, 0.0, 1.0),
                metallic_factor: 0.0,
                ..Default::default()
            },
            TransportMaterial {
                base_color_factor: Vec4::new(0.0, 0.8, 0.0, 1.0),
                metallic_factor: 0.0,
                ..Default::default()
            },
        ],
        &[JobLight {
            translation: [-0.25, 1.0, 0.0],
            rotation_xyzw: [0.0, 0.0, 0.0, 1.0],
            color_rgba: [1.0; 4],
            radius: 4.0,
            intensity_lumens: bevyout_core::lighting::AUTHORED_LIGHTING_SCALE,
            kind: "point".into(),
            flags: 0,
            spot_fov_radians: 0.0,
            spot_falloff_exponent: 0.0,
        }],
        &DirectionalBakeLight {
            color_rgba: [0.0; 4],
            rotation_xyzw: [0.0, 0.0, 0.0, 1.0],
            illuminance: 0.0,
        },
        Vec3::ZERO,
        Vec3::new(0.0, 1.0, 0.0),
        Vec3::NEG_X,
        10.0,
    );

    assert!(radiance.x > 1.0, "red wall should receive direct light");
    assert!(radiance.y < 1e-6 && radiance.z < 1e-6);
}

#[test]
fn white_lambertian_surface_receives_ambient_without_receiver_color_baking() {
    let triangle = IrradianceTriangle {
        vertices: [
            Point3::new(-2.0, 0.0, -2.0),
            Point3::new(0.0, 0.0, 2.0),
            Point3::new(2.0, 0.0, -2.0),
        ],
        normals: [Vec3::Y; 3],
        uvs: [Vec2::ZERO; 3],
        colors: [Vec4::ONE; 3],
        material: 0,
        node_index: 0,
    };
    let mut triangles = vec![triangle];
    let bvh = Bvh::build(&mut triangles);
    let materials = [TransportMaterial {
        base_color_factor: Vec4::new(0.5, 0.5, 0.5, 1.0),
        metallic_factor: 0.0,
        ..Default::default()
    }];
    let ambient = Vec3::splat(std::f32::consts::PI);
    let radiance = trace_radiance(
        &bvh,
        &triangles,
        &materials,
        &[],
        &DirectionalBakeLight {
            color_rgba: [0.0; 4],
            rotation_xyzw: [0.0, 0.0, 0.0, 1.0],
            illuminance: 0.0,
        },
        ambient,
        Vec3::new(0.0, 0.25, 0.0),
        Vec3::NEG_Y,
        10.0,
    );
    // The receiver albedo participates exactly once in the Lambertian
    // response; it is not pre-multiplied into the ambient contract twice.
    let expected = Vec3::from_array(bevyout_core::lighting::srgb_to_linear_rgb([0.5, 0.5, 0.5]));
    assert!((radiance.x - expected.x).abs() < 1e-5);
    assert_eq!(radiance, expected);
}

#[test]
fn ambient_is_visible_when_a_probe_ray_escapes_the_scene() {
    let mut triangles = Vec::<IrradianceTriangle>::new();
    let bvh = Bvh::build(&mut triangles);
    let radiance = trace_radiance(
        &bvh,
        &triangles,
        &[TransportMaterial::default()],
        &[],
        &DirectionalBakeLight {
            color_rgba: [0.0; 4],
            rotation_xyzw: [0.0, 0.0, 0.0, 1.0],
            illuminance: 0.0,
        },
        Vec3::splat(std::f32::consts::PI),
        Vec3::ZERO,
        Vec3::Y,
        10.0,
    );
    assert_eq!(radiance, Vec3::ONE);
}

#[test]
fn authored_environment_map_adds_directional_irradiance_and_escape_radiance() {
    let map = EnvironmentMap::from_pixels(4, 2, vec![[2.0, 1.0, 0.5]; 8]).unwrap();
    let mut triangles = Vec::<IrradianceTriangle>::new();
    let bvh = Bvh::build(&mut triangles);
    let directional = DirectionalBakeLight {
        color_rgba: [0.0; 4],
        rotation_xyzw: [0.0, 0.0, 0.0, 1.0],
        illuminance: 0.0,
    };
    let irradiance = direct_irradiance_with_environment(
        &bvh,
        &triangles,
        &[TransportMaterial::default()],
        &[],
        &directional,
        Vec3::ZERO,
        Some(&map),
        Vec3::ZERO,
        Vec3::Y,
    );
    let expected_irradiance = Vec3::new(2.0, 1.0, 0.5) * std::f32::consts::PI;
    assert!((irradiance - expected_irradiance).abs().max_element() < 1e-5);

    let escape = trace_radiance_with_emissive_and_environment(
        &bvh,
        &triangles,
        &[TransportMaterial::default()],
        &[],
        &directional,
        &EmissiveSampler::default(),
        Vec3::ZERO,
        Some(&map),
        Vec3::ZERO,
        Vec3::X,
        10.0,
    );
    assert_eq!(escape, Vec3::new(2.0, 1.0, 0.5));
}

#[test]
fn surface_irradiance_adds_deterministic_emissive_one_bounce() {
    fn triangle(
        a: [f32; 3],
        b: [f32; 3],
        c: [f32; 3],
        normal: Vec3,
        material: usize,
    ) -> IrradianceTriangle {
        IrradianceTriangle {
            vertices: [Point3::from(a), Point3::from(b), Point3::from(c)],
            normals: [normal; 3],
            uvs: [Vec2::ZERO; 3],
            colors: [Vec4::ONE; 3],
            material,
            node_index: 0,
        }
    }

    let mut triangles = vec![
        triangle(
            [-2.0, 0.0, -2.0],
            [0.0, 0.0, 2.0],
            [2.0, 0.0, -2.0],
            Vec3::Y,
            0,
        ),
        triangle(
            [-2.0, 1.0, -2.0],
            [2.0, 1.0, -2.0],
            [0.0, 1.0, 2.0],
            Vec3::NEG_Y,
            1,
        ),
    ];
    let bvh = Bvh::build(&mut triangles);
    let materials = [
        TransportMaterial {
            base_color_factor: Vec4::ONE,
            metallic_factor: 0.0,
            ..Default::default()
        },
        TransportMaterial {
            emissive_factor: Vec3::splat(100.0),
            ..Default::default()
        },
    ];
    let directional = DirectionalBakeLight {
        color_rgba: [0.0; 4],
        rotation_xyzw: [0.0, 0.0, 0.0, 1.0],
        illuminance: 0.0,
    };
    let first = surface_irradiance_with_bounces(
        &bvh,
        &triangles,
        &materials,
        &[],
        &directional,
        Vec3::ZERO,
        42,
        7,
        64,
        1,
        Vec3::ZERO,
        Vec3::Y,
    );
    let second = surface_irradiance_with_bounces(
        &bvh,
        &triangles,
        &materials,
        &[],
        &directional,
        Vec3::ZERO,
        42,
        7,
        64,
        1,
        Vec3::ZERO,
        Vec3::Y,
    );
    let emitters = EmissiveSampler::new(&triangles, &materials);
    let explicit = surface_irradiance_with_emissive(
        &bvh,
        &triangles,
        &materials,
        &[],
        &directional,
        &emitters,
        Vec3::ZERO,
        42,
        7,
        0,
        1,
        Vec3::ZERO,
        Vec3::Y,
    );

    assert_eq!(first, second);
    assert!(
        first.max_element() > 0.0,
        "expected emissive indirect light"
    );
    assert!(
        explicit.max_element() > 0.0,
        "explicit area-light sampling should illuminate without cosine hits"
    );
    let disabled = surface_irradiance_with_emissive(
        &bvh,
        &triangles,
        &materials,
        &[],
        &directional,
        &emitters,
        Vec3::ZERO,
        42,
        7,
        0,
        0,
        Vec3::ZERO,
        Vec3::Y,
    );
    assert_eq!(disabled, Vec3::ZERO);
}

#[test]
fn surface_irradiance_reaches_an_emissive_panel_through_two_bounces() {
    fn plane_x(x: f32, normal: Vec3, material: usize) -> IrradianceTriangle {
        IrradianceTriangle {
            vertices: [
                Point3::new(x, -10.0, -10.0),
                Point3::new(x, 10.0, -10.0),
                Point3::new(x, 0.0, 10.0),
            ],
            normals: [normal; 3],
            uvs: [Vec2::ZERO; 3],
            colors: [Vec4::ONE; 3],
            material,
            node_index: 0,
        }
    }

    let mut triangles = vec![plane_x(1.0, Vec3::NEG_X, 0), plane_x(-1.0, Vec3::X, 1)];
    let bvh = Bvh::build(&mut triangles);
    let materials = [
        TransportMaterial {
            base_color_factor: Vec4::splat(0.8),
            metallic_factor: 0.0,
            ..Default::default()
        },
        TransportMaterial {
            emissive_factor: Vec3::new(100.0, 0.0, 0.0),
            ..Default::default()
        },
    ];
    let directional = DirectionalBakeLight {
        color_rgba: [0.0; 4],
        rotation_xyzw: [0.0, 0.0, 0.0, 1.0],
        illuminance: 0.0,
    };
    let emitters = EmissiveSampler::new(&triangles, &materials);
    let one_bounce = surface_irradiance_with_emissive(
        &bvh,
        &triangles,
        &materials,
        &[],
        &directional,
        &emitters,
        Vec3::ZERO,
        7,
        11,
        64,
        1,
        Vec3::ZERO,
        Vec3::X,
    );
    let two_bounces = surface_irradiance_with_emissive(
        &bvh,
        &triangles,
        &materials,
        &[],
        &directional,
        &emitters,
        Vec3::ZERO,
        7,
        11,
        64,
        2,
        Vec3::ZERO,
        Vec3::X,
    );

    assert!(
        one_bounce.x > 0.0,
        "explicit area-light sampling should reach the first diffuse hit"
    );
    assert!(
        two_bounces.x > 0.0,
        "expected multi-bounce emissive transfer"
    );
    assert!(two_bounces.y < 1e-6 && two_bounces.z < 1e-6);
}

#[test]
fn surface_and_volume_paths_share_static_diffuse_transport() {
    let triangle = IrradianceTriangle {
        vertices: [
            Point3::new(-100.0, 0.0, -100.0),
            Point3::new(100.0, 0.0, -100.0),
            Point3::new(0.0, 0.0, 100.0),
        ],
        normals: [Vec3::Y; 3],
        uvs: [Vec2::ZERO; 3],
        colors: [Vec4::ONE; 3],
        material: 0,
        node_index: 0,
    };
    let mut triangles = vec![triangle];
    let bvh = Bvh::build(&mut triangles);
    let materials = [TransportMaterial {
        base_color_factor: Vec4::splat(0.7),
        metallic_factor: 0.0,
        ..Default::default()
    }];
    let lights = [JobLight {
        translation: [0.0, 4.0, 0.0],
        rotation_xyzw: Quat::IDENTITY.to_array(),
        color_rgba: [1.0; 4],
        radius: 10.0,
        intensity_lumens: bevyout_core::lighting::AUTHORED_LIGHTING_SCALE,
        kind: "point".into(),
        flags: 0,
        spot_fov_radians: 0.0,
        spot_falloff_exponent: 0.0,
    }];
    let directional = DirectionalBakeLight {
        color_rgba: [0.0; 4],
        rotation_xyzw: Quat::IDENTITY.to_array(),
        illuminance: 0.0,
    };
    let emitters = EmissiveSampler::new(&triangles, &materials);
    let surface_position = Vec3::ZERO;
    let surface_irradiance = surface_irradiance_with_emissive(
        &bvh,
        &triangles,
        &materials,
        &lights,
        &directional,
        &emitters,
        Vec3::ZERO,
        0x0517_a71c,
        0,
        1,
        0,
        surface_position,
        Vec3::Y,
    );
    let volume_hit_radiance = trace_radiance_with_emissive(
        &bvh,
        &triangles,
        &materials,
        &lights,
        &directional,
        &emitters,
        Vec3::ZERO,
        surface_position + Vec3::Y * 0.25,
        Vec3::NEG_Y,
        100.0,
    );
    let base_color = bevyout_core::lighting::srgb_to_linear_rgb([0.7; 3]);
    let expected_volume_radiance =
        surface_irradiance * Vec3::from_array(base_color) / std::f32::consts::PI;
    assert!(surface_irradiance.max_element() > 0.0);
    let error = (volume_hit_radiance - expected_volume_radiance)
        .abs()
        .max_element();
    assert!(error < 1e-5, "shared transport paths diverged by {error}");
}

#[test]
fn emissive_distribution_and_mis_weights_are_deterministic() {
    let triangle = |material| IrradianceTriangle {
        vertices: [
            Point3::new(-1.0, 0.0, -1.0),
            Point3::new(1.0, 0.0, -1.0),
            Point3::new(0.0, 0.0, 1.0),
        ],
        normals: [Vec3::Y; 3],
        uvs: [Vec2::ZERO; 3],
        colors: [Vec4::ONE; 3],
        material,
        node_index: 0,
    };
    let triangles = vec![triangle(0), triangle(1)];
    let materials = [
        TransportMaterial::default(),
        TransportMaterial {
            emissive_factor: Vec3::splat(10.0),
            ..Default::default()
        },
    ];
    let emitters = EmissiveSampler::new(&triangles, &materials);
    assert_eq!(emitters.entries.len(), 1);
    assert_eq!(emitters.entries[0].triangle_index, 1);
    assert_eq!(emitters.selection_probability(1), 1.0);
    assert_eq!(
        emitters.select(0.1).triangle_index,
        emitters.select(0.9).triangle_index
    );
    assert!((power_heuristic(1.0, 1.0) - 0.5).abs() < 1e-6);
    assert!(power_heuristic(10.0, 1.0) > 0.98);
    assert!(power_heuristic(1.0, 10.0) < 0.02);
}

#[test]
fn ray_offsets_scale_with_world_position_and_remain_bounded() {
    assert_eq!(ray_epsilon(Vec3::ZERO), RAY_EPSILON);
    assert_eq!(ray_epsilon(Vec3::splat(16.0)), RAY_EPSILON);
    assert!(ray_epsilon(Vec3::splat(10_000.0)) > RAY_EPSILON);
    assert_eq!(ray_epsilon(Vec3::splat(1_000_000.0)), MAX_RAY_OFFSET);
    assert_eq!(ray_epsilon(Vec3::splat(f32::NAN)), RAY_EPSILON);
}

#[test]
fn russian_roulette_is_deterministic_and_starts_after_three_bounces() {
    let white = Vec3::ONE;
    assert_eq!(russian_roulette_survival(0, white, 1.0), 1.0);
    assert_eq!(russian_roulette_survival(2, white, 1.0), 1.0);
    let survival = russian_roulette_survival(3, Vec3::splat(0.4), 1.0);
    assert!((MIN_RUSSIAN_ROULETTE_SURVIVAL..=MAX_RUSSIAN_ROULETTE_SURVIVAL).contains(&survival));
    assert_eq!(
        sample_uniform_1d(0x1234, 77, 3),
        sample_uniform_1d(0x1234, 77, 3)
    );
    assert_ne!(
        sample_uniform_1d(0x1234, 77, 3),
        sample_uniform_1d(0x1234, 78, 3)
    );
}

#[test]
fn scaled_ray_bias_does_not_skip_a_close_blocker() {
    fn floor(y: f32) -> IrradianceTriangle {
        IrradianceTriangle {
            vertices: [
                Point3::new(98.0, y, -2.0),
                Point3::new(102.0, y, -2.0),
                Point3::new(98.0, y, 2.0),
            ],
            normals: [Vec3::Y; 3],
            uvs: [Vec2::ZERO; 3],
            colors: [Vec4::ONE; 3],
            material: 0,
            node_index: 0,
        }
    }

    let mut triangles = vec![floor(0.0), floor(0.01)];
    let bvh = Bvh::build(&mut triangles);
    let visibility = direct_irradiance(
        &bvh,
        &triangles,
        &[TransportMaterial::default()],
        &[JobLight {
            translation: [100.0, 1.0, 0.0],
            rotation_xyzw: [0.0, 0.0, 0.0, 1.0],
            color_rgba: [1.0; 4],
            radius: 4.0,
            intensity_lumens: bevyout_core::lighting::AUTHORED_LIGHTING_SCALE,
            kind: "point".into(),
            flags: 0,
            spot_fov_radians: 0.0,
            spot_falloff_exponent: 0.0,
        }],
        &DirectionalBakeLight {
            color_rgba: [0.0; 4],
            rotation_xyzw: [0.0, 0.0, 0.0, 1.0],
            illuminance: 0.0,
        },
        Vec3::ZERO,
        Vec3::new(100.0, 0.0, 0.0),
        Vec3::Y,
    );

    assert_eq!(visibility, Vec3::ZERO);
}

#[test]
fn point_light_shadow_distance_starts_at_shifted_origin() {
    let position = Vec3::new(100.0, 0.0, 0.0);
    let light_position = Vec3::new(100.866_03, 0.5, 0.0);
    let light_direction = (light_position - position).normalize();
    let shadow_origin = position + Vec3::Y * ray_epsilon(position);
    let blocker_center = shadow_origin + light_direction * 0.9985;
    let tangent = Vec3::new(-light_direction.y, light_direction.x, 0.0);

    let receiver = IrradianceTriangle {
        vertices: [
            Point3::new(98.0, 0.0, -2.0),
            Point3::new(102.0, 0.0, -2.0),
            Point3::new(100.0, 0.0, 2.0),
        ],
        normals: [Vec3::Y; 3],
        uvs: [Vec2::ZERO; 3],
        colors: [Vec4::ONE; 3],
        material: 0,
        node_index: 0,
    };
    let blocker = IrradianceTriangle {
        vertices: [
            point3(blocker_center + tangent * 2.0 + Vec3::Z * 2.0),
            point3(blocker_center - tangent * 2.0 + Vec3::Z * 2.0),
            point3(blocker_center - tangent * 2.0 - Vec3::Z * 2.0),
        ],
        normals: [Vec3::Y; 3],
        uvs: [Vec2::ZERO; 3],
        colors: [Vec4::ONE; 3],
        material: 0,
        node_index: 0,
    };
    let light = JobLight {
        translation: light_position.to_array(),
        rotation_xyzw: [0.0, 0.0, 0.0, 1.0],
        color_rgba: [1.0; 4],
        radius: 4.0,
        intensity_lumens: bevyout_core::lighting::AUTHORED_LIGHTING_SCALE,
        kind: "point".into(),
        flags: 0,
        spot_fov_radians: 0.0,
        spot_falloff_exponent: 0.0,
    };
    let directional = DirectionalBakeLight {
        color_rgba: [0.0; 4],
        rotation_xyzw: [0.0, 0.0, 0.0, 1.0],
        illuminance: 0.0,
    };
    let sample = |triangles: &mut Vec<IrradianceTriangle>| {
        let bvh = Bvh::build(triangles);
        direct_irradiance(
            &bvh,
            triangles,
            &[TransportMaterial::default()],
            std::slice::from_ref(&light),
            &directional,
            Vec3::ZERO,
            position,
            Vec3::Y,
        )
    };

    let visible = sample(&mut vec![receiver.clone()]);
    let blocked = sample(&mut vec![receiver, blocker]);

    assert!(visible.max_element() > 0.0);
    assert_eq!(blocked, Vec3::ZERO);
}
