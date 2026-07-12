[bevy](../../index.html)::[picking](../index.html)::[pointer](index.html)

# Function update\_pointer\_map 

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/pointer.rs.html#106)

```rust
pub fn update_pointer_map(
    pointers: Query<'_, '_, (Entity, &PointerId)>,
    map: ResMut<'_, PointerMap>,
)
```

Update the [`PointerMap`](struct.PointerMap.html "struct bevy::picking::pointer::PointerMap") resource with the current frame’s data.