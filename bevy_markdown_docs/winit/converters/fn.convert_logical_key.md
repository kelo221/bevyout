[bevy](../../index.html)::[winit](../index.html)::[converters](index.html)

# Function convert\_logical\_key 

[Source](https://docs.rs/bevy_winit/0.19.0/x86_64-unknown-linux-gnu/src/bevy_winit/converters.rs.html#307)

```rust
pub fn convert_logical_key(logical_key_code: &Key) -> Key
```

Converts a [`winit::keyboard::Key`](https://docs.rs/winit/0.30.13/x86_64-unknown-linux-gnu/winit/keyboard/enum.Key.html "enum winit::keyboard::Key") to a Bevy [`bevy_input::keyboard::Key`](../../input/keyboard/enum.Key.html "enum bevy::input::keyboard::Key")