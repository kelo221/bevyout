[bevy](../../index.html)::[picking](../index.html)::[hover](index.html)

# Function update\_interactions 

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/hover.rs.html#237-245)

```rust
pub fn update_interactions(
    hover_map: Res<'_, HoverMap>,
    previous_hover_map: Res<'_, PreviousHoverMap>,
    commands: Commands<'_, '_>,
    pointers: Query<'_, '_, (&PointerId, &PointerPress, &mut PointerInteraction)>,
    interact: Query<'_, '_, &mut PickingInteraction>,
)
```

Uses [`HoverMap`](struct.HoverMap.html "struct bevy::picking::hover::HoverMap") changes to update [`PointerInteraction`](../pointer/struct.PointerInteraction.html "struct bevy::picking::pointer::PointerInteraction") and [`PickingInteraction`](enum.PickingInteraction.html "enum bevy::picking::hover::PickingInteraction") components.