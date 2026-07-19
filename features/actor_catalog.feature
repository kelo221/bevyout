Feature: Actor catalog template resolution
  # `vsa::prepare::actor_catalog` is pure (issue #103, M4 wave 1 task C):
  # given plain NPC_/CREA/leveled-list/race/class/faction/package inputs and
  # ACHR/ACRE placements, it resolves each placement into an ActorBlueprint
  # by applying ACBS template flags field-by-field through TPLT chains.
  # Byte-level NPC_/CREA/RACE/CLAS/FACT/PACK decoding is covered by
  # tests/actors.rs and tests/actor_support.rs (tasks A/B).

  Scenario: A flagged group is inherited from the template
    Given an NPC_ actor 0x00000010 with race 0x000000AA
    And an NPC_ actor 0x00000020 with race 0x000000BB
    And actor 0x00000020 has template 0x00000010 using traits
    And a placement 0x00000001 of base 0x00000020 as Npc
    When the actor catalog is built
    Then blueprint for reference 0x00000001 has race 0x000000AA
    And blueprint for reference 0x00000001 is inherited

  Scenario: An unflagged group keeps the actor's own data
    Given an NPC_ actor 0x00000010 with race 0x000000AA
    And an NPC_ actor 0x00000020 with race 0x000000BB
    And actor 0x00000020 has template 0x00000010 using stats
    And a placement 0x00000001 of base 0x00000020 as Npc
    When the actor catalog is built
    Then blueprint for reference 0x00000001 has race 0x000000BB

  Scenario: An actor with no template flags at all is never marked inherited
    Given an NPC_ actor 0x00000010 with race 0x000000AA
    And an NPC_ actor 0x00000020 with race 0x000000BB
    And a placement 0x00000001 of base 0x00000020 as Npc
    When the actor catalog is built
    Then blueprint for reference 0x00000001 has race 0x000000BB
    And blueprint for reference 0x00000001 is not inherited

  Scenario: A template cycle is diagnosed, not silently followed forever
    Given an NPC_ actor 0x00000010 with race 0x00000001
    And an NPC_ actor 0x00000020 with race 0x00000002
    And actor 0x00000010 has template 0x00000020 using traits
    And actor 0x00000020 has template 0x00000010 using traits
    And a placement 0x00000001 of base 0x00000010 as Npc
    When the actor catalog is built
    Then blueprint for reference 0x00000001 has diagnostic containing "template cycle"

  Scenario: A missing template target is diagnosed, not silently ignored
    Given an NPC_ actor 0x00000010 with race 0x00000001
    And actor 0x00000010 has template 0x0000DEAD using stats
    And a placement 0x00000001 of base 0x00000010 as Npc
    When the actor catalog is built
    Then blueprint for reference 0x00000001 has diagnostic containing "unresolved template target 0000dead"

  Scenario: A template chain through a leveled list records the candidate set
    Given an NPC_ actor 0x00000010 with race 0x00000001
    And actor 0x00000010 has template 0x00000099 using traits
    And a leveled list 0x00000099 with entries "00000030,00000010,00000020"
    And a placement 0x00000001 of base 0x00000010 as Npc
    When the actor catalog is built
    Then blueprint for reference 0x00000001 is a leveled template with candidates "00000010,00000020,00000030"

  Scenario: A nested leveled template resolves one concrete actor for traits and inventory
    Given an NPC_ actor 0x00000010 with race 0x000000AA
    And actor 0x00000010 has template 0x00000090 using traits,inventory
    And a leveled list 0x00000090 with entries "00000091"
    And a leveled list 0x00000091 with entries "00000020"
    And an NPC_ actor 0x00000020 with race 0x000000BB
    And actor 0x00000020 is female
    And actor 0x00000020 has inventory item 0x00000500 x1
    And a placement 0x00000001 of base 0x00000010 as Npc
    When the actor catalog is built
    Then blueprint for reference 0x00000001 is a leveled template with candidates "00000091"
    And blueprint for reference 0x00000001 resolves base 0x00000020
    And blueprint for reference 0x00000001 is female
    And blueprint for reference 0x00000001 has race 0x000000BB
    And blueprint for reference 0x00000001 has inventory item 0x00000500 x1

  Scenario: Unresolved race, class, faction, and package links are diagnosed
    Given an NPC_ actor 0x00000010 with race 0x00000001
    And actor 0x00000010 class is 0x00000002
    And actor 0x00000010 has faction 0x00000003 rank 0
    And actor 0x00000010 has package 0x00000004
    And a placement 0x00000001 of base 0x00000010 as Npc
    When the actor catalog is built
    Then blueprint for reference 0x00000001 has diagnostic "unresolved race link 00000001"
    And blueprint for reference 0x00000001 has diagnostic "unresolved class link 00000002"
    And blueprint for reference 0x00000001 has diagnostic "unresolved faction link 00000003"
    And blueprint for reference 0x00000001 has diagnostic "unresolved package link 00000004"

  Scenario: A resolved faction membership carries its rank title
    Given an NPC_ actor 0x00000010 with race 0x00000001
    And actor 0x00000010 has faction 0x00000050 rank 2
    And faction 0x00000050 is known with rank 2 male title "Paladin" female title "Paladine"
    And a placement 0x00000001 of base 0x00000010 as Npc
    When the actor catalog is built
    Then blueprint for reference 0x00000001 has faction 0x00000050 title "Paladin"

  Scenario: A missing base record is a diagnosed unresolved entry
    Given a placement 0x00000001 of base 0x0000FFEE as Npc
    When the actor catalog is built
    Then entry for reference 0x00000001 is unresolved
    And the actor catalog counts prepared 0 inherited 0 unresolved 1 unsupported 0 skipped 0

  Scenario: A direct leveled-list base is skipped, not silently defaulted
    Given a leveled list 0x00000077 with entries "00000010,00000020"
    And a placement 0x00000001 of base 0x00000077 as Creature
    When the actor catalog is built
    Then entry for reference 0x00000001 is skipped

  Scenario: Entries are ordered by base then reference FormID and serialize deterministically
    Given an NPC_ actor 0x00000010 with race 0x00000001
    And an NPC_ actor 0x00000020 with race 0x00000001
    And a placement 0x00000002 of base 0x00000020 as Npc
    And a placement 0x00000001 of base 0x00000010 as Npc
    And a placement 0x00000001 of base 0x00000020 as Npc
    When the actor catalog is built
    Then the actor catalog entries are ordered "00000010/00000001,00000020/00000001,00000020/00000002"
    And serializing the actor catalog twice yields identical RON
