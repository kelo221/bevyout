Feature: Prepared interior reflection-probe distribution
  Interior cells receive deterministic, bounded reflection captures during prepare.

  Scenario: A small room receives one central probe
    Given reflection-probe region areas "64"
    When probes are allocated with spacing 12 and cap 16
    Then the reflection-probe counts are "1"

  Scenario: A large open region receives denser coverage
    Given reflection-probe region areas "64,576"
    When probes are allocated with spacing 12 and cap 16
    Then the reflection-probe counts are "1,4"

  Scenario: The cell-wide cap prioritizes larger regions
    Given reflection-probe region areas "16,64,36"
    When probes are allocated with spacing 12 and cap 2
    Then the reflection-probe counts are "0,1,1"
