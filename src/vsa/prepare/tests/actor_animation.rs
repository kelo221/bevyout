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

fn idle_record(form_id: u32, model_path: Option<&str>) -> crate::vsa::openmw_esm4::IdleRecord {
    crate::vsa::openmw_esm4::IdleRecord {
        form_id,
        model_path: model_path.map(str::to_owned),
        parent_form_id: Some(0x100),
        previous_sibling_form_id: None,
        group_section_raw: 0x47,
        group_section: 7,
        loop_min: 1,
        loop_max: 2,
        replay_delay_seconds: 3,
        flags: 0x11,
        ..Default::default()
    }
}

#[test]
fn authored_idle_preparation_keeps_roots_and_links_existing_set_clips() {
    let mut catalog = build_actor_animation_catalog(
        ACTOR_ANIMATION_CATALOG_REVISION,
        "source",
        &[ActorAnimationDiscoveryInput {
            reference_form_id: 1,
            base_form_id: 2,
            model_path: "meshes/characters/_male/skeleton.nif".into(),
            skeleton_path: "meshes/characters/_male/skeleton.nif".into(),
            skeleton_fingerprint: "skeleton".into(),
            explicit_kf_paths: vec!["idleanims/swatting.kf".into()],
            ..Default::default()
        }],
        &[ActorAnimationAsset {
            path: "meshes/characters/_male/idleanims/swatting.kf".into(),
            fingerprint: "swat".into(),
            state: ActorAnimationAssetState::Compatible,
        }],
    );
    let mut idles = HashMap::new();
    idles.insert(
        0x20,
        idle_record(0x20, Some("Characters\\_Male\\IdleAnims\\Swatting.KF")),
    );
    idles.insert(0x10, idle_record(0x10, None));

    prepare_actor_idle_definitions(&mut catalog, &idles);

    assert_eq!(
        catalog
            .idle_definitions
            .iter()
            .map(|definition| definition.form_id)
            .collect::<Vec<_>>(),
        [0x10, 0x20]
    );
    assert!(catalog.idle_definitions[0].clip_name.is_none());
    assert_eq!(
        catalog.idle_definitions[1].clip_name.as_deref(),
        Some("swatting")
    );
    assert_eq!(catalog.idle_definitions[1].group_section_raw, 0x47);
}

#[test]
fn authored_idle_path_is_not_added_as_a_second_conversion_clip() {
    let mut catalog = build_actor_animation_catalog(
        ACTOR_ANIMATION_CATALOG_REVISION,
        "source",
        &[ActorAnimationDiscoveryInput {
            reference_form_id: 1,
            base_form_id: 2,
            model_path: "meshes/characters/_male/skeleton.nif".into(),
            skeleton_path: "meshes/characters/_male/skeleton.nif".into(),
            skeleton_fingerprint: "skeleton".into(),
            default_directories: vec!["characters/_male".into()],
            ..Default::default()
        }],
        &[ActorAnimationAsset {
            path: "meshes/characters/_male/idleanims/swatting.kf".into(),
            fingerprint: "swat".into(),
            state: ActorAnimationAssetState::Compatible,
        }],
    );
    let mut idles = HashMap::new();
    idles.insert(
        0x20,
        idle_record(0x20, Some("meshes/characters/_male/idleanims/swatting.kf")),
    );

    prepare_actor_idle_definitions(&mut catalog, &idles);

    assert_eq!(catalog.animation_sets.len(), 1);
    assert_eq!(catalog.animation_sets[0].clips.len(), 1);
    assert_eq!(
        catalog.idle_definitions[0].clip_name.as_deref(),
        Some("swatting")
    );
}
