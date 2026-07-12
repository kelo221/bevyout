[bevy](../index.html)::[image](index.html)

# Trait ToExtents 

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/image.rs.html#576)

```rust
pub trait ToExtents {
    // Required method
    fn to_extents(self) -> Extent3d;
}
```

A trait for creating [`Extent3d`](../render/render_resource/struct.Extent3d.html "struct bevy::render::render_resource::Extent3d") values.

## Required Methods

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/image.rs.html#578)

#### fn [to\_extents](#tymethod.to_extents)(self) -> [Extent3d](../render/render_resource/struct.Extent3d.html "struct bevy::render::render_resource::Extent3d")

Converts this type to an [`Extent3d`](../render/render_resource/struct.Extent3d.html "struct bevy::render::render_resource::Extent3d").

## Dyn Compatibility

This trait **is** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/image.rs.html#580)

### impl [ToExtents](trait.ToExtents.html "trait bevy::image::ToExtents") for [UVec2](../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/image.rs.html#589)

### impl [ToExtents](trait.ToExtents.html "trait bevy::image::ToExtents") for [UVec3](../prelude/struct.UVec3.html "struct bevy::prelude::UVec3")