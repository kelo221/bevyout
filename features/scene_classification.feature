Feature: Scene classification tracking
  Mesh classification (issue #270, PERF wave 1) tracks glow-card hiding and
  AO eligibility from entity and asset events instead of counting the full
  mesh query every frame. Count-based sentinels missed remove+add pairs
  that left entity/asset totals unchanged; the event-driven tracker must
  not.

  Scenario: Glow-card names match case-insensitively without allocation
    When the glow card naming policy is asked about "LightGlow01:0.001"
    Then the glow card naming policy reports a glow card
    When the glow card naming policy is asked about "lightglow01"
    Then the glow card naming policy reports a glow card
    When the glow card naming policy is asked about "ShackHangingLight02:51"
    Then the glow card naming policy does not report a glow card

  Scenario: A discovered eligible mesh waits for exactly one processing pass
    Given a fresh AO eligibility tracker
    When mesh entity 1 is discovered with mesh asset 10 as eligible
    Then mesh asset 10 is pending for AO processing
    When AO processing resolves mesh asset 10
    Then the AO tracker has no pending meshes
    And mesh asset 10 is tracked as eligible

  Scenario: A discovered ineligible mesh is never tracked
    Given a fresh AO eligibility tracker
    When mesh entity 1 is discovered with mesh asset 10 as ineligible
    Then mesh asset 10 is not tracked as eligible
    And the AO tracker has no pending meshes

  Scenario: A remove and add pair with equal totals is still processed
    Given a fresh AO eligibility tracker
    When mesh entity 1 is discovered with mesh asset 10 as eligible
    And AO processing resolves mesh asset 10
    And mesh entity 1 releases its mesh
    And mesh entity 2 is discovered with mesh asset 11 as eligible
    Then mesh asset 10 is not tracked as eligible
    And mesh asset 11 is pending for AO processing

  Scenario: Shared meshes stay eligible until the last referrer leaves
    Given a fresh AO eligibility tracker
    When mesh entity 1 is discovered with mesh asset 10 as eligible
    And mesh entity 2 is discovered with mesh asset 10 as eligible
    And mesh entity 1 releases its mesh
    Then mesh asset 10 is tracked as eligible
    When mesh entity 2 releases its mesh
    Then mesh asset 10 is not tracked as eligible

  Scenario: A mesh handle swap retires the old mesh
    Given a fresh AO eligibility tracker
    When mesh entity 1 is discovered with mesh asset 10 as eligible
    And AO processing resolves mesh asset 10
    And mesh entity 1 is discovered with mesh asset 11 as eligible
    Then mesh asset 10 is not tracked as eligible
    And mesh asset 11 is pending for AO processing

  Scenario: A reloaded mesh is requeued only while still referenced
    Given a fresh AO eligibility tracker
    When mesh asset 10 is added to the asset store
    Then the AO tracker has no pending meshes
    When mesh entity 1 is discovered with mesh asset 10 as eligible
    And AO processing resolves mesh asset 10
    And mesh asset 10 is added to the asset store
    Then mesh asset 10 is pending for AO processing
    When AO processing resolves mesh asset 10
    And mesh entity 1 releases its mesh
    And mesh asset 10 is added to the asset store
    Then the AO tracker has no pending meshes
