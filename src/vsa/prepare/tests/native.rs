use super::*;
use std::sync::Barrier;
use std::sync::atomic::AtomicUsize;

fn job(root: &Path, index: usize) -> AssetJob {
    AssetJob {
        kind: super::super::super::assets::AssetJobKind::StaticNif,
        input: root.join(format!("{index}.nif")),
        output: root.join(format!("{index}.glb")),
        physics_output: root.join(format!("{index}.physics.json.gz")),
        model: format!("synthetic/{index}.nif"),
        conversion: super::super::super::assets::AssetConversion::Preserve,
        root_transform_policy:
            super::super::super::assets::RootTransformPolicy::PreserveReviewRequired,
    }
}

use nif::fo3::{
    Scene, SceneAlphaMode, SceneMaterial, SceneMesh, SceneNode, SceneSkin, Transform,
    merge_actor_scene, merge_actor_scene_attached,
};

fn identity_transform() -> Transform {
    Transform {
        translation: [0.0, 0.0, 0.0],
        rotation: [1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0],
        scale: 1.0,
    }
}

fn bone(name: &str, children: Vec<usize>) -> SceneNode {
    SceneNode {
        source_block: 0,
        name: name.to_owned(),
        transform: identity_transform(),
        children,
        mesh: None,
        skin: None,
    }
}

fn mesh_node(name: &str) -> SceneNode {
    SceneNode {
        source_block: 0,
        name: name.to_owned(),
        transform: identity_transform(),
        children: Vec::new(),
        mesh: Some(SceneMesh {
            name: name.to_owned(),
            positions: vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]],
            normals: Vec::new(),
            tangents: Vec::new(),
            colors: Vec::new(),
            tex_coords: Vec::new(),
            joints: Vec::new(),
            weights: Vec::new(),
            indices: vec![0, 1, 2],
            material: None,
        }),
        skin: None,
    }
}

fn empty_scene() -> Scene {
    Scene {
        nodes: Vec::new(),
        roots: Vec::new(),
        materials: Vec::new(),
        skins: Vec::new(),
        issues: Vec::new(),
        statistics: Default::default(),
        animations: Vec::new(),
        animation_sound_cues: Vec::new(),
    }
}

fn test_material(diffuse_texture: &str) -> SceneMaterial {
    SceneMaterial {
        name: "synthetic".into(),
        base_color: [1.0; 4],
        emissive: [0.0; 3],
        emissive_multiplier: 1.0,
        roughness: 0.5,
        alpha_mode: SceneAlphaMode::Opaque,
        alpha_cutoff: None,
        double_sided: false,
        unlit: false,
        diffuse_texture: Some(diffuse_texture.into()),
        normal_texture: None,
        specular_texture: None,
        glow_texture: None,
        height_texture: None,
        environment_texture: None,
        environment_mask: None,
        shader_type: 0,
        shader_flags_1: 0,
        shader_flags_2: 0,
    }
}

fn textured_part(name: &str, diffuse_texture: &str) -> Scene {
    let mut scene = empty_scene();
    scene.materials.push(test_material(diffuse_texture));
    let mut node = mesh_node(name);
    node.mesh.as_mut().unwrap().material = Some(0);
    scene.nodes.push(node);
    scene.roots.push(0);
    scene
}

/// A minimal FO3-shaped skeleton carrying the two head attachment frames
/// the assembler targets: the static `Bip01 Head` bone and the animated
/// `HeadAnims` node.
fn skeleton_with_head_frames() -> Scene {
    let mut scene = empty_scene();
    // Bip01(0) -> Bip01 Head(1) -> HeadAnims(2)
    scene.nodes.push(bone("Bip01", vec![1]));
    scene.nodes.push(bone("Bip01 Head", vec![2]));
    scene.nodes.push(bone("HeadAnims", Vec::new()));
    scene.roots.push(0);
    scene
}

/// A standalone hair part: an independent root node (head-local space)
/// carrying the visible hair mesh, exactly the shape FO3 hair NIFs decode
/// to before assembly.
fn hair_part() -> Scene {
    let mut scene = empty_scene();
    scene.nodes.push(bone("HairRaiderRoot", vec![1]));
    scene.nodes.push(mesh_node("hairraider"));
    scene.roots.push(0);
    scene
}

fn facegen_head_part(z: f32) -> Scene {
    let mut scene = empty_scene();
    scene.nodes.push(bone("FaceJoint", Vec::new()));
    let mut head = mesh_node("face");
    head.mesh.as_mut().unwrap().positions = vec![[0.0, 0.0, z], [1.0, 0.0, z], [0.0, 1.0, z]];
    head.mesh.as_mut().unwrap().joints = vec![[0, 0, 0, 0]; 3];
    head.mesh.as_mut().unwrap().weights = vec![[1.0, 0.0, 0.0, 0.0]; 3];
    head.skin = Some(0);
    scene.nodes.push(head);
    scene.roots.extend([0, 1]);
    scene.skins.push(SceneSkin {
        name: "Face Skin".into(),
        joints: vec![0],
        inverse_bind_matrices: vec![glam::Mat4::IDENTITY.to_cols_array()],
        skeleton: Some(0),
    });
    scene
}

fn facegen_hair_part() -> Scene {
    let mut scene = empty_scene();
    scene.nodes.push(bone("HairRoot", vec![1]));
    let mut hair = mesh_node("hair");
    hair.mesh.as_mut().unwrap().positions = vec![[0.0, 0.0, 1.0], [1.0, 0.0, 1.0], [0.0, 1.0, 1.0]];
    scene.nodes.push(hair);
    scene.roots.push(0);
    scene
}

fn descends_from(scene: &Scene, ancestor: usize, target: usize) -> bool {
    if ancestor == target {
        return true;
    }
    scene.nodes[ancestor]
        .children
        .iter()
        .any(|&child| descends_from(scene, child, target))
}

// #206: a visible hair head_anim_part selects the `HeadAnims` attachment
// frame; every other head part rides `Bip01 Head`.
#[test]
fn head_anim_parts_attach_to_headanims_others_to_bip01_head() {
    assert_eq!(head_part_attachment(true), "HeadAnims");
    assert_eq!(head_part_attachment(false), "Bip01 Head");
}

#[test]
fn facegen_head_replacement_does_not_rewrite_shared_actor_material() {
    let source = "textures/head.dds";
    let generated = "__bevyout_facegen/head.png";
    let mut actor = textured_part("Body", source);
    let mut head = textured_part("Head", source);

    replace_facegen_diffuse(&mut head, source, generated);
    merge_actor_scene(&mut actor, &head).unwrap();

    assert_eq!(actor.materials[0].diffuse_texture.as_deref(), Some(source));
    assert_eq!(
        actor.materials[1].diffuse_texture.as_deref(),
        Some(generated)
    );
    assert_eq!(actor.nodes[0].mesh.as_ref().unwrap().material, Some(0));
    assert_eq!(actor.nodes[1].mesh.as_ref().unwrap().material, Some(1));
}

#[test]
fn facegen_head_skip_warning_covers_missing_anchor_and_decoded_head() {
    assert_eq!(
        facegen_head_skip_warning(None),
        "actor head FaceGen skipped: descriptor has no head anchor; retaining authored rest pose"
    );
    assert_eq!(
        facegen_head_skip_warning(Some("meshes/head.nif")),
        "actor head FaceGen skipped: head visual meshes/head.nif was not decoded; retaining authored rest pose"
    );
}

// #206: merging a hair part with the `HeadAnims` attachment the assembler
// selects yields a `HeadAnims` node in the output with the hair mesh
// parented beneath it — the node the idle KF's required `HeadAnims` target
// binds against at runtime.
#[test]
fn hair_merged_with_headanims_is_parented_under_a_headanims_node() {
    let mut actor = skeleton_with_head_frames();
    let hair = hair_part();
    merge_actor_scene_attached(&mut actor, &hair, head_part_attachment(true)).unwrap();

    let head_anims = actor
        .nodes
        .iter()
        .position(|node| node.name == "HeadAnims" && node.mesh.is_none())
        .expect("assembled actor must retain a HeadAnims node");
    let hair_mesh = actor
        .nodes
        .iter()
        .position(|node| {
            node.mesh
                .as_ref()
                .is_some_and(|mesh| mesh.name == "hairraider")
        })
        .expect("hair mesh must survive the merge");
    assert!(
        descends_from(&actor, head_anims, hair_mesh),
        "hair mesh must be parented beneath the HeadAnims node"
    );
}

#[test]
fn facegen_hair_fit_follows_only_head_advance_toward_the_hair() {
    let mut actor = skeleton_with_head_frames();
    actor.nodes.push(bone("FaceJoint", Vec::new()));
    let rest_head = facegen_head_part(0.0);
    let deformed_head = facegen_head_part(1.0);
    let fit = build_native_facegen_head_fit(&actor, &rest_head, &deformed_head)
        .expect("synthetic weighted head must produce a fit field");
    let mut hair = facegen_hair_part();

    let moved = fit_native_hair_to_facegen(&actor, &mut hair, "HeadAnims", &fit);

    assert_eq!(moved, 3);
    assert!(
        hair.nodes[1]
            .mesh
            .as_ref()
            .unwrap()
            .positions
            .iter()
            .all(|position| position[2] > 2.0)
    );
}

#[test]
fn facegen_hair_fit_does_not_pull_hair_into_an_inward_morph() {
    let mut actor = skeleton_with_head_frames();
    actor.nodes.push(bone("FaceJoint", Vec::new()));
    let rest_head = facegen_head_part(0.0);
    let deformed_head = facegen_head_part(-1.0);
    let fit = build_native_facegen_head_fit(&actor, &rest_head, &deformed_head)
        .expect("synthetic weighted head must produce a fit field");
    let mut hair = facegen_hair_part();

    let moved = fit_native_hair_to_facegen(&actor, &mut hair, "HeadAnims", &fit);

    assert_eq!(moved, 0);
    assert_eq!(
        hair.nodes[1].mesh.as_ref().unwrap().positions[0],
        [0.0, 0.0, 1.0]
    );
}

#[test]
fn bounded_workers_never_exceed_the_requested_count() {
    let active = AtomicUsize::new(0);
    let maximum = AtomicUsize::new(0);
    let first_wave = Barrier::new(3);
    let values = (0..12).collect::<Vec<_>>();
    let results = run_bounded(&values, 3, |_, value| {
        let current = active.fetch_add(1, Ordering::SeqCst) + 1;
        maximum.fetch_max(current, Ordering::SeqCst);
        if *value < 3 {
            first_wave.wait();
        }
        active.fetch_sub(1, Ordering::SeqCst);
        value * 2
    });
    assert_eq!(
        results,
        values.iter().map(|value| value * 2).collect::<Vec<_>>()
    );
    assert_eq!(maximum.load(Ordering::SeqCst), 3);
}

#[test]
fn duplicate_output_paths_are_rejected_before_workers_start() {
    let root = std::env::temp_dir();
    let first = job(&root, 1);
    let mut second = job(&root, 2);
    second.output = first.output.clone();
    assert!(reject_duplicate_native_outputs(&[first, second]).is_err());
}

#[test]
fn malformed_nif_failure_is_isolated_and_leaves_no_outputs() {
    let root = std::env::temp_dir().join(format!("bevyout-native-batch-{}", std::process::id()));
    let _ = fs::remove_dir_all(&root);
    fs::create_dir_all(&root).unwrap();
    let failed = job(&root, 1);
    fs::write(&failed.input, b"not a nif").unwrap();
    let result = run_native_batch(
        std::slice::from_ref(&failed),
        &root,
        &[],
        &root,
        Some(1),
        false,
    )
    .unwrap();
    assert_eq!(result.summary().failed, 1);
    assert!(!failed.output.exists());
    assert!(!failed.physics_output.exists());
    let strict = run_native_batch(
        std::slice::from_ref(&failed),
        &root,
        &[],
        &root,
        Some(1),
        true,
    )
    .unwrap();
    assert!(strict.enforce_strict(true).is_err());
    let _ = fs::remove_dir_all(root);
}

fn glb_with_embedded_prepared_texture() -> Vec<u8> {
    let geometry = [9u8, 8, 7, 6, 5, 4, 3, 2];
    let mut ktx2 = vec![0; 80];
    ktx2[..12].copy_from_slice(crate::vsa::assets::KTX2_IDENTIFIER);
    ktx2[20..24].copy_from_slice(&2u32.to_le_bytes());
    ktx2[24..28].copy_from_slice(&2u32.to_le_bytes());
    ktx2[40..44].copy_from_slice(&1u32.to_le_bytes());
    let mut binary = geometry.to_vec();
    binary.extend_from_slice(&ktx2);
    let document = serde_json::json!({
        "asset": {"version": "2.0"},
        "buffers": [{"byteLength": binary.len()}],
        "bufferViews": [
            {"buffer": 0, "byteOffset": 0, "byteLength": geometry.len()},
            {"buffer": 0, "byteOffset": geometry.len(), "byteLength": ktx2.len()}
        ],
        "images": [{"bufferView": 1, "mimeType": "image/ktx2"}],
        "textures": [{"source": 0}]
    });
    let mut json = serde_json::to_vec(&document).unwrap();
    while !json.len().is_multiple_of(4) {
        json.push(b' ');
    }
    let total = 20 + json.len() + 8 + binary.len();
    let mut result = Vec::new();
    result.extend_from_slice(b"glTF");
    result.extend_from_slice(&2u32.to_le_bytes());
    result.extend_from_slice(&(total as u32).to_le_bytes());
    result.extend_from_slice(&(json.len() as u32).to_le_bytes());
    result.extend_from_slice(b"JSON");
    result.extend_from_slice(&json);
    result.extend_from_slice(&(binary.len() as u32).to_le_bytes());
    result.extend_from_slice(b"BIN\0");
    result.extend_from_slice(&binary);
    result
}

fn first_image_uri(glb: &[u8]) -> String {
    let json_len = u32::from_le_bytes(glb[12..16].try_into().unwrap()) as usize;
    let document: serde_json::Value = serde_json::from_slice(&glb[20..20 + json_len]).unwrap();
    document["images"][0]["uri"].as_str().unwrap().to_owned()
}

#[test]
fn native_glbs_with_the_same_texture_share_one_external_object() {
    let root = std::env::temp_dir().join(format!(
        "bevyout-native-external-texture-{}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&root);
    fs::create_dir_all(&root).unwrap();
    let first = job(&root, 41);
    let second = job(&root, 42);
    let source = glb_with_embedded_prepared_texture();
    fs::write(&first.output, &source).unwrap();
    fs::write(&second.output, &source).unwrap();

    externalize_native_glb_textures(&first, &root).unwrap();
    externalize_native_glb_textures(&second, &root).unwrap();

    let first_bytes = fs::read(&first.output).unwrap();
    let second_bytes = fs::read(&second.output).unwrap();
    let first_uri = first_image_uri(&first_bytes);
    assert_eq!(first_uri, first_image_uri(&second_bytes));
    assert!(first_uri.starts_with("/objects/texture/"));
    assert_eq!(
        fs::read(root.join(first_uri.trim_start_matches('/'))).unwrap(),
        &source[source.len() - 80..]
    );
    let store = FsPreparedObjectStore::open(&root).unwrap();
    assert_eq!(store.object_count().unwrap(), 1);
    let _ = fs::remove_dir_all(root);
}
