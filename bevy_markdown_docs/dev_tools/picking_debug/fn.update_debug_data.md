[bevy](../../index.html)::[dev\_tools](../index.html)::[picking\_debug](index.html)

# Function update\_debug\_data 

[Source](https://docs.rs/bevy_dev_tools/0.19.0/x86_64-unknown-linux-gnu/src/bevy_dev_tools/picking_debug.rs.html#213-222)

```rust
pub fn update_debug_data(
    hover_map: Res<'_, HoverMap>,
    entity_names: Query<'_, '_, NameOrEntity>,
    pointers: Query<'_, '_, (&PointerId, &PointerLocation, &PointerPress, &mut PointerDebug)>,
)
```

Update typed debug data used to draw overlays