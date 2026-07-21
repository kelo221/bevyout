Feature: ESC pause menu selection
  # Pure selection state behind the Fallout-style ESC pause menu.
  # Runtime wiring (blur snapshot, Camera3d suspend, monofonto UI) lives in
  # src/viewer/pause_menu/ and is covered by unit tests + manual acceptance.

  Scenario: Default selection is Continue and activates resume
    Given a fresh pause menu
    Then the pause menu selection is Continue
    And activating the pause menu yields Continue

  Scenario: Navigation wraps through the full FO3 stack
    Given a fresh pause menu
    When the pause menu moves up
    Then the pause menu selection is Quit
    And activating the pause menu yields Quit
    When the pause menu moves down
    Then the pause menu selection is Continue

  Scenario Outline: Disabled placeholders never activate
    Given a fresh pause menu
    When the pause menu selects <option>
    Then the pause menu selection is <option>
    And activating the pause menu yields nothing

    Examples:
      | option   |
      | Save     |
      | Load     |
      | Settings |
      | Help     |

  Scenario: Labels match Fallout title case
    Given a fresh pause menu
    Then pause menu option Continue is labeled "Continue"
    And pause menu option Save is labeled "Save"
    And pause menu option Load is labeled "Load"
    And pause menu option Settings is labeled "Settings"
    And pause menu option Help is labeled "Help"
    And pause menu option Quit is labeled "Quit"
