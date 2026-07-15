Feature: Deterministic leveled-list resolution
  # Pins issue #74's pure resolver seam: `viewer::interaction::leveled`.
  # Entries are written as "level:base_form_id:count" (comma-separated);
  # resolved stacks are written as "base_form_id x count" (comma-separated,
  # in resolution order). Drives `leveled::resolve_leveled` and
  # `leveled::LeveledSeed::derive` directly, the same way
  # features/door_animation.feature drives `animation::policy`.

  Scenario: The same seed resolves a list identically twice
    Given leveled list 100 with chance-none 0 and flags 0 and entries "1:200:1, 5:201:1"
    And a leveled seed from playthrough 42, cell 16, reference 32 named "seed"
    When list 100 is resolved for player level 10 using seed "seed"
    Then the resolved stacks are "201x1"
    When list 100 is resolved for player level 10 using seed "seed"
    Then the resolved stacks are "201x1"

  Scenario: Only entries at or below the player's level are eligible
    Given leveled list 100 with chance-none 0 and flags 0 and entries "10:200:1"
    And a leveled seed from playthrough 1, cell 1, reference 2 named "seed"
    When list 100 is resolved for player level 1 using seed "seed"
    Then the resolved stacks are empty
    When list 100 is resolved for player level 10 using seed "seed"
    Then the resolved stacks are "200x1"

  Scenario: Chance-none of 100 always empties the list
    Given leveled list 100 with chance-none 100 and flags 0 and entries "1:200:1"
    And a leveled seed from playthrough 1, cell 1, reference 2 named "seed"
    When list 100 is resolved for player level 50 using seed "seed"
    Then the resolved stacks are empty

  Scenario: A nested list recurses to the leaf item and scales its count
    Given leveled list 100 with chance-none 0 and flags 0 and entries "1:101:3"
    And leveled list 101 with chance-none 0 and flags 0 and entries "1:200:2"
    And a leveled seed from playthrough 1, cell 1, reference 2 named "seed"
    When list 100 is resolved for player level 10 using seed "seed"
    Then the resolved stacks are "200x6"

  Scenario: Calculate-for-each-item rolls the nested list once per count unit
    Given leveled list 100 with chance-none 0 and flags calculate-for-each-item and entries "1:101:3"
    And leveled list 101 with chance-none 0 and flags 0 and entries "1:200:1"
    And a leveled seed from playthrough 1, cell 1, reference 2 named "seed"
    When list 100 is resolved for player level 10 using seed "seed"
    Then the resolved stacks are "200x1, 200x1, 200x1"

  Scenario: Use-all keeps every candidate at the highest unlocked level
    Given leveled list 100 with chance-none 0 and flags use-all and entries "1:200:1, 5:201:1, 5:202:1"
    And a leveled seed from playthrough 1, cell 1, reference 2 named "seed"
    When list 100 is resolved for player level 10 using seed "seed"
    Then the resolved stacks are "201x1, 202x1"

  Scenario: A self-referencing list resolves safely instead of looping
    Given leveled list 100 with chance-none 0 and flags 0 and entries "1:100:1"
    And a leveled seed from playthrough 1, cell 1, reference 2 named "seed"
    When list 100 is resolved for player level 10 using seed "seed"
    Then the resolved stacks are empty

  Scenario: A mutual cycle resolves safely instead of looping
    Given leveled list 100 with chance-none 0 and flags 0 and entries "1:101:1"
    And leveled list 101 with chance-none 0 and flags 0 and entries "1:100:1"
    And a leveled seed from playthrough 1, cell 1, reference 2 named "seed"
    When list 100 is resolved for player level 10 using seed "seed"
    Then the resolved stacks are empty

  Scenario: An unresolved list FormID yields no entries
    Given a leveled seed from playthrough 1, cell 1, reference 2 named "seed"
    When list 57005 is resolved for player level 10 using seed "seed"
    Then the resolved stacks are empty

  Scenario: The same playthrough, cell, and reference always derive the same seed
    Given a leveled seed from playthrough 7, cell 1, reference 2 named "a"
    And a leveled seed from playthrough 7, cell 1, reference 2 named "b"
    Then seeds "a" and "b" are identical

  Scenario: A different reference derives an independent seed
    Given a leveled seed from playthrough 7, cell 1, reference 2 named "a"
    And a leveled seed from playthrough 7, cell 1, reference 3 named "b"
    Then seeds "a" and "b" are different

  Scenario: A different cell derives an independent seed
    Given a leveled seed from playthrough 7, cell 1, reference 2 named "a"
    And a leveled seed from playthrough 7, cell 2, reference 2 named "b"
    Then seeds "a" and "b" are different
