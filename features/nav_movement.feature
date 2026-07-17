Feature: Physics-authoritative nav agent movement policy
  # Issue #114 (M4 wave 5). Pure decision tables under test, no Bevy/boxddd
  # involved (`viewer::nav::movement_policy` is std-only): grounded/airborne
  # transitions, collision-rejection clamping (desired vs. achieved
  # velocity), and stuck detection/recovery. The Bevy system
  # (`nav::agent::apply_agent_physics_movement`) only feeds these functions
  # observations sampled from the real `bevy_boxddd` KCC sweep; the
  # minimal-World/App coverage for that wiring lives in
  # `src/viewer/nav/agent.rs`'s own `#[cfg(test)]` module (see
  # `agent_kcc_settles_onto_a_flat_floor_via_physics_collision` and
  # `a_blocked_agent_reports_its_real_near_zero_velocity`).

  Scenario: A walkable plane alone is enough to be grounded
    Given a grounded observation with walkable plane false and stepped up false
    When the grounded decision is made
    Then the agent is not grounded

  Scenario: Either a walkable plane or a successful step-up grounds the agent
    Given a grounded observation with walkable plane true and stepped up false
    When the grounded decision is made
    Then the agent is grounded

  Scenario: A successful step-up alone grounds the agent
    Given a grounded observation with walkable plane false and stepped up true
    When the grounded decision is made
    Then the agent is grounded

  Scenario: No desired motion is never collision-blocked
    Given a velocity observation with desired speed 0.0 and achieved speed 0.0
    When the collision outcome decision is made
    Then the collision outcome is clear

  Scenario: A wall square in front of the agent is collision-blocked
    Given a velocity observation with desired speed 2.5 and achieved speed 0.05
    When the collision outcome decision is made
    Then the collision outcome is blocked

  Scenario: Sliding along a wall at reduced but non-trivial speed stays clear
    Given a velocity observation with desired speed 2.5 and achieved speed 1.2
    When the collision outcome decision is made
    Then the collision outcome is clear

  Scenario: Progress toward the waypoint resets the stuck window
    Given a stuck observation with distance 4.0, best distance 5.0, ticks without progress 200, recovery active true
    When the stuck decision is made
    Then the stuck decision is progressing

  Scenario: No progress within the recovery window keeps progressing
    Given a stuck observation with distance 5.0, best distance 5.0, ticks without progress 59, recovery active false
    When the stuck decision is made
    Then the stuck decision is progressing

  Scenario: Exhausting the recovery window starts recovery exactly once
    Given a stuck observation with distance 5.0, best distance 5.0, ticks without progress 60, recovery active false
    When the stuck decision is made
    Then the stuck decision is start-recovery

  Scenario: A recovery already in flight does not restart
    Given a stuck observation with distance 5.0, best distance 5.0, ticks without progress 60, recovery active true
    When the stuck decision is made
    Then the stuck decision is recovery-pending

  Scenario: Exhausting the failure window after recovery fails deterministically
    Given a stuck observation with distance 5.0, best distance 5.0, ticks without progress 120, recovery active true
    When the stuck decision is made
    Then the stuck decision is stuck
