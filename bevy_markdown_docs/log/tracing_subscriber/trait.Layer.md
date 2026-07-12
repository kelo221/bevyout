[bevy](../../index.html)::[log](../index.html)::[tracing\_subscriber](index.html)

# Trait Layer 

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/layer/mod.rs.html#728)

```rust
pub trait Layer<S>: 'staticwhere
    S: Subscriber,{
    // Provided methods
    fn on_register_dispatch(&self, subscriber: &Dispatch) { ... }
    fn on_layer(&mut self, subscriber: &mut S) { ... }
    fn register_callsite(
        &self,
        metadata: &'static Metadata<'static>,
    ) -> Interest { ... }
    fn enabled(&self, metadata: &Metadata<'_>, ctx: Context<'_, S>) -> bool { ... }
    fn on_new_span(&self, attrs: &Attributes<'_>, id: &Id, ctx: Context<'_, S>) { ... }
    fn on_record(&self, _span: &Id, _values: &Record<'_>, _ctx: Context<'_, S>) { ... }
    fn on_follows_from(&self, _span: &Id, _follows: &Id, _ctx: Context<'_, S>) { ... }
    fn event_enabled(&self, _event: &Event<'_>, _ctx: Context<'_, S>) -> bool { ... }
    fn on_event(&self, _event: &Event<'_>, _ctx: Context<'_, S>) { ... }
    fn on_enter(&self, _id: &Id, _ctx: Context<'_, S>) { ... }
    fn on_exit(&self, _id: &Id, _ctx: Context<'_, S>) { ... }
    fn on_close(&self, _id: Id, _ctx: Context<'_, S>) { ... }
    fn on_id_change(&self, _old: &Id, _new: &Id, _ctx: Context<'_, S>) { ... }
    fn and_then<L>(self, layer: L) -> Layered<L, Self, S>
       where L: Layer<S>,
             Self: Sized { ... }
    fn with_subscriber(self, inner: S) -> Layered<Self, S>
       where Self: Sized { ... }
    fn with_filter<F>(self, filter: F) -> Filtered<Self, F, S>
       where Self: Sized,
             F: Filter<S> { ... }
    fn boxed(self) -> Box<dyn Layer<S> + Send + Sync>
       where Self: Sized + Layer<S> + Send + Sync + 'static,
             S: Subscriber { ... }
}
```

A composable handler for `tracing` events.

A `Layer` implements a behavior for recording or collecting traces that can be composed together with other `Layer`s to build a [`Subscriber`](../tracing/trait.Subscriber.html "trait bevy::log::tracing::Subscriber"). See the [module-level documentation](layer/index.html "mod bevy::log::tracing_subscriber::layer") for details.

## Provided Methods

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/layer/mod.rs.html#754)

#### fn [on\_register\_dispatch](#method.on_register_dispatch)(&self, subscriber: &[Dispatch](../tracing/struct.Dispatch.html "struct bevy::log::tracing::Dispatch"))

Performs late initialization when installing this layer as a [`Subscriber`](../tracing/trait.Subscriber.html "trait bevy::log::tracing::Subscriber").

###### Avoiding Memory Leaks

`Layer`s should not store the [`Dispatch`](../tracing/struct.Dispatch.html "struct bevy::log::tracing::Dispatch") pointing to the [`Subscriber`](../tracing/trait.Subscriber.html "trait bevy::log::tracing::Subscriber") that they are a part of. Because the `Dispatch` owns the `Subscriber`, storing the `Dispatch` within the `Subscriber` will create a reference count cycle, preventing the `Dispatch` from ever being dropped.

Instead, when it is necessary to store a cyclical reference to the `Dispatch` within a `Layer`, use [`Dispatch::downgrade`](../tracing/struct.Dispatch.html#method.downgrade "method bevy::log::tracing::Dispatch::downgrade") to convert a `Dispatch` into a [`WeakDispatch`](../tracing/dispatcher/struct.WeakDispatch.html "struct bevy::log::tracing::dispatcher::WeakDispatch"). This type is analogous to [`std::sync::Weak`](../../platform/sync/struct.Weak.html "struct bevy::platform::sync::Weak"), and does not create a reference count cycle. A [`WeakDispatch`](../tracing/dispatcher/struct.WeakDispatch.html "struct bevy::log::tracing::dispatcher::WeakDispatch") can be stored within a subscriber without causing a memory leak, and can be [upgraded](../tracing/dispatcher/struct.WeakDispatch.html#method.upgrade "method bevy::log::tracing::dispatcher::WeakDispatch::upgrade") into a `Dispatch` temporarily when the `Dispatch` must be accessed by the subscriber.

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/layer/mod.rs.html#785)

#### fn [on\_layer](#method.on_layer)(&mut self, subscriber: [&mut S](https://doc.rust-lang.org/nightly/std/primitive.reference.html))

Performs late initialization when attaching a `Layer` to a [`Subscriber`](../tracing/trait.Subscriber.html "trait bevy::log::tracing::Subscriber").

This is a callback that is called when the `Layer` is added to a [`Subscriber`](../tracing/trait.Subscriber.html "trait bevy::log::tracing::Subscriber") (e.g. in [`Layer::with_subscriber`](trait.Layer.html#method.with_subscriber "method bevy::log::tracing_subscriber::Layer::with_subscriber") and [`SubscriberExt::with`](prelude/trait.__tracing_subscriber_SubscriberExt.html#method.with "method bevy::log::tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt::with")). Since this can only occur before the [`Subscriber`](../tracing/trait.Subscriber.html "trait bevy::log::tracing::Subscriber") has been set as the default, both the `Layer` and [`Subscriber`](../tracing/trait.Subscriber.html "trait bevy::log::tracing::Subscriber") are passed to this method _mutably_. This gives the `Layer` the opportunity to set any of its own fields with values received by method calls on the [`Subscriber`](../tracing/trait.Subscriber.html "trait bevy::log::tracing::Subscriber").

For example, [`Filtered`](filter/struct.Filtered.html "struct bevy::log::tracing_subscriber::filter::Filtered") layers implement `on_layer` to call the [`Subscriber`](../tracing/trait.Subscriber.html "trait bevy::log::tracing::Subscriber")’s [`register_filter`](registry/trait.LookupSpan.html#method.register_filter "method bevy::log::tracing_subscriber::registry::LookupSpan::register_filter") method, and store the returned [`FilterId`](filter/struct.FilterId.html "struct bevy::log::tracing_subscriber::filter::FilterId") as a field.

**Note** In most cases, `Layer` implementations will not need to implement this method. However, in cases where a type implementing `Layer` wraps one or more other types that implement `Layer`, like the [`Layered`](layer/struct.Layered.html "struct bevy::log::tracing_subscriber::layer::Layered") and [`Filtered`](filter/struct.Filtered.html "struct bevy::log::tracing_subscriber::filter::Filtered") types in this crate, that type MUST ensure that the inner `Layer`s’ `on_layer` methods are called. Otherwise, functionality that relies on `on_layer`, such as [per-layer filtering](#per-layer-filtering), may not work correctly.

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/layer/mod.rs.html#829)

#### fn [register\_callsite](#method.register_callsite)(&self, metadata: &'static [Metadata](../tracing/struct.Metadata.html "struct bevy::log::tracing::Metadata")<'static>) -> [Interest](../tracing/subscriber/struct.Interest.html "struct bevy::log::tracing::subscriber::Interest")

Registers a new callsite with this layer, returning whether or not the layer is interested in being notified about the callsite, similarly to [`Subscriber::register_callsite`](../tracing/trait.Subscriber.html#method.register_callsite "method bevy::log::tracing::Subscriber::register_callsite").

By default, this returns [`Interest::always()`](../tracing/subscriber/struct.Interest.html#method.always "associated function bevy::log::tracing::subscriber::Interest::always") if [`self.enabled`](trait.Layer.html#method.enabled "method bevy::log::tracing_subscriber::Layer::enabled") returns true, or [`Interest::never()`](../tracing/subscriber/struct.Interest.html#method.never "associated function bevy::log::tracing::subscriber::Interest::never") if it returns false.

**Note**: This method (and [
`Layer::enabled`](#method.enabled)) determine whether a span or event is
globally enabled, _not_ whether the individual layer will be
notified about that span or event. This is intended to be used
by layers that implement filtering for the entire stack. Layers which do
not wish to be notified about certain spans or events but do not wish to
globally disable them should ignore those spans or events in their
[`on_event`](#method.on_event),
[`on_enter`](#method.on_enter),
[`on_exit`](#method.on_exit), and other notification
methods.

See [the trait-level documentation](#filtering-with-layers) for more information on filtering with `Layer`s.

Layers may also implement this method to perform any behaviour that should be run once per callsite. If the layer wishes to use `register_callsite` for per-callsite behaviour, but does not want to globally enable or disable those callsites, it should always return [`Interest::always()`](../tracing/subscriber/struct.Interest.html#method.always "associated function bevy::log::tracing::subscriber::Interest::always").

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/layer/mod.rs.html#869)

#### fn [enabled](#method.enabled)(&self, metadata: &[Metadata](../tracing/struct.Metadata.html "struct bevy::log::tracing::Metadata")<'\_>, ctx: [Context](layer/struct.Context.html "struct bevy::log::tracing_subscriber::layer::Context")<'\_, S>) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Returns `true` if this layer is interested in a span or event with the given `metadata` in the current [`Context`](layer/struct.Context.html "struct bevy::log::tracing_subscriber::layer::Context"), similarly to [`Subscriber::enabled`](../tracing/trait.Subscriber.html#tymethod.enabled "method bevy::log::tracing::Subscriber::enabled").

By default, this always returns `true`, allowing the wrapped subscriber to choose to disable the span.

**Note**: This method (and [
`Layer::register_callsite`](#method.register_callsite)) determine whether a span or event is
globally enabled, _not_ whether the individual layer will be
notified about that span or event. This is intended to be used
by layers that implement filtering for the entire stack. Layers which do
not wish to be notified about certain spans or events but do not wish to
globally disable them should ignore those spans or events in their
[`on_event`](#method.on_event),
[`on_enter`](#method.on_enter),
[`on_exit`](#method.on_exit), and other notification
methods.

See [the trait-level documentation](#filtering-with-layers) for more information on filtering with `Layer`s.

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/layer/mod.rs.html#876)

#### fn [on\_new\_span](#method.on_new_span)(&self, attrs: &[Attributes](../tracing/span/struct.Attributes.html "struct bevy::log::tracing::span::Attributes")<'\_>, id: &[Id](../tracing/struct.Id.html "struct bevy::log::tracing::Id"), ctx: [Context](layer/struct.Context.html "struct bevy::log::tracing_subscriber::layer::Context")<'\_, S>)

Notifies this layer that a new span was constructed with the given `Attributes` and `Id`.

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/layer/mod.rs.html#893)

#### fn [on\_record](#method.on_record)(&self, \_span: &[Id](../tracing/struct.Id.html "struct bevy::log::tracing::Id"), \_values: &[Record](../tracing/span/struct.Record.html "struct bevy::log::tracing::span::Record")<'\_>, \_ctx: [Context](layer/struct.Context.html "struct bevy::log::tracing_subscriber::layer::Context")<'\_, S>)

Notifies this layer that a span with the given `Id` recorded the given `values`.

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/layer/mod.rs.html#900)

#### fn [on\_follows\_from](#method.on_follows_from)(&self, \_span: &[Id](../tracing/struct.Id.html "struct bevy::log::tracing::Id"), \_follows: &[Id](../tracing/struct.Id.html "struct bevy::log::tracing::Id"), \_ctx: [Context](layer/struct.Context.html "struct bevy::log::tracing_subscriber::layer::Context")<'\_, S>)

Notifies this layer that a span with the ID `span` recorded that it follows from the span with the ID `follows`.

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/layer/mod.rs.html#923)

#### fn [event\_enabled](#method.event_enabled)(&self, \_event: &[Event](../tracing/struct.Event.html "struct bevy::log::tracing::Event")<'\_>, \_ctx: [Context](layer/struct.Context.html "struct bevy::log::tracing_subscriber::layer::Context")<'\_, S>) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Called before [`on_event`](trait.Layer.html#method.on_event "method bevy::log::tracing_subscriber::Layer::on_event"), to determine if `on_event` should be called.

**Note**: This method determines whether an event is globally enabled,
_not_ whether the individual `Layer` will be notified about the
event. This is intended to be used by `Layer`s that implement
filtering for the entire stack. `Layer`s which do not wish to be
notified about certain events but do not wish to globally disable them
should ignore those events in their [on\_event](trait.Layer.html#method.on_event "method bevy::log::tracing_subscriber::Layer::on_event").

See [the trait-level documentation](#filtering-with-layers) for more information on filtering with `Layer`s.

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/layer/mod.rs.html#928)

#### fn [on\_event](#method.on_event)(&self, \_event: &[Event](../tracing/struct.Event.html "struct bevy::log::tracing::Event")<'\_>, \_ctx: [Context](layer/struct.Context.html "struct bevy::log::tracing_subscriber::layer::Context")<'\_, S>)

Notifies this layer that an event has occurred.

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/layer/mod.rs.html#931)

#### fn [on\_enter](#method.on_enter)(&self, \_id: &[Id](../tracing/struct.Id.html "struct bevy::log::tracing::Id"), \_ctx: [Context](layer/struct.Context.html "struct bevy::log::tracing_subscriber::layer::Context")<'\_, S>)

Notifies this layer that a span with the given ID was entered.

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/layer/mod.rs.html#934)

#### fn [on\_exit](#method.on_exit)(&self, \_id: &[Id](../tracing/struct.Id.html "struct bevy::log::tracing::Id"), \_ctx: [Context](layer/struct.Context.html "struct bevy::log::tracing_subscriber::layer::Context")<'\_, S>)

Notifies this layer that the span with the given ID was exited.

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/layer/mod.rs.html#937)

#### fn [on\_close](#method.on_close)(&self, \_id: [Id](../tracing/struct.Id.html "struct bevy::log::tracing::Id"), \_ctx: [Context](layer/struct.Context.html "struct bevy::log::tracing_subscriber::layer::Context")<'\_, S>)

Notifies this layer that the span with the given ID has been closed.

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/layer/mod.rs.html#941)

#### fn [on\_id\_change](#method.on_id_change)(&self, \_old: &[Id](../tracing/struct.Id.html "struct bevy::log::tracing::Id"), \_new: &[Id](../tracing/struct.Id.html "struct bevy::log::tracing::Id"), \_ctx: [Context](layer/struct.Context.html "struct bevy::log::tracing_subscriber::layer::Context")<'\_, S>)

Notifies this layer that a span ID has been cloned, and that the subscriber returned a different ID.

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/layer/mod.rs.html#1040-1043)

#### fn [and\_then](#method.and_then)<L>(self, layer: L) -> [Layered](layer/struct.Layered.html "struct bevy::log::tracing_subscriber::layer::Layered")<L, Self, S>

where L: [Layer](trait.Layer.html "trait bevy::log::tracing_subscriber::Layer")<S>, Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Composes this layer around the given `Layer`, returning a `Layered` struct implementing `Layer`.

The returned `Layer` will call the methods on this `Layer` and then those of the new `Layer`, before calling the methods on the subscriber it wraps. For example:

```rust
pub struct FooLayer {
    // ...
}

pub struct BarLayer {
    // ...
}

pub struct MySubscriber {
    // ...
}

impl<S: Subscriber> Layer<S> for FooLayer {
    // ...
}

impl<S: Subscriber> Layer<S> for BarLayer {
    // ...
}

let subscriber = FooLayer::new()
    .and_then(BarLayer::new())
    .with_subscriber(MySubscriber::new());
```

Multiple layers may be composed in this manner:

```rust
pub struct BazLayer {
    // ...
}

impl<S: Subscriber> Layer<S> for BazLayer {
    // ...
}

let subscriber = FooLayer::new()
    .and_then(BarLayer::new())
    .and_then(BazLayer::new())
    .with_subscriber(MySubscriber::new());
```

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/layer/mod.rs.html#1092-1094)

#### fn [with\_subscriber](#method.with_subscriber)(self, inner: S) -> [Layered](layer/struct.Layered.html "struct bevy::log::tracing_subscriber::layer::Layered")<Self, S>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Composes this `Layer` with the given [`Subscriber`](../tracing/trait.Subscriber.html "trait bevy::log::tracing::Subscriber"), returning a `Layered` struct that implements [`Subscriber`](../tracing/trait.Subscriber.html "trait bevy::log::tracing::Subscriber").

The returned `Layered` subscriber will call the methods on this `Layer` and then those of the wrapped subscriber.

For example:

```rust
pub struct FooLayer {
    // ...
}

pub struct MySubscriber {
    // ...
}

impl<S: Subscriber> Layer<S> for FooLayer {
    // ...
}

let subscriber = FooLayer::new()
    .with_subscriber(MySubscriber::new());
```

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/layer/mod.rs.html#1111-1114)

#### fn [with\_filter](#method.with_filter)<F>(self, filter: F) -> [Filtered](filter/struct.Filtered.html "struct bevy::log::tracing_subscriber::filter::Filtered")<Self, F, S>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), F: [Filter](layer/trait.Filter.html "trait bevy::log::tracing_subscriber::layer::Filter")<S>,

Available on **crate features `registry` and `std`** only.

Combines `self` with a [`Filter`](layer/trait.Filter.html "trait bevy::log::tracing_subscriber::layer::Filter"), returning a [`Filtered`](filter/struct.Filtered.html "struct bevy::log::tracing_subscriber::filter::Filtered") layer.

The [`Filter`](layer/trait.Filter.html "trait bevy::log::tracing_subscriber::layer::Filter") will control which spans and events are enabled for this layer. See [the trait-level documentation](layer/index.html#per-layer-filtering "mod bevy::log::tracing_subscriber::layer") for details on per-layer filtering.

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/layer/mod.rs.html#1235-1239)

#### fn [boxed](#method.boxed)(self) -> [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Layer](trait.Layer.html "trait bevy::log::tracing_subscriber::Layer")<S> + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync")\>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [Layer](trait.Layer.html "trait bevy::log::tracing_subscriber::Layer")<S> + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static, S: [Subscriber](../tracing/trait.Subscriber.html "trait bevy::log::tracing::Subscriber"),

Available on **crate features `alloc` or `std`** only.

Erases the type of this [`Layer`](trait.Layer.html "trait bevy::log::tracing_subscriber::Layer"), returning a [`Box`](../../prelude/struct.Box.html "struct bevy::prelude::Box")ed `dyn Layer` trait object.

This can be used when a function returns a `Layer` which may be of one of several types, or when a `Layer` subscriber has a very long type signature.

##### Examples

The following example will _not_ compile, because the value assigned to `log_layer` may have one of several different types:

[ⓘ](# "This example deliberately fails to compile")

```rust
use tracing_subscriber::{Layer, filter::LevelFilter, prelude::*};
use std::{path::PathBuf, fs::File, io};

/// Configures whether logs are emitted to a file, to stdout, or to stderr.
pub enum LogConfig {
    File(PathBuf),
    Stdout,
    Stderr,
}

let config = // ...

// Depending on the config, construct a layer of one of several types.
let log_layer = match config {
    // If logging to a file, use a maximally-verbose configuration.
    LogConfig::File(path) => {
        let file = File::create(path)?;
        tracing_subscriber::fmt::layer()
            .with_thread_ids(true)
            .with_thread_names(true)
            // Selecting the JSON logging format changes the layer's
            // type.
            .json()
            .with_span_list(true)
            // Setting the writer to use our log file changes the
            // layer's type again.
            .with_writer(file)
    },

    // If logging to stdout, use a pretty, human-readable configuration.
    LogConfig::Stdout => tracing_subscriber::fmt::layer()
        // Selecting the "pretty" logging format changes the
        // layer's type!
        .pretty()
        .with_writer(io::stdout)
        // Add a filter based on the RUST_LOG environment variable;
        // this changes the type too!
        .and_then(tracing_subscriber::EnvFilter::from_default_env()),

    // If logging to stdout, only log errors and warnings.
    LogConfig::Stderr => tracing_subscriber::fmt::layer()
        // Changing the writer changes the layer's type
        .with_writer(io::stderr)
        // Only log the `WARN` and `ERROR` levels. Adding a filter
        // changes the layer's type to `Filtered<LevelFilter, ...>`.
        .with_filter(LevelFilter::WARN),
};

tracing_subscriber::registry()
    .with(log_layer)
    .init();
```

However, adding a call to `.boxed()` after each match arm erases the layer’s type, so this code _does_ compile:

```rust
let log_layer = match config {
    LogConfig::File(path) => {
        let file = File::create(path)?;
        tracing_subscriber::fmt::layer()
            .with_thread_ids(true)
            .with_thread_names(true)
            .json()
            .with_span_list(true)
            .with_writer(file)
            // Erase the type by boxing the layer
            .boxed()
    },

    LogConfig::Stdout => tracing_subscriber::fmt::layer()
        .pretty()
        .with_writer(io::stdout)
        .and_then(tracing_subscriber::EnvFilter::from_default_env())
        // Erase the type by boxing the layer
        .boxed(),

    LogConfig::Stderr => tracing_subscriber::fmt::layer()
        .with_writer(io::stderr)
        .with_filter(LevelFilter::WARN)
        // Erase the type by boxing the layer
        .boxed(),
};

tracing_subscriber::registry()
    .with(log_layer)
    .init();
```

##### [Examples found in repository](#scraped-examples)[?](../../../scrape-examples-help.html)

examples/app/log\_layers.rs ([line 34](../../../src/log_layers/log_layers.rs.html#34))

```rust
29fn custom_layer(_app: &mut App) -> Option<BoxedLayer> {
30    // You can provide multiple layers like this, since Vec<Layer> is also a layer:
31    Some(Box::new(vec![
32        bevy::log::tracing_subscriber::fmt::layer()
33            .with_file(true)
34            .boxed(),
35        CustomLayer.boxed(),
36    ]))
37}
```

Hide additional examples

examples/app/log\_layers\_ecs.rs ([line 111](../../../src/log_layers_ecs/log_layers_ecs.rs.html#111))

```rust
101fn custom_layer(app: &mut App) -> Option<BoxedLayer> {
102    let (sender, receiver) = mpsc::channel();
103
104    let layer = CaptureLayer { sender };
105    let resource = CapturedLogMessages(receiver);
106
107    app.insert_non_send(resource);
108    app.add_message::<LogMessage>();
109    app.add_systems(Update, transfer_log_messages);
110
111    Some(layer.boxed())
112}
```

## Trait Implementations

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/layer/mod.rs.html#1769-1771)

### impl<S> [Layer](trait.Layer.html "trait bevy::log::tracing_subscriber::Layer")<S> for [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Layer](trait.Layer.html "trait bevy::log::tracing_subscriber::Layer")<S> + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync")\>

where S: [Subscriber](../tracing/trait.Subscriber.html "trait bevy::log::tracing::Subscriber"),

Available on **crate features `alloc` or `std`** only.

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/layer/mod.rs.html#1773)

#### fn [on\_register\_dispatch](trait.Layer.html#method.on_register_dispatch)(&self, subscriber: &[Dispatch](../tracing/struct.Dispatch.html "struct bevy::log::tracing::Dispatch"))

Performs late initialization when installing this layer as a [`Subscriber`](../tracing/trait.Subscriber.html "trait bevy::log::tracing::Subscriber"). [Read more](trait.Layer.html#method.on_register_dispatch)

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/layer/mod.rs.html#1773)

#### fn [on\_layer](trait.Layer.html#method.on_layer)(&mut self, subscriber: [&mut S](https://doc.rust-lang.org/nightly/std/primitive.reference.html))

Performs late initialization when attaching a `Layer` to a [`Subscriber`](../tracing/trait.Subscriber.html "trait bevy::log::tracing::Subscriber"). [Read more](trait.Layer.html#method.on_layer)

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/layer/mod.rs.html#1773)

#### fn [on\_new\_span](trait.Layer.html#method.on_new_span)(&self, attrs: &[Attributes](../tracing/span/struct.Attributes.html "struct bevy::log::tracing::span::Attributes")<'\_>, id: &[Id](../tracing/struct.Id.html "struct bevy::log::tracing::Id"), ctx: [Context](layer/struct.Context.html "struct bevy::log::tracing_subscriber::layer::Context")<'\_, S>)

Notifies this layer that a new span was constructed with the given `Attributes` and `Id`.

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/layer/mod.rs.html#1773)

#### fn [register\_callsite](trait.Layer.html#method.register_callsite)(&self, metadata: &'static [Metadata](../tracing/struct.Metadata.html "struct bevy::log::tracing::Metadata")<'static>) -> [Interest](../tracing/subscriber/struct.Interest.html "struct bevy::log::tracing::subscriber::Interest")

Registers a new callsite with this layer, returning whether or not the layer is interested in being notified about the callsite, similarly to [`Subscriber::register_callsite`](../tracing/trait.Subscriber.html#method.register_callsite "method bevy::log::tracing::Subscriber::register_callsite"). [Read more](trait.Layer.html#method.register_callsite)

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/layer/mod.rs.html#1773)

#### fn [enabled](trait.Layer.html#method.enabled)(&self, metadata: &[Metadata](../tracing/struct.Metadata.html "struct bevy::log::tracing::Metadata")<'\_>, ctx: [Context](layer/struct.Context.html "struct bevy::log::tracing_subscriber::layer::Context")<'\_, S>) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Returns `true` if this layer is interested in a span or event with the given `metadata` in the current [`Context`](layer/struct.Context.html "struct bevy::log::tracing_subscriber::layer::Context"), similarly to [`Subscriber::enabled`](../tracing/trait.Subscriber.html#tymethod.enabled "method bevy::log::tracing::Subscriber::enabled"). [Read more](trait.Layer.html#method.enabled)

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/layer/mod.rs.html#1773)

#### fn [on\_record](trait.Layer.html#method.on_record)(&self, span: &[Id](../tracing/struct.Id.html "struct bevy::log::tracing::Id"), values: &[Record](../tracing/span/struct.Record.html "struct bevy::log::tracing::span::Record")<'\_>, ctx: [Context](layer/struct.Context.html "struct bevy::log::tracing_subscriber::layer::Context")<'\_, S>)

Notifies this layer that a span with the given `Id` recorded the given `values`.

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/layer/mod.rs.html#1773)

#### fn [on\_follows\_from](trait.Layer.html#method.on_follows_from)(&self, span: &[Id](../tracing/struct.Id.html "struct bevy::log::tracing::Id"), follows: &[Id](../tracing/struct.Id.html "struct bevy::log::tracing::Id"), ctx: [Context](layer/struct.Context.html "struct bevy::log::tracing_subscriber::layer::Context")<'\_, S>)

Notifies this layer that a span with the ID `span` recorded that it follows from the span with the ID `follows`.

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/layer/mod.rs.html#1773)

#### fn [event\_enabled](trait.Layer.html#method.event_enabled)(&self, event: &[Event](../tracing/struct.Event.html "struct bevy::log::tracing::Event")<'\_>, ctx: [Context](layer/struct.Context.html "struct bevy::log::tracing_subscriber::layer::Context")<'\_, S>) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Called before [`on_event`](trait.Layer.html#method.on_event "method bevy::log::tracing_subscriber::Layer::on_event"), to determine if `on_event` should be called. [Read more](trait.Layer.html#method.event_enabled)

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/layer/mod.rs.html#1773)

#### fn [on\_event](trait.Layer.html#method.on_event)(&self, event: &[Event](../tracing/struct.Event.html "struct bevy::log::tracing::Event")<'\_>, ctx: [Context](layer/struct.Context.html "struct bevy::log::tracing_subscriber::layer::Context")<'\_, S>)

Notifies this layer that an event has occurred.

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/layer/mod.rs.html#1773)

#### fn [on\_enter](trait.Layer.html#method.on_enter)(&self, id: &[Id](../tracing/struct.Id.html "struct bevy::log::tracing::Id"), ctx: [Context](layer/struct.Context.html "struct bevy::log::tracing_subscriber::layer::Context")<'\_, S>)

Notifies this layer that a span with the given ID was entered.

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/layer/mod.rs.html#1773)

#### fn [on\_exit](trait.Layer.html#method.on_exit)(&self, id: &[Id](../tracing/struct.Id.html "struct bevy::log::tracing::Id"), ctx: [Context](layer/struct.Context.html "struct bevy::log::tracing_subscriber::layer::Context")<'\_, S>)

Notifies this layer that the span with the given ID was exited.

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/layer/mod.rs.html#1773)

#### fn [on\_close](trait.Layer.html#method.on_close)(&self, id: [Id](../tracing/struct.Id.html "struct bevy::log::tracing::Id"), ctx: [Context](layer/struct.Context.html "struct bevy::log::tracing_subscriber::layer::Context")<'\_, S>)

Notifies this layer that the span with the given ID has been closed.

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/layer/mod.rs.html#1773)

#### fn [on\_id\_change](trait.Layer.html#method.on_id_change)(&self, old: &[Id](../tracing/struct.Id.html "struct bevy::log::tracing::Id"), new: &[Id](../tracing/struct.Id.html "struct bevy::log::tracing::Id"), ctx: [Context](layer/struct.Context.html "struct bevy::log::tracing_subscriber::layer::Context")<'\_, S>)

Notifies this layer that a span ID has been cloned, and that the subscriber returned a different ID.

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/layer/mod.rs.html#1040-1043)

#### fn [and\_then](trait.Layer.html#method.and_then)<L>(self, layer: L) -> [Layered](layer/struct.Layered.html "struct bevy::log::tracing_subscriber::layer::Layered")<L, Self, S>

where L: [Layer](trait.Layer.html "trait bevy::log::tracing_subscriber::Layer")<S>, Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Composes this layer around the given `Layer`, returning a `Layered` struct implementing `Layer`. [Read more](trait.Layer.html#method.and_then)

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/layer/mod.rs.html#1092-1094)

#### fn [with\_subscriber](trait.Layer.html#method.with_subscriber)(self, inner: S) -> [Layered](layer/struct.Layered.html "struct bevy::log::tracing_subscriber::layer::Layered")<Self, S>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Composes this `Layer` with the given [`Subscriber`](../tracing/trait.Subscriber.html "trait bevy::log::tracing::Subscriber"), returning a `Layered` struct that implements [`Subscriber`](../tracing/trait.Subscriber.html "trait bevy::log::tracing::Subscriber"). [Read more](trait.Layer.html#method.with_subscriber)

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/layer/mod.rs.html#1111-1114)

#### fn [with\_filter](trait.Layer.html#method.with_filter)<F>(self, filter: F) -> [Filtered](filter/struct.Filtered.html "struct bevy::log::tracing_subscriber::filter::Filtered")<Self, F, S>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), F: [Filter](layer/trait.Filter.html "trait bevy::log::tracing_subscriber::layer::Filter")<S>,

Available on **crate features `registry` and `std`** only.

Combines `self` with a [`Filter`](layer/trait.Filter.html "trait bevy::log::tracing_subscriber::layer::Filter"), returning a [`Filtered`](filter/struct.Filtered.html "struct bevy::log::tracing_subscriber::filter::Filtered") layer. [Read more](trait.Layer.html#method.with_filter)

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/layer/mod.rs.html#1235-1239)

#### fn [boxed](trait.Layer.html#method.boxed)(self) -> [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Layer](trait.Layer.html "trait bevy::log::tracing_subscriber::Layer")<S> + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync")\>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [Layer](trait.Layer.html "trait bevy::log::tracing_subscriber::Layer")<S> + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static, S: [Subscriber](../tracing/trait.Subscriber.html "trait bevy::log::tracing::Subscriber"),

Erases the type of this [`Layer`](trait.Layer.html "trait bevy::log::tracing_subscriber::Layer"), returning a [`Box`](../../prelude/struct.Box.html "struct bevy::prelude::Box")ed `dyn Layer` trait object. [Read more](trait.Layer.html#method.boxed)

## Dyn Compatibility

This trait **is** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementations on Foreign Types

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/layer/mod.rs.html#1560-1563)

### impl<L, S> [Layer](trait.Layer.html "trait bevy::log::tracing_subscriber::Layer")<S> for [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<L>

where L: [Layer](trait.Layer.html "trait bevy::log::tracing_subscriber::Layer")<S>, S: [Subscriber](../tracing/trait.Subscriber.html "trait bevy::log::tracing::Subscriber"),

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/layer/mod.rs.html#1565)

#### fn [on\_layer](#method.on_layer)(&mut self, subscriber: [&mut S](https://doc.rust-lang.org/nightly/std/primitive.reference.html))

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/layer/mod.rs.html#1572)

#### fn [on\_new\_span](#method.on_new_span)(&self, attrs: &[Attributes](../tracing/span/struct.Attributes.html "struct bevy::log::tracing::span::Attributes")<'\_>, id: &[Id](../tracing/struct.Id.html "struct bevy::log::tracing::Id"), ctx: [Context](layer/struct.Context.html "struct bevy::log::tracing_subscriber::layer::Context")<'\_, S>)

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/layer/mod.rs.html#1579)

#### fn [register\_callsite](#method.register_callsite)(&self, metadata: &'static [Metadata](../tracing/struct.Metadata.html "struct bevy::log::tracing::Metadata")<'static>) -> [Interest](../tracing/subscriber/struct.Interest.html "struct bevy::log::tracing::subscriber::Interest")

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/layer/mod.rs.html#1587)

#### fn [enabled](#method.enabled)(&self, metadata: &[Metadata](../tracing/struct.Metadata.html "struct bevy::log::tracing::Metadata")<'\_>, ctx: [Context](layer/struct.Context.html "struct bevy::log::tracing_subscriber::layer::Context")<'\_, S>) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/layer/mod.rs.html#1607)

#### fn [on\_record](#method.on_record)(&self, span: &[Id](../tracing/struct.Id.html "struct bevy::log::tracing::Id"), values: &[Record](../tracing/span/struct.Record.html "struct bevy::log::tracing::span::Record")<'\_>, ctx: [Context](layer/struct.Context.html "struct bevy::log::tracing_subscriber::layer::Context")<'\_, S>)

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/layer/mod.rs.html#1614)

#### fn [on\_follows\_from](#method.on_follows_from)(&self, span: &[Id](../tracing/struct.Id.html "struct bevy::log::tracing::Id"), follows: &[Id](../tracing/struct.Id.html "struct bevy::log::tracing::Id"), ctx: [Context](layer/struct.Context.html "struct bevy::log::tracing_subscriber::layer::Context")<'\_, S>)

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/layer/mod.rs.html#1621)

#### fn [event\_enabled](#method.event_enabled)(&self, event: &[Event](../tracing/struct.Event.html "struct bevy::log::tracing::Event")<'\_>, ctx: [Context](layer/struct.Context.html "struct bevy::log::tracing_subscriber::layer::Context")<'\_, S>) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/layer/mod.rs.html#1629)

#### fn [on\_event](#method.on_event)(&self, event: &[Event](../tracing/struct.Event.html "struct bevy::log::tracing::Event")<'\_>, ctx: [Context](layer/struct.Context.html "struct bevy::log::tracing_subscriber::layer::Context")<'\_, S>)

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/layer/mod.rs.html#1636)

#### fn [on\_enter](#method.on_enter)(&self, id: &[Id](../tracing/struct.Id.html "struct bevy::log::tracing::Id"), ctx: [Context](layer/struct.Context.html "struct bevy::log::tracing_subscriber::layer::Context")<'\_, S>)

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/layer/mod.rs.html#1643)

#### fn [on\_exit](#method.on_exit)(&self, id: &[Id](../tracing/struct.Id.html "struct bevy::log::tracing::Id"), ctx: [Context](layer/struct.Context.html "struct bevy::log::tracing_subscriber::layer::Context")<'\_, S>)

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/layer/mod.rs.html#1650)

#### fn [on\_close](#method.on_close)(&self, id: [Id](../tracing/struct.Id.html "struct bevy::log::tracing::Id"), ctx: [Context](layer/struct.Context.html "struct bevy::log::tracing_subscriber::layer::Context")<'\_, S>)

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/layer/mod.rs.html#1657)

#### fn [on\_id\_change](#method.on_id_change)(&self, old: &[Id](../tracing/struct.Id.html "struct bevy::log::tracing::Id"), new: &[Id](../tracing/struct.Id.html "struct bevy::log::tracing::Id"), ctx: [Context](layer/struct.Context.html "struct bevy::log::tracing_subscriber::layer::Context")<'\_, S>)

[Source](https://docs.rs/tracing-tracy/0.11.4/x86_64-unknown-linux-gnu/src/tracing_tracy/lib.rs.html#166-169)

### impl<S, C> [Layer](trait.Layer.html "trait bevy::log::tracing_subscriber::Layer")<S> for [TracyLayer](https://docs.rs/tracing-tracy/0.11.4/x86_64-unknown-linux-gnu/tracing_tracy/struct.TracyLayer.html "struct tracing_tracy::TracyLayer")<C>

where S: [Subscriber](../tracing/trait.Subscriber.html "trait bevy::log::tracing::Subscriber") + for<'a> [LookupSpan](registry/trait.LookupSpan.html "trait bevy::log::tracing_subscriber::registry::LookupSpan")<'a>, C: [Config](https://docs.rs/tracing-tracy/0.11.4/x86_64-unknown-linux-gnu/tracing_tracy/config/trait.Config.html "trait tracing_tracy::config::Config") + 'static,

[Source](https://docs.rs/tracing-tracy/0.11.4/x86_64-unknown-linux-gnu/src/tracing_tracy/lib.rs.html#171)

#### fn [on\_new\_span](#method.on_new_span)(&self, attrs: &[Attributes](../tracing/span/struct.Attributes.html "struct bevy::log::tracing::span::Attributes")<'\_>, id: &[Id](../tracing/struct.Id.html "struct bevy::log::tracing::Id"), ctx: [Context](layer/struct.Context.html "struct bevy::log::tracing_subscriber::layer::Context")<'\_, S>)

[Source](https://docs.rs/tracing-tracy/0.11.4/x86_64-unknown-linux-gnu/src/tracing_tracy/lib.rs.html#189)

#### fn [on\_record](#method.on_record)(&self, id: &[Id](../tracing/struct.Id.html "struct bevy::log::tracing::Id"), values: &[Record](../tracing/span/struct.Record.html "struct bevy::log::tracing::span::Record")<'\_>, ctx: [Context](layer/struct.Context.html "struct bevy::log::tracing_subscriber::layer::Context")<'\_, S>)

[Source](https://docs.rs/tracing-tracy/0.11.4/x86_64-unknown-linux-gnu/src/tracing_tracy/lib.rs.html#209)

#### fn [on\_event](#method.on_event)(&self, event: &[Event](../tracing/struct.Event.html "struct bevy::log::tracing::Event")<'\_>, \_: [Context](layer/struct.Context.html "struct bevy::log::tracing_subscriber::layer::Context")<'\_, S>)

[Source](https://docs.rs/tracing-tracy/0.11.4/x86_64-unknown-linux-gnu/src/tracing_tracy/lib.rs.html#235)

#### fn [on\_enter](#method.on_enter)(&self, id: &[Id](../tracing/struct.Id.html "struct bevy::log::tracing::Id"), ctx: [Context](layer/struct.Context.html "struct bevy::log::tracing_subscriber::layer::Context")<'\_, S>)

[Source](https://docs.rs/tracing-tracy/0.11.4/x86_64-unknown-linux-gnu/src/tracing_tracy/lib.rs.html#287)

#### fn [on\_exit](#method.on_exit)(&self, id: &[Id](../tracing/struct.Id.html "struct bevy::log::tracing::Id"), \_: [Context](layer/struct.Context.html "struct bevy::log::tracing_subscriber::layer::Context")<'\_, S>)

[Source](https://docs.rs/tracing-tracy/0.11.4/x86_64-unknown-linux-gnu/src/tracing_tracy/lib.rs.html#307)

#### fn [on\_close](#method.on_close)(&self, id: [Id](../tracing/struct.Id.html "struct bevy::log::tracing::Id"), ctx: [Context](layer/struct.Context.html "struct bevy::log::tracing_subscriber::layer::Context")<'\_, S>)

[Source](https://docs.rs/tracing-error/0.2.1/x86_64-unknown-linux-gnu/src/tracing_error/layer.rs.html#36-39)

### impl<S, F> [Layer](trait.Layer.html "trait bevy::log::tracing_subscriber::Layer")<S> for [ErrorLayer](https://docs.rs/tracing-error/0.2.1/x86_64-unknown-linux-gnu/tracing_error/layer/struct.ErrorLayer.html "struct tracing_error::layer::ErrorLayer")<S, F>

where S: [Subscriber](../tracing/trait.Subscriber.html "trait bevy::log::tracing::Subscriber") + for<'span> [LookupSpan](registry/trait.LookupSpan.html "trait bevy::log::tracing_subscriber::registry::LookupSpan")<'span>, F: for<'writer> [FormatFields](fmt/trait.FormatFields.html "trait bevy::log::tracing_subscriber::fmt::FormatFields")<'writer> + 'static,

[Source](https://docs.rs/tracing-error/0.2.1/x86_64-unknown-linux-gnu/src/tracing_error/layer.rs.html#43)

#### fn [on\_new\_span](#method.on_new_span)(&self, attrs: &[Attributes](../tracing/span/struct.Attributes.html "struct bevy::log::tracing::span::Attributes")<'\_>, id: &[Id](../tracing/struct.Id.html "struct bevy::log::tracing::Id"), ctx: [Context](layer/struct.Context.html "struct bevy::log::tracing_subscriber::layer::Context")<'\_, S>)

Notifies this layer that a new span was constructed with the given `Attributes` and `Id`.

[Source](https://docs.rs/tracing-chrome/0.7.2/x86_64-unknown-linux-gnu/src/tracing_chrome/lib.rs.html#531-533)

### impl<S> [Layer](trait.Layer.html "trait bevy::log::tracing_subscriber::Layer")<S> for [ChromeLayer](https://docs.rs/tracing-chrome/0.7.2/x86_64-unknown-linux-gnu/tracing_chrome/struct.ChromeLayer.html "struct tracing_chrome::ChromeLayer")<S>

where S: [Subscriber](../tracing/trait.Subscriber.html "trait bevy::log::tracing::Subscriber") + for<'span> [LookupSpan](registry/trait.LookupSpan.html "trait bevy::log::tracing_subscriber::registry::LookupSpan")<'span> + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/tracing-chrome/0.7.2/x86_64-unknown-linux-gnu/src/tracing_chrome/lib.rs.html#535)

#### fn [on\_enter](#method.on_enter)(&self, id: &[Id](../tracing/struct.Id.html "struct bevy::log::tracing::Id"), ctx: [Context](layer/struct.Context.html "struct bevy::log::tracing_subscriber::layer::Context")<'\_, S>)

[Source](https://docs.rs/tracing-chrome/0.7.2/x86_64-unknown-linux-gnu/src/tracing_chrome/lib.rs.html#544)

#### fn [on\_record](#method.on_record)(&self, id: &[Id](../tracing/struct.Id.html "struct bevy::log::tracing::Id"), values: &[Record](../tracing/span/struct.Record.html "struct bevy::log::tracing::span::Record")<'\_>, ctx: [Context](layer/struct.Context.html "struct bevy::log::tracing_subscriber::layer::Context")<'\_, S>)

[Source](https://docs.rs/tracing-chrome/0.7.2/x86_64-unknown-linux-gnu/src/tracing_chrome/lib.rs.html#558)

#### fn [on\_event](#method.on_event)(&self, event: &[Event](../tracing/struct.Event.html "struct bevy::log::tracing::Event")<'\_>, \_ctx: [Context](layer/struct.Context.html "struct bevy::log::tracing_subscriber::layer::Context")<'\_, S>)

[Source](https://docs.rs/tracing-chrome/0.7.2/x86_64-unknown-linux-gnu/src/tracing_chrome/lib.rs.html#564)

#### fn [on\_exit](#method.on_exit)(&self, id: &[Id](../tracing/struct.Id.html "struct bevy::log::tracing::Id"), ctx: [Context](layer/struct.Context.html "struct bevy::log::tracing_subscriber::layer::Context")<'\_, S>)

[Source](https://docs.rs/tracing-chrome/0.7.2/x86_64-unknown-linux-gnu/src/tracing_chrome/lib.rs.html#572)

#### fn [on\_new\_span](#method.on_new_span)(&self, attrs: &[Attributes](../tracing/span/struct.Attributes.html "struct bevy::log::tracing::span::Attributes")<'\_>, id: &[Id](../tracing/struct.Id.html "struct bevy::log::tracing::Id"), ctx: [Context](layer/struct.Context.html "struct bevy::log::tracing_subscriber::layer::Context")<'\_, S>)

[Source](https://docs.rs/tracing-chrome/0.7.2/x86_64-unknown-linux-gnu/src/tracing_chrome/lib.rs.html#588)

#### fn [on\_close](#method.on_close)(&self, id: [Id](../tracing/struct.Id.html "struct bevy::log::tracing::Id"), ctx: [Context](layer/struct.Context.html "struct bevy::log::tracing_subscriber::layer::Context")<'\_, S>)

## Implementors

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/layer/mod.rs.html#1761-1764)

### impl<L, S> [Layer](trait.Layer.html "trait bevy::log::tracing_subscriber::Layer")<S> for [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<L>

where L: [Layer](trait.Layer.html "trait bevy::log::tracing_subscriber::Layer")<S>, S: [Subscriber](../tracing/trait.Subscriber.html "trait bevy::log::tracing::Subscriber"),

Available on **crate features `alloc` or `std`** only.

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/reload.rs.html#114-117)

### impl<L, S> [Layer](trait.Layer.html "trait bevy::log::tracing_subscriber::Layer")<S> for bevy::log::tracing\_subscriber::reload::[Layer](reload/struct.Layer.html "struct bevy::log::tracing_subscriber::reload::Layer")<L, S>

where L: [Layer](trait.Layer.html "trait bevy::log::tracing_subscriber::Layer")<S> + 'static, S: [Subscriber](../tracing/trait.Subscriber.html "trait bevy::log::tracing::Subscriber"),

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/layer/layered.rs.html#244-248)

### impl<S, A, B> [Layer](trait.Layer.html "trait bevy::log::tracing_subscriber::Layer")<S> for [Layered](layer/struct.Layered.html "struct bevy::log::tracing_subscriber::layer::Layered")<A, B, S>

where A: [Layer](trait.Layer.html "trait bevy::log::tracing_subscriber::Layer")<S>, B: [Layer](trait.Layer.html "trait bevy::log::tracing_subscriber::Layer")<S>, S: [Subscriber](../tracing/trait.Subscriber.html "trait bevy::log::tracing::Subscriber"),

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/filter/filter_fn.rs.html#643-647)

### impl<S, F, R> [Layer](trait.Layer.html "trait bevy::log::tracing_subscriber::Layer")<S> for [DynFilterFn](filter/struct.DynFilterFn.html "struct bevy::log::tracing_subscriber::filter::DynFilterFn")<S, F, R>

where F: [Fn](https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html "trait core::ops::function::Fn")(&[Metadata](../tracing/struct.Metadata.html "struct bevy::log::tracing::Metadata")<'\_>, &[Context](layer/struct.Context.html "struct bevy::log::tracing_subscriber::layer::Context")<'\_, S>) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html) + 'static, R: [Fn](https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html "trait core::ops::function::Fn")(&'static [Metadata](../tracing/struct.Metadata.html "struct bevy::log::tracing::Metadata")<'static>) -> [Interest](../tracing/subscriber/struct.Interest.html "struct bevy::log::tracing::subscriber::Interest") + 'static, S: [Subscriber](../tracing/trait.Subscriber.html "trait bevy::log::tracing::Subscriber"),

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/filter/filter_fn.rs.html#324-327)

### impl<S, F> [Layer](trait.Layer.html "trait bevy::log::tracing_subscriber::Layer")<S> for [FilterFn](filter/struct.FilterFn.html "struct bevy::log::tracing_subscriber::filter::FilterFn")<F>

where F: [Fn](https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html "trait core::ops::function::Fn")(&[Metadata](../tracing/struct.Metadata.html "struct bevy::log::tracing::Metadata")<'\_>) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html) + 'static, S: [Subscriber](../tracing/trait.Subscriber.html "trait bevy::log::tracing::Subscriber"),

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/filter/layer_filters/mod.rs.html#718-722)

### impl<S, L, F> [Layer](trait.Layer.html "trait bevy::log::tracing_subscriber::Layer")<S> for [Filtered](filter/struct.Filtered.html "struct bevy::log::tracing_subscriber::filter::Filtered")<L, F, S>

where S: [Subscriber](../tracing/trait.Subscriber.html "trait bevy::log::tracing::Subscriber") + for<'span> [LookupSpan](registry/trait.LookupSpan.html "trait bevy::log::tracing_subscriber::registry::LookupSpan")<'span> + 'static, F: [Filter](layer/trait.Filter.html "trait bevy::log::tracing_subscriber::layer::Filter")<S> + 'static, L: [Layer](trait.Layer.html "trait bevy::log::tracing_subscriber::Layer")<S>,

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/layer/mod.rs.html#1776-1779)

### impl<S, L> [Layer](trait.Layer.html "trait bevy::log::tracing_subscriber::Layer")<S> for [Vec](../../prelude/struct.Vec.html "struct bevy::prelude::Vec")<L>

where L: [Layer](trait.Layer.html "trait bevy::log::tracing_subscriber::Layer")<S>, S: [Subscriber](../tracing/trait.Subscriber.html "trait bevy::log::tracing::Subscriber"),

Available on **crate features `alloc` or `std`** only.

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/fmt/fmt_layer.rs.html#867-872)

### impl<S, N, E, W> [Layer](trait.Layer.html "trait bevy::log::tracing_subscriber::Layer")<S> for bevy::log::tracing\_subscriber::fmt::[Layer](fmt/struct.Layer.html "struct bevy::log::tracing_subscriber::fmt::Layer")<S, N, E, W>

where S: [Subscriber](../tracing/trait.Subscriber.html "trait bevy::log::tracing::Subscriber") + for<'a> [LookupSpan](registry/trait.LookupSpan.html "trait bevy::log::tracing_subscriber::registry::LookupSpan")<'a>, N: for<'writer> [FormatFields](fmt/trait.FormatFields.html "trait bevy::log::tracing_subscriber::fmt::FormatFields")<'writer> + 'static, E: [FormatEvent](fmt/trait.FormatEvent.html "trait bevy::log::tracing_subscriber::fmt::FormatEvent")<S, N> + 'static, W: for<'writer> [MakeWriter](fmt/trait.MakeWriter.html "trait bevy::log::tracing_subscriber::fmt::MakeWriter")<'writer> + 'static,

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/layer/mod.rs.html#1769-1771)

### impl<S> [Layer](trait.Layer.html "trait bevy::log::tracing_subscriber::Layer")<S> for [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Layer](trait.Layer.html "trait bevy::log::tracing_subscriber::Layer")<S> + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync")\>

where S: [Subscriber](../tracing/trait.Subscriber.html "trait bevy::log::tracing::Subscriber"),

Available on **crate features `alloc` or `std`** only.

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/filter/env/mod.rs.html#662)

### impl<S> [Layer](trait.Layer.html "trait bevy::log::tracing_subscriber::Layer")<S> for [EnvFilter](struct.EnvFilter.html "struct bevy::log::tracing_subscriber::EnvFilter")

where S: [Subscriber](../tracing/trait.Subscriber.html "trait bevy::log::tracing::Subscriber"),

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/layer/mod.rs.html#1903)

### impl<S> [Layer](trait.Layer.html "trait bevy::log::tracing_subscriber::Layer")<S> for [Identity](layer/struct.Identity.html "struct bevy::log::tracing_subscriber::layer::Identity")

where S: [Subscriber](../tracing/trait.Subscriber.html "trait bevy::log::tracing::Subscriber"),

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/filter/level.rs.html#11)

### impl<S> [Layer](trait.Layer.html "trait bevy::log::tracing_subscriber::Layer")<S> for [LevelFilter](../tracing/level_filters/struct.LevelFilter.html "struct bevy::log::tracing::level_filters::LevelFilter")

where S: [Subscriber](../tracing/trait.Subscriber.html "trait bevy::log::tracing::Subscriber"),

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/filter/targets.rs.html#438-440)

### impl<S> [Layer](trait.Layer.html "trait bevy::log::tracing_subscriber::Layer")<S> for [Targets](filter/struct.Targets.html "struct bevy::log::tracing_subscriber::filter::Targets")

where S: [Subscriber](../tracing/trait.Subscriber.html "trait bevy::log::tracing::Subscriber"),