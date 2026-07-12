[bevy](../../index.html)::[winit](../index.html)::[converters](index.html)

# Function convert\_touch\_input 

[Source](https://docs.rs/bevy_winit/0.19.0/x86_64-unknown-linux-gnu/src/bevy_winit/converters.rs.html#63-67)

```rust
pub fn convert_touch_input(
    touch_input: Touch,
    location: LogicalPosition<f64>,
    window_entity: Entity,
) -> TouchInput
```

Converts a [`winit::event::Touch`](https://docs.rs/winit/0.30.13/x86_64-unknown-linux-gnu/winit/event/struct.Touch.html "struct winit::event::Touch"), [`winit::dpi::LogicalPosition<f64>`](https://docs.rs/dpi/0.1.2/x86_64-unknown-linux-gnu/dpi/struct.LogicalPosition.html "struct dpi::LogicalPosition") and window [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity") to a Bevy [`TouchInput`](../../prelude/struct.TouchInput.html "struct bevy::prelude::TouchInput")