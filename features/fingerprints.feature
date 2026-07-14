Feature: Fingerprint validation
  # Pins the pure fingerprint record/compare/invalidate seam issue #49 adds
  # (src/vsa/prepare/fingerprints.rs, src/vsa/prepare/jobs.rs): every
  # completed cell records four fingerprints (plugin content-set, converter,
  # physics pipeline, prepare pipeline); any single component changing alone
  # invalidates the cell; a mixed manifest re-queues exactly its stale
  # subset; a legacy entry with no recorded fingerprints counts as stale,
  # never a parse error. No game data, no plugin chain, no Blender.

  Scenario: A cell whose recorded fingerprints match the current toolchain is valid
    Given a fresh job manifest with fingerprint "abc123"
    And cell 0x00000001 is marked done
    And cell 0x00000001 has recorded fingerprints plugin "abc123" converter "conv-v1" physics "phys-v1" prepare_pipeline "prep-v1"
    When cell 0x00000001 is checked against current fingerprints plugin "abc123" converter "conv-v1" physics "phys-v1" prepare_pipeline "prep-v1"
    Then the cell is valid

  Scenario Outline: A changed component alone invalidates the cell
    Given a fresh job manifest with fingerprint "abc123"
    And cell 0x00000001 is marked done
    And cell 0x00000001 has recorded fingerprints plugin "abc123" converter "conv-v1" physics "phys-v1" prepare_pipeline "prep-v1"
    When cell 0x00000001 is checked against current fingerprints plugin "<plugin>" converter "<converter>" physics "<physics>" prepare_pipeline "<prepare_pipeline>"
    Then the cell is stale in component "<component>"

    Examples:
      | plugin | converter | physics | prepare_pipeline | component        |
      | def456 | conv-v1   | phys-v1 | prep-v1           | plugin            |
      | abc123 | conv-v2   | phys-v1 | prep-v1           | converter         |
      | abc123 | conv-v1   | phys-v2 | prep-v1           | physics           |
      | abc123 | conv-v1   | phys-v1 | prep-v2           | prepare_pipeline  |

  Scenario: A mixed manifest re-queues exactly the stale subset
    Given a fresh job manifest with fingerprint "abc123"
    And cell 0x00000001 is marked done
    And cell 0x00000001 has recorded fingerprints plugin "abc123" converter "conv-v1" physics "phys-v1" prepare_pipeline "prep-v1"
    And cell 0x00000002 is marked done
    And cell 0x00000002 has recorded fingerprints plugin "abc123" converter "conv-OLD" physics "phys-v1" prepare_pipeline "prep-v1"
    And cell 0x00000003 is marked pending
    When cells "00000001,00000002,00000003" are resumed without force against current fingerprints plugin "abc123" converter "conv-v1" physics "phys-v1" prepare_pipeline "prep-v1"
    Then the checked cells to run are "00000002,00000003"
    And 1 cell(s) were checked as skipped
    And 1 cell(s) were stale

  Scenario: A legacy entry without recorded fingerprints counts as stale, not an error
    Given a fresh job manifest with fingerprint "abc123"
    And cell 0x00000001 is marked done
    When cell 0x00000001 is checked against current fingerprints plugin "abc123" converter "conv-v1" physics "phys-v1" prepare_pipeline "prep-v1"
    Then the cell is stale in component "plugin"
    And the cell is stale in component "converter"
    And the cell is stale in component "physics"
    And the cell is stale in component "prepare_pipeline"
