Feature: Loading-fallback fades, cancellation, and failure recovery
  # Pins issue #59's pure fallback-lifecycle policy and overlay fade-progress
  # function: `src/viewer/world/swap_policy.rs`'s `fallback_lifecycle_outcome`
  # (F59.1) and `fade_in_alpha`/`fade_out_alpha` (F59.2). Instant swaps never
  # enter this lifecycle and are out of scope here (see
  # features/instant_swap.feature). The Bevy-side driver behavior (cancel,
  # supersede, failure notice) is covered by `src/viewer/world/swap.rs`'s own
  # `#[cfg(test)]` unit tests (T59.3), not by these pure-policy scenarios.

  Scenario: No fallback in flight ignores a destination-ready event
    Given no fallback is in flight
    When the destination becomes ready
    Then the fallback lifecycle outcome is Ignore

  Scenario: No fallback in flight ignores a parse-failed event
    Given no fallback is in flight
    When the fallback parse fails
    Then the fallback lifecycle outcome is Ignore

  Scenario: No fallback in flight ignores a player-cancel event
    Given no fallback is in flight
    When the player cancels the fallback
    Then the fallback lifecycle outcome is Ignore

  Scenario: No fallback in flight ignores a superseding request
    Given no fallback is in flight
    When a superseding travel request arrives
    Then the fallback lifecycle outcome is Ignore

  Scenario: An in-flight fallback proceeds once the destination is ready
    Given a fallback is in flight
    When the destination becomes ready
    Then the fallback lifecycle outcome is Proceed

  Scenario: An in-flight fallback returns to source when its parse fails
    Given a fallback is in flight
    When the fallback parse fails
    Then the fallback lifecycle outcome is ReturnToSource

  Scenario: An in-flight fallback cancels when the player presses Esc
    Given a fallback is in flight
    When the player cancels the fallback
    Then the fallback lifecycle outcome is Cancel

  Scenario: An in-flight fallback is superseded by a new travel request
    Given a fallback is in flight
    When a superseding travel request arrives
    Then the fallback lifecycle outcome is Supersede

  Scenario: The overlay fades fully in over its duration
    Given an overlay fade duration of 0.25 seconds and max alpha 0.85
    When the overlay has been fading in for 0 seconds
    Then the overlay alpha is 0
    When the overlay has been fading in for 0.25 seconds
    Then the overlay alpha is 0.85

  Scenario: The overlay fades fully out over its duration
    Given an overlay fade duration of 0.25 seconds and max alpha 0.85
    When the overlay has been fading out for 0 seconds
    Then the overlay alpha is 0.85
    When the overlay has been fading out for 0.25 seconds
    Then the overlay alpha is 0

  Scenario: The out-fade is the symmetric mirror of the in-fade
    Given an overlay fade duration of 0.25 seconds and max alpha 0.85
    When the overlay has been fading out for 0.0625 seconds
    Then the overlay alpha matches fading in for 0.1875 seconds
