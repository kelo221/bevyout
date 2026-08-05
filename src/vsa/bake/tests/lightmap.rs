use super::super::cache::TileCache;
use super::super::job::JobLight;
use super::super::ktx2::write_rgba16f;
use super::super::lightmap::{
    DenoiseFeature, LightmapDebugSettings, LightmapDenoiseSettings, LightmapPage,
    LightmapSamplingSettings, LightmapSamplingSummary, TileRecord, bake_direct_pages, barycentric,
    decode_tile_payload, dilate_chart_aware, encode_rgba16f, encode_tile_payload,
    pack_lightmap_pages, pixel_sample_weights, primitive_tile_fingerprint, write_variance_output,
};
use super::super::rust_irradiance::DirectionalBakeLight;
use super::super::rust_scene::{ComposedPrimitive, synthetic_lightmap_scene_for_test};
use crate::cli::progress::{ProgressMode, ProgressReporter};
use bevy::math::{Vec2, Vec3};
use half::f16;
use std::fs;
use std::io;

fn test_primitive() -> ComposedPrimitive {
    ComposedPrimitive {
        name: "test".into(),
        primitive_key: "primitive:test".into(),
        reference_form_ids: vec![1],
        material: 0,
        positions: vec![Vec3::new(-1.0, -1.0, -1.0), Vec3::new(1.0, 1.0, 1.0)],
        normals: vec![Vec3::Z; 2],
        uvs: vec![Vec2::ZERO; 2],
        colors: vec![bevy::math::Vec4::ONE; 2],
        transport_colors: vec![bevy::math::Vec4::ONE; 2],
        indices: vec![0, 1, 1],
        uv1: Vec::new(),
        uv1_chart_ids: Vec::new(),
        lightmap_texels_per_meter: 16.0,
        lightmap_dimensions: [4, 4],
        lightmap_binding_id: None,
    }
}

fn test_light(translation: [f32; 3], color_rgba: [f32; 4]) -> JobLight {
    JobLight {
        translation,
        rotation_xyzw: [0.0, 0.0, 0.0, 1.0],
        color_rgba,
        radius: 2.0,
        intensity_lumens: 100.0,
        kind: "point".into(),
        flags: 0,
        spot_fov_radians: 0.0,
        spot_falloff_exponent: 0.0,
    }
}

#[test]
fn miniature_surface_bake_produces_finite_ktx2() {
    let root = std::env::temp_dir().join(format!(
        "bevyout-miniature-lightmap-test-{}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&root);
    let output_dir = root.join("baked");
    let cache_dir = root.join("cache");
    fs::create_dir_all(&output_dir).unwrap();

    let mut scene = synthetic_lightmap_scene_for_test();
    let glb_path = output_dir.join("scene.glb");
    scene.write_glb(&glb_path).unwrap();
    let gltf = gltf::Gltf::open(&glb_path).unwrap();
    let primitive = gltf
        .document
        .meshes()
        .flat_map(|mesh| mesh.primitives())
        .next()
        .expect("synthetic GLB should contain one primitive");
    assert!(
        primitive.get(&gltf::mesh::Semantic::TexCoords(1)).is_some(),
        "synthetic bake GLB must contain TEXCOORD_1"
    );
    let mut lights = [test_light([0.0, 0.0, 2.0], [1.0, 1.0, 1.0, 1.0])];
    lights[0].radius = 4.0;
    let directional = DirectionalBakeLight {
        color_rgba: [0.0; 4],
        rotation_xyzw: [0.0, 0.0, 0.0, 1.0],
        illuminance: 0.0,
    };
    let sampling = LightmapSamplingSettings {
        min_samples: 1,
        max_samples: 1,
        variance_threshold: 0.0,
    };
    let mut cache = TileCache::open(&cache_dir, "synthetic-lightmap-v1", false).unwrap();
    let progress = ProgressReporter::with_writer(ProgressMode::Off, io::sink(), false);
    progress.started("CPU bake", None);
    let result = bake_direct_pages(
        &scene,
        &lights,
        &directional,
        [0.0; 4],
        None,
        7,
        &output_dir,
        sampling,
        LightmapDenoiseSettings::default(),
        0,
        4,
        "synthetic-lightmap-v1",
        LightmapDebugSettings::default(),
        &mut cache,
        Some(&progress),
    )
    .unwrap();
    assert!(progress.snapshot().cache_misses > 0);

    assert_eq!(result.pages.len(), 1);
    assert!(result.pages[0].covered_texels > 0);
    assert!(result.sampling.sampled_texels > 0);
    assert!(cache.stats().writes > 0);
    let page = &result.pages[0];
    let raw = fs::read(&page.raw_path).unwrap();
    assert_eq!(raw.len(), page.width as usize * page.height as usize * 8);
    let mut has_positive_radiance = false;
    for pixel in raw.chunks_exact(8) {
        let channels = [
            f16::from_bits(u16::from_le_bytes([pixel[0], pixel[1]])).to_f32(),
            f16::from_bits(u16::from_le_bytes([pixel[2], pixel[3]])).to_f32(),
            f16::from_bits(u16::from_le_bytes([pixel[4], pixel[5]])).to_f32(),
            f16::from_bits(u16::from_le_bytes([pixel[6], pixel[7]])).to_f32(),
        ];
        assert!(channels.iter().all(|channel| channel.is_finite()));
        has_positive_radiance |= channels[..3].iter().any(|channel| *channel > 0.0);
    }
    assert!(has_positive_radiance);

    let resumed_output_dir = root.join("resumed");
    fs::create_dir_all(&resumed_output_dir).unwrap();
    let mut resumed_cache = TileCache::open(&cache_dir, "synthetic-lightmap-v1", false).unwrap();
    let resumed_progress = ProgressReporter::with_writer(ProgressMode::Off, io::sink(), false);
    resumed_progress.started("CPU bake", None);
    let resumed = bake_direct_pages(
        &scene,
        &lights,
        &directional,
        [0.0; 4],
        None,
        7,
        &resumed_output_dir,
        sampling,
        LightmapDenoiseSettings::default(),
        0,
        4,
        "synthetic-lightmap-v1",
        LightmapDebugSettings::default(),
        &mut resumed_cache,
        Some(&resumed_progress),
    )
    .unwrap();
    assert!(resumed_progress.snapshot().cache_hits > 0);
    assert_eq!(resumed.pages.len(), 1);
    assert!(resumed_cache.stats().hits > 0);
    assert_eq!(resumed_cache.stats().writes, 0);

    let (pages, atlases) = pack_lightmap_pages(result.pages, &output_dir, 32, None).unwrap();
    assert_eq!(pages[0].atlas_index, 0);
    assert_eq!(atlases.len(), 1);
    let atlas = &atlases[0];
    let ktx_path = output_dir.join("lightmap-atlas-0000.ktx2");
    write_rgba16f(&atlas.raw_path, &ktx_path, atlas.width, atlas.height).unwrap();
    let encoded = fs::read(&ktx_path).unwrap();
    let reader = ::ktx2::Reader::new(&encoded).unwrap();
    let header = reader.header();
    assert_eq!(header.format, Some(::ktx2::Format::R16G16B16A16_SFLOAT));
    assert_eq!(header.pixel_width, atlas.width);
    assert_eq!(header.pixel_height, atlas.height);
    assert_eq!(header.level_count, 1);
    let level = reader.levels().next().unwrap().data;
    assert_eq!(
        level.len(),
        atlas.width as usize * atlas.height as usize * 8
    );

    let _ = fs::remove_dir_all(&root);
}

#[test]
fn primitive_light_fingerprints_retain_unaffected_pages() {
    let primitive = test_primitive();
    let far_red = test_light([100.0, 0.0, 0.0], [1.0, 0.0, 0.0, 1.0]);
    let far_blue = test_light([100.0, 0.0, 0.0], [0.0, 0.0, 1.0, 1.0]);
    assert_eq!(
        primitive_tile_fingerprint("scene", 0, &primitive, &[far_red]),
        primitive_tile_fingerprint("scene", 0, &primitive, &[far_blue])
    );

    let near_red = test_light([0.0, 0.0, 1.0], [1.0, 0.0, 0.0, 1.0]);
    let near_blue = test_light([0.0, 0.0, 1.0], [0.0, 0.0, 1.0, 1.0]);
    assert_ne!(
        primitive_tile_fingerprint("scene", 0, &primitive, &[near_red]),
        primitive_tile_fingerprint("scene", 0, &primitive, &[near_blue])
    );
}

#[test]
fn raster_barycentrics_accept_inside_and_reject_outside_samples() {
    let triangle = [Vec2::ZERO, Vec2::X, Vec2::Y];
    let inside = barycentric(Vec2::splat(0.25), triangle).expect("inside sample");
    assert!((inside.iter().sum::<f32>() - 1.0).abs() < 1e-6);
    assert!(barycentric(Vec2::new(0.9, 0.9), triangle).is_none());
}

#[test]
fn raster_uses_center_samples_inside_and_2x2_samples_on_edges() {
    let fully_covered = pixel_sample_weights(
        0,
        0,
        1,
        1,
        [
            Vec2::new(-10.0, -10.0),
            Vec2::new(20.0, -10.0),
            Vec2::new(-10.0, 20.0),
        ],
    );
    assert_eq!(fully_covered.1, 1);
    assert!(fully_covered.0[0].is_some());

    let edge = pixel_sample_weights(0, 0, 1, 1, [Vec2::ZERO, Vec2::X, Vec2::Y]);
    assert_eq!(edge.1, 4);
    assert!(edge.0.iter().any(Option::is_none));
    assert!(edge.0.iter().any(Option::is_some));
}

#[test]
fn rgba16f_encoding_is_linear_four_channel_little_endian() {
    let bytes = encode_rgba16f(&[Vec3::new(1.0, 0.5, 0.25)]);
    assert_eq!(bytes.len(), 8);
    assert_eq!(&bytes[0..2], &0x3c00_u16.to_le_bytes());
    assert_eq!(&bytes[6..8], &0x3c00_u16.to_le_bytes());
}

#[test]
fn tile_payload_round_trip_is_sparse_and_preserves_sampling_metadata() {
    let pixels = vec![Vec3::ZERO, Vec3::new(1.0, 2.0, 3.0), Vec3::ZERO];
    let owners = vec![None, Some(7), None];
    let features = vec![
        DenoiseFeature::default(),
        DenoiseFeature {
            position: [1.0, 2.0, 3.0],
            normal: [0.0, 0.0, 1.0],
            material_id: 4,
            relative_variance: 0.25,
            coverage: 0.5,
            sample_count: 4,
        },
        DenoiseFeature::default(),
    ];
    let summary = LightmapSamplingSummary {
        sampled_texels: 1,
        total_samples: 4,
        min_samples: 4,
        max_samples: 4,
        max_relative_variance: 0.25,
    };
    let payload = encode_tile_payload(3, 1, &pixels, &owners, &features, summary).unwrap();
    assert!(payload.len() < 3 * 52);
    let decoded = decode_tile_payload(
        &TileRecord {
            width: 3,
            height: 1,
            payload,
        },
        3,
        1,
    )
    .unwrap();
    assert_eq!(decoded.pixels, pixels);
    assert_eq!(decoded.owners, owners);
    assert_eq!(decoded.features, features);
    assert_eq!(decoded.summary, summary);
}

#[test]
fn chart_dilation_fills_a_single_chart_within_the_padding_radius() {
    let mut pixels = vec![Vec3::ZERO; 5];
    pixels[0] = Vec3::ONE;
    let mut owners = vec![Some(7), None, None, None, None];

    let dilated = dilate_chart_aware(&mut pixels, &mut owners, 2, 5, 1);

    assert_eq!(dilated, 2);
    assert_eq!(owners, vec![Some(7), Some(7), Some(7), None, None]);
    assert_eq!(pixels[2], Vec3::ONE);
}

#[test]
fn chart_dilation_does_not_bridge_two_chart_fronts() {
    let mut pixels = vec![Vec3::ZERO; 6];
    pixels[0] = Vec3::X;
    pixels[5] = Vec3::Y;
    let mut owners = vec![Some(1), None, None, None, None, Some(2)];

    let dilated = dilate_chart_aware(&mut pixels, &mut owners, 4, 6, 1);

    assert_eq!(dilated, 2);
    assert_eq!(owners[1], Some(1));
    assert_eq!(owners[4], Some(2));
    assert_eq!(owners[2], None);
    assert_eq!(owners[3], None);
}

#[test]
fn atlas_packing_assigns_guttered_non_overlapping_regions() {
    let root =
        std::env::temp_dir().join(format!("bevyout-lightmap-pack-test-{}", std::process::id()));
    let _ = fs::remove_dir_all(&root);
    fs::create_dir_all(&root).unwrap();
    let page_paths = [root.join("page-0.raw"), root.join("page-1.raw")];
    fs::write(&page_paths[0], vec![0_u8; 2 * 2 * 8]).unwrap();
    fs::write(&page_paths[1], vec![0_u8; 2 * 3 * 8]).unwrap();
    let pages = vec![
        LightmapPage {
            primitive_index: 0,
            width: 2,
            height: 2,
            raw_path: page_paths[0].clone(),
            covered_texels: 2,
            dilated_texels: 3,
            atlas_index: usize::MAX,
            atlas_offset: [0, 0],
        },
        LightmapPage {
            primitive_index: 1,
            width: 2,
            height: 3,
            raw_path: page_paths[1].clone(),
            covered_texels: 4,
            dilated_texels: 5,
            atlas_index: usize::MAX,
            atlas_offset: [0, 0],
        },
    ];

    let (pages, atlases) = pack_lightmap_pages(pages, &root, 32, None).unwrap();

    assert_eq!(atlases.len(), 1);
    assert!(atlases[0].width <= 32 && atlases[0].height <= 32);
    assert_eq!(pages[0].atlas_index, 0);
    assert_eq!(pages[1].atlas_index, 0);
    assert!(pages[0].atlas_offset[0] >= 2 && pages[0].atlas_offset[1] >= 2);
    assert!(pages[1].atlas_offset[0] >= 2 && pages[1].atlas_offset[1] >= 2);
    assert_ne!(pages[0].atlas_offset, pages[1].atlas_offset);
    assert_eq!(
        fs::metadata(&atlases[0].raw_path).unwrap().len(),
        u64::from(atlases[0].width) * u64::from(atlases[0].height) * 8
    );
    let _ = fs::remove_dir_all(root);
}

#[test]
fn variance_output_persists_covered_values_and_marks_padding_nan() {
    let root = std::env::temp_dir().join(format!(
        "bevyout-lightmap-variance-test-{}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&root);
    fs::create_dir_all(&root).unwrap();
    let owners = vec![Some(3), None];
    let features = vec![
        DenoiseFeature {
            relative_variance: 0.25,
            ..Default::default()
        },
        DenoiseFeature {
            relative_variance: 0.75,
            ..Default::default()
        },
    ];

    write_variance_output(&root, 4, &owners, &features).unwrap();

    let bytes = fs::read(root.join("lightmap-variance-0004.r32f.raw")).unwrap();
    assert_eq!(bytes.len(), 2 * std::mem::size_of::<f32>());
    assert_eq!(f32::from_le_bytes(bytes[0..4].try_into().unwrap()), 0.25);
    assert!(f32::from_le_bytes(bytes[4..8].try_into().unwrap()).is_nan());
    let _ = fs::remove_dir_all(root);
}
