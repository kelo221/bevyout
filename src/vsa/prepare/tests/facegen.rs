use super::*;

fn geometry_bytes(vertex_count: usize, symmetric: usize, asymmetric: usize) -> Vec<u8> {
    let mut bytes = Vec::new();
    bytes.extend_from_slice(EGM_MAGIC);
    bytes.extend_from_slice(&(vertex_count as u32).to_le_bytes());
    bytes.extend_from_slice(&(symmetric as u32).to_le_bytes());
    bytes.extend_from_slice(&(asymmetric as u32).to_le_bytes());
    bytes.extend_from_slice(&2u32.to_le_bytes());
    bytes.extend_from_slice(&[0; 40]);
    for mode in 0..symmetric + asymmetric {
        bytes.extend_from_slice(&1.0f32.to_le_bytes());
        for vertex in 0..vertex_count {
            let x = if mode == 0 && vertex == 0 { 1 } else { 0 };
            bytes.extend_from_slice(&(x as i16).to_le_bytes());
            bytes.extend_from_slice(&0i16.to_le_bytes());
            bytes.extend_from_slice(&0i16.to_le_bytes());
        }
    }
    bytes
}

fn texture_bytes(width: u32, height: u32) -> Vec<u8> {
    let mut bytes = Vec::new();
    bytes.extend_from_slice(EGT_MAGIC);
    bytes.extend_from_slice(&height.to_le_bytes());
    bytes.extend_from_slice(&width.to_le_bytes());
    bytes.extend_from_slice(&50u32.to_le_bytes());
    bytes.extend_from_slice(&0u32.to_le_bytes());
    bytes.extend_from_slice(&2u32.to_le_bytes());
    bytes.extend_from_slice(&[0; 36]);
    for mode in 0..50 {
        bytes.extend_from_slice(&1.0f32.to_le_bytes());
        for channel in 0..3 {
            for pixel in 0..(width * height) {
                let value = if mode == 0 && channel == 0 && pixel == 0 {
                    10i8
                } else {
                    0
                };
                bytes.push(value as u8);
            }
        }
    }
    bytes
}

fn tri_layout_bytes(
    base_vertices: u32,
    triangles: u32,
    uv_count: u32,
    static_vertices: u32,
) -> Vec<u8> {
    let mut bytes = Vec::new();
    bytes.extend_from_slice(TRI_MAGIC);
    for value in [
        base_vertices,
        triangles,
        0,
        0,
        0,
        uv_count,
        1,
        0,
        0,
        static_vertices,
    ] {
        bytes.extend_from_slice(&value.to_le_bytes());
    }
    bytes.extend_from_slice(&[0; 16]);
    bytes.resize(
        bytes.len() + (base_vertices + static_vertices) as usize * 12,
        0,
    );
    bytes.resize(bytes.len() + triangles as usize * 12, 0);
    bytes
}

fn scene_mesh() -> nif::fo3::SceneMesh {
    nif::fo3::SceneMesh {
        name: "Head".into(),
        positions: vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]],
        normals: vec![[0.0, 0.0, 1.0]; 3],
        tangents: vec![[1.0, 0.0, 0.0, 1.0]; 3],
        colors: vec![[1.0; 4]; 3],
        tex_coords: vec![[0.0, 0.0], [1.0, 0.0], [0.0, 1.0]],
        joints: vec![[0; 4]; 3],
        weights: vec![[1.0, 0.0, 0.0, 0.0]; 3],
        indices: vec![0, 1, 2],
        material: None,
    }
}

#[test]
fn egm_parser_accepts_canonical_bounded_layout() {
    let morph = parse_geometry_morph(&geometry_bytes(3, 50, 30)).unwrap();
    assert_eq!(morph.vertex_count, 3);
    assert_eq!(morph.modes.len(), 80);
    assert_eq!(morph.modes[0][0], [1.0, 0.0, 0.0]);
}

#[test]
fn egm_parser_rejects_bad_magic_and_trailing_bytes() {
    let mut bytes = geometry_bytes(3, 50, 30);
    bytes[0] = b'X';
    assert!(matches!(
        parse_geometry_morph(&bytes),
        Err(FaceGenDiagnostic::UnsupportedAsset { .. })
    ));
    let mut bytes = geometry_bytes(3, 50, 30);
    bytes.push(0);
    assert!(matches!(
        parse_geometry_morph(&bytes),
        Err(FaceGenDiagnostic::UnsupportedAsset { .. })
    ));
}

#[test]
fn tri_layout_connects_base_nif_vertices_to_combined_egm_vertices() {
    let layout = parse_tri_layout(&tri_layout_bytes(3, 1, 3, 2)).unwrap();
    assert_eq!(layout.base_vertex_count, 3);
    assert_eq!(layout.combined_vertex_count, 5);
    assert_eq!(layout.triangle_count, 1);
    assert_eq!(layout.texture_coordinate_count, 3);
}

#[test]
fn egt_parser_accepts_signed_pixel_modes() {
    let morph = parse_texture_morph(&texture_bytes(2, 1)).unwrap();
    assert_eq!((morph.width, morph.height), (2, 1));
    assert_eq!(morph.modes[0][0][0], 10.0);
}

#[test]
fn geometry_deformation_preserves_topology_uvs_weights_and_recomputes_basis() {
    let morph = parse_geometry_morph(&geometry_bytes(3, 50, 30)).unwrap();
    let mut scene = nif::fo3::Scene {
        nodes: vec![nif::fo3::SceneNode {
            source_block: 0,
            name: "Head".into(),
            transform: nif::fo3::Transform {
                translation: [0.0; 3],
                rotation: [1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0],
                scale: 1.0,
            },
            children: Vec::new(),
            mesh: Some(scene_mesh()),
            skin: None,
        }],
        roots: vec![0],
        materials: Vec::new(),
        skins: Vec::new(),
        issues: Vec::new(),
        statistics: nif::fo3::SceneStatistics::default(),
        animations: Vec::new(),
        animation_sound_cues: Vec::new(),
    };
    let original_uvs = scene.nodes[0].mesh.as_ref().unwrap().tex_coords.clone();
    let original_weights = scene.nodes[0].mesh.as_ref().unwrap().weights.clone();
    apply_geometry_morph(
        &mut scene,
        &morph,
        &FaceGenCoefficients {
            geometry_symmetric: vec![1.0; 50],
            geometry_asymmetric: vec![0.0; 30],
            texture_symmetric: vec![0.0; 50],
        },
        3,
    )
    .unwrap();
    let mesh = scene.nodes[0].mesh.as_ref().unwrap();
    assert_eq!(mesh.positions[0], [1.0, 0.0, 0.0]);
    assert_eq!(mesh.indices, vec![0, 1, 2]);
    assert_eq!(mesh.tex_coords, original_uvs);
    assert_eq!(mesh.weights, original_weights);
    assert_eq!(mesh.normals[0], [0.0, 0.0, 1.0]);
    assert_eq!(mesh.tangents[0][3], 1.0);
}

#[test]
fn texture_synthesis_preserves_alpha_and_clamps_channels() {
    let morph = parse_texture_morph(&texture_bytes(2, 1)).unwrap();
    let mut base = image::RgbaImage::new(2, 1);
    base.put_pixel(0, 0, image::Rgba([250, 20, 30, 77]));
    base.put_pixel(1, 0, image::Rgba([1, 2, 3, 88]));
    let mut input = Cursor::new(Vec::new());
    image::DynamicImage::ImageRgba8(base)
        .write_to(&mut input, image::ImageFormat::Png)
        .unwrap();
    let output = synthesize_head_diffuse(
        &input.into_inner(),
        &morph,
        &FaceGenCoefficients {
            geometry_symmetric: vec![0.0; 50],
            geometry_asymmetric: vec![0.0; 30],
            texture_symmetric: vec![1.0; 50],
        },
    )
    .unwrap();
    let image = image::load_from_memory(&output).unwrap().to_rgba8();
    assert_eq!(image.get_pixel(0, 0).0, [255, 20, 30, 77]);
    assert_eq!(image.get_pixel(1, 0).0, [1, 2, 3, 88]);
}

#[test]
fn texture_synthesis_upsamples_lower_resolution_egt_without_touching_alpha() {
    let morph = parse_texture_morph(&texture_bytes(2, 1)).unwrap();
    let mut base = image::RgbaImage::new(4, 2);
    for y in 0..2 {
        for x in 0..4 {
            base.put_pixel(x, y, image::Rgba([250, 20, 30, (70 + y * 10 + x) as u8]));
        }
    }
    let mut input = Cursor::new(Vec::new());
    image::DynamicImage::ImageRgba8(base)
        .write_to(&mut input, image::ImageFormat::Png)
        .unwrap();
    let output = synthesize_head_diffuse(
        &input.into_inner(),
        &morph,
        &FaceGenCoefficients {
            geometry_symmetric: vec![0.0; 50],
            geometry_asymmetric: vec![0.0; 30],
            texture_symmetric: vec![1.0; 50],
        },
    )
    .unwrap();
    let image = image::load_from_memory(&output).unwrap().to_rgba8();
    assert_eq!((image.width(), image.height()), (4, 2));
    assert_eq!(image.get_pixel(0, 0).0, [255, 20, 30, 70]);
    assert_eq!(image.get_pixel(3, 0).0, [250, 20, 30, 73]);
    assert_eq!(image.get_pixel(3, 1).0[3], 83);
}
