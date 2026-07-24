Feature: Modular first-person weapon actions and actor damage
  The first playable firearm slice fires and reloads deterministically without
  introducing ammunition accounting, while actor damage remains persisted.

  Scenario: A 10mm shot fires from idle without consuming ammunition
    Given an idle weapon with damage 9 and range 100 metres
    And weapon ammunition accounting is disabled
    When the weapon fire action is requested
    Then the weapon action is firing
    And 1 shot has been accepted
    And no ammunition has been consumed

  Scenario: Reload blocks firing until its action completes
    Given an idle weapon with damage 9 and range 100 metres
    When the weapon reload action is requested
    And the weapon fire action is requested
    Then the weapon action is reloading
    And the fire request is blocked by reload
    When the weapon advances by 1.5 seconds
    And the weapon fire action is requested
    Then the weapon action is firing
    And 1 shot has been accepted

  Scenario: A nonlethal weapon hit persists actor health damage
    Given an alive actor with base health 20
    When the actor receives 9 weapon damage
    Then the weapon-damaged actor health is 11
    And the actor remains alive

  Scenario: A lethal weapon hit changes actor lifecycle
    Given an alive actor with base health 8
    When the actor receives 9 weapon damage
    Then the weapon-damaged actor health is 0
    And the actor is dead
