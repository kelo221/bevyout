Feature: Batch cell selectors for prepare
  # Pins the pure selection-resolution seam behind `prepare --all` /
  # `--all-interiors` / `--worldspace` / multiple positional selectors
  # (src/vsa/prepare/selectors.rs::resolve_selection). Drives a synthetic
  # catalogue only -- no game data, no Blender.

  Scenario: --all-interiors yields exactly the interior subset, sorted
    Given cell 0x00000005 "ExtB" is an exterior cell
    And cell 0x00000001 "IntA" is an interior cell
    And cell 0x00000003 "IntC" is an interior cell
    When cells are selected with --all-interiors
    Then the resolved cell selection is "00000001, 00000003"

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
