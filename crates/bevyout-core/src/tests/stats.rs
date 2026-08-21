//! Unit tests for the pure SPECIAL/skill/derived/leveling kernels (M9 W1
//! #309). Expected values are pinned against the Fallout 3 GOTY defaults
//! documented in `docs/plans/M9_Start.md` and the wave 1 plan.

use super::*;

fn sheet() -> CharacterSheet {
    CharacterSheet::default()
}

fn settings() -> GmstSettings {
    GmstSettings::default()
}

#[test]
fn every_fo3_skill_maps_to_its_primary_special() {
    let cases = [
        (ActorSkill::Barter, SpecialAttribute::Charisma),
        (ActorSkill::BigGuns, SpecialAttribute::Endurance),
        (ActorSkill::EnergyWeapons, SpecialAttribute::Perception),
        (ActorSkill::Explosives, SpecialAttribute::Perception),
        (ActorSkill::Lockpick, SpecialAttribute::Perception),
        (ActorSkill::Medicine, SpecialAttribute::Intelligence),
        (ActorSkill::MeleeWeapons, SpecialAttribute::Strength),
        (ActorSkill::Repair, SpecialAttribute::Intelligence),
        (ActorSkill::Science, SpecialAttribute::Intelligence),
        (ActorSkill::SmallGuns, SpecialAttribute::Agility),
        (ActorSkill::Sneak, SpecialAttribute::Agility),
        (ActorSkill::Speech, SpecialAttribute::Charisma),
        (ActorSkill::Unarmed, SpecialAttribute::Endurance),
    ];
    assert_eq!(cases.len(), FO3_SKILLS.len());
    for (skill, attribute) in cases {
        assert_eq!(
            skill_governing_attribute(skill),
            Some(attribute),
            "{skill:?} governing attribute"
        );
    }
    assert_eq!(skill_governing_attribute(ActorSkill::Throwing), None);
}

#[test]
fn skill_base_formula_uses_two_plus_primary_and_rounded_luck_up() {
    let mut world = sheet();
    // All SPECIAL 5, luck 5: 2 + 2*5 + ceil(5/2)=3 -> 15.
    assert_eq!(
        world.skill_base(ActorSkill::Lockpick),
        15,
        "lockpick base at all-5 SPECIAL"
    );
    // Even luck rounds down, odd luck rounds up.
    world.set_special(SpecialAttribute::Luck, 4);
    assert_eq!(luck_skill_bonus(4), 2);
    world.set_special(SpecialAttribute::Luck, 5);
    assert_eq!(luck_skill_bonus(5), 3);
    // Luck 10, perception 1: 2 + 2 + 5 = 9.
    world.set_special(SpecialAttribute::Luck, 10);
    world.set_special(SpecialAttribute::Perception, 1);
    assert_eq!(world.skill_base(ActorSkill::Lockpick), 9);
}

#[test]
fn tagged_skills_gain_the_flat_bonus() {
    let mut world = sheet();
    world.set_special(SpecialAttribute::Luck, 10);
    assert!(!world.tagged_skills.contains(&ActorSkill::SmallGuns));
    assert_eq!(world.skill_base(ActorSkill::SmallGuns), 17);
    world.tagged_skills.insert(ActorSkill::SmallGuns);
    assert_eq!(world.skill_base(ActorSkill::SmallGuns), 32);
    assert_eq!(world.skill_value(ActorSkill::SmallGuns), 32);
}

#[test]
fn derived_attributes_match_the_gmst_formula() {
    let world = sheet();
    let derived = world.derived(&settings());
    assert_eq!(derived.max_health, 200.0);
    assert_eq!(derived.max_action_points, 75.0);
    assert_eq!(derived.carry_weight, 200.0);
    assert_eq!(derived.critical_chance_bps, 500);
}

#[test]
fn derived_attributes_grow_with_level_and_special() {
    let mut world = sheet();
    world.level = 4;
    world.set_special(SpecialAttribute::Endurance, 8);
    world.set_special(SpecialAttribute::Strength, 7);
    let derived = world.derived(&settings());
    assert_eq!(derived.max_health, 290.0);
    assert_eq!(derived.carry_weight, 220.0);
}

#[test]
fn gmst_settings_override_from_pairs_and_skip_bad_values() {
    let pairs = vec![
        (GMST_HEALTH_ENDURANCE_MULT, GmstValue::Float(25.0)),
        ("iMaxPlayerLevel", GmstValue::Int(20)),
        ("fAVDHealthLevelMult", GmstValue::Float(f32::NAN)),
        ("iXPBase", GmstValue::Int(300)),
        (
            "fAVDCarryWeightsBase",
            GmstValue::Str("not a number".into()),
        ),
    ];
    let overridden = GmstSettings::from_pairs(pairs.into_iter());
    assert_eq!(overridden.health_endurance_mult, 25.0);
    assert_eq!(overridden.max_player_level, 20);
    // NaN skipped, string ignored, name matching is case-insensitive.
    assert_eq!(overridden.health_level_mult, 10.0);
    assert_eq!(overridden.carry_weight_base, 150.0);
    assert_eq!(overridden.xp_base, 300);
    assert_ne!(overridden, GmstSettings::default());
}

#[test]
fn gmst_settings_validation_rejects_non_finite_and_out_of_range() {
    let mut bad = settings();
    bad.health_base = f32::INFINITY;
    assert!(matches!(
        bad.validate(),
        Err(StatsError::NonFiniteSetting("health_base"))
    ));
    let mut negative = settings();
    negative.carry_weight_mult = -1.0;
    assert!(matches!(
        negative.validate(),
        Err(StatsError::NegativeSetting("carry_weight_mult"))
    ));
    let mut cap = settings();
    cap.xp_base = 0;
    assert!(matches!(
        cap.validate(),
        Err(StatsError::XpBaseOutOfRange(0))
    ));
    assert!(settings().validate().is_ok());
}

#[test]
fn xp_thresholds_follow_the_quadratic_curve() {
    let settings = settings();
    assert_eq!(xp_threshold(1, &settings), 0);
    assert_eq!(xp_threshold(2, &settings), 200);
    assert_eq!(xp_threshold(3, &settings), 550);
    assert_eq!(xp_threshold(4, &settings), 1_050);
    assert_eq!(xp_threshold(30, &settings), 66_700);
}

#[test]
fn awarding_threshold_xp_levels_up_once_with_skill_points() {
    let mut world = sheet();
    let outcome = award_xp(&mut world, 200, 10_000, &settings());
    assert_eq!(outcome.levels_gained, 1);
    assert_eq!(outcome.level, 2);
    assert_eq!(outcome.xp, 200);
    assert_eq!(world.level, 2);
    assert_eq!(world.xp_into_level(&settings()), 0);
    // 11 base + (intelligence 5 - 1) * 1.
    assert_eq!(outcome.skill_points_gained, 15);
}

#[test]
fn one_award_can_cross_several_thresholds() {
    let mut world = sheet();
    let outcome = award_xp(&mut world, 700, 10_000, &settings());
    assert_eq!(outcome.level, 3);
    assert_eq!(outcome.levels_gained, 2);
    assert_eq!(world.xp_into_level(&settings()), 150);
    assert_eq!(outcome.skill_points_gained, 30);
}

#[test]
fn xp_accumulation_stops_at_the_level_cap() {
    let mut world = sheet();
    let outcome = award_xp(&mut world, 999_999, 10_000, &settings());
    assert_eq!(outcome.level, 30);
    assert_eq!(outcome.xp, 66_700);
    assert!(world.xp <= xp_threshold(30, &settings()));
    // Further awards are inert.
    let more = award_xp(&mut world, 5_000, 10_000, &settings());
    assert_eq!(more.level, 30);
    assert_eq!(more.xp, 66_700);
}

#[test]
fn a_lower_level_cap_stops_progression_early() {
    let mut world = sheet();
    let mut low_cap = settings();
    low_cap.max_player_level = 2;
    let outcome = award_xp(&mut world, 5_000, 10_000, &low_cap);
    assert_eq!(outcome.level, 2);
    assert_eq!(outcome.xp, 200);
}

#[test]
fn special_values_clamp_into_one_to_ten() {
    let mut world = sheet();
    assert_eq!(world.set_special(SpecialAttribute::Strength, 42), 10);
    assert_eq!(world.mod_special(SpecialAttribute::Strength, 20), 10);
    assert_eq!(world.mod_special(SpecialAttribute::Strength, -30), 1);
    assert_eq!(world.set_special(SpecialAttribute::Strength, 0), 1);
    assert_eq!(world.effective_special(SpecialAttribute::Strength), 1);
}

#[test]
fn skill_values_clamp_into_zero_to_one_hundred() {
    let mut world = sheet();
    // Science base at all-5 SPECIAL is 15.
    assert_eq!(world.skill_base(ActorSkill::Science), 15);
    world.add_skill_points(ActorSkill::Science, 95);
    assert_eq!(world.skill_value(ActorSkill::Science), 100);
    // Increases cannot go negative or above 100.
    world.add_skill_points(ActorSkill::Science, -500);
    assert_eq!(world.skill_value(ActorSkill::Science), 15);
}

#[test]
fn resistances_capped_at_eighty_five_percent() {
    assert_eq!(clamp_resistance_bps(9_900), 8_500);
    assert_eq!(clamp_resistance_bps(3_000), 3_000);
    assert_eq!(base_poison_rad_resistance_bps(5), 2_000);
    assert_eq!(base_poison_rad_resistance_bps(1), 0);
    assert_eq!(base_poison_rad_resistance_bps(10), 4_500);
}

#[test]
fn skill_gates_compare_against_the_requirement() {
    assert!(skill_gate_passes(50, 50));
    assert!(skill_gate_passes(75, 50));
    assert!(!skill_gate_passes(49, 50));
}

#[test]
fn skill_points_per_level_follow_base_plus_intelligence_minus_one() {
    let mut world = sheet();
    world.set_special(SpecialAttribute::Intelligence, 1);
    assert_eq!(skill_points_per_level(&world, &settings(), 0), 11);
    world.set_special(SpecialAttribute::Intelligence, 5);
    assert_eq!(skill_points_per_level(&world, &settings(), 0), 15);
    world.set_special(SpecialAttribute::Intelligence, 10);
    assert_eq!(skill_points_per_level(&world, &settings(), 0), 20);
}

#[test]
fn perk_modifiers_scale_award_xp_and_bonus_skill_points() {
    // M9 wave 2 (#313): the two perk hook points into the kernels. Swift
    // Learner rank 1 multiplies awarded XP by 1.1 (11 000 bps): 1000 XP
    // becomes 1100, and the level clamp still applies afterwards.
    let mut world = sheet();
    let outcome = award_xp(&mut world, 1_000, 11_000, &settings());
    assert_eq!(world.xp, 1_100);
    assert_eq!(outcome.level, 4);
    // Educated adds three points per level on top of the sheet math.
    assert_eq!(skill_points_per_level(&world, &settings(), 3), 18);
    // The multiplier saturates rather than wrapping on absurd values.
    let mut huge = sheet();
    award_xp(&mut huge, u32::MAX, u32::MAX, &settings());
    assert!(huge.xp <= xp_threshold(30, &settings()));
}

#[test]
fn character_sheet_round_trips_through_serde() {
    let mut world = sheet();
    world.set_special(SpecialAttribute::Endurance, 8);
    world.tagged_skills.insert(ActorSkill::SmallGuns);
    world.add_skill_points(ActorSkill::Sneak, 12);
    award_xp(&mut world, 700, 10_000, &settings());
    let encoded = ron::to_string(&world).expect("serialize sheet");
    let decoded: CharacterSheet = ron::from_str(&encoded).expect("deserialize sheet");
    assert_eq!(decoded, world);
    assert_eq!(decoded.level, 3);
}

#[test]
fn default_sheet_and_settings_are_the_documented_goty_values() {
    let defaults = settings();
    assert_eq!(defaults.health_base, 100.0);
    assert_eq!(defaults.health_endurance_mult, 20.0);
    assert_eq!(defaults.health_level_mult, 10.0);
    assert_eq!(defaults.action_points_base, 65.0);
    assert_eq!(defaults.action_points_mult, 2.0);
    assert_eq!(defaults.carry_weight_base, 150.0);
    assert_eq!(defaults.carry_weight_mult, 10.0);
    assert_eq!(defaults.max_player_level, 30);
    assert_eq!(defaults.level_up_skill_points_base, 11);
    assert_eq!(defaults.level_up_skill_points_interval, 1);
    assert_eq!(defaults.xp_base, 200);
    assert_eq!(defaults.xp_bump_base, 150);
    let world = sheet();
    for attribute in [
        SpecialAttribute::Strength,
        SpecialAttribute::Perception,
        SpecialAttribute::Endurance,
        SpecialAttribute::Charisma,
        SpecialAttribute::Intelligence,
        SpecialAttribute::Agility,
        SpecialAttribute::Luck,
    ] {
        assert_eq!(world.effective_special(attribute), 5);
    }
    assert_eq!(world.level, 1);
    assert_eq!(world.xp, 0);
}
