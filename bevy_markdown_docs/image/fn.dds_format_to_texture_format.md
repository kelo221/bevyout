[bevy](../index.html)::[image](index.html)

# Function dds\_format\_to\_texture\_format 

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/dds.rs.html#134-137)

```rust
pub fn dds_format_to_texture_format(
    dds: &Dds,
    is_srgb: bool,
) -> Result<TextureFormat, TextureError>
```

Available on **crate feature `dds`** only.

Gets a [`TextureFormat`](../render/render_resource/enum.TextureFormat.html "enum bevy::render::render_resource::TextureFormat") from a [`Dds`](https://docs.rs/ddsfile/0.5.2/x86_64-unknown-linux-gnu/ddsfile/struct.Dds.html "struct ddsfile::Dds") file.

## Errors

Returns an error for unsupported texture formats.