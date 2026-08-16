Feature: Prepared cache garbage collection
  Unreachable generated artifacts are removed only after reachability and grace checks.

  Scenario: A reachable object is retained regardless of age
    Given a reachable cache object that is 720 hours old
    When cache garbage collection is planned with a 168 hour grace period
    Then the cache entry is retained

  Scenario: A recent unreachable object is retained by the grace period
    Given an unreachable cache object that is 24 hours old
    When cache garbage collection is planned with a 168 hour grace period
    Then the cache entry is retained

  Scenario: An old unreachable object is selected
    Given an unreachable cache object that is 720 hours old
    When cache garbage collection is planned with a 168 hour grace period
    Then the cache entry is selected as an unreferenced object

  Scenario: Quarantine is never selected automatically
    Given an unreachable quarantined cache entry that is 720 hours old
    When cache garbage collection is planned with a 0 hour grace period
    Then the cache entry is retained

  Scenario: Rebuildable assets require an explicit opt-in
    Given an unreachable rebuildable cache asset that is 720 hours old
    When cache garbage collection is planned with a 0 hour grace period
    Then the cache entry is retained
    When cache garbage collection is planned with rebuildable assets and a 0 hour grace period
    Then the cache entry is selected as an unreferenced rebuildable asset
