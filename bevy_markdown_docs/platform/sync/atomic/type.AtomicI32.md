[bevy](../../../index.html)::[platform](../../index.html)::[sync](../index.html)::[atomic](index.html)

# Type Alias AtomicI32 

1.34.0 · [Source](https://doc.rust-lang.org/nightly/src/core/sync/atomic.rs.html#3689-3705)

```rust
pub type AtomicI32 = Atomic<i32>;
```

An integer type which can be safely shared between threads.

This type has the same size and bit validity as the underlying integer type, [`i32`](https://doc.rust-lang.org/nightly/std/primitive.i32.html "primitive i32"). However, the alignment of this type is always equal to its size, even on targets where [`i32`](https://doc.rust-lang.org/nightly/std/primitive.i32.html "primitive i32") has a lesser alignment.

For more about the differences between atomic types and non-atomic types as well as information about the portability of this type, please see the [module-level documentation](https://doc.rust-lang.org/nightly/core/sync/atomic/index.html "mod core::sync::atomic").

**Note:** This type is only available on platforms that support atomic loads and stores of [`i32`](https://doc.rust-lang.org/nightly/std/primitive.i32.html "primitive i32").

## Aliased Type

```rust
pub struct AtomicI32 { /* private fields */ }
```