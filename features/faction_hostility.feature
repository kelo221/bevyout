Feature: Deterministic faction disposition and hostility (issue #116)
  The observer's disposition toward a target and the hostility verdict are
  resolved by an explicit rule precedence with stable tie-breaking, and the
  deciding rule is always reported.

  Scenario: An actor is never hostile to itself
    Given a hostility observer with base disposition 0 and aggression "frenzied"
    And the target is the observer itself
    When hostility is resolved
    Then the hostility verdict is "friendly"
    And the deciding rule is "same_actor"

  Scenario: A frenzied actor is hostile even to a faction ally
    Given faction "10" and "20" are allies
    And a hostility observer in faction "10" with base disposition 50 and aggression "frenzied"
    And the target is in faction "20"
    When hostility is resolved
    Then the hostility verdict is "hostile"
    And the deciding rule is "frenzied"

  Scenario: A faction enemy relation forces hostility despite high disposition
    Given faction "10" and "20" are enemies
    And a hostility observer in faction "10" with base disposition 100 and aggression "unaggressive"
    And the target is in faction "20"
    When hostility is resolved
    Then the hostility verdict is "hostile"
    And the deciding rule is "faction_enemy"

  Scenario: Shared faction membership is friendly and overrides aggression
    Given a hostility observer in faction "10" with base disposition 20 and aggression "aggressive"
    And the target is in faction "10"
    When hostility is resolved
    Then the hostility verdict is "friendly"
    And the deciding rule is "shared_faction"

  Scenario: A faction ally relation is friendly
    Given faction "10" and "30" are allies
    And a hostility observer in faction "10" with base disposition 20 and aggression "aggressive"
    And the target is in faction "30"
    When hostility is resolved
    Then the hostility verdict is "friendly"
    And the deciding rule is "faction_ally_or_friend"

  Scenario: An aggressive actor attacks a low-disposition stranger
    Given a hostility observer with base disposition 40 and aggression "aggressive"
    And the target is the player
    When hostility is resolved
    Then the hostility verdict is "hostile"
    And the deciding rule is "aggressive"

  Scenario: An unaggressive stranger falls through to the disposition bucket
    Given a hostility observer with base disposition 50 and aggression "unaggressive"
    And the target is the player
    When hostility is resolved
    Then the hostility verdict is "neutral"
    And the deciding rule is "disposition_threshold"

  Scenario: Faction modifiers move the disposition value
    Given faction "10" applies a disposition modifier -40 toward faction "20"
    And a hostility observer in faction "10" with base disposition 50 and aggression "unaggressive"
    And the target is in faction "20"
    When hostility is resolved
    Then the resolved disposition is 10
    And the hostility verdict is "hostile"

  Scenario: An unresolved faction is diagnosed, not guessed
    Given a hostility observer in unknown faction "abcd" with base disposition 50 and aggression "unaggressive"
    And the target is the player
    When hostility is resolved
    Then a hostility diagnostic mentions "unresolved observer faction 0000abcd"
