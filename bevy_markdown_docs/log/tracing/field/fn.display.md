[bevy](../../../index.html)::[log](../../index.html)::[tracing](../index.html)::[field](index.html)

# Function display 

[Source](https://docs.rs/tracing-core/0.1.36/x86_64-unknown-linux-gnu/src/tracing_core/field.rs.html#368-370)

```rust
pub fn display<T>(t: T) -> DisplayValue<T>where
    T: Display,
```

Wraps a type implementing `fmt::Display` as a `Value` that can be recorded using its `Display` implementation.