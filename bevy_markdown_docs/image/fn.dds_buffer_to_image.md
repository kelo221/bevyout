[bevy](../index.html)::[image](index.html)

# Function dds\_buffer\_to\_image 

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/dds.rs.html#20-24)

```rust
pub fn dds_buffer_to_image(
    buffer: &[u8],
    supported_compressed_formats: CompressedImageFormats,
    is_srgb: bool,
) -> Result<Image, TextureError>
```

Available on **crate feature `dds`** only.

Converts DDS bytes to a bevy [`Image`](../prelude/struct.Image.html "struct bevy::prelude::Image") using the given compressed format support.

## Errors

Returns an error if the provided buffer contained invalid data, decompression fails, or transcoding of unsupported data formats fails.