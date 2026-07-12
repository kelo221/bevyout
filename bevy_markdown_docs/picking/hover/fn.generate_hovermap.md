[bevy](../../index.html)::[picking](../index.html)::[hover](index.html)

# Function generate\_hovermap 

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/hover.rs.html#96-107)

```rust
pub fn generate_hovermap(
    pickable: Query<'_, '_, &Pickable>,
    pointers: Query<'_, '_, &PointerId>,
    pointer_hits_reader: MessageReader<'_, '_, PointerHits>,
    pointer_input_reader: MessageReader<'_, '_, PointerInput>,
    over_map: Local<'_, HashMap<PointerId, BTreeMap<FloatOrd, Vec<(Entity, HitData)>>>>,
    hover_map: ResMut<'_, HoverMap>,
    previous_hover_map: ResMut<'_, PreviousHoverMap>,
)
```

Coalesces all data from inputs and backends to generate a map of the currently hovered entities. This is the final focusing step to determine which entity the pointer is hovering over.