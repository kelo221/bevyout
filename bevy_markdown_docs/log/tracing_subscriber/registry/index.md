[bevy](../../../index.html)::[log](../../index.html)::[tracing\_subscriber](../index.html)

# Module registry 

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/lib.rs.html#219)

Storage for span data shared by multiple [`Layer`](../trait.Layer.html "trait bevy::log::tracing_subscriber::Layer")s.

### Using the Span Registry

This module provides the [`Registry`](../struct.Registry.html "struct bevy::log::tracing_subscriber::Registry") type, a [`Subscriber`](../../tracing/trait.Subscriber.html "trait bevy::log::tracing::Subscriber") implementation which tracks per-span data and exposes it to [`Layer`](../trait.Layer.html "trait bevy::log::tracing_subscriber::Layer")s. When a `Registry` is used as the base `Subscriber` of a `Layer` stack, the [`layer::Context`](../layer/struct.Context.html "struct bevy::log::tracing_subscriber::layer::Context") type will provide methods allowing `Layer`s to [look up span data](../layer/struct.Context.html#method.span "method bevy::log::tracing_subscriber::layer::Context::span") stored in the registry. While [`Registry`](../struct.Registry.html "struct bevy::log::tracing_subscriber::Registry") is a reasonable default for storing spans and events, other stores that implement [`LookupSpan`](trait.LookupSpan.html "trait bevy::log::tracing_subscriber::registry::LookupSpan") and [`Subscriber`](../../tracing/trait.Subscriber.html "trait bevy::log::tracing::Subscriber") themselves (with [`SpanData`](trait.SpanData.html "trait bevy::log::tracing_subscriber::registry::SpanData") implemented by the per-span data they store) can be used as a drop-in replacement.

For example, we might create a `Registry` and add multiple `Layer`s like so:

```rust
use tracing_subscriber::{registry::Registry, Layer, prelude::*};

let subscriber = Registry::default()
    .with(FooLayer::new())
    .with(BarLayer::new());
```

If a type implementing `Layer` depends on the functionality of a `Registry` implementation, it should bound its `Subscriber` type parameter with the [`LookupSpan`](trait.LookupSpan.html "trait bevy::log::tracing_subscriber::registry::LookupSpan") trait, like so:

```rust
use tracing_subscriber::{registry, Layer};
use tracing_core::Subscriber;

pub struct MyLayer {
    // ...
}

impl<S> Layer<S> for MyLayer
where
    S: Subscriber + for<'a> registry::LookupSpan<'a>,
{
    // ...
}
```

When this bound is added, the `Layer` implementation will be guaranteed access to the [`Context`](../layer/struct.Context.html "struct bevy::log::tracing_subscriber::layer::Context") methods, such as [`Context::span`](../layer/struct.Context.html#method.span "method bevy::log::tracing_subscriber::layer::Context::span"), that require the root subscriber to be a registry.

## Structs

[Data](struct.Data.html "struct bevy::log::tracing_subscriber::registry::Data")`registry`

Span data stored in a [`Registry`](../struct.Registry.html "struct bevy::log::tracing_subscriber::Registry").

[Extensions](struct.Extensions.html "struct bevy::log::tracing_subscriber::registry::Extensions")

An immutable, read-only reference to a Span’s extensions.

[ExtensionsMut](struct.ExtensionsMut.html "struct bevy::log::tracing_subscriber::registry::ExtensionsMut")

An mutable reference to a Span’s extensions.

[Registry](struct.Registry.html "struct bevy::log::tracing_subscriber::registry::Registry")`registry`

A shared, reusable store for spans.

[Scope](struct.Scope.html "struct bevy::log::tracing_subscriber::registry::Scope")

An iterator over the parents of a span, ordered from leaf to root.

[ScopeFromRoot](struct.ScopeFromRoot.html "struct bevy::log::tracing_subscriber::registry::ScopeFromRoot")`alloc` or `std`

An iterator over the parents of a span, ordered from root to leaf.

[SpanRef](struct.SpanRef.html "struct bevy::log::tracing_subscriber::registry::SpanRef")

A reference to \[span data\] and the associated [registry](trait.LookupSpan.html "trait bevy::log::tracing_subscriber::registry::LookupSpan").

## Traits

[LookupSpan](trait.LookupSpan.html "trait bevy::log::tracing_subscriber::registry::LookupSpan")

Provides access to stored span data.

[SpanData](trait.SpanData.html "trait bevy::log::tracing_subscriber::registry::SpanData")

A stored representation of data associated with a span.