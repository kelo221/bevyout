Feature: Stuck detection measures corridor progress, not final-target distance
  # Issue #157. `movement_policy::decide_stuck` is fed a corridor-progress
  # signal -- `route_progress_delta` (this tick's real, KCC-resolved
  # horizontal motion projected onto landmass's *current* steering
  # direction) integrated over the route -- instead of monotone distance to
  # the final `AgentTarget3d`. See `movement_policy.rs`'s module doc comment
  # for the full rationale. `nav::agent::apply_agent_physics_movement`
  # samples `route_progress_delta` every fixed tick; these scenarios
  # simulate that tick loop directly against the pure `movement_policy`
  # functions (no Bevy/boxddd involved).

  Scenario: route progress delta rewards achieved motion along the desired direction
    Given a desired horizontal velocity of 2.0, 0.0
    And an achieved horizontal velocity of 1.5, 0.0
    When the route progress delta is computed
    Then the route progress delta is 1.5

  Scenario: route progress delta ignores achieved motion when nothing is desired
    Given a desired horizontal velocity of 0.0, 0.0
    And an achieved horizontal velocity of 3.0, 3.0
    When the route progress delta is computed
    Then the route progress delta is 0.0

  Scenario: route progress delta penalizes achieved motion opposite the desired direction
    Given a desired horizontal velocity of 1.0, 0.0
    And an achieved horizontal velocity of -0.5, 0.0
    When the route progress delta is computed
    Then the route progress delta is -0.5

  # The invariant this whole issue exists for: a route leg that must move
  # away from the final target (around a wall, doubling back through a
  # doorway) is still corridor progress the whole way, as long as the agent
  # is actually walking it -- unlike distance-to-final-target, which
  # regresses for the entire detour leg and used to false-trigger recovery.
  Scenario: A U-shaped detour that moves away from the final target never latches stuck recovery
    Given a U-shaped detour route of 200 ticks where the agent always achieves its desired horizontal velocity
    When the route is simulated tick by tick
    Then no stuck decision along the route ever reaches start-recovery

  # Same tick budget (`STUCK_RECOVERY_TICKS`/`STUCK_FAILURE_TICKS`, both 60)
  # as the pre-#157 distance-to-target behaviour: a route where the agent
  # never achieves any horizontal motion despite desired motion is still
  # genuinely stuck, not a detour. `decide_stuck` reads the tick counter
  # *before* this tick's increment (matching `apply_agent_physics_movement`
  # exactly), so the transitions land one tick after the raw
  # `STUCK_RECOVERY_TICKS`/`STUCK_RECOVERY_TICKS + STUCK_FAILURE_TICKS`
  # thresholds (61, 121) rather than on them.
  Scenario: A genuinely blocked route still latches stuck recovery in a comparable tick budget
    Given a fully blocked route of 130 ticks with desired horizontal speed 2.0
    When the route is simulated tick by tick
    Then the stuck decision first reaches start-recovery at tick 61
    And the stuck decision first reaches stuck at tick 121

  # Known limitation (external architecture review, issue #157 follow-up),
  # documented in `movement_policy.rs`'s module doc comment: because
  # `route_progress_delta` only ever compares a tick's achieved motion
  # against *that same tick's* desired direction, an agent whose desired
  # direction keeps flipping under oscillating avoidance steering -- and
  # which fully achieves each flip -- reads as perpetual corridor progress
  # here, even though it is effectively orbiting in place. This is NOT
  # desired behaviour; it is the accepted trade-off for this signal, pinned
  # so it cannot silently regress further. The signal's actual field
  # failure mode -- a genuine collision wedge, where achieved motion stays
  # near-zero regardless of the desired direction -- still flatlines and
  # still latches stuck recovery (see the "genuinely blocked route"
  # scenario above).
  Scenario: Known limitation - oscillating but fully-achieved steering never latches stuck recovery
    Given an oscillating route of 200 ticks where the agent always achieves its desired horizontal velocity
    When the route is simulated tick by tick
    Then no stuck decision along the route ever reaches start-recovery

  # Pins today's actual behaviour for a legitimate mid-route avoidance
  # pause (landmass asking for zero horizontal motion for a stretch, e.g.
  # queuing at a doorway) rather than leaving it implicit. Zero desired
  # velocity contributes exactly zero corridor progress
  # (`route_progress_delta`'s near-zero-desired-length guard), so a pause
  # this long flatlines the signal exactly like a genuine block would and
  # does latch stuck recovery/failure on the same
  # `STUCK_RECOVERY_TICKS`/`STUCK_FAILURE_TICKS` budget -- this scenario
  # asserts that current behaviour, not a claim that it is the *right*
  # behaviour for a legitimate stall.
  Scenario: An avoidance pause with no desired motion contributes no progress and eventually latches stuck today
    Given an avoidance-paused route with 10 ticks of progress at speed 2.0 followed by 140 ticks of zero desired velocity
    When the paused route is simulated tick by tick
    Then the stuck decision first reaches start-recovery at tick 70
    And the stuck decision first reaches stuck at tick 130

  # A repath / new target lands right at a near-miss (one tick shy of the
  # recovery threshold from an unrelated earlier block). `nav/agent.rs`'s
  # target-change handler resets `best_distance` to `f32::MAX`,
  # `ticks_without_progress` to 0, and `recovery_active` to false at that
  # point (untouched by issue #157); this scenario pins that the reset
  # means the new detour leg's own slow-but-genuine progress is judged on
  # its own terms and does not inherit the old window. Without that reset
  # this exact setup false-triggers start-recovery one tick into the new
  # leg.
  Scenario: A repath mid-route does not inherit a stale no-progress window
    Given a blocked route of 59 ticks that repaths onto a new detour leg of 200 ticks at speed 0.5
    When the repathed route is simulated tick by tick
    Then no stuck decision along the route ever reaches start-recovery
