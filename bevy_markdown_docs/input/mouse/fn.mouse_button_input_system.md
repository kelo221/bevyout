[bevy](../../index.html)::[input](../index.html)::[mouse](index.html)

# Function mouse\_button\_input\_system 

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/mouse.rs.html#189-192)

```rust
pub fn mouse_button_input_system(
    mouse_button_input: ResMut<'_, ButtonInput<MouseButton>>,
    mouse_button_input_events: MessageReader<'_, '_, MouseButtonInput>,
)
```

Available on **crate feature `mouse`** only.

Updates the [`ButtonInput<MouseButton>`](../../prelude/struct.ButtonInput.html "struct bevy::prelude::ButtonInput") resource with the latest [`MouseButtonInput`](struct.MouseButtonInput.html "struct bevy::input::mouse::MouseButtonInput") events.

### Differences

The main difference between the [`MouseButtonInput`](struct.MouseButtonInput.html "struct bevy::input::mouse::MouseButtonInput") event and the [`ButtonInput<MouseButton>`](../../prelude/struct.ButtonInput.html "struct bevy::prelude::ButtonInput") resource is that the latter has convenient functions like [`ButtonInput::pressed`](../../prelude/struct.ButtonInput.html#method.pressed "method bevy::prelude::ButtonInput::pressed"), [`ButtonInput::just_pressed`](../../prelude/struct.ButtonInput.html#method.just_pressed "method bevy::prelude::ButtonInput::just_pressed") and [`ButtonInput::just_released`](../../prelude/struct.ButtonInput.html#method.just_released "method bevy::prelude::ButtonInput::just_released").