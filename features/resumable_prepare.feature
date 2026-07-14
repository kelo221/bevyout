Feature: Resumable prepare job manifest
  # Pins the pure job-manifest seam issue #48 adds (src/vsa/prepare/jobs.rs):
  # persisted pending/done/failed status per cell, fingerprint-scoped resume
  # filtering, and retry-failed selection. No game data, no plugin chain, no
  # Blender.

  Scenario: A cell marked done persists across a reload with the same fingerprint
    Given a fresh job manifest with fingerprint "abc123"
    And cell 0x00000001 is marked done
    And the manifest is written and reloaded
    Then cell 0x00000001 has status done

  Scenario: A cell marked failed persists its reason across a reload
    Given a fresh job manifest with fingerprint "abc123"
    And cell 0x00000002 is marked failed with reason "missing model foo.nif"
    And the manifest is written and reloaded
    Then cell 0x00000002 has status failed with reason "missing model foo.nif"

  Scenario: Resume filtering skips done cells and keeps pending and failed cells
    Given a fresh job manifest with fingerprint "abc123"
    And cell 0x00000001 is marked done
    And cell 0x00000002 is marked failed with reason "boom"
    And cell 0x00000003 is marked pending
    When cells "00000001,00000002,00000003" are resumed without force
    Then the cells to run are "00000002,00000003"
    And 1 cell(s) were skipped

  Scenario: --force reruns every selected cell regardless of status
    Given a fresh job manifest with fingerprint "abc123"
    And cell 0x00000001 is marked done
    When cells "00000001" are resumed with force
    Then the cells to run are "00000001"
    And 0 cell(s) were skipped

  Scenario: Retry-failed selects exactly the failed cells in the manifest
    Given a fresh job manifest with fingerprint "abc123"
    And cell 0x00000001 is marked done
    And cell 0x00000002 is marked failed with reason "boom"
    And cell 0x00000003 is marked failed with reason "bang"
    Then the failed cells are "00000002,00000003"

  Scenario: A manifest with a different content fingerprint is discarded
    Given a fresh job manifest with fingerprint "abc123"
    And cell 0x00000001 is marked done
    And the manifest is written to disk
    When the manifest is reloaded with fingerprint "def456"
    Then the manifest has no recorded cells
