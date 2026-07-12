[bevy](../../index.html)::[render](../index.html)::[view](index.html)

# Function texture\_format\_to\_code 

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/view/mod.rs.html#105)

```rust
pub fn texture_format_to_code(format: TextureFormat) -> Option<u8>
```

Encode a [`TextureFormat`](../render_resource/enum.TextureFormat.html "enum bevy::render::render_resource::TextureFormat") as a 5-bit code for use in pipeline key bitfields.

Covers all WebGPU renderable and blendable texture formats. Some of them need optional features. See [https://gpuweb.github.io/gpuweb/#plain-color-formats](https://gpuweb.github.io/gpuweb/#plain-color-formats).