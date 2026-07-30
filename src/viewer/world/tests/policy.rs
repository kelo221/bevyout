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
