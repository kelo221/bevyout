[bevy](../../../index.html)::[log](../../index.html)::[tracing\_subscriber](../index.html)::[filter](index.html)

# Function dynamic\_filter\_fn 

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/filter/filter_fn.rs.html#175-177)

```rust
pub fn dynamic_filter_fn<S, F>(f: F) -> DynFilterFn<S, F>where
    F: Fn(&Metadata<'_>, &Context<'_, S>) -> bool,
```

Constructs a [`DynFilterFn`](struct.DynFilterFn.html "struct bevy::log::tracing_subscriber::filter::DynFilterFn") from a function or closure that returns `true` if a span or event should be enabled within a particular [span context](../layer/struct.Context.html "struct bevy::log::tracing_subscriber::layer::Context").

This is equivalent to calling [`DynFilterFn::new`](struct.DynFilterFn.html#method.new "associated function bevy::log::tracing_subscriber::filter::DynFilterFn::new").

Unlike [`filter_fn`](fn.filter_fn.html "fn bevy::log::tracing_subscriber::filter::filter_fn"), this function takes a closure or function pointer taking the [`Metadata`](../../tracing/struct.Metadata.html "struct bevy::log::tracing::Metadata") for a span or event _and_ the current [`Context`](../layer/struct.Context.html "struct bevy::log::tracing_subscriber::layer::Context"). This means that a [`DynFilterFn`](struct.DynFilterFn.html "struct bevy::log::tracing_subscriber::filter::DynFilterFn") can choose whether to enable spans or events based on information about the _current_ span (or its parents).

If this is _not_ necessary, use [`filter_fn`](fn.filter_fn.html "fn bevy::log::tracing_subscriber::filter::filter_fn") instead.

The returned [`DynFilterFn`](struct.DynFilterFn.html "struct bevy::log::tracing_subscriber::filter::DynFilterFn") can be used for both [per-layer filtering](../layer/index.html#per-layer-filtering "mod bevy::log::tracing_subscriber::layer") (using its [`Filter`](../layer/trait.Filter.html "trait bevy::log::tracing_subscriber::layer::Filter") implementation) and [global filtering](../layer/index.html#global-filtering "mod bevy::log::tracing_subscriber::layer") (using its [`Layer`](../trait.Layer.html "trait bevy::log::tracing_subscriber::Layer") implementation).

See the [documentation on filtering with layers](../layer/index.html#filtering-with-layers "mod bevy::log::tracing_subscriber::layer") for details.

## Examples

```rust
use tracing_subscriber::{
    layer::{Layer, SubscriberExt},
    filter,
    util::SubscriberInitExt,
};

// Only enable spans or events within a span named "interesting_span".
let my_filter = filter::dynamic_filter_fn(|metadata, cx| {
    // If this *is* "interesting_span", make sure to enable it.
    if metadata.is_span() && metadata.name() == "interesting_span" {
        return true;
    }

    // Otherwise, are we in an interesting span?
    if let Some(current_span) = cx.lookup_current() {
        return current_span.name() == "interesting_span";
    }

    false
});

let my_layer = tracing_subscriber::fmt::layer();

tracing_subscriber::registry()
    .with(my_layer.with_filter(my_filter))
    .init();

// This event will not be enabled.
tracing::info!("something happened");

tracing::info_span!("interesting_span").in_scope(|| {
    // This event will be enabled.
    tracing::debug!("something else happened");
});
```