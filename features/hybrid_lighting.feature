Feature: Observable hybrid lighting
  Static geometry is baked once while moving geometry stays in the realtime
  shadow pass. Receivers combine both visibility sources in one scene.

  Scenario: Prepared and realtime occlusion are both preserved
    Given prepared point-shadow visibility is 0.35
    And realtime point-shadow visibility is 0.60
    When hybrid point-shadow visibility is combined
    Then combined point-shadow visibility is 0.35

  Scenario: A moving caster can darken a prepared receiver
    Given prepared point-shadow visibility is 0.80
    And realtime point-shadow visibility is 0.20
    When hybrid point-shadow visibility is combined
    Then combined point-shadow visibility is 0.20

  Scenario: The demo caster moves while keeping a stable height
    Given a demo shadow caster orbits at height 1.0 with radius 3.0 and speed 1.0
    When the demo caster is sampled at 0.0 and 1.5707964 seconds
    Then the demo caster positions differ
    And both demo caster samples have height 1.0
