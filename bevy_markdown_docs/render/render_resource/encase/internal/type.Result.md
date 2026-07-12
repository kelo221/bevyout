[bevy](../../../../index.html)::[render](../../../index.html)::[render\_resource](../../index.html)::[encase](../index.html)::[internal](index.html)

# Type Alias Result 

[Source](https://docs.rs/encase/0.12.0/x86_64-unknown-linux-gnu/src/encase/core/rw.rs.html#11)

```rust
pub type Result<T> = Result<T, Error>;
```

## Aliased Type

```rust
pub enum Result<T> {
    Ok(T),
    Err(Error),
}
```

## Variants

1.0.0

### Ok(T)

Contains the success value

1.0.0

### Err([Error](enum.Error.html "enum bevy::render::render_resource::encase::internal::Error"))

Contains the error value