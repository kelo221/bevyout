Feature: In-game console quality of life

  Scenario: Clear wipes the visible transcript but preserves command history
    Given console history contains "help,getpos z"
    And the console transcript contains "help,commands listed,getpos z,42"
    When the console submission "clear" is applied
    Then the console transcript is empty
    And console history is "help,getpos z,clear"

  Scenario: Long sessions retain a bounded recent transcript
    Given an empty console transcript
    When 205 numbered console lines are appended
    Then the console transcript contains 200 lines
    And the first retained console line is "line 5"
    And the last retained console line is "line 204"
