Feature: AI package families (#196 Travel/Patrol, #197 Idle/Eat/Sleep)
  The pure family dispatch turns an active package's type into concrete
  behaviour, emitting only navigation and animation requests -- never a
  transform write -- and reporting lifecycle transitions back.

  Scenario: Travel routes to its destination then reports arrival
    Given a travel family targeting (10, 0, 0) with tolerance 0.5
    When the actor is at (0, 0, 0) still en route
    Then the family requests a route to (10, 0, 0)
    When the actor arrives at (10, 0, 0)
    Then the family stops routing and completes the package

  Scenario: Patrol visits its markers in order
    Given a patrol family over markers (0,0,0) then (10,0,0) then (10,0,10)
    When the actor arrives at patrol marker 0
    Then the family advances to patrol marker 1
    When the actor arrives at patrol marker 1
    Then the family advances to patrol marker 2
    When the actor arrives at patrol marker 2
    Then the family advances to patrol marker 0

  Scenario: A short patrol leg does not advance while the actor is stationary
    Given a patrol family over markers (0,0,0) then (1.4575,0,0) then (10,0,0) with tolerance 1.5
    When the actor arrives at patrol marker 0
    Then the family advances to patrol marker 1
    When the actor remains at patrol marker 0 for 20 ticks
    Then the family remains at patrol marker 1

  Scenario: A patrol stops and idles at a marker before departing
    Given a patrol family with default marker dwell over markers (0,0,0) then (10,0,0) then (10,0,10)
    When the actor arrives at patrol marker 0
    Then the family stops at patrol marker 0 for its dwell
    When the patrol family ticks 2.0 seconds at patrol marker 0
    Then the family holds idle at patrol marker 0
    When the patrol family ticks 1.1 seconds at patrol marker 0
    Then the family advances to patrol marker 1

  Scenario: Idle routes to its location then plays the idle animation
    Given an idle family at (5, 0, 0) with tolerance 0.5
    When the actor is at (0, 0, 0) still en route
    Then the family requests a route to (5, 0, 0)
    When the actor arrives at (5, 0, 0)
    Then the family requests the idle animation

  Scenario: Eat occupies its interaction point and plays the eat state
    Given an eat family at interaction point 61453 located at (0, 0, 0)
    When the actor arrives at the interaction point
    Then the family occupies interaction point 61453
    And the family requests the eat animation

  Scenario: Sleep releases its interaction point on preempt
    Given a sleep family at interaction point 2989 located at (0, 0, 0)
    When the actor arrives at the interaction point
    Then the family occupies interaction point 2989
    When the sleep package is preempted
    Then the family releases interaction point 2989
