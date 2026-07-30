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
