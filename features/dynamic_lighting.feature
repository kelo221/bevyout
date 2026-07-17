Feature: Isolated DynamicLighting core

  Scenario: Dynamic lights enable one bounce by default
    Given the default dynamic light state
    Then the dynamic light bounce multiplier is 1

  Scenario: Strobe intensity is deterministic
    Given a strobe effect at 2 Hz
    When the strobe is sampled at 0.26 seconds
    Then the dynamic light intensity multiplier is 0.25

  Scenario: Every imported intensity effect evaluates
    Given every imported dynamic light effect
    When the effects are sampled at 0.5 seconds
    Then every dynamic light effect returns a finite multiplier
