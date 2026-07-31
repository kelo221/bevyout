use super::*;

fn reference(reference_form_id: u32, base: u32, position: [f32; 3]) -> ResolvedReference {
    ResolvedReference {
        reference_form_id,
        base_form_id: base,
        cell_form_id: 0x1000,
        position,
        entity: Some(u64::from(reference_form_id)),
        linked_reference: None,
    }
}

fn context_with(references: Vec<ResolvedReference>) -> ResolutionContext {
    let mut bases: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut refs = HashMap::new();
    for reference in references {
        bases
            .entry(reference.base_form_id)
            .or_default()
            .push(reference.reference_form_id);
        refs.insert(reference.reference_form_id, reference);
    }
    ResolutionContext {
        current_cell_form_id: 0x1000,
        actor_position: [0.0, 0.0, 0.0],
        references: refs,
        bases,
        ..ResolutionContext::default()
    }
}

fn location(location_type: u32, form_id: Option<u32>, radius: i32) -> PackageLocation {
    PackageLocation {
        location_type,
        form_id,
        raw_value: form_id.unwrap_or_default(),
        radius,
    }
}

#[test]
fn near_reference_resolves_to_that_references_position_and_entity() {
    let context = context_with(vec![reference(0x20, 0xAA, [3.0, 4.0, 5.0])]);
    let resolved = resolve_location(&location(0, Some(0x20), 7), &context).unwrap();
    assert_eq!(resolved.position, [3.0, 4.0, 5.0]);
    assert_eq!(resolved.entity, Some(0x20));
    assert_eq!(resolved.radius, 7.0);
    assert_eq!(resolved.source, ResolutionSource::Reference(0x20));
}

#[test]
fn near_reference_missing_reference_is_a_diagnostic_not_a_panic() {
    let context = context_with(vec![]);
    let error = resolve_location(&location(0, Some(0xDEAD), 0), &context).unwrap_err();
    assert!(error.message.contains("0000dead"));
}

#[test]
fn in_cell_resolves_only_for_the_current_cell() {
    let context = context_with(vec![]);
    let resolved = resolve_location(&location(1, Some(0x1000), 0), &context).unwrap();
    assert_eq!(resolved.source, ResolutionSource::InCell(0x1000));
    assert_eq!(resolved.position, context.actor_position);

    let other = resolve_location(&location(1, Some(0x2000), 0), &context).unwrap_err();
    assert!(other.message.contains("not the current cell"));
}

#[test]
fn near_current_location_is_the_actor_position() {
    let mut context = context_with(vec![]);
    context.actor_position = [1.0, 2.0, 3.0];
    let resolved = resolve_location(&location(2, None, 0), &context).unwrap();
    assert_eq!(resolved.position, [1.0, 2.0, 3.0]);
    assert_eq!(resolved.source, ResolutionSource::ActorPosition);
}

#[test]
fn near_editor_location_needs_an_authored_point() {
    let mut context = context_with(vec![]);
    assert!(resolve_location(&location(3, None, 0), &context).is_err());
    context.actor_editor_location = Some([9.0, 9.0, 9.0]);
    let resolved = resolve_location(&location(3, None, 0), &context).unwrap();
    assert_eq!(resolved.position, [9.0, 9.0, 9.0]);
    assert_eq!(resolved.source, ResolutionSource::EditorLocation);
}

#[test]
fn object_id_location_picks_the_nearest_instance() {
    let context = context_with(vec![
        reference(0x20, 0xAA, [10.0, 0.0, 0.0]),
        reference(0x21, 0xAA, [2.0, 0.0, 0.0]),
    ]);
    let resolved = resolve_location(&location(4, Some(0xAA), 0), &context).unwrap();
    assert_eq!(resolved.position, [2.0, 0.0, 0.0]);
    assert_eq!(resolved.source, ResolutionSource::NearestOfBase(0xAA));
}

#[test]
fn object_id_location_with_no_instance_is_a_diagnostic() {
    let context = context_with(vec![]);
    let error = resolve_location(&location(4, Some(0xAA), 0), &context).unwrap_err();
    assert!(
        error
            .message
            .contains("no present reference of base 000000aa")
    );
}

#[test]
fn object_type_location_is_deterministically_unresolvable() {
    let context = context_with(vec![]);
    let error = resolve_location(&location(5, None, 0), &context).unwrap_err();
    assert!(error.message.contains("form-type index"));
}

#[test]
fn near_linked_reference_resolves_through_the_context() {
    let mut context = context_with(vec![reference(0x30, 0xBB, [5.0, 6.0, 7.0])]);
    context.linked_reference = Some(0x30);
    let resolved = resolve_location(&location(6, None, 0), &context).unwrap();
    assert_eq!(resolved.position, [5.0, 6.0, 7.0]);
    assert_eq!(resolved.source, ResolutionSource::LinkedReference(0x30));
}

#[test]
fn at_package_location_uses_the_anchor() {
    let mut context = context_with(vec![]);
    context.package_location_anchor = Some([-1.0, -2.0, -3.0]);
    let resolved = resolve_location(&location(7, None, 0), &context).unwrap();
    assert_eq!(resolved.position, [-1.0, -2.0, -3.0]);
    assert_eq!(resolved.source, ResolutionSource::PackageAnchor);
}

#[test]
fn unsupported_location_type_is_diagnosed() {
    let context = context_with(vec![]);
    let error = resolve_location(&location(99, None, 0), &context).unwrap_err();
    assert!(error.message.contains("unsupported location type 99"));
}

#[test]
fn specific_reference_target_resolves() {
    let context = context_with(vec![reference(0x40, 0xCC, [1.0, 1.0, 1.0])]);
    let target = PackageTarget {
        target_type: 0,
        form_id: Some(0x40),
        raw_value: 0x40,
        count_or_distance: 5,
    };
    let resolved = resolve_target(&target, &context).unwrap();
    assert_eq!(resolved.position, [1.0, 1.0, 1.0]);
    assert_eq!(resolved.radius, 5.0);
    assert_eq!(resolved.source, ResolutionSource::Reference(0x40));
}

#[test]
fn object_id_target_picks_nearest() {
    let context = context_with(vec![
        reference(0x50, 0xDD, [8.0, 0.0, 0.0]),
        reference(0x51, 0xDD, [1.0, 0.0, 0.0]),
    ]);
    let target = PackageTarget {
        target_type: 1,
        form_id: Some(0xDD),
        raw_value: 0xDD,
        count_or_distance: 0,
    };
    let resolved = resolve_target(&target, &context).unwrap();
    assert_eq!(resolved.position, [1.0, 0.0, 0.0]);
}

#[test]
fn follow_target_resolves_through_context() {
    let mut context = context_with(vec![reference(0x60, 0xEE, [2.0, 2.0, 2.0])]);
    context.follow_target = Some(0x60);
    let target = PackageTarget {
        target_type: 3,
        form_id: None,
        raw_value: 0,
        count_or_distance: 0,
    };
    let resolved = resolve_target(&target, &context).unwrap();
    assert_eq!(resolved.position, [2.0, 2.0, 2.0]);
    assert_eq!(resolved.source, ResolutionSource::FollowTarget(0x60));
}

#[test]
fn object_type_target_is_deterministically_unresolvable() {
    let context = context_with(vec![]);
    let target = PackageTarget {
        target_type: 2,
        form_id: None,
        raw_value: 42,
        count_or_distance: 0,
    };
    let error = resolve_target(&target, &context).unwrap_err();
    assert!(error.message.contains("form-type index"));
}

#[test]
fn unsupported_target_type_is_diagnosed() {
    let context = context_with(vec![]);
    let target = PackageTarget {
        target_type: 77,
        ..PackageTarget::default()
    };
    let error = resolve_target(&target, &context).unwrap_err();
    assert!(error.message.contains("unsupported target type 77"));
}

/// Like `reference`, but carries an `XLKR` link to the next marker in a
/// patrol chain (issue #213).
fn linked_marker(
    reference_form_id: u32,
    position: [f32; 3],
    linked_reference: Option<u32>,
) -> ResolvedReference {
    ResolvedReference {
        linked_reference,
        ..reference(reference_form_id, 0x34, position)
    }
}

#[test]
fn linked_reference_chain_walks_a_single_marker() {
    let context = context_with(vec![linked_marker(0x10, [1.0, 2.0, 3.0], None)]);
    let points = linked_reference_chain(&context, 0x10);
    assert_eq!(points.len(), 1);
    assert_eq!(points[0].position, [1.0, 2.0, 3.0]);
    assert_eq!(points[0].source, ResolutionSource::LinkedReference(0x10));
}

#[test]
fn linked_reference_chain_walks_a_three_marker_cycle_and_terminates() {
    // A -> B -> C -> A: the chain must yield exactly the three markers,
    // in order, then stop instead of looping forever.
    let context = context_with(vec![
        linked_marker(0xA, [0.0, 0.0, 0.0], Some(0xB)),
        linked_marker(0xB, [1.0, 0.0, 0.0], Some(0xC)),
        linked_marker(0xC, [2.0, 0.0, 0.0], Some(0xA)),
    ]);
    let points = linked_reference_chain(&context, 0xA);
    assert_eq!(
        points
            .iter()
            .map(|point| point.position)
            .collect::<Vec<_>>(),
        vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [2.0, 0.0, 0.0]]
    );
}

#[test]
fn linked_reference_chain_terminates_cleanly_on_a_broken_or_missing_link() {
    // The start reference itself is not present in the runtime at all.
    let empty_context = context_with(vec![]);
    assert_eq!(linked_reference_chain(&empty_context, 0x99), Vec::new());

    // A real first marker whose own link points at a FormID the context
    // does not know about: one waypoint, then a clean stop.
    let dangling_context = context_with(vec![linked_marker(0x10, [5.0, 5.0, 5.0], Some(0xDEAD))]);
    let points = linked_reference_chain(&dangling_context, 0x10);
    assert_eq!(points.len(), 1);
    assert_eq!(points[0].position, [5.0, 5.0, 5.0]);
}

#[test]
fn resolve_family_point_falls_back_from_the_preferred_slot() {
    let context = context_with(vec![reference(0x70, 0xFF, [7.0, 0.0, 0.0])]);
    let location = location(0, Some(0x70), 0);
    let target = PackageTarget {
        target_type: 2,
        ..PackageTarget::default()
    };

    let direct = resolve_family_point(Some(location), Some(target), &context, false).unwrap();
    assert_eq!(direct.position, [7.0, 0.0, 0.0]);
    let fallback = resolve_family_point(Some(location), Some(target), &context, true).unwrap();
    assert_eq!(fallback.position, [7.0, 0.0, 0.0]);
}

#[test]
fn resolve_family_point_reports_the_preferred_diagnostic() {
    let context = context_with(vec![]);
    let location = location(3, None, 0);
    let target = PackageTarget {
        target_type: 2,
        ..PackageTarget::default()
    };
    let error = resolve_family_point(Some(location), Some(target), &context, false).unwrap_err();
    assert!(error.message.contains("editor location") || error.message.contains("authored"));
}

#[test]
fn resolve_family_point_without_slots_is_a_diagnostic() {
    let error = resolve_family_point(None, None, &context_with(vec![]), false).unwrap_err();
    assert!(error.message.contains("no resolvable location or target"));
}
