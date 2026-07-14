Feature: Door and activator controller animation policy
  # Pins issue #57's pure clip-selection and open-lead seams:
  # `src/viewer/animation/policy.rs`. Opening prefers an "Open" clip
  # (falling back to the alphabetically-first clip for an activator with no
  # "Open"), closing prefers "Close" with no fallback, and an asset with no
  # clips selects nothing either way. A travel door's open-lead is the
  # "Open" clip's duration capped at ~0.6s; no clip means zero lead so
  # `DoorTravelRequested` still fires the same frame (wave-2 behavior).
  # Drives `animation::policy` directly, the same way
  # features/instant_swap.feature drives `world::swap_policy`.

  Scenario: Opening a door with both clips picks Open
    Given a placement with clips "Close, Open"
    When the placement is opened
    Then the selected clip is "Open"

  Scenario: Closing a door with both clips picks Close
    Given a placement with clips "Close, Open"
    When the placement is closed
    Then the selected clip is "Close"

  Scenario: Activating a clip-bearing activator with no Open clip picks the first clip
    Given a placement with clips "Use, Activate"
    When the placement is opened
    Then the selected clip is "Activate"

  Scenario: A placement with no clips selects nothing when opened
    Given a placement with no clips
    When the placement is opened
    Then no clip is selected

  Scenario: A placement with no clips selects nothing when closed
    Given a placement with no clips
    When the placement is closed
    Then no clip is selected

  Scenario: A short Open clip's duration becomes the travel lead uncapped
    Given a travel door with an Open clip lasting 0.3 seconds
    When the open lead is computed
    Then the open lead is 0.3 seconds

  Scenario: A long Open clip's duration is capped at the lead cap
    Given a travel door with an Open clip lasting 1.33 seconds
    When the open lead is computed
    Then the open lead is 0.6 seconds

  Scenario: A travel door with no Open clip has zero lead
    Given a travel door with no Open clip
    When the open lead is computed
    Then the open lead is 0.0 seconds
