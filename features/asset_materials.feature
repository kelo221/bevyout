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
  # Scope note: (1) is decided entirely inside the embedded Blender Python
  # conversion script (the NIFTOOLS_COMPAT_PY constant in src/vsa/assets.rs,
  # around `alpha_blend`/`alpha_test`/the normal-alpha-to-specular link) --
  # there is no pure Rust function for it, only Blender-side node-graph
  # wiring that this hermetic, no-Blender test suite cannot exercise. So
  # this feature pins what IS decided in Rust instead: which conversion
  # profile a mesh gets (src/vsa/assets.rs `asset_conversion`/
  # `AssetConversion::profile_tag`), which is the seam that actually chooses
  # between the AO bake path and the "preserve authored materials" path
  # promise (2) above.

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
