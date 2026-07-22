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
