[bevy](../../../index.html)::[log](../../index.html)::[tracing](../index.html)::[dispatcher](index.html)

# Function set\_global\_default 

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/dispatcher.rs.html#299)

```rust
pub fn set_global_default(
    dispatcher: Dispatch,
) -> Result<(), SetGlobalDefaultError>
```

Sets this dispatch as the global default for the duration of the entire program. Will be used as a fallback if no thread-local dispatch has been set in a thread (using `with_default`.)

Can only be set once; subsequent attempts to set the global default will fail. Returns `Err` if the global default has already been set.

    **Warning**: In general, libraries should _not_ call
    `set_global_default()`! Doing so will cause conflicts when
    executables that depend on the library try to set the default later.