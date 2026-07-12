[bevy](../../../index.html)::[log](../../index.html)::[tracing](../index.html)::[dispatcher](index.html)

# Function get\_default 

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/dispatcher.rs.html#379-381)

```rust
pub fn get_default<T, F>(f: F) -> Twhere
    F: FnMut(&Dispatch) -> T,
```

Available on **crate feature `std`** only.

Executes a closure with a reference to this thread’s current [dispatcher](../struct.Dispatch.html "struct bevy::log::tracing::Dispatch").

Note that calls to `get_default` should not be nested; if this function is called while inside of another `get_default`, that closure will be provided with `Dispatch::none` rather than the previously set dispatcher.