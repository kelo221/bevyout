[bevy](../index.html)::[image](index.html)

# Function ktx2\_dfd\_header\_to\_texture\_format 

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/ktx2.rs.html#478-481)

```rust
pub fn ktx2_dfd_header_to_texture_format(
    basic_data_format_descriptor: &Basic,
    is_srgb: bool,
) -> Result<TextureFormat, TextureError>
```

Available on **crate feature `ktx2`** only.

Reads the [`TextureFormat`](../render/render_resource/enum.TextureFormat.html "enum bevy::render::render_resource::TextureFormat") from a KTX2 data format descriptor header.

## Errors

Returns an error for invalid or unsupported texture formats.