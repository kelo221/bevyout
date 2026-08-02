//! Pure actor residency and canonical handoff decisions.
//!
//! This module is an adapter policy, not an actor store. The caller supplies
//! the owners it observed from the live resident/ECS projection and applies
//! the returned transition to the existing authorities: `ActorRuntime`,
//! `ActorStateRuntime`, `ActiveSaveState`, and the canonical item ledger.
//! Nothing here owns an `ActorInstanceState`, a component, or a collection of
//! resident actors.

// W3-C is intentionally blocked until gate #10. Keep the pure seam warning-
// free while it has no runtime caller yet; the integration lane will consume
// these items rather than adding another authority.
#![allow(dead_code)]

use bevyout_core::item_transaction::HolderId;

/// The only identity used for actor persistence and canonical item ownership.
///
/// Prepared base/assembly data remains presentation and definition data. A
/// reference FormID is what distinguishes two placements of the same base
/// actor, matching both `ActorInstanceState.reference_form_id` and
/// `HolderId::Actor`.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub(crate) struct ActorIdentity {
    reference_form_id: u32,
}

impl ActorIdentity {
    pub(crate) const fn reference_form_id(self) -> u32 {
        self.reference_form_id
    }

    pub(crate) const fn state_reference_form_id(self) -> u32 {
        self.reference_form_id
    }

    pub(crate) const fn holder(self) -> HolderId {
        HolderId::Actor {
            reference_form_id: self.reference_form_id,
        }
    }
}

/// Which existing authority supplied an actor reference during identity
/// validation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ActorIdentitySource {
    Runtime,
    Prepared,
    SavedState,
}

/// Identity failures are rejected before any residency transition is
/// considered. This prevents a stale prepared catalog or saved record from
/// creating a second canonical actor identity.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ActorIdentityError {
    InvalidReference {
        source: ActorIdentitySource,
    },
    MismatchedReference {
        source: ActorIdentitySource,
        expected: u32,
        actual: u32,
    },
    CanonicalHolderMismatch {
        expected: HolderId,
        actual: HolderId,
    },
}

/// Adapts the reference IDs already carried by `ActorRuntime`, the prepared
/// actor definition, and an optional `ActorInstanceState` into one canonical
/// identity. `None` is valid only for a not-yet-seeded save record.
pub(crate) fn resolve_actor_identity(
    runtime_reference_form_id: u32,
    prepared_reference_form_id: u32,
    saved_reference_form_id: Option<u32>,
) -> Result<ActorIdentity, ActorIdentityError> {
    if runtime_reference_form_id == 0 {
        return Err(ActorIdentityError::InvalidReference {
            source: ActorIdentitySource::Runtime,
        });
    }
    if prepared_reference_form_id == 0 {
        return Err(ActorIdentityError::InvalidReference {
            source: ActorIdentitySource::Prepared,
        });
    }
    if prepared_reference_form_id != runtime_reference_form_id {
        return Err(ActorIdentityError::MismatchedReference {
            source: ActorIdentitySource::Prepared,
            expected: runtime_reference_form_id,
            actual: prepared_reference_form_id,
        });
    }
    if let Some(saved_reference_form_id) = saved_reference_form_id {
        if saved_reference_form_id == 0 {
            return Err(ActorIdentityError::InvalidReference {
                source: ActorIdentitySource::SavedState,
            });
        }
        if saved_reference_form_id != runtime_reference_form_id {
            return Err(ActorIdentityError::MismatchedReference {
                source: ActorIdentitySource::SavedState,
                expected: runtime_reference_form_id,
                actual: saved_reference_form_id,
            });
        }
    }
    Ok(ActorIdentity {
        reference_form_id: runtime_reference_form_id,
    })
}

/// Checks the observed canonical item holder without changing the ledger.
pub(crate) fn validate_canonical_holder(
    identity: ActorIdentity,
    observed: HolderId,
) -> Result<(), ActorIdentityError> {
    let expected = identity.holder();
    (observed == expected)
        .then_some(())
        .ok_or(ActorIdentityError::CanonicalHolderMismatch {
            expected,
            actual: observed,
        })
}

/// A resident-cell ownership token. The generation is supplied by the
/// lifecycle owner; equality on both fields prevents an old completion from
/// taking ownership of a reloaded cell with the same FormID.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub(crate) struct ActorResidencyOwner {
    cell_form_id: u32,
    generation: u64,
}

impl ActorResidencyOwner {
    pub(crate) fn new(
        cell_form_id: u32,
        generation: u64,
    ) -> Result<Self, ActorResidencyOwnerError> {
        (cell_form_id != 0)
            .then_some(Self {
                cell_form_id,
                generation,
            })
            .ok_or(ActorResidencyOwnerError::InvalidCell)
    }

    pub(crate) const fn cell_form_id(self) -> u32 {
        self.cell_form_id
    }

    pub(crate) const fn generation(self) -> u64 {
        self.generation
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ActorResidencyOwnerError {
    InvalidCell,
}

/// A request made by a lifecycle adapter. The policy only decides; the
/// adapter remains responsible for changing ECS and `ActiveSaveState`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ActorResidencyRequest {
    Bind {
        destination: ActorResidencyOwner,
    },
    Retain {
        owner: ActorResidencyOwner,
    },
    Handoff {
        source: ActorResidencyOwner,
        destination: ActorResidencyOwner,
    },
    Unload {
        source: ActorResidencyOwner,
    },
    Restore {
        destination: ActorResidencyOwner,
    },
}

/// How the canonical `ActorInstanceState` record is to be treated. These
/// are instructions about the existing save authority, not a second copy of
/// that state.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum CanonicalStateTransition {
    EnsureAt {
        cell_form_id: u32,
    },
    KeepAt {
        cell_form_id: u32,
    },
    Move {
        from_cell_form_id: u32,
        to_cell_form_id: u32,
    },
}

/// The accepted transition returned to the runtime adapter.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ActorResidencyTransition {
    Bind {
        owner: ActorResidencyOwner,
        state: CanonicalStateTransition,
    },
    Retain {
        owner: ActorResidencyOwner,
        state: CanonicalStateTransition,
    },
    Handoff {
        source: ActorResidencyOwner,
        destination: ActorResidencyOwner,
        state: CanonicalStateTransition,
    },
    Unload {
        owner: ActorResidencyOwner,
        state: CanonicalStateTransition,
    },
    Restore {
        owner: ActorResidencyOwner,
        state: CanonicalStateTransition,
    },
}

/// One accepted transition. `actor` and `holder` are repeated on every
/// result so the integration seam cannot accidentally key one operation by a
/// base FormID or invent a different item-holder identity.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct ActorResidencyPlan {
    actor: ActorIdentity,
    holder: HolderId,
    transition: ActorResidencyTransition,
}

impl ActorResidencyPlan {
    pub(crate) const fn actor(self) -> ActorIdentity {
        self.actor
    }

    pub(crate) const fn holder(self) -> HolderId {
        self.holder
    }

    pub(crate) const fn transition(self) -> ActorResidencyTransition {
        self.transition
    }
}

/// Rejections are explicit so callers can count or log stale completions
/// without guessing whether an actor was duplicated or merely out of date.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ActorResidencyRejection {
    InvalidOwner {
        owner: ActorResidencyOwner,
    },
    InvalidHandoff {
        source: ActorResidencyOwner,
        destination: ActorResidencyOwner,
    },
    DuplicateOwner {
        count: usize,
    },
    CompetingOwner {
        actual: ActorResidencyOwner,
    },
    StaleSource {
        expected: ActorResidencyOwner,
        actual: Option<ActorResidencyOwner>,
    },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ActorResidencyDecision {
    Apply(ActorResidencyPlan),
    Reject(ActorResidencyRejection),
}

/// Resolves one actor request against a caller-owned observation of its live
/// owners. An empty slice means unowned; one entry is the only valid unique
/// owner; two or more entries are rejected as duplicate projections. The
/// slice is never retained or mutated.
pub(crate) fn decide_actor_residency(
    actor: ActorIdentity,
    owners: &[ActorResidencyOwner],
    request: ActorResidencyRequest,
) -> ActorResidencyDecision {
    if let Some(&owner) = owners.iter().find(|owner| owner.cell_form_id == 0) {
        return ActorResidencyDecision::Reject(ActorResidencyRejection::InvalidOwner { owner });
    }
    if owners.len() > 1 {
        return ActorResidencyDecision::Reject(ActorResidencyRejection::DuplicateOwner {
            count: owners.len(),
        });
    }
    let current = owners.first().copied();

    match request {
        ActorResidencyRequest::Bind { destination } => {
            if destination.cell_form_id == 0 {
                return invalid_owner(destination);
            }
            if let Some(actual) = current {
                return ActorResidencyDecision::Reject(ActorResidencyRejection::CompetingOwner {
                    actual,
                });
            }
            apply(
                actor,
                ActorResidencyTransition::Bind {
                    owner: destination,
                    state: CanonicalStateTransition::EnsureAt {
                        cell_form_id: destination.cell_form_id,
                    },
                },
            )
        }
        ActorResidencyRequest::Retain { owner } => {
            if owner.cell_form_id == 0 {
                return invalid_owner(owner);
            }
            match current {
                Some(actual) if actual == owner => apply(
                    actor,
                    ActorResidencyTransition::Retain {
                        owner,
                        state: CanonicalStateTransition::KeepAt {
                            cell_form_id: owner.cell_form_id,
                        },
                    },
                ),
                Some(actual) if actual.cell_form_id == owner.cell_form_id => stale(owner, current),
                Some(actual) => competing(actual),
                None => stale(owner, None),
            }
        }
        ActorResidencyRequest::Handoff {
            source,
            destination,
        } => {
            if source.cell_form_id == 0 {
                return invalid_owner(source);
            }
            if destination.cell_form_id == 0 {
                return invalid_owner(destination);
            }
            if source.cell_form_id == destination.cell_form_id {
                return ActorResidencyDecision::Reject(ActorResidencyRejection::InvalidHandoff {
                    source,
                    destination,
                });
            }
            match current {
                Some(actual) if actual == source => apply(
                    actor,
                    ActorResidencyTransition::Handoff {
                        source,
                        destination,
                        state: CanonicalStateTransition::Move {
                            from_cell_form_id: source.cell_form_id,
                            to_cell_form_id: destination.cell_form_id,
                        },
                    },
                ),
                Some(_) => stale(source, current),
                None => stale(source, None),
            }
        }
        ActorResidencyRequest::Unload { source } => {
            if source.cell_form_id == 0 {
                return invalid_owner(source);
            }
            match current {
                Some(actual) if actual == source => apply(
                    actor,
                    ActorResidencyTransition::Unload {
                        owner: source,
                        state: CanonicalStateTransition::KeepAt {
                            cell_form_id: source.cell_form_id,
                        },
                    },
                ),
                Some(actual) if actual.cell_form_id == source.cell_form_id => {
                    stale(source, current)
                }
                Some(actual) => competing(actual),
                None => stale(source, None),
            }
        }
        ActorResidencyRequest::Restore { destination } => {
            if destination.cell_form_id == 0 {
                return invalid_owner(destination);
            }
            if let Some(actual) = current {
                return competing(actual);
            }
            apply(
                actor,
                ActorResidencyTransition::Restore {
                    owner: destination,
                    state: CanonicalStateTransition::KeepAt {
                        cell_form_id: destination.cell_form_id,
                    },
                },
            )
        }
    }
}

fn apply(actor: ActorIdentity, transition: ActorResidencyTransition) -> ActorResidencyDecision {
    ActorResidencyDecision::Apply(ActorResidencyPlan {
        actor,
        holder: actor.holder(),
        transition,
    })
}

fn invalid_owner(owner: ActorResidencyOwner) -> ActorResidencyDecision {
    ActorResidencyDecision::Reject(ActorResidencyRejection::InvalidOwner { owner })
}

fn competing(actual: ActorResidencyOwner) -> ActorResidencyDecision {
    ActorResidencyDecision::Reject(ActorResidencyRejection::CompetingOwner { actual })
}

fn stale(
    expected: ActorResidencyOwner,
    actual: Option<ActorResidencyOwner>,
) -> ActorResidencyDecision {
    ActorResidencyDecision::Reject(ActorResidencyRejection::StaleSource { expected, actual })
}

#[cfg(test)]
#[path = "tests/actor_residency.rs"]
mod tests;
