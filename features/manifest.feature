Feature: Schema-7 prepared scene manifest
  # Pins the manifest promises from README.md's "Prepare and render a cell"
  # section: "The schema-7 manifest also retains item/container metadata,
  # ownership and enable-parent state, door locks and destinations, cell
  # acoustic/music metadata, native footstep and landing sound banks, and
  # source NAVM payloads (NAVM is retained metadata, not runtime
  # navigation)." Verified against a checked-in golden fixture
  # (features/fixtures/scene.ron) parsed with the real
  # `PreparedSceneManifest` type from src/vsa/manifest.rs, so schema drift
  # (a renamed/removed/retyped field) fails this test loudly instead of
  # silently.
  #
  # Intended change path when the schema legitimately changes: bump
  # `PreparedSceneManifest::schema_version` in src/vsa/manifest.rs and
  # regenerate/update features/fixtures/scene.ron (and this feature file if
  # the promised fields themselves changed) in the same change.

  Scenario: The golden fixture parses as a schema-7 manifest
    Given the golden manifest fixture "features/fixtures/scene.ron"
    Then it parses as a PreparedSceneManifest
    And the schema version is 7

  Scenario: Retained container inventory and ownership/enable-parent metadata
    Given the golden manifest fixture "features/fixtures/scene.ron"
    Then the placement "SuperMartCooler01" is a Container
    And the placement "SuperMartCooler01" has 1 inventory entry
    And the placement "SuperMartCooler01" inventory includes "PurifiedWater"
    And the placement "SuperMartCooler01" has an owner faction rank of -1
    And the placement "SuperMartCooler01" has an enable parent that pops in

  Scenario: Retained door lock level and destination
    Given the golden manifest fixture "features/fixtures/scene.ron"
    Then the placement "SuperMartFrontDoor" is a Door
    And the placement "SuperMartFrontDoor" has a lock level of 50
    And the placement "SuperMartFrontDoor" destination cell is 0x00020901

  Scenario: Retained cell acoustics, footstep banks, and NAVM payload metadata
    Given the golden manifest fixture "features/fixtures/scene.ron"
    Then the cell acoustic environment type is 2
    And the footstep set "concrete" has a land clip
    And there is 1 hard landing clip
    And there is a retained NAVM payload with signature "NVNM"
