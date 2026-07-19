Feature: Cross-mesh portals - validated edges, interval-based links, runtime step-height
  # Issue #154 (M4 wave 8). A review of #113's cross-mesh merge derivation
  # found two real-data defects: the original one-directional
  # nearest-midpoint matching could manufacture repeated-destination
  # candidates (every edge of a facing triangle claiming the same opposing
  # edge), and link lengths could exceed the nominal seam gap with no
  # geometric validation at all. This feature drives the reworked
  # `vsa::prepare::nav_graph::compute_mesh_merges` pipeline directly
  # (reusing nav_graph.feature's "a nav graph mesh"/"has source vertex"/
  # "has triangle"/"the nav graph is built" steps -- see that feature and
  # nav_adapter.feature for the original merge-derivation seam this
  # reworks) plus `viewer::nav::landmass_graph`'s runtime interval-to-link
  # conversion (reusing nav_backend.feature/nav_adapter.feature's
  # "landmass mesh"/"a prepared merge connects"/"the merge-link
  # descriptors are resolved" steps).
  #
  # All meshes below are two-triangle rectangles ("quads"): a lone right
  # triangle's *other* two edges can themselves become spurious portal
  # candidates against an opposing mesh under #154's full pairwise-
  # candidate validation (an earlier revision of this feature's own
  # fixtures hit exactly that), where a quad's three non-facing sides are
  # each perpendicular to the facing side and so can never satisfy the
  # opposing-direction check regardless of distance.

  Scenario: A clean matching seam records the matched edges' vertex-index identity
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
    And cross-mesh merge 0 connects mesh 0x00000010 polygon 0 to mesh 0x00000020 polygon 0
    And cross-mesh merge 0 has edge_a 0,1 and edge_b 0,1

  Scenario: Close parallel walls do not portal
    Given a nav graph mesh 0x00000010 for cell 0x00000C00
    And mesh 0x00000010 has source vertex 0, 0, 0
    And mesh 0x00000010 has source vertex 70, 0, 0
    And mesh 0x00000010 has source vertex 70, 210, 0
    And mesh 0x00000010 has source vertex 0, 210, 0
    And mesh 0x00000010 has triangle 0,1,2 with edges -1,-1,1
    And mesh 0x00000010 has triangle 0,2,3 with edges 0,-1,-1
    And a nav graph mesh 0x00000020 for cell 0x00000C00
    And mesh 0x00000020 has source vertex 0, -35, 0
    And mesh 0x00000020 has source vertex 70, -35, 0
    And mesh 0x00000020 has source vertex 70, -245, 0
    And mesh 0x00000020 has source vertex 0, -245, 0
    And mesh 0x00000020 has triangle 0,1,2 with edges -1,-1,1
    And mesh 0x00000020 has triangle 0,2,3 with edges 0,-1,-1
    When the nav graph is built for cell 0x00000C00
    Then the nav graph has 0 cross-mesh merges
    And the nav graph has an "warning" diagnostic containing "not opposing enough"

  Scenario: A vertically offset seam is still accepted prepare-side, with the drop recorded
    # Review correction: prepare-time validation must not bake an
    # agent-class assumption (step height) into the universal prepared
    # graph -- see the runtime scenario below for where that check now
    # lives.
    Given a nav graph mesh 0x00000010 for cell 0x00000C00
    And mesh 0x00000010 has source vertex 0, 0, 0
    And mesh 0x00000010 has source vertex 70, 0, 0
    And mesh 0x00000010 has source vertex 70, 210, 0
    And mesh 0x00000010 has source vertex 0, 210, 0
    And mesh 0x00000010 has triangle 0,1,2 with edges -1,-1,1
    And mesh 0x00000010 has triangle 0,2,3 with edges 0,-1,-1
    And a nav graph mesh 0x00000020 for cell 0x00000C00
    And mesh 0x00000020 has source vertex 70, -35, 70
    And mesh 0x00000020 has source vertex 0, -35, 70
    And mesh 0x00000020 has source vertex 0, -245, 70
    And mesh 0x00000020 has source vertex 70, -245, 70
    And mesh 0x00000020 has triangle 0,1,2 with edges -1,-1,1
    And mesh 0x00000020 has triangle 0,2,3 with edges 0,-1,-1
    When the nav graph is built for cell 0x00000C00
    Then the nav graph has 1 cross-mesh merge
    And cross-mesh merge 0 has a vertical drop of about 1.0 metres

  Scenario: A conflicting shorter candidate is rejected, keeping the longer accepted interval
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
    When the nav graph is built for cell 0x00000C00
    Then the nav graph has 1 cross-mesh merge
    And cross-mesh merge 0 connects mesh 0x00000010 polygon 0 to mesh 0x00000020 polygon 0
    And the nav graph has an "warning" diagnostic containing "overlaps another accepted portal interval"

  Scenario: One long edge legitimately matches two short collinear edges
    Given a nav graph mesh 0x00000010 for cell 0x00000C00
    And mesh 0x00000010 has source vertex 0, 0, 0
    And mesh 0x00000010 has source vertex 140, 0, 0
    And mesh 0x00000010 has source vertex 140, 210, 0
    And mesh 0x00000010 has source vertex 0, 210, 0
    And mesh 0x00000010 has triangle 0,1,2 with edges -1,-1,1
    And mesh 0x00000010 has triangle 0,2,3 with edges 0,-1,-1
    And a nav graph mesh 0x00000020 for cell 0x00000C00
    And mesh 0x00000020 has source vertex 140, -21, 0
    And mesh 0x00000020 has source vertex 70, -21, 0
    And mesh 0x00000020 has source vertex 70, -231, 0
    And mesh 0x00000020 has source vertex 140, -231, 0
    And mesh 0x00000020 has source vertex 70, -21, 0
    And mesh 0x00000020 has source vertex 0, -21, 0
    And mesh 0x00000020 has source vertex 0, -231, 0
    And mesh 0x00000020 has source vertex 70, -231, 0
    And mesh 0x00000020 has triangle 0,1,2 with edges -1,-1,1
    And mesh 0x00000020 has triangle 0,2,3 with edges 0,-1,-1
    And mesh 0x00000020 has triangle 4,5,6 with edges -1,-1,3
    And mesh 0x00000020 has triangle 4,6,7 with edges 2,-1,-1
    When the nav graph is built for cell 0x00000C00
    Then the nav graph has 2 cross-mesh merges
    And cross-mesh merge 0 connects mesh 0x00000010 polygon 0 to mesh 0x00000020 polygon 0
    And cross-mesh merge 1 connects mesh 0x00000010 polygon 0 to mesh 0x00000020 polygon 2

  Scenario: Runtime merge links use the portal-interval midpoint and a real distance cost
    Given a landmass mesh 0x00000010
    And a landmass mesh 0x00000020
    And a prepared merge connects mesh 0x00000010 triangle 0 to mesh 0x00000020 triangle 0 with interval 0, 0, 0 to 1, 0, 0 and interval 2, 0, 0 to 3, 0, 0
    When the merge-link descriptors are resolved
    Then there is 1 merge-link descriptor
    And merge-link descriptor 0 has a cost of about 2.0

  Scenario: A runtime-excessive vertical drop skips the merge link despite prepare-side acceptance
    Given a landmass mesh 0x00000010
    And a landmass mesh 0x00000020
    And a prepared merge connects mesh 0x00000010 triangle 0 to mesh 0x00000020 triangle 1 with interval 0, 0, 0 to 1, 0, 0 and interval 0, 1, 0 to 1, 1, 0
    When the merge-link descriptors are resolved
    Then there are 0 merge-link descriptors
