Feature: Instant door transition swap with loading-screen fallback
  # Pins issue #52's pure swap-eligibility and save-application seams:
  # `src/viewer/world/swap_policy.rs`. A resident, fully-loaded (Ready)
  # destination cell swaps instantly; anything else (still Loading, or not
  # resident at all) falls back to a loading screen, and a fallback whose
  # load fails returns the player to the source cell. Save-layer reference
  # deltas (disabled/deleted/transform) are applied to a cell's placements
  # on activation. Drives `swap_policy` directly, the same way
  # features/preload_policy.feature drives `world::policy`.

  Scenario: A Ready resident destination swaps instantly
    Given the destination cell residency is Ready
    And the destination manifest exists on disk
    When the swap decision is computed
    Then the swap decision is Instant

  Scenario: A Loading resident destination falls back
    Given the destination cell residency is Loading
    And the destination manifest exists on disk
    When the swap decision is computed
    Then the swap decision is Fallback

  Scenario: An absent destination with a manifest on disk falls back
    Given the destination cell residency is Absent
    And the destination manifest exists on disk
    When the swap decision is computed
    Then the swap decision is Fallback

  Scenario: An absent destination with no manifest on disk falls back
    Given the destination cell residency is Absent
    And the destination manifest does not exist on disk
    When the swap decision is computed
    Then the swap decision is Fallback

  Scenario: A fallback load that succeeds proceeds with the swap
    Given a fallback load that succeeds
    When the fallback outcome is computed
    Then the fallback outcome is Proceed

  Scenario: A fallback load that fails returns the player to the source cell
    Given a fallback load that fails
    When the fallback outcome is computed
    Then the fallback outcome is ReturnToSource

