[bevy](../../index.html)::[transform](../index.html)::[systems](index.html)

# Function propagate\_transforms\_for 

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/systems.rs.html#11-14)

```rust
pub fn propagate_transforms_for<F>(
    tf_helper: TransformHelper<'_, '_>,
    query: Query<'_, '_, (Entity, &mut GlobalTransform), F>,
)where
    F: QueryFilter + 'static,
```

Available on **crate feature `bevy-support`** only.

Generic system that propagates transforms, using [`TransformHelper`](../../prelude/struct.TransformHelper.html "struct bevy::prelude::TransformHelper") for any entity matching the filter `F`. Useful for moving and rendering in the same frame.