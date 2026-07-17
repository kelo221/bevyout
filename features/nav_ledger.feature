Feature: Intercell nav-agent ledger and swap eligibility
  # Issue #134 (M4 wave 4). Pure `viewer::nav::ledger_policy` seams under
  # test, no game data, no Bevy: recording/claiming ledger entries on cell
  # activation, stale-entry diagnosis when a door-marker's destination door
  # is missing from the active cell, and the strict follow-through-vs-
  # freeze swap-eligibility table. `viewer::nav::agent`'s runtime wiring
  # (handoff/freeze/restore, `tna travel`, `tna status`) is covered by its
  # own `#[cfg(test)]` minimal-App tests, not cucumber, per this module's
  # std/serde-only, no-Bevy constraint.

  Scenario: A ledgered agent is claimed only when its cell activates
    Given a ledger entry for agent 1 in cell 0x00100000 frozen at 1, 2, 3
    And a ledger entry for agent 9 in cell 0x00200000 frozen at 9, 0, 0
    When the ledger is claimed for cell 0x00100000 with known doors none
    Then 1 entry is restored
    And restored entry 0 is agent 1 frozen at 1, 2, 3
    And 0 entries are stale
    And the ledger still holds an entry for agent 9

  Scenario: Claiming a cell with no matching entries restores nothing
    Given a ledger entry for agent 1 in cell 0x00100000 frozen at 0, 0, 0
    When the ledger is claimed for cell 0x00999999 with known doors none
    Then 0 entries are restored
    And 0 entries are stale
    And the ledger still holds an entry for agent 1

  Scenario: A door-marker entry restores when its destination door is known
    Given a ledger entry for agent 1 in cell 0x00100000 with door marker 0x0000D00
    When the ledger is claimed for cell 0x00100000 with known doors 0x0000D00
    Then 1 entry is restored
    And restored entry 0 is agent 1 with door marker 0x0000D00
    And 0 entries are stale

  Scenario: A door-marker entry missing its destination door is diagnosed as stale
    Given a ledger entry for agent 1 in cell 0x00100000 with door marker 0x0000D00
    When the ledger is claimed for cell 0x00100000 with known doors none
    Then 0 entries are restored
    And 1 entry is stale
    And stale entry 0 is agent 1 cell 0x00100000 missing door 0x0000D00

  Scenario: Recording an entry for the same agent replaces the previous one
    Given a ledger entry for agent 1 in cell 0x00100000 frozen at 0, 0, 0
    And a ledger entry for agent 1 in cell 0x00200000 with door marker 0x0000D50
    When the ledger is claimed for cell 0x00100000 with known doors none
    Then 0 entries are restored
    When the ledger is claimed for cell 0x00200000 with known doors 0x0000D50
    Then 1 entry is restored
    And restored entry 0 is agent 1 with door marker 0x0000D50

  Scenario Outline: Follow-through requires the agent's active route to end at the exact door the player used
    Given the agent's active route door is <route_door>
    When the swap eligibility is decided for door 0x0000D00
    Then the swap eligibility is <eligibility>

    Examples:
      | route_door | eligibility    |
      | none       | freeze         |
      | 0x0000D00  | follow-through |
      | 0x0000D50  | freeze         |
