//! Bevy-side driver for issue #55's bounded chunked reveal.
//!
//! Wave-2 acceptance measured Vault101d's first-ever reveal (1,371
//! placements) at 84 ms in one frame: `swap.rs`'s `activate_resident_cell`
//! flipped the destination cell's root `Visibility` straight to `Visible`,
//! and Bevy's visibility propagation cascaded that to every child in the
//! same frame, so all 1,200+ newly-visible entities did their first-ever
//! render extraction / pipeline specialization in that single frame.
//! Revisits (22-33 ms) are far cheaper because that specialization work is
//! cached after the first time.
//!
//! `begin_chunked_reveal` is called from `swap::activate_resident_cell`
//! *after* `apply_save_state_to_cell` has written each placement's baseline
//! `Visibility` (`Inherited` for a visible reference, `Hidden` for a
//! deleted/disabled one -- see that function's doc comment). It only ever
//! touches entities already `Inherited`, so permanently-hidden save-state
//! decisions are left untouched: it force-hides every placement outside the
//! nearest-to-arrival chunk (so the subsequent root `Visible` flip cascades
//! to that first chunk only), then queues the remaining chunks in
//! `PendingReveal` for `advance_pending_reveal` to drain one chunk per
//! frame. A cell at or under one chunk's worth of entities has no tail to
//! queue, so it still reveals in exactly one frame -- wave-2 behavior
//! preserved bit-for-bit (F55.3).
//!
//! An interrupting second swap must not strand hidden entities: any reveal
//! still draining is fast-forwarded (every remaining chunk revealed
//! immediately) before a new one begins. Eviction is safe because the only
//! cell that ever has a `PendingReveal` is the currently active one (the
//! preload policy in `preload.rs` never evicts the active cell), and every
//! entity write here is fallible (`World::get_entity_mut`) in case that
//! invariant is ever broken, so a despawned entity is silently skipped
//! rather than panicking.
//!
//! Nothing here is ported from OpenMW; it is bevyout's own design.

use std::collections::VecDeque;
use std::time::Instant;

use bevy::prelude::*;

use super::super::interaction;
use super::reveal_policy::{self, RevealCandidate};

/// Per-frame reveal budget (F55.2): how many of a freshly-activated
/// preloaded cell's placement entities flip from hidden to visible in a
/// single frame. Tuned on real data (wave-3 acceptance, warm asset cache):
/// 256 measured best (Vault101d first reveal 35.6 ms over 5 chunks; 128
/// paradoxically measured worse at 71.5 ms because the longer reveal window
/// overlaps more concurrent work — collider builds, neighbor preload). The
/// per-frame cost is NOT visibility-flip-count-bound (visflip_ms ~0.1), so
/// shrinking chunks buys nothing; see #55 for the remaining render-prep
/// investigation. Tune against `reveal ... visflip_ms=/frame_ms=` telemetry.
pub(crate) const REVEAL_BUDGET_PER_FRAME: usize = 256;

struct PendingRevealState {
    form_id: u32,
    chunks: VecDeque<Vec<Entity>>,
}

#[derive(Resource, Default)]
pub(crate) struct PendingReveal(Option<PendingRevealState>);

impl PendingReveal {
    /// Drops this cell's queued reveal chunks without revealing them.
    /// Called from `preload.rs`'s eviction loop: the active cell (the only
    /// cell that ever has a pending reveal -- `policy::CellGraph::plan`
    /// never evicts it) can't normally reach this, so this is
    /// defense-in-depth against that invariant ever breaking. Without it, a
    /// hypothetically-evicted mid-reveal cell's `PendingReveal` would keep
    /// referencing despawned entities; `advance_pending_reveal`'s fallible
    /// `get_entity_mut` already tolerates that safely, but there is no
    /// reason to keep draining a dead queue.
    pub(crate) fn clear_if(&mut self, form_id: u32) {
        if self
            .0
            .as_ref()
            .is_some_and(|state| state.form_id == form_id)
        {
            self.0 = None;
        }
    }
}

struct RevealMeasurement {
    form_id: u32,
    entity_count: usize,
    chunk_count: usize,
    visflip_ms: f64,
    frames_remaining: u32,
    max_frame_ms: f64,
}

#[derive(Resource, Default)]
pub(crate) struct RevealTelemetry(Option<RevealMeasurement>);

pub(crate) fn install(app: &mut App) {
    app.init_resource::<PendingReveal>()
        .init_resource::<RevealTelemetry>();
}

/// F55.2/F55.3 entry point, called from `swap::activate_resident_cell` after
/// `apply_save_state_to_cell`. `arrival_point` is the player's teleport
/// destination this activation (same world-space coordinates as placement
/// translations), used to order the nearest-to-arrival chunk first.
/// `budget` is `REVEAL_BUDGET_PER_FRAME` at the real call site; tests pass a
/// smaller value to exercise multi-chunk reveals without spawning hundreds
/// of entities.
pub(crate) fn begin_chunked_reveal(
    world: &mut World,
    destination_cell: u32,
    destination_root: Entity,
    arrival_point: Vec3,
    budget: usize,
) {
    let start = Instant::now();
    fast_forward_pending_reveal(world);

    let mut query = world.query::<(Entity, &interaction::PlacementRoot, &ChildOf, &Visibility)>();
    let candidates: Vec<(Entity, [f32; 3])> = query
        .iter(world)
        .filter(|(_, _, child_of, visibility)| {
            child_of.parent() == destination_root && **visibility == Visibility::Inherited
        })
        .map(|(entity, placement_root, _, _)| (entity, placement_root.placement().translation))
        .collect();

    let reveal_candidates: Vec<RevealCandidate> = candidates
        .iter()
        .enumerate()
        .map(|(index, (_, position))| RevealCandidate {
            index,
            position: *position,
        })
        .collect();
    let chunk_indices =
        reveal_policy::plan_reveal_chunks(&reveal_candidates, arrival_point.to_array(), budget);

    let entity_count = candidates.len();
    let chunk_count = chunk_indices.len().max(1);

    let mut chunks: VecDeque<Vec<Entity>> = chunk_indices
        .into_iter()
        .map(|indices| {
            indices
                .into_iter()
                .map(|index| candidates[index].0)
                .collect()
        })
        .collect();

    // Force every entity outside the first chunk hidden so the root
    // visibility flip below only cascades to that nearest-to-arrival chunk;
    // `advance_pending_reveal` reveals the rest over the following frames.
    if chunks.pop_front().is_some() {
        for tail_chunk in &chunks {
            for &entity in tail_chunk {
                if let Ok(mut entity_mut) = world.get_entity_mut(entity) {
                    entity_mut.insert(Visibility::Hidden);
                }
            }
        }
    }

    world
        .entity_mut(destination_root)
        .insert(Visibility::Visible);

    world.resource_mut::<PendingReveal>().0 = if chunks.is_empty() {
        None
    } else {
        Some(PendingRevealState {
            form_id: destination_cell,
            chunks,
        })
    };

    let visflip_ms = start.elapsed().as_secs_f64() * 1000.0;
    world.resource_mut::<RevealTelemetry>().0 = Some(RevealMeasurement {
        form_id: destination_cell,
        entity_count,
        chunk_count,
        visflip_ms,
        frames_remaining: chunk_count as u32,
        max_frame_ms: 0.0,
    });
}

/// Immediately reveals every remaining chunk of whatever reveal is still
/// draining, so an interrupting second swap never strands hidden entities.
/// A no-op when nothing is pending.
fn fast_forward_pending_reveal(world: &mut World) {
    let Some(state) = world.resource_mut::<PendingReveal>().0.take() else {
        return;
    };
    for chunk in state.chunks {
        reveal_chunk(world, chunk);
    }
}

fn reveal_chunk(world: &mut World, chunk: Vec<Entity>) {
    for entity in chunk {
        if let Ok(mut entity_mut) = world.get_entity_mut(entity) {
            entity_mut.insert(Visibility::Inherited);
        }
    }
}

/// F55.2/F55.3: drains one chunk per frame from whatever reveal is pending.
pub(crate) fn advance_pending_reveal(world: &mut World) {
    let Some(chunk) = world
        .resource_mut::<PendingReveal>()
        .0
        .as_mut()
        .and_then(|state| state.chunks.pop_front())
    else {
        return;
    };
    reveal_chunk(world, chunk);
    let done = world
        .resource::<PendingReveal>()
        .0
        .as_ref()
        .is_some_and(|state| state.chunks.is_empty());
    if done {
        world.resource_mut::<PendingReveal>().0 = None;
    }
}

/// F55.1: logs `reveal <formid:08x> entities=<n> chunks=<c> visflip_ms=<a>
/// frame_ms=<b>` once the reveal's active frame window elapses (the
/// activation frame plus one per additional chunk, mirroring `swap.rs`'s
/// `measure_swap_frame_times` window) -- `frame_ms` is the max observed
/// frame time across that window, `visflip_ms` the CPU-only cost of the
/// synchronous chunk-planning/visibility-flip work `begin_chunked_reveal`
/// did on the activation frame. A `frame_ms` far larger than `visflip_ms`
/// means render/pipeline-prep work, not the visibility-flip count, is what
/// dominates the frame -- see this issue's final report.
pub(crate) fn measure_reveal_frame_time(time: Res<Time>, mut telemetry: ResMut<RevealTelemetry>) {
    let Some(measurement) = telemetry.0.as_mut() else {
        return;
    };
    let frame_ms = time.delta().as_secs_f64() * 1000.0;
    measurement.max_frame_ms = measurement.max_frame_ms.max(frame_ms);
    measurement.frames_remaining = measurement.frames_remaining.saturating_sub(1);
    if measurement.frames_remaining == 0 {
        info!(
            "reveal {:08x} entities={} chunks={} visflip_ms={:.2} frame_ms={:.1}",
            measurement.form_id,
            measurement.entity_count,
            measurement.chunk_count,
            measurement.visflip_ms,
            measurement.max_frame_ms
        );
        telemetry.0 = None;
    }
}

#[cfg(test)]
#[path = "tests/reveal.rs"]
mod tests;
