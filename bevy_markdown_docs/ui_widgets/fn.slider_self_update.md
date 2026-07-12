[bevy](../index.html)::[ui\_widgets](index.html)

# Function slider\_self\_update 

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/slider.rs.html#717)

```rust
pub fn slider_self_update(
    value_change: On<'_, '_, ValueChange<f32>>,
    commands: Commands<'_, '_>,
)
```

Observer function which updates the slider value in response to a [`ValueChange`](struct.ValueChange.html "struct bevy::ui_widgets::ValueChange") event. This can be used to make the slider automatically update its own state when dragged, as opposed to managing the slider state externally.