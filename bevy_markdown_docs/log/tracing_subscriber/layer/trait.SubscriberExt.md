[bevy](../../../index.html)::[log](../../index.html)::[tracing\_subscriber](../index.html)::[layer](index.html)

# Trait SubscriberExt 

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/layer/mod.rs.html#1500)

```rust
pub trait SubscriberExt: Subscriber + Sealed {
    // Provided method
    fn with<L>(self, layer: L) -> Layered<L, Self>
       where L: Layer<Self>,
             Self: Sized { ... }
}
```

Extension trait adding a `with(Layer)` combinator to `Subscriber`s.

## Provided Methods

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/layer/mod.rs.html#1502-1505)

#### fn [with](#method.with)<L>(self, layer: L) -> [Layered](struct.Layered.html "struct bevy::log::tracing_subscriber::layer::Layered")<L, Self>

where L: [Layer](../trait.Layer.html "trait bevy::log::tracing_subscriber::Layer")<Self>, Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Wraps `self` with the provided `layer`.

## Dyn Compatibility

This trait **is** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/layer/mod.rs.html#1899)

### impl<S> [SubscriberExt](../prelude/trait.__tracing_subscriber_SubscriberExt.html "trait bevy::log::tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt") for S

where S: [Subscriber](../../tracing/trait.Subscriber.html "trait bevy::log::tracing::Subscriber"),