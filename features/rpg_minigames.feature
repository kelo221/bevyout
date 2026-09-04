Feature: Headless lockpicking and terminal hacking (M9 wave 7)
  Integer lockpick and hacking sessions are core-owned. Identical
  initial state plus identical ordered input yields the same output,
  item consumption, lock or terminal mutation, and PRNG draw index
  without Bevy. Saving is unavailable while a minigame is active.

  Scenario: Boundary pick angles are accepted and out of range is rejected
    Given a lockpick session difficulty 50 skill 50 sweet spot 0 tolerance 1000
    And the player holds 2 bobby pins
    When the lockpick pick angle is set to -90000
    Then the lockpick session is active
    When the lockpick pick angle is set to 90000
    Then the lockpick session is active
    When the lockpick pick angle is set to 90001
    Then the lockpick input is rejected
    And the minigame RNG draw index is 0

  Scenario: Zero tolerance only matches the exact sweet spot
    Given a lockpick session difficulty 25 skill 25 sweet spot 15000 tolerance 0
    And the player holds 2 bobby pins
    When the lockpick pick angle is set to 15000
    And lockpick torque is applied for 1000 ms
    Then the lock is unlocked
    Given a lockpick session difficulty 25 skill 25 sweet spot 15000 tolerance 0
    And the player holds 2 bobby pins
    When the lockpick pick angle is set to 15001
    And lockpick torque is applied for 1000 ms
    Then the lock is not unlocked

  Scenario: Repeated torque produces identical stress and rotation
    Given a lockpick session difficulty 50 skill 50 sweet spot 0 tolerance 500
    And the player holds 2 bobby pins
    When the lockpick pick angle is set to 20000
    And lockpick torque is applied for 100 ms
    And lockpick torque is applied for 100 ms
    Then the lockpick cylinder milli is 0
    And the lockpick stress is 2000
    Given a lockpick session difficulty 50 skill 50 sweet spot 0 tolerance 500
    And the player holds 2 bobby pins
    When the lockpick pick angle is set to 20000
    And lockpick torque is applied for 100 ms
    And lockpick torque is applied for 100 ms
    Then the lockpick cylinder milli is 0
    And the lockpick stress is 2000

  Scenario: In-sweet-spot torque unlocks without breaking a pin
    Given a lockpick session difficulty 50 skill 75 sweet spot 0 tolerance 5000
    And the player holds 2 bobby pins
    When the lockpick pick angle is set to 0
    And lockpick torque is applied for 1000 ms
    Then the lock is unlocked
    And the player bobby pin count is 2
    And the minigame RNG draw index is 0

  Scenario: Rejected inputs consume no PRNG draw
    Given a lockpick session difficulty 50 skill 50 sweet spot 0 tolerance 1000
    And the player holds 2 bobby pins
    When the lockpick pick angle is set to 180000
    Then the lockpick input is rejected
    And the minigame RNG draw index is 0

  Scenario: Pin break consumes exactly one canonical pin
    Given a lockpick session difficulty 75 skill 10 sweet spot 0 tolerance 100
    And the player holds 2 bobby pins
    When the lockpick pick angle is set to 80000
    And lockpick torque is applied for 1000 ms
    Then a bobby pin broke
    And the player bobby pin count is 1
    And the lock is not unlocked

  Scenario: Cancellation does not unlock or consume a pin
    Given a lockpick session difficulty 50 skill 50 sweet spot 0 tolerance 1000
    And the player holds 2 bobby pins
    When the lockpick pick angle is set to 0
    And lockpick torque is applied for 100 ms
    And the lockpick session is cancelled
    Then the lockpick session is cancelled
    And the lock is not unlocked
    And the player bobby pin count is 2

  Scenario: Force-lock chance and draw index are inspectable
    Given a lockpick session difficulty 100 skill 1 sweet spot 0 tolerance 0
    And the player holds 2 bobby pins
    When the lock is force-attempted
    Then the force lock chance bps is 500
    And the minigame RNG draw index is 1
    And the lockpick session is not succeeded

  Scenario: Force-lock success unlocks without consuming a pin
    Given a lockpick session difficulty 0 skill 100 sweet spot 0 tolerance 0
    And the player holds 2 bobby pins
    When the lock is force-attempted
    Then the lock is unlocked
    And the player bobby pin count is 2
    And the minigame RNG draw index is 1

  Scenario: Identical lockpick inputs replay with the same snapshot
    Given a lockpick session difficulty 50 skill 50 sweet spot 0 tolerance 500 seed 7
    And the player holds 3 bobby pins
    When the lockpick pick angle is set to 40000
    And lockpick torque is applied for 250 ms
    And lockpick torque is applied for 250 ms
    Then the lockpick snapshot round-trips
    Given a lockpick session difficulty 50 skill 50 sweet spot 0 tolerance 500 seed 7
    And the player holds 3 bobby pins
    When the lockpick pick angle is set to 40000
    And lockpick torque is applied for 250 ms
    And lockpick torque is applied for 250 ms
    Then the lockpick snapshot matches the previous snapshot
    And the minigame RNG draw index is 0

  Scenario: Likeness is exhaustive for equal and unequal synthetic words
    Given a hacking session with words PASS WORD XXXX and password PASS
    When the hacking word WORD is guessed
    Then the hacking likeness is 0
    And the hacking attempts remaining are 3
    When the hacking word PASS is guessed
    Then the terminal is unlocked
    And the hacking likeness is 4

  Scenario: Generated boards contain one solution and equal word lengths
    Given a synthetic hacking word bank VENT DOOR LOCK SAFE KEYS
    And a hacking password VENT
    When a hacking board is generated with seed 11
    Then the hacking board has one password
    And every hacking board word has length 4
    And the minigame RNG draw index is greater than 0

  Scenario: Only board words are valid guesses
    Given a hacking session with words PASS WORD XXXX and password PASS
    When the hacking word NOPE is guessed
    Then the hacking input is rejected
    And the hacking attempts remaining are 4
    And the minigame RNG draw index is 0

  Scenario: Bracket pairs cannot be reused
    Given a hacking session with words PASS WORD XXXX DUDY and password PASS
    And a hacking dud bracket 1 and reset bracket 2
    When hacking bracket 1 is used
    Then the hacking board dud count is 2
    When hacking bracket 1 is used
    Then the hacking input is rejected
    When hacking bracket 2 is used
    Then the hacking attempts remaining are 4

  Scenario: Fourth failed word locks the terminal out
    Given a hacking session with words PASS WORD XXXX DUDY and password PASS
    When the hacking word WORD is guessed
    And the hacking word XXXX is guessed
    And the hacking word DUDY is guessed
    And the hacking word WORD is guessed
    Then the terminal is locked out
    And the hacking attempts remaining are 0

  Scenario: Attempt reset never exceeds the configured maximum
    Given a hacking session with words PASS WORD XXXX and password PASS
    And a hacking dud bracket 1 and reset bracket 2
    When the hacking word WORD is guessed
    Then the hacking attempts remaining are 3
    When hacking bracket 2 is used
    Then the hacking attempts remaining are 4
    When hacking bracket 2 is used
    Then the hacking input is rejected
    And the hacking attempts remaining are 4

  Scenario: Same seed and hacking inputs yield byte-identical snapshots
    Given a synthetic hacking word bank VENT DOOR LOCK SAFE
    And a hacking password VENT
    When a hacking board is generated with seed 3
    And the hacking word DOOR is guessed
    Then the hacking snapshot round-trips
    Given a synthetic hacking word bank VENT DOOR LOCK SAFE
    And a hacking password VENT
    When a hacking board is generated with seed 3
    And the hacking word DOOR is guessed
    Then the hacking snapshot matches the previous snapshot

  Scenario: Owned lock attempts report trespass through the crime ledger
    Given a lockpick session difficulty 0 skill 100 sweet spot 0 tolerance 0 owned by 0x0001A2B3
    And the player holds 1 bobby pin
    And a crime witness 0x00041600
    When the lock is force-attempted
    Then the lock is unlocked
    And a trespass crime is reported
    And the player bounty is 40

  Scenario: Unwitnessed owned lock success still does not multiply bounty
    Given a lockpick session difficulty 0 skill 100 sweet spot 0 tolerance 0 owned by 0x0001A2B3
    And the player holds 1 bobby pin
    When the lock is force-attempted
    Then the lock is unlocked
    And no trespass crime is reported
    And the player bounty is 0

  Scenario: Saving is unavailable while a minigame session is active
    Given a lockpick session difficulty 50 skill 50 sweet spot 0 tolerance 1000
    And the player holds 1 bobby pin
    Then saving is blocked for an active minigame
    When the lockpick session is cancelled
    Then saving is allowed after minigame cancellation
