//! Pure repath-decision policy (issue #113, M4 wave 4 feature 4): given
//! observations about the agent's active route (door/link state, target
//! movement, destination-cell availability, whether the agent has fallen
//! off its link), decides whether the runtime should keep the current
//! route, request a new path, or give up. No `bevy`/`bevy_landmass` import
//! -- cucumber-includable via `#[path]`, matching `door_link.rs`. The Bevy
//! system (`nav/agent.rs`) only feeds observations in; this module owns the
//! decision table.
//!
//! Wired into the runtime narrowly for this wave (see `nav/agent.rs`'s
//! `door_availability_system`): a door-usability flip yields one `Repath`
//! decision, applied as spawning/despawning the affected two-sided door
//! links, re-inserting the agent's target so landmass replans, and -- when
//! the agent is `Paused` at that very door and it became usable --
//! requesting the door open. A door that becomes locked while the agent is
//! already `Paused` waiting on it does *not* fail the wait early: the
//! paused lifecycle keeps polling the open state and resolves through
//! `door_link::MAX_WAIT_TICKS` to the deterministic `Failed` outcome. A
//! fully dynamic "any door state change mid-route triggers a live link
//! rebuild" system (rebuilding the archipelago's links while an unrelated
//! route is already in flight) is out of scope for this pass -- the
//! decision table itself is complete and tested per the plan's "repath
//! decision table" requirement, but only one concrete call site consumes
//! it this wave.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) struct RepathObservation {
    /// A door link the current route relies on transitioned from
    /// usable/unlocked to blocked/locked.
    pub(crate) door_became_blocked: bool,
    /// A door link the current route relies on transitioned from
    /// blocked/locked to usable/unlocked.
    pub(crate) door_became_unblocked: bool,
    /// The route's target (an entity or point) moved beyond the agent's
    /// target-reached tolerance since the route was planned.
    pub(crate) target_moved_beyond_tolerance: bool,
    /// The destination cell for a travel-door route is no longer the
    /// active/loaded cell.
    pub(crate) destination_cell_unloaded: bool,
    /// The agent's position is no longer associated with any polygon/link
    /// of the planned route (e.g. pushed off by avoidance).
    pub(crate) agent_off_link: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum RepathDecision {
    /// Nothing changed that invalidates the current route; keep going.
    KeepRoute,
    /// Something changed that may make a different route better or
    /// necessary; the caller should re-request a path (e.g. re-set
    /// `AgentTarget3d` to the same target so landmass replans).
    Repath,
    /// The route can no longer possibly succeed; give up rather than
    /// attempt to repath (there is nothing a new plan could reach).
    Fail,
}

/// Deterministic decision table (issue #113 feature 4), evaluated in a
/// fixed precedence order so exactly one observation combination always
/// yields the same decision:
///
/// 1. `destination_cell_unloaded` always fails outright -- there is no
///    reachable destination to plan a new route *to*.
/// 2. Any of `door_became_blocked`, `door_became_unblocked`,
///    `target_moved_beyond_tolerance`, or `agent_off_link` triggers a
///    repath attempt. A newly blocked door invalidates the current plan; a
///    newly unblocked door may open a better one; a moved target or an
///    off-link agent both need a fresh plan from the agent's actual
///    situation. If the resulting fresh plan itself has no route, that
///    surfaces through landmass's own `NoPath` agent state, not this
///    function.
/// 3. Otherwise, keep the current route.
pub(crate) fn decide(observation: RepathObservation) -> RepathDecision {
    if observation.destination_cell_unloaded {
        return RepathDecision::Fail;
    }
    if observation.door_became_blocked
        || observation.door_became_unblocked
        || observation.target_moved_beyond_tolerance
        || observation.agent_off_link
    {
        return RepathDecision::Repath;
    }
    RepathDecision::KeepRoute
}

/// Pure blocked-door availability rule (issue #113 feature 3): what
/// `nav/agent.rs` observed about one door.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) struct DoorObservation {
    /// The door is locked per its prepared lock/key data and the player's
    /// inventory (the `interaction::door_is_locked` check).
    pub(crate) locked: bool,
    /// The door's runtime open state (`InteractionState.open`).
    pub(crate) open: bool,
}

/// Whether a door link is usable for route planning: already open (an open
/// door is passable regardless of its lock record), or not locked. Locked
/// and closed = blocked (excluded from planning until usable).
pub(crate) fn door_usable(observation: DoorObservation) -> bool {
    observation.open || !observation.locked
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_observations_keeps_the_route() {
        assert_eq!(
            decide(RepathObservation::default()),
            RepathDecision::KeepRoute
        );
    }

    #[test]
    fn destination_cell_unloaded_always_fails_even_alongside_other_triggers() {
        let observation = RepathObservation {
            destination_cell_unloaded: true,
            door_became_unblocked: true,
            ..Default::default()
        };
        assert_eq!(decide(observation), RepathDecision::Fail);
    }

    #[test]
    fn a_newly_blocked_door_triggers_repath() {
        let observation = RepathObservation {
            door_became_blocked: true,
            ..Default::default()
        };
        assert_eq!(decide(observation), RepathDecision::Repath);
    }

    #[test]
    fn a_newly_unblocked_door_triggers_repath() {
        let observation = RepathObservation {
            door_became_unblocked: true,
            ..Default::default()
        };
        assert_eq!(decide(observation), RepathDecision::Repath);
    }

    #[test]
    fn a_moved_target_triggers_repath() {
        let observation = RepathObservation {
            target_moved_beyond_tolerance: true,
            ..Default::default()
        };
        assert_eq!(decide(observation), RepathDecision::Repath);
    }

    #[test]
    fn an_off_link_agent_triggers_repath() {
        let observation = RepathObservation {
            agent_off_link: true,
            ..Default::default()
        };
        assert_eq!(decide(observation), RepathDecision::Repath);
    }
}
