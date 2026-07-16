Feature: Asset conversion profile selection
  # README.md promises two material-adjacent behaviours per prepared asset:
  #  1. "NIF alpha flags and diffuse texture alpha are exported as glTF
  #     MASK/BLEND materials. Fallout normal-map RGB is used for
  #     tangent-space normals and its alpha is exported as
  #     KHR_materials_specular specular strength."
  #  2. "Static meshes receive a fast, mild ao-quick-v1 vertex AO pass during
  #     conversion; dynamic, NPC, creature, weapon, and furniture assets
  #     preserve their authored materials."
  #
  # The material node graph is rebuilt inside the embedded Blender Python
  # converter. The std-only policy seam below pins the authored-emission
  # decision without requiring Blender, while src/vsa/assets/tests/mod.rs
  # also checks that the embedded script reads the NIFTools property and
  # preserves the existing bulb/glow override order.

  Scenario: Static meshes get the quick vertex-AO conversion profile
    Given an asset is static
    When its conversion profile is selected
    Then the conversion is QuickAo
    And the profile tag is "ao-quick-v1"

  Scenario: Dynamic, NPC, creature, weapon, and furniture assets preserve authored materials
    Given an asset is dynamic
    When its conversion profile is selected
    Then the conversion is Preserve
    And the profile tag is "ao-none"

  Scenario: Nonzero NIFTools authored emission exports its source multiplier
    Given an imported material has NIFTools emissive color (0.8, 0.4, 0.1)
    And the source emission multiplier is 2.5
    When its material emission policy is evaluated
    Then the exported emission color is (0.8, 0.4, 0.1)
    And the exported emission strength is 2.5

  Scenario: Zero NIFTools authored emission remains non-emissive
    Given an imported material has NIFTools emissive color (0.0, 0.0, 0.0)
    And the source emission multiplier is 2.5
    When its material emission policy is evaluated
    Then the exported material has no emission

  Scenario: Glow texture is the final emission override
    Given an imported material has NIFTools emissive color (0.8, 0.4, 0.1)
    And the source emission multiplier is 2.5
    And an explicit emission is present
    And an emissive bulb override is present
    And a glow texture override is present
    When its material emission policy is evaluated
    Then the selected emission source is Glow
