Feature: Pure SPECIAL, skill, derived-stat, and leveling kernels
  Player stats are computed by Bevy-free kernels from SPECIAL attributes,
  tagged skills, and GMST settings with Fallout 3 GOTY defaults, so derived
  values, XP thresholds, and level-ups are deterministic everywhere.

  Scenario: Skill bases follow primary SPECIAL and round luck up
    Given a player sheet with all SPECIAL at 5 and luck at 5
    Then the lockpick skill base is 15
    And the unarmed skill base is 15
    And the small guns skill base is 15

  Scenario: A tagged skill gains its flat bonus
    Given a player sheet with all SPECIAL at 5 and luck at 10
    When the player tags the small guns skill
    Then the small guns skill base is 32
    And the small guns skill value is 32

  Scenario: Derived attributes follow the GMST multiplier formula
    Given a player sheet with all SPECIAL at 5 and luck at 5
    Then the derived max health is 200
    And the derived max action points is 75
    And the derived carry weight is 200
    And the derived critical chance is 500 basis points

  Scenario: Derived attributes grow with level and SPECIAL
    Given a player sheet with all SPECIAL at 5 and luck at 5
    When the player reaches level 4 with endurance 8 and strength 7
    Then the derived max health is 290
    And the derived carry weight is 220

  Scenario: Awarding threshold XP levels up once and grants skill points
    Given a player sheet with all SPECIAL at 5 and luck at 5
    When the player is awarded 200 XP
    Then the player is level 2 with 0 XP
    And the level-up granted 15 skill points

  Scenario: A single award can cross several level thresholds
    Given a player sheet with all SPECIAL at 5 and luck at 5
    When the player is awarded 700 XP
    Then the player is level 3 with 150 XP
    And the level-up granted 30 skill points

  Scenario: XP stops accumulating at the level cap
    Given a player sheet with all SPECIAL at 5 and luck at 5
    When the player is awarded 999999 XP
    Then the player is level 30
    And the player XP never exceeds the level 30 threshold

  Scenario: Effective SPECIAL and skills stay clamped to their ranges
    Given a player sheet with all SPECIAL at 5 and luck at 5
    When the player strength is raised by 20 and reduced by 30
    And the player spends 95 skill points on the science skill
    Then the effective strength is 1
    And the science skill value is 100

  Scenario: Resistances are capped at 85 percent
    Given a player sheet with all SPECIAL at 5 and luck at 5
    When damage resistance 9900 basis points is clamped
    Then the clamped damage resistance is 8500 basis points
    And the base poison resistance from endurance 5 is 2000 basis points
