Feature: M6 wave 3 actor residency and resident NAVM topology
  # These scenarios execute the policy seams landed by W3-A and W3-B. The
  # gameplay runtime adapter remains a later W3-C change gated by issue #10.

  Scenario: Actor bind and handoff preserve one canonical identity
    Given actor identity runtime 0x00000020 prepared 0x00000020 and saved state 0x00000020
    And the actor has no resident owner
    When actor residency bind is requested for cell 0x00000100 generation 1
    Then actor residency accepts a Bind transition
    And the canonical actor holder is actor reference 0x00000020
    Given the actor is observed in cell 0x00000100 generation 1
    When actor residency handoff is requested from cell 0x00000100 generation 1 to cell 0x00000200 generation 1
    Then actor residency accepts a Handoff transition
    And the canonical actor holder is actor reference 0x00000020

  Scenario: Stale and duplicate actor projections are rejected
    Given actor identity runtime 0x00000020 prepared 0x00000020 and saved state 0x00000020
    And the actor is observed in cell 0x00000100 generation 8
    When actor residency retain is requested for cell 0x00000100 generation 7
    Then actor residency rejects a StaleSource decision
    Given the actor has resident owners in cell 0x00000100 generation 8 and cell 0x00000200 generation 1
    When actor residency retain is requested for cell 0x00000100 generation 8
    Then actor residency rejects a DuplicateOwner decision

  Scenario: Unload and restore keep the canonical actor state location
    Given actor identity runtime 0x00000020 prepared 0x00000020 and saved state 0x00000020
    And the actor is observed in cell 0x00000100 generation 4
    When actor residency unload is requested from cell 0x00000100 generation 4
    Then actor residency accepts an Unload transition
    When the actor projection is cleared
    And actor residency restore is requested for cell 0x00000100 generation 5
    Then actor residency accepts a Restore transition
    And the canonical actor holder is actor reference 0x00000020

  Scenario: Resident NAVM links require two ready sides
    Given resident NAVM side 0x0000000a generation 1 is navigation-ready
    And resident NAVM side 0x0000000b generation 1 is Loading
    And a resident NAVM portal joins side 0x0000000a generation 1 to side 0x0000000b generation 1
    When the resident NAVM topology is rebuilt
    Then resident NAVM has 1 ready cell and 0 cross-cell links
    Given resident NAVM side 0x0000000b generation 1 is navigation-ready
    When the resident NAVM topology is rebuilt
    Then resident NAVM has 2 ready cells and 1 cross-cell link

  Scenario: Eviction removes stale links and reload needs a new generation portal
    Given resident NAVM side 0x0000000a generation 1 is navigation-ready
    And resident NAVM side 0x0000000b generation 1 is navigation-ready
    And a resident NAVM portal joins side 0x0000000a generation 1 to side 0x0000000b generation 1
    When the resident NAVM topology is rebuilt
    Then resident NAVM has 2 ready cells and 1 cross-cell link
    Given resident NAVM side 0x0000000b generation 1 is Evicting
    When the resident NAVM topology is rebuilt
    Then resident NAVM has 1 ready cell and 0 cross-cell links
    Given resident NAVM side 0x0000000a generation 2 is navigation-ready
    And resident NAVM side 0x0000000b generation 2 is navigation-ready
    When the resident NAVM topology is rebuilt
    Then resident NAVM has 2 ready cells and 0 cross-cell links
    Given a resident NAVM portal joins side 0x0000000a generation 2 to side 0x0000000b generation 2
    When the resident NAVM topology is rebuilt
    Then resident NAVM has 2 ready cells and 1 cross-cell link
