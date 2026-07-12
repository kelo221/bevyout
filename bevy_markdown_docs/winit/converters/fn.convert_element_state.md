[bevy](../../index.html)::[winit](../index.html)::[converters](index.html)

# Function convert\_element\_state 

[Source](https://docs.rs/bevy_winit/0.19.0/x86_64-unknown-linux-gnu/src/bevy_winit/converters.rs.html#33)

```rust
pub fn convert_element_state(element_state: ElementState) -> ButtonState
```

Converts a [`winit::event::ElementState`](https://docs.rs/winit/0.30.13/x86_64-unknown-linux-gnu/winit/event/enum.ElementState.html "enum winit::event::ElementState") to a Bevy [`ButtonState`](../../input/enum.ButtonState.html "enum bevy::input::ButtonState")