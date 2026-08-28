Feature: Field repair, schematic crafting, and merchant barter
  Every repair, craft, purchase, and sale is an idempotent canonical
  ItemLedger mutation. Repair uses integer condition units matching
  prepared max_condition. Barter quotes are computed from Fallout3.esm
  GMSTs and committed through existing Buy/Sell. Recipe CTDA that is
  still opaque is UnsupportedCondition, never true. Merchant restock is
  a game-time policy; Wave 9 owns the scheduler.

  Scenario: Repair combines two items and consumes one donor
    Given the canonical player holds item 0x1 form 0x00000010 x1 condition 40
    And the canonical player also holds item 0x2 form 0x00000010 x3 condition 40
    And the repair max condition is 100
    And the player repair skill is 50
    When repairing target 0x1 with donor 0x2
    Then the repair succeeds
    And the canonical player item 0x1 has condition 50
    And the canonical player item 0x2 has count 2

  Scenario: Repair never exceeds the skill cap
    Given the canonical player holds item 0x1 form 0x00000010 x1 condition 10
    And the canonical player also holds item 0x2 form 0x00000010 x1 condition 10
    And the repair max condition is 100
    And the player repair skill is 100
    When repairing target 0x1 with donor 0x2
    Then the repair succeeds
    And the canonical player item 0x1 has condition 45

  Scenario: Repair rejects identical target and donor
    Given the canonical player holds item 0x1 form 0x00000010 x2 condition 40
    And the repair max condition is 100
    And the player repair skill is 50
    When repairing target 0x1 with donor 0x1
    Then the repair is rejected as same item

  Scenario: Repair rejects incompatible items
    Given the canonical player holds item 0x1 form 0x00000010 x1 condition 40
    And the canonical player also holds item 0x2 form 0x00000011 x1 condition 40
    And the repair max condition is 100
    And the player repair skill is 50
    When repairing target 0x1 with donor 0x2
    Then the repair is rejected as incompatible

  Scenario: Repair rejects an equipped donor
    Given the canonical player holds item 0x1 form 0x00000010 x1 condition 40
    And the canonical player also holds item 0x2 form 0x00000010 x1 condition 40
    And the canonical player equips item 0x2
    And the repair max condition is 100
    And the player repair skill is 50
    When repairing target 0x1 with donor 0x2
    Then the repair is rejected as equipped donor
    And the canonical player item 0x2 has count 1

  Scenario: A repeated repair transaction id does not consume another donor
    Given the canonical player holds item 0x1 form 0x00000010 x1 condition 40
    And the canonical player also holds item 0x2 form 0x00000010 x3 condition 40
    And the repair max condition is 100
    And the player repair skill is 50
    When repairing target 0x1 with donor 0x2 using transaction 7
    And repairing target 0x1 with donor 0x2 using transaction 7
    Then the repair succeeds
    And the canonical player item 0x2 has count 2

  Scenario: Crafting consumes ingredients and allocates output ids
    Given the canonical player holds item 0x1 form 0x00000030 x5 condition none
    And a known schematic 0x00000020 requiring 2 of 0x00000030 and producing 1 of 0x00000040
    When crafting recipe 0x00000020 once
    Then the craft succeeds
    And the canonical player item 0x1 has count 3
    And the canonical player holds 1 of form 0x00000040

  Scenario: Unsupported recipe conditions change nothing
    Given the canonical player holds item 0x1 form 0x00000030 x5 condition none
    And a known schematic 0x00000021 with an opaque condition requiring 1 of 0x00000030 and producing 1 of 0x00000040
    When crafting recipe 0x00000021 once
    Then the craft is rejected as unsupported condition
    And the canonical player item 0x1 has count 5

  Scenario: Failed crafting does not consume the next item id
    Given the canonical player holds item 0x1 form 0x00000030 x1 condition none
    And a known schematic 0x00000020 requiring 2 of 0x00000030 and producing 1 of 0x00000040
    When crafting recipe 0x00000020 once
    Then the craft is rejected as missing ingredients
    And the next canonical item id is unchanged

  Scenario: Barter buy uses Fallout 3 buy GMSTs
    Given a catalog item 0x00000010 worth 100 caps
    And the player barter skill is 0
    When quoting a buy of 1 of item 0x00000010
    Then the barter unit price is 155
    And the barter total is 155

  Scenario: Barter sell uses Fallout 3 sell GMSTs
    Given a catalog item 0x00000010 worth 100 caps
    And the player barter skill is 100
    When quoting a sell of 2 of item 0x00000010
    Then the barter unit price is 90
    And the barter total is 180

  Scenario: A stale barter quote is rejected
    Given the canonical player holds item 0x1 form 0x00000010 x1 condition none
    And the canonical merchant 0x00000009 holds item 0x2 form 0x00000010 x1 condition none with 50 caps
    And the canonical player has 200 caps
    And a catalog item 0x00000010 worth 100 caps
    And the player barter skill is 0
    When quoting a buy of item 0x2 from merchant 0x00000009
    And the merchant holder revision changes
    And committing the last barter quote
    Then the barter commit is rejected as stale

  Scenario: Merchant restock is due after 72 game hours
    Given a merchant restock last due at 0 ms
    When restock is evaluated at 259199999 ms
    Then restock is not due
    When restock is evaluated at 259200000 ms
    Then restock is due
    And the restock generation is 1
