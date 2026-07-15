Feature: Rust irradiance volume layout
  The normal bake stays in Bevy coordinates and emits the ambient-cube atlas
  layout consumed by Bevy's irradiance-volume runtime.

  Scenario: Probe resolution preserves Bevy XYZ axis order
    Given a Rust bake volume scale of 94.52831, 31.064644, 87.1143 metres
    And irradiance probe spacing is 8 metres
    When the Rust irradiance layout is planned
    Then the probe resolution is 13, 5, 12

  Scenario: Six ambient-cube faces use Bevy's two-by-three atlas layout
    Given a probe resolution of 13, 5, 12
    When the Rust irradiance atlas is planned
    Then the atlas dimensions are 13, 10, 36

  Scenario: Ray work is deterministic from resolution and sample count
    Given a probe resolution of 13, 5, 12
    And irradiance sample count is 64
    When the Rust irradiance ray count is planned
    Then the primary ray count is 299520
