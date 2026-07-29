Feature: Fallout time-of-day lighting policy

  Scenario: Ordinary interiors keep authored lighting
    Given an ordinary interior cell
    When day-night preview is disabled
    Then dynamic day-night lighting is disabled

  Scenario: Exterior-like interiors follow climate lighting
    Given an interior cell that behaves like exterior
    When day-night preview is disabled
    Then dynamic day-night lighting is enabled

  Scenario: Preview overrides the interior gate
    Given an ordinary interior cell
    When day-night preview is enabled
    Then dynamic day-night lighting is enabled

  Scenario: One real minute advances one full day
    Given a Fallout clock at hour 12 with timescale 1440
    When 60 real seconds elapse
    Then the Fallout clock reads hour 12

  Scenario: The clock wraps through midnight
    Given a Fallout clock at hour 23 with timescale 720
    When 10 real seconds elapse
    Then the Fallout clock reads hour 1

  Scenario: Sunrise reaches its authored color at the midpoint
    Given scalar weather colors night 0 sunrise 1 day 2 sunset 3
    When weather color is sampled at hour 6
    Then the sampled weather color is 1

  Scenario: Sunset reaches its authored color at the midpoint
    Given scalar weather colors night 0 sunrise 1 day 2 sunset 3
    When weather color is sampled at hour 18
    Then the sampled weather color is 3

  Scenario: Preview fallback prefers WastelandClear deterministically
    Given preview weather candidates "00000001:Cloudy,00000050:WastelandClear,00000002:Rain"
    When preview fallback weather is selected
    Then preview weather 00000050 is selected

  Scenario: Preview fallback uses the lowest usable FormID without WastelandClear
    Given preview weather candidates "00000020:Cloudy,00000002:Rain"
    When preview fallback weather is selected
    Then preview weather 00000002 is selected
