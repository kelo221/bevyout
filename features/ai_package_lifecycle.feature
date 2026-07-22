Feature: AI package lifecycle state machine
  # `viewer::ai::lifecycle` (issue #194) drives one actor's active package
  # through select -> start -> tick -> pause/preempt -> complete -> fail ->
  # retry. Preemption pauses the running package (its step preserved) and
  # re-selecting it later RESUMES at that step rather than restarting; failure
  # backs off exponentially so a broken package does not spin, and is terminal
  # once retries are exhausted. Only the active-or-paused package's identity,
  # step, and elapsed time persist -- the assertions below check the observable
  # resumed step, never the machine's private bookkeeping.

  Scenario: Selecting a package starts it running at step zero
    Given a fresh package lifecycle
    When package 0x00000010 is selected
    Then the lifecycle phase is running
    And the active package is 0x00000010
    And the active step is 0

  Scenario: A preempted package resumes at its saved step, not a restart
    Given a fresh package lifecycle
    When package 0x0000000A is selected
    And the active package advances 2 steps
    And package 0x0000000B is selected
    Then the active package is 0x0000000B
    And the paused package is 0x0000000A
    When the active package completes
    And package 0x0000000A is selected
    Then the active package is 0x0000000A
    And the active step is 2
    And the lifecycle phase is running

  Scenario: A schedule gap pauses the running package
    Given a fresh package lifecycle
    When package 0x0000000A is selected
    And the active package advances 1 steps
    And no package is selected for the lifecycle
    Then the lifecycle phase is paused
    And the paused package is 0x0000000A

  Scenario: Failure backs off and does not spin, then restarts fresh
    Given a fresh package lifecycle with backoff 1.0 and max retries 3
    When package 0x00000010 is selected
    And the active package fails
    Then the lifecycle phase is awaiting-retry
    When the lifecycle ticks 0.5 seconds
    Then the lifecycle phase is awaiting-retry
    When the lifecycle ticks 0.6 seconds
    Then the lifecycle phase is running
    And the active step is 0
    And the retry count is 1

  Scenario: Retry exhaustion terminally fails the package
    Given a fresh package lifecycle with backoff 0.1 and max retries 2
    When package 0x00000010 is selected
    And the active package fails
    And the lifecycle ticks 0.2 seconds
    And the active package fails
    And the lifecycle ticks 0.2 seconds
    And the active package fails
    Then the lifecycle phase is failed
    When the lifecycle ticks 10.0 seconds
    Then the lifecycle phase is failed

  Scenario: A running package snapshots and resumes at the right step across save/load
    Given a fresh package lifecycle
    When package 0x00001234 is selected
    And the active package advances 3 steps
    And the lifecycle ticks 4.5 seconds
    And the lifecycle checkpoint is persisted
    And the lifecycle is rebuilt from the checkpoint
    Then the lifecycle phase is running
    And the active package is 0x00001234
    And the active step is 3
    And the active elapsed is 4.5
