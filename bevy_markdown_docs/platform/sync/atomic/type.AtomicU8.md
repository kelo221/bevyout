[bevy](../../../index.html)::[platform](../../index.html)::[sync](../index.html)::[atomic](index.html)

# Type Alias AtomicU8 

1.34.0 · [Source](https://doc.rust-lang.org/nightly/src/core/sync/atomic.rs.html#3635-3651)

```rust
pub type AtomicU8 = Atomic<u8>;
```

An integer type which can be safely shared between threads.

This type has the same size, alignment, and bit validity as the underlying integer type, [`u8`](https://doc.rust-lang.org/nightly/std/primitive.u8.html "primitive u8").

For more about the differences between atomic types and non-atomic types as well as information about the portability of this type, please see the [module-level documentation](https://doc.rust-lang.org/nightly/core/sync/atomic/index.html "mod core::sync::atomic").

**Note:** This type is only available on platforms that support atomic loads and stores of [`u8`](https://doc.rust-lang.org/nightly/std/primitive.u8.html "primitive u8").

## Aliased Type

```rust
pub struct AtomicU8 { /* private fields */ }
```