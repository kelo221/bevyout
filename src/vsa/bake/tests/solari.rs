use super::*;

#[test]
fn proxy_uses_the_pinned_solari_attribute_contract() {
    let primitive = ComposedPrimitive {
        name: "fixture".into(),
        primitive_key: "fixture".into(),
        reference_form_ids: vec![1],
        material: 0,
        positions: vec![
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
        ],
        normals: vec![Vec3::Z; 3],
        uvs: vec![[0.0, 0.0].into(), [1.0, 0.0].into(), [0.0, 1.0].into()],
        colors: vec![[1.0; 4].into(); 3],
        transport_colors: vec![[1.0; 4].into(); 3],
        indices: vec![0, 1, 2],
        uv1: vec![[0.0, 0.0].into(); 3],
        uv1_chart_ids: vec![0; 3],
        lightmap_texels_per_meter: 16.0,
        lightmap_dimensions: [4, 4],
        lightmap_binding_id: Some(1),
    };
    let mesh = build_proxy_mesh(&primitive).unwrap();
    assert!(mesh.enable_raytracing);
    assert!(mesh.attribute(Mesh::ATTRIBUTE_POSITION).is_some());
    assert!(mesh.attribute(Mesh::ATTRIBUTE_NORMAL).is_some());
    assert!(mesh.attribute(Mesh::ATTRIBUTE_UV_0).is_some());
    assert!(mesh.attribute(Mesh::ATTRIBUTE_TANGENT).is_some());
    assert!(mesh.attribute(Mesh::ATTRIBUTE_UV_1).is_none());
    assert!(matches!(mesh.indices(), Some(Indices::U32(indices)) if indices == &[0, 1, 2]));
}

#[test]
fn proxy_material_carries_explicit_side_table_identity() {
    let mut images = Assets::<Image>::default();
    let material = solari_material(&TransportMaterial::default(), 37, &mut images);
    assert_eq!(material.reflectance, 37.0);
}

#[test]
fn bake_request_preserves_shared_texel_light_and_readback_contract() {
    let texels = vec![SolariBakeTexel {
        position: [1.0, 2.0, 3.0],
        normal: [0.0, 1.0, 0.0],
        spatial_index: 0,
    }];
    let lights = vec![SolariBakeLight {
        position: [1.0, 4.0, 3.0],
        color: [2.0, 1.0, 0.5],
        range: 8.0,
        direction: [0.0, 0.0, -1.0],
        outer_cosine: -1.0,
        inner_cosine: -1.0,
        falloff_exponent: 0.0,
    }];
    let (request, readback) = SolariBakeRequest::new(texels.clone(), lights.clone(), 0, 7);
    assert_eq!(request.texels, texels);
    assert_eq!(request.lights, lights);
    assert_eq!(request.sample_count, 1);
    assert_eq!(request.bounce_count, 0);
    assert!(request.environment.is_none());
    assert_eq!(request.revision, 7);
    assert!(readback.lock().unwrap().is_none());
    assert_eq!(f32_bytes(&[1.0, -2.0]), vec![0, 0, 128, 63, 0, 0, 0, 192]);
    assert_eq!(u32_bytes(&[1, 2]), vec![1, 0, 0, 0, 2, 0, 0, 0]);
}

#[test]
fn alpha_side_table_preserves_mask_mode_and_factor_in_primitive_order() {
    use crate::vsa::bake::rust_scene::{AlphaMode, TransportMaterial};
    use bevy::math::Vec4;

    let primitive = ComposedPrimitive {
        name: "masked".into(),
        primitive_key: "masked".into(),
        reference_form_ids: vec![1],
        material: 0,
        positions: vec![Vec3::ZERO, Vec3::X, Vec3::Y],
        normals: vec![Vec3::Z; 3],
        uvs: vec![[0.0, 0.0].into(), [1.0, 0.0].into(), [0.0, 1.0].into()],
        colors: vec![[1.0; 4].into(); 3],
        transport_colors: vec![[1.0; 4].into(); 3],
        indices: vec![0, 1, 2],
        uv1: vec![[0.0, 0.0].into(); 3],
        uv1_chart_ids: vec![0; 3],
        lightmap_texels_per_meter: 1.0,
        lightmap_dimensions: [1, 1],
        lightmap_binding_id: Some(1),
    };
    let material = TransportMaterial {
        base_color_factor: Vec4::new(1.0, 1.0, 1.0, 0.25),
        alpha_mode: AlphaMode::Mask,
        alpha_cutoff: 0.6,
        ..TransportMaterial::default()
    };

    let alpha_scene = build_alpha_scene(&[primitive], &[material]).unwrap();
    assert_eq!(alpha_scene.records.len(), 1);
    assert_eq!(
        alpha_scene.records[0].data_offset_width_height_mode,
        [0, 0, 0, 1]
    );
    assert_eq!(
        alpha_scene.records[0].base_alpha_cutoff_wrap,
        [0.25, 0.6, 0.0, 0.0]
    );
    assert_eq!(alpha_scene.records[0].flags, [0, 0, 0, 0]);
    assert!(alpha_scene.texels.is_empty());
    assert_eq!(alpha_scene.vertex_records.len(), 1);
    assert_eq!(
        alpha_scene.vertex_records[0],
        SolariBakeVertexRecord {
            color_offset: 0,
            index_offset: 0,
            position_offset: 0,
            vertex_count: 3,
            index_count: 3,
        }
    );
    assert_eq!(alpha_scene.vertex_colors.len(), 3);
    assert_eq!(alpha_scene.vertex_indices.as_slice(), &[0, 1, 2]);

    let blend_scene = build_alpha_scene(
        &[ComposedPrimitive {
            name: "blended".into(),
            primitive_key: "blended".into(),
            reference_form_ids: vec![2],
            material: 0,
            positions: vec![Vec3::ZERO, Vec3::X, Vec3::Y],
            normals: vec![Vec3::Z; 3],
            uvs: vec![[0.0, 0.0].into(), [1.0, 0.0].into(), [0.0, 1.0].into()],
            colors: vec![[1.0; 4].into(); 3],
            transport_colors: vec![[1.0; 4].into(); 3],
            indices: vec![0, 1, 2],
            uv1: vec![[0.0, 0.0].into(); 3],
            uv1_chart_ids: vec![0; 3],
            lightmap_texels_per_meter: 1.0,
            lightmap_dimensions: [1, 1],
            lightmap_binding_id: Some(2),
        }],
        &[TransportMaterial {
            base_color_factor: Vec4::new(1.0, 1.0, 1.0, 0.5),
            alpha_mode: AlphaMode::Blend,
            ..TransportMaterial::default()
        }],
    )
    .unwrap();
    assert_eq!(
        blend_scene.records[0].data_offset_width_height_mode,
        [0, 0, 0, 2]
    );
}

#[test]
fn bake_shader_keeps_solari_scene_and_bevyout_bindings_separate() {
    let shader = include_str!("../backend/solari_bake.wgsl");
    assert!(shader.contains("enable wgpu_ray_query;"));
    assert!(shader.contains("bevy_solari::scene_bindings"));
    assert!(shader.contains("@group(1) @binding(0) var<storage, read> texels"));
    assert!(shader.contains("@group(1) @binding(2) var<storage, read_write> output"));
    assert!(shader.contains("@group(1) @binding(11) var<storage, read> vertex_positions"));
    assert!(shader.contains("@group(1) @binding(12) var<storage, read> environment_texels"));
    assert!(shader.contains("@group(1) @binding(14) var<storage, read> emissive_triangles"));
    assert!(shader.contains("direction_and_outer_cosine"));
    assert!(shader.contains("resolve_ray_hit_full"));
    assert!(shader.contains("hit.material.base_color"));
    assert!(shader.contains("let bounce_count = min(params[3], MAX_BOUNCE_COUNT)"));
    assert!(shader.contains("path_throughput *= hit_diffuse"));
    assert!(shader.contains("vertex_color_for_hit"));
    assert!(shader.contains("* vertex_color.xyz"));
    assert!(shader.contains("alpha_materials"));
    assert!(shader.contains("hit_opacity"));
    assert!(shader.contains("ray_visibility"));
    assert!(shader.contains("emissive_irradiance"));
    assert!(shader.contains("side_table_index"));
    assert!(shader.contains("dot(geometric_normal, -ray_direction)"));
    assert!(shader.contains("cdf_residual"));
    assert!(shader.contains("RAY_FLAG_NONE"));
    assert!(!shader.contains("random_emissive_light_pdf"));
    assert!(!shader.contains("RAY_FLAG_TERMINATE_ON_FIRST_HIT"));
    assert!(shader.contains("environment_radiance"));
    assert!(shader.contains("@compute @workgroup_size(64, 1, 1)"));
    assert!(shader.contains("trace_ray("));
}

#[test]
#[ignore = "requires a compatible hardware ray-query adapter"]
fn headless_solari_alpha_mask_skips_transparent_occluders() {
    use crate::vsa::bake::rust_irradiance::{
        DirectionalBakeLight, IrradianceTriangle, direct_irradiance,
    };
    use crate::vsa::bake::rust_scene::{AlphaMode, TransportMaterial};
    use bevy::math::{Vec2, Vec4};
    use bvh::bvh::Bvh;
    use nalgebra::Point3;

    let plane = |name: &str, z: f32, material: usize| ComposedPrimitive {
        name: name.into(),
        primitive_key: name.into(),
        reference_form_ids: vec![material as u32 + 1],
        material,
        positions: vec![
            Vec3::new(-2.0, -2.0, z),
            Vec3::new(2.0, -2.0, z),
            Vec3::new(0.0, 2.0, z),
        ],
        normals: vec![Vec3::Z; 3],
        uvs: vec![[0.0, 0.0].into(), [1.0, 0.0].into(), [0.5, 1.0].into()],
        colors: vec![[1.0; 4].into(); 3],
        transport_colors: vec![[1.0; 4].into(); 3],
        indices: vec![0, 1, 2],
        uv1: vec![[0.0, 0.0].into(), [1.0, 0.0].into(), [0.5, 1.0].into()],
        uv1_chart_ids: vec![0; 3],
        lightmap_texels_per_meter: 1.0,
        lightmap_dimensions: [1, 1],
        lightmap_binding_id: Some(material as u32 + 1),
    };
    let primitives = vec![plane("receiver", 0.0, 0), plane("mask", 1.0, 1)];
    let directional = SolariBakeDirectionalLight {
        direction: Vec3::Z.to_array(),
        color: [1.0; 3],
        illuminance: 2.0,
    };
    let cpu_directional = DirectionalBakeLight {
        color_rgba: [1.0; 4],
        rotation_xyzw: [0.0, 0.0, 0.0, 1.0],
        illuminance: 2.0,
    };

    let bake = |mask_alpha: f32| {
        let materials = vec![
            TransportMaterial::default(),
            TransportMaterial {
                base_color_factor: Vec4::new(1.0, 1.0, 1.0, mask_alpha),
                alpha_mode: AlphaMode::Mask,
                alpha_cutoff: 0.5,
                double_sided: true,
                ..TransportMaterial::default()
            },
        ];
        let mut session = SolariBakeSession::new(&primitives, &materials)
            .expect("headless Solari alpha fixture should create a scene");
        let result = session
            .bake_texels(
                vec![SolariBakeTexel {
                    position: [0.0, 0.0, 0.0],
                    normal: Vec3::Z.to_array(),
                    spatial_index: 0,
                }],
                &[],
                [0.0; 3],
                directional,
                1,
                0,
                31,
                31,
            )
            .expect("headless Solari alpha bake should return a readback");

        let mut triangles = vec![
            IrradianceTriangle {
                vertices: [
                    Point3::new(-2.0, -2.0, 0.0),
                    Point3::new(2.0, -2.0, 0.0),
                    Point3::new(0.0, 2.0, 0.0),
                ],
                normals: [Vec3::Z; 3],
                uvs: [Vec2::ZERO; 3],
                colors: [Vec4::ONE; 3],
                material: 0,
                node_index: 0,
            },
            IrradianceTriangle {
                vertices: [
                    Point3::new(-2.0, -2.0, 1.0),
                    Point3::new(2.0, -2.0, 1.0),
                    Point3::new(0.0, 2.0, 1.0),
                ],
                normals: [Vec3::Z; 3],
                uvs: [Vec2::ZERO; 3],
                colors: [Vec4::ONE; 3],
                material: 1,
                node_index: 1,
            },
        ];
        let bvh = Bvh::build(&mut triangles);
        let expected = direct_irradiance(
            &bvh,
            &triangles,
            &materials,
            &[],
            &cpu_directional,
            Vec3::ZERO,
            Vec3::ZERO,
            Vec3::Z,
        );
        (
            Vec3::from_array([result[0][0], result[0][1], result[0][2]]),
            expected,
        )
    };

    let (transparent, transparent_expected) = bake(0.0);
    let (opaque, opaque_expected) = bake(1.0);
    assert!((transparent - transparent_expected).abs().max_element() <= 1.0e-3);
    assert!((opaque - opaque_expected).abs().max_element() <= 1.0e-3);
    assert!(transparent.max_element() > 0.0);
    assert!(opaque.max_element() <= 1.0e-3);
}

#[test]
#[ignore = "requires a compatible hardware ray-query adapter"]
fn headless_solari_blended_alpha_matches_cpu_transmittance() {
    use crate::vsa::bake::rust_irradiance::{
        DirectionalBakeLight, IrradianceTriangle, direct_irradiance,
    };
    use crate::vsa::bake::rust_scene::{AlphaMode, TransportMaterial};
    use bevy::math::{Vec2, Vec4};
    use bvh::bvh::Bvh;
    use nalgebra::Point3;

    let plane = |name: &str, z: f32, material: usize| ComposedPrimitive {
        name: name.into(),
        primitive_key: name.into(),
        reference_form_ids: vec![material as u32 + 1],
        material,
        positions: vec![
            Vec3::new(-2.0, -2.0, z),
            Vec3::new(2.0, -2.0, z),
            Vec3::new(0.0, 2.0, z),
        ],
        normals: vec![Vec3::Z; 3],
        uvs: vec![[0.0, 0.0].into(), [1.0, 0.0].into(), [0.5, 1.0].into()],
        colors: vec![[1.0; 4].into(); 3],
        transport_colors: vec![[1.0; 4].into(); 3],
        indices: vec![0, 1, 2],
        uv1: vec![[0.0, 0.0].into(), [1.0, 0.0].into(), [0.5, 1.0].into()],
        uv1_chart_ids: vec![0; 3],
        lightmap_texels_per_meter: 1.0,
        lightmap_dimensions: [1, 1],
        lightmap_binding_id: Some(material as u32 + 1),
    };
    let primitives = vec![plane("receiver", 0.0, 0), plane("blend", 1.0, 1)];
    let directional = SolariBakeDirectionalLight {
        direction: Vec3::Z.to_array(),
        color: [1.0; 3],
        illuminance: 2.0,
    };
    let cpu_directional = DirectionalBakeLight {
        color_rgba: [1.0; 4],
        rotation_xyzw: [0.0, 0.0, 0.0, 1.0],
        illuminance: 2.0,
    };

    for alpha in [0.0, 0.5, 1.0] {
        let materials = vec![
            TransportMaterial::default(),
            TransportMaterial {
                base_color_factor: Vec4::new(1.0, 1.0, 1.0, alpha),
                alpha_mode: AlphaMode::Blend,
                double_sided: true,
                ..TransportMaterial::default()
            },
        ];
        let mut session = SolariBakeSession::new(&primitives, &materials)
            .expect("headless Solari blended-alpha fixture should create a scene");
        let result = session
            .bake_texels(
                vec![SolariBakeTexel {
                    position: [0.0, 0.0, 0.0],
                    normal: Vec3::Z.to_array(),
                    spatial_index: alpha.to_bits(),
                }],
                &[],
                [0.0; 3],
                directional,
                1,
                0,
                61 + alpha.to_bits() as u64,
                61 + alpha.to_bits(),
            )
            .expect("headless Solari blended-alpha bake should return a readback");

        let mut triangles = vec![
            IrradianceTriangle {
                vertices: [
                    Point3::new(-2.0, -2.0, 0.0),
                    Point3::new(2.0, -2.0, 0.0),
                    Point3::new(0.0, 2.0, 0.0),
                ],
                normals: [Vec3::Z; 3],
                uvs: [Vec2::ZERO; 3],
                colors: [Vec4::ONE; 3],
                material: 0,
                node_index: 0,
            },
            IrradianceTriangle {
                vertices: [
                    Point3::new(-2.0, -2.0, 1.0),
                    Point3::new(2.0, -2.0, 1.0),
                    Point3::new(0.0, 2.0, 1.0),
                ],
                normals: [Vec3::Z; 3],
                uvs: [Vec2::ZERO; 3],
                colors: [Vec4::ONE; 3],
                material: 1,
                node_index: 1,
            },
        ];
        let bvh = Bvh::build(&mut triangles);
        let expected = direct_irradiance(
            &bvh,
            &triangles,
            &materials,
            &[],
            &cpu_directional,
            Vec3::ZERO,
            Vec3::ZERO,
            Vec3::Z,
        );
        let actual = Vec3::from_array([result[0][0], result[0][1], result[0][2]]);
        assert!(
            (actual - expected).abs().max_element() <= 1.0e-3,
            "Solari/CPU blended-alpha mismatch alpha={alpha}: actual={actual:?}, expected={expected:?}"
        );
    }
}

#[test]
#[ignore = "requires a compatible hardware ray-query adapter"]
fn headless_solari_one_bounce_uses_authored_diffuse_material() {
    use crate::vsa::bake::rust_irradiance::{
        DirectionalBakeLight, IrradianceTriangle, surface_irradiance_with_bounces,
    };
    use crate::vsa::bake::rust_scene::TransportMaterial;
    use bevy::math::{Vec2, Vec4};
    use bvh::bvh::Bvh;
    use nalgebra::Point3;

    let receiver = ComposedPrimitive {
        name: "receiver".into(),
        primitive_key: "receiver".into(),
        reference_form_ids: vec![1],
        material: 0,
        positions: vec![
            Vec3::new(-0.05, -0.05, 0.0),
            Vec3::new(0.05, -0.05, 0.0),
            Vec3::new(0.0, 0.05, 0.0),
        ],
        normals: vec![Vec3::Z; 3],
        uvs: vec![[0.0, 0.0].into(), [1.0, 0.0].into(), [0.5, 1.0].into()],
        colors: vec![[1.0; 4].into(); 3],
        transport_colors: vec![[1.0; 4].into(); 3],
        indices: vec![0, 1, 2],
        uv1: vec![[0.0, 0.0].into(), [1.0, 0.0].into(), [0.5, 1.0].into()],
        uv1_chart_ids: vec![0; 3],
        lightmap_texels_per_meter: 4.0,
        lightmap_dimensions: [1, 1],
        lightmap_binding_id: Some(1),
    };
    let secondary = ComposedPrimitive {
        name: "secondary".into(),
        primitive_key: "secondary".into(),
        reference_form_ids: vec![2],
        material: 1,
        positions: vec![
            Vec3::new(-100.0, -100.0, 1.0),
            Vec3::new(100.0, -100.0, 1.0),
            Vec3::new(0.0, 100.0, 1.0),
        ],
        normals: vec![Vec3::NEG_Z; 3],
        uvs: vec![[0.0, 0.0].into(), [1.0, 0.0].into(), [0.5, 1.0].into()],
        colors: vec![[1.0; 4].into(); 3],
        transport_colors: vec![[1.0; 4].into(); 3],
        indices: vec![0, 1, 2],
        uv1: vec![[0.0, 0.0].into(), [1.0, 0.0].into(), [0.5, 1.0].into()],
        uv1_chart_ids: vec![0; 3],
        lightmap_texels_per_meter: 4.0,
        lightmap_dimensions: [1, 1],
        lightmap_binding_id: Some(2),
    };
    let diffuse = TransportMaterial {
        base_color_factor: Vec4::new(0.2, 0.4, 0.8, 1.0),
        metallic_factor: 0.0,
        double_sided: true,
        ..TransportMaterial::default()
    };
    let materials = vec![TransportMaterial::default(), diffuse.clone()];
    let primitives = vec![receiver, secondary];
    let zero_light = JobLight {
        translation: [0.0, 0.0, 0.0],
        rotation_xyzw: [0.0, 0.0, 0.0, 1.0],
        color_rgba: [0.0; 4],
        radius: 1.0,
        intensity_lumens: 0.0,
        kind: "point".into(),
        flags: 0,
        spot_fov_radians: 0.0,
        spot_falloff_exponent: 0.0,
    };
    let mut session = SolariBakeSession::new(&primitives, &materials)
        .expect("headless Solari scene should accept authored materials");
    let directional = SolariBakeDirectionalLight {
        direction: [0.0, 0.0, -1.0],
        color: [1.0; 3],
        illuminance: 2.0,
    };
    let actual = session
        .bake_texels(
            vec![SolariBakeTexel {
                position: [0.0, 0.0, 0.0],
                normal: Vec3::Z.to_array(),
                spatial_index: 0,
            }],
            std::slice::from_ref(&zero_light),
            [0.0; 3],
            directional,
            1,
            1,
            23,
            23,
        )
        .expect("headless Solari one-bounce bake should return a readback");

    let mut triangles = vec![
        IrradianceTriangle {
            vertices: [
                Point3::new(-0.05, -0.05, 0.0),
                Point3::new(0.05, -0.05, 0.0),
                Point3::new(0.0, 0.05, 0.0),
            ],
            normals: [Vec3::Z; 3],
            uvs: [Vec2::ZERO; 3],
            colors: [Vec4::ONE; 3],
            material: 0,
            node_index: 0,
        },
        IrradianceTriangle {
            vertices: [
                Point3::new(-100.0, -100.0, 1.0),
                Point3::new(100.0, -100.0, 1.0),
                Point3::new(0.0, 100.0, 1.0),
            ],
            normals: [Vec3::NEG_Z; 3],
            uvs: [Vec2::ZERO; 3],
            colors: [Vec4::ONE; 3],
            material: 1,
            node_index: 1,
        },
    ];
    let bvh = Bvh::build(&mut triangles);
    let expected = surface_irradiance_with_bounces(
        &bvh,
        &triangles,
        &materials,
        std::slice::from_ref(&zero_light),
        &DirectionalBakeLight {
            color_rgba: [1.0; 4],
            rotation_xyzw: [1.0, 0.0, 0.0, 0.0],
            illuminance: 2.0,
        },
        Vec3::ZERO,
        23,
        0,
        1,
        1,
        Vec3::ZERO,
        Vec3::Z,
    );
    let actual = Vec3::from_array([actual[0][0], actual[0][1], actual[0][2]]);
    assert!(
        (actual - expected).abs().max_element() <= 1.0e-3,
        "Solari/CPU one-bounce material mismatch: actual={actual:?}, expected={expected:?}"
    );
}

#[test]
#[ignore = "requires a compatible hardware ray-query adapter"]
fn headless_solari_two_bounces_accumulate_authored_diffuse_transport() {
    use crate::vsa::bake::rust_scene::TransportMaterial;
    use bevy::math::Vec4;

    let plane = |name: &str, z: f32, material: usize, transport_color: Vec4, extent: f32| {
        ComposedPrimitive {
            name: name.into(),
            primitive_key: name.into(),
            reference_form_ids: vec![material as u32 + 1],
            material,
            positions: vec![
                Vec3::new(-extent, -extent, z),
                Vec3::new(extent, -extent, z),
                Vec3::new(0.0, extent, z),
            ],
            normals: vec![Vec3::Z; 3],
            uvs: vec![[0.0, 0.0].into(), [1.0, 0.0].into(), [0.5, 1.0].into()],
            colors: vec![[1.0; 4].into(); 3],
            transport_colors: vec![transport_color; 3],
            indices: vec![0, 1, 2],
            uv1: vec![[0.0, 0.0].into(), [1.0, 0.0].into(), [0.5, 1.0].into()],
            uv1_chart_ids: vec![0; 3],
            lightmap_texels_per_meter: 1.0,
            lightmap_dimensions: [1, 1],
            lightmap_binding_id: Some(material as u32 + 1),
        }
    };
    let primitives = vec![
        plane("receiver", 0.0, 0, Vec4::ONE, 0.05),
        plane("secondary", 1.0, 1, Vec4::splat(0.5), 1000.0),
        // The corrected double-sided hit normal faces the incoming ray. The
        // next diffuse bounce therefore travels below the secondary plane.
        plane("tertiary", -1.0, 2, Vec4::splat(0.5), 1000.0),
    ];
    let half_albedo = TransportMaterial {
        base_color_factor: Vec4::new(0.5, 0.5, 0.5, 1.0),
        metallic_factor: 0.5,
        double_sided: true,
        ..TransportMaterial::default()
    };
    let materials = vec![
        TransportMaterial::default(),
        half_albedo.clone(),
        half_albedo,
    ];
    let mut session = SolariBakeSession::new(&primitives, &materials)
        .expect("headless Solari deeper-bounce fixture should create a scene");
    let texel = vec![SolariBakeTexel {
        position: [0.0, 0.0, 0.0],
        normal: Vec3::Z.to_array(),
        spatial_index: 0,
    }];
    let one_bounce = session
        .bake_texels(
            texel.clone(),
            &[],
            [1.0; 3],
            SolariBakeDirectionalLight::default(),
            1,
            1,
            41,
            41,
        )
        .expect("headless Solari one-bounce ambient bake should return a readback");
    let two_bounces = session
        .bake_texels(
            texel,
            &[],
            [1.0; 3],
            SolariBakeDirectionalLight::default(),
            1,
            2,
            42,
            42,
        )
        .expect("headless Solari two-bounce ambient bake should return a readback");

    let linear_albedo = 0.5 * 0.5 * 0.5;
    let expected_one = 1.0 + linear_albedo;
    let expected_two = expected_one + linear_albedo * linear_albedo;
    let one = Vec3::from_array([one_bounce[0][0], one_bounce[0][1], one_bounce[0][2]]);
    let two = Vec3::from_array([two_bounces[0][0], two_bounces[0][1], two_bounces[0][2]]);
    assert!(
        (one - Vec3::splat(expected_one)).abs().max_element() <= 1.0e-3,
        "one-bounce actual={one:?} expected={expected_one}"
    );
    assert!(
        (two - Vec3::splat(expected_two)).abs().max_element() <= 1.0e-3,
        "two-bounce actual={two:?} expected={expected_two}"
    );
    assert!(two.max_element() > one.max_element());
}

#[test]
#[ignore = "requires a compatible hardware ray-query adapter"]
fn headless_solari_emissive_mesh_contributes_to_bounce_transport() {
    use crate::vsa::bake::rust_scene::TransportMaterial;
    use bevy::math::Vec4;

    let plane = |name: &str, z: f32, normal: Vec3, material: usize| ComposedPrimitive {
        name: name.into(),
        primitive_key: name.into(),
        reference_form_ids: vec![material as u32 + 1],
        material,
        positions: vec![
            Vec3::new(-100.0, -100.0, z),
            Vec3::new(100.0, -100.0, z),
            Vec3::new(0.0, 100.0, z),
        ],
        normals: vec![normal; 3],
        uvs: vec![[0.0, 0.0].into(), [1.0, 0.0].into(), [0.5, 1.0].into()],
        colors: vec![[1.0; 4].into(); 3],
        transport_colors: vec![[1.0; 4].into(); 3],
        indices: vec![0, 1, 2],
        uv1: vec![[0.0, 0.0].into(), [1.0, 0.0].into(), [0.5, 1.0].into()],
        uv1_chart_ids: vec![0; 3],
        lightmap_texels_per_meter: 1.0,
        lightmap_dimensions: [1, 1],
        lightmap_binding_id: Some(material as u32 + 1),
    };
    let primitives = vec![
        plane("receiver", 0.0, Vec3::Z, 0),
        plane("emitter", 1.0, Vec3::NEG_Z, 1),
    ];
    let materials = vec![
        TransportMaterial::default(),
        TransportMaterial {
            base_color_factor: Vec4::ONE,
            emissive_factor: Vec3::new(0.8, 0.4, 0.2),
            metallic_factor: 0.0,
            double_sided: true,
            ..TransportMaterial::default()
        },
    ];
    let mut session = SolariBakeSession::new(&primitives, &materials)
        .expect("headless Solari emissive fixture should create a scene");
    let result = session
        .bake_texels(
            vec![SolariBakeTexel {
                position: [0.0, 0.0, 0.0],
                normal: Vec3::Z.to_array(),
                spatial_index: 0,
            }],
            &[],
            [0.0; 3],
            SolariBakeDirectionalLight::default(),
            1,
            1,
            51,
            51,
        )
        .expect("headless Solari emissive bake should return a readback");
    let value = Vec3::from_array([result[0][0], result[0][1], result[0][2]]);
    assert!(
        value.is_finite() && value.max_element() > 0.0,
        "emissive mesh did not reach the receiver: {value:?}"
    );
}

#[test]
#[ignore = "requires a compatible hardware ray-query adapter"]
fn headless_solari_constant_environment_matches_pi_irradiance() {
    use crate::vsa::bake::environment::EnvironmentMap;

    let primitive = ComposedPrimitive {
        name: "receiver".into(),
        primitive_key: "receiver".into(),
        reference_form_ids: vec![1],
        material: 0,
        positions: vec![
            Vec3::new(-100.0, -100.0, 0.0),
            Vec3::new(100.0, -100.0, 0.0),
            Vec3::new(0.0, 100.0, 0.0),
        ],
        normals: vec![Vec3::Z; 3],
        uvs: vec![[0.0, 0.0].into(), [1.0, 0.0].into(), [0.5, 1.0].into()],
        colors: vec![[1.0; 4].into(); 3],
        transport_colors: vec![[1.0; 4].into(); 3],
        indices: vec![0, 1, 2],
        uv1: vec![[0.0, 0.0].into(), [1.0, 0.0].into(), [0.5, 1.0].into()],
        uv1_chart_ids: vec![0; 3],
        lightmap_texels_per_meter: 1.0,
        lightmap_dimensions: [1, 1],
        lightmap_binding_id: Some(1),
    };
    let environment = EnvironmentMap::from_pixels(2, 1, vec![[1.0; 3], [1.0; 3]])
        .expect("constant equirectangular environment should be valid");
    let mut session = SolariBakeSession::new(
        std::slice::from_ref(&primitive),
        &[crate::vsa::bake::rust_scene::TransportMaterial::default()],
    )
    .expect("headless Solari environment fixture should create a scene");
    let texel = vec![SolariBakeTexel {
        position: [0.0, 0.0, 0.0],
        normal: Vec3::Z.to_array(),
        spatial_index: 0,
    }];
    let direct = session
        .bake_texels_with_environment(
            texel.clone(),
            &[],
            [0.0; 3],
            SolariBakeDirectionalLight::default(),
            1,
            0,
            Some(&environment),
            61,
            61,
        )
        .expect("constant environment direct bake should return a readback");
    let with_escape = session
        .bake_texels_with_environment(
            texel,
            &[],
            [0.0; 3],
            SolariBakeDirectionalLight::default(),
            1,
            1,
            Some(&environment),
            62,
            62,
        )
        .expect("constant environment indirect bake should return a readback");
    let direct_value = Vec3::from_array([direct[0][0], direct[0][1], direct[0][2]]);
    let escape_value = Vec3::from_array([with_escape[0][0], with_escape[0][1], with_escape[0][2]]);
    let expected_direct = std::f32::consts::PI;
    assert!(
        (direct_value - Vec3::splat(expected_direct))
            .abs()
            .max_element()
            <= 1.0e-3,
        "constant environment direct mismatch: actual={direct_value:?}, expected={expected_direct}"
    );
    assert!(
        (escape_value - Vec3::splat(expected_direct))
            .abs()
            .max_element()
            <= 1.0e-3,
        "constant environment escape mismatch: actual={escape_value:?}, expected={expected_direct}"
    );
}

#[test]
#[ignore = "requires a compatible hardware ray-query adapter"]
fn headless_solari_session_returns_direct_irradiance() {
    let primitive = ComposedPrimitive {
        name: "fixture".into(),
        primitive_key: "fixture".into(),
        reference_form_ids: vec![1],
        material: 0,
        positions: vec![
            Vec3::new(-1.0, -1.0, 0.0),
            Vec3::new(1.0, -1.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
        ],
        normals: vec![Vec3::Z; 3],
        uvs: vec![[0.0, 0.0].into(), [1.0, 0.0].into(), [0.5, 1.0].into()],
        colors: vec![[1.0; 4].into(); 3],
        transport_colors: vec![[1.0; 4].into(); 3],
        indices: vec![0, 1, 2],
        uv1: vec![[0.0, 0.0].into(), [1.0, 0.0].into(), [0.5, 1.0].into()],
        uv1_chart_ids: vec![0; 3],
        lightmap_texels_per_meter: 4.0,
        lightmap_dimensions: [4, 4],
        lightmap_binding_id: Some(1),
    };
    let light = JobLight {
        translation: [0.0, 0.0, 2.0],
        rotation_xyzw: [0.0, 0.0, 0.0, 1.0],
        color_rgba: [1.0, 1.0, 1.0, 1.0],
        radius: 8.0,
        intensity_lumens: 8_192.0,
        kind: "point".into(),
        flags: 0,
        spot_fov_radians: 0.0,
        spot_falloff_exponent: 0.0,
    };
    let result = bake_direct_texels(
        &[primitive],
        &[crate::vsa::bake::rust_scene::TransportMaterial::default()],
        vec![SolariBakeTexel {
            position: [0.0, 0.0, 0.0],
            normal: [0.0, 0.0, 1.0],
            spatial_index: 0,
        }],
        &[light],
        [0.0; 3],
        SolariBakeDirectionalLight::default(),
        1,
        17,
    )
    .expect("headless Solari bake session should return a readback");
    assert_eq!(result.len(), 1);
    assert!(result[0][0].is_finite() && result[0][0] > 0.0);
}

#[test]
#[ignore = "requires a compatible hardware ray-query adapter"]
fn headless_solari_direct_irradiance_matches_cpu_reference() {
    use crate::vsa::bake::rust_irradiance::{
        DirectionalBakeLight, IrradianceTriangle, direct_irradiance,
    };
    use crate::vsa::bake::rust_scene::TransportMaterial;
    use bevy::math::{Vec2, Vec4};
    use bvh::bvh::Bvh;
    use nalgebra::Point3;

    let primitive = ComposedPrimitive {
        name: "parity-fixture".into(),
        primitive_key: "parity-fixture".into(),
        reference_form_ids: vec![1],
        material: 0,
        positions: vec![
            Vec3::new(-1.0, -1.0, 0.0),
            Vec3::new(1.0, -1.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
        ],
        normals: vec![Vec3::Z; 3],
        uvs: vec![[0.0, 0.0].into(), [1.0, 0.0].into(), [0.5, 1.0].into()],
        colors: vec![[1.0; 4].into(); 3],
        transport_colors: vec![[1.0; 4].into(); 3],
        indices: vec![0, 1, 2],
        uv1: vec![[0.0, 0.0].into(), [1.0, 0.0].into(), [0.5, 1.0].into()],
        uv1_chart_ids: vec![0; 3],
        lightmap_texels_per_meter: 4.0,
        lightmap_dimensions: [4, 4],
        lightmap_binding_id: Some(1),
    };
    let sample_positions = [
        Vec3::new(-0.5, -0.5, 0.0),
        Vec3::new(0.5, -0.5, 0.0),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(-0.25, 0.25, 0.0),
        Vec3::new(0.25, 0.25, 0.0),
    ];
    let normal = Vec3::Z;
    let light = JobLight {
        translation: [0.0, 0.0, 2.0],
        rotation_xyzw: [0.0, 0.0, 0.0, 1.0],
        color_rgba: [1.0, 1.0, 1.0, 1.0],
        radius: 8.0,
        intensity_lumens: 8_192.0,
        kind: "point".into(),
        flags: 0,
        spot_fov_radians: 0.0,
        spot_falloff_exponent: 0.0,
    };
    let directional = SolariBakeDirectionalLight::default();
    let result = bake_direct_texels(
        std::slice::from_ref(&primitive),
        &[crate::vsa::bake::rust_scene::TransportMaterial::default()],
        sample_positions
            .iter()
            .map(|position| SolariBakeTexel {
                position: position.to_array(),
                normal: normal.to_array(),
                spatial_index: 0,
            })
            .collect(),
        std::slice::from_ref(&light),
        [0.0; 3],
        directional,
        1,
        18,
    )
    .expect("headless Solari bake session should return a readback");

    let mut triangles = vec![IrradianceTriangle {
        vertices: [
            Point3::new(-1.0, -1.0, 0.0),
            Point3::new(1.0, -1.0, 0.0),
            Point3::new(0.0, 1.0, 0.0),
        ],
        normals: [Vec3::Z; 3],
        uvs: [Vec2::ZERO; 3],
        colors: [Vec4::ONE; 3],
        material: 0,
        node_index: 0,
    }];
    let bvh = Bvh::build(&mut triangles);
    let zero_directional = DirectionalBakeLight {
        color_rgba: [0.0; 4],
        rotation_xyzw: [0.0, 0.0, 0.0, 1.0],
        illuminance: 0.0,
    };
    let mut maximum_error: f32 = 0.0;
    let mut mean_error: f32 = 0.0;
    for (position, actual_value) in sample_positions.iter().zip(&result) {
        let expected = direct_irradiance(
            &bvh,
            &triangles,
            &[TransportMaterial::default()],
            std::slice::from_ref(&light),
            &zero_directional,
            Vec3::ZERO,
            *position,
            normal,
        );
        let actual = Vec3::from_array([actual_value[0], actual_value[1], actual_value[2]]);
        let error = (actual - expected).abs().max_element();
        maximum_error = maximum_error.max(error);
        mean_error += error;
    }
    mean_error /= sample_positions.len() as f32;
    assert!(
        result
            .iter()
            .all(|value| value[..3].iter().all(|channel| channel.is_finite()))
            && maximum_error <= 1.0e-3,
        "Solari/CPU direct irradiance mismatch across {} texels: max_error={maximum_error}, mean_error={mean_error}",
        sample_positions.len()
    );

    let spot_light = JobLight {
        translation: [0.0, 0.0, 2.0],
        rotation_xyzw: [0.0, 0.0, 0.0, 1.0],
        color_rgba: [1.0, 1.0, 1.0, 1.0],
        radius: 8.0,
        intensity_lumens: 8_192.0,
        kind: "spot".into(),
        flags: 0,
        spot_fov_radians: std::f32::consts::FRAC_PI_2,
        spot_falloff_exponent: 2.0,
    };
    let spot_result = bake_direct_texels(
        std::slice::from_ref(&primitive),
        &[crate::vsa::bake::rust_scene::TransportMaterial::default()],
        sample_positions
            .iter()
            .map(|position| SolariBakeTexel {
                position: position.to_array(),
                normal: normal.to_array(),
                spatial_index: 0,
            })
            .collect(),
        std::slice::from_ref(&spot_light),
        [0.0; 3],
        directional,
        1,
        19,
    )
    .expect("headless Solari spotlight bake should return a readback");
    let mut spot_maximum_error: f32 = 0.0;
    let mut spot_mean_error: f32 = 0.0;
    for (position, actual_value) in sample_positions.iter().zip(&spot_result) {
        let expected = direct_irradiance(
            &bvh,
            &triangles,
            &[TransportMaterial::default()],
            std::slice::from_ref(&spot_light),
            &zero_directional,
            Vec3::ZERO,
            *position,
            normal,
        );
        let actual = Vec3::from_array([actual_value[0], actual_value[1], actual_value[2]]);
        let error = (actual - expected).abs().max_element();
        spot_maximum_error = spot_maximum_error.max(error);
        spot_mean_error += error;
    }
    spot_mean_error /= sample_positions.len() as f32;
    assert!(
        spot_result
            .iter()
            .all(|value| value[..3].iter().all(|channel| channel.is_finite()))
            && spot_maximum_error <= 1.0e-3,
        "Solari/CPU spotlight mismatch across {} texels: max_error={spot_maximum_error}, mean_error={spot_mean_error}",
        sample_positions.len()
    );
}
