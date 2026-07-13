use super::*;

#[test]
fn bake_job_emits_resolved_cell_directional_light() {
    let lighting = PreparedCellLighting {
        directional_rgba: [0.5, 0.5, 0.5, 1.0],
        directional_fade: 2.0,
        ..Default::default()
    };
    assert_eq!(cell_directional_illuminance(&lighting), 10_000.0);
    assert_eq!(
        cell_directional_illuminance(&PreparedCellLighting::default()),
        0.0
    );
}

#[test]
fn only_static_semantics_are_batchable_without_changing_bake_inclusion() {
    fn placement(semantic: PreparedSemantic) -> PreparedPlacement {
        PreparedPlacement {
            reference_form_id: 1,
            base_form_id: 2,
            asset_path: Some("assets/test.glb".into()),
            translation: [0.0; 3],
            rotation_xyzw: [0.0, 0.0, 0.0, 1.0],
            scale: 1.0,
            error: None,
            physics_asset_path: None,
            physics_source: None,
            physics_classification: PreparedPhysicsClassification::Static,
            reference_kind: "REFR".into(),
            base_kind: "STAT".into(),
            editor_id: None,
            display_name: None,
            count: 1,
            semantic,
            initially_enabled: true,
            enable_parent: None,
            owner_form_id: None,
            owner_faction_rank: None,
            inventory: Vec::new(),
            audio: Default::default(),
            ao_mode: "ao-none".into(),
        }
    }

    let static_placement = placement(PreparedSemantic::Static);
    assert!(is_bake_static(&static_placement));
    assert!(is_batchable_static(&static_placement));

    let mut dynamic_placement = placement(PreparedSemantic::Static);
    dynamic_placement.physics_classification = PreparedPhysicsClassification::Dynamic;
    assert!(!is_bake_static(&dynamic_placement));
    assert!(!is_batchable_static(&dynamic_placement));

    for semantic in [
        PreparedSemantic::Furniture,
        PreparedSemantic::Npc(super::super::manifest::PreparedActor {
            base_template_form_id: None,
        }),
        PreparedSemantic::Creature(super::super::manifest::PreparedActor {
            base_template_form_id: None,
        }),
        PreparedSemantic::Unsupported,
    ] {
        let placement = placement(semantic);
        assert!(is_bake_static(&placement));
        assert!(!is_batchable_static(&placement));
    }
}
