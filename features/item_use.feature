Feature: Consumable use and reading
  # Pure `item_use` seam (issue #99): Aid is usable, Book/Note with text is
  # readable, Key/Misc are inert; using a consumable removes exactly one
  # from its stack; a quest-flagged item (issue #81) is readable but can
  # never be consumed away.

  Scenario: Aid items are usable
    Given an item with stats Aid quest no
    Then the item use action is Use

  Scenario: A quest-flagged Aid item can never be consumed
    Given an item with stats Aid quest yes
    Then the item use action is Inert

  Scenario: Books with text are readable
    Given an item with stats Book text yes quest no
    Then the item use action is Read

  Scenario: Books without text are inert
    Given an item with stats Book text no quest no
    Then the item use action is Inert

  Scenario: Notes with text are readable
    Given an item with stats Note text yes quest no
    Then the item use action is Read

  Scenario: Notes without text are inert
    Given an item with stats Note text no quest no
    Then the item use action is Inert

  Scenario: A quest-flagged note stays readable
    Given an item with stats Note text yes quest yes
    Then the item use action is Read

  Scenario: Keys are inert
    Given an item with stats Key quest no
    Then the item use action is Inert

  Scenario: Misc items are inert
    Given an item with stats Misc quest no
    Then the item use action is Inert

  Scenario: Using a consumable removes exactly one from its stack
    Given an empty player inventory
    When 3 items 0x00000010 without condition are added
    And item 0x00000010 with stats Aid quest no is used
    Then the inventory has 1 stack and 2 total items

  Scenario: Using the last consumable removes the stack entirely
    Given an empty player inventory
    When 1 item 0x00000010 without condition is added
    And item 0x00000010 with stats Aid quest no is used
    Then the inventory has 0 stacks and 0 total items

  Scenario: A quest-flagged consumable is never removed by use
    Given an empty player inventory
    When 1 item 0x00000010 without condition is added
    And item 0x00000010 with stats Aid quest yes is used
    Then the inventory has 1 stack and 1 total items

  Scenario: Using an inert item does not touch the stack
    Given an empty player inventory
    When 1 item 0x00000010 without condition is added
    And item 0x00000010 with stats Key quest no is used
    Then the inventory has 1 stack and 1 total items
