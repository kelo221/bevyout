[bevy](../../index.html)::[winit](../index.html)::[converters](index.html)

# Function convert\_physical\_native\_key\_code 

[Source](https://docs.rs/bevy_winit/0.19.0/x86_64-unknown-linux-gnu/src/bevy_winit/converters.rs.html#89-91)

```rust
pub fn convert_physical_native_key_code(
    native_key_code: NativeKeyCode,
) -> NativeKeyCode
```

Converts a [`winit::keyboard::NativeKeyCode`](https://docs.rs/winit/0.30.13/x86_64-unknown-linux-gnu/winit/keyboard/enum.NativeKeyCode.html "enum winit::keyboard::NativeKeyCode") to a Bevy [`NativeKeyCode`](../../input/keyboard/enum.NativeKeyCode.html "enum bevy::input::keyboard::NativeKeyCode")