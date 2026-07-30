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

  Scenario Outline: NIF glossiness becomes perceptual GGX roughness
    Given a NIF material glossiness exponent <glossiness>
    When its PBR material policy is evaluated
    Then its perceptual roughness is approximately <roughness>

    Examples:
      | glossiness | roughness |
      | 0          | 1.000000  |
      | 10         | 1.000000  |
      | 40         | 0.817492  |
      | 70         | 0.714435  |
      | 100        | 0.654856  |

  Scenario Outline: Missing or invalid glossiness uses exponent ten
    Given a NIF material glossiness value "<glossiness>"
    When its PBR material policy is evaluated
    Then its perceptual roughness is approximately 1.000000

    Examples:
      | glossiness |
      | missing    |
      | negative   |
      | nan        |
      | infinite   |

  Scenario: Exact normalized diffuse paths select binary metalness
    Given metallic material CSV "diffuse_texture,object_name,metallic\ntextures/weapons/test.dds,Test Weapon,1\n"
    And a material diffuse texture "Data\\Textures\\Weapons\\TEST.DDS"
    When its PBR material policy is evaluated
    Then its metallic factor is 1

  Scenario: Unlisted diffuse paths remain dielectric
    Given metallic material CSV "diffuse_texture,object_name,metallic\ntextures/weapons/test.dds,Test Weapon,1\n"
    And a material diffuse texture "textures/weapons/other.dds"
    When its PBR material policy is evaluated
    Then its metallic factor is 0

  Scenario Outline: Invalid metallic CSV is rejected
    Given metallic material CSV "<csv>"
    When the metallic material CSV is parsed
    Then the metallic material CSV is rejected

    Examples:
      | csv                                                                                                                       |
      | diffuse_texture,object_name,metallic\ntextures/weapons/test.dds,Test Weapon,0.5\n                                      |
      | diffuse_texture,object_name,metallic\ntextures/weapons/test.dds,Test Weapon,1\nTEXTURES\\WEAPONS\\TEST.DDS,Duplicate Weapon,0\n |
      | diffuse_texture,object_name,metallic\ntextures/weapons/test.dds,,1\n                                                |

  Scenario: DirectX normal Y is converted without changing specular alpha
    Given a DirectX normal texel (12, 34, 56, 78)
    When its normal convention is converted for Bevy
    Then the converted normal texel is (12, 221, 56, 78)

  Scenario: Specular-enabled Fallout materials reuse normal alpha as specular strength
    Given a Fallout material has specular enabled
    And its normal texture is "textures/furniture/chair03_n.dds"
    When its PBR material policy is evaluated
    Then its specular texture is "textures/furniture/chair03_n.dds"

  Scenario: Specular-disabled Fallout materials export no specular texture
    Given a Fallout material has specular disabled
    And its normal texture is "textures/furniture/chair03_n.dds"
    When its PBR material policy is evaluated
    Then it has no specular texture

  Scenario: Fallout materials without a normal map export no specular texture
    Given a Fallout material has specular enabled
    When its PBR material policy is evaluated
    Then it has no specular texture

  Scenario Outline: Blender staging recognizes only normal-map filenames
    Given the staged texture path "<path>"
    When its Blender texture role is classified
    Then it <classification> converted as a normal map

    Examples:
      | path                                      | classification |
      | textures/architecture/Wall_N.DDS          | is             |
      | textures/characters/face_normal.dds       | is             |
      | textures/architecture/wall.dds            | is not         |
      | textures/effects/terminal_g.dds            | is not         |
