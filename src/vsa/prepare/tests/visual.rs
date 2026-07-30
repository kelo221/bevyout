use super::*;

#[test]
fn issue_format_is_stable_and_uses_hex_form_ids() {
    let issue = PreparedVisualIssue {
        code: "unreviewed_root_transform".into(),
        severity: "warning".into(),
        model_path: "architecture/test.nif".into(),
        base_form_ids: vec![0x9cffc],
        reference_form_ids: vec![0x2943e, 0xab30d],
        message: "architecture/test.nif needs review".into(),
    };
    assert_eq!(
        format_visual_issue(&issue),
        "WARNING [unreviewed_root_transform] architecture/test.nif needs review (bases: 0009CFFC; references: 0002943E, 000AB30D)"
    );
}

#[test]
fn visual_issues_warn_normally_and_fail_strict_preparation() {
    let issues = vec![PreparedVisualIssue {
        code: "unreviewed_root_transform".into(),
        severity: "warning".into(),
        model_path: "architecture/test.nif".into(),
        base_form_ids: vec![1],
        reference_form_ids: vec![2],
        message: "review root".into(),
    }];
    enforce_strict_visual_completeness(false, 0, &issues).unwrap();
    let error = enforce_strict_visual_completeness(true, 0, &issues)
        .unwrap_err()
        .to_string();
    assert!(error.contains("1 visual completeness issue"));
}

#[test]
fn actor_fallback_issue_keeps_source_resolved_and_reference_identity() {
    let mut placement: PreparedPlacement = ron::from_str(
        r#"(
                reference_form_id: 48,
                base_form_id: 16,
                asset_path: None,
                translation: (0.0, 0.0, 0.0),
                rotation_xyzw: (0.0, 0.0, 0.0, 1.0),
                scale: 1.0,
                error: None,
                semantic: Npc((base_template_form_id: None, assembly: None)),
            )"#,
    )
    .expect("synthetic actor placement");
    let PreparedSemantic::Npc(actor) = &mut placement.semantic else {
        unreachable!("test semantic is an NPC")
    };
    actor.assembly = Some(bevyout_core::actor::ActorAssemblyBlueprint {
        source_base_form_id: 0x10,
        resolved_base_form_id: 0x20,
        reference_form_id: 0x30,
        fallback: bevyout_core::actor::ActorFallbackDecision {
            reasons: vec![bevyout_core::actor::ActorFallbackReason::MissingFaceGen],
            ..Default::default()
        },
        ..Default::default()
    });

    let issues = actor_fallback_visual_issues(&[placement]);
    assert_eq!(issues.len(), 1);
    assert_eq!(issues[0].code, "actor_missing_facegen");
    assert_eq!(issues[0].base_form_ids, vec![0x10, 0x20]);
    assert_eq!(issues[0].reference_form_ids, vec![0x30]);
    assert!(issues[0].message.contains("tier=AuthoredExact"));
}
