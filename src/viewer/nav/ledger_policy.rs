//! Pure intercell nav-agent ledger + swap-eligibility policy (issue #134,
//! M4 wave 4). No `bevy`/`bevy_landmass` import: this module only decides
//! what the ledger holds and who is eligible for what on a cell swap, so it
//! is unit-testable (and includable verbatim by `tests/features.rs` via
//! `#[path]`) without a `World`. `nav/agent.rs` drives it with the real
//! `TestNavAgentState`/`NavArchipelagoState` data and does the actual
//! spawn/despawn/transform work.
//!
//! An intercell agent ledger entry answers "where does this agent belong
//! and how does it come back": which cell it should be restored into, how
//! to spawn it there (at a travel door's destination marker, or frozen at
//! wherever it stood), and what it should resume doing once restored.

use std::collections::HashSet;

/// How a claimed ledger entry spawns back into its cell (issue #134 scope:
/// exactly these two kinds -- no offscreen simulation in between).
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum SpawnKind {
    /// Spawn at the door reference `destination_door_form_id`'s own placed
    /// position in the active cell -- the far side of the travel door the
    /// agent crossed (via traversal handoff) or the player used while the
    /// agent's active route ended at that same door (follow-through).
    DoorMarker { destination_door_form_id: u32 },
    /// Spawn at a captured world position -- the agent's cell tore down
    /// while it was doing anything else (idle, mid-route to a different
    /// door, mid-route to a point) and the player later returns to find it
    /// exactly where it stood.
    FrozenPosition { position: [f32; 3] },
}

/// One ledgered agent: identity, which cell it restores into, how to spawn
/// it there, and what it should resume once restored (`None` = idle).
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct LedgerEntry {
    pub(crate) agent_id: u32,
    pub(crate) cell_form_id: u32,
    pub(crate) spawn_kind: SpawnKind,
    pub(crate) remaining_target: Option<[f32; 3]>,
}

/// The ledger resource's pure payload: a small set of ledgered agents,
/// never more than one entry per `agent_id` (see `record`). Kept as a
/// `Vec` rather than a map so claim order can be made explicit and
/// deterministic (`claim_for_activation` sorts by `agent_id`) independent
/// of any hashing.
#[derive(Debug, Clone, Default, PartialEq)]
pub(crate) struct Ledger {
    entries: Vec<LedgerEntry>,
}

/// The result of `Ledger::claim_for_activation`: entries ready to spawn,
/// and entries whose `DoorMarker` destination door could not be resolved
/// in the active cell (diagnosed, not restored -- see that method's doc
/// comment).
#[derive(Debug, Clone, Default, PartialEq)]
pub(crate) struct ClaimResult {
    pub(crate) restored: Vec<LedgerEntry>,
    pub(crate) stale: Vec<StaleEntry>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct StaleEntry {
    pub(crate) agent_id: u32,
    pub(crate) cell_form_id: u32,
    pub(crate) missing_door_form_id: u32,
}

impl Ledger {
    /// Records `entry`, replacing any existing entry for the same
    /// `agent_id` -- an agent can only ever be in one ledgered place at a
    /// time (it is either live in the active cell, or ledgered exactly
    /// once, never both).
    pub(crate) fn record(&mut self, entry: LedgerEntry) {
        self.entries
            .retain(|existing| existing.agent_id != entry.agent_id);
        self.entries.push(entry);
    }

    /// The entry currently ledgered for `agent_id`, if any (for `tna
    /// status`'s "handed off to cell ..." report while ledgered).
    pub(crate) fn entry_for(&self, agent_id: u32) -> Option<LedgerEntry> {
        self.entries
            .iter()
            .copied()
            .find(|entry| entry.agent_id == agent_id)
    }

    /// Whether any entry is ledgered for `cell_form_id` -- the cheap
    /// fast-path bailout `restore_ledgered_agents_system` uses before
    /// building the (comparatively expensive) known-door set and calling
    /// `claim_for_activation`. Scans every entry rather than a fixed
    /// `agent_id` range (issue #215 removed the roster's upper bound, so
    /// there is no longer a small range to enumerate).
    pub(crate) fn has_entry_for_cell(&self, cell_form_id: u32) -> bool {
        self.entries
            .iter()
            .any(|entry| entry.cell_form_id == cell_form_id)
    }

    /// Claims every ledger entry whose `cell_form_id` matches
    /// `active_cell_form_id`, removing each claimed entry from the ledger
    /// (an entry is claimed at most once). Claimed entries are then split
    /// by resolvability, in deterministic ascending-`agent_id` order:
    /// - `FrozenPosition` entries always resolve (the position was
    ///   captured directly at freeze time).
    /// - `DoorMarker` entries resolve only if `known_door_form_ids` (the
    ///   active cell's own door placement references) contains
    ///   `destination_door_form_id`; otherwise the entry is stale (data
    ///   mismatch between the door's recorded destination and the cell
    ///   actually loaded) and is diagnosed rather than silently dropped or
    ///   spawned at an arbitrary position.
    pub(crate) fn claim_for_activation(
        &mut self,
        active_cell_form_id: u32,
        known_door_form_ids: &HashSet<u32>,
    ) -> ClaimResult {
        let mut claimed: Vec<LedgerEntry> = Vec::new();
        self.entries.retain(|entry| {
            let matches = entry.cell_form_id == active_cell_form_id;
            if matches {
                claimed.push(*entry);
            }
            !matches
        });
        claimed.sort_by_key(|entry| entry.agent_id);

        let mut result = ClaimResult::default();
        for entry in claimed {
            match entry.spawn_kind {
                SpawnKind::DoorMarker {
                    destination_door_form_id,
                } if !known_door_form_ids.contains(&destination_door_form_id) => {
                    result.stale.push(StaleEntry {
                        agent_id: entry.agent_id,
                        cell_form_id: entry.cell_form_id,
                        missing_door_form_id: destination_door_form_id,
                    });
                }
                _ => result.restored.push(entry),
            }
        }
        result
    }

    #[cfg(test)]
    pub(crate) fn len(&self) -> usize {
        self.entries.len()
    }
}

/// Whether a live agent follows the player through the door it just used,
/// or is left frozen behind in the cell that is tearing down.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SwapEligibility {
    FollowThrough,
    Freeze,
}

/// Strict swap-eligibility decision (issue #134): follow-through iff the
/// agent's active route is targeting the exact door the player just used;
/// every other state (idle, routed to a point, mid-route to a *different*
/// door) freezes the agent in the cell it stands in. No offscreen
/// pathfinding decides eligibility for anyone not already routed to that
/// specific door -- an agent one step from the door but not yet routed to
/// it freezes, matching the plan's "strict eligibility" requirement.
pub(crate) fn decide_swap_eligibility(
    agent_active_route_door: Option<u32>,
    door_form_id_used_by_player: u32,
) -> SwapEligibility {
    match agent_active_route_door {
        Some(door) if door == door_form_id_used_by_player => SwapEligibility::FollowThrough,
        _ => SwapEligibility::Freeze,
    }
}

#[cfg(test)]
#[path = "tests/ledger_policy.rs"]
mod tests;
