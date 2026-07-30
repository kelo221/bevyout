use super::*;

fn candidate(index: usize, x: f32) -> RevealCandidate {
    RevealCandidate {
        index,
        position: [x, 0.0, 0.0],
    }
}

// T55.1: n entities, budget b => ceil(n/b) chunks.
#[test]
fn chunk_count_is_ceil_of_entities_over_budget() {
    let candidates: Vec<RevealCandidate> = (0..10).map(|i| candidate(i, i as f32)).collect();
    let chunks = plan_reveal_chunks(&candidates, [0.0, 0.0, 0.0], 3);
    assert_eq!(chunks.len(), 4); // ceil(10/3) = 4
    assert_eq!(chunks.iter().map(Vec::len).sum::<usize>(), 10);
}

// T55.1: the chunk nearest the arrival point reveals first, and chunk
// membership stays contiguous by distance.
#[test]
fn nearest_chunk_to_arrival_point_is_first() {
    let candidates = vec![
        candidate(0, 100.0),
        candidate(1, 0.0),
        candidate(2, 50.0),
        candidate(3, 10.0),
    ];
    let chunks = plan_reveal_chunks(&candidates, [0.0, 0.0, 0.0], 2);
    assert_eq!(chunks, vec![vec![1, 3], vec![2, 0]]);
}

// T51.1-style stability check (T55.1): equidistant candidates keep their
// original relative order rather than depending on unspecified sort
// tie-breaking.
#[test]
fn equidistant_candidates_keep_original_order() {
    let candidates = vec![candidate(5, 10.0), candidate(2, 10.0), candidate(9, 10.0)];
    let chunks = plan_reveal_chunks(&candidates, [0.0, 0.0, 0.0], 10);
    assert_eq!(chunks, vec![vec![5, 2, 9]]);
}

// T55.2: a cell at or under one budget's worth of entities reveals in
// exactly one chunk (wave-2 single-frame reveal, preserved bit-for-bit).
#[test]
fn a_cell_at_or_under_budget_reveals_in_one_chunk() {
    let candidates: Vec<RevealCandidate> = (0..5).map(|i| candidate(i, i as f32)).collect();
    assert_eq!(plan_reveal_chunks(&candidates, [0.0, 0.0, 0.0], 5).len(), 1);
    assert_eq!(
        plan_reveal_chunks(&candidates, [0.0, 0.0, 0.0], 99).len(),
        1
    );
}

#[test]
fn an_empty_candidate_list_plans_no_chunks() {
    assert_eq!(
        plan_reveal_chunks(&[], [0.0, 0.0, 0.0], 128),
        Vec::<Vec<usize>>::new()
    );
}

// A misconfigured zero budget is clamped to 1 rather than looping
// forever or dividing by zero.
#[test]
fn a_zero_budget_is_clamped_to_one_entity_per_chunk() {
    let candidates: Vec<RevealCandidate> = (0..3).map(|i| candidate(i, i as f32)).collect();
    let chunks = plan_reveal_chunks(&candidates, [0.0, 0.0, 0.0], 0);
    assert_eq!(chunks.len(), 3);
    assert!(chunks.iter().all(|chunk| chunk.len() == 1));
}
