use super::*;

fn transform(translation: [f32; 3]) -> TransformDelta {
    TransformDelta {
        translation,
        rotation_xyzw: [0.0, 0.0, 0.0, 1.0],
        scale: [1.0, 1.0, 1.0],
    }
}

// T60.1: a placement with no enable_parent uses its own baseline.
#[test]
fn a_standalone_placement_uses_its_own_baseline() {
    let placements = [PlacementInfo {
        reference_form_id: 0x10,
        initially_enabled: false,
        enable_parent: None,
    }];
    let effective = resolve_effective_enabled(&placements, &HashMap::new());
    assert!(!effective[&0x10]);
}

// T60.1: parent disabled -> child hidden.
#[test]
fn a_disabled_parent_hides_its_child() {
    let placements = [
        PlacementInfo {
            reference_form_id: 0x1,
            initially_enabled: false,
            enable_parent: None,
        },
        PlacementInfo {
            reference_form_id: 0x2,
            initially_enabled: true, // ignored: enable_parent overrides it
            enable_parent: Some(EnableParentLink {
                reference_form_id: 0x1,
                inverted: false,
            }),
        },
    ];
    let effective = resolve_effective_enabled(&placements, &HashMap::new());
    assert!(!effective[&0x1]);
    assert!(!effective[&0x2]);
}

// T60.1: the inverted flag is honored when the manifest models it.
#[test]
fn an_inverted_enable_parent_link_flips_the_parent_state() {
    let placements = [
        PlacementInfo {
            reference_form_id: 0x1,
            initially_enabled: false,
            enable_parent: None,
        },
        PlacementInfo {
            reference_form_id: 0x2,
            initially_enabled: true,
            enable_parent: Some(EnableParentLink {
                reference_form_id: 0x1,
                inverted: true,
            }),
        },
    ];
    let effective = resolve_effective_enabled(&placements, &HashMap::new());
    assert!(!effective[&0x1]);
    assert!(effective[&0x2]);
}

// T60.1: an explicit delta on the child overrides both its own baseline
// and the parent cascade.
#[test]
fn a_delta_overrides_the_enable_parent_cascade() {
    let placements = [
        PlacementInfo {
            reference_form_id: 0x1,
            initially_enabled: true,
            enable_parent: None,
        },
        PlacementInfo {
            reference_form_id: 0x2,
            initially_enabled: true,
            enable_parent: Some(EnableParentLink {
                reference_form_id: 0x1,
                inverted: false,
            }),
        },
    ];
    let deltas = HashMap::from([(
        0x2,
        ReferenceDelta {
            enabled: Some(false),
            ..Default::default()
        },
    )]);
    let effective = resolve_effective_enabled(&placements, &deltas);
    assert!(effective[&0x1]);
    assert!(!effective[&0x2]);
}

// F60.1(a): `enable_root_form_id` redirects resolution to another
// reference's effective state even when the manifest itself has no
// `enable_parent` link for this placement.
#[test]
fn enable_root_form_id_redirects_resolution_without_a_manifest_link() {
    let placements = [
        PlacementInfo {
            reference_form_id: 0x1,
            initially_enabled: false,
            enable_parent: None,
        },
        PlacementInfo {
            reference_form_id: 0x2,
            initially_enabled: true,
            enable_parent: None,
        },
    ];
    let deltas = HashMap::from([(
        0x2,
        ReferenceDelta {
            enable_root_form_id: Some(0x1),
            ..Default::default()
        },
    )]);
    let effective = resolve_effective_enabled(&placements, &deltas);
    assert!(!effective[&0x2]);
}

fn placement_info(reference_form_id: u32) -> PlacementInfo {
    PlacementInfo {
        reference_form_id,
        initially_enabled: true,
        enable_parent: None,
    }
}

// T60.2: a moved dynamic body produces a transform+body delta.
#[test]
fn a_moved_dynamic_body_produces_transform_and_body_deltas() {
    let baselines = [BaselinePlacement {
        reference_form_id: 0x100,
        transform: transform([0.0, 0.0, 0.0]),
    }];
    let snapshots = [RuntimeSnapshot {
        reference_form_id: 0x100,
        present: true,
        transform: Some(transform([1.0, 0.0, 0.0])),
        activated: None,
        body: Some(BodyDelta {
            linear_velocity: [0.5, 0.0, 0.0],
            angular_velocity: [0.0, 0.0, 0.0],
            sleeping: false,
        }),
    }];
    let deltas = diff_capture(&baselines, &snapshots);
    let delta = deltas.get(&0x100).expect("expected a delta");
    assert_eq!(delta.transform, Some(transform([1.0, 0.0, 0.0])));
    assert!(delta.body.is_some());
}

// T60.2: an opened container produces an activated delta.
#[test]
fn an_open_container_produces_an_activated_delta() {
    let baselines = [BaselinePlacement {
        reference_form_id: 0x200,
        transform: transform([0.0, 0.0, 0.0]),
    }];
    let snapshots = [RuntimeSnapshot {
        reference_form_id: 0x200,
        present: true,
        transform: Some(transform([0.0, 0.0, 0.0])),
        activated: Some(true),
        body: None,
    }];
    let deltas = diff_capture(&baselines, &snapshots);
    assert_eq!(deltas[&0x200].activated, Some(true));
}

// T60.2: a taken pickup produces a deleted delta.
#[test]
fn a_taken_pickup_produces_a_deleted_delta() {
    let baselines = [BaselinePlacement {
        reference_form_id: 0x300,
        transform: transform([0.0, 0.0, 0.0]),
    }];
    let snapshots = [RuntimeSnapshot {
        reference_form_id: 0x300,
        present: false,
        transform: None,
        activated: None,
        body: None,
    }];
    let deltas = diff_capture(&baselines, &snapshots);
    assert!(deltas[&0x300].deleted);
}

// T60.2: an untouched reference produces no delta at all.
#[test]
fn an_untouched_reference_produces_no_delta() {
    let baselines = [BaselinePlacement {
        reference_form_id: 0x400,
        transform: transform([0.0, 0.0, 0.0]),
    }];
    let snapshots = [RuntimeSnapshot {
        reference_form_id: 0x400,
        present: true,
        transform: Some(transform([0.0, 0.0, 0.0])),
        activated: None,
        body: Some(BodyDelta::default()),
    }];
    let deltas = diff_capture(&baselines, &snapshots);
    assert!(!deltas.contains_key(&0x400));
}

// T60.3: capturing an applied delta and re-applying it reproduces the
// same visibility/transform/activated/body decisions (pure round trip).
#[test]
fn capture_then_apply_round_trips_a_moved_open_reference() {
    let placements = [placement_info(0x500)];
    let deltas = HashMap::from([(
        0x500,
        ReferenceDelta {
            transform: Some(transform([2.0, 3.0, 4.0])),
            activated: Some(true),
            body: Some(BodyDelta {
                linear_velocity: [0.1, 0.0, 0.0],
                angular_velocity: [0.0, 0.0, 0.0],
                sleeping: false,
            }),
            ..Default::default()
        },
    )]);
    let applications = plan_apply(&placements, &deltas);
    let application = applications
        .iter()
        .find(|application| application.reference_form_id == 0x500)
        .unwrap();
    assert_eq!(application.visibility, VisibilityDecision::Visible);

    let baselines = [BaselinePlacement {
        reference_form_id: 0x500,
        transform: transform([0.0, 0.0, 0.0]),
    }];
    let snapshots = [RuntimeSnapshot {
        reference_form_id: 0x500,
        present: true,
        transform: application.transform,
        activated: application.activated,
        body: application.body,
    }];
    let recaptured = diff_capture(&baselines, &snapshots);
    assert_eq!(recaptured.get(&0x500), Some(&deltas[&0x500]));
}

// T60.2/T60.3: a deleted reference is always hidden regardless of any
// enable-parent chain, and produces the same deleted-only delta.
#[test]
fn a_deleted_reference_is_hidden_even_when_its_enable_parent_is_active() {
    let placements = [
        placement_info(0x1),
        PlacementInfo {
            reference_form_id: 0x2,
            initially_enabled: true,
            enable_parent: Some(EnableParentLink {
                reference_form_id: 0x1,
                inverted: false,
            }),
        },
    ];
    let deltas = HashMap::from([(
        0x2,
        ReferenceDelta {
            deleted: true,
            ..Default::default()
        },
    )]);
    let applications = plan_apply(&placements, &deltas);
    let application = applications
        .iter()
        .find(|application| application.reference_form_id == 0x2)
        .unwrap();
    assert_eq!(application.visibility, VisibilityDecision::Hidden);
}

// Issue #76 (T76.2): a container whose live stacks match the manifest
// baseline and was never resolved produces no delta at all.
#[test]
fn a_baseline_identical_container_captures_no_delta() {
    let baselines = HashMap::from([(
        0x700,
        ContainerBaseline {
            stacks: vec![(0x10, 3)],
        },
    )]);
    let snapshots = HashMap::from([(
        0x700,
        ContainerSnapshot {
            stacks: vec![(0x10, 3)],
            resolved: false,
        },
    )]);
    let deltas = diff_capture_containers(&baselines, &snapshots);
    assert!(!deltas.contains_key(&0x700));
}

// T76.2: a looted container (stacks differ from baseline, no roll
// happened) produces a minimal inventory-only delta.
#[test]
fn a_looted_container_captures_a_minimal_inventory_delta() {
    let baselines = HashMap::from([(
        0x701,
        ContainerBaseline {
            stacks: vec![(0x10, 3)],
        },
    )]);
    let snapshots = HashMap::from([(
        0x701,
        ContainerSnapshot {
            stacks: vec![(0x10, 1)],
            resolved: false,
        },
    )]);
    let deltas = diff_capture_containers(&baselines, &snapshots);
    let delta = deltas.get(&0x701).expect("expected a delta");
    assert_eq!(delta.inventory, Some(vec![(0x10, 1)]));
    assert_eq!(delta.leveled_resolved, None);
}

// T76.2: a container whose leveled roll happened but whose resulting
// stacks still match the baseline (e.g. chance-none) produces an
// LVLR-only delta.
#[test]
fn a_resolved_but_baseline_identical_container_captures_an_lvlr_only_delta() {
    let baselines = HashMap::from([(
        0x702,
        ContainerBaseline {
            stacks: vec![(0x10, 3)],
        },
    )]);
    let snapshots = HashMap::from([(
        0x702,
        ContainerSnapshot {
            stacks: vec![(0x10, 3)],
            resolved: true,
        },
    )]);
    let deltas = diff_capture_containers(&baselines, &snapshots);
    let delta = deltas.get(&0x702).expect("expected a delta");
    assert_eq!(delta.inventory, None);
    assert_eq!(delta.leveled_resolved, Some(true));
}

// T76.2: duplicate FormIDs and zero-count entries normalize away before
// comparison, so a container that nets to the baseline after merging
// still captures no delta.
#[test]
fn container_stacks_normalize_before_comparison() {
    let baselines = HashMap::from([(
        0x703,
        ContainerBaseline {
            stacks: vec![(0x10, 3)],
        },
    )]);
    let snapshots = HashMap::from([(
        0x703,
        ContainerSnapshot {
            stacks: vec![(0x10, 5), (0x10, -2), (0x20, 1), (0x20, -1)],
            resolved: false,
        },
    )]);
    let deltas = diff_capture_containers(&baselines, &snapshots);
    assert!(!deltas.contains_key(&0x703));
}

// Issue #76 (F76.3): a container with a saved delta is seeded with the
// saved stacks and resolved marker before first activation.
#[test]
fn apply_seeds_a_container_from_its_delta() {
    let baselines = HashMap::from([(
        0x800,
        ContainerBaseline {
            stacks: vec![(0x10, 3)],
        },
    )]);
    let deltas = HashMap::from([(
        0x800,
        ContainerDelta {
            inventory: Some(vec![(0x10, 1)]),
            leveled_resolved: Some(true),
        },
    )]);
    let seeded = plan_apply_containers(&baselines, &deltas);
    let snapshot = seeded.get(&0x800).expect("expected a seeded snapshot");
    assert_eq!(snapshot.stacks, vec![(0x10, 1)]);
    assert!(snapshot.resolved);
}

// F76.3: a container with no saved delta is not seeded at all, so it
// stays unresolved and rolls on first open.
#[test]
fn apply_does_not_seed_an_unopened_container() {
    let baselines = HashMap::from([(
        0x801,
        ContainerBaseline {
            stacks: vec![(0x10, 3)],
        },
    )]);
    let seeded = plan_apply_containers(&baselines, &HashMap::new());
    assert!(!seeded.contains_key(&0x801));
}

// F76.3: an LVLR-only delta (resolved, but stacks matched baseline at
// capture) seeds the container with the baseline stacks and
// `resolved = true`.
#[test]
fn apply_seeds_baseline_stacks_for_an_lvlr_only_delta() {
    let baselines = HashMap::from([(
        0x802,
        ContainerBaseline {
            stacks: vec![(0x10, 3)],
        },
    )]);
    let deltas = HashMap::from([(
        0x802,
        ContainerDelta {
            inventory: None,
            leveled_resolved: Some(true),
        },
    )]);
    let seeded = plan_apply_containers(&baselines, &deltas);
    let snapshot = seeded.get(&0x802).expect("expected a seeded snapshot");
    assert_eq!(snapshot.stacks, vec![(0x10, 3)]);
    assert!(snapshot.resolved);
}
