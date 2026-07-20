Feature: Collision-derived navmesh validation and clearance
  # Issue #153 (M4 wave 10). `vsa::prepare::nav_clearance` validates the
  # authored FO3 NAVM against the cell's cooked static collision, prepare-side,
  # before the graph reaches the runtime. Behaviours under test on synthetic
  # geometry:
  #   F153.1 -- a walkable triangle with no collision surface under it (the
  #     authored mesh paved over a void) is removed as unsupported.
  #   F153.2 -- a tall wall-like collider on an interior triangle cuts it,
  #     while a step-overable riser (stairs) does not.
  #   F153.3 -- the agent-radius clearance-fit test, measured on authored
  #     geometry: a ~1 m doorway stays connected, a genuine sub-diameter
  #     pinch disconnects. Verified with the connected-component diagnostic.

  Scenario: A triangle over a void is removed as unsupported
    Given a clearance mesh
    And clearance mesh has vertex 0 at 0, 0, 0
    And clearance mesh has vertex 1 at 4, 0, 0
    And clearance mesh has vertex 2 at 4, 0, 2
    And clearance mesh has vertex 3 at 0, 0, 2
    And clearance mesh has polygon 0 with vertices 0,1,2
    And clearance mesh has polygon 1 with vertices 0,2,3
    # Floor only reaches x=1.5, so polygon 0's centroid and every edge
    # midpoint (nearest at x=2) sit over the void -> removed; a single
    # supported sample would keep it (crack tolerance).
    And a collision floor from 0, 1.5 by -0.5, 2.5 at height 0
    When the clearance pass runs
    Then 1 polygon is removed as unsupported
    And clearance polygon 0 is not walkable

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

  Scenario: A tall interior collider cuts only the interior triangle it sits on
    # A big triangle midpoint-subdivided into four; polygon 3 is the one fully
    # interior triangle. A tall wall stub stands on it.
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
    Then 1 polygon is cut as obstructed
    And clearance polygon 3 is not walkable

  Scenario: A step-overable riser does not cut the stair tread
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
    And a collision wall from 1.2, 1.5 at z 1.33 from 0 to 0.3
    When the clearance pass runs
    Then 0 polygons are cut as obstructed

  Scenario: A one-metre doorway stays connected with an eroded passage
    # A uniform 1.0 m corridor (half-width 0.5 > agent radius 0.35): its
    # centre line clears each wall, so nothing drops and it stays one
    # connected component -- the wave-6 miter regression must not reappear.
    Given a clearance mesh
    And clearance mesh has vertex 0 at 0, 0, 0.5
    And clearance mesh has vertex 1 at 0, 0, 1.5
    And clearance mesh has vertex 2 at 2, 0, 0.5
    And clearance mesh has vertex 3 at 2, 0, 1.5
    And clearance mesh has vertex 4 at 4, 0, 0.5
    And clearance mesh has vertex 5 at 4, 0, 1.5
    And clearance mesh has polygon 0 with vertices 0,2,3
    And clearance mesh has polygon 1 with vertices 0,3,1
    And clearance mesh has polygon 2 with vertices 2,4,5
    And clearance mesh has polygon 3 with vertices 2,5,3
    When the clearance pass runs
    Then 0 polygons are dropped as unfit
    And the walkable set forms 1 connected component
    And the largest connected component has 4 polygons

  Scenario: A sub-diameter pinch disconnects the two wide ends
    # Wide (half-width 1.0) at both ends, pinched to a 0.5 m gap (half-width
    # 0.25 < agent radius) in the middle: the neck fits nowhere and drops,
    # splitting the corridor into two components.
    Given a clearance mesh
    And clearance mesh has vertex 0 at 0, 0, 0
    And clearance mesh has vertex 1 at 0, 0, 2
    And clearance mesh has vertex 2 at 2, 0, 0.75
    And clearance mesh has vertex 3 at 2, 0, 1.25
    And clearance mesh has vertex 4 at 4, 0, 0.75
    And clearance mesh has vertex 5 at 4, 0, 1.25
    And clearance mesh has vertex 6 at 6, 0, 0
    And clearance mesh has vertex 7 at 6, 0, 2
    And clearance mesh has polygon 0 with vertices 0,2,3
    And clearance mesh has polygon 1 with vertices 0,3,1
    And clearance mesh has polygon 2 with vertices 2,4,5
    And clearance mesh has polygon 3 with vertices 2,5,3
    And clearance mesh has polygon 4 with vertices 4,6,7
    And clearance mesh has polygon 5 with vertices 4,7,5
    When the clearance pass runs
    Then at least 1 polygon is dropped as unfit
    And the walkable set forms 2 connected components
