[bevy](../../index.html)::[transform](../index.html)::[systems](index.html)

# Function mark\_dirty\_trees 

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/systems.rs.html#111-130)

```rust
pub fn mark_dirty_trees(
    changed: Query<'_, '_, Entity, Or<(Changed<Transform>, Changed<ChildOf>, Added<GlobalTransform>)>>,
    orphaned: RemovedComponents<'_, '_, ChildOf>,
    transforms: Query<'_, '_, &mut TransformTreeChanged>,
    parents: Query<'_, '_, &ChildOf>,
    static_optimizations: Res<'_, StaticTransformOptimizations>,
    shared_bitset: Local<'_, Vec<Atomic<u64>>>,
    local_bitset: Local<'_, Parallel<Vec<u64>>>,
    consumer_channels: Local<'_, BufferedChannel<Entity>>,
    traversal_channels: Local<'_, BufferedChannel<Entity>>,
)
```

Available on **crate feature `bevy-support`** only.

Optimization for static scenes.

Propagates a “dirty bit” up the hierarchy towards ancestors. Transform propagation can ignore entire subtrees of the hierarchy if it encounters an entity without the dirty bit.

Configure behavior with [`StaticTransformOptimizations`](../../prelude/enum.StaticTransformOptimizations.html "enum bevy::prelude::StaticTransformOptimizations").