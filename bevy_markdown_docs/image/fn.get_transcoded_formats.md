[bevy](../index.html)::[image](index.html)

# Function get\_transcoded\_formats 

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/ktx2.rs.html#310-314)

```rust
pub fn get_transcoded_formats(
    supported_compressed_formats: CompressedImageFormats,
    data_format: TextureChannelLayout,
    is_srgb: bool,
) -> (TranscoderBlockFormat, TextureFormat)
```

Available on **crate feature `basis-universal`** only.

Determines an appropriate wgpu-compatible format based on compressed format support, and a basis universal [`TextureChannelLayout`](enum.TextureChannelLayout.html "enum bevy::image::TextureChannelLayout").