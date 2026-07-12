[bevy](../../../index.html)::[platform](../../index.html)::[sync](../index.html)::[atomic](index.html)

# Type Alias AtomicU64 

1.34.0 · [Source](https://doc.rust-lang.org/nightly/src/core/sync/atomic.rs.html#3743-3759)

```rust
pub type AtomicU64 = Atomic<u64>;
```

An integer type which can be safely shared between threads.

This type has the same size and bit validity as the underlying integer type, [`u64`](https://doc.rust-lang.org/nightly/std/primitive.u64.html "primitive u64"). However, the alignment of this type is always equal to its size, even on targets where [`u64`](https://doc.rust-lang.org/nightly/std/primitive.u64.html "primitive u64") has a lesser alignment.

For more about the differences between atomic types and non-atomic types as well as information about the portability of this type, please see the [module-level documentation](https://doc.rust-lang.org/nightly/core/sync/atomic/index.html "mod core::sync::atomic").

**Note:** This type is only available on platforms that support atomic loads and stores of [`u64`](https://doc.rust-lang.org/nightly/std/primitive.u64.html "primitive u64").

## Aliased Type

```rust
pub struct AtomicU64 { /* private fields */ }
```