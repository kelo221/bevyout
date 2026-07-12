[bevy](../../../index.html)::[log](../../index.html)::[tracing](../index.html)::[dispatcher](index.html)

# Function set\_default 

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/dispatcher.rs.html#276)

```rust
pub fn set_default(dispatcher: &Dispatch) -> DefaultGuard
```

Available on **crate feature `std`** only.

Sets the dispatch as the default dispatch for the duration of the lifetime of the returned DefaultGuard

    **Note**: This function required the Rust standard library.
    `no_std` users should use [
    `set_global_default`](fn.set_global_default.html) instead.