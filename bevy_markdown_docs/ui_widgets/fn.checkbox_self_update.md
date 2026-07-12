[bevy](../index.html)::[ui\_widgets](index.html)

# Function checkbox\_self\_update 

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/checkbox.rs.html#274)

```rust
pub fn checkbox_self_update(
    value_change: On<'_, '_, ValueChange<bool>>,
    commands: Commands<'_, '_>,
)
```

Observer function which updates the checkbox value in response to a [`ValueChange`](struct.ValueChange.html "struct bevy::ui_widgets::ValueChange") event. This can be used to make the checkbox automatically update its own state when clicked, as opposed to managing the checkbox state externally.