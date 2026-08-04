use super::*;

#[test]
fn unavailable_weapon_can_still_be_the_canonical_equipment_choice() {
    let selected = select_starting_weapon(&[
        ActorWeaponCandidate {
            item_form_id: 2,
            model_path: Some("pistol.nif".to_owned()),
            damage: 10,
            value: 100,
            available: true,
        },
        ActorWeaponCandidate {
            item_form_id: 1,
            model_path: Some("rifle.nif".to_owned()),
            damage: 20,
            value: 50,
            available: false,
        },
    ])
    .expect("a weapon is selected");

    assert_eq!(selected.item_form_id, 1);
    assert!(!selected.model_available);
}

#[test]
fn fallback_keeps_identity_when_no_visual_assets_are_supported() {
    let decision = resolve_actor_fallback(
        &ActorAppearanceAvailability {
            kind: ActorKind::Creature,
            base_form_id: 0x10,
            reference_form_id: 0x20,
            ..Default::default()
        },
        Vec::new(),
    );

    assert_eq!(decision.level, ActorFallbackLevel::ProxyMesh);
    assert_eq!(decision.proxy_kind, ActorProxyKind::Bounds);
    assert_eq!(decision.base_form_id, 0x10);
    assert_eq!(decision.reference_form_id, 0x20);
}

#[test]
fn compatible_facegen_keeps_authored_exact_assembly_without_missing_reason() {
    let decision = resolve_actor_fallback(
        &ActorAppearanceAvailability {
            kind: ActorKind::Humanoid,
            base_form_id: 0x10,
            reference_form_id: 0x20,
            exact_available: true,
            facegen: FaceGenAvailability::Compatible,
            ..Default::default()
        },
        Vec::new(),
    );
    assert_eq!(decision.level, ActorFallbackLevel::AuthoredExact);
    assert_eq!(decision.facegen_policy, FaceGenPolicy::Authored);
    assert!(decision.reasons.is_empty());
}

#[test]
fn blueprint_default_uses_neutral_root_scale() {
    assert_eq!(ActorAssemblyBlueprint::default().root_scale, 1.0);
}

#[test]
fn canonicalization_retains_distinct_left_and_right_eye_meshes() {
    let eye = |name: &str, path: &str| AssembledMeshPart {
        name: name.to_owned(),
        source_form_id: Some(0x4253),
        model_path: path.to_owned(),
        attachment_point: ActorAttachmentPoint::Head,
        role: ActorMeshRole::Eyes,
        is_visible: true,
    };
    let mut parts = vec![
        eye("right eye", "characters/eyes/eyeright.nif"),
        eye("left eye", "characters/eyes/eyeleft.nif"),
        eye("left eye duplicate", "CHARACTERS/EYES/EYELEFT.NIF"),
    ];

    canonicalize_mesh_parts(&mut parts);

    assert_eq!(parts.len(), 2);
    assert_eq!(parts[0].model_path, "characters/eyes/eyeleft.nif");
    assert_eq!(parts[1].model_path, "characters/eyes/eyeright.nif");
}
