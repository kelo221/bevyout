Feature: Bounded runtime performance probes
  # Pins the pure frame-window policy used by the BRP/MCP performance probe.
  # A probe selects samples strictly newer than its start marker, retains the
  # newest bounded window, calculates nearest-rank percentiles, and counts
  # frames that exceed (not equal) the requested budget.

  Scenario: A probe summarizes only frames after its marker
    Given frame-time samples "0:8,1:16,2:40,3:20"
    When frames after sample 0 are summarized with latest limit 3 and budget 16 ms
    Then the frame probe covers samples 1 through 3
    And the frame probe has 3 samples
    And the frame probe p95 and max are 40 and 40 ms
    And 2 frames exceed the probe budget

  Scenario: A probe retains the newest bounded window
    Given frame-time samples "0:5,1:10,2:15,3:20,4:25"
    When frames after sample 0 are summarized with latest limit 2 and budget 25 ms
    Then the frame probe covers samples 3 through 4
    And the frame probe has 2 samples
    And 0 frames exceed the probe budget

  Scenario: A frame equal to the budget is not a hitch
    Given frame-time samples "7:16,8:17"
    When all frames are summarized with latest limit 10 and budget 16 ms
    Then 1 frame exceeds the probe budget
