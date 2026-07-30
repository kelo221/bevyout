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
#[path = "tests/reveal_policy.rs"]
mod tests;
