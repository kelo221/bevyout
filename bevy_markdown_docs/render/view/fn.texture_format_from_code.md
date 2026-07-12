[bevy](../../index.html)::[render](../index.html)::[view](index.html)

# Function texture\_format\_from\_code 

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/view/mod.rs.html#138)

```rust
pub fn texture_format_from_code(code: u8) -> Option<TextureFormat>
```

Decode a 5-bit code back into a [`TextureFormat`](../render_resource/enum.TextureFormat.html "enum bevy::render::render_resource::TextureFormat").

Inverse of [`texture_format_to_code`](fn.texture_format_to_code.html "fn bevy::render::view::texture_format_to_code").