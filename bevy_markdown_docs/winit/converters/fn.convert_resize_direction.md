[bevy](../../index.html)::[winit](../index.html)::[converters](index.html)

# Function convert\_resize\_direction 

[Source](https://docs.rs/bevy_winit/0.19.0/x86_64-unknown-linux-gnu/src/bevy_winit/converters.rs.html#731)

```rust
pub fn convert_resize_direction(
    resize_direction: CompassOctant,
) -> ResizeDirection
```

Converts a Bevy [`CompassOctant`](../../math/enum.CompassOctant.html "enum bevy::math::CompassOctant") to a [`winit::window::ResizeDirection`](https://docs.rs/winit/0.30.13/x86_64-unknown-linux-gnu/winit/window/enum.ResizeDirection.html "enum winit::window::ResizeDirection")