Feature: Exterior gameplay-actor residency sequencing
  One prepared exterior actor is bound in the cell that owns it, handed off
  across a cell border, unloaded when its cell evicts, and restored when that
  cell reloads at a new generation -- with exactly one live owner claiming it
  at every step, so the canonical actor identity and its item holder never
  split in two.

  Scenario: an actor is bound, handed off, unloaded, and restored under one canonical owner
    Given resident exterior cell 00000c67 at generation 1 covers grid 5,-6
    And resident exterior cell 00000c68 at generation 1 covers grid 4,-6
    And exterior cell 00000c67 prepares actor 000638e8
    When exterior actor residency is planned
    Then the plan binds actor 000638e8 to cell 00000c67
    And no more than one live owner claimed actor 000638e8

    When actor 000638e8 is projected in cell 00000c67 at generation 1 on grid 5,-6
    And exterior actor residency is planned
    Then the plan leaves actor 000638e8 alone
    And no more than one live owner claimed actor 000638e8

    When actor 000638e8 crosses into grid 4,-6
    And exterior actor residency is planned
    Then the plan hands actor 000638e8 from cell 00000c67 to cell 00000c68
    And no more than one live owner claimed actor 000638e8

    When actor 000638e8 is projected in cell 00000c68 at generation 1 on grid 4,-6
    And exterior cell 00000c68 begins evicting at generation 2
    And exterior actor residency is planned
    Then the plan unloads actor 000638e8 from cell 00000c68
    And no more than one live owner claimed actor 000638e8

    When actor 000638e8 is no longer projected
    And actor 000638e8 has saved canonical state
    And exterior cell 00000c67 reloads at generation 2
    And exterior actor residency is planned
    Then the plan restores actor 000638e8 to cell 00000c67
    And no more than one live owner claimed actor 000638e8
