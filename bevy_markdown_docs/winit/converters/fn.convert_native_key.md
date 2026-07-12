[bevy](../../index.html)::[winit](../index.html)::[converters](index.html)

# Function convert\_native\_key 

[Source](https://docs.rs/bevy_winit/0.19.0/x86_64-unknown-linux-gnu/src/bevy_winit/converters.rs.html#639)

```rust
pub fn convert_native_key(native_key: &NativeKey) -> NativeKey
```

Converts a [`winit::keyboard::NativeKey`](https://docs.rs/winit/0.30.13/x86_64-unknown-linux-gnu/winit/keyboard/enum.NativeKey.html "enum winit::keyboard::NativeKey") to a Bevy [`NativeKey`](../../input/keyboard/enum.NativeKey.html "enum bevy::input::keyboard::NativeKey")