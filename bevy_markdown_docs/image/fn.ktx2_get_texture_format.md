[bevy](../index.html)::[image](index.html)

# Function ktx2\_get\_texture\_format 

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/ktx2.rs.html#396-399)

```rust
pub fn ktx2_get_texture_format<Data>(
    ktx2: &Reader<Data>,
    is_srgb: bool,
) -> Result<TextureFormat, TextureError>where
    Data: AsRef<[u8]>,
```

Available on **crate feature `ktx2`** only.

Reads the [`TextureFormat`](../render/render_resource/enum.TextureFormat.html "enum bevy::render::render_resource::TextureFormat") from a [`ktx2::Reader`](https://docs.rs/ktx2/0.5.0/x86_64-unknown-linux-gnu/ktx2/struct.Reader.html "struct ktx2::Reader").

## Errors

Returns an error for invalid KTX2 data, or unsupported texture formats.