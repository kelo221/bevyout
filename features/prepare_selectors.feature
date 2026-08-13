Feature: Batch cell selectors for prepare
  # Pins the pure selection-resolution seam behind `prepare --all` /
  # `--all-interiors` / `--all-exteriors` / `--worldspace` /
  # `--exterior-radius` / multiple positional selectors
  # (src/vsa/prepare/selectors.rs::resolve_selection). Drives a synthetic
  # catalogue only -- no game data, no Blender.

  Scenario: --all-interiors yields exactly the interior subset, sorted
    Given cell 0x00000005 "ExtB" is an exterior cell
    And cell 0x00000001 "IntA" is an interior cell
    And cell 0x00000003 "IntC" is an interior cell
    When cells are selected with --all-interiors
    Then the resolved cell selection is "00000001, 00000003"

  Scenario: --all-exteriors yields exactly the exterior subset, sorted and deduplicated
    Given cell 0x00000005 "ExtB" is an exterior cell
    And cell 0x00000001 "IntA" is an interior cell
    And cell 0x00000003 "ExtC" is an exterior cell
    And cell 0x00000003 "ExtCOverride" is an exterior cell
    When cells are selected with --all-exteriors
    Then the resolved cell selection is "00000003, 00000005"

  Scenario: An explicit list mixing EditorID and FormID resolves, dedupes, and sorts
    Given cell 0x00000010 "VaultDoor" is an interior cell
    And cell 0x00000002 "Wasteland" is an exterior cell
    When cells are selected with selectors "Wasteland, 00000010, wasteland"
    Then the resolved cell selection is "00000002, 00000010"

  Scenario: An unknown worldspace names the available worldspaces
    Given cell 0x00000001 "Cell1" is an exterior cell in worldspace 0x00000100
    And worldspace 0x00000100 is named "Capital Wasteland"
    When cells are selected with worldspace "Nowhere"
    Then the cell selection fails naming worldspace "Capital Wasteland"

  Scenario: --exterior-radius selects a same-worldspace square around one anchor
    Given cell 0x00000005 "Center" is an exterior cell in worldspace 0x00000100 at grid (10,-4)
    And cell 0x00000001 "West" is an exterior cell in worldspace 0x00000100 at grid (9,-4)
    And cell 0x00000003 "Diagonal" is an exterior cell in worldspace 0x00000100 at grid (11,-3)
    And cell 0x00000002 "Outside" is an exterior cell in worldspace 0x00000100 at grid (12,-4)
    And cell 0x00000004 "OtherWorld" is an exterior cell in worldspace 0x00000200 at grid (10,-4)
    When cells are selected with anchor "Center" and --exterior-radius 1
    Then the resolved cell selection is "00000001, 00000003, 00000005"
