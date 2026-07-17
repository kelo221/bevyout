Feature: Real dead actors decode as source-authored corpses
  # Issue #120 (M4 wave 6). `vsa::openmw_esm4::parse_reference` already
  # threads the raw ACHR/ACRE record-header `flags` DWORD onto
  # `ReferenceRecord.flags` -- the same field `RECORD_DELETED`/
  # `RECORD_DISABLED` already read from. F119.1 adds the "starts dead" bit
  # (0x00000200) to that meaning: OpenMW's `components/esm4/common.hpp`
  # documents it as `Rec_StartDead` for `ACHR` (the same bit means something
  # else, `MotionBlurCastsShadows`, on a plain `REFR` object reference, and
  # FO3 documents no ACRE-specific override, so only ACHR/NPC references are
  # in scope). This feature drives the real byte-level decode with synthetic
  # ACHR records to prove that bit round-trips onto the parsed reference.
  #
  # Turning a dead reference's `PreparedSemantic` into `Corpse` (the other
  # half of F119.1) and preserving its identity/transform/inventory (F119.2)
  # live in `vsa::prepare::placements::prepared_semantic`/`prepared_placement`,
  # which is not std/serde-only reachable from this harness (see that
  # module's doc comment on why it is not pulled in here) -- those are
  # covered by `#[cfg(test)]` unit tests in `src/vsa/prepare/tests/mod.rs`
  # instead: `prepared_semantic_a_source_dead_npc_reference_prepares_as_a_corpse`,
  # `prepared_semantic_a_living_npc_reference_still_prepares_as_npc`,
  # `prepared_semantic_the_starts_dead_bit_is_not_read_for_creature_references`,
  # and `prepared_placement_preserves_identity_transform_and_inventory_for_a_dead_actor`.

  Scenario: A source-authored dead NPC reference decodes the starts-dead flag
    Given a real-corpses cell 0x00028138
    And an ACHR reference 0x00054398 of base 0x00012345 that starts dead
    When the real-corpses content set is parsed
    Then the parsed reference 0x00054398 starts dead

  Scenario: A living NPC reference does not carry the starts-dead flag
    Given a real-corpses cell 0x00028138
    And a living ACHR reference 0x00054399 of base 0x00012345
    When the real-corpses content set is parsed
    Then the parsed reference 0x00054399 does not start dead
