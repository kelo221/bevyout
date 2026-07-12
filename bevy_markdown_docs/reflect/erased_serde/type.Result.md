[bevy](../../index.html)::[reflect](../index.html)::[erased\_serde](index.html)

# Type Alias Result 

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/error.rs.html#14)

```rust
pub type Result<T> = Result<T, Error>;
```

Result type alias where the error is `erased_serde::Error`.

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

### Err([Error](struct.Error.html "struct bevy::reflect::erased_serde::Error"))

Contains the error value