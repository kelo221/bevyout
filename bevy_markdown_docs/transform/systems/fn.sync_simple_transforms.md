[bevy](../../index.html)::[transform](../index.html)::[systems](index.html)

# Function sync\_simple\_transforms 

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/systems.rs.html#42-55)

```rust
pub fn sync_simple_transforms(
    query: ParamSet<'_, '_, (Query<'_, '_, (&Transform, &mut GlobalTransform), (Or<(Changed<Transform>, Added<GlobalTransform>)>, Without<ChildOf>, Without<Children>)>, Query<'_, '_, (Ref<'_, Transform>, &mut GlobalTransform), (Without<ChildOf>, Without<Children>)>)>,
    orphaned: RemovedComponents<'_, '_, ChildOf>,
)
```

Available on **crate feature `bevy-support`** only.

Update [`GlobalTransform`](../../prelude/struct.GlobalTransform.html "struct bevy::prelude::GlobalTransform") component of entities that aren’t in the hierarchy

Third party plugins should ensure that this is used in concert with [`propagate_parent_transforms`](fn.propagate_parent_transforms.html "fn bevy::transform::systems::propagate_parent_transforms") and [`mark_dirty_trees`](fn.mark_dirty_trees.html "fn bevy::transform::systems::mark_dirty_trees").