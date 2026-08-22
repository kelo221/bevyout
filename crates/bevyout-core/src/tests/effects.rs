use super::*;

use crate::actor_state::{ActorSkill, ActorValue, SpecialAttribute};
use crate::effects::{
    ActiveEffect, ActiveEffectsLedger, EffectSource, PERMANENT_MS, actor_value_from_effect_index,
    projected_special,
};
use crate::stats::CharacterSheet;

fn chem(value: ActorValue, magnitude: f32, remaining_ms: u32) -> ActiveEffect {
    ActiveEffect {
        source: EffectSource::Chem,
        actor_value: value,
        magnitude,
        remaining_ms,
    }
}

#[test]
fn effect_index_mapping_matches_the_probed_engine_table() {
    // GECK Actor Value Codes + real MGEF ground truth (wave 3 probe).
    assert_eq!(
        actor_value_from_effect_index(5),
        Some(ActorValue::Special(SpecialAttribute::Strength))
    );
    assert_eq!(
        actor_value_from_effect_index(11),
        Some(ActorValue::Special(SpecialAttribute::Luck))
    );
    // ChemIncAPJet = 12, RestoreHealthStimpak = 16, IncreaseRadResist = 20,
    // RestoreRadiationLevel = 54.
    assert_eq!(
        actor_value_from_effect_index(12),
        Some(ActorValue::ActionPoints)
    );
    assert_eq!(actor_value_from_effect_index(16), Some(ActorValue::Health));
    assert_eq!(
        actor_value_from_effect_index(20),
        Some(ActorValue::RadResist)
    );
    assert_eq!(actor_value_from_effect_index(54), Some(ActorValue::Rads));
    // IncreaseSkillBarter = 32 ... IncreaseSkillUnarmed = 45.
    assert_eq!(
        actor_value_from_effect_index(32),
        Some(ActorValue::Skill(ActorSkill::Barter))
    );
    assert_eq!(
        actor_value_from_effect_index(45),
        Some(ActorValue::Skill(ActorSkill::Unarmed))
    );
    // Unmapped families stay unresolved: limb conditions (wave 4) and -1.
    assert_eq!(actor_value_from_effect_index(25), None);
    assert_eq!(actor_value_from_effect_index(-1), None);
}

#[test]
fn ledger_merges_reapplication_and_keeps_other_sources() {
    let mut ledger = ActiveEffectsLedger::default();
    ledger.apply(chem(
        ActorValue::Special(SpecialAttribute::Strength),
        2.0,
        240_000,
    ));
    ledger.apply(chem(
        ActorValue::Special(SpecialAttribute::Endurance),
        3.0,
        240_000,
    ));
    assert_eq!(ledger.len(), 2);
    // Re-dosing refreshes the timer and takes the newer magnitude instead
    // of stacking.
    ledger.apply(chem(
        ActorValue::Special(SpecialAttribute::Strength),
        4.0,
        100_000,
    ));
    assert_eq!(ledger.len(), 2);
    assert_eq!(
        ledger.modifier_for(ActorValue::Special(SpecialAttribute::Strength)),
        4.0
    );
    // Same value from another source coexists and sums.
    ledger.apply(ActiveEffect {
        source: EffectSource::Withdrawal,
        actor_value: ActorValue::Special(SpecialAttribute::Strength),
        magnitude: -1.0,
        remaining_ms: PERMANENT_MS,
    });
    assert_eq!(
        ledger.modifier_for(ActorValue::Special(SpecialAttribute::Strength)),
        3.0
    );
}

#[test]
fn tick_expires_timed_entries_and_returns_them_for_withdrawal() {
    let mut ledger = ActiveEffectsLedger::default();
    ledger.apply(chem(
        ActorValue::Special(SpecialAttribute::Agility),
        1.0,
        1_500,
    ));
    ledger.apply(ActiveEffect {
        source: EffectSource::Withdrawal,
        actor_value: ActorValue::Special(SpecialAttribute::Agility),
        magnitude: -1.0,
        remaining_ms: PERMANENT_MS,
    });
    // Partial tick: 1000 ms leaves 500 ms on the timed entry.
    assert!(ledger.tick(1_000).is_empty());
    assert_eq!(ledger.len(), 2);
    let expired = ledger.tick(500);
    assert_eq!(expired.len(), 1);
    assert_eq!(
        expired[0].actor_value,
        ActorValue::Special(SpecialAttribute::Agility)
    );
    // The permanent withdrawal entry survives every tick.
    assert!(ledger.tick(u32::MAX - 1).is_empty());
    assert_eq!(ledger.len(), 1);
    assert_eq!(
        ledger.modifier_for(ActorValue::Special(SpecialAttribute::Agility)),
        -1.0
    );
}

#[test]
fn clear_source_value_removes_only_that_pair() {
    let mut ledger = ActiveEffectsLedger::default();
    ledger.apply(chem(
        ActorValue::Special(SpecialAttribute::Perception),
        5.0,
        240_000,
    ));
    ledger.apply(ActiveEffect {
        source: EffectSource::Withdrawal,
        actor_value: ActorValue::Special(SpecialAttribute::Perception),
        magnitude: -1.0,
        remaining_ms: PERMANENT_MS,
    });
    ledger.clear_source_value(
        EffectSource::Withdrawal,
        ActorValue::Special(SpecialAttribute::Perception),
    );
    assert_eq!(ledger.len(), 1);
    assert_eq!(
        ledger.modifier_for(ActorValue::Special(SpecialAttribute::Perception)),
        5.0
    );
}

#[test]
fn projected_special_combines_sheet_effects_and_radiation() {
    let mut sheet = CharacterSheet::default();
    sheet.set_special(SpecialAttribute::Strength, 5);
    sheet.set_special(SpecialAttribute::Endurance, 5);
    let mut ledger = ActiveEffectsLedger::default();
    // Buffout: STR +2, END +3.
    ledger.apply(chem(
        ActorValue::Special(SpecialAttribute::Strength),
        2.0,
        240_000,
    ));
    ledger.apply(chem(
        ActorValue::Special(SpecialAttribute::Endurance),
        3.0,
        240_000,
    ));
    // 600 rads: -3 END, -2 AGI, -1 STR.
    let projected = projected_special(&sheet, &ledger, 600);
    assert_eq!(projected[&SpecialAttribute::Strength], 6); // 5 + 2 - 1
    assert_eq!(projected[&SpecialAttribute::Endurance], 5); // 5 + 3 - 3
    assert_eq!(projected[&SpecialAttribute::Agility], 3); // 5 - 2
    // Without the chem: rads alone bite; with neither: base returns.
    let rad_only = projected_special(&sheet, &ActiveEffectsLedger::default(), 600);
    assert_eq!(rad_only[&SpecialAttribute::Strength], 4);
    let clean = projected_special(&sheet, &ActiveEffectsLedger::default(), 0);
    assert_eq!(clean[&SpecialAttribute::Strength], 5);
}

#[test]
fn projected_special_clamps_to_one_through_ten() {
    let mut sheet = CharacterSheet::default();
    sheet.set_special(SpecialAttribute::Strength, 10);
    sheet.set_special(SpecialAttribute::Luck, 1);
    let mut ledger = ActiveEffectsLedger::default();
    ledger.apply(chem(
        ActorValue::Special(SpecialAttribute::Strength),
        5.0,
        1_000,
    ));
    ledger.apply(chem(
        ActorValue::Special(SpecialAttribute::Luck),
        -5.0,
        1_000,
    ));
    let projected = projected_special(&sheet, &ledger, 0);
    assert_eq!(projected[&SpecialAttribute::Strength], 10);
    assert_eq!(projected[&SpecialAttribute::Luck], 1);
}

#[test]
fn ingestible_definitions_gate_addiction_on_chance_and_effect() {
    use crate::effects::IngestibleDefinition;
    let jet = IngestibleDefinition {
        addiction_chance_percent: 20.0,
        withdrawal_form_id: 0x0003_3067,
        ..IngestibleDefinition::default()
    };
    assert!(jet.addictive());
    assert_eq!(jet.addiction_chance_bps(), 2_000);
    let no_chance = IngestibleDefinition {
        withdrawal_form_id: 0x0003_3067,
        ..IngestibleDefinition::default()
    };
    assert!(!no_chance.addictive());
    let no_effect = IngestibleDefinition {
        addiction_chance_percent: 20.0,
        ..IngestibleDefinition::default()
    };
    assert!(!no_effect.addictive());
    // Effect durations convert game seconds to whole milliseconds once.
    use crate::effects::IngestibleEffect;
    let ap_boost = IngestibleEffect {
        duration_s: 240,
        ..IngestibleEffect::default()
    };
    assert_eq!(ap_boost.duration_ms(), 240_000);
}
