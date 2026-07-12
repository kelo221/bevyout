[bevy](../../index.html)::[platform](../index.html)::[sync](index.html)

# Type Alias TryLockResult 

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/std/sync/poison.rs.html#243)

```rust
pub type TryLockResult<Guard> = Result<Guard, TryLockError<Guard>>;
```

A type alias for the result of a nonblocking locking method.

For more information, see [`LockResult`](type.LockResult.html "type bevy::platform::sync::LockResult"). A `TryLockResult` doesn’t necessarily hold the associated guard in the [`Err`](https://doc.rust-lang.org/nightly/core/result/enum.Result.html#variant.Err "variant core::result::Result::Err") type as the lock might not have been acquired for other reasons.

## Aliased Type

```rust
pub enum TryLockResult<Guard> {
    Ok(Guard),
    Err(TryLockError<Guard>),
}
```

## Variants

1.0.0

### Ok(Guard)

Contains the success value

1.0.0

### Err([TryLockError](enum.TryLockError.html "enum bevy::platform::sync::TryLockError")<Guard>)

Contains the error value