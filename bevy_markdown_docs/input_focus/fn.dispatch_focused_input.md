[bevy](../index.html)::[input\_focus](index.html)

# Function dispatch\_focused\_input 

[Source](https://docs.rs/bevy_input_focus/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input_focus/lib.rs.html#341-347)

```rust
pub fn dispatch_focused_input<M>(
    input_reader: MessageReader<'_, '_, M>,
    focus: ResMut<'_, InputFocus>,
    windows: Query<'_, '_, Entity, With<PrimaryWindow>>,
    entities: &Entities,
    commands: Commands<'_, '_>,
)where
    M: Message + Clone,
```

System which dispatches bubbled input events to the focused entity, or to the primary window if no entity has focus.

If the currently focused entity no longer exists (has been despawned), this system will automatically clear the focus and dispatch events to the primary window instead.