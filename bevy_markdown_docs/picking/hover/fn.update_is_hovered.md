[bevy](../../index.html)::[picking](../index.html)::[hover](index.html)

# Function update\_is\_hovered 

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/hover.rs.html#367-372)

```rust
pub fn update_is_hovered(
    hover_map: Option<Res<'_, HoverMap>>,
    hovers: Query<'_, '_, (Entity, &Hovered)>,
    parent_query: Query<'_, '_, &ChildOf>,
    commands: Commands<'_, '_>,
)
```

Uses [`HoverMap`](struct.HoverMap.html "struct bevy::picking::hover::HoverMap") changes to update [`Hovered`](struct.Hovered.html "struct bevy::picking::hover::Hovered") components.