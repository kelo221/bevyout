Feature: Container state persists through the save layer
  # Pins issue #76's pure container capture/apply policy:
  # `world::persist_policy::{diff_capture_containers, plan_apply_containers}`.
  # A container's stacks and its resolved-leveled marker only produce a save
  # delta when they differ from the manifest baseline (an untouched container
  # produces no delta at all); a delta with no inventory override still
  # carries the resolved marker on its own (an "LVLR-only" delta), and a
  # container with no delta stays unresolved so it still rolls its leveled
  # entries on first open. Drives `persist_policy` directly, the same way
  # features/state_persistence.feature drives its placement-shaped capture
  # and apply.

  Scenario: A baseline-identical, unresolved container captures no delta
    Given a container baseline 0x00000700 with stack 0x00000010 x 3
    And a container snapshot 0x00000700 with stack 0x00000010 x 3
    When container state is captured
    Then no container delta is captured for 0x00000700

  Scenario: A looted container captures a minimal inventory-only delta
    Given a container baseline 0x00000701 with stack 0x00000010 x 3
    And a container snapshot 0x00000701 with stack 0x00000010 x 1
    When container state is captured
    Then the captured container delta for 0x00000701 has stack 0x00000010 x 1
    And the captured container delta for 0x00000701 has no resolved marker

  Scenario: A resolved container with baseline-identical stacks captures an LVLR-only delta
    Given a container baseline 0x00000702 with stack 0x00000010 x 3
    And a resolved container snapshot 0x00000702 with stack 0x00000010 x 3
    When container state is captured
    Then the captured container delta for 0x00000702 has no inventory override
    And the captured container delta for 0x00000702 is resolved

  Scenario: Applying a container delta seeds its runtime stacks and resolved marker
    Given a container baseline 0x00000800 with stack 0x00000010 x 3
    And a container delta 0x00000800 with stack 0x00000010 x 1 and resolved
    When the container state is applied
    Then the seeded container 0x00000800 has stack 0x00000010 x 1
    And the seeded container 0x00000800 is resolved

  Scenario: An LVLR-only delta seeds the baseline stacks with the resolved marker
    Given a container baseline 0x00000801 with stack 0x00000010 x 3
    And a resolved-only container delta 0x00000801
    When the container state is applied
    Then the seeded container 0x00000801 has stack 0x00000010 x 3
    And the seeded container 0x00000801 is resolved

  Scenario: A container with no delta is not seeded and stays unresolved
    Given a container baseline 0x00000802 with stack 0x00000010 x 3
    When the container state is applied
    Then no container is seeded for 0x00000802
