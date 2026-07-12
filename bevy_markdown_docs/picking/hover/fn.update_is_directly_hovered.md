[bevy](../../index.html)::[picking](../index.html)::[hover](index.html)

# Function update\_is\_directly\_hovered 

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/hover.rs.html#409-413)

```rust
pub fn update_is_directly_hovered(
    hover_map: Option<Res<'_, HoverMap>>,
    hovers: Query<'_, '_, (Entity, &DirectlyHovered)>,
    commands: Commands<'_, '_>,
)
```

Uses [`HoverMap`](struct.HoverMap.html "struct bevy::picking::hover::HoverMap") changes to update [`DirectlyHovered`](struct.DirectlyHovered.html "struct bevy::picking::hover::DirectlyHovered") components.