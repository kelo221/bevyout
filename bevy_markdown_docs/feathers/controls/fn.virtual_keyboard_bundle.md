[bevy](../../index.html)::[feathers](../index.html)::[controls](index.html)

# Function virtual\_keyboard\_bundle 

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/virtual_keyboard.rs.html#110-114)

```rust
pub fn virtual_keyboard_bundle<T>(
    keys: impl Iterator<Item = Vec<T>> + Send + Sync + 'static,
) -> impl Bundlewhere
    T: AsRef<str> + Clone + Send + Sync + 'static,
```

👎Deprecated since 0.19.0:

Use the virtual\_keyboard() BSN function

Function to spawn a virtual keyboard

## Emitted events

*   [`crate::controls::VirtualKeyPressed<T>`](struct.VirtualKeyPressed.html "struct bevy::feathers::controls::VirtualKeyPressed") when a virtual key on the keyboard is un-pressed.

These events can be disabled by adding an [`bevy_ui::InteractionDisabled`](../../ui/struct.InteractionDisabled.html "struct bevy::ui::InteractionDisabled") component to the entity