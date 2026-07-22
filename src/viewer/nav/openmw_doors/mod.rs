//! OpenMW-derived AI door-access decision rule (issue #185).
//!
//! Adapts `MWMechanics::AiPackage::openDoors()`'s core test -- untrapped and
//! unlocked always passes; locked passes only if the actor's own inventory
//! holds the door's key (`if (keyId.empty()) return;` -- an empty key
//! requirement always leaves the door shut, never "anyone may force it");
//! trapped never passes at all, a deliberate bevyout-side simplification of
//! OpenMW's literal fall-through documented in this directory's `NOTICE.md`.
//!
//! Std-only (no `bevy`/`bevy_landmass`/`bevyout_core` import), mirroring
//! `door_link.rs`/`movement_policy.rs`, so `tests/features.rs` can include
//! it verbatim via `#[path]`. `nav/agent.rs` is the only Bevy-side caller: it
//! resolves the observation (the door's prepared `DoorLockInfo` plus whether
//! the specific routing agent's own canonical inventory holds the key) and
//! folds the verdict into its existing `locked` boolean -- every downstream
//! consumer (`repath::door_usable`, `door_link::crossing_gate`,
//! `door_link::effective_door_open`) already treats "locked" as "not
//! openable" and needs no change of its own.

/// What `door_openable` needs to decide one door, for one specific actor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) struct DoorAccessObservation {
    /// The door's prepared lock level (`PreparedDoor::lock_level`):
    /// `None` or `<= 0` means unlocked, matching `interaction::
    /// door_is_locked`'s own rule.
    pub(crate) lock_level: Option<i8>,
    /// `PreparedDoor::trapped` (see its doc comment).
    pub(crate) trapped: bool,
    /// The door's prepared key requirement (`PreparedDoor::key_form_id`).
    /// `None` means the door has no assigned key at all -- OpenMW's
    /// `keyId.empty()` case, which stays shut regardless of lock level.
    pub(crate) key_form_id: Option<u32>,
    /// Whether the specific actor being asked about currently holds an item
    /// stack whose base FormID matches `key_form_id`. Meaningless (and
    /// never consulted) when `key_form_id` is `None`.
    pub(crate) holder_has_key: bool,
}

/// Whether a door is openable by the actor `observation` describes,
/// mirroring OpenMW's `AiPackage::openDoors()` decision (see this module's
/// doc comment for the one deliberate divergence: trapped is an
/// unconditional veto here, not OpenMW's literal fall-through).
pub(crate) fn door_openable(observation: DoorAccessObservation) -> bool {
    if observation.trapped {
        return false;
    }
    let locked = observation.lock_level.is_some_and(|level| level > 0);
    if !locked {
        return true;
    }
    match observation.key_form_id {
        None => false,
        Some(_) => observation.holder_has_key,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn base() -> DoorAccessObservation {
        DoorAccessObservation {
            lock_level: Some(25),
            trapped: false,
            key_form_id: Some(0x1234),
            holder_has_key: false,
        }
    }

    #[test]
    fn an_untrapped_unlocked_door_is_always_openable() {
        assert!(door_openable(DoorAccessObservation {
            lock_level: None,
            ..base()
        }));
        assert!(door_openable(DoorAccessObservation {
            lock_level: Some(0),
            ..base()
        }));
    }

    #[test]
    fn a_locked_door_is_openable_when_the_actor_holds_its_key() {
        assert!(door_openable(DoorAccessObservation {
            holder_has_key: true,
            ..base()
        }));
    }

    #[test]
    fn a_locked_door_stays_shut_when_the_actor_lacks_the_key() {
        assert!(!door_openable(DoorAccessObservation {
            holder_has_key: false,
            ..base()
        }));
    }

    #[test]
    fn a_locked_door_with_no_assigned_key_never_opens_for_anyone() {
        // OpenMW's `keyId.empty()` case: `if (keyId.empty()) return;` --
        // there is no key to hold, so no actor can force it open this way.
        assert!(!door_openable(DoorAccessObservation {
            key_form_id: None,
            holder_has_key: false,
            ..base()
        }));
    }

    #[test]
    fn a_trapped_door_never_opens_even_with_the_right_key() {
        assert!(!door_openable(DoorAccessObservation {
            trapped: true,
            holder_has_key: true,
            ..base()
        }));
    }

    #[test]
    fn a_trapped_but_unlocked_door_still_never_opens() {
        assert!(!door_openable(DoorAccessObservation {
            trapped: true,
            lock_level: None,
            ..base()
        }));
    }
}
