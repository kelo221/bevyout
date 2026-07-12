[bevy](../index.html)::[image](index.html)

# Constant TRANSPARENT\_IMAGE\_HANDLE 

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/image.rs.html#176)

```rust
pub const TRANSPARENT_IMAGE_HANDLE: Handle<Image>;
```

A handle to a 1 x 1 transparent white image.

Like [`Handle<Image>::default`](../prelude/enum.Handle.html#method.default "associated function bevy::prelude::Handle::default"), this is a handle to a fallback image asset. While that handle points to an opaque white 1 x 1 image, this handle points to a transparent 1 x 1 white image.