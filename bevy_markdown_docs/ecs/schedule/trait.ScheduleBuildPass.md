[bevy](../../index.html)::[ecs](../index.html)::[schedule](index.html)

# Trait ScheduleBuildPass 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/pass.rs.html#22)

```rust
pub trait ScheduleBuildPass:
    Send
    + Sync
    + Debug
    + 'static {
    type EdgeOptions: 'static;

    // Required methods
    fn add_dependency(
        &mut self,
        from: NodeId,
        to: NodeId,
        options: Option<&Self::EdgeOptions>,
    );
    fn collapse_set(
        &mut self,
        set: SystemSetKey,
        systems: &IndexSet<SystemKey, FixedHasher>,
        dependency_flattening: &Graph<true, NodeId>,
    ) -> impl Iterator<Item = (NodeId, NodeId)>;
    fn build(
        &mut self,
        world: &mut World,
        graph: &mut ScheduleGraph,
        dependency_flattened: FlattenedDependencies<'_>,
    ) -> Result<(), ScheduleBuildError>;
}
```

A pass for modular modification of the dependency graph.

## Required Associated Types

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/pass.rs.html#24)

#### type [EdgeOptions](#associatedtype.EdgeOptions): 'static

Custom options for dependencies between sets or systems.

## Required Methods

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/pass.rs.html#27)

#### fn [add\_dependency](#tymethod.add_dependency)( &mut self, from: [NodeId](enum.NodeId.html "enum bevy::ecs::schedule::NodeId"), to: [NodeId](enum.NodeId.html "enum bevy::ecs::schedule::NodeId"), options: [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&Self::[EdgeOptions](trait.ScheduleBuildPass.html#associatedtype.EdgeOptions "type bevy::ecs::schedule::ScheduleBuildPass::EdgeOptions")\>, )

Called when a dependency between sets or systems was explicitly added to the graph.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/pass.rs.html#32-37)

#### fn [collapse\_set](#tymethod.collapse_set)( &mut self, set: [SystemSetKey](struct.SystemSetKey.html "struct bevy::ecs::schedule::SystemSetKey"), systems: &[IndexSet](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/set/struct.IndexSet.html "struct indexmap::set::IndexSet")<[SystemKey](struct.SystemKey.html "struct bevy::ecs::schedule::SystemKey"), [FixedHasher](../../platform/hash/struct.FixedHasher.html "struct bevy::platform::hash::FixedHasher")\>, dependency\_flattening: &Graph<true, [NodeId](enum.NodeId.html "enum bevy::ecs::schedule::NodeId")\>, ) -> impl [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = ([NodeId](enum.NodeId.html "enum bevy::ecs::schedule::NodeId"), [NodeId](enum.NodeId.html "enum bevy::ecs::schedule::NodeId"))>

Called while flattening the dependency graph. For each `set`, this method is called with the `systems` associated with the set as well as an immutable reference to the current graph. Instead of modifying the graph directly, this method should return an iterator of edges to add to the graph.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/pass.rs.html#40-45)

#### fn [build](#tymethod.build)( &mut self, world: &mut [World](../../prelude/struct.World.html "struct bevy::prelude::World"), graph: &mut [ScheduleGraph](struct.ScheduleGraph.html "struct bevy::ecs::schedule::ScheduleGraph"), dependency\_flattened: [FlattenedDependencies](struct.FlattenedDependencies.html "struct bevy::ecs::schedule::FlattenedDependencies")<'\_>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [ScheduleBuildError](enum.ScheduleBuildError.html "enum bevy::ecs::schedule::ScheduleBuildError")\>

The implementation will be able to modify the `ScheduleGraph` here.

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/auto_insert_apply_deferred.rs.html#63)

### impl [ScheduleBuildPass](trait.ScheduleBuildPass.html "trait bevy::ecs::schedule::ScheduleBuildPass") for [AutoInsertApplyDeferredPass](passes/struct.AutoInsertApplyDeferredPass.html "struct bevy::ecs::schedule::passes::AutoInsertApplyDeferredPass")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/auto_insert_apply_deferred.rs.html#64)

#### type [EdgeOptions](#associatedtype.EdgeOptions) = [IgnoreDeferred](passes/struct.IgnoreDeferred.html "struct bevy::ecs::schedule::passes::IgnoreDeferred")