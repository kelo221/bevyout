Feature: Per-cell collider ownership (issue #63)
  Collider construction records what each cell's build creates, so leaving
  or evicting a cell can tear down exactly that set — no duplicated shapes
  on revisit, no departed cell's colliders lingering for the session.

  Scenario: Releasing a cell returns exactly what its build recorded
    Given cell 0x00000100 recorded 3 static shapes, 1 keyframed body, and 2 dynamic bodies
    When cell 0x00000100 is released
    Then the released set has 3 static shapes and 3 bodies
    And cell 0x00000100 is no longer tracked

  Scenario: Releasing an untracked cell yields nothing
    When cell 0x00000999 is released
    Then nothing is released

  Scenario: Two cells' recordings are independent
    Given cell 0x00000100 recorded 2 static shapes, 0 keyframed bodies, and 0 dynamic bodies
    And cell 0x00000200 recorded 1 static shapes, 0 keyframed bodies, and 1 dynamic bodies
    When cell 0x00000100 is released
    Then the released set has 2 static shapes and 0 bodies
    And cell 0x00000200 is still tracked

  Scenario: A rebuild after release starts a fresh recording
    Given cell 0x00000100 recorded 2 static shapes, 0 keyframed bodies, and 0 dynamic bodies
    When cell 0x00000100 is released
    And cell 0x00000100 records 1 more static shape
    And cell 0x00000100 is released
    Then the released set has 1 static shapes and 0 bodies
