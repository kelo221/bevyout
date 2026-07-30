use super::*;

#[test]
fn parent_directory_keeps_the_mesh_root() {
    assert_eq!(
        parent_directory("characters/_female/skeleton.nif"),
        "meshes/characters/_female"
    );
}

#[test]
fn disabled_conversion_is_diagnostic_not_failure() {
    let mut catalog = build_actor_animation_catalog(
        ACTOR_ANIMATION_CATALOG_REVISION,
        "source",
        &[ActorAnimationDiscoveryInput {
            reference_form_id: 1,
            base_form_id: 2,
            model_path: "meshes/characters/_male/skeleton.nif".into(),
            skeleton_path: "meshes/characters/_male/skeleton.nif".into(),
            skeleton_fingerprint: "skeleton".into(),
            explicit_kf_paths: vec!["idle.kf".into()],
            ..Default::default()
        }],
        &[ActorAnimationAsset {
            path: "meshes/characters/_male/idle.kf".into(),
            fingerprint: "idle".into(),
            state: ActorAnimationAssetState::Compatible,
        }],
    );
    let summary = convert_actor_animation_catalog(
        &mut catalog,
        &ActorAnimationConversionContext {
            converter: crate::converter_policy::ActorAnimationBackend::Disabled,
            converter_revision: actor_animation_converter_revision(
                crate::converter_policy::ActorAnimationBackend::Disabled,
            ),
            blender: None,
            data_root: Path::new("unused-data"),
            archives: &[],
            staging_dir: Path::new("unused-staging"),
            assets_dir: Path::new("unused-assets"),
            rebuild: false,
        },
    )
    .expect("disabled conversion must not touch tools or files");

    assert_eq!(summary, ActorAnimationConversionSummary::default());
    assert_eq!(
        catalog.animation_sets[0].clips[0].status,
        PreparedActorAnimationClipStatus::NotConverted
    );
    assert!(
        catalog.animation_sets[0]
            .diagnostics
            .iter()
            .any(|diagnostic| {
                diagnostic.code == "conversion_not_requested" && diagnostic.severity == "info"
            })
    );
}
