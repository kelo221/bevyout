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
