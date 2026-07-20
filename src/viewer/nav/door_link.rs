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

// ---------------------------------------------------------------------
// Mid-route crossing gate (issue #137)
// ---------------------------------------------------------------------

/// Whether an agent approaching a door-flagged NAVM triangle *mid-route*
/// (not an existing #113 link endpoint -- see `nav/agent.rs`'s module doc
/// for why this case needs its own trigger) should proceed, or be gated
/// into this module's pause -> scripted-open -> resume lifecycle. The
/// triangle itself is never removed from the walkable mesh (it is
/// ordinary contiguous ground, not a seam between two islands like the
/// #113 door links) -- this table only decides whether `nav/agent.rs`
/// should fire a `DoorLinkEvent::LinkReached` for it this tick.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum CrossingGate {
    /// The door is already open: this is plain walkable ground, proceed
    /// without pausing.
    Pass,
    /// The door is closed and not locked: gate through the normal
    /// lifecycle -- it is expected to open once requested.
    Wait,
    /// The door is closed *and* locked: the caller still gates
    /// identically to `Wait` (the agent must not clip through), but this
    /// distinct value lets it log the locked case immediately rather than
    /// only after `MAX_WAIT_TICKS` -- the same eventual `Failed` outcome
    /// either way, since a locked door never opens through the scripted
    /// activation boundary (see `MAX_WAIT_TICKS`'s doc comment).
    Blocked,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) struct CrossingObservation {
    pub(crate) door_open: bool,
    pub(crate) door_locked: bool,
}

/// Deterministic table: an open door always passes (mirrors
/// `repath::door_usable`'s "already open is passable regardless of lock"
/// rule); otherwise closed-and-locked is `Blocked`, closed-and-unlocked is
/// `Wait`.
pub(crate) fn crossing_gate(observation: CrossingObservation) -> CrossingGate {
    if observation.door_open {
        CrossingGate::Pass
    } else if observation.door_locked {
        CrossingGate::Blocked
    } else {
        CrossingGate::Wait
    }
}

// ---------------------------------------------------------------------
// Travel hand-off lock authority (issue #165, real-data acceptance
// follow-up)
// ---------------------------------------------------------------------

/// What a `Paused` agent's `Tick` event should report as `door_open`,
/// given the door's raw physical open flag (`InteractionState.open`) and
/// its current lock observation. `crossing_gate`'s "already open passes
/// regardless of lock" rule (mirroring `repath::door_usable`) is correct
/// for an ordinary `IntraCell` mid-route crossing -- walking through a
/// door that is standing open is a physical act the lock record cannot
/// retroactively undo. It is *not* correct for a `Travel` destination's
/// hand-off: that is a scripted cell transition, not a physical
/// walk-through, and a *prior* successful travel through the very same
/// door leaves it physically open forever (a hand-off never closes it).
/// Without this override, `setlock`-ing that door and reissuing `tna
/// travel` would reach this arm with `physically_open == true` on the
/// very first tick, skip straight to `Traversing`, and hand the agent off
/// through a locked door -- the real-data measurement this function
/// fixes. Lock state is authoritative for a `Travel` destination
/// regardless of the door's physical open state; an `IntraCell` crossing
/// keeps the ordinary physical-open rule unchanged.
pub(crate) fn effective_door_open(
    destination: LinkDestination,
    physically_open: bool,
    door_locked: bool,
) -> bool {
    match destination {
        LinkDestination::Travel { .. } => physically_open && !door_locked,
        LinkDestination::IntraCell => physically_open,
    }
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

    // -------------------------------------------------------------
    // Mid-route crossing gate (issue #137)
    // -------------------------------------------------------------

    #[test]
    fn an_open_door_passes() {
        assert_eq!(
            crossing_gate(CrossingObservation {
                door_open: true,
                door_locked: false,
            }),
            CrossingGate::Pass
        );
    }

    #[test]
    fn an_open_door_passes_even_if_its_lock_record_is_still_locked() {
        // Mirrors `repath::door_usable`'s rule: an already-open door is
        // passable regardless of its lock record.
        assert_eq!(
            crossing_gate(CrossingObservation {
                door_open: true,
                door_locked: true,
            }),
            CrossingGate::Pass
        );
    }

    #[test]
    fn a_closed_unlocked_door_waits() {
        assert_eq!(
            crossing_gate(CrossingObservation {
                door_open: false,
                door_locked: false,
            }),
            CrossingGate::Wait
        );
    }

    #[test]
    fn a_closed_locked_door_is_blocked() {
        assert_eq!(
            crossing_gate(CrossingObservation {
                door_open: false,
                door_locked: true,
            }),
            CrossingGate::Blocked
        );
    }

    // -------------------------------------------------------------
    // `effective_door_open` (issue #165, real-data acceptance follow-up:
    // a travel target left physically open by a *prior* hand-off must
    // still fail when locked).
    // -------------------------------------------------------------

    fn travel_to(cell: u32) -> LinkDestination {
        LinkDestination::Travel {
            destination_cell_form_id: cell,
        }
    }

    #[test]
    fn a_physically_open_locked_travel_destination_is_not_effectively_open() {
        // The exact real-data shape: a prior hand-off left the door open;
        // `setlock` locked it again. Lock is authoritative for the
        // hand-off despite the door still standing open.
        assert!(!effective_door_open(travel_to(0xC0DE), true, true));
    }

    #[test]
    fn a_physically_open_unlocked_travel_destination_is_effectively_open() {
        assert!(effective_door_open(travel_to(0xC0DE), true, false));
    }

    #[test]
    fn a_physically_closed_travel_destination_is_never_effectively_open_regardless_of_lock() {
        assert!(!effective_door_open(travel_to(0xC0DE), false, true));
        assert!(!effective_door_open(travel_to(0xC0DE), false, false));
    }

    #[test]
    fn an_intra_cell_crossing_keeps_the_physical_open_rule_regardless_of_lock() {
        // Unlike `Travel`, an ordinary mid-route crossing passes through
        // an already-open door regardless of its lock record -- mirrors
        // `crossing_gate`'s own `an_open_door_passes_even_if_its_lock_
        // record_is_still_locked` rule.
        assert!(effective_door_open(LinkDestination::IntraCell, true, true));
        assert!(effective_door_open(LinkDestination::IntraCell, true, false));
        assert!(!effective_door_open(
            LinkDestination::IntraCell,
            false,
            true
        ));
    }
}

// ---------------------------------------------------------------------
// Stalled-approach crossing gate (issue #177 acceptance)
// ---------------------------------------------------------------------

/// How close (metres, XZ) to a door's crossing footprint a *stalled* agent
/// may be and still have that door count as the thing it is stalled against.
///
/// The containment gate (`landmass_graph::point_in_door_triangle`) requires
/// the agent to be standing *on* the crossing polygon, which is the right
/// trigger whenever the agent actually gets there. Real data showed it does
/// not always get there: on a doorway whose approach the prepare-side
/// clearance pass has eroded, an agent routed at a closed door halts short of
/// the crossing with a completely free collision sweep and simply stops
/// making progress -- never entering the polygon, so never gating, never
/// requesting the open, and never continuing. Containment alone is a trigger
/// that can be starved.
///
/// This bound is deliberately generous because it is only ever consulted for
/// an agent that has *already* stopped making progress toward a target it
/// still has, and only for a crossing its own route demonstrably continues
/// past ([`approach_gate`]'s `crossing_is_on_the_way` term). A door the agent
/// is merely walking past, or one behind it, can never satisfy those terms,
/// so widening the radius cannot reintroduce issue #155's defect -- the
/// proximity scan it replaced fired for any door near the *corridor*,
/// with no stall and no route relationship required.
pub(crate) const DOOR_CROSSING_APPROACH_DISTANCE: f32 = 3.0;

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct ApproachObservation {
    /// XZ distance from the agent to the crossing polygon's footprint
    /// (`0.0` when the agent is inside it).
    pub(crate) distance_to_crossing: f32,
    /// XZ distance from the agent to its current target.
    pub(crate) agent_distance_to_target: f32,
    /// XZ distance from the crossing polygon to that same target.
    pub(crate) crossing_distance_to_target: f32,
    /// The agent has stopped making progress (`AgentKcc`'s
    /// `collision_blocked`/`stuck` latches, which
    /// `movement_policy::decide_collision_outcome`/`decide_stuck` own).
    pub(crate) stalled: bool,
}

/// Whether a stalled agent should gate on a nearby door crossing it has not
/// managed to reach: it must actually be stalled, the crossing must be within
/// [`DOOR_CROSSING_APPROACH_DISTANCE`], and the crossing must lie *between*
/// the agent and its target rather than behind or beside it -- otherwise an
/// agent stuck for an unrelated reason would start opening whatever door
/// happened to be nearest.
pub(crate) fn approach_gate(observation: ApproachObservation) -> bool {
    if !observation.stalled {
        return false;
    }
    if observation.distance_to_crossing > DOOR_CROSSING_APPROACH_DISTANCE {
        return false;
    }
    observation.crossing_distance_to_target < observation.agent_distance_to_target
}

#[cfg(test)]
mod approach_tests {
    use super::*;

    fn observation() -> ApproachObservation {
        ApproachObservation {
            distance_to_crossing: 2.2,
            agent_distance_to_target: 6.0,
            crossing_distance_to_target: 4.0,
            stalled: true,
        }
    }

    #[test]
    fn a_stalled_agent_short_of_a_crossing_on_its_route_gates() {
        assert!(approach_gate(observation()));
    }

    #[test]
    fn an_agent_still_making_progress_never_gates_on_approach() {
        assert!(!approach_gate(ApproachObservation {
            stalled: false,
            ..observation()
        }));
    }

    #[test]
    fn a_crossing_beyond_the_approach_bound_never_gates() {
        assert!(!approach_gate(ApproachObservation {
            distance_to_crossing: DOOR_CROSSING_APPROACH_DISTANCE + 0.01,
            ..observation()
        }));
    }

    #[test]
    fn a_crossing_behind_the_agent_never_gates() {
        // Further from the target than the agent is: the route does not
        // continue through it, so it is not what the agent is stalled on.
        assert!(!approach_gate(ApproachObservation {
            crossing_distance_to_target: 7.0,
            ..observation()
        }));
    }

    #[test]
    fn standing_inside_the_crossing_while_stalled_still_gates() {
        assert!(approach_gate(ApproachObservation {
            distance_to_crossing: 0.0,
            ..observation()
        }));
    }
}
