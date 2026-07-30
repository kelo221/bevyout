use super::*;

fn flat_list(
    chance_none: u8,
    flags: u8,
    entries: Vec<PreparedLeveledEntry>,
) -> PreparedLeveledList {
    PreparedLeveledList {
        chance_none,
        flags,
        entries,
    }
}

fn entry(level: u16, base_form_id: u32, count: i32) -> PreparedLeveledEntry {
    PreparedLeveledEntry {
        level,
        base_form_id,
        count,
    }
}

#[test]
fn same_inputs_resolve_identically_twice() {
    let mut lists = BTreeMap::new();
    lists.insert(
        0x100,
        flat_list(
            0,
            0,
            vec![entry(1, 0x200, 1), entry(5, 0x201, 1), entry(5, 0x202, 1)],
        ),
    );
    let seed = LeveledSeed::derive(42, 0x10, 0x20);
    let first = resolve_leveled(0x100, &lists, seed, 10);
    let second = resolve_leveled(0x100, &lists, seed, 10);
    assert_eq!(first, second);
    assert_eq!(first.len(), 1);
}

#[test]
fn different_reference_form_id_is_an_independent_stream() {
    let mut lists = BTreeMap::new();
    lists.insert(
        0x100,
        flat_list(
            0,
            0,
            vec![
                entry(1, 0x201, 1),
                entry(1, 0x202, 1),
                entry(1, 0x203, 1),
                entry(1, 0x204, 1),
            ],
        ),
    );
    let outcomes: Vec<Vec<(u32, i32)>> = (0..16_u32)
        .map(|reference_form_id| {
            let seed = LeveledSeed::derive(7, 0x10, reference_form_id);
            resolve_leveled(0x100, &lists, seed, 10)
        })
        .collect();
    // Across 16 distinct references the draw must not always land on
    // the same candidate -- otherwise the reference id is not actually
    // folded into the stream.
    assert!(outcomes.windows(2).any(|pair| pair[0] != pair[1]));
}

#[test]
fn different_cell_form_id_is_an_independent_stream() {
    let a = LeveledSeed::derive(7, 0x10, 0x999);
    let b = LeveledSeed::derive(7, 0x11, 0x999);
    assert_ne!(a, b);
}

#[test]
fn chance_none_100_always_resolves_empty() {
    let mut lists = BTreeMap::new();
    lists.insert(0x100, flat_list(100, 0, vec![entry(1, 0x200, 1)]));
    for reference_form_id in 0..8 {
        let seed = LeveledSeed::derive(1, 0x1, reference_form_id);
        assert!(resolve_leveled(0x100, &lists, seed, 50).is_empty());
    }
}

#[test]
fn nested_list_recurses_to_a_leaf_item_and_scales_its_count() {
    let mut lists = BTreeMap::new();
    lists.insert(0x100, flat_list(0, 0, vec![entry(1, 0x101, 3)])); // -> nested list, count 3
    lists.insert(0x101, flat_list(0, 0, vec![entry(1, 0x200, 2)])); // leaf, count 2
    let seed = LeveledSeed::derive(1, 0x1, 0x2);
    let resolved = resolve_leveled(0x100, &lists, seed, 10);
    assert_eq!(resolved, vec![(0x200, 6)]); // 3 (outer) * 2 (leaf)
}

#[test]
fn calculate_for_each_item_rolls_independently_per_count_unit() {
    let mut lists = BTreeMap::new();
    lists.insert(
        0x100,
        flat_list(0, LEVELED_CALCULATE_FOR_EACH_ITEM, vec![entry(1, 0x101, 3)]),
    );
    lists.insert(0x101, flat_list(0, 0, vec![entry(1, 0x200, 1)]));
    let seed = LeveledSeed::derive(1, 0x1, 0x2);
    let resolved = resolve_leveled(0x100, &lists, seed, 10);
    // Three independent rolls of the same single-candidate nested list
    // -- three separate leaf entries, not one entry scaled by 3.
    assert_eq!(resolved, vec![(0x200, 1), (0x200, 1), (0x200, 1)]);
}

#[test]
fn use_all_keeps_every_candidate_at_the_highest_unlocked_level() {
    let mut lists = BTreeMap::new();
    lists.insert(
        0x100,
        flat_list(
            0,
            LEVELED_USE_ALL,
            vec![entry(1, 0x200, 1), entry(5, 0x201, 1), entry(5, 0x202, 1)],
        ),
    );
    let seed = LeveledSeed::derive(1, 0x1, 0x2);
    let mut resolved = resolve_leveled(0x100, &lists, seed, 10);
    resolved.sort();
    assert_eq!(resolved, vec![(0x201, 1), (0x202, 1)]);
}

#[test]
fn only_entries_at_or_below_player_level_are_eligible() {
    let mut lists = BTreeMap::new();
    lists.insert(0x100, flat_list(0, 0, vec![entry(10, 0x200, 1)]));
    let seed = LeveledSeed::derive(1, 0x1, 0x2);
    assert!(resolve_leveled(0x100, &lists, seed, 1).is_empty());
    assert_eq!(resolve_leveled(0x100, &lists, seed, 10), vec![(0x200, 1)]);
}

#[test]
fn self_cycle_terminates_instead_of_looping() {
    let mut lists = BTreeMap::new();
    lists.insert(0x100, flat_list(0, 0, vec![entry(1, 0x100, 1)]));
    let seed = LeveledSeed::derive(1, 0x1, 0x2);
    // Regression: this call must return promptly; a broken cycle guard
    // would hang the test suite instead of failing an assertion.
    assert!(resolve_leveled(0x100, &lists, seed, 10).is_empty());
}

#[test]
fn mutual_cycle_terminates_instead_of_looping() {
    let mut lists = BTreeMap::new();
    lists.insert(0x100, flat_list(0, 0, vec![entry(1, 0x101, 1)]));
    lists.insert(0x101, flat_list(0, 0, vec![entry(1, 0x100, 1)]));
    let seed = LeveledSeed::derive(1, 0x1, 0x2);
    assert!(resolve_leveled(0x100, &lists, seed, 10).is_empty());
}

#[test]
fn diamond_shaped_dag_is_not_treated_as_a_cycle() {
    // A -> [B, C]; B -> D; C -> D. D is reachable twice via independent
    // (non-cyclic) branches and must resolve both times.
    let mut lists = BTreeMap::new();
    lists.insert(
        0xA,
        flat_list(0, LEVELED_USE_ALL, vec![entry(1, 0xB, 1), entry(1, 0xC, 1)]),
    );
    lists.insert(0xB, flat_list(0, 0, vec![entry(1, 0xD, 1)]));
    lists.insert(0xC, flat_list(0, 0, vec![entry(1, 0xD, 1)]));
    lists.insert(0xD, flat_list(0, 0, vec![entry(1, 0x200, 1)]));
    let seed = LeveledSeed::derive(1, 0x1, 0x2);
    let resolved = resolve_leveled(0xA, &lists, seed, 10);
    assert_eq!(resolved, vec![(0x200, 1), (0x200, 1)]);
}

#[test]
fn unresolved_list_form_id_yields_no_entries() {
    let lists = BTreeMap::new();
    let seed = LeveledSeed::derive(1, 0x1, 0x2);
    assert!(resolve_leveled(0xDEAD, &lists, seed, 10).is_empty());
}
