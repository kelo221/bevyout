Feature: Resolved script record inventory
  Script preparation consumes one load-order stream whose collectors agree on
  winning overrides and deletions before any SCPT semantics are interpreted.

  Scenario: A later record version wins with complete provenance
    Given record 00000400 from master.esm carries payload "original"
    And record 00000400 from patch.esp carries payload "patched"
    When the record versions are collected in load order
    Then record 00000400 has winning payload "patched"
    And record 00000400 has provenance "master.esm,patch.esp"

  Scenario: A deleted override removes the winning record
    Given record 00000400 from master.esm carries payload "original"
    And record 00000400 is deleted by patch.esp
    When the record versions are collected in load order
    Then record 00000400 is absent from the winning records

  Scenario: A top-level script is decoded structurally without interpreting bytecode
    Given a synthetic object script with one local and one reference
    When the structural script record is decoded
    Then the script kind is object
    And the compiled script bytes are preserved
    And local slot 3 is named "Counter"
    And script reference 00000010 is resolved

  Scenario: Malformed and unknown script subrecords remain inspectable
    Given a synthetic script with a malformed header and unknown subrecord ZZZZ
    When the structural script record is decoded
    Then the script has a SCHR size diagnostic
    And unknown script subrecord ZZZZ is preserved

  Scenario: A direct script attachment resolves through its owner's plugin
    Given a synthetic owner record with SCRI 01000010
    When the owner's script attachments are decoded with plugin index 2
    Then attachment slot 0 targets script 02000010

  Scenario: Package action scripts have stable embedded identities
    Given a synthetic package with OnBegin and OnEnd embedded scripts
    When the package's embedded scripts are decoded
    Then the package has embedded script slots OnBegin, OnEnd
    And each embedded script preserves its compiled bytes

  Scenario: Script inventory JSON is byte-identical across repeated renders
    Given a synthetic script inventory report
    When the script inventory report is rendered twice
    Then both script inventory JSON renders are byte-identical
    And the script inventory summary reports 1 top-level and 1 embedded script
