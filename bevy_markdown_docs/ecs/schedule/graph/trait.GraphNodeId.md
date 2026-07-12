[bevy](../../../index.html)::[ecs](../../index.html)::[schedule](../index.html)::[graph](index.html)

# Trait GraphNodeId 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/graph/graph_map.rs.html#27)

```rust
pub trait GraphNodeId:
    Copy
    + Eq
    + Hash
    + Ord
    + Debug {
    type Adjacent: Copy + Debug + From<(Self, Direction)> + Into<(Self, Direction)>;
    type Edge: Copy + Eq + Hash + Debug + From<(Self, Self)> + Into<(Self, Self)>;

    // Required method
    fn kind(&self) -> &'static str;
}
```

Types that can be used as node identifiers in a [`DiGraph`](type.DiGraph.html "type bevy::ecs::schedule::graph::DiGraph")/[`UnGraph`](type.UnGraph.html "type bevy::ecs::schedule::graph::UnGraph").

## Required Associated Types

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/graph/graph_map.rs.html#30)

#### type [Adjacent](#associatedtype.Adjacent): [Copy](https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html "trait core::marker::Copy") + [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") + [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<(Self, [Direction](enum.Direction.html "enum bevy::ecs::schedule::graph::Direction"))> + [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<(Self, [Direction](enum.Direction.html "enum bevy::ecs::schedule::graph::Direction"))>

The type that packs and unpacks this [`GraphNodeId`](trait.GraphNodeId.html "trait bevy::ecs::schedule::graph::GraphNodeId") with a [`Direction`](enum.Direction.html "enum bevy::ecs::schedule::graph::Direction"). This is used to save space in the graph’s adjacency list.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/graph/graph_map.rs.html#33)

#### type [Edge](#associatedtype.Edge): [Copy](https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html "trait core::marker::Copy") + [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash") + [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") + [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<(Self, Self)> + [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<(Self, Self)>

The type that packs and unpacks this [`GraphNodeId`](trait.GraphNodeId.html "trait bevy::ecs::schedule::graph::GraphNodeId") with another [`GraphNodeId`](trait.GraphNodeId.html "trait bevy::ecs::schedule::graph::GraphNodeId"). This is used to save space in the graph’s edge list.

## Required Methods

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/graph/graph_map.rs.html#39)

#### fn [kind](#tymethod.kind)(&self) -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

Name of the kind of this node id.

For structs, this should return a human-readable name of the struct. For enums, this should return a human-readable name of the enum variant.

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/node.rs.html#327)

### impl [GraphNodeId](trait.GraphNodeId.html "trait bevy::ecs::schedule::graph::GraphNodeId") for [NodeId](../enum.NodeId.html "enum bevy::ecs::schedule::NodeId")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/node.rs.html#328)

#### type [Adjacent](#associatedtype.Adjacent) = [CompactNodeIdAndDirection](../struct.CompactNodeIdAndDirection.html "struct bevy::ecs::schedule::CompactNodeIdAndDirection")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/node.rs.html#329)

#### type [Edge](#associatedtype.Edge) = [CompactNodeIdPair](../struct.CompactNodeIdPair.html "struct bevy::ecs::schedule::CompactNodeIdPair")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/node.rs.html#248)

### impl [GraphNodeId](trait.GraphNodeId.html "trait bevy::ecs::schedule::graph::GraphNodeId") for [SystemKey](../struct.SystemKey.html "struct bevy::ecs::schedule::SystemKey")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/node.rs.html#249)

#### type [Adjacent](#associatedtype.Adjacent) = ([SystemKey](../struct.SystemKey.html "struct bevy::ecs::schedule::SystemKey"), [Direction](enum.Direction.html "enum bevy::ecs::schedule::graph::Direction"))

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/node.rs.html#250)

#### type [Edge](#associatedtype.Edge) = ([SystemKey](../struct.SystemKey.html "struct bevy::ecs::schedule::SystemKey"), [SystemKey](../struct.SystemKey.html "struct bevy::ecs::schedule::SystemKey"))

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/node.rs.html#257)

### impl [GraphNodeId](trait.GraphNodeId.html "trait bevy::ecs::schedule::graph::GraphNodeId") for [SystemSetKey](../struct.SystemSetKey.html "struct bevy::ecs::schedule::SystemSetKey")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/node.rs.html#258)

#### type [Adjacent](#associatedtype.Adjacent) = ([SystemSetKey](../struct.SystemSetKey.html "struct bevy::ecs::schedule::SystemSetKey"), [Direction](enum.Direction.html "enum bevy::ecs::schedule::graph::Direction"))

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/node.rs.html#259)

#### type [Edge](#associatedtype.Edge) = ([SystemSetKey](../struct.SystemSetKey.html "struct bevy::ecs::schedule::SystemSetKey"), [SystemSetKey](../struct.SystemSetKey.html "struct bevy::ecs::schedule::SystemSetKey"))