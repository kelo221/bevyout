Feature: Integer game clock and lifecycle (M9 wave 9)
  One integer game-time advance drives effects, restocks, healing,
  cell reset, encounter-zone lock, and fast travel in pinned order.
  Lighting hours are a projection and never become save authority.

  Scenario: Calendar projection and lighting hour do not feed back
    Given the game clock is 0 ms with timescale 30
    Then the calendar is 2277-10-23 00:00
    And the lighting hour projection is 0
    When the game clock advances 43200000 ms for console
    Then the calendar is 2277-10-23 12:00
    And the lighting hour projection is 12
    And the lighting hour projection does not change the clock

  Scenario: Realtime remainder accumulates without losing milliseconds
    Given the game clock is 0 ms with timescale 30
    When the game clock advances 500 real microseconds
    Then the game clock is 15 ms
    And the timescale remainder is 0
    When the game clock advances 1 real microseconds
    Then the game clock is 15 ms
    And the timescale remainder is 30
    When the game clock advances 33 real microseconds
    Then the game clock is 16 ms

  Scenario: Overflow leaves the clock unchanged
    Given the game clock is 18446744073709551615 ms with timescale 30
    When the game clock advances 1 ms for console
    Then the time advance is rejected as overflow
    And the game clock is 18446744073709551615 ms

  Scenario: 72-hour restock boundary is exclusive then inclusive
    Given a scheduled merchant restock owner 1 due at 259200000 ms
    When the game clock advances 259199999 ms for wait
    Then restock did not run
    And the executed lifecycle kinds are effects, radiation, death
    When the game clock advances 1 ms for wait
    Then restock ran with generation 1
    And the executed lifecycle kinds are effects, radiation, death, restock

  Scenario: Large jump processes every crossed restock deadline
    Given a scheduled merchant restock owner 1 due at 259200000 ms
    When the game clock advances 518400000 ms for wait
    Then restock ran with generation 2

  Scenario: Same-timestamp tasks sort by kind then owner
    Given a scheduled merchant restock owner 2 due at 1000 ms
    And a scheduled merchant restock owner 1 due at 1000 ms
    And a scheduled cell reset owner 9 due at 1000 ms
    When the game clock advances 1000 ms for wait
    Then the due task owners are 1, 2, 9

  Scenario: Occupied cells do not reset
    Given cell 0x000151e3 last visited at 0 ms occupied
    When the game clock advances 259200000 ms for wait
    Then cell 0x000151e3 did not reset
    When cell 0x000151e3 is vacated at 259200000 ms
    And the game clock advances 259200000 ms for wait
    Then cell 0x000151e3 reset generation is 1

  Scenario: Unique and player-owned holders survive reset
    Given cell 0x000151e3 last visited at 0 ms vacant
    And cell 0x000151e3 unique ref 0x10
    And cell 0x000151e3 container 0x20 with a player-owned item
    And cell 0x000151e3 container 0x21 with only unowned items
    And cell 0x000151e3 non-unique actor 0x30
    And cell 0x000151e3 unique actor 0x31
    And cell 0x000151e3 corpse 0x40
    When the game clock advances 259200000 ms for wait
    Then cell 0x000151e3 reset generation is 1
    And container 0x20 was preserved
    And container 0x21 was restored
    And actor 0x30 was respawned
    And actor 0x31 survived
    And corpse 0x40 was removed
    When the same cell reset is applied again
    Then the cell reset is rejected as already applied

  Scenario: Encounter zone locks on first entry
    Given encounter zone 0x0002a4a0 min 2 max 10
    When the player at level 6 enters zone 0x0002a4a0
    Then the locked encounter level is 6
    When the player at level 10 enters zone 0x0002a4a0
    Then the locked encounter level is 6

  Scenario: Fast travel is blocked independently
    Given a discovered destination 0x0001a000 travel 3600000 ms
    And fast travel evidence danger
    When fast travel is committed
    Then fast travel is blocked by danger
    Given a discovered destination 0x0001a000 travel 3600000 ms
    And fast travel evidence interior
    When fast travel is committed
    Then fast travel is blocked by interior
    Given a discovered destination 0x0001a000 travel 3600000 ms
    And fast travel evidence encumbered
    When fast travel is committed
    Then fast travel is blocked by encumbered
    Given a discovered destination 0x0001a000 travel 3600000 ms
    And fast travel evidence combat
    When fast travel is committed
    Then fast travel is blocked by combat
    Given a discovered destination 0x0001a000 travel 3600000 ms
    And fast travel evidence radiation
    When fast travel is committed
    Then fast travel is blocked by radiation
    Given an undiscovered destination 0x0001a000 travel 3600000 ms
    When fast travel is committed
    Then fast travel is blocked by undiscovered

  Scenario: Fast travel advances time then tasks then arrival
    Given a timed effect remaining 1000 ms
    And a scheduled merchant restock owner 1 due at 1000 ms
    And a discovered destination 0x0001a000 travel 3600000 ms
    When fast travel is committed
    Then the game clock is 3600000 ms
    And the timed effect expired
    And restock ran with generation 1
    And the player location is 0x0001a000
    And arrival was requested for 0x0001a000

  Scenario: Effects expire during a console wait
    Given a timed effect remaining 500 ms
    When the game clock advances 499 ms for wait
    Then the timed effect remains
    When the game clock advances 1 ms for wait
    Then the timed effect expired
