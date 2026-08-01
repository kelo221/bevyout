Feature: Material clamp policy
  The viewer owns one incremental clamp policy for the StandardMaterial
  metallic gate, dielectric-specular gate, and roughness scale
  (`setrender metallic|dielectric_specular|roughness_scale`). Engaging a
  clamp snapshots the pre-clamp factors and rewrites each material once;
  disengaging restores the snapshots bit-exactly and forgets them. Between
  settings changes only asset events are processed, and removed assets lose
  their baselines immediately.

  Background:
    Given a fresh material clamp policy

  Scenario: The metallic gate clamps every material and restores exactly
    Given clamp material 1 has metallic 0.75, reflectance 0.5, and roughness 0.25
    And clamp material 2 has metallic 0.0, reflectance 0.375, and roughness 0.75
    When the clamp "metallic" gate engages
    Then clamp material 1 has metallic 0.0, reflectance 0.5, and roughness 0.25
    And clamp material 2 has metallic 0.0, reflectance 0.375, and roughness 0.75
    And a steady clamp frame performs no full material pass
    When the clamp "metallic" gate disengages
    Then clamp material 1 has metallic 0.75, reflectance 0.5, and roughness 0.25
    And clamp material 2 has metallic 0.0, reflectance 0.375, and roughness 0.75
    And the clamp store holds no baselines

  Scenario: The dielectric gate leaves metallic factors alone
    Given clamp material 1 has metallic 0.75, reflectance 0.5, and roughness 0.25
    When the clamp "dielectric_specular" gate engages
    Then clamp material 1 has metallic 0.75, reflectance 0.0, and roughness 0.25
    When the clamp "dielectric_specular" gate disengages
    Then clamp material 1 has metallic 0.75, reflectance 0.5, and roughness 0.25
    And the clamp store holds no baselines

  Scenario: Materials loaded while a gate is engaged are clamped from load
    Given clamp material 1 has metallic 0.75, reflectance 0.5, and roughness 0.25
    When the clamp "metallic" gate engages
    And clamp material 7 loads with metallic 0.5, reflectance 0.625, and roughness 0.75
    Then clamp material 7 has metallic 0.0, reflectance 0.625, and roughness 0.75
    When the clamp "metallic" gate disengages
    Then clamp material 7 has metallic 0.5, reflectance 0.625, and roughness 0.75
    And the clamp store holds no baselines

  Scenario: Roughness scaling reuses snapshots across repeated changes
    Given clamp material 1 has metallic 0.0, reflectance 0.5, and roughness 0.5
    And clamp material 2 has metallic 0.0, reflectance 0.5, and roughness 0.75
    When the clamp roughness scale becomes 1.5
    Then clamp material 1 has metallic 0.0, reflectance 0.5, and roughness 0.75
    And clamp material 2 has metallic 0.0, reflectance 0.5, and roughness 1.0
    And a steady clamp frame performs no full material pass
    When the clamp roughness scale becomes 0.5
    Then clamp material 1 has metallic 0.0, reflectance 0.5, and roughness 0.25
    And clamp material 2 has metallic 0.0, reflectance 0.5, and roughness 0.375
    When the clamp roughness scale becomes 1.0
    Then clamp material 1 has metallic 0.0, reflectance 0.5, and roughness 0.5
    And clamp material 2 has metallic 0.0, reflectance 0.5, and roughness 0.75
    And the clamp store holds no baselines

  Scenario: Overlapping clamps restore each factor from its own snapshot
    Given clamp material 1 has metallic 0.75, reflectance 0.5, and roughness 0.5
    When the clamp "metallic" gate engages
    And the clamp roughness scale becomes 0.5
    Then clamp material 1 has metallic 0.0, reflectance 0.5, and roughness 0.25
    And the clamp store keeps 1 baseline entry
    When the clamp "metallic" gate disengages
    Then clamp material 1 has metallic 0.75, reflectance 0.5, and roughness 0.25
    And the clamp store keeps 1 baseline entry
    When the clamp roughness scale becomes 1.0
    Then clamp material 1 has metallic 0.75, reflectance 0.5, and roughness 0.5
    And the clamp store holds no baselines

  Scenario: A removed material drops its baseline immediately
    Given clamp material 1 has metallic 0.75, reflectance 0.5, and roughness 0.25
    And clamp material 2 has metallic 0.5, reflectance 0.5, and roughness 0.25
    When the clamp "metallic" gate engages
    And clamp material 1 is removed from the asset store
    Then the clamp store keeps 1 baseline entry
    When the clamp "metallic" gate disengages
    Then clamp material 2 has metallic 0.5, reflectance 0.5, and roughness 0.25
    And the clamp store holds no baselines

  Scenario: A disengaged policy stays inert
    Given clamp material 1 has metallic 0.75, reflectance 0.5, and roughness 0.25
    When a steady clamp frame runs
    Then clamp material 1 has metallic 0.75, reflectance 0.5, and roughness 0.25
    And the clamp store holds no baselines
    And the steady clamp frame needed no full material pass
