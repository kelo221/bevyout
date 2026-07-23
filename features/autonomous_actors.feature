Feature: Autonomous package driver (issues #218/#224)
  Before this wave, making an NPC patrol required a human to type `tna bind`
  + `runpackage` by hand for every actor, and a nav-bound actor's locomotion
  clip flapped between idle and run because the classifier read the raw,
  un-smoothed achieved speed. This feature exercises the pure decision rules
  behind both fixes: the eligibility gate the autonomous package driver
  (`viewer::ai::autonomous`) consults before binding + starting an actor with
  no console command, and the achieved-speed smoothing
  (`viewer::nav::locomotion::smooth_achieved_speed`) that keeps the
  locomotion classifier from flapping when the raw sample swings between
  near-zero and full route speed every tick.

  Scenario: An alive actor with a Patrol package is selected for autonomous bind and start
    Given an actor with life state "alive"
    And package type 13 dispatches to the Patrol family
    When the autonomous package driver evaluates the actor
    Then the actor is selected for autonomous bind and start

  Scenario: A dead actor is never selected for autonomous start
    Given an actor with life state "dead"
    When the autonomous package driver evaluates the actor
    Then the actor is not selected for autonomous bind and start

  Scenario: An actor the console already nav-bound is left alone
    Given an actor with life state "alive"
    And the actor is already nav-bound
    When the autonomous package driver evaluates the actor
    Then the actor is not selected for autonomous bind and start

  Scenario: An actor already running a console-started package is left alone
    Given an actor with life state "alive"
    And the actor already has a running package controller
    When the autonomous package driver evaluates the actor
    Then the actor is not selected for autonomous bind and start

  Scenario: Achieved speed alternating every tick between full route speed and near zero does not flap the locomotion classifier
    Given a bound actor currently in the idle locomotion state
    When its achieved horizontal speed alternates every tick between full route speed and near zero for 128 ticks, smoothed before classification
    Then its locomotion state changed at most 1 time after the smoothing warmed up
