Feature: Prepared AI package catalog
  # `vsa::prepare::package_catalog` is pure (issue #175, M4 wave 11 lane C):
  # given plain decoded PACK inputs it stages a revisioned, FormID-sorted
  # catalog and diagnoses unsupported package types, genuinely unsupported
  # subrecords, and unresolved (Object-ID-typed) location/target FormIDs.
  # Per-actor priority ordering (including template inheritance) is
  # `vsa::prepare::actor_catalog`'s own seam -- see the first scenario below,
  # which reuses that feature's step vocabulary. Byte-level PACK decoding is
  # covered by tests/actor_support.rs (M4 wave 1 task B). Conditions are
  # carried as opaque data only -- no evaluator exists yet (#115/#15).
  #
  # M4 wave 11 follow-up (real-data acceptance on Fallout3.esm cell
  # 0001a273 found the original diagnostics were 100% noise): known FO3
  # script/idle/topic subrecords are "deferred" (aggregate count, no
  # per-package diagnostic), and a location/target FormID whose PLDT/PTDT
  # type means it can point anywhere in the game ("Near Reference"/"In
  # Cell"/"Specific Reference") is "out of scope" (also aggregate-only) --
  # only an "Object ID"-typed location/target missing from the decoded
  # load order is genuinely "unresolved". See package_catalog.rs's module
  # doc comment for the measured before/after numbers.

  Scenario: Package priority order is preserved through template inheritance
    Given an NPC_ actor 0x00000010 with race 0x000000AA
    And actor 0x00000010 has package 0x00000005
    And actor 0x00000010 has package 0x00000001
    And actor 0x00000010 has package 0x00000003
    And an NPC_ actor 0x00000020 with race 0x000000BB
    And actor 0x00000020 has template 0x00000010 using ai_packages
    And a placement 0x00000001 of base 0x00000020 as Npc
    When the actor catalog is built
    Then blueprint for reference 0x00000001 has packages "00000005,00000001,00000003" in order

  Scenario: A package's decoded data is staged into the catalog
    Given a package 0x00000010 with type 5
    And package 0x00000010 has schedule month -1 day -1 date 0 time 8 duration 3600
    When the package catalog is built
    Then the package catalog has 1 package
    And package 0x00000010 has no diagnostics

  Scenario: An unsupported package type is diagnosed
    Given a package 0x00000010 with type 200
    When the package catalog is built
    Then package 0x00000010 has diagnostic containing "unsupported package type 200"
    And the package catalog counts unsupported_type 1 unsupported_subrecord 0 deferred_subrecord 0 unresolved_location 0 unresolved_target 0 out_of_scope_location 0 out_of_scope_target 0

  Scenario: A genuinely unsupported subrecord is diagnosed
    Given a package 0x00000010 with type 0
    And package 0x00000010 has unsupported subrecord "XNAM"
    When the package catalog is built
    Then package 0x00000010 has diagnostic containing "unsupported subrecord(s): XNAM"
    And the package catalog counts unsupported_type 0 unsupported_subrecord 1 deferred_subrecord 0 unresolved_location 0 unresolved_target 0 out_of_scope_location 0 out_of_scope_target 0

  Scenario: A known FO3 script/idle subrecord is deferred, not unsupported
    Given a package 0x00000010 with type 0
    And package 0x00000010 has unsupported subrecord "SCHR"
    When the package catalog is built
    Then package 0x00000010 has no diagnostics
    And the package catalog counts unsupported_type 0 unsupported_subrecord 0 deferred_subrecord 1 unresolved_location 0 unresolved_target 0 out_of_scope_location 0 out_of_scope_target 0

  Scenario: An unresolved Object-ID location FormID is diagnosed
    Given a package 0x00000010 with type 0
    And package 0x00000010 has location type 4 target 0x0000DEAD radius 0
    When the package catalog is built
    Then package 0x00000010 has diagnostic containing "location references unresolved FormID 0000dead"
    And the package catalog counts unsupported_type 0 unsupported_subrecord 0 deferred_subrecord 0 unresolved_location 1 unresolved_target 0 out_of_scope_location 0 out_of_scope_target 0

  Scenario: An unresolved Object-ID target FormID is diagnosed
    Given a package 0x00000010 with type 0
    And package 0x00000010 has target type 1 target 0x0000BEEF count 1
    When the package catalog is built
    Then package 0x00000010 has diagnostic containing "target references unresolved FormID 0000beef"
    And the package catalog counts unsupported_type 0 unsupported_subrecord 0 deferred_subrecord 0 unresolved_location 0 unresolved_target 1 out_of_scope_location 0 out_of_scope_target 0

  Scenario: A Near Reference location FormID is out of scope, not unresolved
    Given a package 0x00000010 with type 0
    And package 0x00000010 has location type 0 target 0x0000DEAD radius 0
    When the package catalog is built
    Then package 0x00000010 has no diagnostics
    And the package catalog counts unsupported_type 0 unsupported_subrecord 0 deferred_subrecord 0 unresolved_location 0 unresolved_target 0 out_of_scope_location 1 out_of_scope_target 0

  Scenario: A Specific Reference target FormID is out of scope, not unresolved
    Given a package 0x00000010 with type 0
    And package 0x00000010 has target type 0 target 0x0000BEEF count 1
    When the package catalog is built
    Then package 0x00000010 has no diagnostics
    And the package catalog counts unsupported_type 0 unsupported_subrecord 0 deferred_subrecord 0 unresolved_location 0 unresolved_target 0 out_of_scope_location 0 out_of_scope_target 1

  Scenario: A resolved Object-ID location and target FormID are not flagged
    Given known package FormIDs "00000020"
    And a package 0x00000010 with type 0
    And package 0x00000010 has location type 4 target 0x00000020 radius 5
    And package 0x00000010 has target type 1 target 0x00000020 count 1
    When the package catalog is built
    Then package 0x00000010 has no diagnostics
