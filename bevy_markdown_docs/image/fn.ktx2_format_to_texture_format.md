[bevy](../index.html)::[image](index.html)

# Function ktx2\_format\_to\_texture\_format 

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/ktx2.rs.html#1205-1208)

```rust
pub fn ktx2_format_to_texture_format(
    ktx2_format: Format,
    is_srgb: bool,
) -> Result<TextureFormat, TextureError>
```

Available on **crate feature `ktx2`** only.

Converts a KTX2 texture format identifier to a [`TextureFormat`](../render/render_resource/enum.TextureFormat.html "enum bevy::render::render_resource::TextureFormat").

## Errors

Returns an error for unsupported texture formats.