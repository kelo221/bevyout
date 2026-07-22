Feature: Deterministic target perception and awareness (issue #116)
  One authoritative awareness state per observer folds per-tick geometry,
  line of sight, and life state into acquire/loss decisions with hysteresis.

  Scenario: A visible target in the view cone is acquired after sustained sight
    Given a perception observer
    And a player target 5.0 metres ahead in clear view
    When perception advances for 0.4 seconds
    Then the observer has not acquired a target
    When perception advances for 0.4 seconds
    Then the observer has acquired the player

  Scenario: An occluded target is not acquired
    Given a perception observer
    And a player target 5.0 metres ahead but occluded
    When perception advances for 0.4 seconds
    And perception advances for 0.4 seconds
    Then the observer has not acquired a target

  Scenario: A target behind the observer is out of the view cone
    Given a perception observer
    And a player target 5.0 metres behind in clear view
    When perception advances for 0.4 seconds
    And perception advances for 0.4 seconds
    Then the observer has not acquired a target

  Scenario: A briefly occluded acquired target is retained
    Given a perception observer that has acquired the player
    And a player target 5.0 metres ahead but occluded
    When perception advances for 0.5 seconds
    Then the observer has acquired the player

  Scenario: An acquired target that disappears is lost
    Given a perception observer that has acquired the player
    And the target has disappeared
    When perception advances for 0.1 seconds
    Then the observer has lost its target

  Scenario: Awareness state survives a save and reload
    Given a perception observer that has acquired the player
    When the awareness state is serialized and reloaded
    Then the observer has acquired the player
