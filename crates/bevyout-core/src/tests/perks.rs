//! Unit tests for the pure perk kernels (M9 wave 2 #313). Expected values
//! are pinned to perks probed from the real GOTY `Fallout3.esm`: Swift
//! Learner (00031DD3), Educated (00031DD8), and Strong Back (00031DDE).

use super::*;
use crate::actor_state::ActorSkill;
use crate::stats::GmstSettings;

/// Swift Learner as decoded from the real ESM: level 2, 3 ranks, INT 4
/// gate (AV index 9), and XP-multiplier entry points 1.1/1.2/1.3 at
/// 0-based ranks 0..2.
fn swift_learner() -> PerkDefinition {
    PerkDefinition {
        form_id: 0x0003_1dd3,
        editor_id: "SwiftLearner".into(),
        name: Some("Swift Learner".into()),
        min_level: 2,
        ranks: 3,
        playable: true,
        conditions: vec![PerkCondition {
            actor_value: ActorValue::Special(SpecialAttribute::Intelligence),
            threshold: 4,
        }],
        entries: [1.1_f32, 1.2, 1.3]
            .into_iter()
            .enumerate()
            .map(|(rank, value)| PerkEntry::EntryPoint {
                rank: rank as u8,
                code: ENTRY_CODE_XP_AWARD_MULTIPLIER,
                param_count: 3,
                priority: 0,
                payload: EntryPointPayload::Value(value),
            })
            .collect(),
        ..PerkDefinition::default()
    }
}

/// Educated as decoded from the real ESM: level 4, 1 rank, INT 4 gate,
/// and a +3.0 bonus-skill-points entry point.
fn educated() -> PerkDefinition {
    PerkDefinition {
        form_id: 0x0003_1dd8,
        editor_id: "Educated".into(),
        min_level: 4,
        ranks: 1,
        playable: true,
        conditions: vec![PerkCondition {
            actor_value: ActorValue::Special(SpecialAttribute::Intelligence),
            threshold: 4,
        }],
        entries: vec![PerkEntry::EntryPoint {
            rank: 0,
            code: ENTRY_CODE_BONUS_SKILL_POINTS,
            param_count: 2,
            priority: 0,
            payload: EntryPointPayload::Value(3.0),
        }],
        ..PerkDefinition::default()
    }
}

/// Strong Back as decoded from the real ESM: level 8, 1 rank, STR 5 and
/// END 5 gates (AV indices 5 and 7).
fn strong_back() -> PerkDefinition {
    PerkDefinition {
        form_id: 0x0003_1dde,
        editor_id: "StrongBack".into(),
        min_level: 8,
        ranks: 1,
        playable: true,
        conditions: vec![
            PerkCondition {
                actor_value: ActorValue::Special(SpecialAttribute::Strength),
                threshold: 5,
            },
            PerkCondition {
                actor_value: ActorValue::Special(SpecialAttribute::Endurance),
                threshold: 5,
            },
        ],
        ..PerkDefinition::default()
    }
}

fn defs() -> BTreeMap<u32, PerkDefinition> {
    [swift_learner(), educated(), strong_back()]
        .into_iter()
        .map(|def| (def.form_id, def))
        .collect()
}

fn sheet() -> CharacterSheet {
    CharacterSheet::default()
}

#[test]
fn av_condition_indices_match_the_probed_engine_enum() {
    // Hard facts probed from Fallout3.esm perk CTDAs paired with their
    // published requirements (see the mapping's doc comment).
    let special = [
        (5, SpecialAttribute::Strength),
        (6, SpecialAttribute::Perception),
        (7, SpecialAttribute::Endurance),
        (8, SpecialAttribute::Charisma),
        (9, SpecialAttribute::Intelligence),
        (10, SpecialAttribute::Agility),
        (11, SpecialAttribute::Luck),
    ];
    for (index, attribute) in special {
        assert_eq!(
            actor_value_from_condition_index(index),
            Some(ActorValue::Special(attribute)),
            "index {index}"
        );
    }
    let skills = [
        (32, ActorSkill::Barter),
        (33, ActorSkill::BigGuns),
        (34, ActorSkill::EnergyWeapons),
        (35, ActorSkill::Explosives),
        (36, ActorSkill::Lockpick),
        (37, ActorSkill::Medicine),
        (38, ActorSkill::MeleeWeapons),
        (39, ActorSkill::Repair),
        (40, ActorSkill::Science),
        (41, ActorSkill::SmallGuns),
        (42, ActorSkill::Sneak),
        (43, ActorSkill::Speech),
        (44, ActorSkill::Throwing),
        (45, ActorSkill::Unarmed),
    ];
    for (index, skill) in skills {
        assert_eq!(
            actor_value_from_condition_index(index),
            Some(ActorValue::Skill(skill)),
            "index {index}"
        );
    }
    // Unmapped indices stay unresolved rather than guessed: the SPECIAL
    // and skill blocks, derived/condition values, and the 0x3E8+ AVIF
    // FormIDs are all outside the probed enum range.
    for index in [0, 4, 12, 31, 46, 100, 0x3E8, 1000] {
        assert_eq!(
            actor_value_from_condition_index(index),
            None,
            "index {index}"
        );
    }
}

#[test]
fn level_and_rank_gates_block_eligibility() {
    let progression = PerkProgression::default();
    // Level 1 sheet vs Swift Learner's level-2 gate.
    let blocked = can_take_perk(&sheet(), &swift_learner(), &progression);
    assert_eq!(
        blocked.reasons(),
        &[PerkBlockReason::MinLevel {
            required: 2,
            current: 1
        }]
    );
    // At level 2 with INT 5 the perk is eligible.
    let mut leveled = sheet();
    leveled.level = 2;
    assert!(can_take_perk(&leveled, &swift_learner(), &progression).is_eligible());
    // Owning all ranks blocks with the rank reason.
    let mut maxed = PerkProgression::default();
    maxed.set_rank(0x0003_1dd3, 3);
    let reasons = can_take_perk(&leveled, &swift_learner(), &maxed)
        .reasons()
        .to_vec();
    assert_eq!(reasons, vec![PerkBlockReason::MaxRanksReached { ranks: 3 }]);
    // A perk with zero ranks can never be taken.
    let broken = PerkDefinition {
        ranks: 0,
        ..swift_learner()
    };
    assert_eq!(
        can_take_perk(&leveled, &broken, &progression).reasons(),
        &[PerkBlockReason::MaxRanksReached { ranks: 0 }]
    );
}

#[test]
fn actor_value_conditions_evaluate_against_the_sheet() {
    // Strong Back needs STR 5 and END 5; the all-5 sheet passes.
    let mut leveled = sheet();
    leveled.level = 8;
    assert!(can_take_perk(&leveled, &strong_back(), &PerkProgression::default()).is_eligible());
    // Dropping Endurance below 5 blocks with a condition reason.
    leveled.set_special(SpecialAttribute::Endurance, 4);
    let blocked = can_take_perk(&leveled, &strong_back(), &PerkProgression::default());
    let reasons = blocked.reasons();
    assert_eq!(
        reasons,
        &[PerkBlockReason::ConditionNotMet {
            actor_value: ActorValue::Special(SpecialAttribute::Endurance),
            required: 5,
            actual: 4,
        }]
    );
    // Skill gates read the effective skill value (base 15 at all-5
    // SPECIAL for Perception-governed Lockpick).
    let mut gated = PerkDefinition {
        form_id: 0x0004_4cb0,
        editor_id: "Infiltrator".into(),
        min_level: 1,
        ranks: 1,
        playable: true,
        conditions: vec![PerkCondition {
            actor_value: ActorValue::Skill(ActorSkill::Lockpick),
            threshold: 50,
        }],
        ..PerkDefinition::default()
    };
    gated.conditions[0].threshold = 15;
    assert!(
        can_take_perk(&sheet(), &gated, &PerkProgression::default()).is_eligible(),
        "lockpick base 15 meets the 15 threshold"
    );
    gated.conditions[0].threshold = 16;
    assert!(!can_take_perk(&sheet(), &gated, &PerkProgression::default()).is_eligible());
}

#[test]
fn unknown_conditions_block_eligibility() {
    let mut spooky = swift_learner();
    spooky.min_level = 1;
    spooky.unknown_conditions = 1;
    assert_eq!(
        can_take_perk(&sheet(), &spooky, &PerkProgression::default()).reasons(),
        &[PerkBlockReason::UnknownCondition]
    );
    // A condition on an actor value the sheet cannot supply (e.g. a
    // derived value sneaking into a definition) also blocks.
    let mut derived = swift_learner();
    derived.min_level = 1;
    derived.conditions = vec![PerkCondition {
        actor_value: ActorValue::Health,
        threshold: 100,
    }];
    assert_eq!(
        can_take_perk(&sheet(), &derived, &PerkProgression::default()).reasons(),
        &[PerkBlockReason::UnknownCondition]
    );
}

#[test]
fn owned_rank_grants_only_that_ranks_entry() {
    let defs = defs();
    let mut progression = PerkProgression::default();
    // No perks: neutral modifiers.
    assert_eq!(
        active_perk_modifiers(&progression, &defs),
        PerkModifiers::default()
    );
    // Swift Learner rank 2 -> the 1.2 entry of the OWNED rank, not
    // 1.1 * 1.2.
    progression.set_rank(0x0003_1dd3, 2);
    assert_eq!(
        active_perk_modifiers(&progression, &defs).xp_award_multiplier_bps,
        12_000
    );
    // Rank 3 replaces rank 2; Educated's +3 adds independently.
    progression.set_rank(0x0003_1dd3, 3);
    assert_eq!(
        active_perk_modifiers(&progression, &defs).xp_award_multiplier_bps,
        13_000
    );
    progression.set_rank(0x0003_1dd8, 1);
    let modifiers = active_perk_modifiers(&progression, &defs);
    assert_eq!(modifiers.xp_award_multiplier_bps, 13_000);
    assert_eq!(modifiers.bonus_skill_points, 3);
    // Ranks held for perks missing from the definitions are ignored.
    progression.set_rank(0xdead_beef, 1);
    assert_eq!(
        active_perk_modifiers(&progression, &defs),
        active_perk_modifiers(
            &{
                let mut trimmed = progression.clone();
                trimmed.set_rank(0xdead_beef, 0);
                trimmed
            },
            &defs
        )
    );
}

#[test]
fn progression_set_rank_removes_at_zero() {
    let mut progression = PerkProgression::default();
    assert_eq!(progression.rank(0x0003_1dd3), 0);
    progression.set_rank(0x0003_1dd3, 1);
    assert_eq!(progression.rank(0x0003_1dd3), 1);
    progression.set_rank(0x0003_1dd3, 2);
    progression.set_rank(0x0003_1dd3, 0);
    assert_eq!(progression.rank(0x0003_1dd3), 0);
    assert!(progression.is_empty());
}

fn entry_point(rank: u8, code: u8, payload: EntryPointPayload) -> PerkEntry {
    PerkEntry::EntryPoint {
        rank,
        code,
        param_count: 0,
        priority: 0,
        payload,
    }
}

#[test]
fn modifier_rounding_saturates_and_reasons_have_stable_kinds() {
    // Two stacked 1.1x multipliers from distinct perks: 1.21 -> 12 100
    // bps exactly; an enormous product saturates at u32::MAX.
    let mut doubles = BTreeMap::new();
    for form_id in [0x1_u32, 0x2] {
        doubles.insert(
            form_id,
            PerkDefinition {
                form_id,
                min_level: 1,
                ranks: 1,
                entries: vec![entry_point(
                    0,
                    ENTRY_CODE_XP_AWARD_MULTIPLIER,
                    EntryPointPayload::Value(1.1),
                )],
                ..PerkDefinition::default()
            },
        );
    }
    let mut progression = PerkProgression::default();
    progression.set_rank(0x1, 1);
    progression.set_rank(0x2, 1);
    assert_eq!(
        active_perk_modifiers(&progression, &doubles).xp_award_multiplier_bps,
        12_100
    );
    doubles.get_mut(&0x2).unwrap().entries[0] = entry_point(
        0,
        ENTRY_CODE_XP_AWARD_MULTIPLIER,
        EntryPointPayload::Value(1.0e30),
    );
    assert_eq!(
        active_perk_modifiers(&progression, &doubles).xp_award_multiplier_bps,
        u32::MAX
    );
    // Raw payloads are not interpreted as values.
    doubles.insert(
        0x3,
        PerkDefinition {
            form_id: 0x3,
            min_level: 1,
            ranks: 1,
            entries: vec![entry_point(
                0,
                ENTRY_CODE_BONUS_SKILL_POINTS,
                EntryPointPayload::Raw(100),
            )],
            ..PerkDefinition::default()
        },
    );
    progression.set_rank(0x3, 1);
    assert_eq!(
        active_perk_modifiers(&progression, &doubles).bonus_skill_points,
        0
    );
    for value in [-1.0, f32::NAN, f32::INFINITY] {
        doubles.insert(
            0x4,
            PerkDefinition {
                form_id: 0x4,
                min_level: 1,
                ranks: 1,
                entries: vec![entry_point(
                    0,
                    ENTRY_CODE_XP_AWARD_MULTIPLIER,
                    EntryPointPayload::Value(value),
                )],
                ..PerkDefinition::default()
            },
        );
        progression.set_rank(0x1, 0);
        progression.set_rank(0x2, 0);
        progression.set_rank(0x4, 1);
        assert_eq!(
            active_perk_modifiers(&progression, &doubles).xp_award_multiplier_bps,
            10_000
        );
        progression.set_rank(0x4, 0);
    }
    // Stable machine kinds for the console surface.
    assert_eq!(
        PerkBlockReason::MinLevel {
            required: 2,
            current: 1
        }
        .kind(),
        "min_level"
    );
    assert_eq!(
        PerkBlockReason::MaxRanksReached { ranks: 3 }.kind(),
        "max_ranks"
    );
    assert_eq!(
        PerkBlockReason::ConditionNotMet {
            actor_value: ActorValue::Special(SpecialAttribute::Strength),
            required: 5,
            actual: 4
        }
        .kind(),
        "condition"
    );
    assert_eq!(
        PerkBlockReason::UnknownCondition.kind(),
        "unknown_condition"
    );
}

#[test]
fn perk_definitions_and_progression_round_trip_through_serde() {
    let defs = defs();
    let mut progression = PerkProgression::default();
    progression.set_rank(0x0003_1dd3, 2);
    let encoded = ron::to_string(&(&defs, &progression)).expect("serialize perks");
    let (decoded_defs, decoded_progression): (BTreeMap<u32, PerkDefinition>, PerkProgression) =
        ron::from_str(&encoded).expect("deserialize perks");
    assert_eq!(decoded_defs, defs);
    assert_eq!(decoded_progression, progression);
    // A catalog entry round-trips with serde-defaulted fields intact.
    let educated_def = decoded_defs.get(&0x0003_1dd8).unwrap();
    assert_eq!(educated_def.entries.len(), 1);
    assert_eq!(
        educated_def.entries[0],
        PerkEntry::EntryPoint {
            rank: 0,
            code: ENTRY_CODE_BONUS_SKILL_POINTS,
            param_count: 2,
            priority: 0,
            payload: EntryPointPayload::Value(3.0),
        }
    );
    // The leveling kernels consume the modifiers end to end: Educated's
    // +3 lifts the per-level points at INT 5 from 15 to 18, and Swift
    // Learner rank 2 scales a 1000 XP award to 1200.
    let plain = sheet();
    assert_eq!(
        crate::stats::skill_points_per_level(&plain, &GmstSettings::default(), 3),
        18
    );
    let mut earning = sheet();
    crate::stats::award_xp(&mut earning, 1_000, 12_000, &GmstSettings::default());
    assert_eq!(earning.xp, 1_200);
}
