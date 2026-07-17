Feature: Source-compatible DynamicLighting core

  Scenario: Dynamic light defaults match the frozen upstream package
    Given the default dynamic light configuration
    Then its intensity is 2 radius is 4 and bounce multiplier is 1

  Scenario: Effect and spatial discriminants match upstream
    Given the imported DynamicLighting catalogs
    Then all DynamicLighting enum discriminants match upstream

  Scenario: Strobe uses the original 30 Hz toggle
    Given a source-compatible strobe light
    When the dynamic light advances two frames at 60 Hz
    Then the dynamic light intensity multiplier is 0.25

  Scenario: Every imported intensity effect advances through the source runtime
    Given every imported dynamic light effect
    When every effect advances two frames at 60 Hz
    Then every dynamic light effect returns a finite multiplier
