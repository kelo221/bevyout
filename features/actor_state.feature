Feature: Stable actor definition and mutable instance state
  Actor identity and authored traits are immutable while runtime mutations
  survive cell eviction and save/load without duplicating canonical inventory.

  Scenario: Actor values resolve through authored layers and runtime mutation
    Given an actor definition with template health 80 and base health 100
    And the actor has race health modifier 5, class health modifier 10, and faction health modifier -2
    And the actor instance has runtime health mutation -25
    When the actor health is resolved
    Then the effective actor health is 88
    And the persisted actor state contains no derived value snapshot

  Scenario: A missing base value falls back to its inherited template
    Given an actor definition with template fatigue 70 and no base fatigue
    And the actor instance has runtime fatigue mutation 3
    When the actor fatigue is resolved
    Then the effective actor fatigue is 73

  Scenario: Faction membership does not imply hostility
    Given actor reference 0x00041600 belongs to faction 0x0001f17b at rank 2
    Then actor reference 0x00041600 has faction 0x0001f17b at rank 2
    And the actor definition contains no hostility decision

  Scenario: Cell revisit seeds mutable actor state exactly once
    Given an empty actor state store
    When actor reference 0x00041600 in cell 0x00017f37 is seeded alive
    And actor reference 0x00041600 receives runtime health mutation -12
    And actor reference 0x00041600 in cell 0x00017f37 is seeded alive again
    Then actor reference 0x00041600 has runtime health mutation -12
    And exactly one actor instance is stored

  Scenario: Lifecycle and package checkpoint survive serialization
    Given actor reference 0x0005443b in cell 0x00024511 is dead
    And actor reference 0x0005443b is running package 0x0002c6f1 procedure 3 for 4.5 seconds
    When the actor state store is serialized and restored
    Then actor reference 0x0005443b remains dead
    And actor reference 0x0005443b retains package 0x0002c6f1 procedure 3 at 4.5 seconds

  Scenario: Canonical actor inventory is not duplicated by revisit
    Given canonical actor reference 0x00041600 owns item instance 7 with 2 of base item 0x00004322
    When canonical actor reference 0x00041600 is projected twice
    Then canonical actor reference 0x00041600 still owns 2 items in one instance
