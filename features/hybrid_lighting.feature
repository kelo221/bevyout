Feature: Observable hybrid lighting
  Static geometry is baked once while moving geometry stays in the realtime
  shadow pass. Receivers combine both visibility sources in one scene.

  Scenario: A moving caster can darken a prepared receiver
    Given prepared point-shadow visibility is 0.80
    And realtime point-shadow visibility is 0.20
    When hybrid point-shadow visibility is combined
    Then combined point-shadow visibility is 0.20

  Scenario: Prepared visibility wins when it is darker
    Given prepared point-shadow visibility is 0.35
    And realtime point-shadow visibility is 0.60
    When hybrid point-shadow visibility is combined
    Then combined point-shadow visibility is 0.35

  Scenario: A missing source leaves the available visibility intact
    Given prepared point-shadow visibility is 0.35
    When hybrid point-shadow visibility is combined
    Then combined point-shadow visibility is 0.35
