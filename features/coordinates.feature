Feature: Fallout to Bevy coordinate conversion
  # Pins the informal spec in README.md's "Prepare and render a cell" section:
  # roughly 70 Fallout world units per Bevy metre, and the axis remap from
  # Fallout's Z-up world into Bevy's Y-up world. Drives the real functions in
  # src/vsa/paths.rs (placement_transform_parts, parse_cell_selector,
  # normalize_asset_path) -- no game data, no Blender.

  Scenario: Fallout uses roughly 70 world units per metre
    Given a Fallout position of 70, 0, 0 world units
    When it is converted to a Bevy placement transform
    Then the Bevy translation is 1, 0, 0 metres

  Scenario: Fallout Z-up axes remap onto Bevy's Y-up axes
    Given a Fallout position of 70, 140, 210 world units
    When it is converted to a Bevy placement transform
    Then the Bevy translation is 1, 3, -2 metres

  Scenario: Identity rotation and uniform scale pass through unchanged
    Given a Fallout position of 0, 0, 0 world units
    And a Fallout rotation of 0, 0, 0 radians
    And a Fallout scale of 1.5
    When it is converted to a Bevy placement transform
    Then the Bevy rotation is the identity quaternion
    And the Bevy scale is 1.5

  Scenario: The metre conversion round-trips back to the original world units
    Given a Fallout position of 350, -420, 700 world units
    When it is converted to a Bevy placement transform
    And the Bevy translation is converted back to Fallout world units
    Then the recovered position matches the original Fallout position

  Scenario: A cell selector recognizes hexadecimal FormIDs
    # README: "Prepared scenes continue to use their hexadecimal FormID
    # directory internally; for example, SuperDuperMart resolves to
    # 00017f37." This scenario pins selector *parsing*, not the ESM lookup
    # that performs that resolution (which needs real game data).
    Given the cell selector text "00017f37"
    Then it parses as the hexadecimal FormID 0x00017f37

  Scenario: A cell selector recognizes GECK EditorIDs
    Given the cell selector text "SuperDuperMart"
    Then it parses as the editor ID "SuperDuperMart"

  Scenario: Game asset paths are normalized regardless of source casing or separators
    Given the raw asset path "\Textures\Foo\BAR.DDS"
    Then the normalized asset path is "textures/foo/bar.dds"
