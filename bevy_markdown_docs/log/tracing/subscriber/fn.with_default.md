[bevy](../../../index.html)::[log](../../index.html)::[tracing](../index.html)::[subscriber](index.html)

# Function with\_default 

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/subscriber.rs.html#20-22)

```rust
pub fn with_default<T, S>(subscriber: S, f: impl FnOnce() -> T) -> Twhere
    S: Subscriber + Send + Sync + 'static,
```

Available on **crate feature `std`** only.

Sets this [`Subscriber`](../trait.Subscriber.html "trait bevy::log::tracing::Subscriber") as the default for the current thread for the duration of a closure.

The default subscriber is used when creating a new [`Span`](../struct.Span.html "struct bevy::log::tracing::Span") or [`Event`](../struct.Event.html "struct bevy::log::tracing::Event").