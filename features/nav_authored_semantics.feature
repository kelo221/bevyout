Feature: Authored NAVM semantics - preferred pathing, NVTR external-edge evidence, NVEX/NVCI correlation
  # Issue #156 (M4 wave 9). The ESM4 NAVM/NAVI decoder (#111/#113/#154/#155)
  # already surfaces authored semantics fopdoc documents -- preferred-pathing
  # triangle flags, per-edge NVTR external-edge flags, NVEX external
  # connections, and NAVI NVCI correlation entries -- but the prepared nav
  # graph and runtime conversion previously dropped everything except
  # vertices/triangles/water/doors. This feature drives the pure
  # `vsa::prepare::nav_graph` module directly (reusing nav_graph.feature's "a
  # nav graph mesh"/"has source vertex"/"has triangle"/"the nav graph is
  # built" steps, and nav_portals.feature's cross-mesh merge steps), plus a
  # small structural pin on `viewer::nav::landmass_graph`'s door/
  # preferred-pathing type-index precedence (that mapping's exhaustive
  # deterministic coverage lives in `landmass_graph.rs`'s own unit tests, the
  # same split issue #155's door-typing already established -- see this
  # feature's final scenario).
  #
  # NVEX/NVCI are correlation-only: this feature asserts counters and
  # diagnostics, never a runtime pathing effect (no exterior stitching is
  # built by this issue).

  Scenario: A preferred-pathing triangle flag reaches the prepared polygon
    Given a nav graph mesh 0x00000010 for cell 0x00000C00
    And mesh 0x00000010 has source vertex 0, 0, 0
    And mesh 0x00000010 has source vertex 70, 0, 0
    And mesh 0x00000010 has source vertex 0, 70, 0
    And mesh 0x00000010 has triangle 0,1,2 with edges -1,-1,-1
    And mesh 0x00000010 triangle 0 has flags 0x00000040
    When the nav graph is built for cell 0x00000C00
    Then mesh 0x00000010 polygon 0 is marked preferred-pathing

  Scenario: A triangle without the preferred-pathing bit is not marked
    Given a nav graph mesh 0x00000010 for cell 0x00000C00
    And mesh 0x00000010 has source vertex 0, 0, 0
    And mesh 0x00000010 has source vertex 70, 0, 0
    And mesh 0x00000010 has source vertex 0, 70, 0
    And mesh 0x00000010 has triangle 0,1,2 with edges -1,-1,-1
    When the nav graph is built for cell 0x00000C00
    Then mesh 0x00000010 polygon 0 is not marked preferred-pathing

  Scenario: NVTR per-edge external flags reach the prepared polygon
    Given a nav graph mesh 0x00000010 for cell 0x00000C00
    And mesh 0x00000010 has source vertex 0, 0, 0
    And mesh 0x00000010 has source vertex 70, 0, 0
    And mesh 0x00000010 has source vertex 0, 70, 0
    And mesh 0x00000010 has triangle 0,1,2 with edges -1,-1,-1
    And mesh 0x00000010 triangle 0 has flags 0x00000001
    When the nav graph is built for cell 0x00000C00
    Then mesh 0x00000010 polygon 0 edge 0 is authored-external
    And mesh 0x00000010 polygon 0 edge 1 is not authored-external
    And mesh 0x00000010 polygon 0 edge 2 is not authored-external

  Scenario: A merge backed by NVTR authored evidence on both sides is marked authored
    Given a nav graph mesh 0x00000010 for cell 0x00000C00
    And mesh 0x00000010 has source vertex 0, 0, 0
    And mesh 0x00000010 has source vertex 70, 0, 0
    And mesh 0x00000010 has source vertex 70, 210, 0
    And mesh 0x00000010 has source vertex 0, 210, 0
    And mesh 0x00000010 has triangle 0,1,2 with edges -1,-1,1
    And mesh 0x00000010 has triangle 0,2,3 with edges 0,-1,-1
    And mesh 0x00000010 triangle 0 has flags 0x00000001
    And a nav graph mesh 0x00000020 for cell 0x00000C00
    And mesh 0x00000020 has source vertex 70, -35, 0
    And mesh 0x00000020 has source vertex 0, -35, 0
    And mesh 0x00000020 has source vertex 0, -245, 0
    And mesh 0x00000020 has source vertex 70, -245, 0
    And mesh 0x00000020 has triangle 0,1,2 with edges -1,-1,1
    And mesh 0x00000020 has triangle 0,2,3 with edges 0,-1,-1
    And mesh 0x00000020 triangle 0 has flags 0x00000001
    When the nav graph is built for cell 0x00000C00
    Then the nav graph has 1 cross-mesh merge
    And cross-mesh merge 0 is authored
    And the nav graph counters report merges authored 1 geometric 0
    And the nav graph counters report merge candidates authored 1 geometric 0

  Scenario: A purely geometric merge is not marked authored
    Given a nav graph mesh 0x00000010 for cell 0x00000C00
    And mesh 0x00000010 has source vertex 0, 0, 0
    And mesh 0x00000010 has source vertex 70, 0, 0
    And mesh 0x00000010 has source vertex 70, 210, 0
    And mesh 0x00000010 has source vertex 0, 210, 0
    And mesh 0x00000010 has triangle 0,1,2 with edges -1,-1,1
    And mesh 0x00000010 has triangle 0,2,3 with edges 0,-1,-1
    And a nav graph mesh 0x00000020 for cell 0x00000C00
    And mesh 0x00000020 has source vertex 70, -35, 0
    And mesh 0x00000020 has source vertex 0, -35, 0
    And mesh 0x00000020 has source vertex 0, -245, 0
    And mesh 0x00000020 has source vertex 70, -245, 0
    And mesh 0x00000020 has triangle 0,1,2 with edges -1,-1,1
    And mesh 0x00000020 has triangle 0,2,3 with edges 0,-1,-1
    When the nav graph is built for cell 0x00000C00
    Then the nav graph has 1 cross-mesh merge
    And cross-mesh merge 0 is geometric
    And the nav graph counters report merges authored 0 geometric 1

  Scenario: Authored evidence is prioritized over a longer purely geometric conflicting candidate
    # Same geometry as nav_portals.feature's "A conflicting shorter candidate
    # is rejected, keeping the longer accepted interval" -- but this time the
    # shorter (`loser`) candidate carries the authored NVTR external-edge
    # flag and the longer (`winner`) candidate does not, so the shorter,
    # authored candidate wins instead.
    Given a nav graph mesh 0x00000010 for cell 0x00000C00
    And mesh 0x00000010 has source vertex 0, 0, 0
    And mesh 0x00000010 has source vertex 140, 0, 0
    And mesh 0x00000010 has source vertex 140, 210, 0
    And mesh 0x00000010 has source vertex 0, 210, 0
    And mesh 0x00000010 has triangle 0,1,2 with edges -1,-1,1
    And mesh 0x00000010 has triangle 0,2,3 with edges 0,-1,-1
    And a nav graph mesh 0x00000020 for cell 0x00000C00
    And mesh 0x00000020 has source vertex 140, -35, 0
    And mesh 0x00000020 has source vertex 0, -35, 0
    And mesh 0x00000020 has source vertex 0, -245, 0
    And mesh 0x00000020 has source vertex 140, -245, 0
    And mesh 0x00000020 has source vertex 105, -35, 0
    And mesh 0x00000020 has source vertex 35, -35, 0
    And mesh 0x00000020 has source vertex 35, -245, 0
    And mesh 0x00000020 has source vertex 105, -245, 0
    And mesh 0x00000020 has triangle 0,1,2 with edges -1,-1,1
    And mesh 0x00000020 has triangle 0,2,3 with edges 0,-1,-1
    And mesh 0x00000020 has triangle 4,5,6 with edges -1,-1,3
    And mesh 0x00000020 has triangle 4,6,7 with edges 2,-1,-1
    And mesh 0x00000020 triangle 2 has flags 0x00000001
    When the nav graph is built for cell 0x00000C00
    Then the nav graph has 1 cross-mesh merge
    And cross-mesh merge 0 connects mesh 0x00000010 polygon 0 to mesh 0x00000020 polygon 2
    And cross-mesh merge 0 is authored
    And the nav graph has an "warning" diagnostic containing "overlaps another accepted portal interval"

  Scenario: NVEX external connections are correlated against this cell's own NAVM set
    Given a nav graph mesh 0x00000010 for cell 0x00000C00
    And mesh 0x00000010 has source vertex 0, 0, 0
    And mesh 0x00000010 has source vertex 70, 0, 0
    And mesh 0x00000010 has source vertex 0, 70, 0
    And mesh 0x00000010 has triangle 0,1,2 with edges -1,-1,-1
    And mesh 0x00000010 has an external connection to 0x00000010 at triangle 0
    And mesh 0x00000010 has an external connection to 0x00000999 at triangle 0
    When the nav graph is built for cell 0x00000C00
    Then the nav graph counters report nvex outside-cell 1 inside-cell 1
    And the nav graph has an "info" diagnostic containing "NVEX correlation"

  Scenario: NVCI correlation entries are cross-referenced against this cell's doors and NAVMs
    Given a nav graph mesh 0x00000010 for cell 0x00000C00
    And mesh 0x00000010 has source vertex 0, 0, 0
    And mesh 0x00000010 has source vertex 70, 0, 0
    And mesh 0x00000010 has source vertex 0, 70, 0
    And mesh 0x00000010 has triangle 0,1,2 with edges -1,-1,-1
    And mesh 0x00000010 has a door 0x00000099 at triangle 0
    And a nav graph NAVI correlation with leading NAVM 0x00000010
    And that NAVI correlation has an entry linking NAVM 0x00000010 and NAVM 0x00000999 via door 0x00000099
    When the nav graph is built for cell 0x00000C00
    Then the nav graph counters report nvci subrecords 1 entries 1 door-matches 1 navmesh-matches 2
    And the nav graph has an "info" diagnostic containing "NVCI correlation"

  Scenario: No NVEX/NVCI data produces no correlation diagnostics
    Given a nav graph mesh 0x00000010 for cell 0x00000C00
    And mesh 0x00000010 has source vertex 0, 0, 0
    And mesh 0x00000010 has source vertex 70, 0, 0
    And mesh 0x00000010 has source vertex 0, 70, 0
    And mesh 0x00000010 has triangle 0,1,2 with edges -1,-1,-1
    When the nav graph is built for cell 0x00000C00
    Then the nav graph counters report nvex outside-cell 0 inside-cell 0
    And the nav graph counters report nvci subrecords 0 entries 0 door-matches 0 navmesh-matches 0

  Scenario: A door triangle that is also preferred-pathing still validates as a landmass mesh
    # Structural pin (issue #156 feature 1's coexistence rule with issue
    # #155's door typing: a door-typed triangle's landmass polygon type
    # index always wins over preferred-pathing when a triangle is both --
    # the exhaustive precedence/mapping coverage lives in
    # `landmass_graph.rs`'s own `resolve_polygon_type_index`/
    # `preferred_pathing_type_index` unit tests, the same split #155's own
    # `door_type_indices` mapping already established for this suite).
    Given a landmass mesh 0x00000010
    And landmass mesh 0x00000010 has vertex 0 at 0, 0, 0
    And landmass mesh 0x00000010 has vertex 1 at 1, 0, 0
    And landmass mesh 0x00000010 has vertex 2 at 0, 0, 1
    And landmass mesh 0x00000010 has polygon 0 with vertices 0,1,2
    And landmass mesh 0x00000010 polygon 0 is preferred-pathing
    And landmass mesh 0x00000010 has a door 0x00000099 at polygon 0
    When the mesh is converted to a landmass navigation mesh
    Then the landmass conversion produces a navigation mesh
