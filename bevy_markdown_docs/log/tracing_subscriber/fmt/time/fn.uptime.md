[bevy](../../../../index.html)::[log](../../../index.html)::[tracing\_subscriber](../../index.html)::[fmt](../index.html)::[time](index.html)

# Function uptime 

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/fmt/time/mod.rs.html#85)

```rust
pub fn uptime() -> Uptime
```

Available on **crate features `fmt` and `std`** only.

Returns a new `Uptime` timestamp provider.

With this timer, timestamps will be formatted with the amount of time elapsed since the timestamp provider was constructed.

This can then be configured further to determine how timestamps should be configured.

This is equivalent to calling

```rust
tracing_subscriber::fmt::time::Uptime::default()
```