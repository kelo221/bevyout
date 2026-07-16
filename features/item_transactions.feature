Feature: Canonical item instances and atomic holder transactions
  # M3/#95 pure transaction seam. Every stack has a stable id; partial moves
  # split deterministically, compatible merges report their id remap, and a
  # failed request leaves both holders unchanged.

  Scenario: A partial transfer preserves the source id and allocates a destination id
    Given the canonical player holds item 0x7 form 0x00000001 x5 condition 80
    And the canonical holder 0x00000009 is empty
    When transferring 2 of item 0x7 to canonical holder 0x00000009
    Then the canonical player item 0x7 has count 3
    And canonical holder 0x00000009 has item count 2
    And the transaction moved item id 0x8

  Scenario: A failed transfer is atomic
    Given the canonical player holds item 0x7 form 0x00000001 x2 condition none
    And the canonical holder 0x00000009 is empty
    When transferring 3 of item 0x7 to canonical holder 0x00000009
    Then the canonical transaction is rejected
    And the canonical player item 0x7 has count 2
    And canonical holder 0x00000009 is empty
