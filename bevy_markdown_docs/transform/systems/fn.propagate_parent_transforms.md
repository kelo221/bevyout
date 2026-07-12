[bevy](../../index.html)::[transform](../index.html)::[systems](index.html)

# Function propagate\_parent\_transforms 

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/systems.rs.html#506-520)

```rust
pub fn propagate_parent_transforms(
    queue: Local<'_, WorkQueue>,
    roots: Query<'_, '_, (Entity, Ref<'_, Transform>, &mut GlobalTransform, &Children, Ref<'_, TransformTreeChanged>), Without<ChildOf>>,
    nodes: Query<'_, '_, (Entity, (Ref<'static, Transform>, Mut<'static, GlobalTransform>, Ref<'static, TransformTreeChanged>), (Option<&'static Children>, &'static ChildOf))>,
    static_optimizations: Res<'_, StaticTransformOptimizations>,
)
```

Available on **crate feature `bevy-support`** only.

Update [`GlobalTransform`](../../prelude/struct.GlobalTransform.html "struct bevy::prelude::GlobalTransform") component of entities based on entity hierarchy and [`Transform`](../../prelude/struct.Transform.html "struct bevy::prelude::Transform") component.

Third party plugins should ensure that this is used in concert with [`sync_simple_transforms`](fn.sync_simple_transforms.html "fn bevy::transform::systems::sync_simple_transforms") and [`mark_dirty_trees`](fn.mark_dirty_trees.html "fn bevy::transform::systems::mark_dirty_trees").