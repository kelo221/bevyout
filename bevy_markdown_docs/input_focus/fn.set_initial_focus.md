[bevy](../index.html)::[input\_focus](index.html)

# Function set\_initial\_focus 

[Source](https://docs.rs/bevy_input_focus/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input_focus/lib.rs.html#327-330)

```rust
pub fn set_initial_focus(
    input_focus: ResMut<'_, InputFocus>,
    window: Single<'_, '_, Entity, With<PrimaryWindow>>,
)
```

If no entity is focused, sets the focus to the primary window, if any.