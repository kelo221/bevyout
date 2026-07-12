[bevy](../../index.html)::[platform](../index.html)::[sync](index.html)

# Type Alias LockResult 

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/std/sync/poison.rs.html#235)

```rust
pub type LockResult<T> = Result<T, PoisonError<T>>;
```

A type alias for the result of a lock method which can be poisoned.

The [`Ok`](https://doc.rust-lang.org/nightly/core/result/enum.Result.html#variant.Ok "variant core::result::Result::Ok") variant of this result indicates that the primitive was not poisoned, and the operation result is contained within. The [`Err`](https://doc.rust-lang.org/nightly/core/result/enum.Result.html#variant.Err "variant core::result::Result::Err") variant indicates that the primitive was poisoned. Note that the [`Err`](https://doc.rust-lang.org/nightly/core/result/enum.Result.html#variant.Err "variant core::result::Result::Err") variant _also_ carries an associated value assigned by the lock method, and it can be acquired through the [`into_inner`](struct.PoisonError.html#method.into_inner "method bevy::platform::sync::PoisonError::into_inner") method. The semantics of the associated value depends on the corresponding lock method.

## Aliased Type

```rust
pub enum LockResult<T> {
    Ok(T),
    Err(PoisonError<T>),
}
```

## Variants

1.0.0

### Ok(T)

Contains the success value

1.0.0

### Err([PoisonError](struct.PoisonError.html "struct bevy::platform::sync::PoisonError")<T>)

Contains the error value