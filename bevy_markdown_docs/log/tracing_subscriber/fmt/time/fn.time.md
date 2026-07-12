[bevy](../../../../index.html)::[log](../../../index.html)::[tracing\_subscriber](../../index.html)::[fmt](../index.html)::[time](index.html)

# Function time 

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/fmt/time/mod.rs.html#67)

```rust
pub fn time() -> SystemTime
```

Available on **crate features `fmt` and `std`** only.

Returns a new `SystemTime` timestamp provider.

This can then be configured further to determine how timestamps should be configured.

This is equivalent to calling

```rust
tracing_subscriber::fmt::time::SystemTime::default()
```