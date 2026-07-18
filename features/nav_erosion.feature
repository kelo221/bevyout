Feature: Navmesh clearance erosion for agent radius
  # Issue #136 (M4 wave 6). `viewer::nav::erosion_policy` erodes the
  # walkable polygon boundary inward by the agent radius before
  # `bevy_landmass` sees the mesh, so its path smoothing cannot string-pull
  # a route within less than a capsule-width of a wall/prop collider (the
  # wedging regression M4 wave 5's physics-authoritative movement exposed).
  # Two pure behaviours under test: a room corner is pulled diagonally off
  # the wall by the full agent radius, and a corridor narrower than twice
  # the agent radius engages the corridor-pinch guard (F136.2) instead of
  # inverting or degenerating.

  Scenario: A room corner is pulled diagonally off the wall by the agent radius
    Given an erosion mesh
    And erosion mesh has vertex 0 at 0, 0, 0
    And erosion mesh has vertex 1 at 4, 0, 0
    And erosion mesh has vertex 2 at 4, 0, 4
    And erosion mesh has vertex 3 at 0, 0, 4
    And erosion mesh has polygon 0 with vertices 0,1,2
    And erosion mesh has polygon 1 with vertices 0,2,3
    When the erosion mesh is eroded by radius 0.35
    Then eroded vertex 0 moved into the room on both the x and z axes
    And the erosion pinch guard count is 0

  Scenario: A room corner keeps the full agent radius clearance from each wall (miter offset)
    # Regression (issue #136 follow-up, external review): a fixed-length
    # (non-mitered) corner offset only gave each wall radius/sqrt(2)
    # (~0.247 m) clearance at this 90-degree corner instead of the full
    # 0.35 m agent radius. The offset must be a miter -- scaled so BOTH
    # walls keep at least the full radius, measured against the original
    # (pre-erosion) wall lines, not the post-erosion displacement.
    Given an erosion mesh
    And erosion mesh has vertex 0 at 0, 0, 0
    And erosion mesh has vertex 1 at 4, 0, 0
    And erosion mesh has vertex 2 at 4, 0, 4
    And erosion mesh has vertex 3 at 0, 0, 4
    And erosion mesh has polygon 0 with vertices 0,1,2
    And erosion mesh has polygon 1 with vertices 0,2,3
    When the erosion mesh is eroded by radius 0.35
    Then eroded vertex 0 keeps at least 0.35 clearance from the wall through vertices 0 and 1
    And eroded vertex 0 keeps at least 0.35 clearance from the wall through vertices 3 and 0
    And the erosion pinch guard count is 0

  Scenario: A corridor narrower than twice the agent radius engages the pinch guard without inverting
    Given an erosion mesh
    And erosion mesh has vertex 0 at 0, 0, 0
    And erosion mesh has vertex 1 at 2, 0, 0
    And erosion mesh has vertex 2 at 2, 0, 0.3
    And erosion mesh has vertex 3 at 0, 0, 0.3
    And erosion mesh has polygon 0 with vertices 0,1,2
    And erosion mesh has polygon 1 with vertices 0,2,3
    When the erosion mesh is eroded by radius 0.35
    Then the erosion pinch guard count is greater than 0
    And every eroded polygon keeps its original winding sign

  Scenario: A corridor much wider than twice the agent radius erodes fully without the pinch guard
    Given an erosion mesh
    And erosion mesh has vertex 0 at 0, 0, 0
    And erosion mesh has vertex 1 at 2, 0, 0
    And erosion mesh has vertex 2 at 2, 0, 3
    And erosion mesh has vertex 3 at 0, 0, 3
    And erosion mesh has polygon 0 with vertices 0,1,2
    And erosion mesh has polygon 1 with vertices 0,2,3
    When the erosion mesh is eroded by radius 0.35
    Then the erosion pinch guard count is 0

  Scenario: A shared vertex's combined push never inverts a neighboring triangle
    # Regression (real-data failure on Vault 101 Entrance mesh 0001862b,
    # polygon 164: "concave or has edges in clockwise order"). Vertex 0
    # is a narrow-corridor corner (polygons 0 and 1) that also anchors an
    # unrelated thin triangle (polygon 2) sticking out to one side. The
    # corridor's own boundary edges push vertex 0 hard enough that, at
    # full erosion strength, the thin triangle it shares no boundary edge
    # with still flips -- exactly the "a shared vertex's combined
    # displacement inverts a neighboring polygon" class a per-polygon-
    # isolated check misses.
    Given an erosion mesh
    And erosion mesh has vertex 0 at 0, 0, 0
    And erosion mesh has vertex 1 at 2, 0, 0
    And erosion mesh has vertex 2 at 2, 0, 0.1
    And erosion mesh has vertex 3 at 0, 0, 0.1
    And erosion mesh has vertex 4 at -1, 0, 0.05
    And erosion mesh has vertex 5 at -1, 0, -0.05
    And erosion mesh has polygon 0 with vertices 0,1,2
    And erosion mesh has polygon 1 with vertices 0,2,3
    And erosion mesh has polygon 2 with vertices 0,4,5
    When the erosion mesh is eroded by radius 0.35
    Then the erosion pinch guard count is greater than 0
    And the erosion relax passes is greater than 0
    And every eroded polygon keeps its original winding sign
