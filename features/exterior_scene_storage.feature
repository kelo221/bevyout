Feature: Compact prepared exterior scene roots

  Scenario: Exterior packages remain canonical worldspace artifacts
    Given exterior cell 00000cb8 belongs to worldspace 0000003c
    When its exterior scene storage is planned
    Then the exterior package path is "worldspaces/0000003c/cells/00000cb8.ron"
    And the scene root does not embed the exterior package
    And the scene root does not embed content-wide diagnostics
