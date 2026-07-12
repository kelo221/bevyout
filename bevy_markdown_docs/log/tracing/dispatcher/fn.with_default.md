[bevy](../../../index.html)::[log](../../index.html)::[tracing](../index.html)::[dispatcher](index.html)

# Function with\_default 

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/dispatcher.rs.html#254)

```rust
pub fn with_default<T>(dispatcher: &Dispatch, f: impl FnOnce() -> T) -> T
```

Available on **crate feature `std`** only.

Sets this dispatch as the default for the duration of a closure.

The default dispatcher is used when creating a new [span](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/tracing_core/span/index.html "mod tracing_core::span") or [`Event`](../struct.Event.html "struct bevy::log::tracing::Event").

    **Note**: This function required the Rust standard library.
    `no_std` users should use [
    `set_global_default`](fn.set_global_default.html) instead.