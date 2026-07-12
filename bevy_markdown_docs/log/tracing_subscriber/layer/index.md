[bevy](../../../index.html)::[log](../../index.html)::[tracing\_subscriber](../index.html)

# Module layer 

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/lib.rs.html#221)

The [`Layer`](../trait.Layer.html "trait bevy::log::tracing_subscriber::Layer") trait, a composable abstraction for building [`Subscriber`](../../tracing/trait.Subscriber.html "trait bevy::log::tracing::Subscriber")s.

The [`Subscriber`](../../tracing/trait.Subscriber.html "trait bevy::log::tracing::Subscriber") trait in `tracing-core` represents the _complete_ set of functionality required to consume `tracing` instrumentation. This means that a single `Subscriber` instance is a self-contained implementation of a complete strategy for collecting traces; but it _also_ means that the `Subscriber` trait cannot easily be composed with other `Subscriber`s.

In particular, [`Subscriber`](../../tracing/trait.Subscriber.html "trait bevy::log::tracing::Subscriber")s are responsible for generating [span IDs](../../tracing/struct.Id.html "struct bevy::log::tracing::Id") and assigning them to spans. Since these IDs must uniquely identify a span within the context of the current trace, this means that there may only be a single `Subscriber` for a given thread at any point in time — otherwise, there would be no authoritative source of span IDs.

On the other hand, the majority of the [`Subscriber`](../../tracing/trait.Subscriber.html "trait bevy::log::tracing::Subscriber") trait’s functionality is composable: any number of subscribers may _observe_ events, span entry and exit, and so on, provided that there is a single authoritative source of span IDs. The [`Layer`](../trait.Layer.html "trait bevy::log::tracing_subscriber::Layer") trait represents this composable subset of the [`Subscriber`](../../tracing/trait.Subscriber.html "trait bevy::log::tracing::Subscriber") behavior; it can _observe_ events and spans, but does not assign IDs.

## Composing Layers

Since a [`Layer`](../trait.Layer.html "trait bevy::log::tracing_subscriber::Layer") does not implement a complete strategy for collecting traces, it must be composed with a `Subscriber` in order to be used. The [`Layer`](../trait.Layer.html "trait bevy::log::tracing_subscriber::Layer") trait is generic over a type parameter (called `S` in the trait definition), representing the types of `Subscriber` they can be composed with. Thus, a [`Layer`](../trait.Layer.html "trait bevy::log::tracing_subscriber::Layer") may be implemented that will only compose with a particular `Subscriber` implementation, or additional trait bounds may be added to constrain what types implementing `Subscriber` a `Layer` can wrap.

`Layer`s may be added to a `Subscriber` by using the [`SubscriberExt::with`](../prelude/trait.__tracing_subscriber_SubscriberExt.html#method.with "method bevy::log::tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt::with") method, which is provided by `tracing-subscriber`’s [prelude](../prelude/index.html "mod bevy::log::tracing_subscriber::prelude"). This method returns a [`Layered`](struct.Layered.html "struct bevy::log::tracing_subscriber::layer::Layered") struct that implements `Subscriber` by composing the `Layer` with the `Subscriber`.

For example:

```rust
use tracing_subscriber::Layer;
use tracing_subscriber::prelude::*;
use tracing::Subscriber;

pub struct MyLayer {
    // ...
}

impl<S: Subscriber> Layer<S> for MyLayer {
    // ...
}

pub struct MySubscriber {
    // ...
}

impl Subscriber for MySubscriber {
    // ...
}

let subscriber = MySubscriber::new()
    .with(MyLayer::new());

tracing::subscriber::set_global_default(subscriber);
```

Multiple `Layer`s may be composed in the same manner:

```rust
pub struct MyOtherLayer {
    // ...
}

impl<S: Subscriber> Layer<S> for MyOtherLayer {
    // ...
}

pub struct MyThirdLayer {
    // ...
}

impl<S: Subscriber> Layer<S> for MyThirdLayer {
    // ...
}

let subscriber = MySubscriber::new()
    .with(MyLayer::new())
    .with(MyOtherLayer::new())
    .with(MyThirdLayer::new());

tracing::subscriber::set_global_default(subscriber);
```

The [`Layer::with_subscriber`](../trait.Layer.html#method.with_subscriber "method bevy::log::tracing_subscriber::Layer::with_subscriber") constructs the [`Layered`](struct.Layered.html "struct bevy::log::tracing_subscriber::layer::Layered") type from a [`Layer`](../trait.Layer.html "trait bevy::log::tracing_subscriber::Layer") and [`Subscriber`](../../tracing/trait.Subscriber.html "trait bevy::log::tracing::Subscriber"), and is called by [`SubscriberExt::with`](../prelude/trait.__tracing_subscriber_SubscriberExt.html#method.with "method bevy::log::tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt::with"). In general, it is more idiomatic to use [`SubscriberExt::with`](../prelude/trait.__tracing_subscriber_SubscriberExt.html#method.with "method bevy::log::tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt::with"), and treat [`Layer::with_subscriber`](../trait.Layer.html#method.with_subscriber "method bevy::log::tracing_subscriber::Layer::with_subscriber") as an implementation detail, as `with_subscriber` calls must be nested, leading to less clear code for the reader.

### Runtime Configuration With `Layer`s

In some cases, a particular [`Layer`](../trait.Layer.html "trait bevy::log::tracing_subscriber::Layer") may be enabled or disabled based on runtime configuration. This can introduce challenges, because the type of a layered [`Subscriber`](../../tracing/trait.Subscriber.html "trait bevy::log::tracing::Subscriber") depends on which layers are added to it: if an `if` or `match` expression adds some [`Layer`](../trait.Layer.html "trait bevy::log::tracing_subscriber::Layer") implementation in one branch, and other layers in another, the [`Subscriber`](../../tracing/trait.Subscriber.html "trait bevy::log::tracing::Subscriber") values returned by those branches will have different types. For example, the following _will not_ work:

[ⓘ](# "This example deliberately fails to compile")

```rust
use std::fs::File;
use tracing_subscriber::{Registry, prelude::*};

let stdout_log = tracing_subscriber::fmt::layer().pretty();
let subscriber = Registry::default().with(stdout_log);

// The compile error will occur here because the if and else
// branches have different (and therefore incompatible) types.
let subscriber = if cfg.is_prod {
    let file = File::create(cfg.path)?;
    let layer = tracing_subscriber::fmt::layer()
        .json()
        .with_writer(Arc::new(file));
    layer.with(subscriber)
} else {
    layer
};

tracing::subscriber::set_global_default(subscriber)
    .expect("Unable to set global subscriber");
```

However, a [`Layer`](../trait.Layer.html "trait bevy::log::tracing_subscriber::Layer") wrapped in an [`Option`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option") [also implements the `Layer` trait](../trait.Layer.html#impl-Layer%3CS%3E-for-Option%3CL%3E "trait bevy::log::tracing_subscriber::Layer"). This allows individual layers to be enabled or disabled at runtime while always producing a [`Subscriber`](../../tracing/trait.Subscriber.html "trait bevy::log::tracing::Subscriber") of the same type. For example:

```rust
use std::fs::File;
use tracing_subscriber::{Registry, prelude::*};

let stdout_log = tracing_subscriber::fmt::layer().pretty();
let subscriber = Registry::default().with(stdout_log);

// if `cfg.is_prod` is true, also log JSON-formatted logs to a file.
let json_log = if cfg.is_prod {
    let file = File::create(cfg.path)?;
    let json_log = tracing_subscriber::fmt::layer()
        .json()
        .with_writer(file);
    Some(json_log)
} else {
    None
};

// If `cfg.is_prod` is false, then `json` will be `None`, and this layer
// will do nothing. However, the subscriber will still have the same type
// regardless of whether the `Option`'s value is `None` or `Some`.
let subscriber = subscriber.with(json_log);

tracing::subscriber::set_global_default(subscriber)
   .expect("Unable to set global subscriber");
```

If a [`Layer`](../trait.Layer.html "trait bevy::log::tracing_subscriber::Layer") may be one of several different types, note that [`Box<dyn Layer<S> + Send + Sync>` implements `Layer`](../trait.Layer.html#impl-Layer%3CS%3E-for-Box%3Cdyn%20Layer%3CS%3E%20+%20Send%20+%20Sync%3E "trait bevy::log::tracing_subscriber::Layer"). This may be used to erase the type of a [`Layer`](../trait.Layer.html "trait bevy::log::tracing_subscriber::Layer").

For example, a function that configures a [`Layer`](../trait.Layer.html "trait bevy::log::tracing_subscriber::Layer") to log to one of several outputs might return a `Box<dyn Layer<S> + Send + Sync + 'static>`:

```rust
use tracing_subscriber::{
    Layer,
    registry::LookupSpan,
    prelude::*,
};
use std::{path::PathBuf, fs::File, io};

/// Configures whether logs are emitted to a file, to stdout, or to stderr.
pub enum LogConfig {
    File(PathBuf),
    Stdout,
    Stderr,
}

impl LogConfig {
    pub fn layer<S>(self) -> Box<dyn Layer<S> + Send + Sync + 'static>
    where
        S: tracing_core::Subscriber,
        for<'a> S: LookupSpan<'a>,
    {
        // Shared configuration regardless of where logs are output to.
        let fmt = tracing_subscriber::fmt::layer()
            .with_target(true)
            .with_thread_names(true);

        // Configure the writer based on the desired log target:
        match self {
            LogConfig::File(path) => {
                let file = File::create(path).expect("failed to create log file");
                Box::new(fmt.with_writer(file))
            },
            LogConfig::Stdout => Box::new(fmt.with_writer(io::stdout)),
            LogConfig::Stderr => Box::new(fmt.with_writer(io::stderr)),
        }
    }
}

let config = LogConfig::Stdout;
tracing_subscriber::registry()
    .with(config.layer())
    .init();
```

The [`Layer::boxed`](../trait.Layer.html#method.boxed "method bevy::log::tracing_subscriber::Layer::boxed") method is provided to make boxing a `Layer` more convenient, but [`Box::new`](../../../prelude/struct.Box.html#method.new "associated function bevy::prelude::Box::new") may be used as well.

When the number of `Layer`s varies at runtime, note that a [`Vec<L> where L: Layer` also implements `Layer`](../trait.Layer.html#impl-Layer%3CS%3E-for-Vec%3CL%3E "trait bevy::log::tracing_subscriber::Layer"). This can be used to add a variable number of `Layer`s to a `Subscriber`:

```rust
use tracing_subscriber::{Layer, prelude::*};
struct MyLayer {
    // ...
}

impl<S: tracing_core::Subscriber> Layer<S> for MyLayer {
    // ...
}

/// Returns how many layers we need
fn how_many_layers() -> usize {
    // ...
}

// Create a variable-length `Vec` of layers
let mut layers = Vec::new();
for _ in 0..how_many_layers() {
    layers.push(MyLayer::new());
}

tracing_subscriber::registry()
    .with(layers)
    .init();
```

If a variable number of `Layer` is needed and those `Layer`s have different types, a `Vec` of [boxed `Layer` trait objects](../trait.Layer.html#impl-Layer%3CS%3E-for-Box%3Cdyn%20Layer%3CS%3E%20+%20Send%20+%20Sync%3E "trait bevy::log::tracing_subscriber::Layer") may be used. For example:

```rust
use tracing_subscriber::{filter::LevelFilter, Layer, prelude::*};
use std::fs::File;
struct Config {
    enable_log_file: bool,
    enable_stdout: bool,
    enable_stderr: bool,
    // ...
}

let cfg = Config::from_config_file()?;

// Based on our dynamically loaded config file, create any number of layers:
let mut layers = Vec::new();

if cfg.enable_log_file {
    let file = File::create("myapp.log")?;
    let layer = tracing_subscriber::fmt::layer()
        .with_thread_names(true)
        .with_target(true)
        .json()
        .with_writer(file)
        // Box the layer as a type-erased trait object, so that it can
        // be pushed to the `Vec`.
        .boxed();
    layers.push(layer);
}

if cfg.enable_stdout {
    let layer = tracing_subscriber::fmt::layer()
        .pretty()
        .with_filter(LevelFilter::INFO)
        // Box the layer as a type-erased trait object, so that it can
        // be pushed to the `Vec`.
        .boxed();
    layers.push(layer);
}

if cfg.enable_stdout {
    let layer = tracing_subscriber::fmt::layer()
        .with_target(false)
        .with_filter(LevelFilter::WARN)
        // Box the layer as a type-erased trait object, so that it can
        // be pushed to the `Vec`.
        .boxed();
    layers.push(layer);
}

tracing_subscriber::registry()
    .with(layers)
    .init();
```

Finally, if the number of layers _changes_ at runtime, a `Vec` of subscribers can be used alongside the [`reload`](../reload/index.html "mod bevy::log::tracing_subscriber::reload") module to add or remove subscribers dynamically at runtime.

## Recording Traces

The [`Layer`](../trait.Layer.html "trait bevy::log::tracing_subscriber::Layer") trait defines a set of methods for consuming notifications from tracing instrumentation, which are generally equivalent to the similarly named methods on [`Subscriber`](../../tracing/trait.Subscriber.html "trait bevy::log::tracing::Subscriber"). Unlike [`Subscriber`](../../tracing/trait.Subscriber.html "trait bevy::log::tracing::Subscriber"), the methods on `Layer` are additionally passed a [`Context`](struct.Context.html "struct bevy::log::tracing_subscriber::layer::Context") type, which exposes additional information provided by the wrapped subscriber (such as [the current span](struct.Context.html#method.current_span "method bevy::log::tracing_subscriber::layer::Context::current_span")) to the layer.

## Filtering with `Layer`s

As well as strategies for handling trace events, the `Layer` trait may also be used to represent composable _filters_. This allows the determination of what spans and events should be recorded to be decoupled from _how_ they are recorded: a filtering layer can be applied to other layers or subscribers. `Layer`s can be used to implement _global filtering_, where a `Layer` provides a filtering strategy for the entire subscriber. Additionally, individual recording `Layer`s or sets of `Layer`s may be combined with _per-layer filters_ that control what spans and events are recorded by those layers.

### Global Filtering

A `Layer` that implements a filtering strategy should override the [`register_callsite`](../trait.Layer.html#method.register_callsite "method bevy::log::tracing_subscriber::Layer::register_callsite") and/or [`enabled`](../trait.Layer.html#method.enabled "method bevy::log::tracing_subscriber::Layer::enabled") methods. It may also choose to implement methods such as [`on_enter`](../trait.Layer.html#method.on_enter "method bevy::log::tracing_subscriber::Layer::on_enter"), if it wishes to filter trace events based on the current span context.

Note that the [`Layer::register_callsite`](../trait.Layer.html#method.register_callsite "method bevy::log::tracing_subscriber::Layer::register_callsite") and [`Layer::enabled`](../trait.Layer.html#method.enabled "method bevy::log::tracing_subscriber::Layer::enabled") methods determine whether a span or event is enabled _globally_. Thus, they should **not** be used to indicate whether an individual layer wishes to record a particular span or event. Instead, if a layer is only interested in a subset of trace data, but does _not_ wish to disable other spans and events for the rest of the layer stack should ignore those spans and events in its notification methods.

The filtering methods on a stack of `Layer`s are evaluated in a top-down order, starting with the outermost `Layer` and ending with the wrapped [`Subscriber`](../../tracing/trait.Subscriber.html "trait bevy::log::tracing::Subscriber"). If any layer returns `false` from its [`enabled`](../trait.Layer.html#method.enabled "method bevy::log::tracing_subscriber::Layer::enabled") method, or [`Interest::never()`](../../tracing/subscriber/struct.Interest.html#method.never "associated function bevy::log::tracing::subscriber::Interest::never") from its [`register_callsite`](../trait.Layer.html#method.register_callsite "method bevy::log::tracing_subscriber::Layer::register_callsite") method, filter evaluation will short-circuit and the span or event will be disabled.

#### Enabling Interest

Whenever an tracing event (or span) is emitted, it goes through a number of steps to determine how and how much it should be processed. The earlier an event is disabled, the less work has to be done to process the event, so `Layer`s that implement filtering should attempt to disable unwanted events as early as possible. In order, each event checks:

*   [`register_callsite`](../trait.Layer.html#method.register_callsite "method bevy::log::tracing_subscriber::Layer::register_callsite"), once per callsite (roughly: once per time that `event!` or `span!` is written in the source code; this is cached at the callsite). See [`Subscriber::register_callsite`](../../tracing/trait.Subscriber.html#method.register_callsite "method bevy::log::tracing::Subscriber::register_callsite") and [`tracing_core::callsite`](../../tracing/callsite/index.html "mod bevy::log::tracing::callsite") for a summary of how this behaves.
*   [`enabled`](../trait.Layer.html#method.enabled "method bevy::log::tracing_subscriber::Layer::enabled"), once per emitted event (roughly: once per time that `event!` or `span!` is _executed_), and only if `register_callsite` registers an [`Interest::sometimes`](../../tracing/subscriber/struct.Interest.html#method.sometimes "associated function bevy::log::tracing::subscriber::Interest::sometimes"). This is the main customization point to globally filter events based on their [`Metadata`](../../tracing/struct.Metadata.html "struct bevy::log::tracing::Metadata"). If an event can be disabled based only on [`Metadata`](../../tracing/struct.Metadata.html "struct bevy::log::tracing::Metadata"), it should be, as this allows the construction of the actual `Event`/`Span` to be skipped.
*   For events only (and not spans), [`event_enabled`](../trait.Layer.html#method.event_enabled "method bevy::log::tracing_subscriber::Layer::event_enabled") is called just before processing the event. This gives layers one last chance to say that an event should be filtered out, now that the event’s fields are known.

### Per-Layer Filtering

**Note**: per-layer filtering APIs currently require the [`"registry"` crate feature flag](../index.html#feature-flags "mod bevy::log::tracing_subscriber") to be enabled.

Sometimes, it may be desirable for one `Layer` to record a particular subset of spans and events, while a different subset of spans and events are recorded by other `Layer`s. For example:

*   A layer that records metrics may wish to observe only events including particular tracked values, while a logging layer ignores those events.
*   If recording a distributed trace is expensive, it might be desirable to only send spans with `INFO` and lower verbosity to the distributed tracing system, while logging more verbose spans to a file.
*   Spans and events with a particular target might be recorded differently from others, such as by generating an HTTP access log from a span that tracks the lifetime of an HTTP request.

The [`Filter`](../filter/index.html "mod bevy::log::tracing_subscriber::filter") trait is used to control what spans and events are observed by an individual `Layer`, while still allowing other `Layer`s to potentially record them. The [`Layer::with_filter`](../trait.Layer.html#method.with_filter "method bevy::log::tracing_subscriber::Layer::with_filter") method combines a `Layer` with a [`Filter`](../filter/index.html "mod bevy::log::tracing_subscriber::filter"), returning a [`Filtered`](../filter/struct.Filtered.html "struct bevy::log::tracing_subscriber::filter::Filtered") layer.

This crate’s [`filter`](../filter/index.html "mod bevy::log::tracing_subscriber::filter") module provides a number of types which implement the [`Filter`](../filter/index.html "mod bevy::log::tracing_subscriber::filter") trait, such as [`LevelFilter`](../../tracing/level_filters/struct.LevelFilter.html "struct bevy::log::tracing::level_filters::LevelFilter"), [`Targets`](../filter/struct.Targets.html "struct bevy::log::tracing_subscriber::filter::Targets"), and [`FilterFn`](../filter/struct.FilterFn.html "struct bevy::log::tracing_subscriber::filter::FilterFn"). These [`Filter`](../filter/index.html "mod bevy::log::tracing_subscriber::filter")s provide ready-made implementations of common forms of filtering. For custom filtering policies, the [`FilterFn`](../filter/struct.FilterFn.html "struct bevy::log::tracing_subscriber::filter::FilterFn") and [`DynFilterFn`](../filter/struct.DynFilterFn.html "struct bevy::log::tracing_subscriber::filter::DynFilterFn") types allow implementing a [`Filter`](../filter/index.html "mod bevy::log::tracing_subscriber::filter") with a closure or function pointer. In addition, when more control is required, the [`Filter`](../filter/index.html "mod bevy::log::tracing_subscriber::filter") trait may also be implemented for user-defined types.

[`Option<Filter>`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option") also implements [`Filter`](../filter/index.html "mod bevy::log::tracing_subscriber::filter"), which allows for an optional filter. [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") filters out _nothing_ (that is, allows everything through). For example:

```rust
fn setup_tracing<S: Subscriber>(filter_config: Option<&str>) {
    let layer = MyLayer::<S>::new()
        .with_filter(filter_config.map(|config| filter_fn(my_filter(config))));
//...
}
```

     **Warning**: Currently, the [
     `Registry`](../struct.Registry.html) type defined in this crate is the only root
     `Subscriber` capable of supporting `Layer`s with
     per-layer filters. In the future, new APIs will be added to allow other
     root `Subscriber`s to support per-layer filters.
 

For example, to generate an HTTP access log based on spans with the `http_access` target, while logging other spans and events to standard out, a [`Filter`](../filter/index.html "mod bevy::log::tracing_subscriber::filter") can be added to the access log layer:

```rust
use tracing_subscriber::{filter, prelude::*};

// Generates an HTTP access log.
let access_log = // ...

// Add a filter to the access log layer so that it only observes
// spans and events with the `http_access` target.
let access_log = access_log.with_filter(filter::filter_fn(|metadata| {
    // Returns `true` if and only if the span or event's target is
    // "http_access".
    metadata.target() == "http_access"
}));

// A general-purpose logging layer.
let fmt_layer = tracing_subscriber::fmt::layer();

// Build a subscriber that combines the access log and stdout log
// layers.
tracing_subscriber::registry()
    .with(fmt_layer)
    .with(access_log)
    .init();
```

Multiple layers can have their own, separate per-layer filters. A span or event will be recorded if it is enabled by _any_ per-layer filter, but it will be skipped by the layers whose filters did not enable it. Building on the previous example:

```rust
use tracing_subscriber::{filter::{filter_fn, LevelFilter}, prelude::*};

let access_log = // ...
let fmt_layer = tracing_subscriber::fmt::layer();

tracing_subscriber::registry()
    // Add the filter for the "http_access" target to the access
    // log layer, like before.
    .with(access_log.with_filter(filter_fn(|metadata| {
        metadata.target() == "http_access"
    })))
    // Add a filter for spans and events with the INFO level
    // and below to the logging layer.
    .with(fmt_layer.with_filter(LevelFilter::INFO))
    .init();

// Neither layer will observe this event
tracing::debug!(does_anyone_care = false, "a tree fell in the forest");

// This event will be observed by the logging layer, but not
// by the access log layer.
tracing::warn!(dose_roentgen = %3.8, "not great, but not terrible");

// This event will be observed only by the access log layer.
tracing::trace!(target: "http_access", "HTTP request started");

// Both layers will observe this event.
tracing::error!(target: "http_access", "HTTP request failed with a very bad error!");
```

A per-layer filter can be applied to multiple [`Layer`](../trait.Layer.html "trait bevy::log::tracing_subscriber::Layer")s at a time, by combining them into a [`Layered`](struct.Layered.html "struct bevy::log::tracing_subscriber::layer::Layered") layer using [`Layer::and_then`](../trait.Layer.html#method.and_then "method bevy::log::tracing_subscriber::Layer::and_then"), and then calling [`Layer::with_filter`](../trait.Layer.html#method.with_filter "method bevy::log::tracing_subscriber::Layer::with_filter") on the resulting [`Layered`](struct.Layered.html "struct bevy::log::tracing_subscriber::layer::Layered") layer.

Consider the following:

*   `layer_a` and `layer_b`, which should only receive spans and events at the [`INFO`](../../struct.Level.html#associatedconstant.INFO "associated constant bevy::log::Level::INFO") [level](../../struct.Level.html "struct bevy::log::Level") and above.
*   A third layer, `layer_c`, which should receive spans and events at the [`DEBUG`](../../struct.Level.html#associatedconstant.DEBUG "associated constant bevy::log::Level::DEBUG") [level](../../struct.Level.html "struct bevy::log::Level") as well.

The layers and filters would be composed thusly:

```rust
use tracing_subscriber::{filter::LevelFilter, prelude::*};

let layer_a = // ...
let layer_b =  // ...
let layer_c =  // ...

let info_layers = layer_a
    // Combine `layer_a` and `layer_b` into a `Layered` layer:
    .and_then(layer_b)
    // ...and then add an `INFO` `LevelFilter` to that layer:
    .with_filter(LevelFilter::INFO);

tracing_subscriber::registry()
    // Add `layer_c` with a `DEBUG` filter.
    .with(layer_c.with_filter(LevelFilter::DEBUG))
    .with(info_layers)
    .init();
```

If a [`Filtered`](../filter/struct.Filtered.html "struct bevy::log::tracing_subscriber::filter::Filtered") [`Layer`](../trait.Layer.html "trait bevy::log::tracing_subscriber::Layer") is combined with another [`Layer`](../trait.Layer.html "trait bevy::log::tracing_subscriber::Layer") [`Layer::and_then`](../trait.Layer.html#method.and_then "method bevy::log::tracing_subscriber::Layer::and_then"), and a filter is added to the [`Layered`](struct.Layered.html "struct bevy::log::tracing_subscriber::layer::Layered") layer, that layer will be filtered by _both_ the inner filter and the outer filter. Only spans and events that are enabled by _both_ filters will be observed by that layer. This can be used to implement complex filtering trees.

As an example, consider the following constraints:

*   Suppose that a particular [target](../../tracing/struct.Metadata.html#method.target "method bevy::log::tracing::Metadata::target") is used to indicate events that should be counted as part of a metrics system, which should be only observed by a layer that collects metrics.
*   A log of high-priority events ([`INFO`](../../struct.Level.html#associatedconstant.INFO "associated constant bevy::log::Level::INFO") and above) should be logged to stdout, while more verbose events should be logged to a debugging log file.
*   Metrics-focused events should _not_ be included in either log output.

In that case, it is possible to apply a filter to both logging layers to exclude the metrics events, while additionally adding a [`LevelFilter`](../../tracing/level_filters/struct.LevelFilter.html "struct bevy::log::tracing::level_filters::LevelFilter") to the stdout log:

```rust
use tracing_subscriber::{filter, prelude::*};
use std::{fs::File, sync::Arc};

// A layer that logs events to stdout using the human-readable "pretty"
// format.
let stdout_log = tracing_subscriber::fmt::layer()
    .pretty();

// A layer that logs events to a file.
let file = File::create("debug.log")?;
let debug_log = tracing_subscriber::fmt::layer()
    .with_writer(Arc::new(file));

// A layer that collects metrics using specific events.
let metrics_layer = /* ... */ filter::LevelFilter::INFO;

tracing_subscriber::registry()
    .with(
        stdout_log
            // Add an `INFO` filter to the stdout logging layer
            .with_filter(filter::LevelFilter::INFO)
            // Combine the filtered `stdout_log` layer with the
            // `debug_log` layer, producing a new `Layered` layer.
            .and_then(debug_log)
            // Add a filter to *both* layers that rejects spans and
            // events whose targets start with `metrics`.
            .with_filter(filter::filter_fn(|metadata| {
                !metadata.target().starts_with("metrics")
            }))
    )
    .with(
        // Add a filter to the metrics label that *only* enables
        // events whose targets start with `metrics`.
        metrics_layer.with_filter(filter::filter_fn(|metadata| {
            metadata.target().starts_with("metrics")
        }))
    )
    .init();

// This event will *only* be recorded by the metrics layer.
tracing::info!(target: "metrics::cool_stuff_count", value = 42);

// This event will only be seen by the debug log file layer:
tracing::debug!("this is a message, and part of a system of messages");

// This event will be seen by both the stdout log layer *and*
// the debug log file layer, but not by the metrics layer.
tracing::warn!("the message is a warning about danger!");
```

## Structs

[Context](struct.Context.html "struct bevy::log::tracing_subscriber::layer::Context")

Represents information about the current context provided to [`Layer`](../trait.Layer.html "trait bevy::log::tracing_subscriber::Layer")s by the wrapped [`Subscriber`](../../tracing/trait.Subscriber.html "trait bevy::log::tracing::Subscriber").

[Identity](struct.Identity.html "struct bevy::log::tracing_subscriber::layer::Identity")

A layer that does nothing.

[Layered](struct.Layered.html "struct bevy::log::tracing_subscriber::layer::Layered")

A [`Subscriber`](../../tracing/trait.Subscriber.html "trait bevy::log::tracing::Subscriber") composed of a `Subscriber` wrapped by one or more [`Layer`](../trait.Layer.html "trait bevy::log::tracing_subscriber::Layer")s.

## Traits

[Filter](trait.Filter.html "trait bevy::log::tracing_subscriber::layer::Filter")`registry` and `std`

A per-[`Layer`](../trait.Layer.html "trait bevy::log::tracing_subscriber::Layer") filter that determines whether a span or event is enabled for an individual layer.

[Layer](trait.Layer.html "trait bevy::log::tracing_subscriber::layer::Layer")

A composable handler for `tracing` events.

[SubscriberExt](trait.SubscriberExt.html "trait bevy::log::tracing_subscriber::layer::SubscriberExt")

Extension trait adding a `with(Layer)` combinator to `Subscriber`s.