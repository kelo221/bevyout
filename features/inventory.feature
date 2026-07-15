Feature: Authoritative inventory and drop policy

  Scenario: Equal-condition items merge into one stack
    Given an empty player inventory
    When 2 items 0x00000010 at condition 100 are added
    And 3 items 0x00000010 at condition 100 are added
    Then the inventory has 1 stack and 5 total items

  Scenario: Differently damaged items do not merge
    Given an empty player inventory
    When 1 item 0x00000010 at condition 100 is added
    And 1 item 0x00000010 at condition 80 is added
    Then the inventory has 2 stacks and 2 total items

  Scenario: Carried weight is authoritative
    Given an empty player inventory
    When 3 items 0x00000010 without condition are added
    And item 0x00000010 weighs 2.5
    Then carried weight is 7.5

  Scenario: Small stacks drop one directly
    Given an inventory stack of 3 items
    When its right-click drop policy is evaluated
    Then one item is selected for dropping

  Scenario: Large stacks request a quantity
    Given an inventory stack of 50 items
    When its right-click drop policy is evaluated
    Then a quantity from 1 through 50 is requested with default 1

  Scenario: Failed transfer preserves the stack
    Given an inventory stack of 2 items
    When removing 3 items is attempted
    Then the transfer is rejected and 2 items remain
