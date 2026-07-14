//! Pure predictive preload policy (issue #51): door-graph adjacency, BFS
//! graph distance, and the deterministic `plan` decision (which prepared
//! neighbor cells to load, which resident cells to evict).
//!
//! This module is bevyout's own -- nothing here is ported from OpenMW (see
//! `apps/openmw/mwworld/cellpreloader.cpp` / `scene.cpp` in the OpenMW tree
//! for the prior art this was designed against, read but not copied). It is
//! deliberately kept dependency-free (`std` only, `+serde` would be fine but
//! isn't even needed) exactly like `src/vsa/cell_map.rs` -- see that module's
//! doc comment -- so `tests/features.rs` can pull it in verbatim without
//! compiling any Bevy or ESM4 parser code. It defines its own plain
//! `DoorLink` input shape mirroring the connectivity fields of
//! `vsa::cell_map::DoorEdge` rather than depending on that type.

use std::collections::{HashMap, HashSet, VecDeque};

/// A single directed door connection between two cells, mirroring the
/// connectivity-relevant fields of `vsa::cell_map::DoorEdge`. The graph built
/// from a set of `DoorLink`s is undirected (see `CellGraph::build`): a
/// teleport is treated as connecting its two cells regardless of which
/// direction the content set happened to record.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct DoorLink {
    pub(crate) source_cell_form_id: u32,
    pub(crate) destination_cell_form_id: u32,
}

/// The result of `CellGraph::plan`: cells to start loading, and resident
/// cells to evict, this pass.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct PreloadPlan {
    pub(crate) load: Vec<u32>,
    pub(crate) evict: Vec<u32>,
}

/// Undirected door-graph adjacency built from a content set's door edges.
#[derive(Debug, Clone, Default)]
pub(crate) struct CellGraph {
    adjacency: HashMap<u32, Vec<u32>>,
}

impl CellGraph {
    /// Builds the adjacency map. Each `DoorLink` adds an edge in both
    /// directions, since a door reached from either side should treat both
    /// cells as neighbors regardless of which cell the content set recorded
    /// as the teleport's source.
    pub(crate) fn build(edges: &[DoorLink]) -> Self {
        let mut adjacency: HashMap<u32, Vec<u32>> = HashMap::new();
        for edge in edges {
            let forward = adjacency.entry(edge.source_cell_form_id).or_default();
            if !forward.contains(&edge.destination_cell_form_id) {
                forward.push(edge.destination_cell_form_id);
            }
            let backward = adjacency.entry(edge.destination_cell_form_id).or_default();
            if !backward.contains(&edge.source_cell_form_id) {
                backward.push(edge.source_cell_form_id);
            }
        }
        Self { adjacency }
    }

    fn neighbors(&self, cell: u32) -> impl Iterator<Item = u32> + '_ {
        self.adjacency.get(&cell).into_iter().flatten().copied()
    }

    /// BFS shortest-path distance (in door hops) from `start` to every cell
    /// reachable from it. `start` itself has distance 0.
    pub(crate) fn distances_from(&self, start: u32) -> HashMap<u32, usize> {
        let mut distances = HashMap::new();
        distances.insert(start, 0);
        let mut queue = VecDeque::new();
        queue.push_back(start);
        while let Some(cell) = queue.pop_front() {
            let distance = distances[&cell];
            for neighbor in self.neighbors(cell) {
                if let std::collections::hash_map::Entry::Vacant(entry) = distances.entry(neighbor)
                {
                    entry.insert(distance + 1);
                    queue.push_back(neighbor);
                }
            }
        }
        distances
    }

    /// Decides which prepared neighbor cells to start loading and which
    /// resident cells to evict this pass.
    ///
    /// The desired resident set is the active cell plus its 1-hop door-graph
    /// neighbors that are already in `prepared_cells` -- a cell is never
    /// planned for load unless it is prepared (epic #5 runtime policy). Any
    /// desired cell not already in `resident_cells` is planned for load.
    ///
    /// If residents after the planned loads would exceed `budget`, the
    /// farthest resident cells by graph distance from `active_cell` are
    /// evicted first until the budget is met; ties are broken by higher
    /// FormID first, so the result is fully deterministic. `active_cell` is
    /// never evicted, even if that means staying over budget.
    ///
    /// Re-planning when everything desired is already resident and the
    /// budget is already met returns an empty plan.
    pub(crate) fn plan(
        &self,
        active_cell: u32,
        resident_cells: &[u32],
        prepared_cells: &HashSet<u32>,
        budget: usize,
    ) -> PreloadPlan {
        let distances = self.distances_from(active_cell);
        let resident_set: HashSet<u32> = resident_cells.iter().copied().collect();

        let mut desired: HashSet<u32> = HashSet::from([active_cell]);
        for neighbor in self.neighbors(active_cell) {
            if prepared_cells.contains(&neighbor) {
                desired.insert(neighbor);
            }
        }

        // The active cell is never something to "load" -- it is how the
        // player got here, so it is implicitly already resident regardless
        // of whether the caller's `resident_cells` snapshot includes it.
        let mut load: Vec<u32> = desired
            .into_iter()
            .filter(|&cell| cell != active_cell && !resident_set.contains(&cell))
            .collect();
        load.sort_unstable();

        let mut residents_after = resident_set;
        residents_after.insert(active_cell);
        residents_after.extend(load.iter().copied());

        let mut evict = Vec::new();
        if residents_after.len() > budget {
            let mut evictable: Vec<u32> = residents_after
                .into_iter()
                .filter(|&cell| cell != active_cell)
                .collect();
            // Farthest-by-graph-distance first; ties broken by higher FormID
            // first, so ordering never depends on hash-map iteration order.
            evictable.sort_unstable_by(|a, b| {
                let distance_a = distances.get(a).copied().unwrap_or(usize::MAX);
                let distance_b = distances.get(b).copied().unwrap_or(usize::MAX);
                distance_b.cmp(&distance_a).then_with(|| b.cmp(a))
            });
            let residents_len = resident_cells
                .iter()
                .copied()
                .chain(load.iter().copied())
                .collect::<HashSet<_>>()
                .len();
            let overflow = residents_len.saturating_sub(budget);
            evict.extend(evictable.into_iter().take(overflow));
        }

        PreloadPlan { load, evict }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn link(source: u32, destination: u32) -> DoorLink {
        DoorLink {
            source_cell_form_id: source,
            destination_cell_form_id: destination,
        }
    }

    // T51.1: active cell with two door neighbors, both prepared, plans loads
    // for both; an unprepared third neighbor is never planned.
    #[test]
    fn plan_never_loads_an_unprepared_neighbor() {
        let graph = CellGraph::build(&[link(0x100, 0x200), link(0x100, 0x300)]);
        let prepared = HashSet::from([0x200]);
        let plan = graph.plan(0x100, &[], &prepared, 4);
        assert_eq!(plan.load, vec![0x200]);
        assert!(!plan.load.contains(&0x300));
    }

    // T51.2: budget 2 with residents {A, B, C}, C farthest by graph distance
    // from active A, is evicted; the active cell is never evicted even when
    // still over budget.
    #[test]
    fn plan_evicts_the_farthest_resident_and_never_the_active_cell() {
        let graph = CellGraph::build(&[link(0x100, 0x200), link(0x200, 0x300)]);
        let prepared = HashSet::new();
        let resident = [0x100, 0x200, 0x300];
        let plan = graph.plan(0x100, &resident, &prepared, 2);
        assert_eq!(plan.evict, vec![0x300]);
        assert!(!plan.evict.contains(&0x100));
    }

    // Active cell is never evicted even if the budget cannot be met without
    // it (e.g. budget smaller than 1).
    #[test]
    fn plan_never_evicts_the_active_cell_even_when_still_over_budget() {
        let graph = CellGraph::build(&[link(0x100, 0x200)]);
        let prepared = HashSet::new();
        let resident = [0x100, 0x200];
        let plan = graph.plan(0x100, &resident, &prepared, 0);
        assert_eq!(plan.evict, vec![0x200]);
        assert!(!plan.evict.contains(&0x100));
    }

    // T51.3: everything resident and within budget already -> empty,
    // idempotent plan.
    #[test]
    fn plan_is_empty_and_idempotent_when_everything_is_already_resident() {
        let graph = CellGraph::build(&[link(0x100, 0x200)]);
        let prepared = HashSet::from([0x200]);
        let resident = [0x100, 0x200];
        let plan = graph.plan(0x100, &resident, &prepared, 4);
        assert_eq!(plan, PreloadPlan::default());
    }

    #[test]
    fn distances_from_computes_bfs_shortest_hop_count() {
        let graph = CellGraph::build(&[link(0x1, 0x2), link(0x2, 0x3), link(0x3, 0x4)]);
        let distances = graph.distances_from(0x1);
        assert_eq!(distances.get(&0x1), Some(&0));
        assert_eq!(distances.get(&0x2), Some(&1));
        assert_eq!(distances.get(&0x3), Some(&2));
        assert_eq!(distances.get(&0x4), Some(&3));
    }

    #[test]
    fn distances_from_does_not_include_unreachable_cells() {
        let graph = CellGraph::build(&[link(0x1, 0x2)]);
        let distances = graph.distances_from(0x1);
        assert_eq!(distances.get(&0x99), None);
    }

    // Deterministic tie-breaking: two residents at the same graph distance
    // from the active cell must always evict the same one -- higher FormID
    // first -- regardless of HashMap/HashSet iteration order.
    #[test]
    fn eviction_ties_break_by_higher_form_id_first() {
        let graph = CellGraph::build(&[link(0x10, 0x20), link(0x10, 0x30)]);
        let prepared = HashSet::new();
        let resident = [0x10, 0x20, 0x30];
        let plan = graph.plan(0x10, &resident, &prepared, 2);
        assert_eq!(plan.evict, vec![0x30]);
    }

    #[test]
    fn a_door_link_is_treated_as_bidirectional_for_adjacency() {
        let graph = CellGraph::build(&[link(0x100, 0x200)]);
        let distances_from_source = graph.distances_from(0x100);
        let distances_from_destination = graph.distances_from(0x200);
        assert_eq!(distances_from_source.get(&0x200), Some(&1));
        assert_eq!(distances_from_destination.get(&0x100), Some(&1));
    }
}
