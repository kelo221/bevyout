Feature: bevy_landmass navigation backend spike
  # Issue #112 (M4 wave 3). Three pure seams under test, all synthetic
  # in-memory fixtures (no game data, no committed binaries):
  #
  # - `viewer::nav::landmass_graph` converts a prepared-nav-graph-shaped
  #   mesh into a validated `bevy_landmass` navigation mesh: vertex/polygon
  #   mapping, winding (including the retry-with-reversed-order fallback),
  #   non-walkable (water) exclusion, and degenerate/invalid-polygon
  #   diagnostics.
  # - The same module's door-link descriptor extraction: a door FormID
  #   associated with exactly two triangles across the graph's meshes
  #   becomes one descriptor, deterministically.
  # - The same module's `landmass` agent-state -> project-native status
  #   mapping, and `viewer::nav::door_link`'s pure pause/resume/failure
  #   state machine.

  Scenario: A known-good square mesh converts and validates with no diagnostics
    Given a landmass mesh 0x00000010
    And landmass mesh 0x00000010 has vertex 0 at 0, 0, 0
    And landmass mesh 0x00000010 has vertex 1 at 1, 0, 0
    And landmass mesh 0x00000010 has vertex 2 at 0, 0, 1
    And landmass mesh 0x00000010 has vertex 3 at 1, 0, 1
    And landmass mesh 0x00000010 has polygon 0 with vertices 0,1,2
    And landmass mesh 0x00000010 has polygon 1 with vertices 1,3,2
    When the mesh is converted to a landmass navigation mesh
    Then the landmass conversion produces a navigation mesh
    And the landmass conversion has no diagnostics

  Scenario: Reversed source winding still validates after the retry, with a warning
    Given a landmass mesh 0x00000010
    And landmass mesh 0x00000010 has vertex 0 at 0, 0, 0
    And landmass mesh 0x00000010 has vertex 1 at 1, 0, 0
    And landmass mesh 0x00000010 has vertex 2 at 0, 0, 1
    And landmass mesh 0x00000010 has vertex 3 at 1, 0, 1
    And landmass mesh 0x00000010 has polygon 0 with vertices 0,2,1
    And landmass mesh 0x00000010 has polygon 1 with vertices 1,2,3
    When the mesh is converted to a landmass navigation mesh
    Then the landmass conversion produces a navigation mesh
    And the landmass conversion has a "warning" diagnostic containing "required reversal"

  Scenario: A water polygon is excluded as non-walkable
    Given a landmass mesh 0x00000010
    And landmass mesh 0x00000010 has vertex 0 at 0, 0, 0
    And landmass mesh 0x00000010 has vertex 1 at 1, 0, 0
    And landmass mesh 0x00000010 has vertex 2 at 0, 0, 1
    And landmass mesh 0x00000010 has vertex 3 at 1, 0, 1
    And landmass mesh 0x00000010 has polygon 0 with vertices 0,1,2
    And landmass mesh 0x00000010 has polygon 1 with vertices 1,3,2
    And landmass mesh 0x00000010 polygon 1 is water
    When the mesh is converted to a landmass navigation mesh
    Then the landmass conversion produces a navigation mesh

  Scenario: An all-water mesh produces no navigation mesh
    Given a landmass mesh 0x00000010
    And landmass mesh 0x00000010 has vertex 0 at 0, 0, 0
    And landmass mesh 0x00000010 has vertex 1 at 1, 0, 0
    And landmass mesh 0x00000010 has vertex 2 at 0, 0, 1
    And landmass mesh 0x00000010 has polygon 0 with vertices 0,1,2
    And landmass mesh 0x00000010 polygon 0 is water
    When the mesh is converted to a landmass navigation mesh
    Then the landmass conversion produces no navigation mesh
    And the landmass conversion has a "warning" diagnostic containing "no walkable polygons"

  Scenario: A degenerate polygon is skipped with a warning diagnostic
    Given a landmass mesh 0x00000010
    And landmass mesh 0x00000010 has vertex 0 at 0, 0, 0
    And landmass mesh 0x00000010 has vertex 1 at 1, 0, 0
    And landmass mesh 0x00000010 has vertex 2 at 0, 0, 1
    And landmass mesh 0x00000010 has polygon 0 with vertices 0,1,2
    And landmass mesh 0x00000010 has polygon 1 with vertices 1,1,2
    When the mesh is converted to a landmass navigation mesh
    Then the landmass conversion produces a navigation mesh
    And the landmass conversion has a "warning" diagnostic containing "degenerate triangle"

  Scenario: An invalid vertex index is skipped with an error diagnostic and never panics
    Given a landmass mesh 0x00000010
    And landmass mesh 0x00000010 has vertex 0 at 0, 0, 0
    And landmass mesh 0x00000010 has vertex 1 at 1, 0, 0
    And landmass mesh 0x00000010 has vertex 2 at 0, 0, 1
    And landmass mesh 0x00000010 has polygon 0 with vertices 0,1,2
    And landmass mesh 0x00000010 has polygon 1 with vertices 1,2,9
    When the mesh is converted to a landmass navigation mesh
    Then the landmass conversion produces a navigation mesh
    And the landmass conversion has an "error" diagnostic containing "invalid vertex index"

  Scenario: A door associated with exactly two triangles becomes one deterministic descriptor
    Given a landmass mesh 0x00000010
    And landmass mesh 0x00000010 has vertex 0 at 0, 0, 0
    And landmass mesh 0x00000010 has vertex 1 at 1, 0, 0
    And landmass mesh 0x00000010 has vertex 2 at 0, 0, 1
    And landmass mesh 0x00000010 has polygon 0 with vertices 0,1,2
    And landmass mesh 0x00000010 has a door 0x00000D00 at polygon 0
    And a landmass mesh 0x00000020
    And landmass mesh 0x00000020 has vertex 0 at 3, 0, 0
    And landmass mesh 0x00000020 has vertex 1 at 4, 0, 0
    And landmass mesh 0x00000020 has vertex 2 at 3, 0, 1
    And landmass mesh 0x00000020 has polygon 0 with vertices 0,1,2
    And landmass mesh 0x00000020 has a door 0x00000D00 at polygon 0
    When the door-link descriptors are extracted
    Then there is 1 door-link descriptor
    And door-link descriptor 0 links door 0x00000D00 between mesh 0x00000010 and mesh 0x00000020
    When the door-link descriptors are extracted again
    Then both door-link descriptor extractions are identical

  Scenario: A door associated with only one triangle produces no descriptor
    Given a landmass mesh 0x00000010
    And landmass mesh 0x00000010 has vertex 0 at 0, 0, 0
    And landmass mesh 0x00000010 has vertex 1 at 1, 0, 0
    And landmass mesh 0x00000010 has vertex 2 at 0, 0, 1
    And landmass mesh 0x00000010 has polygon 0 with vertices 0,1,2
    And landmass mesh 0x00000010 has a door 0x00000D00 at polygon 0
    When the door-link descriptors are extracted
    Then there are 0 door-link descriptors

  Scenario Outline: landmass agent states map to project-native nav agent statuses
    Then landmass agent state "<state>" maps to nav agent status "<status>"

    Examples:
      | state              | status      |
      | Idle                | idle        |
      | Moving              | moving      |
      | ReachedAnimationLink | moving      |
      | UsingAnimationLink  | moving      |
      | ReachedTarget       | reached     |
      | AgentNotOnNavMesh   | unreachable |
      | TargetNotOnNavMesh  | unreachable |
      | NoPath              | unreachable |
      | Paused              | paused      |

  Scenario: The door-link state machine pauses, resumes, and idles on a normal traversal
    Given a fresh door-link state
    When the door-link reaches door 0x00000D00
    Then the door-link state is paused for door 0x00000D00
    When the door-link ticks with the door closed
    Then the door-link state is paused for door 0x00000D00
    When the door-link ticks with the door open
    Then the door-link state is traversing door 0x00000D00
    When the door-link traversal completes
    Then the door-link state is idle

  Scenario: The door-link state machine fails deterministically after the wait bound
    Given a fresh door-link state
    When the door-link reaches door 0x00000D00
    And the door-link ticks 120 times with the door closed
    Then the door-link state is failed for door 0x00000D00
