Feature: Bounded native NIF conversion batches

  Scenario: Native conversion is the default and Blender remains an explicit option
    Given no NIF converter was explicitly requested
    When the preparation converter is resolved
    Then the resolved preparation converter is "native"
    When the "blender" NIF converter is explicitly requested
    Then the resolved preparation converter is "blender"

  Scenario: Native unlit materials are accepted by the Rust bake
    Given a native GLB requires extensions "KHR_materials_unlit"
    When the Rust bake validates its required glTF extensions
    Then no required glTF extensions are unsupported

  Scenario: Mixed native conversion results have a deterministic summary
    Given native conversion outcomes "2:failed,0:converted,1:unsupported"
    When the native conversion batch is summarized
    Then the native conversion summary is "native conversion: completed 3/3 ok=1 failed=1 unsupported=1"
    And the native conversion outcome order is "0,1,2"

  Scenario: Native worker count is bounded by the number of assets
    Given 3 native conversion assets and 8 requested workers
    When the native worker count is resolved
    Then 3 native conversion workers are used

  Scenario: Native worker count never becomes zero
    Given 0 native conversion assets and 0 requested workers
    When the native worker count is resolved
    Then 1 native conversion workers are used

  Scenario: Native worker count defaults to every host processor
    Given 24 native conversion assets, no requested workers, and 24 host processors
    When the native worker count is resolved
    Then 24 native conversion workers are used
