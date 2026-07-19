Feature: Travel-door hand-off respects runtime lock state
  # Issue #165 (M4 wave 9), following on from #113's travel-arrival
  # lifecycle and #155's mid-route crossing gate. Real-data measurement on
  # FranklinMetro02 (door 0007f7e3) found that locking the agent's own
  # `travel_intent` door still ended in a hand-off -- the travel-arrival
  # branch transitioned into the pause -> open -> traverse -> `TravelReached`
  # lifecycle without ever consulting the lock/open observation the
  # mid-route crossing gate (`viewer::nav::door_link::crossing_gate`)
  # already uses as its source of truth.
  #
  # The pure `door_link` state machine these scenarios drive is exactly the
  # module `nav/agent.rs`'s travel-arrival branch and `request_door_open`
  # now both consult explicitly (see `nav/agent.rs`'s own doc comment on
  # the fix): a locked door's `LinkReached` still enters `Paused` (mirroring
  # the mid-route gate's own `Blocked` handling -- a locked door gates
  # identically to a merely-closed one, just with an immediate log), and
  # only fails after the deterministic `MAX_WAIT_TICKS` bound, never
  # opening and never reaching `TravelReached`. The A-B-A sequence below
  # (unlocked travel completes -> locked travel fails deterministically ->
  # unlocked travel completes again) is the exact invariant the issue's
  # acceptance script exercises on real data; this feature pins its pure
  # state-machine half.
  #
  # `nav/agent.rs`'s own `#[cfg(test)]` module additionally pins the part
  # this pure FSM cannot: real data showed the door registered as both a
  # `travel_doors` entry and a `mid_route_doors` crossing-gate candidate
  # (every real travel door is both), and a `Failed` travel arrival that
  # only cleared `travel_intent` (not `AgentTarget3d`) let the mid-route
  # gate "rediscover" the same door and restart the lifecycle forever
  # instead of settling at this deterministic terminal.

  Scenario: A-B-A -- unlocked travel completes, a locked travel fails deterministically, unlocking again completes
    # A: unlocked travel reaches the destination cell.
    Given a fresh door-link state
    When the door-link reaches travel door 0x99 to cell 0xC0DE
    And the door-link ticks with the door open
    Then the door-link state is traversing door 0x99
    When the door-link traversal completes
    Then the door-link state is travel-reached for door 0x99 to cell 0xC0DE

    # B: a locked travel door never opens through the scripted boundary and
    # gives up after the deterministic wait bound -- no hand-off.
    Given a fresh door-link state
    When the door-link reaches travel door 0x99 to cell 0xC0DE
    And the door-link ticks 120 times with the door closed
    Then the door-link state is failed for door 0x99

    # A again: reissuing the travel after the door is unlocked restarts the
    # lifecycle cleanly from the failed terminal (the existing one-repath
    # retry contract -- `door_link::transition`'s own table lets `Idle`,
    # `Failed`, and `TravelReached` all leave on a fresh `LinkReached`) and
    # completes the hand-off exactly like the first attempt.
    When the door-link reaches travel door 0x99 to cell 0xC0DE
    And the door-link ticks with the door open
    Then the door-link state is traversing door 0x99
    When the door-link traversal completes
    Then the door-link state is travel-reached for door 0x99 to cell 0xC0DE
