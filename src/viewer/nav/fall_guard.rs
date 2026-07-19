//! Pure std-only fall-out-of-world guard policy (issue #164).
//!
//! FranklinMetro02 (cell `0001a273`) has walkable navmesh over regions with
//! no collision floor beneath them: an agent that steps onto such a region
//! is not caught by any collider and descends under gravity forever, its
//! `tna status` pinned at `status=unreachable stuck=true` while its Y falls
//! without bound. This module is the deterministic guard that ends that
//! descent: given the prepared cell's minimum geometry Y and an agent's
//! current Y, it decides whether the agent has fallen clear out of the
//! world and should be despawned/reset.
//!
//! The kill plane is *derived* from the cell's own prepared geometry bounds
//! -- a fixed [`FALL_GUARD_MARGIN_METRES`] clearance below the lowest
//! authored/walkable surface in the active cell -- never a hard-coded world
//! Y. Nothing walkable sits below the prepared bounds' minimum by
//! construction (it is the minimum of that geometry), so a capsule whose
//! centre has dropped a whole margin below it can only have left the world.
//! Std-only (no `bevy`/`bevy_landmass` import) so `tests/features.rs` can
//! include it verbatim via `#[path]`, mirroring `movement_policy.rs`.
//!
//! The Bevy system that applies this verdict lives in `nav/agent.rs`; it
//! samples each agent's real capsule-centre Y and the active cell's bounds
//! and does nothing but feed them here and act on the result -- no
//! decision-making in the system itself.

/// Metres below the prepared cell's lowest geometry Y at which a nav agent
/// counts as having fallen clear out of the world. A fixed clearance, not a
/// magic absolute Y: the kill plane itself ([`fall_kill_z`]) is always
/// re-derived per cell from that cell's own bounds minimum. Sized generously
/// enough that an agent merely standing on the lowest walkable surface --
/// whose capsule centre sits *above* the bounds minimum, never below it --
/// can never trip it, while a capsule that has actually dropped through a
/// missing floor (gravity carries it metres past the minimum within a
/// handful of ticks) is caught almost immediately.
pub(crate) const FALL_GUARD_MARGIN_METRES: f32 = 5.0;

/// The kill-plane Y for a cell whose prepared geometry bounds have minimum
/// Y `bounds_min_y`: [`FALL_GUARD_MARGIN_METRES`] below it. An agent whose
/// capsule-centre Y drops below this has fallen out of the world.
pub(crate) fn fall_kill_z(bounds_min_y: f32) -> f32 {
    bounds_min_y - FALL_GUARD_MARGIN_METRES
}

/// The guard's verdict for one agent this tick.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum FallVerdict {
    /// The agent is at or above the kill plane -- normal, no action.
    InBounds,
    /// The agent's Y has dropped below the kill plane: it has fallen out of
    /// the world and must be despawned/reset deterministically.
    FellOutOfWorld,
}

/// Whether `agent_y` (the agent's capsule-centre Y) has fallen below the
/// kill plane derived from the active cell's `bounds_min_y`. The comparison
/// is strict (`<`): an agent resting exactly at the kill plane is still in
/// bounds, so the verdict only fires once the agent is unambiguously past
/// it.
pub(crate) fn evaluate_fall(bounds_min_y: f32, agent_y: f32) -> FallVerdict {
    if agent_y < fall_kill_z(bounds_min_y) {
        FallVerdict::FellOutOfWorld
    } else {
        FallVerdict::InBounds
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn kill_z_is_a_fixed_margin_below_the_bounds_minimum() {
        assert_eq!(fall_kill_z(94.168), 94.168 - FALL_GUARD_MARGIN_METRES);
        assert_eq!(fall_kill_z(0.0), -FALL_GUARD_MARGIN_METRES);
    }

    #[test]
    fn an_agent_above_the_kill_plane_is_in_bounds() {
        // Real FranklinMetro02 (0001a273) whole-graph bounds minimum Y is
        // ~94.168; a capsule resting on the lowest walkable surface sits
        // above it, never a whole margin below.
        assert_eq!(evaluate_fall(94.168, 95.0), FallVerdict::InBounds);
    }

    #[test]
    fn an_agent_resting_exactly_at_the_kill_plane_is_still_in_bounds() {
        let min_y = 94.168;
        assert_eq!(
            evaluate_fall(min_y, fall_kill_z(min_y)),
            FallVerdict::InBounds
        );
    }

    #[test]
    fn an_agent_below_the_kill_plane_has_fallen_out_of_the_world() {
        let min_y = 94.168;
        assert_eq!(
            evaluate_fall(min_y, fall_kill_z(min_y) - 0.01),
            FallVerdict::FellOutOfWorld
        );
    }

    #[test]
    fn an_agent_that_descended_far_below_the_cell_is_out_of_the_world() {
        // The exact failure mode #164 describes: an agent that fell through
        // a missing floor keeps descending without bound.
        assert_eq!(evaluate_fall(94.168, -1000.0), FallVerdict::FellOutOfWorld);
    }
}
