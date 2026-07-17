Feature: Real dead actors decode as source-authored corpses
  # Issue #120 (M4 wave 6). First attempt classified a source-dead NPC via
  # the ACHR reference-header "starts dead" bit (OpenMW's documented
  # `Rec_StartDead`, 0x00000200) -- real-data acceptance against
  # `Fallout3.esm` falsified that: the bit is clear on all 1454 ACHR
  # records in the game, including this issue's own actor
  # (`CG04DeadOldLady`, ACHR 00054398, header 0x00000400 = Persistent
  # only). A data survey found the real FO3 signal on the BASE `NPC_`
  # record's own header flags instead: bit 0x00080000 is set on 174 `NPC_`
  # records, and every one of them -- by editor ID, or (for the 115 without
  # "dead" in the name) by manually checking -- is an actor found dead
  # in-game: `WilliamBrandiceCorpse`, `MS16Corpse1`-`MS16Corpse4`,
  # `DeathclawLoot1*`, `LvlWastelanderDISMEMBER`, `Argyle`, `MS16Beatrice`,
  # among others, with zero false positives. `vsa::openmw_esm4::parse_base`'s
  # wildcard record-dispatch arm already stamps every base record's header
  # flags DWORD onto `BaseRecord.record_flags` (used generically, not
  # NPC_-specific), so no new ESM4 subrecord decode was needed -- this
  # feature drives that real byte-level decode with a synthetic `NPC_`
  # record to prove the FO3-specific starts-dead bit round-trips onto the
  # parsed base.
  #
  # The original ACHR-level bit is kept as a harmless secondary
  # OR-condition in `vsa::prepare::placements::prepared_semantic` (real FO3
  # data never sets it, per the survey above, but it costs nothing to also
  # honor), so its own round-trip scenarios are kept below unchanged.
  #
  # Turning a dead NPC_'s `PreparedSemantic` into `Corpse` and preserving
  # identity/transform/inventory (F119.2) live in
  # `vsa::prepare::placements::prepared_semantic`/`prepared_placement`,
  # which is not std/serde-only reachable from this harness (see that
  # module's doc comment on why it is not pulled in here) -- those are
  # covered by `#[cfg(test)]` unit tests in `src/vsa/prepare/tests/mod.rs`
  # instead: `prepared_semantic_a_dead_npc_base_record_prepares_as_a_corpse`,
  # `prepared_semantic_a_living_npc_base_record_still_prepares_as_npc`,
  # `prepared_semantic_a_living_npc_reference_still_prepares_as_npc`,
  # `prepared_semantic_the_secondary_achr_flag_still_prepares_as_a_corpse_with_no_base`,
  # `prepared_semantic_the_starts_dead_bit_is_not_read_for_creature_references`,
  # and `prepared_placement_preserves_identity_transform_and_inventory_for_a_dead_actor`.

  Scenario: A source-authored dead NPC_ base record decodes the starts-dead flag
    Given an NPC_ base 0x00012345 that starts dead
    When the real-corpses content set is parsed
    Then the parsed NPC_ base 0x00012345 starts dead

  Scenario: A living NPC_ base record does not carry the starts-dead flag
    Given a living NPC_ base 0x00012345
    When the real-corpses content set is parsed
    Then the parsed NPC_ base 0x00012345 does not start dead

  Scenario: A source-authored dead NPC reference decodes the secondary starts-dead flag
    Given a real-corpses cell 0x00028138
    And an ACHR reference 0x00054398 of base 0x00012345 that starts dead
    When the real-corpses content set is parsed
    Then the parsed reference 0x00054398 starts dead

  Scenario: A living NPC reference does not carry the secondary starts-dead flag
    Given a real-corpses cell 0x00028138
    And a living ACHR reference 0x00054399 of base 0x00012345
    When the real-corpses content set is parsed
    Then the parsed reference 0x00054399 does not start dead
