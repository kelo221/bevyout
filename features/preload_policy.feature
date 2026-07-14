Feature: Predictive door-graph neighbor preload policy
  # Pins issue #51's pure preload policy: the desired resident set is the
  # active cell plus its 1-hop door-graph neighbors that are already
  # prepared (never an unprepared cell -- epic #5 runtime policy), and
  # eviction under budget removes the farthest resident cell by graph
  # distance first, never the active cell. Drives
  # src/viewer/world/policy.rs directly (the pure CellGraph/plan type),
  # which is deliberately dependency-free (std only) so this suite can
  # include it verbatim, the same way features/cell_map.feature drives
  # src/vsa/cell_map.rs.

  Scenario: An unprepared neighbor is never planned for load
    Given a door edge from cell 0x00000100 to cell 0x00000200
    And a door edge from cell 0x00000100 to cell 0x00000300
    And cell 0x00000200 is prepared
    And the active cell is 0x00000100
    And the resident cell limit is 4
    When the preload plan is computed
    Then the plan loads cell 0x00000200
    And the plan does not load cell 0x00000300
    And the plan does not load cell 0x00000100

  Scenario: Over budget evicts the farthest resident cell, never the active cell
    Given a door edge from cell 0x00000100 to cell 0x00000200
    And a door edge from cell 0x00000200 to cell 0x00000300
    And the active cell is 0x00000100
    And cell 0x00000100 is resident
    And cell 0x00000200 is resident
    And cell 0x00000300 is resident
    And the resident cell limit is 2
    When the preload plan is computed
    Then the plan evicts cell 0x00000300
    And the plan does not evict cell 0x00000100

  Scenario: Replanning once everything is already resident is a no-op
    Given a door edge from cell 0x00000100 to cell 0x00000200
    And cell 0x00000200 is prepared
    And the active cell is 0x00000100
    And cell 0x00000100 is resident
    And cell 0x00000200 is resident
    And the resident cell limit is 4
    When the preload plan is computed
    Then the plan loads nothing
    And the plan evicts nothing
