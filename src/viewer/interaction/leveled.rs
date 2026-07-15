//! Pure deterministic leveled-list resolver (issue #74).
//!
//! This is bevyout's own design -- nothing here is ported from OpenMW (the
//! ESM4 *parsing* of `LVLI`/`LVLN`/`LVLC` records lives in the isolated,
//! attributed `vsa::openmw_esm4` area per `AGENTS.md`; this module only
//! consumes the plain data that parser already produced, reduced to a
//! manifest). Kept dependency-free (std only, no Bevy, no `vsa::manifest`
//! import) exactly like `viewer::world::persist_policy`/`swap_policy` --
//! see those modules' doc comments for the pattern -- so `tests/features.rs`
//! can pull this in verbatim without compiling any Bevy or ESM4 parser code.
//! `PreparedLeveledList`/`PreparedLeveledEntry` below deliberately mirror
//! the runtime-relevant fields of `vsa::manifest::PreparedLeveledList`/
//! `PreparedLeveledEntry` with their own plain types rather than depending
//! on that module; the real call site (`viewer::interaction`, issue #75)
//! converts a manifest's `leveled_lists` map into this shape once per
//! resolve.
//!
//! Resolution algorithm, applied per list: roll `chance_none` first (a miss
//! empties the list); otherwise gather every entry whose `level` is at most
//! the player's level, narrow to the entries at the single highest such
//! level (Bethesda leveled lists always favor the highest unlocked tier),
//! then either keep all of them (`LEVELED_USE_ALL`) or draw exactly one at
//! random. A chosen entry whose `base_form_id` is itself a known list
//! recurses; `LEVELED_CALCULATE_FOR_EACH_ITEM` controls whether a `count`
//! greater than one rolls that recursion independently per unit (each
//! roll potentially producing a different leaf) or rolls once and scales
//! the result. A list FormID absent from `lists` (unresolved content)
//! resolves to nothing rather than erroring.
//!
//! Nothing outside this module's own tests calls it yet: wiring it into a
//! container's first-open path is issue #75's job (see
//! `M3_WAVE2_PLAN.md`'s fixed seams), so `dead_code` is relaxed here rather
//! than reaching into that issue's files, matching
//! `vsa::content_index::mod`'s own "built ahead of its wiring" precedent.
#![allow(dead_code)]

use std::collections::{BTreeMap, HashSet};

/// `LVLF` bit: keep every candidate at the highest unlocked level instead of
/// drawing one at random.
pub(crate) const LEVELED_USE_ALL: u8 = 0x04;
/// `LVLF` bit: a chosen entry's `count` greater than one rolls its
/// recursion independently per unit rather than once-and-scale.
pub(crate) const LEVELED_CALCULATE_FOR_EACH_ITEM: u8 = 0x02;

/// Mirrors `vsa::manifest::PreparedLeveledList` (issue #74 fixed seam) --
/// see the module doc comment for why this is a local plain type rather
/// than an import.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct PreparedLeveledList {
    pub(crate) chance_none: u8,
    pub(crate) flags: u8,
    pub(crate) entries: Vec<PreparedLeveledEntry>,
}

/// Mirrors `vsa::manifest::PreparedLeveledEntry`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct PreparedLeveledEntry {
    pub(crate) level: u16,
    pub(crate) base_form_id: u32,
    pub(crate) count: i32,
}

/// Deterministic seed for one leveled-list resolution stream, mixed from
/// the playthrough's persistent RNG seed plus the cell and reference the
/// roll happens for (F74.4): the same triple always produces the same
/// stream, and different references decorrelate immediately rather than
/// sharing the first draw. Internally a splitmix64-style generator: each
/// draw advances the state and mixes it, so the whole resolution (chance
/// roll, candidate pick, any nested recursion) is one continuous,
/// reproducible stream.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct LeveledSeed(u64);

impl LeveledSeed {
    pub(crate) fn derive(playthrough_seed: u64, cell_form_id: u32, reference_form_id: u32) -> Self {
        let mut state = playthrough_seed;
        state = Self::mix(state ^ (u64::from(cell_form_id).wrapping_mul(0x9E37_79B9_7F4A_7C15)));
        state =
            Self::mix(state ^ (u64::from(reference_form_id).wrapping_mul(0xBF58_476D_1CE4_E5B9)));
        Self(state)
    }

    /// splitmix64's mixing step (Steele, Lea & Flood 2014, public-domain
    /// reference algorithm): three xorshift/multiply rounds turn a
    /// linearly-advancing counter into well-distributed 64-bit output.
    fn mix(mut z: u64) -> u64 {
        z = z.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut x = z;
        x = (x ^ (x >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        x = (x ^ (x >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        x ^ (x >> 31)
    }

    fn next_u64(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        Self::mix(self.0)
    }

    /// A value in `[0, 100)`, used for chance-none rolls.
    fn roll_percent(&mut self) -> u8 {
        (self.next_u64() % 100) as u8
    }

    /// A uniform index in `[0, len)`. `len` must be nonzero.
    fn roll_index(&mut self, len: usize) -> usize {
        (self.next_u64() % len as u64) as usize
    }
}

/// Resolves `list_form_id` against `lists` into `(base_form_id, count)`
/// pairs (the fixed seam, issue #74). Deterministic for identical
/// `(list_form_id, lists, seed, player_level)`; nested lists recurse;
/// cycles are detected against the current recursion chain and skipped
/// (returning without expanding further) rather than looping.
pub(crate) fn resolve_leveled(
    list_form_id: u32,
    lists: &BTreeMap<u32, PreparedLeveledList>,
    seed: LeveledSeed,
    player_level: u16,
) -> Vec<(u32, i32)> {
    let mut rng = seed;
    let mut chain = HashSet::new();
    let mut output = Vec::new();
    resolve_into(
        list_form_id,
        lists,
        &mut rng,
        player_level,
        &mut chain,
        &mut output,
    );
    output
}

fn resolve_into(
    list_form_id: u32,
    lists: &BTreeMap<u32, PreparedLeveledList>,
    rng: &mut LeveledSeed,
    player_level: u16,
    chain: &mut HashSet<u32>,
    output: &mut Vec<(u32, i32)>,
) {
    let Some(list) = lists.get(&list_form_id) else {
        return;
    };
    // Cycle guard: `chain` tracks the current ancestor path, not every list
    // visited overall, so a diamond-shaped (non-cyclic) DAG referencing the
    // same nested list from two different entries still resolves both.
    if !chain.insert(list_form_id) {
        return;
    }
    resolve_body(list, lists, rng, player_level, chain, output);
    chain.remove(&list_form_id);
}

fn resolve_body(
    list: &PreparedLeveledList,
    lists: &BTreeMap<u32, PreparedLeveledList>,
    rng: &mut LeveledSeed,
    player_level: u16,
    chain: &mut HashSet<u32>,
    output: &mut Vec<(u32, i32)>,
) {
    if rng.roll_percent() < list.chance_none.min(100) {
        return;
    }

    let eligible: Vec<&PreparedLeveledEntry> = list
        .entries
        .iter()
        .filter(|entry| entry.level <= player_level)
        .collect();
    let Some(highest_level) = eligible.iter().map(|entry| entry.level).max() else {
        return;
    };
    let candidates: Vec<&PreparedLeveledEntry> = eligible
        .into_iter()
        .filter(|entry| entry.level == highest_level)
        .collect();

    let use_all = list.flags & LEVELED_USE_ALL != 0;
    let each_item = list.flags & LEVELED_CALCULATE_FOR_EACH_ITEM != 0;

    if use_all {
        for entry in candidates {
            emit_entry(entry, lists, rng, player_level, chain, output, each_item);
        }
    } else {
        let index = rng.roll_index(candidates.len());
        emit_entry(
            candidates[index],
            lists,
            rng,
            player_level,
            chain,
            output,
            each_item,
        );
    }
}

fn emit_entry(
    entry: &PreparedLeveledEntry,
    lists: &BTreeMap<u32, PreparedLeveledList>,
    rng: &mut LeveledSeed,
    player_level: u16,
    chain: &mut HashSet<u32>,
    output: &mut Vec<(u32, i32)>,
    each_item: bool,
) {
    let count = entry.count.max(0);
    if count == 0 {
        return;
    }
    if !lists.contains_key(&entry.base_form_id) {
        // A plain item (or a FormID unresolved as a list): emit directly.
        output.push((entry.base_form_id, count));
        return;
    }
    if each_item {
        for _ in 0..count {
            resolve_into(entry.base_form_id, lists, rng, player_level, chain, output);
        }
    } else {
        // One roll; scale whatever leaves it produced by this entry's count
        // rather than rolling independently per unit.
        let start = output.len();
        resolve_into(entry.base_form_id, lists, rng, player_level, chain, output);
        for (_, leaf_count) in output.iter_mut().skip(start) {
            *leaf_count = leaf_count.saturating_mul(count);
        }
    }
}

#[cfg(test)]
mod tests {
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
}
