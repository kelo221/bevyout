//! Pure door-link traversal state machine (issue #112 feature 4): the
//! lifecycle a nav agent goes through when it reaches an intra-cell door
//! off-mesh link -- pause, request activation, wait for the door to open,
//! traverse, resume. No `bevy`/`bevy_landmass`/`landmass` import: this
//! module only decides what state comes next given an event, so it is
//! unit-testable (and includable verbatim by `tests/features.rs` via
//! `#[path]`) without a `World` or an archipelago. `nav/agent.rs` drives it
//! with the real `InteractionState.open` poll and the real
//! `interaction::scripted_door_open` request/log lines.

/// How many consecutive "still closed" ticks to tolerate before giving up.
/// A locked door never opens through the scripted (locks-bypassed)
/// activation boundary this wave routes through (see
/// `interaction::scripted_door_open`'s doc comment, mirroring
/// `console::activate_reference`'s "locks bypassed" scripted-activation
/// philosophy) -- so real FO3 data is not expected to exercise this bound in
/// this spike; it exists only as a deterministic backstop.
pub(crate) const MAX_WAIT_TICKS: u32 = 120;

/// Which cell the far side of a link is in (issue #113, M4 wave 4 feature
/// 3). `Traversing`/`Paused` carry this so `TraversalComplete` knows whether
/// to return to `Idle` (intra-cell -- wave 3's original behaviour) or land
/// in the new `TravelReached` terminal state (a travel door: the far side's
/// geometry lives in another, unloaded cell, so there is nothing to
/// continue onto here).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum LinkDestination {
    IntraCell,
    Travel { destination_cell_form_id: u32 },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) enum DoorLinkState {
    /// No door-link traversal in progress.
    #[default]
    Idle,
    /// The agent reached the link and is waiting for `door_form_id` to
    /// open.
    Paused {
        door_form_id: u32,
        waited_ticks: u32,
        destination: LinkDestination,
    },
    /// The door is open; the agent is crossing the link.
    Traversing {
        door_form_id: u32,
        destination: LinkDestination,
    },
    /// The door never opened within `MAX_WAIT_TICKS`; the agent gives up
    /// and stops at the link (deterministic unreachable-style outcome).
    Failed { door_form_id: u32 },
    /// A travel door's traversal completed. #113's scope stops at the door
    /// (a distinct terminal status + tracing line, per this wave's brief);
    /// #134 owns actually moving the agent to `destination_cell_form_id`.
    TravelReached {
        door_form_id: u32,
        destination_cell_form_id: u32,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum DoorLinkEvent {
    /// The agent reached an off-mesh link belonging to this door.
    LinkReached {
        door_form_id: u32,
        destination: LinkDestination,
    },
    /// One wait tick passed while paused; `door_open` reflects the polled
    /// `InteractionState.open` state for this door's reference entity.
    Tick { door_open: bool },
    /// The agent finished crossing the link geometry.
    TraversalComplete,
}

/// Pure transition table:
/// - `Idle`/`Failed`/`TravelReached` only leave on `LinkReached`, moving to
///   `Paused` (a `TravelReached` agent can be sent through a *different*
///   door and restart the lifecycle for it).
/// - `Paused` moves to `Traversing` the tick `door_open` is observed `true`,
///   or to `Failed` after `MAX_WAIT_TICKS` consecutive closed polls.
/// - `Traversing` only leaves on `TraversalComplete`: `IntraCell` returns to
///   `Idle` (wave 3's original behaviour), `Travel` lands in
///   `TravelReached`.
/// - Any other `(state, event)` pair is a no-op (returns `state`
///   unchanged) rather than panicking -- e.g. a stray `Tick` while `Idle`.
pub(crate) fn transition(state: DoorLinkState, event: DoorLinkEvent) -> DoorLinkState {
    match (state, event) {
        (
            DoorLinkState::Idle
            | DoorLinkState::Failed { .. }
            | DoorLinkState::TravelReached { .. },
            DoorLinkEvent::LinkReached {
                door_form_id,
                destination,
            },
        ) => DoorLinkState::Paused {
            door_form_id,
            waited_ticks: 0,
            destination,
        },
        (
            DoorLinkState::Paused {
                door_form_id,
                destination,
                ..
            },
            DoorLinkEvent::Tick { door_open: true },
        ) => DoorLinkState::Traversing {
            door_form_id,
            destination,
        },
        (
            DoorLinkState::Paused {
                door_form_id,
                waited_ticks,
                destination,
            },
            DoorLinkEvent::Tick { door_open: false },
        ) => {
            let waited_ticks = waited_ticks + 1;
            if waited_ticks >= MAX_WAIT_TICKS {
                DoorLinkState::Failed { door_form_id }
            } else {
                DoorLinkState::Paused {
                    door_form_id,
                    waited_ticks,
                    destination,
                }
            }
        }
        (
            DoorLinkState::Traversing {
                door_form_id,
                destination: LinkDestination::IntraCell,
            },
            DoorLinkEvent::TraversalComplete,
        ) => {
            let _ = door_form_id;
            DoorLinkState::Idle
        }
        (
            DoorLinkState::Traversing {
                door_form_id,
                destination:
                    LinkDestination::Travel {
                        destination_cell_form_id,
                    },
            },
            DoorLinkEvent::TraversalComplete,
        ) => DoorLinkState::TravelReached {
            door_form_id,
            destination_cell_form_id,
        },
        (other, _) => other,
    }
}

pub(crate) fn is_paused(state: DoorLinkState) -> bool {
    matches!(state, DoorLinkState::Paused { .. })
}

pub(crate) fn is_traversing(state: DoorLinkState) -> bool {
    matches!(state, DoorLinkState::Traversing { .. })
}

pub(crate) fn is_failed(state: DoorLinkState) -> bool {
    matches!(state, DoorLinkState::Failed { .. })
}

pub(crate) fn is_travel_reached(state: DoorLinkState) -> bool {
    matches!(state, DoorLinkState::TravelReached { .. })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn intra_cell_reached(door_form_id: u32) -> DoorLinkEvent {
        DoorLinkEvent::LinkReached {
            door_form_id,
            destination: LinkDestination::IntraCell,
        }
    }

    fn travel_reached(door_form_id: u32, destination_cell_form_id: u32) -> DoorLinkEvent {
        DoorLinkEvent::LinkReached {
            door_form_id,
            destination: LinkDestination::Travel {
                destination_cell_form_id,
            },
        }
    }

    #[test]
    fn reaching_a_link_pauses_the_agent() {
        let state = transition(DoorLinkState::Idle, intra_cell_reached(0x99));
        assert_eq!(
            state,
            DoorLinkState::Paused {
                door_form_id: 0x99,
                waited_ticks: 0,
                destination: LinkDestination::IntraCell,
            }
        );
        assert!(is_paused(state));
    }

    #[test]
    fn door_opening_resumes_into_traversing() {
        let paused = DoorLinkState::Paused {
            door_form_id: 0x99,
            waited_ticks: 5,
            destination: LinkDestination::IntraCell,
        };
        let state = transition(paused, DoorLinkEvent::Tick { door_open: true });
        assert_eq!(
            state,
            DoorLinkState::Traversing {
                door_form_id: 0x99,
                destination: LinkDestination::IntraCell,
            }
        );
        assert!(is_traversing(state));
    }

    #[test]
    fn still_closed_ticks_accumulate_without_leaving_paused() {
        let mut state = DoorLinkState::Paused {
            door_form_id: 0x99,
            waited_ticks: 0,
            destination: LinkDestination::IntraCell,
        };
        for _ in 0..MAX_WAIT_TICKS - 1 {
            state = transition(state, DoorLinkEvent::Tick { door_open: false });
            assert!(is_paused(state));
        }
    }

    #[test]
    fn exhausting_the_wait_bound_fails_deterministically() {
        let mut state = DoorLinkState::Paused {
            door_form_id: 0x99,
            waited_ticks: 0,
            destination: LinkDestination::IntraCell,
        };
        for _ in 0..MAX_WAIT_TICKS {
            state = transition(state, DoorLinkEvent::Tick { door_open: false });
        }
        assert_eq!(state, DoorLinkState::Failed { door_form_id: 0x99 });
        assert!(is_failed(state));
    }

    #[test]
    fn intra_cell_traversal_complete_returns_to_idle() {
        let traversing = DoorLinkState::Traversing {
            door_form_id: 0x99,
            destination: LinkDestination::IntraCell,
        };
        let state = transition(traversing, DoorLinkEvent::TraversalComplete);
        assert_eq!(state, DoorLinkState::Idle);
    }

    #[test]
    fn travel_door_traversal_complete_lands_in_travel_reached() {
        let state = transition(DoorLinkState::Idle, travel_reached(0x99, 0xC0DE));
        let state = transition(state, DoorLinkEvent::Tick { door_open: true });
        assert_eq!(
            state,
            DoorLinkState::Traversing {
                door_form_id: 0x99,
                destination: LinkDestination::Travel {
                    destination_cell_form_id: 0xC0DE
                },
            }
        );
        let state = transition(state, DoorLinkEvent::TraversalComplete);
        assert_eq!(
            state,
            DoorLinkState::TravelReached {
                door_form_id: 0x99,
                destination_cell_form_id: 0xC0DE,
            }
        );
        assert!(is_travel_reached(state));
        assert!(!is_paused(state) && !is_traversing(state) && !is_failed(state));
    }

    #[test]
    fn a_new_link_reached_after_failure_restarts_the_lifecycle() {
        let failed = DoorLinkState::Failed { door_form_id: 0x99 };
        let state = transition(failed, intra_cell_reached(0x99));
        assert_eq!(
            state,
            DoorLinkState::Paused {
                door_form_id: 0x99,
                waited_ticks: 0,
                destination: LinkDestination::IntraCell,
            }
        );
    }

    #[test]
    fn a_new_link_reached_after_a_travel_door_restarts_the_lifecycle_for_another_door() {
        let reached = DoorLinkState::TravelReached {
            door_form_id: 0x99,
            destination_cell_form_id: 0xC0DE,
        };
        let state = transition(reached, intra_cell_reached(0x50));
        assert_eq!(
            state,
            DoorLinkState::Paused {
                door_form_id: 0x50,
                waited_ticks: 0,
                destination: LinkDestination::IntraCell,
            }
        );
    }

    #[test]
    fn stray_tick_while_idle_is_a_no_op() {
        let state = transition(DoorLinkState::Idle, DoorLinkEvent::Tick { door_open: true });
        assert_eq!(state, DoorLinkState::Idle);
    }

    #[test]
    fn stray_tick_while_travel_reached_is_a_no_op() {
        let reached = DoorLinkState::TravelReached {
            door_form_id: 0x99,
            destination_cell_form_id: 0xC0DE,
        };
        let state = transition(reached, DoorLinkEvent::Tick { door_open: true });
        assert_eq!(state, reached);
    }
}
