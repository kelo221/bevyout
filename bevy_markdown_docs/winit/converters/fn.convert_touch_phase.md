[bevy](../../index.html)::[winit](../index.html)::[converters](index.html)

# Function convert\_touch\_phase 

[Source](https://docs.rs/bevy_winit/0.19.0/x86_64-unknown-linux-gnu/src/bevy_winit/converters.rs.html#53)

```rust
pub fn convert_touch_phase(phase: TouchPhase) -> TouchPhase
```

Converts a [`winit::event::TouchPhase`](https://docs.rs/winit/0.30.13/x86_64-unknown-linux-gnu/winit/event/enum.TouchPhase.html "enum winit::event::TouchPhase") to a Bevy [`TouchPhase`](../../input/touch/enum.TouchPhase.html "enum bevy::input::touch::TouchPhase").