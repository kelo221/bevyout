Feature: ImageSpace transient screen feedback
  # The policy is deliberately driven with integer milliseconds so prepared
  # IMAD playback and runtime requests have deterministic tests.

  Scenario: IMAD curves sample between integer keyframes
    Given an IMAD blood curve from 0 ms at 0 to 100 ms at 1
    When the IMAD curve is sampled at 50 ms
    Then the sampled screen blood is 0.5

  Scenario: Overlapping modifiers use deterministic priority and form ordering
    Given a neutral screen effect base
    And a screen modifier 0x20 with priority 2 and blood 0.2
    And a screen modifier 0x10 with priority 1 and blood 0.4
    When the screen modifiers are composed
    Then 2 screen modifier is active
    And the composed screen blood is 0.6

  Scenario: Repeating a start replaces rather than duplicates
    Given a neutral screen effect base
    And a screen modifier 0x30 with priority 0 and blood 0.25
    When the same screen modifier is started again with blood 0.75
    Then 1 screen modifier is active
    And the composed screen blood is 0.75

  Scenario: Expiry restores the active base and clear is idempotent
    Given a neutral screen effect base
    And a screen modifier 0x40 with duration 100 ms and blood 1
    When screen time advances to 100 ms
    And screen effects are cleared for a cell transition
    Then no screen modifier is active
    And the composed screen blood is 0

  Scenario: Settings can disable screen blood and distortion
    Given a neutral screen effect base
    And screen blood and distortion are disabled
    And a weapon-hit screen modifier 0x50 with blood 1 and double vision 1
    When the screen modifiers are composed
    Then the composed screen blood is 0
    And the composed double vision is 0
