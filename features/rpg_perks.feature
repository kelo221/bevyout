Feature: Pure perk eligibility and active-modifier kernels
  Perk requirements decoded from PERK records (level and rank gates plus
  GetActorValue conditions resolved through the probed AV-index mapping)
  decide whether a perk can be taken, and owned perk entries project onto
  the wave-1 leveling kernels as an XP award multiplier and a per-level
  skill-point bonus.

  Scenario: The level gate blocks low-level players
    Given a player sheet with all SPECIAL at 5 and luck at 5
    And a perk 00031dd3 requiring level 2 with 3 ranks
    When the perk eligibility for 00031dd3 is evaluated
    Then the perk 00031dd3 is blocked with reason min_level

  Scenario: The rank gate blocks fully owned perks
    Given a player sheet with all SPECIAL at 5 and luck at 5
    And a perk 00031dde requiring level 1 with 1 rank
    And the player owns perk 00031dde at rank 1
    When the perk eligibility for 00031dde is evaluated
    Then the perk 00031dde is blocked with reason max_ranks

  Scenario: Actor-value conditions resolve through the probed index mapping
    Given a player sheet with all SPECIAL at 5 and luck at 5
    And a perk 00031dde requiring level 1 with 1 rank
    And a perk 00031dde gated on condition index 7 at 5
    When the perk eligibility for 00031dde is evaluated
    Then the perk 00031dde is eligible
    When the player reaches level 1 with endurance 3 and strength 5
    And the perk eligibility for 00031dde is evaluated
    Then the perk 00031dde is blocked with reason condition

  Scenario: Conditions this build cannot evaluate block conservatively
    Given a player sheet with all SPECIAL at 5 and luck at 5
    And a perk 00030666 requiring level 1 with 1 rank
    And a perk 00030666 with an unknown condition
    When the perk eligibility for 00030666 is evaluated
    Then the perk 00030666 is blocked with reason unknown_condition

  Scenario: The owned rank's entry replaces lower ranks
    Given a player sheet with all SPECIAL at 5 and luck at 5
    And a perk 00031dd3 requiring level 1 with 3 ranks
    And a perk 00031dd3 entry rank 0 with XP multiplier 1.1
    And a perk 00031dd3 entry rank 1 with XP multiplier 1.2
    And a perk 00031dd3 entry rank 2 with XP multiplier 1.3
    When the active perk modifiers are recomputed
    Then the XP award multiplier is 10000 basis points
    Given the player owns perk 00031dd3 at rank 1
    When the active perk modifiers are recomputed
    Then the XP award multiplier is 11000 basis points
    Given the player owns perk 00031dd3 at rank 2
    When the active perk modifiers are recomputed
    Then the XP award multiplier is 12000 basis points

  Scenario: Educated grants bonus skill points on level-up
    Given a player sheet with all SPECIAL at 5 and luck at 5
    And a perk 00031dd8 requiring level 1 with 1 rank
    And a perk 00031dd8 entry rank 0 granting 3 bonus skill points
    Given the player owns perk 00031dd8 at rank 1
    When the active perk modifiers are recomputed
    Then the perk skill point bonus is 3
    When the player is awarded 200 XP with active perk modifiers
    Then the level-up granted 18 skill points

  Scenario: The XP multiplier scales awarded XP through the kernel
    Given a player sheet with all SPECIAL at 5 and luck at 5
    And a perk 00031dd3 requiring level 1 with 3 ranks
    And a perk 00031dd3 entry rank 0 with XP multiplier 1.1
    Given the player owns perk 00031dd3 at rank 1
    When the player is awarded 1000 XP with active perk modifiers
    Then the player is level 4 with 50 XP
