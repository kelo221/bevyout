[bevy](../../../index.html)::[platform](../../index.html)::[sync](../index.html)::[atomic](index.html)

# Type Alias AtomicBool 

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/sync/atomic.rs.html#393)

```rust
pub type AtomicBool = Atomic<bool>;
```

Available on **`target_has_atomic_load_store=8`** only.

A boolean type which can be safely shared between threads.

This type has the same size, alignment, and bit validity as a [`bool`](https://doc.rust-lang.org/nightly/std/primitive.bool.html "primitive bool").

**Note**: This type is only available on platforms that support atomic loads and stores of `u8`.

## Aliased Type

```rust
pub struct AtomicBool { /* private fields */ }
```