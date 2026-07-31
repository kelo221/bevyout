Feature: Standalone Yarn dialogue waves

  @dialogue-wave0
  Scenario: the compatibility fixture has an executable node
    Given a synthetic Yarn dialogue source
    When the dialogue source is parsed
    Then the dialogue source has a Start node

  @dialogue-wave1
  Scenario: prepared dialogue is deterministic
    Given a synthetic Yarn dialogue source
    When dialogue preparation is run twice
    Then the dialogue preparation fingerprints match

  @dialogue-wave2
  Scenario: the prepared catalog exposes native presentation data
    Given a synthetic Yarn dialogue source
    When the dialogue catalog is prepared for runtime
    Then the catalog exposes one line and one choice

  @dialogue-wave3
  Scenario: host commands retain a stable action key
    Given a synthetic dialogue host command "bo_run_action open_gate"
    When the dialogue host command is normalized
    Then the dialogue host command key is "bo_run_action open_gate"

  @dialogue-wave4
  Scenario: narrative variables retain only persistent values at a boundary
    Given a dialogue snapshot with persistent and session variables
    When the dialogue snapshot reaches a boundary
    Then only the persistent dialogue variable remains

  @dialogue-wave5
  Scenario: authored NPC binding resolves a stable conversation
    Given a synthetic Yarn dialogue source
    When the authored NPC dialogue key is resolved
    Then the authored NPC conversation is available

  @dialogue-wave6
  Scenario: Fallout dialogue inventory keeps unsupported records visible
    Given a synthetic Fallout dialogue inventory
    When the Fallout dialogue inventory is rendered
    Then the inventory reports one topic and one unsupported record

  @dialogue-wave7
  Scenario: generated Fallout dialogue preserves source mapping
    Given a synthetic Fallout conversation
    When the Fallout conversation is generated twice
    Then the generated dialogue bytes and mappings match

  @dialogue-wave8
  Scenario: a dialogue checkpoint is explicit and idempotent
    Given an explicit dialogue checkpoint at node "Checkpoint"
    When the checkpoint snapshot is inspected
    Then the checkpoint contains no Yarn VM state

  @dialogue-wave9
  Scenario: presentation coverage and timing are deterministic
    Given a dialogue presentation policy for language "en-US"
    When dialogue coverage is calculated for one line
    Then the dialogue timing and coverage are deterministic
