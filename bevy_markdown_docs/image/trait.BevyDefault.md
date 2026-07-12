[bevy](../index.html)::[image](index.html)

# Trait BevyDefault 

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/image.rs.html#35)

```rust
pub trait BevyDefault {
    // Required method
    fn bevy_default() -> Self;
}
```

👎Deprecated:

Use ExtractedView::texture\_format where possible. Bevy does not encourage a default TextureFormat anymore. If you really need this, use TextureFormat::Rgba8UnormSrgb

Trait used to provide default values for Bevy-external types that do not implement [`Default`](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default").

## Required Methods

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/image.rs.html#37)

#### fn [bevy\_default](#tymethod.bevy_default)() -> Self

👎Deprecated:

Use ExtractedView::texture\_format where possible. Bevy does not encourage a default TextureFormat anymore. If you really need this, use TextureFormat::Rgba8UnormSrgb

Returns the default value for a type.

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/image.rs.html#41)

### impl [BevyDefault](trait.BevyDefault.html "trait bevy::image::BevyDefault") for [TextureFormat](../render/render_resource/enum.TextureFormat.html "enum bevy::render::render_resource::TextureFormat")