Feature: Prepared cache storage accounting
  # Wave 0 of docs/plans/CompressionPlan.md needs deterministic evidence before
  # storage semantics or artifact formats change. These scenarios pin the pure
  # accounting seam consumed by `bevyout cache stats`.

  Scenario: Exact duplicate payloads count once in unique storage
    Given a fresh prepared cache inventory
    And cache file "assets/a.glb" has 100 logical bytes, 128 allocated bytes, and payload "shared-glb"
    And cache file "assets/b.glb" has 100 logical bytes, 128 allocated bytes, and payload "shared-glb"
    And cache file "audio/c.wav" has 40 logical bytes, 64 allocated bytes, and payload "unique-audio"
    When cache storage is summarized
    Then cache logical bytes are 240
    And cache allocated bytes are 320
    And cache unique payload bytes are 140
    And cache duplicate logical bytes are 100
    And cache duplicate physical bytes are 128
    And cache duplicate cluster count is 1

  Scenario: Runtime artifact categories remain distinct
    Given a fresh prepared cache inventory
    And cache file "assets/model.glb" has 10 logical bytes, 16 allocated bytes, and payload "model"
    And cache file "assets/diffuse.ktx2" has 20 logical bytes, 24 allocated bytes, and payload "texture"
    And cache file "assets/terrain/cell.png" has 25 logical bytes, 32 allocated bytes, and payload "texture-source"
    And cache file "scenes/000151e3/shadows/point.ktx2" has 30 logical bytes, 32 allocated bytes, and payload "shadow"
    And cache file "scenes/000151e3/scene.ron" has 40 logical bytes, 48 allocated bytes, and payload "manifest"
    And cache file "scenes/000151e3/actors.ron" has 50 logical bytes, 56 allocated bytes, and payload "catalog"
    And cache file "scenes/000151e3/navgraph.ron" has 60 logical bytes, 64 allocated bytes, and payload "navigation"
    When cache storage is summarized
    Then cache category "glb" has 10 logical bytes
    And cache category "texture" has 20 logical bytes
    And cache category "texture-source" has 25 logical bytes
    And cache category "shadow" has 30 logical bytes
    And cache category "manifest" has 40 logical bytes
    And cache category "catalog" has 50 logical bytes
    And cache category "navigation" has 60 logical bytes

  Scenario: Category and duplicate output order is deterministic
    Given a fresh prepared cache inventory
    And cache file "assets/z.glb" has 12 logical bytes, 16 allocated bytes, and payload "z-copy"
    And cache file "assets/y.glb" has 12 logical bytes, 16 allocated bytes, and payload "z-copy"
    And cache file "audio/b.wav" has 8 logical bytes, 8 allocated bytes, and payload "a-copy"
    And cache file "audio/a.wav" has 8 logical bytes, 8 allocated bytes, and payload "a-copy"
    When cache storage is summarized
    Then cache categories are ordered alphabetically
    And cache duplicate clusters are ordered by recoverable bytes then payload
    And paths inside each duplicate cluster are ordered alphabetically
