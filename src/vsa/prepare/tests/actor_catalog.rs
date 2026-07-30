use super::*;

fn npc(form_id: u32) -> ActorRecordInput {
    ActorRecordInput {
        form_id,
        kind: ActorRecordKind::Npc,
        ..ActorRecordInput::default()
    }
}

fn creature(form_id: u32) -> ActorRecordInput {
    ActorRecordInput {
        form_id,
        kind: ActorRecordKind::Creature,
        ..ActorRecordInput::default()
    }
}

fn placement(
    reference_form_id: u32,
    base_form_id: u32,
    kind: ActorRecordKind,
) -> ActorPlacementInput {
    ActorPlacementInput {
        reference_form_id,
        base_form_id,
        kind,
        ..ActorPlacementInput::default()
    }
}

#[test]
fn revision_is_pinned() {
    assert_eq!(
        ACTOR_CATALOG_REVISION,
        "openmw-actors-v7-faction-relation-table"
    );
}

#[test]
fn inherits_a_flagged_group_from_its_template() {
    let mut template = npc(0x10);
    template.traits.race_form_id = Some(0xAA);
    let mut source = npc(0x20);
    source.base_template_form_id = Some(0x10);
    source.template_usage.traits = true;
    source.traits.race_form_id = Some(0xBB); // must be ignored once inherited

    let mut actors = HashMap::new();
    actors.insert(0x10, template);
    actors.insert(0x20, source);
    let inputs = ActorCatalogInputs {
        actors,
        placements: vec![placement(1, 0x20, ActorRecordKind::Npc)],
        ..ActorCatalogInputs::default()
    };

    let catalog = build_actor_catalog(&inputs, "fp");
    let ActorCatalogEntry::Prepared(blueprint) = &catalog.entries[0] else {
        panic!("expected a prepared blueprint");
    };
    assert_eq!(blueprint.race_form_id, Some(0xAA));
    assert!(blueprint.inherited);
    assert_eq!(catalog.counters.inherited, 1);
    assert_eq!(catalog.counters.prepared, 1);
}

#[test]
fn does_not_inherit_an_unflagged_group() {
    let mut template = npc(0x10);
    template.traits.race_form_id = Some(0xAA);
    let mut source = npc(0x20);
    source.base_template_form_id = Some(0x10);
    source.template_usage.stats = true; // traits NOT flagged
    source.traits.race_form_id = Some(0xBB);

    let mut actors = HashMap::new();
    actors.insert(0x10, template);
    actors.insert(0x20, source);
    let inputs = ActorCatalogInputs {
        actors,
        placements: vec![placement(1, 0x20, ActorRecordKind::Npc)],
        ..ActorCatalogInputs::default()
    };

    let catalog = build_actor_catalog(&inputs, "fp");
    let ActorCatalogEntry::Prepared(blueprint) = &catalog.entries[0] else {
        panic!("expected a prepared blueprint");
    };
    assert_eq!(blueprint.race_form_id, Some(0xBB));
}

#[test]
fn diagnoses_a_template_cycle() {
    let mut a = npc(0x10);
    a.base_template_form_id = Some(0x20);
    a.template_usage.traits = true;
    let mut b = npc(0x20);
    b.base_template_form_id = Some(0x10);
    b.template_usage.traits = true;

    let mut actors = HashMap::new();
    actors.insert(0x10, a);
    actors.insert(0x20, b);
    let inputs = ActorCatalogInputs {
        actors,
        placements: vec![placement(1, 0x10, ActorRecordKind::Npc)],
        ..ActorCatalogInputs::default()
    };

    let catalog = build_actor_catalog(&inputs, "fp");
    let ActorCatalogEntry::Prepared(blueprint) = &catalog.entries[0] else {
        panic!("expected a prepared blueprint");
    };
    assert!(
        blueprint
            .diagnostics
            .iter()
            .any(|message| message.contains("template cycle"))
    );
}

#[test]
fn diagnoses_a_missing_template_target() {
    let mut a = npc(0x10);
    a.base_template_form_id = Some(0xDEAD);
    a.template_usage.stats = true;

    let inputs = ActorCatalogInputs {
        actors: HashMap::from([(0x10, a)]),
        placements: vec![placement(1, 0x10, ActorRecordKind::Npc)],
        ..ActorCatalogInputs::default()
    };

    let catalog = build_actor_catalog(&inputs, "fp");
    let ActorCatalogEntry::Prepared(blueprint) = &catalog.entries[0] else {
        panic!("expected a prepared blueprint");
    };
    assert!(
        blueprint
            .diagnostics
            .iter()
            .any(|message| message.contains("unresolved template target 0000dead"))
    );
}

#[test]
fn records_leveled_template_candidates_deterministically() {
    let mut a = npc(0x10);
    a.base_template_form_id = Some(0x99);
    a.template_usage.traits = true;

    let leveled = LeveledInput {
        form_id: 0x99,
        entries: vec![0x30, 0x10, 0x20, 0x10],
    };

    let inputs = ActorCatalogInputs {
        actors: HashMap::from([(0x10, a)]),
        leveled: HashMap::from([(0x99, leveled)]),
        placements: vec![placement(1, 0x10, ActorRecordKind::Npc)],
        ..ActorCatalogInputs::default()
    };

    let catalog = build_actor_catalog(&inputs, "fp");
    let ActorCatalogEntry::Prepared(blueprint) = &catalog.entries[0] else {
        panic!("expected a prepared blueprint");
    };
    assert!(blueprint.is_leveled_template);
    assert_eq!(blueprint.template_candidates, vec![0x10, 0x20, 0x30]);
}

#[test]
fn leveled_template_resolves_flagged_groups_from_one_seeded_actor() {
    let mut shell = npc(0x10);
    shell.base_template_form_id = Some(0x99);
    shell.template_usage.traits = true;
    shell.template_usage.model_animation = true;
    shell.traits.race_form_id = Some(0xAA);
    shell.model_animation.model_path = Some("characters/shell.nif".into());

    let mut concrete = npc(0x20);
    concrete.traits.race_form_id = Some(0xBB);
    concrete.model_animation.model_path = Some("characters/concrete.nif".into());

    let inputs = ActorCatalogInputs {
        actors: HashMap::from([(0x10, shell), (0x20, concrete)]),
        leveled: HashMap::from([(
            0x99,
            LeveledInput {
                form_id: 0x99,
                entries: vec![0x20],
            },
        )]),
        placements: vec![placement(1, 0x10, ActorRecordKind::Npc)],
        races: HashSet::from([0xBB]),
        ..ActorCatalogInputs::default()
    };

    let catalog = build_actor_catalog(&inputs, "fp");
    let ActorCatalogEntry::Prepared(blueprint) = &catalog.entries[0] else {
        panic!("expected a prepared blueprint");
    };
    assert_eq!(blueprint.resolved_base_form_id, Some(0x20));
    assert_eq!(blueprint.race_form_id, Some(0xBB));
    assert_eq!(
        blueprint.model_path.as_deref(),
        Some("characters/concrete.nif")
    );
}

#[test]
fn an_unused_dangling_template_produces_no_leveled_marker() {
    let mut a = npc(0x10);
    a.base_template_form_id = Some(0xDEAD); // dangling, but no flags use it

    let inputs = ActorCatalogInputs {
        actors: HashMap::from([(0x10, a)]),
        placements: vec![placement(1, 0x10, ActorRecordKind::Npc)],
        ..ActorCatalogInputs::default()
    };

    let catalog = build_actor_catalog(&inputs, "fp");
    let ActorCatalogEntry::Prepared(blueprint) = &catalog.entries[0] else {
        panic!("expected a prepared blueprint");
    };
    assert!(!blueprint.is_leveled_template);
    assert!(blueprint.diagnostics.is_empty());
}

#[test]
fn unresolved_race_class_faction_package_links_are_diagnosed() {
    let mut a = npc(0x10);
    a.traits.race_form_id = Some(0x01);
    a.class_form_id = Some(0x02);
    a.factions.push(ActorFactionInput {
        faction_form_id: 0x03,
        rank: 0,
    });
    a.package_form_ids.push(0x04);

    let inputs = ActorCatalogInputs {
        actors: HashMap::from([(0x10, a)]),
        placements: vec![placement(1, 0x10, ActorRecordKind::Npc)],
        ..ActorCatalogInputs::default()
    };

    let catalog = build_actor_catalog(&inputs, "fp");
    let ActorCatalogEntry::Prepared(blueprint) = &catalog.entries[0] else {
        panic!("expected a prepared blueprint");
    };
    for expected in [
        "unresolved race link 00000001",
        "unresolved class link 00000002",
        "unresolved faction link 00000003",
        "unresolved package link 00000004",
    ] {
        assert!(
            blueprint.diagnostics.iter().any(|m| m == expected),
            "missing {expected:?} in {:?}",
            blueprint.diagnostics
        );
    }
}

/// Issue #175 (F175.2): the ESM's authored `PKID` order is FO3's package
/// priority semantics -- `resolve_group!("ai_packages", ..)` must carry
/// it through a template chain verbatim (no sorting), and an actor's own
/// unused `package_form_ids` must not leak through once inherited.
#[test]
fn package_priority_order_is_preserved_through_template_inheritance() {
    let mut template = npc(0x10);
    template.package_form_ids = vec![0x05, 0x01, 0x03];
    let mut source = npc(0x20);
    source.base_template_form_id = Some(0x10);
    source.template_usage.ai_packages = true;
    source.package_form_ids = vec![0x99]; // must be ignored once inherited

    let inputs = ActorCatalogInputs {
        actors: HashMap::from([(0x10, template), (0x20, source)]),
        packages: HashSet::from([0x01, 0x03, 0x05]),
        placements: vec![placement(1, 0x20, ActorRecordKind::Npc)],
        ..ActorCatalogInputs::default()
    };

    let catalog = build_actor_catalog(&inputs, "fp");
    let ActorCatalogEntry::Prepared(blueprint) = &catalog.entries[0] else {
        panic!("expected a prepared blueprint");
    };
    assert_eq!(blueprint.package_form_ids, vec![0x05, 0x01, 0x03]);
    assert!(blueprint.diagnostics.is_empty());
}

/// An actor with no template flag for `ai_packages` keeps its own
/// authored order untouched, even when a template exists for other
/// groups.
#[test]
fn package_priority_order_is_not_sorted_when_not_inherited() {
    let mut a = npc(0x10);
    a.package_form_ids = vec![0x30, 0x10, 0x20];

    let inputs = ActorCatalogInputs {
        actors: HashMap::from([(0x10, a)]),
        packages: HashSet::from([0x10, 0x20, 0x30]),
        placements: vec![placement(1, 0x10, ActorRecordKind::Npc)],
        ..ActorCatalogInputs::default()
    };

    let catalog = build_actor_catalog(&inputs, "fp");
    let ActorCatalogEntry::Prepared(blueprint) = &catalog.entries[0] else {
        panic!("expected a prepared blueprint");
    };
    assert_eq!(blueprint.package_form_ids, vec![0x30, 0x10, 0x20]);
}

#[test]
fn faction_rank_titles_are_resolved_by_sex() {
    let mut a = npc(0x10);
    a.traits.female = true;
    a.factions.push(ActorFactionInput {
        faction_form_id: 0x50,
        rank: 2,
    });
    let faction = FactionInput {
        form_id: 0x50,
        ranks: vec![FactionRankInput {
            rank_number: 2,
            male_title: Some("Paladin".into()),
            female_title: Some("Paladine".into()),
        }],
        ..Default::default()
    };

    let inputs = ActorCatalogInputs {
        actors: HashMap::from([(0x10, a)]),
        factions: HashMap::from([(0x50, faction)]),
        placements: vec![placement(1, 0x10, ActorRecordKind::Npc)],
        ..ActorCatalogInputs::default()
    };

    let catalog = build_actor_catalog(&inputs, "fp");
    let ActorCatalogEntry::Prepared(blueprint) = &catalog.entries[0] else {
        panic!("expected a prepared blueprint");
    };
    assert_eq!(blueprint.factions[0].title.as_deref(), Some("Paladine"));
}

#[test]
fn race_skill_boosts_reach_the_runtime_definition_as_additive_modifiers() {
    let mut actor = ActorRecordInput {
        form_id: 0x10,
        kind: ActorRecordKind::Npc,
        ..Default::default()
    };
    actor.traits.race_form_id = Some(0x20);
    let mut skills = [0; 14];
    skills[0] = 12;
    actor.stats.npc_skill_values = Some(skills);
    let inputs = ActorCatalogInputs {
        actors: HashMap::from([(0x10, actor)]),
        races: HashSet::from([0x20]),
        race_modifiers: HashMap::from([(
            0x20,
            RaceModifierInput {
                skill_modifiers: BTreeMap::from([(ActorValue::Skill(ActorSkill::Barter), 5.0)]),
                ..Default::default()
            },
        )]),
        placements: vec![ActorPlacementInput {
            reference_form_id: 1,
            base_form_id: 0x10,
            kind: ActorRecordKind::Npc,
            ..Default::default()
        }],
        ..Default::default()
    };
    let catalog = build_actor_catalog(&inputs, "fp");
    let ActorCatalogEntry::Prepared(blueprint) = &catalog.entries[0] else {
        panic!("expected prepared actor")
    };
    let definition = blueprint.runtime_definition();
    let state = bevyout_core::actor_state::ActorInstanceState::new(
        1,
        bevyout_core::actor_state::ActorLifeState::Alive,
    );

    assert_eq!(
        definition
            .resolve_value(&state, ActorValue::Skill(ActorSkill::Barter))
            .effective,
        17.0
    );
}

#[test]
fn missing_base_record_is_unresolved_not_silent() {
    let inputs = ActorCatalogInputs {
        placements: vec![placement(1, 0xFFEE, ActorRecordKind::Npc)],
        ..ActorCatalogInputs::default()
    };
    let catalog = build_actor_catalog(&inputs, "fp");
    assert_eq!(catalog.counters.unresolved, 1);
    assert!(matches!(
        &catalog.entries[0],
        ActorCatalogEntry::Unresolved { .. }
    ));
}

#[test]
fn direct_leveled_base_is_skipped_not_silently_defaulted() {
    let leveled = LeveledInput {
        form_id: 0x77,
        entries: vec![0x10, 0x20],
    };
    let inputs = ActorCatalogInputs {
        leveled: HashMap::from([(0x77, leveled)]),
        placements: vec![placement(1, 0x77, ActorRecordKind::Creature)],
        ..ActorCatalogInputs::default()
    };
    let catalog = build_actor_catalog(&inputs, "fp");
    assert_eq!(catalog.counters.skipped, 1);
    assert!(matches!(
        &catalog.entries[0],
        ActorCatalogEntry::Skipped { .. }
    ));
}

#[test]
fn direct_nested_leveled_base_resolves_to_a_concrete_actor() {
    let inputs = ActorCatalogInputs {
        actors: HashMap::from([(0x20, npc(0x20))]),
        leveled: HashMap::from([
            (
                0x77,
                LeveledInput {
                    form_id: 0x77,
                    entries: vec![0x78],
                },
            ),
            (
                0x78,
                LeveledInput {
                    form_id: 0x78,
                    entries: vec![0x20],
                },
            ),
        ]),
        placements: vec![placement(1, 0x77, ActorRecordKind::Npc)],
        ..ActorCatalogInputs::default()
    };

    let catalog = build_actor_catalog(&inputs, "fp");
    let ActorCatalogEntry::Prepared(blueprint) = &catalog.entries[0] else {
        panic!("expected a prepared blueprint");
    };
    assert_eq!(blueprint.source_base_form_id, 0x77);
    assert_eq!(blueprint.resolved_base_form_id, Some(0x20));
    assert_eq!(blueprint.template_candidates, vec![0x78]);
    assert_eq!(catalog.counters.prepared, 1);
    assert_eq!(catalog.counters.skipped, 0);
}

#[test]
fn nested_candidate_resolution_keeps_valid_leaves_and_diagnoses_bad_branches() {
    let actors = HashMap::from([(0x20, npc(0x20)), (0x30, creature(0x30))]);
    let leveled = HashMap::from([
        (
            0x90,
            LeveledInput {
                form_id: 0x90,
                entries: vec![0x91, 0x20],
            },
        ),
        (
            0x91,
            LeveledInput {
                form_id: 0x91,
                entries: vec![0x90, 0x30, 0xDEAD],
            },
        ),
    ]);

    let resolved = resolve_concrete_actor_candidates(0x90, ActorRecordKind::Npc, &actors, &leveled);
    assert_eq!(resolved.form_ids, vec![0x20]);
    for expected in [
        "leveled list cycle at 00000090",
        "leveled candidate 00000030 is Creature, expected Npc",
        "unresolved leveled candidate 0000dead",
    ] {
        assert!(
            resolved
                .diagnostics
                .iter()
                .any(|message| message == expected),
            "missing {expected:?} in {:?}",
            resolved.diagnostics
        );
    }
}

#[test]
fn nested_candidate_selection_is_stable_across_entry_order_and_duplicates() {
    let catalog_for = |root_entries: Vec<u32>, nested_entries: Vec<u32>| {
        let inputs = ActorCatalogInputs {
            actors: HashMap::from([(0x20, npc(0x20)), (0x30, npc(0x30))]),
            leveled: HashMap::from([
                (
                    0x90,
                    LeveledInput {
                        form_id: 0x90,
                        entries: root_entries,
                    },
                ),
                (
                    0x91,
                    LeveledInput {
                        form_id: 0x91,
                        entries: nested_entries,
                    },
                ),
            ]),
            placements: vec![placement(1, 0x90, ActorRecordKind::Npc)],
            ..ActorCatalogInputs::default()
        };
        build_actor_catalog(&inputs, "fp")
    };

    let first = catalog_for(vec![0x91, 0x20, 0x91], vec![0x30, 0x20]);
    let second = catalog_for(vec![0x20, 0x91], vec![0x20, 0x30, 0x20]);
    let resolved = |catalog: &PreparedActorCatalog| {
        let ActorCatalogEntry::Prepared(blueprint) = &catalog.entries[0] else {
            panic!("expected a prepared blueprint");
        };
        blueprint.resolved_base_form_id
    };
    assert_eq!(resolved(&first), resolved(&second));
}

#[test]
fn nested_candidate_resolution_is_depth_bounded() {
    let mut leveled = HashMap::new();
    for depth in 0..=MAX_LEVELED_LIST_DEPTH {
        let form_id = 0x100 + depth as u32;
        leveled.insert(
            form_id,
            LeveledInput {
                form_id,
                entries: vec![form_id + 1],
            },
        );
    }
    let actors = HashMap::from([(
        0x100 + MAX_LEVELED_LIST_DEPTH as u32 + 1,
        npc(0x100 + MAX_LEVELED_LIST_DEPTH as u32 + 1),
    )]);

    let resolved =
        resolve_concrete_actor_candidates(0x100, ActorRecordKind::Npc, &actors, &leveled);
    assert!(resolved.form_ids.is_empty());
    assert!(
        resolved
            .diagnostics
            .iter()
            .any(|message| message.contains("maximum nesting depth 32"))
    );
}

#[test]
fn kind_mismatch_is_unsupported() {
    let creature = ActorRecordInput {
        form_id: 0x10,
        kind: ActorRecordKind::Creature,
        ..ActorRecordInput::default()
    };
    let inputs = ActorCatalogInputs {
        actors: HashMap::from([(0x10, creature)]),
        // ACHR (Npc) pointing at a CREA base.
        placements: vec![placement(1, 0x10, ActorRecordKind::Npc)],
        ..ActorCatalogInputs::default()
    };
    let catalog = build_actor_catalog(&inputs, "fp");
    assert_eq!(catalog.counters.unsupported, 1);
    assert!(matches!(
        &catalog.entries[0],
        ActorCatalogEntry::Unsupported { .. }
    ));
}

#[test]
fn entries_are_sorted_by_base_then_reference_form_id() {
    let mut actors = HashMap::new();
    actors.insert(0x20, npc(0x20));
    actors.insert(0x10, npc(0x10));
    let inputs = ActorCatalogInputs {
        actors,
        placements: vec![
            placement(2, 0x20, ActorRecordKind::Npc),
            placement(1, 0x10, ActorRecordKind::Npc),
            placement(1, 0x20, ActorRecordKind::Npc),
        ],
        ..ActorCatalogInputs::default()
    };
    let catalog = build_actor_catalog(&inputs, "fp");
    let keys: Vec<(u32, u32)> = catalog.entries.iter().map(entry_sort_key).collect();
    assert_eq!(keys, vec![(0x10, 1), (0x20, 1), (0x20, 2)]);
}

#[test]
fn serialization_is_deterministic_across_runs() {
    let mut actors = HashMap::new();
    actors.insert(0x20, npc(0x20));
    actors.insert(0x10, npc(0x10));
    let inputs = ActorCatalogInputs {
        actors,
        placements: vec![
            placement(2, 0x20, ActorRecordKind::Npc),
            placement(1, 0x10, ActorRecordKind::Npc),
        ],
        ..ActorCatalogInputs::default()
    };
    let a = build_actor_catalog(&inputs, "fp");
    let b = build_actor_catalog(&inputs, "fp");
    let ron_a = ron::ser::to_string_pretty(&a, ron::ser::PrettyConfig::default()).unwrap();
    let ron_b = ron::ser::to_string_pretty(&b, ron::ser::PrettyConfig::default()).unwrap();
    assert_eq!(ron_a, ron_b);
}

/// Regression test for the M4 wave 1 acceptance bug: the actor catalog
/// was originally keyed by the content-set source fingerprint like the
/// item/recipe catalogs, so preparing several cells from the same
/// content set overwrote one shared `catalogs/<fp>/actors.ron` and every
/// manifest ended up pointing at the last-prepared cell's actors. Two
/// different cells from the same content set must get distinct per-cell
/// paths, distinct hashes for distinct contents, and both files must
/// survive preparing the other cell.
#[test]
fn two_cells_from_the_same_content_set_get_distinct_catalog_artifacts() {
    let cache_dir = std::env::temp_dir().join(format!(
        "bevyout-actor-catalog-per-cell-{}-{:?}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
    ));

    // Same content set (same fingerprint), different cells: different
    // ACHR placements reference different bases.
    let mut actors = HashMap::new();
    actors.insert(0x10, npc(0x10));
    actors.insert(0x20, npc(0x20));
    let cell_a_inputs = ActorCatalogInputs {
        actors: actors.clone(),
        placements: vec![placement(1, 0x10, ActorRecordKind::Npc)],
        ..ActorCatalogInputs::default()
    };
    let cell_b_inputs = ActorCatalogInputs {
        actors,
        placements: vec![placement(2, 0x20, ActorRecordKind::Npc)],
        ..ActorCatalogInputs::default()
    };
    let catalog_a = build_actor_catalog(&cell_a_inputs, "shared-fp");
    let catalog_b = build_actor_catalog(&cell_b_inputs, "shared-fp");

    let artifact_a = write_actor_catalog(&cache_dir, 0x0000_3a35, &catalog_a).unwrap();
    let artifact_b = write_actor_catalog(&cache_dir, 0x0002_4d6b, &catalog_b).unwrap();

    assert_ne!(artifact_a.relative_path, artifact_b.relative_path);
    assert_eq!(artifact_a.relative_path, "scenes/00003a35/actors.ron");
    assert_eq!(artifact_b.relative_path, "scenes/00024d6b/actors.ron");
    assert_ne!(artifact_a.hash, artifact_b.hash);

    // Writing cell B must not clobber cell A's artifact: A's file still
    // exists, still matches A's recorded hash, and still round-trips to
    // A's own catalog (not B's).
    let bytes_a = std::fs::read(cache_dir.join(&artifact_a.relative_path)).unwrap();
    assert_eq!(fingerprint(&bytes_a), artifact_a.hash);
    let decoded_a: PreparedActorCatalog =
        ron::de::from_str(std::str::from_utf8(&bytes_a).unwrap()).unwrap();
    assert_eq!(decoded_a, catalog_a);

    let bytes_b = std::fs::read(cache_dir.join(&artifact_b.relative_path)).unwrap();
    assert_eq!(fingerprint(&bytes_b), artifact_b.hash);

    // Re-preparing the same cell with identical content reuses the file.
    let artifact_a_again = write_actor_catalog(&cache_dir, 0x0000_3a35, &catalog_a).unwrap();
    assert!(artifact_a_again.reused);
    assert_eq!(artifact_a_again.hash, artifact_a.hash);

    std::fs::remove_dir_all(&cache_dir).unwrap();
}
