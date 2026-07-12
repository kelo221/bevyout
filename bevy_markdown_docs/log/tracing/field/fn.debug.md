[bevy](../../../index.html)::[log](../../index.html)::[tracing](../index.html)::[field](index.html)

# Function debug 

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#377-379)

```rust
pub fn debug<T>(t: T) -> DebugValue<T>where
    T: Debug,
```

Wraps a type implementing `fmt::Debug` as a `Value` that can be recorded using its `Debug` implementation.