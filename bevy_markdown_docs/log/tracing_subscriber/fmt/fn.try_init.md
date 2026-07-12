[bevy](../../../index.html)::[log](../../index.html)::[tracing\_subscriber](../index.html)::[fmt](index.html)

# Function try\_init 

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/fmt/mod.rs.html#1200)

```rust
pub fn try_init() -> Result<(), Box<dyn Error + Send + Sync>>
```

Available on **crate features `fmt` and `std`** only.

Install a global tracing subscriber that listens for events and filters based on the value of the [`RUST_LOG` environment variable](../struct.EnvFilter.html#associatedconstant.DEFAULT_ENV "associated constant bevy::log::tracing_subscriber::EnvFilter::DEFAULT_ENV"), if one is not already set.

If the `tracing-log` feature is enabled, this will also install the [`LogTracer`](https://docs.rs/tracing-log/0.1.0/tracing_log/struct.LogTracer.html) to convert `log` records into `tracing` `Event`s.

This is shorthand for

```rust
tracing_subscriber::fmt().try_init()
```

## Errors

Returns an Error if the initialization was unsuccessful, likely because a global subscriber was already installed by another call to `try_init`.