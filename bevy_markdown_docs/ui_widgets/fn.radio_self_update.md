[bevy](../index.html)::[ui\_widgets](index.html)

# Function radio\_self\_update 

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/radio.rs.html#343-348)

```rust
pub fn radio_self_update(
    value_change: On<'_, '_, ValueChange<Entity>>,
    q_radio_group: Query<'_, '_, &Children, With<RadioGroup>>,
    q_radio: Query<'_, '_, Entity, With<RadioButton>>,
    commands: Commands<'_, '_>,
)
```

Observer function which updates the radio buttons in a group in response to a [`ValueChange`](struct.ValueChange.html "struct bevy::ui_widgets::ValueChange") event. This can be used to make the radio buttons automatically update their own states and within the correct radio group when clicked, as opposed to managing the states externally.