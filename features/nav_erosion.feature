Feature: Retired runtime navmesh erosion is a no-op passthrough
  # Issue #136's interim runtime erosion was retired in issue #153 (M4
  # wave 10): collision-derived validation + clearance now runs prepare-side
  # in `vsa::prepare::nav_clearance` and is baked into the prepared graph, so
  # `viewer::nav::erosion_policy::erode` must never move a vertex (a second,
  # runtime erosion of the already clearance-offset prepared vertices would
  # over-shrink real doorways -- the wave-6 regression). The new clearance
  # semantics (miter corners, collision support removal, interior-obstruction
  # cutting, sub-diameter corridor disconnection) are covered by
  # `nav_collision_clearance.feature` and `nav_clearance.rs`'s unit tests.

  Scenario: Eroding a room corner leaves every vertex unchanged
    Given an erosion mesh
    And erosion mesh has vertex 0 at 0, 0, 0
    And erosion mesh has vertex 1 at 4, 0, 0
    And erosion mesh has vertex 2 at 4, 0, 4
    And erosion mesh has vertex 3 at 0, 0, 4
    And erosion mesh has polygon 0 with vertices 0,1,2
    And erosion mesh has polygon 1 with vertices 0,2,3
    When the erosion mesh is eroded by radius 0.35
    Then no eroded vertex moved
    And the erosion pinch guard count is 0
    And every eroded polygon keeps its original winding sign

  Scenario: Eroding a narrow corridor no longer moves or drops anything
    Given an erosion mesh
    And erosion mesh has vertex 0 at 0, 0, 0
    And erosion mesh has vertex 1 at 2, 0, 0
    And erosion mesh has vertex 2 at 2, 0, 0.3
    And erosion mesh has vertex 3 at 0, 0, 0.3
    And erosion mesh has polygon 0 with vertices 0,1,2
    And erosion mesh has polygon 1 with vertices 0,2,3
    When the erosion mesh is eroded by radius 0.35
    Then no eroded vertex moved
    And the erosion pinch guard count is 0
