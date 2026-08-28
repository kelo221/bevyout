Feature: Shared RPG inspection, Pip-Boy projections, and save v9 freeze (M9 wave 10)
  Pip-Boy, console, and BRP format one RpgInspectionSnapshot. Derived
  HP, radiation stages, cripple flags, and calendar fields are never
  recalculated in presentation. V.A.T.S. stays unavailable until wave 8.
  Save v9 is frozen at RPGS revision 1.

  Scenario: Default sheet projects GOTY vitals without current AP
    Given a default RPG inspection sheet
    When the RPG inspection snapshot is built
    Then the inspection HP is 200 / 200
    And the inspection AP max is 75 and current is unavailable
    And the inspection XP is 0 / 200
    And VATS inspection is unavailable for planned wave 8
    And the inspection calendar is 2277-10-23 00:00

  Scenario: Radiation stage and effects come from the snapshot
    Given a default RPG inspection sheet
    And inspection radiation is 200 rads
    And an inspection chem effect on strength remaining 2000 ms
    When the RPG inspection snapshot is built
    Then the inspection radiation stage is 200
    And the inspection effects list strength then remaining 2000 ms
    And Pip-Boy radiation text is "RADS  200  STAGE 200"

  Scenario: Limb order and cripple flags are frozen in the snapshot
    Given a default RPG inspection sheet
    And inspection left leg is crippled
    When the RPG inspection snapshot is built
    Then the inspection limbs are head, torso, left_arm, right_arm, left_leg, right_leg
    And inspection left leg is marked crippled

  Scenario: Integer world clock is formatted from the snapshot
    Given a default RPG inspection sheet
    And inspection game time is 3600000 ms
    When the RPG inspection snapshot is built
    Then Pip-Boy world clock text is "GAME TIME  2277-10-23 01:00  3600000 ms"
