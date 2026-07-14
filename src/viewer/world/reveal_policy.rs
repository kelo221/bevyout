//! Pure reveal-chunk planning (issue #55): partitions a freshly-activated
//! preloaded cell's placement entities into bounded reveal chunks, ordered
//! so the chunk nearest the player's arrival point reveals first.
//!
//! `world::reveal` (the thin Bevy-side driver) flips at most one chunk's
//! worth of entities from hidden to visible per frame instead of an entire
//! large cell at once -- see that module's doc comment for why (issue #55's
//! measured first-reveal spike). This module is dependency-free (`std`
//! only), the same pattern `policy.rs` (issue #51) and `swap_policy.rs`
//! (issue #52) use, so `tests/features.rs` can include it verbatim.
//!
//! Nothing here is ported from OpenMW; it is bevyout's own design.

use std::cmp::Ordering;

/// One placement entity awaiting reveal: an opaque index into the caller's
/// own entity list (`world::reveal` maps this back to a Bevy `Entity`), and
/// the placement's world-space translation for arrival-proximity ordering.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct RevealCandidate {
    pub(crate) index: usize,
    pub(crate) position: [f32; 3],
}

/// Splits `candidates` into chunks of at most `budget` entities each,
/// ordered nearest-to-`arrival_point` first (T55.1: the chunk containing the
/// arrival door / player position reveals first). `budget` is clamped to at
/// least 1 so a misconfigured zero budget can't loop forever or panic.
///
/// `n <= budget` always yields exactly one chunk containing every candidate
/// (T55.2) -- wave-2's single-frame reveal, preserved bit-for-bit for cells
/// at or under one chunk's worth of entities. Ties in distance keep the
/// candidates' original relative order, since `slice::sort_by` is a stable
/// sort: planning is fully deterministic regardless of how the caller
/// enumerated the entities (e.g. hash-map iteration order upstream).
pub(crate) fn plan_reveal_chunks(
    candidates: &[RevealCandidate],
    arrival_point: [f32; 3],
    budget: usize,
) -> Vec<Vec<usize>> {
    if candidates.is_empty() {
        return Vec::new();
    }
    let budget = budget.max(1);

    let mut order: Vec<usize> = (0..candidates.len()).collect();
    order.sort_by(|&a, &b| {
        distance_sq(candidates[a].position, arrival_point)
            .partial_cmp(&distance_sq(candidates[b].position, arrival_point))
            .unwrap_or(Ordering::Equal)
    });
    let indices: Vec<usize> = order.into_iter().map(|i| candidates[i].index).collect();

    indices.chunks(budget).map(<[usize]>::to_vec).collect()
}

fn distance_sq(a: [f32; 3], b: [f32; 3]) -> f32 {
    let dx = a[0] - b[0];
    let dy = a[1] - b[1];
    let dz = a[2] - b[2];
    dx * dx + dy * dy + dz * dz
}

#[cfg(test)]
mod tests {
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
}
