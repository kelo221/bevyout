Feature: Bounded chunked reveal for a freshly-activated preloaded cell
  # Pins issue #55's pure reveal-chunk planning:
  # `src/viewer/world/reveal_policy.rs`. A preloaded cell's placement
  # entities flip visible in bounded chunks of at most one budget's worth
  # each, ordered nearest-to-the-player's-arrival-point first, instead of
  # all at once (the measured 84 ms first-reveal spike for Vault101d's
  # 1,371 placements). A cell at or under one budget's worth of entities
  # still reveals in exactly one chunk -- wave-2's single-frame reveal,
  # preserved bit-for-bit. Drives `reveal_policy` directly, the same way
  # features/preload_policy.feature drives `world::policy`.

  Scenario: A cell larger than the budget splits into ceil(n / budget) chunks
    Given 10 reveal candidates evenly spaced from the arrival point
    And the reveal budget is 3
    When the reveal chunks are planned
    Then there are 4 reveal chunks
    And every reveal candidate appears in exactly one chunk

  Scenario: The chunk nearest the arrival point reveals first
    Given a reveal candidate at distance 100 from the arrival point
    And a reveal candidate at distance 0 from the arrival point
    And a reveal candidate at distance 50 from the arrival point
    And a reveal candidate at distance 10 from the arrival point
    And the reveal budget is 2
    When the reveal chunks are planned
    Then the first reveal chunk contains the candidate at distance 0
    And the first reveal chunk contains the candidate at distance 10

  Scenario: A cell at or under the budget reveals in exactly one chunk
    Given 5 reveal candidates evenly spaced from the arrival point
    And the reveal budget is 5
    When the reveal chunks are planned
    Then there is 1 reveal chunk

  Scenario: A cell well under the budget still reveals in exactly one chunk
    Given 5 reveal candidates evenly spaced from the arrival point
    And the reveal budget is 99
    When the reveal chunks are planned
    Then there is 1 reveal chunk
