Feature: Static physics readiness before dynamic props
  # A swapped-in cell must finish structural collision before any dynamic
  # placement receives a gravity-enabled body.

  Scenario: Static collision work is partitioned before dynamic bodies
    Given collider placements have kinds "static,dynamic,static,dynamic"
    When collider placement work is partitioned
    Then static collider indices are "0,2"
    And dynamic collider indices are "1,3"
    And static collision is required before dynamic bodies
