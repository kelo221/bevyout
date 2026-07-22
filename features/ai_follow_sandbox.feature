Feature: AI follow and sandbox package families (#198)
  The Follow family trails a moving leader within a hysteresis distance band,
  names a door it cannot open rather than re-pathing forever, and handles
  target loss. The Sandbox family roams deterministically within a radius,
  idling between legs. Both drive the same request/lifecycle mechanism as the
  other five families and never move the actor themselves.

  Scenario: A follower holds its distance band and idles when the leader is near
    Given a follow family with band 2 to 5 and tolerance 0.5
    When the leader is at (3, 0, 0)
    And the follower at (0, 0, 0) ticks
    Then the family requests the idle animation

  Scenario: A follower chases a leader that drifts outside the band
    Given a follow family with band 2 to 5 and tolerance 0.5
    When the leader is at (9, 0, 0)
    And the follower at (0, 0, 0) ticks
    Then the family requests a route to (9, 0, 0)

  Scenario: Hysteresis keeps a closing follower from stuttering at the band edge
    Given a follow family with band 2 to 5 and tolerance 0.5
    When the leader is at (9, 0, 0)
    And the follower at (0, 0, 0) ticks
    Then the family requests a route to (9, 0, 0)
    When the follower at (5.5, 0, 0) ticks
    Then the follow family keeps closing without stopping
    When the follower at (7.5, 0, 0) ticks
    Then the follow family stops routing

  Scenario: A blocked follower names the unreachable door and abandons
    Given a follow family with band 2 to 5 and tolerance 0.5
    When the leader is at (20, 0, 0)
    And the follower at (0, 0, 0) ticks
    Then the family requests a route to (20, 0, 0)
    When the route is blocked by locked door 00011abc
    And the follower at (5, 0, 0) ticks
    Then the follow family names blocking door 00011abc and abandons

  Scenario: A follower whose leader vanishes stops and keeps running
    Given a follow family with band 2 to 5 and tolerance 0.5
    When the leader is at (9, 0, 0)
    And the follower at (0, 0, 0) ticks
    Then the family requests a route to (9, 0, 0)
    When the leader is lost
    And the follower at (2, 0, 0) ticks
    Then the follow family stops routing and keeps the package running

  Scenario: A sandbox actor roams within its radius and idles between legs
    Given a sandbox family roaming within 6 of (10, 0, 10) seeded 3735928559
    When the sandbox actor ticks at (10, 0, 10)
    Then the sandbox family routes within the roam radius
    When the sandbox actor arrives at its roam point
    Then the family requests the idle animation
