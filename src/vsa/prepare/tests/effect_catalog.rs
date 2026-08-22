use super::*;

use bevyout_core::actor_state::{ActorValue, SpecialAttribute};
use bevyout_core::effects::{
    ARCHETYPE_VALUE_AND_PARTS, ARCHETYPE_VALUE_MODIFIER, EffectDefinition, IngestibleDefinition,
    IngestibleEffect,
};

fn jet() -> IngestibleDefinition {
    IngestibleDefinition {
        form_id: 0x0001_5164,
        editor_id: "Jet".into(),
        name: Some("Jet".into()),
        value_caps: 20,
        flags: 0x01,
        weight: 0.2,
        withdrawal_form_id: 0x0003_3067,
        addiction_chance_percent: 20.0,
        effects: vec![IngestibleEffect {
            mgef_form_id: 0x0006_6eb8,
            editor_id: "ChemIncAPJet".into(),
            magnitude: 30.0,
            duration_s: 240,
            actor_value: Some(ActorValue::ActionPoints),
            conditioned: false,
        }],
    }
}

fn stimpak() -> IngestibleDefinition {
    IngestibleDefinition {
        form_id: 0x0001_5169,
        editor_id: "Stimpak".into(),
        value_caps: 25,
        flags: 0x05,
        withdrawal_form_id: 0,
        addiction_chance_percent: 0.0,
        effects: vec![
            IngestibleEffect {
                mgef_form_id: 0x0009_fe63,
                editor_id: "RestoreHealthStimpak".into(),
                magnitude: 30.0,
                duration_s: 0,
                actor_value: Some(ActorValue::Health),
                conditioned: false,
            },
            IngestibleEffect {
                mgef_form_id: 0x0009_fe63,
                editor_id: "RestoreHealthStimpak".into(),
                magnitude: 36.0,
                duration_s: 0,
                actor_value: Some(ActorValue::Health),
                conditioned: true,
            },
        ],
        ..IngestibleDefinition::default()
    }
}

#[test]
fn revision_is_pinned() {
    assert_eq!(EFFECT_CATALOG_REVISION, "openmw-effects-v1");
}

#[test]
fn builds_sorted_entries_with_counters() {
    let inputs = EffectCatalogInputs {
        ingestibles: vec![stimpak(), jet()],
        effects: vec![
            EffectDefinition {
                form_id: 0x0006_6eb8,
                editor_id: "ChemIncAPJet".into(),
                flags: 0x72,
                archetype: ARCHETYPE_VALUE_MODIFIER,
                actor_value_index: 12,
                actor_value: Some(ActorValue::ActionPoints),
                ..EffectDefinition::default()
            },
            EffectDefinition {
                form_id: 0x0009_fe63,
                editor_id: "RestoreHealthStimpak".into(),
                flags: 0x70,
                archetype: ARCHETYPE_VALUE_AND_PARTS,
                actor_value_index: 16,
                actor_value: Some(ActorValue::Health),
                ..EffectDefinition::default()
            },
            // Out-of-order FormID to prove the sort; SPECIAL effect keeps
            // its resolved value through the build.
            EffectDefinition {
                form_id: 0x0006_697c,
                editor_id: "ChemIncSTBuffout".into(),
                actor_value_index: 5,
                actor_value: Some(ActorValue::Special(SpecialAttribute::Strength)),
                ..EffectDefinition::default()
            },
        ],
    };
    let catalog = build_effect_catalog(&inputs, "fp");
    assert_eq!(catalog.revision, EFFECT_CATALOG_REVISION);
    assert_eq!(catalog.source_fingerprint, "fp");
    assert_eq!(
        catalog
            .ingestibles
            .iter()
            .map(|ingestible| ingestible.form_id)
            .collect::<Vec<_>>(),
        vec![0x0001_5164, 0x0001_5169]
    );
    assert_eq!(
        catalog
            .effects
            .iter()
            .map(|effect| effect.form_id)
            .collect::<Vec<_>>(),
        vec![0x0006_697c, 0x0006_6eb8, 0x0009_fe63]
    );
    assert_eq!(catalog.counters.ingestibles, 2);
    assert_eq!(catalog.counters.effects, 3);
    assert_eq!(catalog.counters.addictive, 1);
    assert_eq!(catalog.counters.conditioned_effects, 1);
    // Both of Jet's effect items resolve; Stimpak's do too.
    assert_eq!(catalog.counters.unresolved_effects, 0);
}

#[test]
fn catalog_round_trips_through_ron() {
    let catalog = build_effect_catalog(
        &EffectCatalogInputs {
            ingestibles: vec![jet()],
            effects: Vec::new(),
        },
        "fp",
    );
    let text = ron::ser::to_string_pretty(&catalog, ron::ser::PrettyConfig::default())
        .expect("serialize effect catalog");
    let back: PreparedEffectCatalog = ron::from_str(&text).expect("deserialize effect catalog");
    assert_eq!(back, catalog);
}

#[test]
fn relative_path_is_deterministic_from_fingerprint() {
    let path = PreparedEffectCatalog::relative_path("abc123");
    assert_eq!(
        path.to_string_lossy().replace('\\', "/"),
        "catalogs/abc123/effects.ron"
    );
}

#[test]
fn unresolved_effect_items_are_counted_not_dropped() {
    // The engine-builtin UMON monitor (MGEF 0x0000014F) resolves to no
    // decoded record: cataloged with no actor value and counted.
    let inputs = EffectCatalogInputs {
        ingestibles: vec![IngestibleDefinition {
            form_id: 0x0001_5164,
            editor_id: "Jet".into(),
            withdrawal_form_id: 0x0003_3067,
            addiction_chance_percent: 20.0,
            effects: vec![
                IngestibleEffect {
                    mgef_form_id: 0x0006_6eb8,
                    editor_id: "ChemIncAPJet".into(),
                    magnitude: 30.0,
                    duration_s: 240,
                    actor_value: Some(ActorValue::ActionPoints),
                    conditioned: false,
                },
                IngestibleEffect {
                    mgef_form_id: 0x0000_014f,
                    editor_id: String::new(),
                    magnitude: 30.0,
                    duration_s: 108_000,
                    actor_value: None,
                    conditioned: false,
                },
            ],
            ..IngestibleDefinition::default()
        }],
        effects: Vec::new(),
    };
    let catalog = build_effect_catalog(&inputs, "fp");
    assert_eq!(catalog.counters.unresolved_effects, 1);
    assert_eq!(catalog.ingestibles[0].effects.len(), 2);
}
