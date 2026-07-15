Feature: State persistence through the save layer
  # Pins issues #60/#61's pure persistence policy:
  # `src/viewer/world/persist_policy.rs`. Effective-enabled resolution walks
  # manifest enable-parent chains (honoring inversion) and lets save deltas
  # override any link in the chain; capture diffing turns a runtime snapshot
  # against the manifest baseline into the minimal delta set (untouched
  # references produce no delta); apply planning turns deltas back into
  # per-placement decisions, closing the round trip. Drives `persist_policy`
  # directly, the same way features/instant_swap.feature drives
  # `world::swap_policy`.

  Scenario: A child of a disabled enable parent is hidden
    Given a placement 0x00000001 that is initially disabled
    And a placement 0x00000002 enable-parented to 0x00000001
    When effective enabled state is resolved
    Then placement 0x00000002 resolves disabled

  Scenario: An inverted enable-parent link flips the parent state
    Given a placement 0x00000001 that is initially disabled
    And a placement 0x00000002 enable-parented to 0x00000001 with the inverted flag
    When effective enabled state is resolved
    Then placement 0x00000002 resolves enabled

  Scenario: A save delta overrides the enable-parent cascade
    Given a placement 0x00000001 that is initially enabled
    And a placement 0x00000002 enable-parented to 0x00000001
    And a save delta disabling reference 0x00000002
    When effective enabled state is resolved
    Then placement 0x00000001 resolves enabled
    And placement 0x00000002 resolves disabled

  Scenario: An enable-root redirect follows the root's effective state
    Given a placement 0x00000001 that is initially disabled
    And a placement 0x00000002 that is initially enabled
    And a save delta pointing reference 0x00000002 at enable root 0x00000001
    When effective enabled state is resolved
    Then placement 0x00000002 resolves disabled

  Scenario: A moved dynamic body captures transform and body deltas
    Given a baseline placement 0x00000100 at [0, 0, 0]
    And a runtime snapshot of 0x00000100 at [1, 0, 0] with linear velocity [0.5, 0, 0]
    When the cell state is captured
    Then the captured delta for 0x00000100 has a transform
    And the captured delta for 0x00000100 has a body state

  Scenario: An opened container captures an activated delta
    Given a baseline placement 0x00000200 at [0, 0, 0]
    And a runtime snapshot of 0x00000200 that is open
    When the cell state is captured
    Then the captured delta for 0x00000200 is activated

  Scenario: A taken pickup captures a deleted delta
    Given a baseline placement 0x00000300 at [0, 0, 0]
    And a runtime snapshot of 0x00000300 that is no longer present
    When the cell state is captured
    Then the captured delta for 0x00000300 is deleted

  Scenario: An untouched reference captures no delta
    Given a baseline placement 0x00000400 at [0, 0, 0]
    And a runtime snapshot of 0x00000400 at [0, 0, 0] at rest
    When the cell state is captured
    Then no delta is captured for 0x00000400

  Scenario: A captured delta re-applied reproduces the runtime snapshot
    Given a baseline placement 0x00000500 at [0, 0, 0]
    And a runtime snapshot of 0x00000500 at [2, 3, 4] with linear velocity [0.1, 0, 0]
    When the cell state is captured
    And the captured state is applied
    Then the applied placement 0x00000500 is visible
    And the applied placement 0x00000500 has translation [2, 3, 4]

  Scenario: A deleted reference is hidden when applied
    Given a placement 0x00000600 that is initially enabled
    And a save delta deleting reference 0x00000600
    When the save state is applied
    Then the applied placement 0x00000600 is hidden
