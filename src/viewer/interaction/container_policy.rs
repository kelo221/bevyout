//! Pure container seeding and transfer policy (issue #75).
//!
//! Dependency-free (`std` only, no Bevy) exactly like `viewer::world::policy`
//! -- see that module's doc comment -- so `tests/features.rs` can pull it in
//! verbatim without compiling any Bevy code. It defines its own plain
//! [`SeedEntry`] input shape mirroring the leveled-relevant fields of
//! `vsa::manifest::PreparedInventoryEntry` rather than depending on that
//! type.
//!
//! The #74 leveled-list resolver (`viewer::interaction::leveled::resolve_leveled`)
//! does not exist in this worktree yet. This module takes "resolved stacks
//! for a leveled list" as an input closure (`resolve_leveled: impl FnMut(u32)
//! -> Vec<(u32, i32)>`) rather than calling the resolver directly, so it
//! compiles and is fully testable without agent A's code; the orchestrator
//! wires the real `resolve_leveled` function in at integration.

/// Mirrors the leveled-relevant fields of `PreparedInventoryEntry`: which
/// base record (a concrete item, or -- when `leveled` -- a leveled list's
/// form id) this container's manifest entry names, and how many.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct SeedEntry {
    pub(crate) base_form_id: u32,
    pub(crate) count: i32,
    pub(crate) leveled: bool,
}

/// A container's runtime inventory per the fixed seam: `stacks` as
/// `(base_form_id, count)` pairs, and whether its leveled entries (if any)
/// have already been rolled this session.
#[derive(Debug, Clone, Default, PartialEq)]
pub(crate) struct ContainerState {
    pub(crate) stacks: Vec<(u32, i32)>,
    pub(crate) resolved: bool,
}

/// Why a transfer operation was rejected (F75.1: zero/negative counts, or
/// asking to move more than the source stack holds).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum TransferError {
    NonPositiveCount,
    InsufficientSource,
}

fn apply_delta(stacks: &mut Vec<(u32, i32)>, form_id: u32, delta: i32) {
    match stacks.iter().position(|&(id, _)| id == form_id) {
        Some(index) => {
            stacks[index].1 += delta;
            if stacks[index].1 <= 0 {
                stacks.remove(index);
            }
        }
        None if delta > 0 => stacks.push((form_id, delta)),
        None => {}
    }
}

/// Current count of `form_id` in `stacks`, or 0 if absent.
pub(crate) fn stack_count(stacks: &[(u32, i32)], form_id: u32) -> i32 {
    stacks
        .iter()
        .find(|&&(id, _)| id == form_id)
        .map_or(0, |&(_, count)| count)
}

/// F75.1: seeds a container's runtime state from its prepared inventory
/// entries. Non-leveled entries with a positive count become stacks
/// directly; leveled entries are resolved via `resolve_leveled` (the #74
/// seam, called once per leveled entry) and their resulting `(form_id,
/// count)` pairs are merged in too. Always returns `resolved: true` -- a
/// container with no leveled entries is trivially "resolved" on first open,
/// same as one whose leveled entries were just rolled.
pub(crate) fn seed_container(
    entries: &[SeedEntry],
    mut resolve_leveled: impl FnMut(u32) -> Vec<(u32, i32)>,
) -> ContainerState {
    let mut stacks: Vec<(u32, i32)> = Vec::new();
    for entry in entries {
        if entry.leveled {
            for (form_id, count) in resolve_leveled(entry.base_form_id) {
                if count > 0 {
                    apply_delta(&mut stacks, form_id, count);
                }
            }
        } else if entry.count > 0 {
            apply_delta(&mut stacks, entry.base_form_id, entry.count);
        }
    }
    stacks.sort_unstable_by_key(|&(form_id, _)| form_id);
    ContainerState {
        stacks,
        resolved: true,
    }
}

/// T75.2: the first-open/reopen seam. `existing` is the container's current
/// runtime state, if any (`None` the first time this reference is opened
/// this session). A container whose leveled entries already resolved
/// short-circuits straight back to its current state -- reopening never
/// re-rolls, and `resolve_leveled` is not called again.
pub(crate) fn open_container(
    existing: Option<ContainerState>,
    entries: &[SeedEntry],
    resolve_leveled: impl FnMut(u32) -> Vec<(u32, i32)>,
) -> ContainerState {
    match existing {
        Some(state) if state.resolved => state,
        _ => seed_container(entries, resolve_leveled),
    }
}

/// Removes up to `count` units of `form_id` from `stacks`, returning the
/// amount actually removed (always exactly `count` on success). Rejects
/// non-positive counts and counts exceeding what's available without
/// mutating `stacks`.
pub(crate) fn remove_from_stack(
    stacks: &mut Vec<(u32, i32)>,
    form_id: u32,
    count: i32,
) -> Result<i32, TransferError> {
    if count <= 0 {
        return Err(TransferError::NonPositiveCount);
    }
    if count > stack_count(stacks, form_id) {
        return Err(TransferError::InsufficientSource);
    }
    apply_delta(stacks, form_id, -count);
    Ok(count)
}

/// Adds `count` units of `form_id` to `stacks`. Rejects non-positive counts
/// without mutating `stacks`.
pub(crate) fn add_to_stack(
    stacks: &mut Vec<(u32, i32)>,
    form_id: u32,
    count: i32,
) -> Result<(), TransferError> {
    if count <= 0 {
        return Err(TransferError::NonPositiveCount);
    }
    apply_delta(stacks, form_id, count);
    Ok(())
}

/// Moves `count` units of `form_id` from `source` to `destination`,
/// conserving the total between the two stack lists. The five named
/// transfer ops below (`take_one`, `take_stack`, `take_all`, `store_one`,
/// `store_stack`) are all this same operation with `source`/`destination`
/// chosen per direction.
pub(crate) fn transfer(
    source: &mut Vec<(u32, i32)>,
    destination: &mut Vec<(u32, i32)>,
    form_id: u32,
    count: i32,
) -> Result<i32, TransferError> {
    let moved = remove_from_stack(source, form_id, count)?;
    add_to_stack(destination, form_id, moved).expect("a positive removed count is always addable");
    Ok(moved)
}

pub(crate) fn take_one(
    container: &mut Vec<(u32, i32)>,
    player: &mut Vec<(u32, i32)>,
    form_id: u32,
) -> Result<i32, TransferError> {
    transfer(container, player, form_id, 1)
}

pub(crate) fn take_stack(
    container: &mut Vec<(u32, i32)>,
    player: &mut Vec<(u32, i32)>,
    form_id: u32,
    count: i32,
) -> Result<i32, TransferError> {
    transfer(container, player, form_id, count)
}

pub(crate) fn take_all(
    container: &mut Vec<(u32, i32)>,
    player: &mut Vec<(u32, i32)>,
    form_id: u32,
) -> Result<i32, TransferError> {
    transfer(container, player, form_id, stack_count(container, form_id))
}

pub(crate) fn store_one(
    player: &mut Vec<(u32, i32)>,
    container: &mut Vec<(u32, i32)>,
    form_id: u32,
) -> Result<i32, TransferError> {
    transfer(player, container, form_id, 1)
}

pub(crate) fn store_stack(
    player: &mut Vec<(u32, i32)>,
    container: &mut Vec<(u32, i32)>,
    form_id: u32,
    count: i32,
) -> Result<i32, TransferError> {
    transfer(player, container, form_id, count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn non_leveled_entries_seed_stacks_directly() {
        let entries = [
            SeedEntry {
                base_form_id: 0x10,
                count: 3,
                leveled: false,
            },
            SeedEntry {
                base_form_id: 0x11,
                count: 1,
                leveled: false,
            },
        ];
        let state = seed_container(&entries, |_| Vec::new());
        assert_eq!(state.stacks, vec![(0x10, 3), (0x11, 1)]);
        assert!(state.resolved);
    }

    #[test]
    fn leveled_entries_resolve_through_the_seam_and_merge() {
        let entries = [SeedEntry {
            base_form_id: 0x99,
            count: 1,
            leveled: true,
        }];
        let mut calls = 0;
        let state = seed_container(&entries, |list_form_id| {
            calls += 1;
            assert_eq!(list_form_id, 0x99);
            vec![(0x20, 2)]
        });
        assert_eq!(calls, 1);
        assert_eq!(state.stacks, vec![(0x20, 2)]);
        assert!(state.resolved);
    }

    #[test]
    fn reopen_short_circuits_and_never_calls_the_resolver_again() {
        let entries = [SeedEntry {
            base_form_id: 0x99,
            count: 1,
            leveled: true,
        }];
        let mut calls = 0;
        let first = open_container(None, &entries, |_| {
            calls += 1;
            vec![(0x20, 2)]
        });
        assert_eq!(calls, 1);
        let second = open_container(Some(first.clone()), &entries, |_| {
            calls += 1;
            vec![(0x20, 2)]
        });
        assert_eq!(calls, 1, "reopening a resolved container must not re-roll");
        assert_eq!(second, first);
    }

    #[test]
    fn take_all_empties_the_stack_and_conserves_the_total() {
        let mut container = vec![(0x10, 5)];
        let mut player = vec![];
        let moved = take_all(&mut container, &mut player, 0x10).unwrap();
        assert_eq!(moved, 5);
        assert_eq!(stack_count(&container, 0x10), 0);
        assert_eq!(stack_count(&player, 0x10), 5);
    }

    #[test]
    fn store_into_an_empty_container_conserves_the_total() {
        let mut container = vec![];
        let mut player = vec![(0x10, 4)];
        let moved = store_stack(&mut player, &mut container, 0x10, 4).unwrap();
        assert_eq!(moved, 4);
        assert_eq!(stack_count(&container, 0x10), 4);
        assert_eq!(stack_count(&player, 0x10), 0);
    }

    #[test]
    fn zero_and_negative_counts_are_rejected_without_mutating() {
        let mut container = vec![(0x10, 5)];
        let mut player = vec![];
        assert_eq!(
            take_stack(&mut container, &mut player, 0x10, 0),
            Err(TransferError::NonPositiveCount)
        );
        assert_eq!(
            take_stack(&mut container, &mut player, 0x10, -3),
            Err(TransferError::NonPositiveCount)
        );
        assert_eq!(container, vec![(0x10, 5)]);
        assert!(player.is_empty());
    }

    #[test]
    fn taking_more_than_available_is_rejected() {
        let mut container = vec![(0x10, 2)];
        let mut player = vec![];
        assert_eq!(
            take_stack(&mut container, &mut player, 0x10, 3),
            Err(TransferError::InsufficientSource)
        );
        assert_eq!(container, vec![(0x10, 2)]);
    }

    // F118.1: a corpse uses the same lossless holder policy as a container;
    // the stable reference identity lives in the caller's FormID-keyed
    // runtime map, while this policy owns the stack conservation contract.
    #[test]
    fn corpse_take_all_conserves_every_stack() {
        let mut corpse = vec![(0x10, 5), (0x11, 2)];
        let mut player = vec![(0x10, 1)];
        let before = corpse
            .iter()
            .chain(&player)
            .map(|&(_, count)| count)
            .sum::<i32>();

        let moved = take_all(&mut corpse, &mut player, 0x10).unwrap();

        let after = corpse
            .iter()
            .chain(&player)
            .map(|&(_, count)| count)
            .sum::<i32>();
        assert_eq!(moved, 5);
        assert_eq!(before, after);
        assert_eq!(corpse, vec![(0x11, 2)]);
        assert_eq!(player, vec![(0x10, 6)]);
    }

    // F118.1: a failed corpse transfer is atomic, so a bad take cannot
    // destroy loot or partially credit the player.
    #[test]
    fn corpse_failed_transfer_leaves_both_holders_unchanged() {
        let mut corpse = vec![(0x10, 2)];
        let mut player = vec![(0x10, 1)];
        let before_corpse = corpse.clone();
        let before_player = player.clone();

        assert_eq!(
            take_stack(&mut corpse, &mut player, 0x10, 3),
            Err(TransferError::InsufficientSource)
        );
        assert_eq!(corpse, before_corpse);
        assert_eq!(player, before_player);
    }
}
