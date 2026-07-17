Feature: NAVM/NAVI decode and prepared nav graph
  # Issue #111 (M4 wave 2). Two seams under test, both with synthetic
  # in-memory fixtures (no game data, no committed binaries):
  #
  # - Byte-level NAVM/NAVI subrecord decode drives the real attributed ESM4
  #   reader (vsa::openmw_esm4) with plugin byte streams assembled in step
  #   code, verifying the fopdoc FalloutNV NAVM/NAVI layouts.
  # - Graph construction drives the pure vsa::prepare::nav_graph module
  #   (std/serde only) with plain inputs, verifying coordinate conversion,
  #   adjacency, door/external links, validation severities, and
  #   deterministic ordering.

  Scenario: A fully-populated NAVM record decodes every subrecord
    Given a plugin cell 0x00000C00
    And a NAVM 0x00000500 in the cell with version 12
    And the NAVM declares counts vertices 3 triangles 1 external 1 cover 2 doors 1
    And the NAVM has vertices "70,140,-210;0,0,0;35,0,0"
    And the NAVM has triangle 0,1,2 with edges -1,-1,-1 and flags 0x00000640
    And the NAVM has cover ids "0,0"
    And the NAVM has door 0x00000D00 at triangle 0
    And the NAVM has a grid with divisor 2
    And the NAVM has an external connection to 0x00000501 at triangle 0
    When the content set is parsed for cell 0x00000C00
    Then the parsed navmesh 0x00000500 has version 12 and owner cell 0x00000C00
    And the parsed navmesh 0x00000500 has 3 vertices and 1 triangles
    And parsed navmesh 0x00000500 triangle 0 has flags 0x00000640
    And the parsed navmesh 0x00000500 has cover ids "0,0"
    And the parsed navmesh 0x00000500 has door 0x00000D00 at triangle 0
    And the parsed navmesh 0x00000500 has grid divisor 2
    And the parsed navmesh 0x00000500 has an external connection to 0x00000501 at triangle 0
    And the content set has no NAVM diagnostics

  Scenario: DATA counts that disagree with the payload are diagnosed, not fatal
    Given a plugin cell 0x00000C00
    And a NAVM 0x00000500 in the cell with version 12
    And the NAVM declares counts vertices 5 triangles 0 external 0 cover 0 doors 0
    And the NAVM has vertices "70,140,-210;0,0,0;35,0,0"
    When the content set is parsed for cell 0x00000C00
    Then the parsed navmesh 0x00000500 has 3 vertices and 0 triangles
    And the content set diagnostics include "NAVM 00000500: NVVX count mismatch: DATA declares 5, decoded 3"

  Scenario: A truncated NVTR payload degrades with a diagnostic instead of a panic
    Given a plugin cell 0x00000C00
    And a NAVM 0x00000500 in the cell with version 12
    And the NAVM has triangle 0,1,2 with edges -1,-1,-1 and flags 0x00000000
    And the NAVM NVTR payload is truncated by 3 bytes
    When the content set is parsed for cell 0x00000C00
    Then the parsed navmesh 0x00000500 has 0 vertices and 0 triangles
    And the content set diagnostics include "NAVM 00000500: NVTR truncated: 13 trailing byte(s) do not form a full 16-byte triangle"

  Scenario: The NAVI singleton is captured with its NVMI entries
    Given a plugin cell 0x00000C00
    And a NAVM 0x00000500 in the cell with version 12
    And the plugin has a NAVI 0x00000E00 entry linking NAVM 0x00000500 to location 0x00000C00 grid 3,-4
    When the content set is parsed for cell 0x00000C00
    Then the navigation singleton links NAVM 0x00000500 to location 0x00000C00 grid 3,-4

  Scenario: A later plugin's NAVI record overrides an earlier one
    Given a plugin cell 0x00000C00
    And the plugin has a NAVI 0x00000E00 entry linking NAVM 0x00000500 to location 0x00000C00 grid 1,1
    And a second plugin has a NAVI 0x00000E00 entry linking NAVM 0x00000501 to location 0x00000C00 grid 2,2
    When the content set is parsed for cell 0x00000C00
    Then the navigation singleton links NAVM 0x00000501 to location 0x00000C00 grid 2,2

  Scenario: A deleted NAVI override clears the navigation singleton
    Given a plugin cell 0x00000C00
    And the plugin has a NAVI 0x00000E00 entry linking NAVM 0x00000500 to location 0x00000C00 grid 1,1
    And a second plugin deletes the NAVI 0x00000E00 record
    When the content set is parsed for cell 0x00000C00
    Then there is no navigation singleton

  Scenario: A known source vertex converts to Bevy metres exactly once
    Given a nav graph mesh 0x00000010 for cell 0x00000C00
    And mesh 0x00000010 has source vertex 70, 140, -210
    When the nav graph is built for cell 0x00000C00
    Then mesh 0x00000010 vertex 0 is 1, -3, -2 in Bevy metres

  Scenario: Intra-mesh adjacency is built from NVTR edge fields
    Given a nav graph mesh 0x00000010 for cell 0x00000C00
    And mesh 0x00000010 has source vertex 0, 0, 0
    And mesh 0x00000010 has source vertex 70, 0, 0
    And mesh 0x00000010 has source vertex 0, 70, 0
    And mesh 0x00000010 has source vertex 70, 70, 0
    And mesh 0x00000010 has triangle 0,1,2 with edges -1,1,-1
    And mesh 0x00000010 has triangle 1,3,2 with edges -1,-1,0
    When the nav graph is built for cell 0x00000C00
    Then mesh 0x00000010 polygon 0 edge 1 neighbours polygon 1
    And mesh 0x00000010 polygon 1 edge 2 neighbours polygon 0
    And the nav graph has no diagnostics

  Scenario: Door and external links are carried onto the graph
    Given a nav graph mesh 0x00000010 for cell 0x00000C00
    And mesh 0x00000010 has source vertex 0, 0, 0
    And mesh 0x00000010 has source vertex 70, 0, 0
    And mesh 0x00000010 has source vertex 0, 70, 0
    And mesh 0x00000010 has triangle 0,1,2 with edges -1,-1,-1
    And mesh 0x00000010 has a door 0x00000D00 at triangle 0
    And mesh 0x00000010 has an external connection to 0x00000020 at triangle 0
    When the nav graph is built for cell 0x00000C00
    Then mesh 0x00000010 has door 0x00000D00 at polygon 0
    And mesh 0x00000010 has an external connection to 0x00000020 at polygon 0
    And the nav graph counters are meshes 1 polygons 1 vertices 3 doors 1 external 1

  Scenario: An external connection without a target NAVM is an error diagnostic
    Given a nav graph mesh 0x00000010 for cell 0x00000C00
    And mesh 0x00000010 has source vertex 0, 0, 0
    And mesh 0x00000010 has source vertex 70, 0, 0
    And mesh 0x00000010 has source vertex 0, 70, 0
    And mesh 0x00000010 has triangle 0,1,2 with edges -1,-1,-1
    And mesh 0x00000010 has an external connection with no target at triangle 0
    When the nav graph is built for cell 0x00000C00
    Then the nav graph has an "error" diagnostic containing "missing target NAVM FormID"

  Scenario: Out-of-range indices are errors and disconnected islands are warnings
    Given a nav graph mesh 0x00000010 for cell 0x00000C00
    And mesh 0x00000010 has source vertex 0, 0, 0
    And mesh 0x00000010 has source vertex 70, 0, 0
    And mesh 0x00000010 has source vertex 0, 70, 0
    And mesh 0x00000010 has triangle 0,1,9 with edges -1,-1,-1
    And mesh 0x00000010 has triangle 0,1,2 with edges -1,-1,-1
    When the nav graph is built for cell 0x00000C00
    Then the nav graph has an "error" diagnostic containing "vertex index 9 out of range"
    And the nav graph has a "warning" diagnostic containing "disconnected islands"

  Scenario: Meshes are ordered by FormID and serialize deterministically
    Given a nav graph mesh 0x00000020 for cell 0x00000C00
    And a nav graph mesh 0x00000010 for cell 0x00000C00
    When the nav graph is built for cell 0x00000C00
    Then the nav graph meshes are ordered "00000010,00000020"
    And serializing the nav graph twice yields identical RON

  Scenario: NAVI grid coordinates attach to the matching mesh
    Given a nav graph mesh 0x00000010 for cell 0x00000C00
    And a nav graph NAVI entry links mesh 0x00000010 to location 0x00000C00 grid 3,-4
    When the nav graph is built for cell 0x00000C00
    Then mesh 0x00000010 grid is 3,-4
    And the nav graph has no diagnostics

  Scenario: A NAVI location that disagrees with the NAVM owner cell is a warning
    Given a nav graph mesh 0x00000010 for cell 0x00000C00
    And a nav graph NAVI entry links mesh 0x00000010 to location 0x00000099 grid 3,-4
    When the nav graph is built for cell 0x00000C00
    Then the nav graph has a "warning" diagnostic containing "does not match this NAVM's owning cell"
