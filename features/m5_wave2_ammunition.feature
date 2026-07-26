Feature: Canonical weapon ammunition
  Loaded rounds belong to one weapon instance and reload decisions are
  deterministic before any Bevy presentation or spatial query.

  Scenario: An operational reload consumes only missing rounds
    Given a magazine for ammo 00004241 with 7 of 12 rounds loaded
    And 20 compatible reserve rounds
    When a reload with ammo 00004241 is planned
    Then the reload kind is operational
    And the reload consumes 5 reserve rounds
    And the reload returns 0 loaded rounds

  Scenario: Switching ammunition is planned atomically
    Given a magazine for ammo 00004241 with 7 of 12 rounds loaded
    And 20 compatible reserve rounds
    When a reload with ammo 01000800 is planned
    Then the reload kind is ammo switch
    And the reload consumes 12 reserve rounds
    And the reload returns 7 loaded rounds

  Scenario: Dry fire preserves an empty magazine
    Given a magazine for ammo 00004241 with 0 of 12 rounds loaded
    When one loaded round is consumed
    Then fire is blocked because the magazine is empty
    And the magazine still contains 0 rounds

