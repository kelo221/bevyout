Feature: Batch session cache accounting
  # Pins the pure batch-cache-accounting seam issue #47 adds
  # (src/vsa/prepare/batch_cache.rs) -- the hit/miss counter behind the
  # session-level physics sidecar cache, the aggregated asset cache totals,
  # the deterministic end-of-batch summary line, and writing the CellMap
  # artifact. No game data, no plugin chain, no BSAs, no Blender.

  Scenario: A physics key shared by two cells in a batch is a hit on the second cell
    Given a fresh batch cache
    When cell "VaultA" reads physics key "assets/shared.physics.json.gz"
    And cell "VaultB" reads physics key "assets/shared.physics.json.gz"
    Then physics reads is 2
    And physics hits is 1

  Scenario: Distinct physics keys across cells never count as hits
    Given a fresh batch cache
    When cell "VaultA" reads physics key "assets/one.physics.json.gz"
    And cell "VaultB" reads physics key "assets/two.physics.json.gz"
    Then physics reads is 2
    And physics hits is 0

  Scenario: Per-cell asset cache counts aggregate into one batch total
    Given a fresh batch cache
    When cell "VaultA" reports asset cache reused 3, built 1, invalid 0, explicit 0
    And cell "VaultB" reports asset cache reused 2, built 0, invalid 1, explicit 1
    Then the batch cache summary line is "batch cache: assets reused 5, built 1, rebuilt 2, physics reads 0, physics hits 0"

  Scenario: The batch summary line combines asset totals and physics hit/miss counts
    Given a fresh batch cache
    When cell "VaultA" reports asset cache reused 1, built 1, invalid 0, explicit 0
    And cell "VaultA" reads physics key "assets/shared.physics.json.gz"
    And cell "VaultB" reads physics key "assets/shared.physics.json.gz"
    Then the batch cache summary line is "batch cache: assets reused 1, built 1, rebuilt 0, physics reads 2, physics hits 1"

  Scenario: A batch run writes the deterministic cell map under the cache dir root
    Given a cell map with an interior cell "VaultA" 0x00000001 and no grid
    And the cell map has an exterior cell "Wasteland01" 0x00000002 with grid -2, 5 in worldspace 0x00000010
    When the cell map is written to the batch cache dir
    Then the written cell map file exists at "cellmap.ron" under the batch cache dir
    And the written cell map has 2 cells
    And there are 0 door edges
