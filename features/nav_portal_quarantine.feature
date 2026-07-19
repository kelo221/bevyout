Feature: Per-link portal quarantine - deterministic merge-link kind assignment and allow-list computation
  # Issue #162 (M4 wave 10). A timed-out KCC-swept merge-portal crossing
  # excludes only that specific link for the agent's subsequent repaths
  # instead of clearing its whole route (the wave-8 minimum-viable
  # mitigation this replaces). `viewer::nav::landmass_graph` owns the pure,
  # Bevy-engine-free half of that mechanism: assigning each validated
  # merge candidate its own `landmass` animation-link `kind`
  # (`merge_link_kind`, deterministic by build order, starting at 1 so it
  # never collides with the reserved door kind 0 -- every door link keeps
  # sharing kind 0, so a blocked merge portal can never lock a door), and
  # computing the `PermittedAnimationLinks`-equivalent allow-list a
  # quarantined agent may still use (`permitted_animation_link_kinds`).
  #
  # The Bevy-side wiring -- `merge_traversal_system`'s timeout branch, the
  # one-tick target-blank repath that forces a genuine landmass solve, and
  # the `tna goto`/`tna travel` quarantine-clear lifecycle -- is exercised
  # by `nav/agent.rs`'s own `#[cfg(test)]` unit tests against a live
  # `Archipelago3d`, mirroring how issue #155's door-lock query-time
  # exclusion split its own coverage the same way (this feature stays free
  # of any Bevy `App`/`World`, per `landmass_graph.rs`'s own module doc
  # comment on why it cannot depend on `bevy`).

  Scenario: Merge-link kinds start at 1, one past the reserved door kind
    Given merge-link candidate index 0
    When the merge-link kind is resolved
    Then the merge-link kind is 1

  Scenario: Later candidates get later kinds, in build order
    Given merge-link candidate index 4
    When the merge-link kind is resolved
    Then the merge-link kind is 5

  Scenario: An unquarantined agent has no kind restriction
    Given 3 merge-link kinds exist
    When the permitted animation link kinds are computed
    Then every animation link kind is permitted

  Scenario: A quarantined link is excluded but every other kind, including the reserved door kind, stays permitted
    Given 3 merge-link kinds exist
    And merge-link kind 2 is quarantined
    When the permitted animation link kinds are computed
    Then the permitted animation link kinds are 0, 1, 3

  Scenario: Quarantining every merge kind still leaves the door kind permitted
    Given 2 merge-link kinds exist
    And merge-link kind 1 is quarantined
    And merge-link kind 2 is quarantined
    When the permitted animation link kinds are computed
    Then the permitted animation link kinds are 0

  Scenario: A stale quarantine entry past this build's own kind range is harmless
    Given 2 merge-link kinds exist
    And merge-link kind 99 is quarantined
    When the permitted animation link kinds are computed
    Then the permitted animation link kinds are 0, 1, 2
