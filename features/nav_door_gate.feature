Feature: Mid-route door crossing gate
  # Issue #137 (M4 wave 5), building on #113's door-link state machine and
  # repath policy. Real FO3 doors are single-sided NAVM triangles
  # (`viewer::nav::landmass_graph::single_sided_doors`); one that does not
  # resolve to a travel-cell destination is an ordinary interior door in
  # the middle of an otherwise-contiguous walkable mesh, not a #113 link
  # endpoint. `viewer::nav::door_link::crossing_gate` is the pure decision
  # table `nav/agent.rs`'s crossing-check trigger consults (door-flagged
  # triangle x door open/lock state -> pass/wait/blocked) -- see
  # `nav/agent.rs`'s module doc for why a route-crossing proximity check
  # was chosen over off-mesh links across the door triangle. `Wait` and
  # `Blocked` both gate identically at the runtime call site (the *same*
  # `door_link` pause -> scripted-open -> resume lifecycle, and a locked
  # door resolves through the *same* `MAX_WAIT_TICKS` -> `Failed` backstop
  # feature 2 of the two-sided #113 door links already relies on); the
  # distinct `Blocked` value only lets the caller log the locked case
  # immediately.

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
