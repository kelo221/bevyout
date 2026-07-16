Feature: Camera-relative dropped item placement

  Scenario: A clear camera path keeps the one metre drop distance
    Given no drop candidates are blocked
    When the drop placement candidates are evaluated
    Then the drop placement mode is Camera
    And the selected drop distance is 1.0 metres

  Scenario: An obstruction retreats in ten centimetre steps
    Given the first 3 drop candidates are blocked
    When the drop placement candidates are evaluated
    Then the drop placement mode is Retreat
    And the selected drop distance is 0.7 metres

  Scenario: Fully blocked placement uses the player fallback
    Given every drop candidate is blocked
    When the drop placement candidates are evaluated
    Then the drop placement mode is PlayerFallback
    And the drop distance is the player fallback
