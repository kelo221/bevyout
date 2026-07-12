[bevy](../index.html)::[image](index.html)

# Trait TextureSrgbViewFormats 

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/image.rs.html#48)

```rust
pub trait TextureSrgbViewFormats {
    // Required method
    fn srgb_view_formats(&self) -> &'static [TextureFormat];
}
```

Trait used to provide texture srgb view formats with static lifetime for `TextureDescriptor.view_formats`.

## Required Methods

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/image.rs.html#50)

#### fn [srgb\_view\_formats](#tymethod.srgb_view_formats)(&self) -> &'static \[[TextureFormat](../render/render_resource/enum.TextureFormat.html "enum bevy::render::render_resource::TextureFormat")\]

Returns the srgb view formats for a type.

## Dyn Compatibility

This trait **is** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/image.rs.html#53)

### impl [TextureSrgbViewFormats](trait.TextureSrgbViewFormats.html "trait bevy::image::TextureSrgbViewFormats") for [TextureFormat](../render/render_resource/enum.TextureFormat.html "enum bevy::render::render_resource::TextureFormat")