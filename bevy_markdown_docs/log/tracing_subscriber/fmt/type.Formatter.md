[bevy](../../../index.html)::[log](../../index.html)::[tracing\_subscriber](../index.html)::[fmt](index.html)

# Type Alias Formatter 

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/fmt/mod.rs.html#240-244)

```rust
pub type Formatter<N = DefaultFields, E = Format, W = fn() -> Stdout> = Layered<Layer<Registry, N, E, W>, Registry>;
```

Available on **crate features `fmt` and `std`** only.

A `Subscriber` that logs formatted representations of `tracing` events. This type only logs formatted events; it does not perform any filtering.

## Aliased Type

```rust
pub struct Formatter<N = DefaultFields, E = Format, W = fn() -> Stdout> { /* private fields */ }
```