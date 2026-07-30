use super::*;

fn open_sheet_fragment(material: usize) -> ComposedPrimitive {
    ComposedPrimitive {
        name: "paper_sheet".into(),
        reference_form_ids: vec![1],
        material,
        positions: vec![Vec3::ZERO, Vec3::X, Vec3::new(1.0, 1.0, 0.0), Vec3::Y],
        normals: vec![Vec3::Z; 4],
        uvs: vec![Vec2::ZERO, Vec2::X, Vec2::ONE, Vec2::Y],
        colors: vec![Vec4::ONE; 4],
        transport_colors: vec![Vec4::ONE; 4],
        indices: vec![0, 1, 2, 0, 2, 3],
    }
}

#[test]
fn alpha_sampling_wraps_with_gltf_upper_left_origin() {
    let image = RgbaImage::from_raw(1, 2, vec![255, 0, 0, 255, 0, 255, 0, 128]).unwrap();
    let texture = SampledTexture {
        image: Arc::new(image),
        wrap_s: WrapMode::Repeat,
        wrap_t: WrapMode::Clamp,
    };
    assert_eq!(texture.sample(Vec2::ZERO), Vec4::new(1.0, 0.0, 0.0, 1.0));
    assert!((texture.sample(Vec2::new(0.0, 1.0)).w - 128.0 / 255.0).abs() < 1e-6);
}

#[test]
fn local_thickness_map_marks_a_thin_closed_surface_as_transmissive() {
    let fragment = ComposedPrimitive {
        name: "thin_panel".into(),
        reference_form_ids: vec![1],
        material: 0,
        positions: vec![
            Vec3::new(0.0, 0.1, 0.0),
            Vec3::new(1.0, 0.1, 0.0),
            Vec3::new(1.0, 0.1, 1.0),
            Vec3::new(0.0, 0.1, 1.0),
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(1.0, 0.0, 1.0),
            Vec3::new(0.0, 0.0, 1.0),
        ],
        normals: vec![Vec3::Y; 4]
            .into_iter()
            .chain(vec![Vec3::NEG_Y; 4])
            .collect(),
        uvs: vec![
            Vec2::ZERO,
            Vec2::X,
            Vec2::ONE,
            Vec2::Y,
            Vec2::ZERO,
            Vec2::X,
            Vec2::ONE,
            Vec2::Y,
        ],
        colors: vec![Vec4::ONE; 8],
        transport_colors: vec![Vec4::ONE; 8],
        indices: vec![0, 1, 2, 0, 2, 3, 4, 6, 5, 4, 7, 6],
    };
    let image = local_thickness_map(&fragment).expect("closed surface has a thickness map");
    let maximum_transmission = image.pixels().map(|pixel| pixel[3]).max().unwrap_or(0);
    assert!(maximum_transmission > 200);
    assert!(image.pixels().any(|pixel| pixel[1] > 0));
}

#[test]
fn local_thickness_map_treats_an_open_sheet_as_paper_thin() {
    let fragment = open_sheet_fragment(0);
    let image = local_thickness_map(&fragment).expect("open sheet has a thickness map");
    assert!(image.pixels().all(|pixel| pixel[3] > 240));
    assert!(image.pixels().all(|pixel| pixel[1] == 0));
}

#[test]
fn metallic_materials_do_not_generate_local_thickness_maps() {
    let mut resources = OutputResources::default();
    resources.materials.push(json!({
        "extras": {
            "bevyout_fallout_material": {
                "translucency_enabled": true,
                "translucency_strength": 0.2
            }
        }
    }));
    resources.transport_materials.push(TransportMaterial {
        metallic_factor: 1.0,
        ..TransportMaterial::default()
    });
    let mut fragments = vec![open_sheet_fragment(0)];

    assert_eq!(
        prepare_local_translucency(&mut fragments, &mut resources).unwrap(),
        0
    );
    assert_eq!(resources.materials.len(), 1);
}

#[test]
fn batching_offsets_indices_and_preserves_reference_ids() {
    let fragment = |name: &str, reference| ComposedPrimitive {
        name: name.into(),
        reference_form_ids: vec![reference],
        material: 0,
        positions: vec![Vec3::ZERO, Vec3::X, Vec3::Y],
        normals: vec![Vec3::Z; 3],
        uvs: vec![Vec2::ZERO; 3],
        colors: vec![Vec4::ONE; 3],
        transport_colors: vec![Vec4::ONE; 3],
        indices: vec![0, 1, 2],
    };
    let (batched, stats) = batch_fragments(vec![fragment("a", 1), fragment("b", 2)], 64.0);
    assert_eq!(stats.batches_created, 1);
    assert_eq!(batched[0].indices, vec![0, 1, 2, 3, 4, 5]);
    assert_eq!(batched[0].reference_form_ids, vec![1, 2]);
}

#[test]
fn static_skin_is_flattened_from_joint_bind_matrices() {
    let positions = [Vec3::new(1.0, 0.0, 0.0)];
    let normals = [Vec3::Y];
    let joints = [[0, 1, 0, 0]];
    let weights = [[0.25, 0.75, 0.0, 0.0]];
    let matrices = [
        Mat4::from_translation(Vec3::new(0.0, 2.0, 0.0)),
        Mat4::from_translation(Vec3::new(0.0, 4.0, 0.0)),
    ];

    let (flattened_positions, flattened_normals) =
        flatten_skin(&positions, &normals, &joints, &weights, &matrices).unwrap();

    assert_eq!(flattened_positions, vec![Vec3::new(1.0, 3.5, 0.0)]);
    assert_eq!(flattened_normals, vec![Vec3::Y]);
}

fn floor_quad(
    name: &str,
    reference_form_id: u32,
    minimum_x: f32,
    maximum_x: f32,
    reverse_winding: bool,
) -> ComposedPrimitive {
    let positions = vec![
        Vec3::new(minimum_x, 0.0, 0.0),
        Vec3::new(maximum_x, 0.0, 0.0),
        Vec3::new(maximum_x, 0.0, 1.0),
        Vec3::new(minimum_x, 0.0, 1.0),
    ];
    let indices = if reverse_winding {
        vec![0, 1, 2, 0, 2, 3]
    } else {
        vec![0, 2, 1, 0, 3, 2]
    };
    ComposedPrimitive {
        name: name.into(),
        reference_form_ids: vec![reference_form_id],
        material: 0,
        positions,
        normals: vec![Vec3::Y; 4],
        uvs: vec![Vec2::ZERO, Vec2::X, Vec2::ONE, Vec2::Y],
        colors: vec![Vec4::new(1.0, 0.5, 0.25, 1.0); 4],
        transport_colors: vec![Vec4::ONE; 4],
        indices,
    }
}

fn triangle_fragment(
    name: &str,
    reference_form_id: u32,
    positions: [Vec3; 3],
) -> ComposedPrimitive {
    ComposedPrimitive {
        name: name.into(),
        reference_form_ids: vec![reference_form_id],
        material: 0,
        positions: positions.into(),
        normals: vec![Vec3::Y; 3],
        uvs: vec![Vec2::ZERO, Vec2::X, Vec2::Y],
        colors: vec![Vec4::ONE; 3],
        transport_colors: vec![Vec4::ONE; 3],
        indices: vec![0, 1, 2],
    }
}

#[test]
fn seam_stitch_moves_only_positions_across_coplanar_placement_boundary() {
    let delta = 0.00003;
    let left = floor_quad("left", 1, 0.0, 1.0, false);
    let right = floor_quad("right", 2, 1.0 + delta, 2.0, false);
    let original_left = left.clone();
    let original_right = right.clone();
    let mut fragments = vec![left, right];

    let stats = stitch_static_seams(&mut fragments);

    assert_eq!(stats.edges_matched, 1);
    assert_eq!(stats.vertices_adjusted, 4);
    assert!(stats.max_correction_meters > 0.0);
    assert_eq!(fragments[0].positions[1], fragments[1].positions[0]);
    assert_eq!(fragments[0].positions[2], fragments[1].positions[3]);
    assert_eq!(fragments[0].positions[0], original_left.positions[0]);
    assert_eq!(fragments[1].positions[1], original_right.positions[1]);
    assert_eq!(fragments[0].normals, original_left.normals);
    assert_eq!(fragments[1].normals, original_right.normals);
    assert_eq!(fragments[0].uvs, original_left.uvs);
    assert_eq!(fragments[1].uvs, original_right.uvs);
    assert_eq!(fragments[0].colors, original_left.colors);
    assert_eq!(fragments[1].colors, original_right.colors);
    assert_eq!(fragments[0].indices, original_left.indices);
    assert_eq!(fragments[1].indices, original_right.indices);
}

#[test]
fn seam_stitch_rejects_opposite_winding_far_and_same_placement_edges() {
    let delta = 0.00003;
    let mut opposite = vec![
        floor_quad("left", 1, 0.0, 1.0, false),
        floor_quad("right", 2, 1.0 + delta, 2.0, true),
    ];
    let opposite_original = opposite.clone();
    assert_eq!(stitch_static_seams(&mut opposite).edges_matched, 0);
    assert_eq!(opposite[0].positions, opposite_original[0].positions);
    assert_eq!(opposite[1].positions, opposite_original[1].positions);

    let mut far = vec![
        floor_quad("left", 1, 0.0, 1.0, false),
        floor_quad(
            "right",
            2,
            1.0 + SEAM_STITCH_TOLERANCE_METERS * 2.0,
            2.0,
            false,
        ),
    ];
    assert_eq!(stitch_static_seams(&mut far).edges_matched, 0);

    let mut same_placement = vec![
        floor_quad("left", 1, 0.0, 1.0, false),
        floor_quad("right", 1, 1.0 + delta, 2.0, false),
    ];
    assert_eq!(stitch_static_seams(&mut same_placement).edges_matched, 0);
}

#[test]
fn seam_stitch_closes_a_coplanar_t_junction_without_merging_vertices() {
    let delta = 0.00003;
    let point_fragment = triangle_fragment(
        "point",
        1,
        [
            Vec3::new(1.0, 0.0, delta),
            Vec3::new(1.0, 0.0, 1.0),
            Vec3::new(2.0, 0.0, 1.0),
        ],
    );
    let edge_fragment = triangle_fragment(
        "edge",
        2,
        [
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, 1.0),
            Vec3::new(2.0, 0.0, 0.0),
        ],
    );
    let mut fragments = vec![point_fragment, edge_fragment];

    let stats = stitch_static_seams(&mut fragments);

    assert_eq!(stats.edges_matched, 0);
    assert_eq!(stats.vertices_adjusted, 1);
    assert_eq!(fragments[0].positions[0], Vec3::new(1.0, 0.0, 0.0));
    assert_eq!(fragments[1].positions[0], Vec3::new(0.0, 0.0, 0.0));
    assert_eq!(fragments[1].positions[2], Vec3::new(2.0, 0.0, 0.0));
}
