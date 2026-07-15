Feature: Resumable batch bake job selection
  # Pins the pure job-selection/skip seam issue #62 adds
  # (src/vsa/bake/plan.rs) on top of the reused #48 job-manifest machinery
  # (src/vsa/prepare/jobs.rs, shared unchanged with `prepare`): a recorded
  # bake is valid only when both its bake pipeline revision and its
  # job-parameter fingerprint match the current toolchain; resume skips
  # exactly the done-and-valid cells; retry-failed selects exactly the
  # failed set; the batch summary line is a stable wording contract.
  # Synthetic manifests only -- no game data, no Blender, no I/O.

  Scenario: A recorded bake matching the current revision and job fingerprint is valid
    Given cell 0x00000001 has a recorded bake with revision "bake-v1" and job fingerprint "job-abc"
    When cell 0x00000001's bake is checked against revision "bake-v1" and job fingerprint "job-abc"
    Then the recorded bake is valid

  Scenario: A bake pipeline revision change alone invalidates the bake
    Given cell 0x00000001 has a recorded bake with revision "bake-v1" and job fingerprint "job-abc"
    When cell 0x00000001's bake is checked against revision "bake-v2" and job fingerprint "job-abc"
    Then the recorded bake is stale

  Scenario: A job fingerprint change alone invalidates the bake
    Given cell 0x00000001 has a recorded bake with revision "bake-v1" and job fingerprint "job-abc"
    When cell 0x00000001's bake is checked against revision "bake-v1" and job fingerprint "job-xyz"
    Then the recorded bake is stale

  Scenario: A cell that was never baked has no valid bake
    Given cell 0x00000001 has no recorded bake
    When cell 0x00000001's bake is checked against revision "bake-v1" and job fingerprint "job-abc"
    Then the recorded bake is stale

  Scenario: Resume skips done-and-valid cells and keeps pending and failed cells
    Given a fresh job manifest with fingerprint "abc123"
    And cell 0x00000001 is marked done
    And cell 0x00000001's recorded bake is currently valid
    And cell 0x00000002 is marked pending
    And cell 0x00000003 is marked failed with reason "Blender bake failed"
    When cells "00000001,00000002,00000003,00000004" are bake-resumed without force
    Then the bake cells to run are "00000002,00000003,00000004"
    And 1 cell(s) were skipped as validly baked
    And no bake cells were stale

  Scenario: A done cell with a stale bake is requeued and reported
    Given a fresh job manifest with fingerprint "abc123"
    And cell 0x00000001 is marked done
    And cell 0x00000001's recorded bake is currently valid
    And cell 0x00000002 is marked done
    And cell 0x00000002's recorded bake is currently stale
    And cell 0x00000003 is marked pending
    When cells "00000001,00000002,00000003" are bake-resumed without force
    Then the bake cells to run are "00000002,00000003"
    And 1 cell(s) were skipped as validly baked
    And the stale bake cells are "00000002"

  Scenario: Force reruns every selected cell regardless of status or bake validity
    Given a fresh job manifest with fingerprint "abc123"
    And cell 0x00000001 is marked done
    And cell 0x00000001's recorded bake is currently valid
    And cell 0x00000002 is marked failed with reason "boom"
    When cells "00000001,00000002" are bake-resumed with force
    Then the bake cells to run are "00000001,00000002"
    And 0 cell(s) were skipped as validly baked
    And no bake cells were stale

  Scenario: Retry-failed selects exactly the failed cells in the bake job manifest
    Given a fresh job manifest with fingerprint "abc123"
    And cell 0x00000001 is marked done
    And cell 0x00000002 is marked failed with reason "Blender bake failed"
    And cell 0x00000003 is marked failed with reason "scene manifest contains no renderable placements"
    Then the failed cells are "00000002,00000003"

  Scenario: The bake batch summary line is a stable wording contract
    Then the bake batch summary line for 2 baked, 5 skipped, and 1 failed is "bake batch: 2 baked, 5 skipped (valid), 1 failed"
    And the stale bake line for cell 0x00012345 is "bake fingerprint: cell 00012345 stale"
