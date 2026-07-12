[bevy](../../../index.html)::[ecs](../../index.html)::[schedule](../index.html)

# Module graph 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/mod.rs.html#19)

An implementation of a graph data structure.

## Structs

[Dag](struct.Dag.html "struct bevy::ecs::schedule::graph::Dag")

A directed acyclic graph structure.

[DagAnalysis](struct.DagAnalysis.html "struct bevy::ecs::schedule::graph::DagAnalysis")

Stores the results of a call to [`Dag::analyze`](struct.Dag.html#method.analyze "method bevy::ecs::schedule::graph::Dag::analyze").

[DagCrossDependencyError](struct.DagCrossDependencyError.html "struct bevy::ecs::schedule::graph::DagCrossDependencyError")

Error indicating that two graphs both have a dependency between the same nodes.

[DagGroups](struct.DagGroups.html "struct bevy::ecs::schedule::graph::DagGroups")

A mapping of keys to groups of values in a [`Dag`](struct.Dag.html "struct bevy::ecs::schedule::graph::Dag").

[DagOverlappingGroupError](struct.DagOverlappingGroupError.html "struct bevy::ecs::schedule::graph::DagOverlappingGroupError")

Error indicating that the graph has overlapping groups between two keys.

[DagRedundancyError](struct.DagRedundancyError.html "struct bevy::ecs::schedule::graph::DagRedundancyError")

Error indicating that the graph has redundant edges.

[GraphInfo](struct.GraphInfo.html "struct bevy::ecs::schedule::graph::GraphInfo")

Metadata about how the node fits in the schedule graph

## Enums

[DiGraphToposortError](enum.DiGraphToposortError.html "enum bevy::ecs::schedule::graph::DiGraphToposortError")

Error returned when topologically sorting a directed graph fails.

[Direction](enum.Direction.html "enum bevy::ecs::schedule::graph::Direction")

Edge direction.

## Traits

[GraphNodeId](trait.GraphNodeId.html "trait bevy::ecs::schedule::graph::GraphNodeId")

Types that can be used as node identifiers in a [`DiGraph`](type.DiGraph.html "type bevy::ecs::schedule::graph::DiGraph")/[`UnGraph`](type.UnGraph.html "type bevy::ecs::schedule::graph::UnGraph").

## Type Aliases

[DiGraph](type.DiGraph.html "type bevy::ecs::schedule::graph::DiGraph")

A `Graph` with directed edges of some [`GraphNodeId`](trait.GraphNodeId.html "trait bevy::ecs::schedule::graph::GraphNodeId") `N`.

[UnGraph](type.UnGraph.html "type bevy::ecs::schedule::graph::UnGraph")

A `Graph` with undirected edges of some [`GraphNodeId`](trait.GraphNodeId.html "trait bevy::ecs::schedule::graph::GraphNodeId") `N`.