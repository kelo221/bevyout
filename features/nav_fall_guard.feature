Feature: A nav agent that falls out of the world is caught deterministically
  # Issue #164. FranklinMetro02 (cell 0001a273) has walkable navmesh over
  # regions with no collision floor beneath them: an agent that steps onto
  # such a region is caught by no collider and descends under gravity
  # forever, its `tna status` pinned at `status=unreachable stuck=true`. The
  # fall guard ends that descent. Its decision is a pure policy
  # (`viewer::nav::fall_guard`): given the active cell's minimum prepared
  # geometry Y and an agent's current capsule-centre Y, it decides whether
  # the agent has dropped a whole margin below the lowest authored surface --
  # unambiguously out of the world. The kill plane is always derived from the
  # cell's own bounds, never a hard-coded world Y. The Bevy guard system in
  # `nav::agent` samples the real capsule Y and cell bounds and does nothing
  # but feed them here; these scenarios exercise the pure policy directly (no
  # Bevy/boxddd involved).

  Scenario: The kill plane is a fixed margin below the cell's minimum geometry Y
    Given a cell whose minimum geometry Y is 94.168
    When the fall kill plane is computed
    Then the fall kill plane is 5.0 metres below the minimum geometry Y

  Scenario: An agent resting on the lowest walkable surface is in bounds
    Given a cell whose minimum geometry Y is 94.168
    And a nav agent at Y 95.0
    When the fall guard is evaluated
    Then the fall guard reports the agent is in bounds

  Scenario: An agent resting exactly at the kill plane is still in bounds
    Given a cell whose minimum geometry Y is 94.168
    And a nav agent resting exactly at the kill plane
    When the fall guard is evaluated
    Then the fall guard reports the agent is in bounds

  Scenario: An agent that dropped just past the kill plane has fallen out of the world
    Given a cell whose minimum geometry Y is 94.168
    And a nav agent just below the kill plane
    When the fall guard is evaluated
    Then the fall guard reports the agent has fallen out of the world

  Scenario: An agent descending without bound through a missing floor has fallen out of the world
    Given a cell whose minimum geometry Y is 94.168
    And a nav agent at Y -1000.0
    When the fall guard is evaluated
    Then the fall guard reports the agent has fallen out of the world
