Feature: Collision-derived navmesh validation and clearance
  # Issue #153 (M4 wave 10), extended to sub-triangle granularity by issue
  # #171 (M4 wave 11). `vsa::prepare::nav_clearance` validates the authored
  # FO3 NAVM against the cell's cooked static collision, prepare-side, before
  # the graph reaches the runtime. Wave 11 replaced the whole-triangle
  # support/obstruction verdicts with a local re-triangulation
  # (`vsa::prepare::nav_clip`) against a pointwise walkability predicate, so
  # the surviving edges lie on the obstruction/void boundary itself rather
  # than on authored triangle edges. Behaviours under test on synthetic
  # geometry:
  #   F171.1 -- a collider intruding into a polygon is clipped out of it with
  #     an agent-radius margin, including posts flanking an opening whose
  #     centroid stays clear: the passage survives when it is wider than the
  #     agent diameter and closes when it is not.
  #   F171.2 -- a polygon straddling floor and void splits along the support
  #     boundary; the supported part stays walkable, the void part does not.
  #     Hairline seams in cooked collision are not voids.
  #   F153.3 -- the agent-radius clearance-fit test, measured on authored
  #     geometry: a ~1 m doorway stays connected, a genuine sub-diameter
  #     pinch disconnects. Verified with the connected-component diagnostic.

  Scenario: The void boundary is clipped out of a straddling triangle
    Given a clearance mesh
    And clearance mesh has vertex 0 at 0, 0, 0
    And clearance mesh has vertex 1 at 4, 0, 0
    And clearance mesh has vertex 2 at 4, 0, 2
    And clearance mesh has vertex 3 at 0, 0, 2
    And clearance mesh has polygon 0 with vertices 0,1,2
    And clearance mesh has polygon 1 with vertices 0,2,3
    # Floor only reaches x=1.5 while the nav quad spans x=0..4, so both
    # authored triangles straddle floor and void: no whole-triangle verdict
    # can express this, but the clip cuts each along the floor's edge.
    And a collision floor from 0, 1.5 by -0.5, 2.5 at height 0
    When the clearance pass runs
    Then at least 1 polygon is removed as unsupported
    And clearance point 0.5, 1.0 is walkable
    And clearance point 3.0, 1.0 is not walkable

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

  Scenario: A tall interior collider is clipped out with an agent-radius margin
    # A big triangle midpoint-subdivided into four, with a tall wall stub
    # standing inside it. The stub's footprint and the agent-radius margin
    # around it are clipped away; floor a clear distance from it survives.
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
    Then at least 1 polygon is cut as obstructed
    And clearance point 1.35, 1.33 is not walkable
    And clearance point 1.35, 1.15 is not walkable
    And clearance point 0.3, 0.3 is walkable

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

  Scenario: Posts flanking a triangle's opening leave a wide enough passage connected
    # F171.1 / the #148 metro-entrance class. Two posts flank an opening
    # *inside* a nav triangle, so the triangle's centroid stays clear and no
    # whole-triangle test can see them. The 1.4 m gap is wider than the 0.7 m
    # agent diameter, so the clip must leave a passage that still connects the
    # two sides.
    Given a clearance mesh
    And clearance mesh has vertex 0 at 0, 0, 0
    And clearance mesh has vertex 1 at 6, 0, 0
    And clearance mesh has vertex 2 at 6, 0, 6
    And clearance mesh has vertex 3 at 0, 0, 6
    And clearance mesh has polygon 0 with vertices 0,1,2
    And clearance mesh has polygon 1 with vertices 0,2,3
    And a collision floor from -1, 7 by -1, 7 at height 0
    And a collision wall from 0, 2.3 at z 3 from 0 to 2
    And a collision wall from 3.7, 6 at z 3 from 0 to 2
    When the clearance pass runs
    Then at least 1 polygon is cut as obstructed
    And clearance point 3.0, 3.0 is walkable
    And clearance points 3.0, 1.0 and 3.0, 5.0 are connected

  Scenario: Posts flanking a triangle's opening disconnect a sub-diameter passage
    # The same geometry with a 0.5 m gap -- narrower than the agent diameter.
    # The clip closes it and the two sides are honestly unreachable from one
    # another, so a route across is `unreachable` at query time instead of
    # wedging an agent in the frame.
    Given a clearance mesh
    And clearance mesh has vertex 0 at 0, 0, 0
    And clearance mesh has vertex 1 at 6, 0, 0
    And clearance mesh has vertex 2 at 6, 0, 6
    And clearance mesh has vertex 3 at 0, 0, 6
    And clearance mesh has polygon 0 with vertices 0,1,2
    And clearance mesh has polygon 1 with vertices 0,2,3
    And a collision floor from -1, 7 by -1, 7 at height 0
    And a collision wall from 0, 2.75 at z 3 from 0 to 2
    And a collision wall from 3.25, 6 at z 3 from 0 to 2
    When the clearance pass runs
    Then clearance point 3.0, 3.0 is not walkable
    And clearance points 3.0, 1.0 and 3.0, 5.0 are not connected

  Scenario: A hairline collision seam does not read as a void
    # Cooked static collision is assembled from independently placed meshes
    # that abut without welding, so hairline seams between floor placements are
    # void by the letter of the geometry. They must not punch holes in the nav
    # mesh.
    Given a clearance mesh
    And clearance mesh has vertex 0 at 0, 0, 0
    And clearance mesh has vertex 1 at 4, 0, 0
    And clearance mesh has vertex 2 at 4, 0, 4
    And clearance mesh has vertex 3 at 0, 0, 4
    And clearance mesh has polygon 0 with vertices 0,1,2
    And clearance mesh has polygon 1 with vertices 0,2,3
    And a collision floor from -1, 1.97 by -1, 5 at height 0
    And a collision floor from 2.03, 5 by -1, 5 at height 0
    When the clearance pass runs
    Then 0 polygons are removed as unsupported
    And clearance point 2.0, 2.0 is walkable
