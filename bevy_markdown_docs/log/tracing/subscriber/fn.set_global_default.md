[bevy](../../../index.html)::[log](../../index.html)::[tracing](../index.html)::[subscriber](index.html)

# Function set\_global\_default 

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/subscriber.rs.html#38-40)

```rust
pub fn set_global_default<S>(subscriber: S) -> Result<(), SetGlobalDefaultError>where
    S: Subscriber + Send + Sync + 'static,
```

Sets this subscriber as the global default for the duration of the entire program. Will be used as a fallback if no thread-local subscriber has been set in a thread (using `with_default`.)

Can only be set once; subsequent attempts to set the global default will fail. Returns whether the initialization was successful.

Note: Libraries should _NOT_ call `set_global_default()`! That will cause conflicts when executables try to set them later.