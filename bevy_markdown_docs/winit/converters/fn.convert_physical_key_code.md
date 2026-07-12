[bevy](../../index.html)::[winit](../index.html)::[converters](index.html)

# Function convert\_physical\_key\_code 

[Source](https://docs.rs/bevy_winit/0.19.0/x86_64-unknown-linux-gnu/src/bevy_winit/converters.rs.html#101)

```rust
pub fn convert_physical_key_code(virtual_key_code: PhysicalKey) -> KeyCode
```

Converts a [`winit::keyboard::PhysicalKey`](https://docs.rs/winit/0.30.13/x86_64-unknown-linux-gnu/winit/keyboard/enum.PhysicalKey.html "enum winit::keyboard::PhysicalKey") to a Bevy [`KeyCode`](../../prelude/enum.KeyCode.html "enum bevy::prelude::KeyCode")