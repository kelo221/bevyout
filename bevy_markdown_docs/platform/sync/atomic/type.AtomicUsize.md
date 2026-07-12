[bevy](../../../index.html)::[platform](../../index.html)::[sync](../index.html)::[atomic](index.html)

# Type Alias AtomicUsize 

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/core/sync/atomic.rs.html#3860-3864)

```rust
pub type AtomicUsize = Atomic<usize>;
```

An integer type which can be safely shared between threads.

This type has the same size and bit validity as the underlying integer type, [`usize`](https://doc.rust-lang.org/nightly/std/primitive.usize.html "primitive usize"). However, the alignment of this type is always equal to its size, even on targets where [`usize`](https://doc.rust-lang.org/nightly/std/primitive.usize.html "primitive usize") has a lesser alignment.

For more about the differences between atomic types and non-atomic types as well as information about the portability of this type, please see the [module-level documentation](https://doc.rust-lang.org/nightly/core/sync/atomic/index.html "mod core::sync::atomic").

**Note:** This type is only available on platforms that support atomic loads and stores of [`usize`](https://doc.rust-lang.org/nightly/std/primitive.usize.html "primitive usize").

## Aliased Type

```rust
pub struct AtomicUsize { /* private fields */ }
```