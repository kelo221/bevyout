[bevy](../index.html)::[image](index.html)

# Trait TextureFormatPixelInfo 

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/image.rs.html#2318)

```rust
pub trait TextureFormatPixelInfo {
    // Required method
    fn pixel_size(&self) -> Result<usize, TextureAccessError>;
}
```

Extends the wgpu [`TextureFormat`](../render/render_resource/enum.TextureFormat.html "enum bevy::render::render_resource::TextureFormat") with information about the pixel.

## Required Methods

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/image.rs.html#2321)

#### fn [pixel\_size](#tymethod.pixel_size)(&self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), [TextureAccessError](enum.TextureAccessError.html "enum bevy::image::TextureAccessError")\>

Returns the size of a pixel in bytes of the format. error with `TextureAccessError::UnsupportedTextureFormat` if the format is compressed.

## Dyn Compatibility

This trait **is** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/image.rs.html#2324)

### impl [TextureFormatPixelInfo](trait.TextureFormatPixelInfo.html "trait bevy::image::TextureFormatPixelInfo") for [TextureFormat](../render/render_resource/enum.TextureFormat.html "enum bevy::render::render_resource::TextureFormat")