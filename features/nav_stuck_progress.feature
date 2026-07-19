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
