Feature: Stealth evidence, ownership, crime, and Karma (M9 wave 6)
  Quantized detection updates the existing awareness authority. Ownership
  policy classifies take versus steal. One crime report yields one bounty
  and Karma mutation. Hidden/Caution/Danger is a HUD projection.

  Scenario: Light alone can acquire a target
    Given a quantized detection observer
    And detection evidence distance 5000 mm angle 0 light 10000 movement 0 armor 0 perception 0 with line of sight
    When quantized detection advances 400 ms
    Then the observer has not acquired a target
    When quantized detection advances 400 ms
    Then the observer has acquired the player

  Scenario: Darkness alone never acquires
    Given a quantized detection observer
    And detection evidence distance 5000 mm angle 0 light 0 movement 0 armor 0 perception 0 with line of sight
    When quantized detection advances 400 ms
    And quantized detection advances 400 ms
    Then the observer has not acquired a target

  Scenario: Movement noise alone can acquire a target
    Given a quantized detection observer
    And detection evidence distance 5000 mm angle 0 light 0 movement 10000 armor 0 perception 0 with line of sight
    When quantized detection advances 400 ms
    And quantized detection advances 400 ms
    Then the observer has acquired the player

  Scenario: Armor noise alone can acquire a target
    Given a quantized detection observer
    And detection evidence distance 5000 mm angle 0 light 0 movement 0 armor 10000 perception 0 with line of sight
    When quantized detection advances 400 ms
    And quantized detection advances 400 ms
    Then the observer has acquired the player

  Scenario: Observer Perception alone can acquire a target
    Given a quantized detection observer
    And detection evidence distance 5000 mm angle 0 light 0 movement 0 armor 0 perception 10 with line of sight
    When quantized detection advances 400 ms
    And quantized detection advances 400 ms
    Then the observer has acquired the player

  Scenario: Occluded evidence is not acquired
    Given a quantized detection observer
    And detection evidence distance 5000 mm angle 0 light 10000 movement 0 armor 0 perception 0 without line of sight
    When quantized detection advances 400 ms
    And quantized detection advances 400 ms
    Then the observer has not acquired a target

  Scenario: Out of cone evidence is not acquired
    Given a quantized detection observer
    And detection evidence distance 5000 mm angle 180000 light 10000 movement 0 armor 0 perception 0 with line of sight
    When quantized detection advances 400 ms
    And quantized detection advances 400 ms
    Then the observer has not acquired a target

  Scenario: Out of range evidence is not acquired
    Given a quantized detection observer
    And detection evidence distance 1000000 mm angle 0 light 10000 movement 0 armor 0 perception 0 with line of sight
    When quantized detection advances 400 ms
    And quantized detection advances 400 ms
    Then the observer has not acquired a target

  Scenario: Score oscillation around the acquire threshold does not flicker
    Given a quantized detection observer with confidence 400 milli
    And detection evidence distance 5000 mm angle 0 light 10000 movement 0 armor 0 perception 0 with line of sight
    When quantized detection advances 100 ms
    Then the observer has not acquired a target
    And the detection confidence milli is 500
    Given detection evidence distance 5000 mm angle 0 light 0 movement 0 armor 0 perception 0 with line of sight
    When quantized detection advances 100 ms
    Then the observer has not acquired a target
    And the detection confidence milli is 400

  Scenario: Equivalent evidence in different order selects the same target
    Given a quantized detection observer
    And two equally distant evidence subjects 0x0000000A then 0x00000005
    When quantized detection advances 400 ms
    And quantized detection advances 400 ms
    Then the observer has acquired actor 0x00000005

  Scenario: Equidistant player beats another actor
    Given a quantized detection observer
    And equidistant player and actor 0x00000050 evidence
    When quantized detection advances 400 ms
    And quantized detection advances 400 ms
    Then the observer has acquired the player

  Scenario: Non-finite geometry is rejected
    Given a quantized detection observer
    When non-finite distance is quantized
    Then detection quantization is rejected

  Scenario: Legacy awareness confidence migrates into milli units
    Given legacy awareness confidence 0.6
    Then migrated confidence milli is 600

  Scenario: Faction membership at required rank is not theft
    Given the player is rank 1 in faction 0x00022457
    And faction 0x00022457 is known
    When taking a faction-owned reference 0x00022457 rank 1 is classified
    Then the take is not theft

  Scenario: Faction membership below required rank is theft
    Given the player is rank 0 in faction 0x00022457
    And faction 0x00022457 is known
    When taking a faction-owned reference 0x00022457 rank 1 is classified
    Then the take is theft from 0x00022457

  Scenario: Non-member taking faction property is theft
    Given faction 0x00022457 is known
    When taking a faction-owned reference 0x00022457 rank 0 is classified
    Then the take is theft from 0x00022457

  Scenario: Unwitnessed theft marks stolen provenance without bounty
    Given a player crime ledger
    And an illegal theft of owner 0x0001A2B3
    When the theft is resolved with no witnesses
    Then the item is marked stolen from 0x0001A2B3
    And no crime was reported
    And the player bounty is 0
    And the player karma is 0

  Scenario: Witnessed theft reports once even with two witnesses
    Given a player crime ledger
    And an illegal theft of owner 0x0001A2B3
    And an eligible witness 0x00041600
    And an eligible witness 0x00041601
    When the theft is resolved with witnesses
    Then a theft crime is reported once
    And the player bounty is 40
    And the player karma is -5
    When the same crime is reported again
    Then the player bounty is 40
    And the player karma is -5

  Scenario: A witness behind a wall is rejected
    Given a player crime ledger
    And an illegal theft of owner 0x0001A2B3
    And an occluded witness 0x00041600
    When the theft is resolved with witnesses
    Then no crime was reported

  Scenario: A witness outside alarm range is rejected
    Given a player crime ledger
    And an illegal theft of owner 0x0001A2B3
    And a distant witness 0x00041600
    When the theft is resolved with witnesses
    Then no crime was reported

  Scenario: A dead, disabled, or victim-hostile witness is rejected
    Given a player crime ledger
    And an illegal theft of owner 0x0001A2B3
    And a dead witness 0x00041600
    And a disabled witness 0x00041601
    And a victim-hostile witness 0x00041602
    When the theft is resolved with witnesses
    Then no crime was reported

  Scenario: Assault escalating to murder does not double-charge assault
    Given a player crime ledger
    And an unreported assault crime
    When the assault is escalated to murder with an eligible witness 0x00041600
    Then a murder crime is reported once
    And the player bounty is 1000
    And the player karma is -100

  Scenario: No aware hostile observer projects Hidden
    Given no detection observers
    When the detection HUD is projected
    Then the detection HUD is Hidden

  Scenario: A suspicious hostile observer projects Caution
    Given a hostile observer with confidence 400 milli and no acquisition
    When the detection HUD is projected
    Then the detection HUD is Caution

  Scenario: A hostile acquisition projects Danger
    Given a hostile observer that has acquired the player
    When the detection HUD is projected
    Then the detection HUD is Danger

  Scenario: Awareness and crime state round-trip through serde
    Given a player crime ledger with bounty 40 karma -5 and sequence 3
    And a quantized detection observer that has acquired the player at 800 milli
    Then awareness and crime serde round-trip
