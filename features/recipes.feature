Feature: Prepared recipe validation
  # The parser and content-addressed catalog are covered by Rust unit tests;
  # this executable-spec seam pins the pure policy future crafting uses before
  # it mutates inventory.

  Scenario: Non-positive quantities are rejected without mutation
    Given a recipe with ingredient 0x00000010 quantity -1 and output 0x00000020 quantity 1
    And recipe items "00000010,00000020" are available
    When the recipe is validated
    Then recipe validation rejects a non-positive quantity
    And the recipe ingredient quantity remains -1

  Scenario: Duplicate ingredients are rejected as one recipe
    Given a recipe with ingredient 0x00000010 quantity 1 and output 0x00000020 quantity 1
    And the recipe also has ingredient 0x00000010 quantity 2
    And recipe items "00000010,00000020" are available
    When the recipe is validated
    Then recipe validation rejects duplicate ingredients
