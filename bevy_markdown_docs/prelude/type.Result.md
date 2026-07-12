[bevy](../index.html)::[prelude](index.html)

# Type Alias Result 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/error/mod.rs.html#82)

```rust
pub type Result<T = (), E = BevyError> = Result<T, E>;
```

A result type for use in fallible systems, commands and observers.

The [`BevyError`](struct.BevyError.html "struct bevy::prelude::BevyError") type is a type-erased error type with optional Bevy-specific diagnostics.

## Aliased Type

```rust
pub enum Result<T = (), E = BevyError> {
    Ok(T),
    Err(E),
}
```

## Variants

1.0.0

### Ok(T)

Contains the success value

1.0.0

### Err(E)

Contains the error value