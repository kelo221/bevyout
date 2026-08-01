Feature: Weapon condition, degradation, jams, and deterministic combat RNG
  Wave 3 keeps condition and jam state on canonical item instances while
  accepted combat intents consume deterministic draws.

  Scenario: An accepted shot degrades one canonical weapon instance
    Given a canonical combat weapon item 1 form 0000434f with condition 100 of 100 and 1 loaded round
    And combat RNG seed 42
    When combat fire transaction 1 is committed
    Then combat fire outcome is fired
    And combat condition is 99
    And combat damage is 9
    And combat draw index is 1
    When combat fire transaction 1 is repeated
    Then combat draw index is 1

  Scenario: A jammed fire intent is rejected without a draw
    Given a canonical combat weapon item 1 form 0000434f with condition 50 of 100 and 1 loaded round
    And the canonical combat weapon is jammed for fire
    And combat RNG seed 7
    When combat fire transaction 2 is attempted
    Then combat action is rejected as jammed
    And combat draw index is 0
    And the combat snapshot is unchanged

  Scenario: A reload jam clears without changing the weapon instance
    Given a canonical combat weapon item 1 form 0000434f with condition 0 of 100 and 0 loaded rounds
    And the canonical combat weapon has 12 reserve rounds
    When a deterministic reload jam transaction 3 is committed
    Then combat reload outcome is jammed
    And combat weapon 1 is jammed for reload
    When combat clear-jam transaction 4 is committed
    Then combat clear-jam outcome is cleared
    And combat weapon 1 is not jammed
