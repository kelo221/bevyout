[bevy](../../index.html)::[winit](../index.html)::[converters](index.html)

# Function convert\_keyboard\_input 

[Source](https://docs.rs/bevy_winit/0.19.0/x86_64-unknown-linux-gnu/src/bevy_winit/converters.rs.html#18-21)

```rust
pub fn convert_keyboard_input(
    keyboard_input: &KeyEvent,
    window: Entity,
) -> KeyboardInput
```

Converts a [`winit::event::KeyEvent`](https://docs.rs/winit/0.30.13/x86_64-unknown-linux-gnu/winit/event/struct.KeyEvent.html "struct winit::event::KeyEvent") and a window [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity") to a Bevy [`KeyboardInput`](../../input/keyboard/struct.KeyboardInput.html "struct bevy::input::keyboard::KeyboardInput")