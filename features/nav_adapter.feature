Feature: Fallout nav adapter: NVMI merges, travel-door links, repath
  # Issue #113 (M4 wave 4). Four pure seams under test, all synthetic
  # in-memory fixtures (no game data, no committed binaries):
  #
  # - `NAVI.NVMI`'s trailing island block (center point, bounds, local
  #   vertex/triangle sub-mesh) drives the real attributed ESM4 reader
  #   with plugin byte streams assembled in step code; truncated tails are
  #   diagnosed, never panicked on. Note: the layout is the real-data
  #   verified one -- FO3's NVMI carries no merged-navmesh/preferred-merge/
  #   linked-door FormID arrays (the OpenMW TES4/TES5 hypothesis the plan
  #   named was disproven against real bytes; see openmw_esm4/NOTICE.md).
  # - `vsa::prepare::nav_graph`'s cross-mesh merge derivation: unconnected
  #   boundary edges of two same-cell meshes within the seam distance
  #   become deterministic merge connections.
  # - `viewer::nav::repath`'s decision table and pure blocked-door
  #   availability rule.
  # - `viewer::nav::door_link`'s travel-door lifecycle: a traversal whose
  #   destination is another cell terminates in the travel-reached state
  #   (issue #134 consumes that seam).
  # - `viewer::nav::landmass_graph`'s single-sided door extraction and
  #   merge-link descriptor resolution.

  Scenario: An NVMI island tail decodes into center, bounds, and sub-mesh
    Given a plugin cell 0x00000C00
    And a NAVM 0x00000500 in the cell with version 12
    And the plugin has a NAVI 0x00000E00 entry linking NAVM 0x00000500 to location 0x00000C00 grid 0,0
    And the NAVI entry has an island tail with center 1,2,3 bounds 0,0,0 to 9,9,9 and vertices "0,0,0;1,0,0;0,1,0" triangle 0,1,2
    When the content set is parsed for cell 0x00000C00
    Then the navigation entry for NAVM 0x00000500 has center 1,2,3
    And the navigation entry for NAVM 0x00000500 has bounds 0,0,0 to 9,9,9
    And the navigation entry for NAVM 0x00000500 has an island with 3 vertices and 1 triangles
    And the navigation entry for NAVM 0x00000500 retains no undecoded tail bytes

  Scenario: An NVMI tail with only a center point has no island
    Given a plugin cell 0x00000C00
    And a NAVM 0x00000500 in the cell with version 12
    And the plugin has a NAVI 0x00000E00 entry linking NAVM 0x00000500 to location 0x00000C00 grid 0,0
    And the NAVI entry has a bare tail with center 4,5,6
    When the content set is parsed for cell 0x00000C00
    Then the navigation entry for NAVM 0x00000500 has center 4,5,6
    And the navigation entry for NAVM 0x00000500 has no island

  Scenario: A truncated NVMI island tail is diagnosed and retained, never panicked on
    Given a plugin cell 0x00000C00
    And a NAVM 0x00000500 in the cell with version 12
    And the plugin has a NAVI 0x00000E00 entry linking NAVM 0x00000500 to location 0x00000C00 grid 0,0
    And the NAVI entry has a truncated island tail declaring 5 vertices
    When the content set is parsed for cell 0x00000C00
    Then the content set diagnostics include "Fallout3.esm NAVI 00000e00: NVMI 0: island truncated: declares 5 vertex(es)/0 triangle(s) needing 60 byte(s), only 12 available"
    And the navigation entry for NAVM 0x00000500 retains undecoded tail bytes

  Scenario: Two same-cell meshes with a near boundary seam gain a merge connection
    # Two-triangle quads (issue #154), not single right triangles: under
    # #154's full pairwise-candidate portal validation, a lone right
    # triangle's other two edges can themselves become spurious candidates
    # against the opposing mesh. A quad's three non-facing sides are each
    # perpendicular to the facing side, which cannot satisfy the
    # opposing-direction check regardless of distance -- see
    # `vsa::prepare::nav_graph`'s own `quad_mesh` unit-test helper for the
    # verified-by-hand geometry this mirrors.
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
    And building the nav graph again yields identical cross-mesh merges

  Scenario: Distant same-cell meshes gain no merge connection
    Given a nav graph mesh 0x00000010 for cell 0x00000C00
    And mesh 0x00000010 has source vertex 0, 0, 0
    And mesh 0x00000010 has source vertex 70, 0, 0
    And mesh 0x00000010 has source vertex 0, 70, 0
    And mesh 0x00000010 has triangle 0,1,2 with edges -1,-1,-1
    And a nav graph mesh 0x00000020 for cell 0x00000C00
    And mesh 0x00000020 has source vertex 70000, 0, 0
    And mesh 0x00000020 has source vertex 70070, 0, 0
    And mesh 0x00000020 has source vertex 70000, 70, 0
    And mesh 0x00000020 has triangle 0,1,2 with edges -1,-1,-1
    When the nav graph is built for cell 0x00000C00
    Then the nav graph has 0 cross-mesh merges

  Scenario Outline: The repath decision table is deterministic
    Given no repath observations
    And the repath observation <observation>
    When the repath decision is made
    Then the repath decision is <decision>

    Examples:
      | observation                  | decision   |
      | none                         | keep-route |
      | door-became-blocked          | repath     |
      | door-became-unblocked        | repath     |
      | target-moved                 | repath     |
      | agent-off-link               | repath     |
      | destination-cell-unloaded    | fail       |

  Scenario: Destination-cell-unloaded outranks every repath trigger
    Given no repath observations
    And the repath observation destination-cell-unloaded
    And the repath observation door-became-unblocked
    And the repath observation target-moved
    When the repath decision is made
    Then the repath decision is fail

  Scenario Outline: Blocked-door exclusion follows the pure availability rule
    Given a door that is <locked_state> and <open_state>
    Then the door is <usability> for route planning

    Examples:
      | locked_state | open_state | usability  |
      | locked       | closed     | not usable |
      | locked       | open       | usable     |
      | unlocked     | closed     | usable     |
      | unlocked     | open       | usable     |

  Scenario: A travel-door traversal terminates in the travel-reached state
    Given a fresh door-link state
    When the door-link reaches travel door 0x00000D00 to cell 0x00000C0D
    Then the door-link state is paused for door 0x00000D00
    When the door-link ticks with the door open
    Then the door-link state is traversing door 0x00000D00
    When the door-link traversal completes
    Then the door-link state is travel-reached for door 0x00000D00 to cell 0x00000C0D

  Scenario: A new link restarts the lifecycle after a travel door was reached
    Given a fresh door-link state
    When the door-link reaches travel door 0x00000D00 to cell 0x00000C0D
    And the door-link ticks with the door open
    And the door-link traversal completes
    And the door-link reaches door 0x00000D50
    Then the door-link state is paused for door 0x00000D50

  Scenario: A door with exactly one triangle is single-sided, two is not
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
    And landmass mesh 0x00000020 has a door 0x00000E00 at polygon 0
    And landmass mesh 0x00000010 has a door 0x00000E00 at polygon 0
    When the single-sided doors are extracted
    Then there is 1 single-sided door
    And single-sided door 0 is door 0x00000D00 on mesh 0x00000010

  Scenario: Merge links resolve both sides from the prepared connection
    Given a landmass mesh 0x00000010
    And landmass mesh 0x00000010 has vertex 0 at 0, 0, 0
    And landmass mesh 0x00000010 has vertex 1 at 1, 0, 0
    And landmass mesh 0x00000010 has vertex 2 at 0, 0, 1
    And landmass mesh 0x00000010 has polygon 0 with vertices 0,1,2
    And a landmass mesh 0x00000020
    And landmass mesh 0x00000020 has vertex 0 at 3, 0, 0
    And landmass mesh 0x00000020 has vertex 1 at 4, 0, 0
    And landmass mesh 0x00000020 has vertex 2 at 3, 0, 1
    And landmass mesh 0x00000020 has polygon 0 with vertices 0,1,2
    And a prepared merge connects mesh 0x00000010 triangle 0 to mesh 0x00000020 triangle 0
    And a prepared merge connects mesh 0x00000010 triangle 0 to mesh 0x00000099 triangle 0
    When the merge-link descriptors are resolved
    Then there is 1 merge-link descriptor
    And merge-link descriptor 0 links mesh 0x00000010 polygon 0 to mesh 0x00000020 polygon 0
