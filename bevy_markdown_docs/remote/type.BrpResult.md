[bevy](../index.html)::[remote](index.html)

# Type Alias BrpResult 

[Source](https://docs.rs/bevy_remote/0.19.0/x86_64-unknown-linux-gnu/src/bevy_remote/lib.rs.html#1428)

```rust
pub type BrpResult<T = Value> = Result<T, BrpError>;
```

The result of a request.

## Aliased Type

```rust
pub enum BrpResult<T = Value> {
    Ok(T),
    Err(BrpError),
}
```

## Variants

1.0.0

### Ok(T)

Contains the success value

1.0.0

### Err([BrpError](struct.BrpError.html "struct bevy::remote::BrpError"))

Contains the error value