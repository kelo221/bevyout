[bevy](../../../index.html)::[log](../../index.html)::[tracing\_subscriber](../index.html)::[fmt](index.html)

# Function init 

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/fmt/mod.rs.html#1262)

```rust
pub fn init()
```

Available on **crate features `fmt` and `std`** only.

Install a global tracing subscriber that listens for events and filters based on the value of the [`RUST_LOG` environment variable](../struct.EnvFilter.html#associatedconstant.DEFAULT_ENV "associated constant bevy::log::tracing_subscriber::EnvFilter::DEFAULT_ENV").

The configuration of the subscriber initialized by this function depends on what [feature flags](../index.html#feature-flags "mod bevy::log::tracing_subscriber") are enabled.

If the `tracing-log` feature is enabled, this will also install the LogTracer to convert `Log` records into `tracing` `Event`s.

If the `env-filter` feature is enabled, this is shorthand for

```rust
tracing_subscriber::fmt()
    .with_env_filter(EnvFilter::from_default_env())
    .init();
```

## Panics

Panics if the initialization was unsuccessful, likely because a global subscriber was already installed by another call to `try_init`.