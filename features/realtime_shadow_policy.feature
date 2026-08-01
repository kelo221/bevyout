Feature: Realtime shadow write gating
  Realtime point shadows are an explicit runtime opt-in that is disabled by
  default. While disabled, the viewer must leave candidate lights and the
  selection record untouched on steady frames: only a settings toggle or a
  lingering selection pays one cleanup pass.

  Scenario: A steady disabled frame performs no writes
    Given the realtime shadow setting was not changed this frame
    And no realtime shadow light is selected
    When the disabled realtime-shadow write gate runs
    Then the disabled realtime-shadow pass is skipped

  Scenario: A realtime shadow settings toggle pays one cleanup pass
    Given the realtime shadow setting changed this frame
    And no realtime shadow light is selected
    When the disabled realtime-shadow write gate runs
    Then the disabled realtime-shadow pass clears candidate state

  Scenario: A lingering realtime shadow selection must be cleared
    Given the realtime shadow setting was not changed this frame
    And a realtime shadow light is selected
    When the disabled realtime-shadow write gate runs
    Then the disabled realtime-shadow pass clears candidate state
