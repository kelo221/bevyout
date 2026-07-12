[bevy](../../../index.html)::[log](../../index.html)::[tracing\_subscriber](../index.html)::[filter](index.html)

# Function filter\_fn 

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/filter/filter_fn.rs.html#104-106)

```rust
pub fn filter_fn<F>(f: F) -> FilterFn<F>where
    F: Fn(&Metadata<'_>) -> bool,
```

Constructs a [`FilterFn`](struct.FilterFn.html "struct bevy::log::tracing_subscriber::filter::FilterFn"), from a function or closure that returns `true` if a span or event should be enabled, based on its [`Metadata`](../../tracing/struct.Metadata.html "struct bevy::log::tracing::Metadata").

The returned [`FilterFn`](struct.FilterFn.html "struct bevy::log::tracing_subscriber::filter::FilterFn") can be used for both [per-layer filtering](../layer/index.html#per-layer-filtering "mod bevy::log::tracing_subscriber::layer") (using its [`Filter`](../layer/trait.Filter.html "trait bevy::log::tracing_subscriber::layer::Filter") implementation) and [global filtering](../layer/index.html#global-filtering "mod bevy::log::tracing_subscriber::layer") (using its [`Layer`](../trait.Layer.html "trait bevy::log::tracing_subscriber::Layer") implementation).

See the [documentation on filtering with layers](../layer/index.html#filtering-with-layers "mod bevy::log::tracing_subscriber::layer") for details.

This is equivalent to calling [`FilterFn::new`](struct.FilterFn.html#method.new "associated function bevy::log::tracing_subscriber::filter::FilterFn::new").

## Examples

```rust
use tracing_subscriber::{
    layer::{Layer, SubscriberExt},
    filter,
    util::SubscriberInitExt,
};

let my_filter = filter::filter_fn(|metadata| {
    // Only enable spans or events with the target "interesting_things"
    metadata.target() == "interesting_things"
});

let my_layer = tracing_subscriber::fmt::layer();

tracing_subscriber::registry()
    .with(my_layer.with_filter(my_filter))
    .init();

// This event will not be enabled.
tracing::warn!("something important but uninteresting happened!");

// This event will be enabled.
tracing::debug!(target: "interesting_things", "an interesting minor detail...");
```