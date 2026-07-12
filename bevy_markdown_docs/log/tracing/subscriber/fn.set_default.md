[bevy](../../../index.html)::[log](../../index.html)::[tracing](../index.html)::[subscriber](index.html)

# Function set\_default 

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/subscriber.rs.html#57-59)

```rust
pub fn set_default<S>(subscriber: S) -> DefaultGuardwhere
    S: Subscriber + Send + Sync + 'static,
```

Available on **crate feature `std`** only.

Sets the [`Subscriber`](../trait.Subscriber.html "trait bevy::log::tracing::Subscriber") as the default for the current thread for the duration of the lifetime of the returned [`DefaultGuard`](../dispatcher/struct.DefaultGuard.html "struct bevy::log::tracing::dispatcher::DefaultGuard").

The default subscriber is used when creating a new [`Span`](../struct.Span.html "struct bevy::log::tracing::Span") or [`Event`](../struct.Event.html "struct bevy::log::tracing::Event").