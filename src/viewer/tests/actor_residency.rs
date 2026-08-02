use super::*;

fn identity() -> ActorIdentity {
    resolve_actor_identity(0x20, 0x20, Some(0x20)).expect("matching actor identity")
}

fn owner(cell_form_id: u32, generation: u64) -> ActorResidencyOwner {
    ActorResidencyOwner::new(cell_form_id, generation).expect("valid resident owner")
}

fn plan(decision: ActorResidencyDecision) -> ActorResidencyPlan {
    match decision {
        ActorResidencyDecision::Apply(plan) => plan,
        ActorResidencyDecision::Reject(reason) => panic!("unexpected rejection: {reason:?}"),
    }
}

#[test]
fn prepared_runtime_and_saved_identity_share_one_canonical_reference_and_holder() {
    let identity = resolve_actor_identity(0x20, 0x20, Some(0x20)).unwrap();

    assert_eq!(identity.reference_form_id(), 0x20);
    assert_eq!(identity.state_reference_form_id(), 0x20);
    assert_eq!(
        identity.holder(),
        HolderId::Actor {
            reference_form_id: 0x20
        }
    );
    assert!(validate_canonical_holder(identity, identity.holder()).is_ok());
}

#[test]
fn mismatched_prepared_or_saved_identity_is_rejected_before_residency() {
    assert_eq!(
        resolve_actor_identity(0x20, 0x21, Some(0x20)),
        Err(ActorIdentityError::MismatchedReference {
            source: ActorIdentitySource::Prepared,
            expected: 0x20,
            actual: 0x21,
        })
    );
    assert_eq!(
        resolve_actor_identity(0x20, 0x20, Some(0x21)),
        Err(ActorIdentityError::MismatchedReference {
            source: ActorIdentitySource::SavedState,
            expected: 0x20,
            actual: 0x21,
        })
    );
}

#[test]
fn a_competing_canonical_holder_is_rejected_without_mutating_the_expected_holder() {
    let identity = identity();
    assert_eq!(
        validate_canonical_holder(
            identity,
            HolderId::Actor {
                reference_form_id: 0x21,
            },
        ),
        Err(ActorIdentityError::CanonicalHolderMismatch {
            expected: HolderId::Actor {
                reference_form_id: 0x20,
            },
            actual: HolderId::Actor {
                reference_form_id: 0x21,
            },
        })
    );
    assert_eq!(
        identity.holder(),
        HolderId::Actor {
            reference_form_id: 0x20
        }
    );
}

#[test]
fn an_unowned_actor_binds_once_to_one_resident_owner() {
    let identity = identity();
    let destination = owner(0x100, 1);
    let decision =
        decide_actor_residency(identity, &[], ActorResidencyRequest::Bind { destination });
    let plan = plan(decision);

    assert_eq!(plan.actor(), identity);
    assert_eq!(plan.holder(), identity.holder());
    assert_eq!(
        plan.transition(),
        ActorResidencyTransition::Bind {
            owner: destination,
            state: CanonicalStateTransition::EnsureAt {
                cell_form_id: 0x100,
            },
        }
    );
}

#[test]
fn binding_an_already_owned_actor_is_rejected_as_a_competing_owner() {
    let identity = identity();
    let current = owner(0x100, 1);
    let destination = owner(0x100, 1);

    assert_eq!(
        decide_actor_residency(
            identity,
            &[current],
            ActorResidencyRequest::Bind { destination },
        ),
        ActorResidencyDecision::Reject(ActorResidencyRejection::CompetingOwner { actual: current })
    );
}

#[test]
fn retaining_the_same_generation_is_the_only_non_mutating_resident_transition() {
    let identity = identity();
    let current = owner(0x100, 7);
    let plan = plan(decide_actor_residency(
        identity,
        &[current],
        ActorResidencyRequest::Retain { owner: current },
    ));

    assert_eq!(
        plan.transition(),
        ActorResidencyTransition::Retain {
            owner: current,
            state: CanonicalStateTransition::KeepAt {
                cell_form_id: 0x100,
            },
        }
    );
}

#[test]
fn a_generation_mismatch_is_stale_even_when_the_cell_matches() {
    let identity = identity();
    let actual = owner(0x100, 8);
    let requested = owner(0x100, 7);

    assert_eq!(
        decide_actor_residency(
            identity,
            &[actual],
            ActorResidencyRequest::Retain { owner: requested },
        ),
        ActorResidencyDecision::Reject(ActorResidencyRejection::StaleSource {
            expected: requested,
            actual: Some(actual),
        })
    );
}

#[test]
fn a_handoff_moves_one_canonical_state_record_and_keeps_the_same_actor_holder() {
    let identity = identity();
    let source = owner(0x100, 3);
    let destination = owner(0x200, 1);
    let plan = plan(decide_actor_residency(
        identity,
        &[source],
        ActorResidencyRequest::Handoff {
            source,
            destination,
        },
    ));

    assert_eq!(plan.actor(), identity);
    assert_eq!(
        plan.holder(),
        HolderId::Actor {
            reference_form_id: 0x20
        }
    );
    assert_eq!(
        plan.transition(),
        ActorResidencyTransition::Handoff {
            source,
            destination,
            state: CanonicalStateTransition::Move {
                from_cell_form_id: 0x100,
                to_cell_form_id: 0x200,
            },
        }
    );
}

#[test]
fn a_handoff_from_an_old_generation_is_rejected_and_cannot_repeat_after_commit() {
    let identity = identity();
    let old_source = owner(0x100, 3);
    let current_destination = owner(0x200, 1);

    assert_eq!(
        decide_actor_residency(
            identity,
            &[current_destination],
            ActorResidencyRequest::Handoff {
                source: old_source,
                destination: owner(0x300, 1),
            },
        ),
        ActorResidencyDecision::Reject(ActorResidencyRejection::StaleSource {
            expected: old_source,
            actual: Some(current_destination),
        })
    );
}

#[test]
fn a_same_cell_generation_change_is_restore_not_a_handoff() {
    let identity = identity();
    let source = owner(0x100, 3);
    let destination = owner(0x100, 4);

    assert_eq!(
        decide_actor_residency(
            identity,
            &[source],
            ActorResidencyRequest::Handoff {
                source,
                destination,
            },
        ),
        ActorResidencyDecision::Reject(ActorResidencyRejection::InvalidHandoff {
            source,
            destination,
        })
    );
}

#[test]
fn a_competing_owner_and_duplicate_live_owners_are_distinct_rejections() {
    let identity = identity();
    let first = owner(0x100, 1);
    let second = owner(0x200, 1);

    assert_eq!(
        decide_actor_residency(
            identity,
            &[first, second],
            ActorResidencyRequest::Retain { owner: first },
        ),
        ActorResidencyDecision::Reject(ActorResidencyRejection::DuplicateOwner { count: 2 })
    );
    assert_eq!(
        decide_actor_residency(
            identity,
            &[second],
            ActorResidencyRequest::Retain { owner: first },
        ),
        ActorResidencyDecision::Reject(ActorResidencyRejection::CompetingOwner { actual: second })
    );
}

#[test]
fn unload_preserves_the_canonical_record_and_restore_reprojects_it_once() {
    let identity = identity();
    let source = owner(0x100, 4);
    let unloaded = plan(decide_actor_residency(
        identity,
        &[source],
        ActorResidencyRequest::Unload { source },
    ));
    assert_eq!(
        unloaded.transition(),
        ActorResidencyTransition::Unload {
            owner: source,
            state: CanonicalStateTransition::KeepAt {
                cell_form_id: 0x100,
            },
        }
    );

    let destination = owner(0x100, 5);
    let restored = plan(decide_actor_residency(
        identity,
        &[],
        ActorResidencyRequest::Restore { destination },
    ));
    assert_eq!(restored.actor(), identity);
    assert_eq!(restored.holder(), identity.holder());
    assert_eq!(
        restored.transition(),
        ActorResidencyTransition::Restore {
            owner: destination,
            state: CanonicalStateTransition::KeepAt {
                cell_form_id: 0x100,
            },
        }
    );
}

#[test]
fn unloading_or_restoring_against_an_existing_owner_never_creates_a_second_projection() {
    let identity = identity();
    let actual = owner(0x200, 2);

    assert_eq!(
        decide_actor_residency(
            identity,
            &[actual],
            ActorResidencyRequest::Restore {
                destination: owner(0x100, 1)
            },
        ),
        ActorResidencyDecision::Reject(ActorResidencyRejection::CompetingOwner { actual })
    );
    assert_eq!(
        decide_actor_residency(
            identity,
            &[],
            ActorResidencyRequest::Unload {
                source: owner(0x100, 1),
            },
        ),
        ActorResidencyDecision::Reject(ActorResidencyRejection::StaleSource {
            expected: owner(0x100, 1),
            actual: None,
        })
    );
}

#[test]
fn identical_inputs_produce_identical_decisions_independent_of_call_order() {
    let identity = identity();
    let source = owner(0x100, 1);
    let destination = owner(0x200, 1);
    let request = ActorResidencyRequest::Handoff {
        source,
        destination,
    };
    let first = decide_actor_residency(identity, &[source], request);
    let second = decide_actor_residency(identity, &[source], request);
    assert_eq!(first, second);
}
