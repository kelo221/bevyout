Feature: Mid-route door crossing gate
  # Issue #137 (M4 wave 5), building on #113's door-link state machine and
  # repath policy. Real FO3 doors are single-sided NAVM triangles
  # (`viewer::nav::landmass_graph::single_sided_doors`), and real-data
  # review of the prepared catalog found nearly every one resolves to a
  # travel-cell destination -- several with 2-3 walkable neighbors, so the
  # mesh continues right through the triangle. An earlier revision of this
  # feature gated only single-sided doors that did *not* resolve to a
  # travel destination, which is an empty set on real data and so never
  # fired at all. The candidate set is now every single-sided door's
  # triangle, travel-door or not; `nav/agent.rs` additionally excludes
  # whichever one door is the agent's own active `travel_intent` target (a
  # goto crossing a *different* travel door's triangle, or crossing one
  # this agent isn't currently travelling to, still gates -- see
  # `nav/agent.rs`'s module doc for the full real-data rationale and the
  # off-mesh-links alternative that was rejected).
  #
  # `viewer::nav::door_link::crossing_gate` is the pure decision table
  # `nav/agent.rs`'s crossing-check trigger consults (door-flagged
  # triangle x door open/lock state -> pass/wait/blocked); it does not
  # itself know or care whether the triangle is also a travel door --
  # that candidate-set and travel_intent-exclusion logic lives entirely in
  # `nav/agent.rs` (Bevy-side, not cucumber-testable; see its own
  # minimal-App tests). `Wait` and `Blocked` both gate identically at the
  # runtime call site (the *same* `door_link` pause -> scripted-open ->
  # resume lifecycle, and a locked door resolves through the *same*
  # `MAX_WAIT_TICKS` -> `Failed` backstop feature 2 of the two-sided #113
  # door links already relies on); the distinct `Blocked` value only lets
  # the caller log the locked case immediately.

  Scenario Outline: The mid-route crossing gate is deterministic
    Given a mid-route door that is <open_state> and <lock_state>
    Then the crossing gate is <gate>

    Examples:
      | open_state | lock_state | gate    |
      | open       | unlocked   | pass    |
      | open       | locked     | pass    |
      | closed     | unlocked   | wait    |
      | closed     | locked     | blocked |

  Scenario: An unblocked mid-route door still gates through the same repath table a two-sided door link uses
    # `crossing_gate`'s `Wait`/`Blocked` distinction is caller-side logging
    # only -- the actual gate/no-gate decision (and the repath a later
    # unlock triggers) is the identical `repath::door_usable` rule the
    # #113 two-sided door links already reuse unchanged for mid-route
    # doors (see `nav/agent.rs`'s `ensure_archipelago`: mid-route doors are
    # just more entries in the same `door_usable` map).
    Given a door that is locked and closed
    Then the door is not usable for route planning
    Given a mid-route door that is closed and locked
    Then the crossing gate is blocked

  # Issue #177 (M4 wave 11): the containment gate above is a trigger that can
  # be starved. Real data (Vault 101, `VDoor01`) had an agent routed at a
  # closed in-cell door halt ~2 m short of its crossing with a completely free
  # collision sweep -- never entering the polygon, so never gating, never
  # requesting the open, and never continuing. `door_link::approach_gate` adds
  # a second trigger for exactly that case, narrowed so it cannot reintroduce
  # issue #155's defect: the agent must have *stopped making progress*, and
  # the crossing must lie between it and its target.
  Scenario Outline: A stalled agent gates on the crossing its route continues through
    Given an agent <progress> <distance> metres from a door crossing
    And the crossing is <crossing_to_target> metres from the target and the agent is <agent_to_target> metres from it
    Then the approach gate <outcome>

    Examples:
      | progress          | distance | crossing_to_target | agent_to_target | outcome     |
      | stalled           | 2.2      | 4.0                | 6.0             | fires       |
      | still moving      | 2.2      | 4.0                | 6.0             | does not fire |
      | stalled           | 3.5      | 4.0                | 6.0             | does not fire |
      | stalled           | 2.2      | 7.0                | 6.0             | does not fire |
      | stalled           | 0.0      | 4.0                | 6.0             | fires       |
