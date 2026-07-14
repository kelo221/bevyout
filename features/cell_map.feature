Feature: Cell map catalogue artifact
  # Pins issue #45's CellMap artifact: sorted-by-FormID worldspace
  # membership, exterior grid coordinates, and content-set-wide door
  # connectivity, exposed as deterministic RON via `cells --map`. Drives
  # src/vsa/cell_map.rs directly (the pure CellMap type and its builder),
  # which is deliberately dependency-free (serde + std only) so this suite
  # can include it verbatim without pulling in the ~2200-line attributed
  # ESM4 reader under src/vsa/openmw_esm4/. That parser's own XCLC parsing,
  # WRLD-group worldspace attribution, and door-graph resolution are
  # unit-tested with synthetic plugin bytes in
  # src/vsa/openmw_esm4/tests/mod.rs instead (T45.1-T45.3); this feature
  # file pins the resulting artifact's shape and determinism (T45.4-T45.5).

  Scenario: An interior cell has no worldspace, an exterior cell has one
    Given a cell map with an interior cell "VaultInterior" 0x00000100 and no grid
    And the cell map has an exterior cell "Wasteland01" 0x00000200 with grid -2, 5 in worldspace 0x00000010
    When the cell map is built
    Then cell 0x00000100 has no worldspace
    And cell 0x00000200 is in worldspace 0x00000010 with grid -2, 5

  Scenario: Opposing teleports produce two directed door edges
    Given a door edge from cell 0x00000100 door 0x00000101 to cell 0x00000200 door 0x00000201
    And a door edge from cell 0x00000200 door 0x00000201 to cell 0x00000100 door 0x00000101
    When the cell map is built
    Then there are 2 door edges
    And there are 0 unresolved doors

  Scenario: A dangling teleport is counted as unresolved, not fatal
    Given a door edge from cell 0x00000100 door 0x00000101 to cell 0x00000200 door 0x00000201
    And 1 unresolved door teleport
    When the cell map is built
    Then there is 1 door edge
    And there is 1 unresolved door

  Scenario: The RON output is byte-identical across two builds of the same input
    Given a cell map with an interior cell "VaultInterior" 0x00000100 and no grid
    And the cell map has an exterior cell "Wasteland01" 0x00000200 with grid -2, 5 in worldspace 0x00000010
    And a door edge from cell 0x00000100 door 0x00000101 to cell 0x00000200 door 0x00000201
    When the cell map is built twice from the same input
    Then both RON outputs are byte-identical
