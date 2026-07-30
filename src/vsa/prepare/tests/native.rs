use super::*;
use std::sync::Barrier;
use std::sync::atomic::AtomicUsize;

fn job(root: &Path, index: usize) -> BlenderAssetJob {
    BlenderAssetJob {
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

use nif::fo3::{Scene, SceneMesh, SceneNode, Transform, merge_actor_scene_attached};

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
    let result =
        run_native_batch(std::slice::from_ref(&failed), &root, &[], Some(1), false).unwrap();
    assert_eq!(result.summary().failed, 1);
    assert!(!failed.output.exists());
    assert!(!failed.physics_output.exists());
    let strict =
        run_native_batch(std::slice::from_ref(&failed), &root, &[], Some(1), true).unwrap();
    assert!(strict.enforce_strict(true).is_err());
    let _ = fs::remove_dir_all(root);
}
