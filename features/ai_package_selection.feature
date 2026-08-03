Feature: AI package selection policy
  # `viewer::ai::selection` (issue #193) is pure: given an actor's prepared
  # package list already in authored priority order, a deterministic game
  # instant, and the M4 condition boundary, it selects the first eligible
  # package and reports why every higher-priority one was rejected. Package
  # types above the fopdoc maximum, out-of-window schedules, and false or
  # unevaluable conditions are all distinct, counted rejection reasons.
  # Conditions route through the boundary only -- an unsupported function is
  # "unevaluable" (deferred to #15), never guessed.

  Scenario: The first eligible package wins and higher-priority rejects are explained
    Given a selection game hour of 12.0
    And a package candidate 0x00000010 of type 200
    And a package candidate 0x00000020 of type 5
    And a package candidate 0x00000030 of type 5
    When the actor's package is selected
    Then the selected package is 0x00000020
    And package candidate 0x00000010 was rejected as "unsupported-type"
    And package candidate 0x00000030 was rejected as "lower-priority"
    And the selection counts unsupported_type 1 out_of_schedule 0 conditions_false 0 conditions_unevaluable 0 schedule_gap 0

  Scenario: A priority tie takes the earlier package
    Given a selection game hour of 12.0
    And a package candidate 0x000000AA of type 0
    And a package candidate 0x000000BB of type 0
    When the actor's package is selected
    Then the selected package is 0x000000AA

  Scenario: A schedule window excludes an out-of-window hour
    Given a selection game hour of 20.0
    And a package candidate 0x00000010 of type 0
    And candidate 0x00000010 has schedule time 8 duration 1
    When the actor's package is selected
    Then no package is selected
    And the selection counts unsupported_type 0 out_of_schedule 1 conditions_false 0 conditions_unevaluable 0 schedule_gap 1

  Scenario: A schedule includes its inclusive start hour
    Given a selection game hour of 8.0
    And a package candidate 0x00000010 of type 0
    And candidate 0x00000010 has schedule time 8 duration 2
    When the actor's package is selected
    Then the selected package is 0x00000010

  Scenario: A schedule wraps past midnight
    Given a selection game hour of 0.5
    And a package candidate 0x00000010 of type 0
    And candidate 0x00000010 has schedule time 22 duration 4
    When the actor's package is selected
    Then the selected package is 0x00000010

  Scenario: A true condition selects the package
    Given a selection game hour of 12.0
    And condition function 100 returns 1.0
    And a package candidate 0x00000010 of type 0
    And candidate 0x00000010 requires function 100 equal 1.0
    When the actor's package is selected
    Then the selected package is 0x00000010

  Scenario: A false condition rejects the package
    Given a selection game hour of 12.0
    And condition function 100 returns 1.0
    And a package candidate 0x00000010 of type 0
    And candidate 0x00000010 requires function 100 equal 5.0
    When the actor's package is selected
    Then no package is selected
    And package candidate 0x00000010 was rejected as "conditions-false"
    And the selection counts unsupported_type 0 out_of_schedule 0 conditions_false 1 conditions_unevaluable 0 schedule_gap 1

  Scenario: An unsupported condition function is unevaluable and routed out
    Given a selection game hour of 12.0
    And a package candidate 0x00000010 of type 0
    And candidate 0x00000010 requires function 4242 equal 1.0
    When the actor's package is selected
    Then no package is selected
    And package candidate 0x00000010 was rejected as "conditions-unevaluable"
    And the selection counts unsupported_type 0 out_of_schedule 0 conditions_false 0 conditions_unevaluable 1 schedule_gap 1

  Scenario: A positive duration is measured in hours and has an exclusive end
    Given a selection game hour of 19.99
    And a package candidate 0x00000010 of type 0
    And candidate 0x00000010 has schedule time 8 duration 12
    When the actor's package is selected
    Then the selected package is 0x00000010

  Scenario: A schedule duration of at least 24 hours is active all day
    Given a selection game hour of 3.0
    And a package candidate 0x00000010 of type 0
    And candidate 0x00000010 has schedule time 8 duration 24
    When the actor's package is selected
    Then the selected package is 0x00000010

  Scenario: A non-positive duration remains open until midnight
    Given a selection game hour of 23.5
    And a package candidate 0x00000010 of type 0
    And candidate 0x00000010 has schedule time 8 duration 0
    When the actor's package is selected
    Then the selected package is 0x00000010
