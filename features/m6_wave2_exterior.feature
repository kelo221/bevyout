Feature: M6 wave 2 exterior reversal and process diagnostics
  # Pins the generation-aware exterior planner and the runtime diagnostics
  # report. The real bridge route remains covered by the manual script; these
  # scenarios keep the two new decision/report seams executable in CI.

  Scenario: Re-entering an evicting cell cancels teardown
    Given the exterior index contains cell 0x00000c49 at grid (0,0)
    And the exterior index contains cell 0x00000c4a at grid (1,0)
    And the exterior stream is at grid (0,0) moving toward (1,0)
    And exterior cell 0x00000c4a at grid (1,0) is Evicting at generation 7
    When the exterior residency plan is computed
    Then the exterior plan cancels cell 0x00000c4a at generation 7

  Scenario: Process memory remains distinct from package estimates
    Given an exterior process-memory trace with samples "1024,4096,2048"
    And the exterior package estimates are resident 256 and peak 512
    When the exterior process-memory report is rendered
    Then the exterior report identifies process memory as "process_resident_set"
    And exterior process memory is resident 2048, peak 4096, ending 2048
    And exterior package estimates remain resident 256 and peak 512
