Feature: Pure active-effect, radiation, and addiction kernels
  The wave-3 engines are three pure core modules: an ordered ledger of
  timed value-modifier effects that ticks in whole milliseconds, a
  radiation pool whose vanilla thresholds replace SPECIAL penalties, and
  a per-chem addiction machine driven by a core-owned splitmix64 PRNG
  whose draws are inspectable and reproducible.

  Scenario: A chem buff ticks down in whole milliseconds and expires
    Given a clean effect ledger
    When the player takes a timed chem effect strength +2 lasting 240 seconds
    And the ledger ticks 239000 milliseconds
    Then the strength modifier is 2
    And the tick expires 0 effects
    When the ledger ticks 1000 milliseconds
    Then the tick expires 1 effects
    And the strength modifier is 0
    And the ledger holds 0 active effects

  Scenario: Re-dosing a chem refreshes the timer instead of stacking
    Given a clean effect ledger
    When the player takes a timed chem effect agility +1 lasting 240 seconds
    And the player takes a timed chem effect agility +2 lasting 120 seconds
    Then the ledger holds 1 active effects
    And the agility modifier is 2
    And the tick expires 0 effects

  Scenario: Radiation penalties replace each other at the thresholds
    Given a radiation pool of 150 rads
    When effective SPECIAL is projected
    Then effective strength is 5
    Given a radiation pool of 200 rads
    When effective SPECIAL is projected
    Then effective endurance is 4
    Given a radiation pool of 400 rads
    When effective SPECIAL is projected
    Then effective endurance is 3
    And effective agility is 4
    Given a radiation pool of 600 rads
    When effective SPECIAL is projected
    Then effective endurance is 2
    And effective agility is 3
    And effective strength is 4
    Given a radiation pool of 800 rads
    When effective SPECIAL is projected
    Then effective endurance is 2
    And effective intelligence is 4
    And effective strength is 3

  Scenario: The fatal dose is exactly 1000 rads
    Given a radiation pool of 999 rads
    Then the highest radiation threshold is 800
    And the radiation pool is not fatal
    When 1 rads are applied with 0 basis points of resistance
    Then the radiation pool is 1000 rads
    And the radiation pool is fatal

  Scenario: Resistance reduces an absorbed dose in whole rads
    Given a radiation pool of 0 rads
    When 100 rads are applied with 2000 basis points of resistance
    Then the absorbed dose is 80 rads
    And the radiation pool is 80 rads

  Scenario: RadAway removes rads but never below zero
    Given a radiation pool of 120 rads
    When 50 rads are removed
    Then the radiation pool is 70 rads
    When 500 rads are removed
    Then the radiation pool is 0 rads
    And only 70 rads were actually removed by the oversized dose

  Scenario: Effective SPECIAL combines chem buffs with radiation penalties
    Given a clean effect ledger
    And a radiation pool of 600 rads
    When the player takes a timed chem effect strength +2 lasting 240 seconds
    And the player takes a timed chem effect endurance +3 lasting 240 seconds
    And effective SPECIAL is projected
    Then effective strength is 6
    And effective endurance is 5
    And effective agility is 3
    And effective luck is 5

  Scenario: The PRNG stream is pinned and addiction rolls reproduce
    Given a fresh addiction rng seeded with 0
    When an addiction roll is made with 2000 bps chance and 0 bps resistance
    Then the addiction roll is false
    And 1 rng draws were consumed
    When an addiction roll is made with 0 bps chance and 0 bps resistance
    Then the addiction roll is false
    And 1 rng draws were still consumed in total
    Given a fresh addiction rng seeded with 6
    When an addiction roll is made with 2000 bps chance and 0 bps resistance
    Then the addiction roll is true

  Scenario: Full chem resistance neutralizes even a certain dose
    Given a fresh addiction rng seeded with 6
    When an addiction roll is made with 10000 bps chance and 10000 bps resistance
    Then the addiction roll is false

  Scenario: The addiction machine walks clean, addicted, withdrawing, cured
    Given a clean addiction state
    Then the withdrawal 00033067 addiction phase is clean
    When the player becomes addicted to withdrawal 00033067
    Then the withdrawal 00033067 addiction phase is addicted
    When the chem wears off for withdrawal 00033067
    Then the withdrawal 00033067 addiction phase is withdrawing
    When the withdrawal 00033067 is cured
    Then the withdrawal 00033067 addiction phase is clean

  Scenario: Derived health and action points use projected actor values
    Given a clean effect ledger
    And a radiation pool of 600 rads
    When the player takes a timed chem effect endurance +3 lasting 240 seconds
    And the player takes a timed action_points effect +30 lasting 240 seconds
    And derived actor values are projected
    Then projected maximum health is 200
    And projected maximum action points is 101

  Scenario: Active RadResist is converted and capped for radiation doses
    Given a clean effect ledger
    When the player takes a timed rad_resist effect +25 lasting 240 seconds
    Then active radiation resistance is 2500 basis points
    When the player takes a timed rad_resist effect +100 lasting 240 seconds
    Then active radiation resistance is 8500 basis points

  Scenario: Real Stimpak conditions select one Fast Metabolism branch
    Given the player does not own perk 00094ebf
    Then the Stimpak 30 health condition is true
    And the Stimpak 36 health condition is false
    Given the player owns perk 00094ebf
    Then the Stimpak 30 health condition is false
    And the Stimpak 36 health condition is true
