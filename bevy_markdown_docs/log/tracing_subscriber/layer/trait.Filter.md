[bevy](../../../index.html)::[log](../../index.html)::[tracing\_subscriber](../index.html)::[layer](index.html)

# Trait Filter 

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/layer/mod.rs.html#1264)

```rust
pub trait Filter<S> {
    // Required method
    fn enabled(&self, meta: &Metadata<'_>, cx: &Context<'_, S>) -> bool;

    // Provided methods
    fn callsite_enabled(&self, meta: &'static Metadata<'static>) -> Interest { ... }
    fn event_enabled(&self, event: &Event<'_>, cx: &Context<'_, S>) -> bool { ... }
    fn max_level_hint(&self) -> Option<LevelFilter> { ... }
    fn on_new_span(&self, attrs: &Attributes<'_>, id: &Id, ctx: Context<'_, S>) { ... }
    fn on_record(&self, id: &Id, values: &Record<'_>, ctx: Context<'_, S>) { ... }
    fn on_enter(&self, id: &Id, ctx: Context<'_, S>) { ... }
    fn on_exit(&self, id: &Id, ctx: Context<'_, S>) { ... }
    fn on_close(&self, id: Id, ctx: Context<'_, S>) { ... }
}
```

Available on **crate features `registry` and `std`** only.

A per-[`Layer`](../trait.Layer.html "trait bevy::log::tracing_subscriber::Layer") filter that determines whether a span or event is enabled for an individual layer.

See [the module-level documentation](index.html#per-layer-filtering "mod bevy::log::tracing_subscriber::layer") for details on using [`Filter`](trait.Filter.html "trait bevy::log::tracing_subscriber::layer::Filter")s.

## Required Methods

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/layer/mod.rs.html#1281)

#### fn [enabled](#tymethod.enabled)(&self, meta: &[Metadata](../../tracing/struct.Metadata.html "struct bevy::log::tracing::Metadata")<'\_>, cx: &[Context](struct.Context.html "struct bevy::log::tracing_subscriber::layer::Context")<'\_, S>) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Returns `true` if this layer is interested in a span or event with the given [`Metadata`](../../tracing/struct.Metadata.html "struct bevy::log::tracing::Metadata") in the current [`Context`](struct.Context.html "struct bevy::log::tracing_subscriber::layer::Context"), similarly to [`Subscriber::enabled`](../../tracing/trait.Subscriber.html#tymethod.enabled "method bevy::log::tracing::Subscriber::enabled").

If this returns `false`, the span or event will be disabled _for the wrapped [`Layer`](../trait.Layer.html "trait bevy::log::tracing_subscriber::Layer")_. Unlike [`Layer::enabled`](../trait.Layer.html#method.enabled "method bevy::log::tracing_subscriber::Layer::enabled"), the span or event will still be recorded if any _other_ layers choose to enable it. However, the layer [filtered](../filter/struct.Filtered.html "struct bevy::log::tracing_subscriber::filter::Filtered") by this filter will skip recording that span or event.

If all layers indicate that they do not wish to see this span or event, it will be disabled.

## Provided Methods

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/layer/mod.rs.html#1394)

#### fn [callsite\_enabled](#method.callsite_enabled)(&self, meta: &'static [Metadata](../../tracing/struct.Metadata.html "struct bevy::log::tracing::Metadata")<'static>) -> [Interest](../../tracing/subscriber/struct.Interest.html "struct bevy::log::tracing::subscriber::Interest")

Returns an [`Interest`](../../tracing/subscriber/struct.Interest.html "struct bevy::log::tracing::subscriber::Interest") indicating whether this layer will [always](../../tracing/subscriber/struct.Interest.html#method.always "associated function bevy::log::tracing::subscriber::Interest::always"), [sometimes](../../tracing/subscriber/struct.Interest.html#method.sometimes "associated function bevy::log::tracing::subscriber::Interest::sometimes"), or [never](../../tracing/subscriber/struct.Interest.html#method.never "associated function bevy::log::tracing::subscriber::Interest::never") be interested in the given [`Metadata`](../../tracing/struct.Metadata.html "struct bevy::log::tracing::Metadata").

When a given callsite will [always](../../tracing/subscriber/struct.Interest.html#method.always "associated function bevy::log::tracing::subscriber::Interest::always") or [never](../../tracing/subscriber/struct.Interest.html#method.never "associated function bevy::log::tracing::subscriber::Interest::never") be enabled, the results of evaluating the filter may be cached for improved performance. Therefore, if a filter is capable of determining that it will always or never enable a particular callsite, providing an implementation of this function is recommended.

**Note**: If a `Filter` will perform
_dynamic filtering_ that depends on the current context in which
a span or event was observed (e.g. only enabling an event when it
occurs within a particular span), it **must** return
`Interest::sometimes()` from this method. If it returns
`Interest::always()` or `Interest::never()`, the
`enabled` method may not be called when a particular instance
of that span or event is recorded.

This method is broadly similar to [`Subscriber::register_callsite`](../../tracing/trait.Subscriber.html#method.register_callsite "method bevy::log::tracing::Subscriber::register_callsite"); however, since the returned value represents only the interest of _this_ layer, the resulting behavior is somewhat different.

If a [`Subscriber`](../../tracing/trait.Subscriber.html "trait bevy::log::tracing::Subscriber") returns [`Interest::always()`](../../tracing/subscriber/struct.Interest.html#method.always "associated function bevy::log::tracing::subscriber::Interest::always") or [`Interest::never()`](../../tracing/subscriber/struct.Interest.html#method.never "associated function bevy::log::tracing::subscriber::Interest::never") for a given [`Metadata`](../../tracing/struct.Metadata.html "struct bevy::log::tracing::Metadata"), its [`enabled`](trait.Filter.html#tymethod.enabled "method bevy::log::tracing_subscriber::layer::Filter::enabled") method is then _guaranteed_ to never be called for that callsite. On the other hand, when a `Filter` returns [`Interest::always()`](../../tracing/subscriber/struct.Interest.html#method.always "associated function bevy::log::tracing::subscriber::Interest::always") or [`Interest::never()`](../../tracing/subscriber/struct.Interest.html#method.never "associated function bevy::log::tracing::subscriber::Interest::never") for a callsite, _other_ [`Layer`](../trait.Layer.html "trait bevy::log::tracing_subscriber::Layer")s may have differing interests in that callsite. If this is the case, the callsite will receive [`Interest::sometimes()`](../../tracing/subscriber/struct.Interest.html#method.sometimes "associated function bevy::log::tracing::subscriber::Interest::sometimes"), and the [`enabled`](trait.Filter.html#tymethod.enabled "method bevy::log::tracing_subscriber::layer::Filter::enabled") method will still be called for that callsite when it records a span or event.

Returning [`Interest::always()`](../../tracing/subscriber/struct.Interest.html#method.always "associated function bevy::log::tracing::subscriber::Interest::always") or [`Interest::never()`](../../tracing/subscriber/struct.Interest.html#method.never "associated function bevy::log::tracing::subscriber::Interest::never") from `Filter::callsite_enabled` will permanently enable or disable a callsite (without requiring subsequent calls to [`enabled`](trait.Filter.html#tymethod.enabled "method bevy::log::tracing_subscriber::layer::Filter::enabled")) if and only if the following is true:

*   all [`Layer`](../trait.Layer.html "trait bevy::log::tracing_subscriber::Layer")s that comprise the subscriber include `Filter`s (this includes a tree of [`Layered`](struct.Layered.html "struct bevy::log::tracing_subscriber::layer::Layered") layers that share the same `Filter`)
*   all those `Filter`s return the same [`Interest`](../../tracing/subscriber/struct.Interest.html "struct bevy::log::tracing::subscriber::Interest").

For example, if a [`Subscriber`](../../tracing/trait.Subscriber.html "trait bevy::log::tracing::Subscriber") consists of two [`Filtered`](../filter/struct.Filtered.html "struct bevy::log::tracing_subscriber::filter::Filtered") layers, and both of those layers return [`Interest::never()`](../../tracing/subscriber/struct.Interest.html#method.never "associated function bevy::log::tracing::subscriber::Interest::never"), that callsite _will_ never be enabled, and the [`enabled`](trait.Filter.html#tymethod.enabled "method bevy::log::tracing_subscriber::layer::Filter::enabled") methods of those [`Filter`](trait.Filter.html "trait bevy::log::tracing_subscriber::layer::Filter")s will not be called.

###### Default Implementation

The default implementation of this method assumes that the `Filter`’s [`enabled`](trait.Filter.html#tymethod.enabled "method bevy::log::tracing_subscriber::layer::Filter::enabled") method _may_ perform dynamic filtering, and returns [`Interest::sometimes()`](../../tracing/subscriber/struct.Interest.html#method.sometimes "associated function bevy::log::tracing::subscriber::Interest::sometimes"), to ensure that [`enabled`](trait.Filter.html#tymethod.enabled "method bevy::log::tracing_subscriber::layer::Filter::enabled") is called to determine whether a particular _instance_ of the callsite is enabled in the current context. If this is _not_ the case, and the `Filter`’s [`enabled`](trait.Filter.html#tymethod.enabled "method bevy::log::tracing_subscriber::layer::Filter::enabled") method will always return the same result for a particular [`Metadata`](../../tracing/struct.Metadata.html "struct bevy::log::tracing::Metadata"), this method can be overridden as follows:

```rust
use tracing_subscriber::layer;
use tracing_core::{Metadata, subscriber::Interest};

struct MyFilter {
    // ...
}

impl MyFilter {
    // The actual logic for determining whether a `Metadata` is enabled
    // must be factored out from the `enabled` method, so that it can be
    // called without a `Context` (which is not provided to the
    // `callsite_enabled` method).
    fn is_enabled(&self, metadata: &Metadata<'_>) -> bool {
        // ...
    }
}

impl<S> layer::Filter<S> for MyFilter {
    fn enabled(&self, metadata: &Metadata<'_>, _: &layer::Context<'_, S>) -> bool {
        // Even though we are implementing `callsite_enabled`, we must still provide a
        // working implementation of `enabled`, as returning `Interest::always()` or
        // `Interest::never()` will *allow* caching, but will not *guarantee* it.
        // Other filters may still return `Interest::sometimes()`, so we may be
        // asked again in `enabled`.
        self.is_enabled(metadata)
    }

    fn callsite_enabled(&self, metadata: &'static Metadata<'static>) -> Interest {
        // The result of `self.enabled(metadata, ...)` will always be
        // the same for any given `Metadata`, so we can convert it into
        // an `Interest`:
        if self.is_enabled(metadata) {
            Interest::always()
        } else {
            Interest::never()
        }
    }
}
```

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/layer/mod.rs.html#1414)

#### fn [event\_enabled](#method.event_enabled)(&self, event: &[Event](../../tracing/struct.Event.html "struct bevy::log::tracing::Event")<'\_>, cx: &[Context](struct.Context.html "struct bevy::log::tracing_subscriber::layer::Context")<'\_, S>) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Called before the filtered \[`Layer]'s [`on\_event`], to determine if` on\_event\` should be called.

This gives a chance to filter events based on their fields. Note, however, that this _does not_ override [`enabled`](trait.Filter.html#tymethod.enabled "method bevy::log::tracing_subscriber::layer::Filter::enabled"), and is not even called if [`enabled`](trait.Filter.html#tymethod.enabled "method bevy::log::tracing_subscriber::layer::Filter::enabled") returns `false`.

###### Default Implementation

By default, this method returns `true`, indicating that no events are filtered out based on their fields.

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/layer/mod.rs.html#1448)

#### fn [max\_level\_hint](#method.max_level_hint)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[LevelFilter](../../tracing/level_filters/struct.LevelFilter.html "struct bevy::log::tracing::level_filters::LevelFilter")\>

Returns an optional hint of the highest [verbosity level](../../struct.Level.html "struct bevy::log::Level") that this `Filter` will enable.

If this method returns a [`LevelFilter`](../../tracing/level_filters/struct.LevelFilter.html "struct bevy::log::tracing::level_filters::LevelFilter"), it will be used as a hint to determine the most verbose level that will be enabled. This will allow spans and events which are more verbose than that level to be skipped more efficiently. An implementation of this method is optional, but strongly encouraged.

If the maximum level the `Filter` will enable can change over the course of its lifetime, it is free to return a different value from multiple invocations of this method. However, note that changes in the maximum level will **only** be reflected after the callsite [`Interest`](../../tracing/subscriber/struct.Interest.html "struct bevy::log::tracing::subscriber::Interest") cache is rebuilt, by calling the [`tracing_core::callsite::rebuild_interest_cache`](../../tracing/callsite/fn.rebuild_interest_cache.html "fn bevy::log::tracing::callsite::rebuild_interest_cache") function. Therefore, if the `Filter will change the value returned by this method, it is responsible for ensuring that [`rebuild\_interest\_cache\`\][rebuild](../../tracing/callsite/fn.rebuild_interest_cache.html "fn bevy::log::tracing::callsite::rebuild_interest_cache") is called after the value of the max level changes.

###### Default Implementation

By default, this method returns `None`, indicating that the maximum level is unknown.

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/layer/mod.rs.html#1458)

#### fn [on\_new\_span](#method.on_new_span)(&self, attrs: &[Attributes](../../tracing/span/struct.Attributes.html "struct bevy::log::tracing::span::Attributes")<'\_>, id: &[Id](../../tracing/struct.Id.html "struct bevy::log::tracing::Id"), ctx: [Context](struct.Context.html "struct bevy::log::tracing_subscriber::layer::Context")<'\_, S>)

Notifies this filter that a new span was constructed with the given `Attributes` and `Id`.

By default, this method does nothing. `Filter` implementations that need to be notified when new spans are created can override this method.

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/layer/mod.rs.html#1469)

#### fn [on\_record](#method.on_record)(&self, id: &[Id](../../tracing/struct.Id.html "struct bevy::log::tracing::Id"), values: &[Record](../../tracing/span/struct.Record.html "struct bevy::log::tracing::span::Record")<'\_>, ctx: [Context](struct.Context.html "struct bevy::log::tracing_subscriber::layer::Context")<'\_, S>)

Notifies this filter that a span with the given `Id` recorded the given `values`.

By default, this method does nothing. `Filter` implementations that need to be notified when new spans are created can override this method.

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/layer/mod.rs.html#1477)

#### fn [on\_enter](#method.on_enter)(&self, id: &[Id](../../tracing/struct.Id.html "struct bevy::log::tracing::Id"), ctx: [Context](struct.Context.html "struct bevy::log::tracing_subscriber::layer::Context")<'\_, S>)

Notifies this filter that a span with the given ID was entered.

By default, this method does nothing. `Filter` implementations that need to be notified when a span is entered can override this method.

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/layer/mod.rs.html#1485)

#### fn [on\_exit](#method.on_exit)(&self, id: &[Id](../../tracing/struct.Id.html "struct bevy::log::tracing::Id"), ctx: [Context](struct.Context.html "struct bevy::log::tracing_subscriber::layer::Context")<'\_, S>)

Notifies this filter that a span with the given ID was exited.

By default, this method does nothing. `Filter` implementations that need to be notified when a span is exited can override this method.

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/layer/mod.rs.html#1493)

#### fn [on\_close](#method.on_close)(&self, id: [Id](../../tracing/struct.Id.html "struct bevy::log::tracing::Id"), ctx: [Context](struct.Context.html "struct bevy::log::tracing_subscriber::layer::Context")<'\_, S>)

Notifies this filter that a span with the given ID has been closed.

By default, this method does nothing. `Filter` implementations that need to be notified when a span is closed can override this method.

## Trait Implementations

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/filter/layer_filters/mod.rs.html#537)

### impl<S> [Filter](trait.Filter.html "trait bevy::log::tracing_subscriber::layer::Filter")<S> for [Box](../../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Filter](trait.Filter.html "trait bevy::log::tracing_subscriber::layer::Filter")<S> + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync")\>

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/filter/layer_filters/mod.rs.html#538)

#### fn [enabled](trait.Filter.html#tymethod.enabled)(&self, meta: &[Metadata](../../tracing/struct.Metadata.html "struct bevy::log::tracing::Metadata")<'\_>, cx: &[Context](struct.Context.html "struct bevy::log::tracing_subscriber::layer::Context")<'\_, S>) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Returns `true` if this layer is interested in a span or event with the given [`Metadata`](../../tracing/struct.Metadata.html "struct bevy::log::tracing::Metadata") in the current [`Context`](struct.Context.html "struct bevy::log::tracing_subscriber::layer::Context"), similarly to [`Subscriber::enabled`](../../tracing/trait.Subscriber.html#tymethod.enabled "method bevy::log::tracing::Subscriber::enabled"). [Read more](trait.Filter.html#tymethod.enabled)

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/filter/layer_filters/mod.rs.html#538)

#### fn [callsite\_enabled](trait.Filter.html#method.callsite_enabled)(&self, meta: &'static [Metadata](../../tracing/struct.Metadata.html "struct bevy::log::tracing::Metadata")<'static>) -> [Interest](../../tracing/subscriber/struct.Interest.html "struct bevy::log::tracing::subscriber::Interest")

Returns an [`Interest`](../../tracing/subscriber/struct.Interest.html "struct bevy::log::tracing::subscriber::Interest") indicating whether this layer will [always](../../tracing/subscriber/struct.Interest.html#method.always "associated function bevy::log::tracing::subscriber::Interest::always"), [sometimes](../../tracing/subscriber/struct.Interest.html#method.sometimes "associated function bevy::log::tracing::subscriber::Interest::sometimes"), or [never](../../tracing/subscriber/struct.Interest.html#method.never "associated function bevy::log::tracing::subscriber::Interest::never") be interested in the given [`Metadata`](../../tracing/struct.Metadata.html "struct bevy::log::tracing::Metadata"). [Read more](trait.Filter.html#method.callsite_enabled)

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/filter/layer_filters/mod.rs.html#538)

#### fn [max\_level\_hint](trait.Filter.html#method.max_level_hint)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[LevelFilter](../../tracing/level_filters/struct.LevelFilter.html "struct bevy::log::tracing::level_filters::LevelFilter")\>

Returns an optional hint of the highest [verbosity level](../../struct.Level.html "struct bevy::log::Level") that this `Filter` will enable. [Read more](trait.Filter.html#method.max_level_hint)

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/filter/layer_filters/mod.rs.html#538)

#### fn [event\_enabled](trait.Filter.html#method.event_enabled)(&self, event: &[Event](../../tracing/struct.Event.html "struct bevy::log::tracing::Event")<'\_>, cx: &[Context](struct.Context.html "struct bevy::log::tracing_subscriber::layer::Context")<'\_, S>) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Called before the filtered \[`Layer]'s [`on\_event`], to determine if` on\_event\` should be called. [Read more](trait.Filter.html#method.event_enabled)

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/filter/layer_filters/mod.rs.html#538)

#### fn [on\_new\_span](trait.Filter.html#method.on_new_span)(&self, attrs: &[Attributes](../../tracing/span/struct.Attributes.html "struct bevy::log::tracing::span::Attributes")<'\_>, id: &[Id](../../tracing/struct.Id.html "struct bevy::log::tracing::Id"), ctx: [Context](struct.Context.html "struct bevy::log::tracing_subscriber::layer::Context")<'\_, S>)

Notifies this filter that a new span was constructed with the given `Attributes` and `Id`. [Read more](trait.Filter.html#method.on_new_span)

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/filter/layer_filters/mod.rs.html#538)

#### fn [on\_record](trait.Filter.html#method.on_record)(&self, id: &[Id](../../tracing/struct.Id.html "struct bevy::log::tracing::Id"), values: &[Record](../../tracing/span/struct.Record.html "struct bevy::log::tracing::span::Record")<'\_>, ctx: [Context](struct.Context.html "struct bevy::log::tracing_subscriber::layer::Context")<'\_, S>)

Notifies this filter that a span with the given `Id` recorded the given `values`. [Read more](trait.Filter.html#method.on_record)

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/filter/layer_filters/mod.rs.html#538)

#### fn [on\_enter](trait.Filter.html#method.on_enter)(&self, id: &[Id](../../tracing/struct.Id.html "struct bevy::log::tracing::Id"), ctx: [Context](struct.Context.html "struct bevy::log::tracing_subscriber::layer::Context")<'\_, S>)

Notifies this filter that a span with the given ID was entered. [Read more](trait.Filter.html#method.on_enter)

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/filter/layer_filters/mod.rs.html#538)

#### fn [on\_exit](trait.Filter.html#method.on_exit)(&self, id: &[Id](../../tracing/struct.Id.html "struct bevy::log::tracing::Id"), ctx: [Context](struct.Context.html "struct bevy::log::tracing_subscriber::layer::Context")<'\_, S>)

Notifies this filter that a span with the given ID was exited. [Read more](trait.Filter.html#method.on_exit)

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/filter/layer_filters/mod.rs.html#538)

#### fn [on\_close](trait.Filter.html#method.on_close)(&self, id: [Id](../../tracing/struct.Id.html "struct bevy::log::tracing::Id"), ctx: [Context](struct.Context.html "struct bevy::log::tracing_subscriber::layer::Context")<'\_, S>)

Notifies this filter that a span with the given ID has been closed. [Read more](trait.Filter.html#method.on_close)

## Dyn Compatibility

This trait **is** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementations on Foreign Types

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/filter/layer_filters/mod.rs.html#544-546)

### impl<F, S> [Filter](trait.Filter.html "trait bevy::log::tracing_subscriber::layer::Filter")<S> for [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<F>

where F: [Filter](trait.Filter.html "trait bevy::log::tracing_subscriber::layer::Filter")<S>,

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/filter/layer_filters/mod.rs.html#549)

#### fn [enabled](#tymethod.enabled)(&self, meta: &[Metadata](../../tracing/struct.Metadata.html "struct bevy::log::tracing::Metadata")<'\_>, ctx: &[Context](struct.Context.html "struct bevy::log::tracing_subscriber::layer::Context")<'\_, S>) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/filter/layer_filters/mod.rs.html#556)

#### fn [callsite\_enabled](#method.callsite_enabled)(&self, meta: &'static [Metadata](../../tracing/struct.Metadata.html "struct bevy::log::tracing::Metadata")<'static>) -> [Interest](../../tracing/subscriber/struct.Interest.html "struct bevy::log::tracing::subscriber::Interest")

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/filter/layer_filters/mod.rs.html#563)

#### fn [max\_level\_hint](#method.max_level_hint)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[LevelFilter](../../tracing/level_filters/struct.LevelFilter.html "struct bevy::log::tracing::level_filters::LevelFilter")\>

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/filter/layer_filters/mod.rs.html#568)

#### fn [event\_enabled](#method.event_enabled)(&self, event: &[Event](../../tracing/struct.Event.html "struct bevy::log::tracing::Event")<'\_>, ctx: &[Context](struct.Context.html "struct bevy::log::tracing_subscriber::layer::Context")<'\_, S>) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/filter/layer_filters/mod.rs.html#575)

#### fn [on\_new\_span](#method.on_new_span)(&self, attrs: &[Attributes](../../tracing/span/struct.Attributes.html "struct bevy::log::tracing::span::Attributes")<'\_>, id: &[Id](../../tracing/struct.Id.html "struct bevy::log::tracing::Id"), ctx: [Context](struct.Context.html "struct bevy::log::tracing_subscriber::layer::Context")<'\_, S>)

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/filter/layer_filters/mod.rs.html#582)

#### fn [on\_record](#method.on_record)(&self, id: &[Id](../../tracing/struct.Id.html "struct bevy::log::tracing::Id"), values: &[Record](../../tracing/span/struct.Record.html "struct bevy::log::tracing::span::Record")<'\_>, ctx: [Context](struct.Context.html "struct bevy::log::tracing_subscriber::layer::Context")<'\_, S>)

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/filter/layer_filters/mod.rs.html#589)

#### fn [on\_enter](#method.on_enter)(&self, id: &[Id](../../tracing/struct.Id.html "struct bevy::log::tracing::Id"), ctx: [Context](struct.Context.html "struct bevy::log::tracing_subscriber::layer::Context")<'\_, S>)

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/filter/layer_filters/mod.rs.html#596)

#### fn [on\_exit](#method.on_exit)(&self, id: &[Id](../../tracing/struct.Id.html "struct bevy::log::tracing::Id"), ctx: [Context](struct.Context.html "struct bevy::log::tracing_subscriber::layer::Context")<'\_, S>)

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/filter/layer_filters/mod.rs.html#603)

#### fn [on\_close](#method.on_close)(&self, id: [Id](../../tracing/struct.Id.html "struct bevy::log::tracing::Id"), ctx: [Context](struct.Context.html "struct bevy::log::tracing_subscriber::layer::Context")<'\_, S>)

## Implementors

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/filter/layer_filters/combinator.rs.html#110-113)

### impl<A, B, S> [Filter](trait.Filter.html "trait bevy::log::tracing_subscriber::layer::Filter")<S> for [And](../filter/combinator/struct.And.html "struct bevy::log::tracing_subscriber::filter::combinator::And")<A, B, S>

where A: [Filter](trait.Filter.html "trait bevy::log::tracing_subscriber::layer::Filter")<S>, B: [Filter](trait.Filter.html "trait bevy::log::tracing_subscriber::layer::Filter")<S>,

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/filter/layer_filters/combinator.rs.html#293-296)

### impl<A, B, S> [Filter](trait.Filter.html "trait bevy::log::tracing_subscriber::layer::Filter")<S> for [Or](../filter/combinator/struct.Or.html "struct bevy::log::tracing_subscriber::filter::combinator::Or")<A, B, S>

where A: [Filter](trait.Filter.html "trait bevy::log::tracing_subscriber::layer::Filter")<S>, B: [Filter](trait.Filter.html "trait bevy::log::tracing_subscriber::layer::Filter")<S>,

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/filter/layer_filters/combinator.rs.html#467-469)

### impl<A, S> [Filter](trait.Filter.html "trait bevy::log::tracing_subscriber::layer::Filter")<S> for [Not](../filter/combinator/struct.Not.html "struct bevy::log::tracing_subscriber::filter::combinator::Not")<A, S>

where A: [Filter](trait.Filter.html "trait bevy::log::tracing_subscriber::layer::Filter")<S>,

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/filter/filter_fn.rs.html#726-729)

### impl<S, F, R> [Filter](trait.Filter.html "trait bevy::log::tracing_subscriber::layer::Filter")<S> for [DynFilterFn](../filter/struct.DynFilterFn.html "struct bevy::log::tracing_subscriber::filter::DynFilterFn")<S, F, R>

where F: [Fn](https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html "trait core::ops::function::Fn")(&[Metadata](../../tracing/struct.Metadata.html "struct bevy::log::tracing::Metadata")<'\_>, &[Context](struct.Context.html "struct bevy::log::tracing_subscriber::layer::Context")<'\_, S>) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html), R: [Fn](https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html "trait core::ops::function::Fn")(&'static [Metadata](../../tracing/struct.Metadata.html "struct bevy::log::tracing::Metadata")<'static>) -> [Interest](../../tracing/subscriber/struct.Interest.html "struct bevy::log::tracing::subscriber::Interest"),

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/filter/filter_fn.rs.html#709-711)

### impl<S, F> [Filter](trait.Filter.html "trait bevy::log::tracing_subscriber::layer::Filter")<S> for [FilterFn](../filter/struct.FilterFn.html "struct bevy::log::tracing_subscriber::filter::FilterFn")<F>

where F: [Fn](https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html "trait core::ops::function::Fn")(&[Metadata](../../tracing/struct.Metadata.html "struct bevy::log::tracing::Metadata")<'\_>) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html),

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/reload.rs.html#211-214)

### impl<S, L> [Filter](trait.Filter.html "trait bevy::log::tracing_subscriber::layer::Filter")<S> for [Layer](../reload/struct.Layer.html "struct bevy::log::tracing_subscriber::reload::Layer")<L, S>

where L: [Filter](trait.Filter.html "trait bevy::log::tracing_subscriber::layer::Filter")<S> + 'static, S: [Subscriber](../../tracing/trait.Subscriber.html "trait bevy::log::tracing::Subscriber"),

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/filter/layer_filters/mod.rs.html#531)

### impl<S> [Filter](trait.Filter.html "trait bevy::log::tracing_subscriber::layer::Filter")<S> for [Arc](../../../platform/sync/struct.Arc.html "struct bevy::platform::sync::Arc")<dyn [Filter](trait.Filter.html "trait bevy::log::tracing_subscriber::layer::Filter")<S> + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync")\>

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/filter/layer_filters/mod.rs.html#537)

### impl<S> [Filter](trait.Filter.html "trait bevy::log::tracing_subscriber::layer::Filter")<S> for [Box](../../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Filter](trait.Filter.html "trait bevy::log::tracing_subscriber::layer::Filter")<S> + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync")\>

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/filter/env/mod.rs.html#708)

### impl<S> [Filter](trait.Filter.html "trait bevy::log::tracing_subscriber::layer::Filter")<S> for [EnvFilter](../struct.EnvFilter.html "struct bevy::log::tracing_subscriber::EnvFilter")

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/filter/layer_filters/mod.rs.html#462)

### impl<S> [Filter](trait.Filter.html "trait bevy::log::tracing_subscriber::layer::Filter")<S> for [LevelFilter](../../tracing/level_filters/struct.LevelFilter.html "struct bevy::log::tracing::level_filters::LevelFilter")

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/filter/targets.rs.html#457)

### impl<S> [Filter](trait.Filter.html "trait bevy::log::tracing_subscriber::layer::Filter")<S> for [Targets](../filter/struct.Targets.html "struct bevy::log::tracing_subscriber::filter::Targets")