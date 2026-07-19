Feature: Collision-derived navmesh validation and clearance
  # Issue #153 (M4 wave 10). `vsa::prepare::nav_clearance` validates the
  # authored FO3 NAVM against the cell's cooked static collision and applies
  # agent-radius clearance, prepare-side, before the graph reaches the
  # runtime. Three behaviours under test on synthetic geometry:
  #   F153.1 -- a walkable triangle with no collision surface under it (the
  #     authored mesh paved over a void) is removed as unsupported.
  #   F153.2 -- a wall-like collider rising into the agent capsule over a
  #     walkable triangle cuts that triangle, while the open doorway beside
  #     it stays walkable.
  #   F153.3 -- a corridor narrower than twice the agent radius disconnects
  #     (no route), while a wide room keeps its clearance offset without
  #     dropping anything.

  Scenario: A triangle over a void is removed as unsupported
    Given a clearance mesh
    And clearance mesh has vertex 0 at 0, 0, 0
    And clearance mesh has vertex 1 at 4, 0, 0
    And clearance mesh has vertex 2 at 4, 0, 2
    And clearance mesh has vertex 3 at 0, 0, 2
    And clearance mesh has polygon 0 with vertices 0,1,2
    And clearance mesh has polygon 1 with vertices 0,2,3
    And a collision floor from 0, 2 by -0.5, 2.5 at height 0
    When the clearance pass runs
    Then 1 polygon is removed as unsupported
    And clearance polygon 0 is not walkable
    And clearance polygon 1 is walkable

  Scenario: A fully supported room keeps every triangle
    Given a clearance mesh
    And clearance mesh has vertex 0 at 0, 0, 0
    And clearance mesh has vertex 1 at 4, 0, 0
    And clearance mesh has vertex 2 at 4, 0, 4
    And clearance mesh has vertex 3 at 0, 0, 4
    And clearance mesh has polygon 0 with vertices 0,1,2
    And clearance mesh has polygon 1 with vertices 0,2,3
    And a collision floor from -1, 5 by -1, 5 at height 0
    When the clearance pass runs
    Then 0 polygons are removed as unsupported
    And 0 polygons are cut as obstructed

  Scenario: An interior collider cuts only the interior triangle it sits on
    # A big triangle midpoint-subdivided into four: the centre triangle
    # (polygon 3) is the one fully interior triangle; a wall stub stands on
    # it. The three boundary corner triangles are left to the clearance
    # offset, so only the genuine interior obstruction (the #148 entrance-
    # frame class) is cut here.
    Given a clearance mesh
    And clearance mesh has vertex 0 at 0, 0, 0
    And clearance mesh has vertex 1 at 4, 0, 0
    And clearance mesh has vertex 2 at 0, 0, 4
    And clearance mesh has vertex 3 at 2, 0, 0
    And clearance mesh has vertex 4 at 2, 0, 2
    And clearance mesh has vertex 5 at 0, 0, 2
    And clearance mesh has polygon 0 with vertices 0,3,5
    And clearance mesh has polygon 1 with vertices 3,1,4
    And clearance mesh has polygon 2 with vertices 5,4,2
    And clearance mesh has polygon 3 with vertices 3,4,5
    And a collision floor from -1, 5 by -1, 5 at height 0
    And a collision wall from 1.2, 1.5 at z 1.33 from 0 to 2
    When the clearance pass runs
    Then 0 polygons are removed as unsupported
    And 1 polygon is cut as obstructed
    And clearance polygon 3 is not walkable
    And clearance polygon 0 is walkable

  Scenario: A sub-diameter corridor disconnects instead of being preserved
    Given a clearance mesh
    And clearance mesh has vertex 0 at 0, 0, 0
    And clearance mesh has vertex 1 at 2, 0, 0
    And clearance mesh has vertex 2 at 2, 0, 0.3
    And clearance mesh has vertex 3 at 0, 0, 0.3
    And clearance mesh has polygon 0 with vertices 0,1,2
    And clearance mesh has polygon 1 with vertices 0,2,3
    When the clearance pass runs
    Then at least 1 polygon is disconnected as narrow
    And at least one clearance polygon is not walkable

  Scenario: A wide room keeps its clearance offset without disconnecting
    Given a clearance mesh
    And clearance mesh has vertex 0 at 0, 0, 0
    And clearance mesh has vertex 1 at 4, 0, 0
    And clearance mesh has vertex 2 at 4, 0, 3
    And clearance mesh has vertex 3 at 0, 0, 3
    And clearance mesh has polygon 0 with vertices 0,1,2
    And clearance mesh has polygon 1 with vertices 0,2,3
    When the clearance pass runs
    Then 0 polygons are disconnected as narrow
    And every clearance polygon is walkable
    And at least one polygon was offset by clearance
