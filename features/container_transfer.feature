Feature: Container open and world-loot transfer policy
  # Pure `container_policy` seam (issue #75): seeding a container from its
  # prepared inventory (leveled entries resolved once via the #74 resolver
  # seam) and the five transfer operations that move stacks between a
  # container and the player, conserving totals.

  Scenario: Non-leveled entries seed a container's stacks on first open
    Given a container inventory entry 0x00000010 x3 leveled no
    And a container inventory entry 0x00000011 x1 leveled no
    When the container is opened for the first time
    Then the container stack for 0x00000010 is 3
    And the container stack for 0x00000011 is 1
    And the container is resolved

  Scenario: A leveled entry resolves exactly once via the resolver seam
    Given a container inventory entry 0x00000099 x1 leveled yes
    And the leveled resolver for list 0x00000099 returns 0x00000020 x2
    When the container is opened for the first time
    Then the container stack for 0x00000020 is 2
    And the resolver was called 1 time

  Scenario: Reopening a resolved container never re-rolls the resolver
    Given a container inventory entry 0x00000099 x1 leveled yes
    And the leveled resolver for list 0x00000099 returns 0x00000020 x2
    When the container is opened for the first time
    And the container is reopened
    Then the resolver was called 1 time
    And the container stack for 0x00000020 is 2

  Scenario: Take one moves a single unit and conserves the total
    Given a container stack of 0x00000010 x5
    And a player stack of 0x00000010 x0
    When one 0x00000010 is taken from the container
    Then the container stack for 0x00000010 is 4
    And the player stack for 0x00000010 is 1

  Scenario: Take a partial stack conserves the total
    Given a container stack of 0x00000010 x5
    And a player stack of 0x00000010 x1
    When a stack of 3 0x00000010 is taken from the container
    Then the container stack for 0x00000010 is 2
    And the player stack for 0x00000010 is 4

  Scenario: Take all empties the container's stack
    Given a container stack of 0x00000010 x5
    And a player stack of 0x00000010 x0
    When all 0x00000010 is taken from the container
    Then the container stack for 0x00000010 is 0
    And the player stack for 0x00000010 is 5

  Scenario: Store one moves a single unit and conserves the total
    Given a container stack of 0x00000010 x0
    And a player stack of 0x00000010 x2
    When one 0x00000010 is stored into the container
    Then the container stack for 0x00000010 is 1
    And the player stack for 0x00000010 is 1

  Scenario: Storing a stack into an empty container conserves the total
    Given a container stack of 0x00000010 x0
    And a player stack of 0x00000010 x4
    When a stack of 4 0x00000010 is stored into the container
    Then the container stack for 0x00000010 is 4
    And the player stack for 0x00000010 is 0

  Scenario: A zero count transfer is rejected and nothing moves
    Given a container stack of 0x00000010 x5
    And a player stack of 0x00000010 x0
    When a stack of 0 0x00000010 is taken from the container
    Then the transfer is rejected
    And the container stack for 0x00000010 is 5
    And the player stack for 0x00000010 is 0

  Scenario: A negative count transfer is rejected and nothing moves
    Given a container stack of 0x00000010 x5
    And a player stack of 0x00000010 x0
    When a stack of -2 0x00000010 is taken from the container
    Then the transfer is rejected
    And the container stack for 0x00000010 is 5
    And the player stack for 0x00000010 is 0
