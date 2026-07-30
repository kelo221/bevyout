use super::*;

fn actor(paths: &[&str]) -> ActorAnimationDiscoveryInput {
    ActorAnimationDiscoveryInput {
        reference_form_id: 1,
        base_form_id: 2,
        model_path: "meshes/characters/_male/skeleton.nif".to_owned(),
        skeleton_path: "meshes/characters/_male/skeleton.nif".to_owned(),
        skeleton_fingerprint: "skeleton-hash".to_owned(),
        explicit_kf_paths: paths.iter().map(|path| (*path).to_owned()).collect(),
        ..Default::default()
    }
}

fn asset(path: &str, hash: &str) -> ActorAnimationAsset {
    ActorAnimationAsset {
        path: path.to_owned(),
        fingerprint: hash.to_owned(),
        state: ActorAnimationAssetState::Compatible,
    }
}

#[test]
fn clip_names_are_normalized_and_collisions_are_explicit() {
    let catalog = build_actor_animation_catalog(
        "v1",
        "source",
        &[actor(&["idles/Power Walk.kf", "combat/POWER-WALK.kf"])],
        &[
            asset("meshes/characters/_male/idles/power walk.kf", "a"),
            asset("meshes/characters/_male/combat/power-walk.kf", "b"),
        ],
    );
    let names = catalog.animation_sets[0]
        .clips
        .iter()
        .map(|clip| clip.name.as_str())
        .collect::<Vec<_>>();
    assert_eq!(names, ["power_walk", "power_walk__2"]);
}

#[test]
fn a_byte_fingerprint_change_invalidates_the_set_identity() {
    let input = actor(&["idle.kf"]);
    let first = build_actor_animation_catalog(
        "v1",
        "source",
        std::slice::from_ref(&input),
        &[asset("meshes/characters/_male/idle.kf", "first")],
    );
    let second = build_actor_animation_catalog(
        "v1",
        "source",
        &[input],
        &[asset("meshes/characters/_male/idle.kf", "second")],
    );
    assert_ne!(
        first.animation_sets[0].source_fingerprint,
        second.animation_sets[0].source_fingerprint
    );
}

#[test]
fn default_directory_discovery_is_recursive_and_sorted() {
    let mut input = actor(&[]);
    input.default_directories = vec!["characters/_male".to_owned()];
    let catalog = build_actor_animation_catalog(
        "v1",
        "source",
        &[input],
        &[
            asset("meshes/characters/_male/z.kf", "z"),
            asset("meshes/characters/_male/idleanims/a.kf", "a"),
            asset("meshes/characters/_female/no.kf", "no"),
        ],
    );
    assert_eq!(
        catalog.animation_sets[0]
            .clips
            .iter()
            .map(|clip| clip.source_kf_path.as_str())
            .collect::<Vec<_>>(),
        [
            "meshes/characters/_male/idleanims/a.kf",
            "meshes/characters/_male/z.kf"
        ]
    );
}

#[test]
fn explicit_relative_paths_always_resolve_beside_the_actor_model() {
    let catalog = build_actor_animation_catalog(
        "v1",
        "source",
        &[actor(&["idle.kf"])],
        &[
            asset("meshes/idle.kf", "wrong"),
            asset("meshes/characters/_male/idle.kf", "right"),
        ],
    );
    assert_eq!(
        catalog.animation_sets[0].clips[0].source_kf_path,
        "meshes/characters/_male/idle.kf"
    );
    assert_eq!(
        catalog.animation_sets[0].clips[0].source_fingerprint,
        "right"
    );
}

#[test]
fn empty_and_partially_invalid_sets_are_retained() {
    let mut empty = actor(&[]);
    empty.default_directories = vec!["meshes/no-animations".to_owned()];
    let mut partial = actor(&["good.kf", "missing.kf"]);
    partial.reference_form_id = 3;
    let catalog = build_actor_animation_catalog(
        "v1",
        "source",
        &[empty, partial],
        &[asset("meshes/characters/_male/good.kf", "good")],
    );
    assert_eq!(catalog.actor_mappings.len(), 2);
    assert!(
        catalog
            .animation_sets
            .iter()
            .any(|set| set.clips.is_empty())
    );
    assert!(catalog.animation_sets.iter().any(|set| {
        set.clips
            .iter()
            .any(|clip| clip.status == PreparedActorAnimationClipStatus::Missing)
    }));
}

#[test]
fn normalized_clip_runtime_metadata_round_trips() {
    let clip = PreparedActorAnimationClip {
        name: "equip".into(),
        source_kf_path: "meshes/characters/_male/1hpequip.kf".into(),
        source_sequence_name: Some("Equip".into()),
        source_start_seconds: Some(0.25),
        source_end_seconds: Some(0.75),
        source_frequency: Some(1.0),
        source_phase: Some(0.0),
        loop_mode: PreparedActorAnimationLoopMode::Clamp,
        root_motion_policy: PreparedActorAnimationRootMotionPolicy::PreserveAuthored,
        accumulation_root: Some("Bip01".into()),
        required_targets: vec!["Bip01 R Hand".into(), "Weapon".into()],
        animated_targets: vec!["Bip01 R Hand".into()],
        controller_types: vec!["NiTransformController".into()],
        interpolator_types: vec!["NiTransformInterpolator".into()],
        text_keys: vec![PreparedActorAnimationTextKey {
            time_seconds: 0.5,
            value: "Attach".into(),
        }],
        ..Default::default()
    };

    let serialized = ron::to_string(&clip).unwrap();
    let decoded: PreparedActorAnimationClip = ron::from_str(&serialized).unwrap();
    assert_eq!(decoded, clip);
}
