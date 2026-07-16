Feature: Caps, ownership, and quest-item flags
  # Pure `item_rules` seam (issue #81): the quest-item header flag decoded
  # from base-record flags, the FO3 drop/store rules for quest items and
  # caps, quest-item exclusion from carried weight, and Take-vs-Steal
  # classification for owned references.

  Scenario: The quest-item header flag is decoded from record flags
    Given a base record with header flags 0x00000400
    Then the record is a quest item

  Scenario: Records without the quest-item header flag are ordinary
    Given a base record with header flags 0x00000009
    Then the record is not a quest item

  Scenario: Quest items cannot be dropped to the world
    When dropping item 0x00012345 quest yes is checked
    Then the transfer is rejected as quest item

  Scenario: Caps cannot be dropped to the world
    When dropping item 0x0000000F quest no is checked
    Then the transfer is rejected as caps

  Scenario: Ordinary items can be dropped to the world
    When dropping item 0x00012345 quest no is checked
    Then the transfer is allowed

  Scenario: Quest items cannot be stored into a container
    When storing item 0x00012345 quest yes is checked
    Then the transfer is rejected as quest item

  Scenario: Caps can be stored into a container
    When storing item 0x0000000F quest no is checked
    Then the transfer is allowed

  Scenario: Carried weight excludes quest items
    Given a carried stack of 3 weighing 2.0 each quest no
    And a carried stack of 1 weighing 50.0 each quest yes
    When the carried weight is totaled
    Then the carried weight is 6.0

  Scenario: Taking an unowned reference is not theft
    When taking a reference with no owner is classified
    Then the take is not theft

  Scenario: Taking a reference owned by another actor is theft
    When taking a reference owned by 0x0001A2B3 is classified
    Then the take is theft from 0x0001A2B3

  Scenario: Taking a reference the player owns is not theft
    When taking a reference owned by 0x00000007 is classified
    Then the take is not theft

  Scenario: The player caps total sums the caps stacks
    Given a player stack of 0x0000000F x120
    And a player stack of 0x00012345 x3
    Then the player caps total is 120
