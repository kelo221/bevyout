Feature: Global prepared object storage
  # Wave 1 of docs/plans/CompressionPlan.md separates recipe identity from
  # final payload identity. The feature seam pins canonical source identities
  # before filesystem publication and migration depend on them.

  Scenario: Fallout source paths normalize independently of host separators
    Given prepared source path "Data\\Meshes\\Architecture\\Megaton\\Wall.NIF"
    When the prepared source path is normalized
    Then the normalized prepared source path is "meshes/architecture/megaton/wall.nif"

  Scenario: Traversal cannot become a prepared source identity
    Given prepared source path "Data/Textures/../Secrets.dds"
    When the prepared source path is normalized
    Then prepared source path normalization is rejected

  Scenario: Recipe identity includes converter and format policy revisions
    Given a prepared GLB recipe for "Data\\Meshes\\Clutter\\Chair.NIF"
    When its prepared recipe identities are calculated
    Then the unchanged prepared recipe identity is stable
    And changing the converter revision changes the prepared recipe identity
    And changing the format policy changes the prepared recipe identity
