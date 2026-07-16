Feature: Equipment slots, weapon/ammo pairing, and equip-locked transfers
  # Pure `player::equipment` seam (issue #98): the Fallout 3 biped-slot model
  # decoded from `ARMO.BMDT`, equip/unequip keyed by `StackKey`, slot
  # conflicts evicting the previous occupant, weapon+ammo pairing keyed by
  # `WEAP.NAM0`, condition-aware equipped identity, and the cannot-drop/
  # transfer-while-equipped query the drop/transfer paths call.

  Scenario: Equipping apparel claims every biped slot in its BMDT mask
    When apparel 0x00001001 condition 100 mask 0x00000005 is equipped
    Then apparel 0x00001001 condition 100 is equipped

  Scenario: Equipping into an occupied slot evicts the previous occupant
    Given apparel 0x00001001 condition 100 mask 0x00000001 is equipped
    When apparel 0x00002002 condition 100 mask 0x00000001 is equipped
    Then apparel 0x00001001 condition 100 is evicted
    And apparel 0x00002002 condition 100 is equipped

  Scenario: Apparel with no biped slot mask cannot be equipped
    When apparel 0x00005005 condition none mask 0x00000000 is equipped
    Then the equip attempt is rejected as not equippable

  Scenario: Re-equipping a different condition of the same base item is a distinct identity
    Given apparel 0x00001001 condition 100 mask 0x00000001 is equipped
    When apparel 0x00001001 condition 50 mask 0x00000001 is equipped
    Then apparel 0x00001001 condition 100 is evicted
    And apparel 0x00001001 condition 50 is equipped

  Scenario: Ammo matching the equipped weapon's type can be equipped
    Given weapon 0x00003003 condition 100 requiring ammo 0x0000000a is equipped
    When ammo 0x0000000a condition none is equipped
    Then ammo 0x0000000a condition none is equipped

  Scenario: Ammo not matching the equipped weapon's type is rejected
    Given weapon 0x00003003 condition 100 requiring ammo 0x0000000a is equipped
    When ammo 0x0000000b condition none is equipped
    Then the equip attempt is rejected as incompatible ammo

  Scenario: Equipping ammo with no weapon equipped is rejected
    When ammo 0x0000000a condition none is equipped
    Then the equip attempt is rejected with no weapon equipped

  Scenario: Equipping a weapon with a different ammo type unequips incompatible ammo
    Given weapon 0x00003003 condition 100 requiring ammo 0x0000000a is equipped
    And ammo 0x0000000a condition none is equipped
    When weapon 0x00004004 condition 100 requiring ammo 0x0000000b is equipped
    Then ammo 0x0000000a condition none is evicted

  Scenario: An equipped item cannot be dropped
    Given apparel 0x00001001 condition 100 mask 0x00000001 is equipped
    Then dropping apparel 0x00001001 condition 100 is refused while equipped

  Scenario: An unequipped item can be dropped
    Then dropping apparel 0x00009009 condition none is allowed
