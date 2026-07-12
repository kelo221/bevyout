[bevy](../index.html)::[image](index.html)

# Type Alias TextureAtlasBuilderResult 

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/texture_atlas_builder.rs.html#68)

```rust
pub type TextureAtlasBuilderResult<T> = Result<T, TextureAtlasBuilderError>;
```

The [`Result`](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result") type used by [`TextureAtlasBuilder`](../prelude/struct.TextureAtlasBuilder.html "struct bevy::prelude::TextureAtlasBuilder").

## Aliased Type

```rust
pub enum TextureAtlasBuilderResult<T> {
    Ok(T),
    Err(TextureAtlasBuilderError),
}
```

## Variants

1.0.0

### Ok(T)

Contains the success value

1.0.0

### Err([TextureAtlasBuilderError](enum.TextureAtlasBuilderError.html "enum bevy::image::TextureAtlasBuilderError"))

Contains the error value