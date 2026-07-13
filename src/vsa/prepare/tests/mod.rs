use super::super::openmw_esm4::EnableParentRecord;
use super::*;

#[test]
fn valid_cached_assets_are_reused_even_when_manifest_is_forced() {
    assert_eq!(
        asset_cache_decision(true, true, false),
        AssetCacheDecision::Reuse
    );
    assert_eq!(
        asset_cache_decision(true, true, true),
        AssetCacheDecision::RebuildRequested
    );
}

#[test]
fn missing_and_invalid_cached_assets_are_rebuilt() {
    assert_eq!(
        asset_cache_decision(false, false, false),
        AssetCacheDecision::BuildMissing
    );
    assert_eq!(
        asset_cache_decision(true, false, false),
        AssetCacheDecision::RebuildInvalid
    );
    assert_eq!(
        asset_cache_decision(true, false, true),
        AssetCacheDecision::RebuildInvalid
    );
}

#[test]
fn structural_static_paths_are_the_only_stair_support_candidates() {
    for path in [
        "architecture/megaton/interior/stairs.nif",
        "dungeons/vault/stairwell.nif",
        "landscape/rocks/cliffstep.nif",
    ] {
        assert!(is_structural_step_support(&PreparedSemantic::Static, path));
    }

    for path in [
        "furniture/chair01.nif",
        "clutter/office/shelf01.nif",
        "vehicles/car01.nif",
        "terminals/terminal01.nif",
    ] {
        assert!(!is_structural_step_support(&PreparedSemantic::Static, path));
    }
}

#[test]
fn non_static_semantics_never_become_stair_support() {
    for semantic in [
        PreparedSemantic::Furniture,
        PreparedSemantic::Door(PreparedDoor {
            lock_level: None,
            key_form_id: None,
            destination: None,
        }),
        PreparedSemantic::Activator,
        PreparedSemantic::Container,
        PreparedSemantic::Unsupported,
    ] {
        assert!(!is_structural_step_support(
            &semantic,
            "architecture/megaton/interior/stairs.nif"
        ));
    }
}

// --- Issue #38: PreparedRuntimeMutability classification -----------------
//
// Fixtures below are synthetic ReferenceRecord/PreparedSemantic values
// built directly (no real ESM bytes, no Bethesda-derived data), matching
// the level classify_runtime_mutability actually operates at.

fn reference_fixture(form_id: u32, enable_parent: Option<EnableParentRecord>) -> ReferenceRecord {
    ReferenceRecord {
        form_id,
        enable_parent,
        ..Default::default()
    }
}

// T38.1: synthetic record fixtures producing each of the four
// classifications; assert one placement of each class after prepare-time
// classification.
#[test]
fn synthetic_records_classify_into_all_four_mutability_classes() {
    // Immutable: plain static scenery, no enable-parent, no error.
    let statics = reference_fixture(0x10, None);
    assert_eq!(
        classify_runtime_mutability(&statics, &PreparedSemantic::Static, false),
        (PreparedRuntimeMutability::Immutable, None)
    );

    // EnableGroup: reference toggled by an enable-parent chain whose root
    // resolved successfully.
    let mut enabled_child = reference_fixture(
        0x20,
        Some(EnableParentRecord {
            parent_reference_form_id: 0x21,
            flags: 0,
        }),
    );
    enabled_child.enable_root_form_id = Some(0x21);
    assert_eq!(
        classify_runtime_mutability(&enabled_child, &PreparedSemantic::Static, false),
        (PreparedRuntimeMutability::EnableGroup, Some(0x21))
    );

    // ScriptAddressable: a known script-reachable record kind (a door).
    let door = reference_fixture(0x30, None);
    let door_semantic = PreparedSemantic::Door(PreparedDoor {
        lock_level: None,
        key_form_id: None,
        destination: None,
    });
    assert_eq!(
        classify_runtime_mutability(&door, &door_semantic, false),
        (PreparedRuntimeMutability::ScriptAddressable, None)
    );

    // Unknown: an unresolved placement (error present) must never be
    // silently treated as Immutable.
    let broken = reference_fixture(0x40, None);
    assert_eq!(
        classify_runtime_mutability(&broken, &PreparedSemantic::Static, true),
        (PreparedRuntimeMutability::Unknown, None)
    );
}

// T38.2: enable-parent fixture: children (however deep in the chain) share
// the same resolved parent root FormID in their EnableGroup classification.
#[test]
fn enable_parent_children_share_the_resolved_chain_root() {
    // root (0x1) <- mid (0x2, enabled by root) <- leaf (0x3, enabled by mid)
    let mut mid = reference_fixture(
        0x2,
        Some(EnableParentRecord {
            parent_reference_form_id: 0x1,
            flags: 0,
        }),
    );
    mid.enable_root_form_id = Some(0x1);
    let mut leaf = reference_fixture(
        0x3,
        Some(EnableParentRecord {
            parent_reference_form_id: 0x2,
            flags: 0,
        }),
    );
    leaf.enable_root_form_id = Some(0x1);

    let mid_result = classify_runtime_mutability(&mid, &PreparedSemantic::Static, false);
    let leaf_result = classify_runtime_mutability(&leaf, &PreparedSemantic::Static, false);
    assert_eq!(
        mid_result,
        (PreparedRuntimeMutability::EnableGroup, Some(0x1))
    );
    assert_eq!(
        leaf_result,
        (PreparedRuntimeMutability::EnableGroup, Some(0x1))
    );
    assert_eq!(mid_result.1, leaf_result.1, "children must share one root");

    // An enable-parent reference whose chain root could not be resolved
    // (cycle/unresolved XESP) is conservatively Unknown, never a guessed
    // EnableGroup root.
    let unresolved = reference_fixture(
        0x4,
        Some(EnableParentRecord {
            parent_reference_form_id: 0x999,
            flags: 0,
        }),
    );
    assert_eq!(
        classify_runtime_mutability(&unresolved, &PreparedSemantic::Static, false),
        (PreparedRuntimeMutability::Unknown, None)
    );
}

// T38.3: ambiguous fixture (record type not in the known-safe set) ->
// Unknown, never Immutable.
#[test]
fn unsupported_record_kind_classifies_as_unknown_not_immutable() {
    let reference = reference_fixture(0x50, None);
    let (mutability, root) =
        classify_runtime_mutability(&reference, &PreparedSemantic::Unsupported, false);
    assert_eq!(mutability, PreparedRuntimeMutability::Unknown);
    assert_eq!(root, None);
    assert_ne!(mutability, PreparedRuntimeMutability::Immutable);
}

// T38.6: classifying the same fixture set twice yields identical results,
// including the derived per-class summary counts (F38.4).
#[test]
fn classification_and_summary_are_deterministic_across_repeated_runs() {
    struct MutabilityCase {
        form_id: u32,
        enable_parent: Option<EnableParentRecord>,
        enable_root: Option<u32>,
        semantic: PreparedSemantic,
        has_error: bool,
    }

    fn fixture_set() -> Vec<PreparedPlacement> {
        let cases = [
            MutabilityCase {
                form_id: 0x1,
                enable_parent: None,
                enable_root: None,
                semantic: PreparedSemantic::Static,
                has_error: false,
            },
            MutabilityCase {
                form_id: 0x2,
                enable_parent: Some(EnableParentRecord {
                    parent_reference_form_id: 0x1,
                    flags: 0,
                }),
                enable_root: Some(0x1),
                semantic: PreparedSemantic::Static,
                has_error: false,
            },
            MutabilityCase {
                form_id: 0x3,
                enable_parent: None,
                enable_root: None,
                semantic: PreparedSemantic::Activator,
                has_error: false,
            },
            MutabilityCase {
                form_id: 0x4,
                enable_parent: None,
                enable_root: None,
                semantic: PreparedSemantic::Unsupported,
                has_error: false,
            },
            MutabilityCase {
                form_id: 0x5,
                enable_parent: None,
                enable_root: None,
                semantic: PreparedSemantic::Static,
                has_error: true,
            },
        ];
        cases
            .into_iter()
            .map(|case| {
                let mut reference = reference_fixture(case.form_id, case.enable_parent);
                reference.enable_root_form_id = case.enable_root;
                let (mutability, mutability_root_form_id) =
                    classify_runtime_mutability(&reference, &case.semantic, case.has_error);
                PreparedPlacement {
                    reference_form_id: case.form_id,
                    base_form_id: 0,
                    asset_path: None,
                    translation: [0.0; 3],
                    rotation_xyzw: [0.0, 0.0, 0.0, 1.0],
                    scale: 1.0,
                    error: case.has_error.then(|| "synthetic error".to_string()),
                    physics_asset_path: None,
                    physics_source: None,
                    physics_classification: PreparedPhysicsClassification::Static,
                    step_support: false,
                    mutability,
                    mutability_root_form_id,
                    reference_kind: "REFR".into(),
                    base_kind: "STAT".into(),
                    editor_id: None,
                    display_name: None,
                    count: 1,
                    semantic: case.semantic,
                    initially_enabled: true,
                    enable_parent: None,
                    owner_form_id: None,
                    owner_faction_rank: None,
                    inventory: Vec::new(),
                    audio: Default::default(),
                    ao_mode: "ao-none".into(),
                }
            })
            .collect()
    }

    let first = fixture_set();
    let second = fixture_set();
    let first_classes = first
        .iter()
        .map(|p| (p.reference_form_id, p.mutability, p.mutability_root_form_id))
        .collect::<Vec<_>>();
    let second_classes = second
        .iter()
        .map(|p| (p.reference_form_id, p.mutability, p.mutability_root_form_id))
        .collect::<Vec<_>>();
    assert_eq!(first_classes, second_classes);

    let first_summary = summarize_mutability(&first);
    let second_summary = summarize_mutability(&second);
    assert_eq!(first_summary, second_summary);
    assert_eq!(
        first_summary,
        PreparedMutabilitySummary {
            immutable: 1,
            enable_group: 1,
            script_addressable: 1,
            unknown: 2,
        }
    );
}

#[test]
fn non_static_physics_classifications_remove_stair_support() {
    assert!(retain_static_step_support(
        true,
        PreparedPhysicsClassification::Static
    ));
    assert!(!retain_static_step_support(
        true,
        PreparedPhysicsClassification::Kinematic
    ));
    assert!(!retain_static_step_support(
        true,
        PreparedPhysicsClassification::Dynamic
    ));
    assert!(!retain_static_step_support(
        false,
        PreparedPhysicsClassification::Static
    ));
}
