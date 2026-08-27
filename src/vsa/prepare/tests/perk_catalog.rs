use super::*;

fn swift_learner() -> PerkDefinition {
    PerkDefinition {
        form_id: 0x0003_1dd3,
        editor_id: "SwiftLearner".into(),
        min_level: 2,
        ranks: 3,
        playable: true,
        conditions: vec![bevyout_core::perks::PerkCondition {
            actor_value: bevyout_core::actor_state::ActorValue::Special(
                bevyout_core::actor_state::SpecialAttribute::Intelligence,
            ),
            threshold: 4,
        }],
        entries: vec![bevyout_core::perks::PerkEntry::EntryPoint {
            rank: 0,
            code: 0x09,
            param_count: 3,
            priority: 0,
            payload: bevyout_core::perks::EntryPointPayload::Value(1.1),
        }],
        ..PerkDefinition::default()
    }
}

#[test]
fn revision_is_pinned() {
    assert_eq!(PERK_CATALOG_REVISION, "openmw-perks-v2");
}

#[test]
fn builds_sorted_entries_with_entry_and_condition_counters() {
    let inputs = PerkCatalogInputs {
        perks: vec![
            swift_learner(),
            PerkDefinition {
                form_id: 0x0004_4cb1,
                editor_id: "IntenseTraining".into(),
                min_level: 2,
                ranks: 10,
                playable: true,
                unknown_conditions: 2,
                entries: vec![
                    bevyout_core::perks::PerkEntry::Quest {
                        rank: 0,
                        quest_form_id: 0x38b2,
                        unknown: 0,
                    },
                    bevyout_core::perks::PerkEntry::Ability {
                        rank: 1,
                        spell_form_id: 0x94ebe,
                    },
                ],
                ..PerkDefinition::default()
            },
            // Out-of-order FormID to prove the sort.
            PerkDefinition {
                form_id: 0x0000_0314,
                editor_id: "HiddenOne".into(),
                hidden: true,
                ..PerkDefinition::default()
            },
        ],
    };
    let catalog = build_perk_catalog(&inputs, "fp");
    assert_eq!(catalog.revision, PERK_CATALOG_REVISION);
    assert_eq!(catalog.source_fingerprint, "fp");
    assert_eq!(
        catalog
            .entries
            .iter()
            .map(|perk| perk.form_id)
            .collect::<Vec<_>>(),
        vec![0x314, 0x0003_1dd3, 0x0004_4cb1]
    );
    assert_eq!(catalog.counters.total, 3);
    assert_eq!(catalog.counters.playable, 2);
    assert_eq!(catalog.counters.hidden, 1);
    assert_eq!(catalog.counters.unknown_conditions, 2);
    assert_eq!(catalog.counters.quest_entries, 1);
    assert_eq!(catalog.counters.ability_entries, 1);
    assert_eq!(catalog.counters.entry_point_entries, 1);
}

#[test]
fn catalog_round_trips_through_ron() {
    let catalog = build_perk_catalog(
        &PerkCatalogInputs {
            perks: vec![swift_learner()],
        },
        "fp",
    );
    let text = ron::ser::to_string_pretty(&catalog, ron::ser::PrettyConfig::default())
        .expect("serialize perk catalog");
    let back: PreparedPerkCatalog = ron::from_str(&text).expect("deserialize perk catalog");
    assert_eq!(back, catalog);
}

#[test]
fn relative_path_is_deterministic_from_fingerprint() {
    let path = PreparedPerkCatalog::relative_path("abc123");
    assert_eq!(
        path.to_string_lossy().replace('\\', "/"),
        "catalogs/abc123/perks.ron"
    );
}
