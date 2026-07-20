Feature: Nav agent locomotion animation state (issue #188)
  A projected actor bound to a nav agent drives its animation state from the
  motion the KCC actually achieved, never from the motion navigation merely
  desired and never from the animation clip itself. Every threshold carries a
  hysteresis band so an agent hovering at a boundary holds one clip instead of
  flapping between two.

  Scenario Outline: achieved horizontal speed selects the locomotion clip
    Given a bound actor currently in the <previous> locomotion state
    When its achieved horizontal speed is <speed> metres per second
    Then its locomotion state becomes <expected>

    Examples:
      | previous | speed | expected |
      | idle     | 0.0   | idle     |
      | idle     | 0.10  | idle     |
      | idle     | 0.30  | walk     |
      | idle     | 2.50  | run      |
      | walk     | 0.20  | walk     |
      | walk     | 0.10  | idle     |
      | walk     | 1.80  | run      |
      | run      | 1.50  | run      |
      | run      | 1.30  | walk     |
      | run      | 0.05  | idle     |

  Scenario: the walk threshold has a hysteresis band on both edges
    Given a bound actor currently in the idle locomotion state
    When its achieved horizontal speed is 0.20 metres per second
    Then its locomotion state becomes idle
    Given a bound actor currently in the walk locomotion state
    When its achieved horizontal speed is 0.20 metres per second
    Then its locomotion state becomes walk

  Scenario: the run threshold has a hysteresis band on both edges
    Given a bound actor currently in the walk locomotion state
    When its achieved horizontal speed is 1.50 metres per second
    Then its locomotion state becomes walk
    Given a bound actor currently in the run locomotion state
    When its achieved horizontal speed is 1.50 metres per second
    Then its locomotion state becomes run

  Scenario: a speed oscillating across the raw walk/run threshold holds one state
    Given a bound actor currently in the walk locomotion state
    When its achieved horizontal speed oscillates 20 times between 1.50 and 1.70 metres per second
    Then its locomotion state never changed
    Then its locomotion state becomes walk

  Scenario: a speed oscillating across the raw idle/walk threshold holds one state
    Given a bound actor currently in the idle locomotion state
    When its achieved horizontal speed oscillates 20 times between 0.13 and 0.24 metres per second
    Then its locomotion state never changed
    Then its locomotion state becomes idle

  Scenario Outline: a stationary agent turning in place selects a turn clip
    Given a bound actor currently in the <previous> locomotion state
    When it is stationary and its yaw rate is <rate> radians per second
    Then its locomotion state becomes <expected>

    Examples:
      | previous   | rate  | expected   |
      | idle       | 0.0   | idle       |
      | idle       | 0.30  | idle       |
      | idle       | 1.00  | turn_left  |
      | idle       | -1.00 | turn_right |
      | turn_left  | 0.30  | turn_left  |
      | turn_left  | 0.10  | idle       |
      | turn_right | -0.30 | turn_right |
      | turn_right | -0.10 | idle       |

  Scenario: translation beats turning so a walking agent rounding a corner keeps walking
    Given a bound actor currently in the walk locomotion state
    When its achieved horizontal speed is 1.00 metres per second and its yaw rate is 2.00 radians per second
    Then its locomotion state becomes walk

  Scenario: a wedged agent that cannot move plays idle rather than walking on the spot
    Given a bound actor currently in the walk locomotion state
    When navigation desires 2.50 metres per second but the KCC achieves 0.00
    Then its locomotion state becomes idle
