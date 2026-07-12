[bevy](../../../../index.html)::[log](../../../index.html)::[tracing\_subscriber](../../index.html)::[fmt](../index.html)::[writer](index.html)

# Type Alias OptionalWriter 

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/fmt/writer.rs.html#574)

```rust
pub type OptionalWriter<T> = EitherWriter<T, Sink>;
```

Available on **crate features `fmt` and `std`** only.

A [writer](https://doc.rust-lang.org/nightly/std/io/trait.Write.html "trait std::io::Write") which may or may not be enabled.

This may be used by [`MakeWriter`](../trait.MakeWriter.html "trait bevy::log::tracing_subscriber::fmt::MakeWriter") implementations that wish to conditionally enable or disable the returned writer based on a span or event’s [`Metadata`](../../../tracing/struct.Metadata.html "struct bevy::log::tracing::Metadata").

## Aliased Type

```rust
pub enum OptionalWriter<T> {
    A(T),
    B(Sink),
}
```

## Variants

### A(T)

A writer of type `A`.

### B([Sink](https://doc.rust-lang.org/nightly/core/io/util/struct.Sink.html "struct core::io::util::Sink"))

A writer of type `B`.