Feature: AI package location and target resolution
  # `viewer::ai::resolution` (issue #195) resolves a selected package's PLDT
  # location and PTDT target into a concrete world position (and entity, when
  # known) from a plain runtime snapshot -- exactly what the prepared catalog
  # deferred as "out of scope". Every unresolvable case yields a deterministic
  # diagnostic, never a panic and never a silent (0,0,0).

  Scenario: A near-reference location resolves to that reference's position
    Given a resolvable reference 0x00000020 of base 0x000000AA at 3.0 4.0 5.0
    And a package location of type 0 referencing 0x00000020 radius 7
    When the package location is resolved
    Then the location resolves to 3.0 4.0 5.0
    And the location resolves via "reference"

  Scenario: A missing reference is a diagnostic, not a panic
    Given a package location of type 0 referencing 0x0000DEAD radius 0
    When the package location is resolved
    Then the location is unresolved with diagnostic containing "0000dead"

  Scenario: An object-id location picks the nearest instance of the base
    Given the resolving actor is at 0.0 0.0 0.0
    And a resolvable reference 0x00000020 of base 0x000000AA at 10.0 0.0 0.0
    And a resolvable reference 0x00000021 of base 0x000000AA at 2.0 0.0 0.0
    And a package location of type 4 referencing 0x000000AA radius 0
    When the package location is resolved
    Then the location resolves to 2.0 0.0 0.0
    And the location resolves via "nearest-of-base"

  Scenario: An object-type location is deterministically unresolvable
    Given a package location of type 5 referencing 0x00000000 radius 0
    When the package location is resolved
    Then the location is unresolved with diagnostic containing "form-type index"

  Scenario: A specific-reference target resolves with its distance
    Given a resolvable reference 0x00000040 of base 0x000000CC at 1.0 1.0 1.0
    And a package target of type 0 referencing 0x00000040 distance 5
    When the package target is resolved
    Then the target resolves to 1.0 1.0 1.0
    And the target radius is 5.0

  Scenario: An object-id target picks the nearest instance of the base
    Given the resolving actor is at 0.0 0.0 0.0
    And a resolvable reference 0x00000050 of base 0x000000DD at 8.0 0.0 0.0
    And a resolvable reference 0x00000051 of base 0x000000DD at 1.0 0.0 0.0
    And a package target of type 1 referencing 0x000000DD distance 0
    When the package target is resolved
    Then the target resolves to 1.0 0.0 0.0

  Scenario: An unsupported target type is diagnosed
    Given a package target of type 77 referencing 0x00000000 distance 0
    When the package target is resolved
    Then the target is unresolved with diagnostic containing "unsupported target type 77"
