[bevy](../../../index.html)::[log](../../index.html)::[tracing\_subscriber](../index.html)::[filter](index.html)

# Trait FilterExt 

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/filter/layer_filters/mod.rs.html#182)

```rust
pub trait FilterExt<S>: Filter<S> {
    // Provided methods
    fn and<B>(self, other: B) -> And<Self, B, S>
       where Self: Sized,
             B: Filter<S> { ... }
    fn or<B>(self, other: B) -> Or<Self, B, S>
       where Self: Sized,
             B: Filter<S> { ... }
    fn not(self) -> Not<Self, S>
       where Self: Sized { ... }
    fn boxed(self) -> Box<dyn Filter<S> + Send + Sync>
       where Self: Sized + Send + Sync + 'static { ... }
}
```

Extension trait adding [combinators](combinator/index.html "mod bevy::log::tracing_subscriber::filter::combinator") for combining [`Filter`](../layer/trait.Filter.html "trait bevy::log::tracing_subscriber::layer::Filter").

## Provided Methods

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/filter/layer_filters/mod.rs.html#224-227)

#### fn [and](#method.and)<B>(self, other: B) -> [And](combinator/struct.And.html "struct bevy::log::tracing_subscriber::filter::combinator::And")<Self, B, S>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), B: [Filter](../layer/trait.Filter.html "trait bevy::log::tracing_subscriber::layer::Filter")<S>,

Combines this [`Filter`](../layer/trait.Filter.html "trait bevy::log::tracing_subscriber::layer::Filter") with another [`Filter`](../layer/trait.Filter.html "trait bevy::log::tracing_subscriber::layer::Filter") s so that spans and events are enabled if and only if _both_ filters return `true`.

##### Examples

Enabling spans or events if they have both a particular target _and_ are above a certain level:

```rust
use tracing_subscriber::{
    filter::{filter_fn, LevelFilter, FilterExt},
    prelude::*,
};

// Enables spans and events with targets starting with `interesting_target`:
let target_filter = filter_fn(|meta| {
    meta.target().starts_with("interesting_target")
});

// Enables spans and events with levels `INFO` and below:
let level_filter = LevelFilter::INFO;

// Combine the two filters together, returning a filter that only enables
// spans and events that *both* filters will enable:
let filter = target_filter.and(level_filter);

tracing_subscriber::registry()
    .with(tracing_subscriber::fmt::layer().with_filter(filter))
    .init();

// This event will *not* be enabled:
tracing::info!("an event with an uninteresting target");

// This event *will* be enabled:
tracing::info!(target: "interesting_target", "a very interesting event");

// This event will *not* be enabled:
tracing::debug!(target: "interesting_target", "interesting debug event...");
```

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/filter/layer_filters/mod.rs.html#306-309)

#### fn [or](#method.or)<B>(self, other: B) -> [Or](combinator/struct.Or.html "struct bevy::log::tracing_subscriber::filter::combinator::Or")<Self, B, S>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), B: [Filter](../layer/trait.Filter.html "trait bevy::log::tracing_subscriber::layer::Filter")<S>,

Combines two [`Filter`](../layer/trait.Filter.html "trait bevy::log::tracing_subscriber::layer::Filter")s so that spans and events are enabled if _either_ filter returns `true`.

##### Examples

Enabling spans and events at the `INFO` level and above, and all spans and events with a particular target:

```rust
use tracing_subscriber::{
    filter::{filter_fn, LevelFilter, FilterExt},
    prelude::*,
};

// Enables spans and events with targets starting with `interesting_target`:
let target_filter = filter_fn(|meta| {
    meta.target().starts_with("interesting_target")
});

// Enables spans and events with levels `INFO` and below:
let level_filter = LevelFilter::INFO;

// Combine the two filters together so that a span or event is enabled
// if it is at INFO or lower, or if it has a target starting with
// `interesting_target`.
let filter = level_filter.or(target_filter);

tracing_subscriber::registry()
    .with(tracing_subscriber::fmt::layer().with_filter(filter))
    .init();

// This event will *not* be enabled:
tracing::debug!("an uninteresting event");

// This event *will* be enabled:
tracing::info!("an uninteresting INFO event");

// This event *will* be enabled:
tracing::info!(target: "interesting_target", "a very interesting event");

// This event *will* be enabled:
tracing::debug!(target: "interesting_target", "interesting debug event...");
```

Enabling a higher level for a particular target by using `or` in conjunction with the [`and`](trait.FilterExt.html#method.and "method bevy::log::tracing_subscriber::filter::FilterExt::and") combinator:

```rust
use tracing_subscriber::{
    filter::{filter_fn, LevelFilter, FilterExt},
    prelude::*,
};

// This filter will enable spans and events with targets beginning with
// `my_crate`:
let my_crate = filter_fn(|meta| {
    meta.target().starts_with("my_crate")
});

let filter = my_crate
    // Combine the `my_crate` filter with a `LevelFilter` to produce a
    // filter that will enable the `INFO` level and lower for spans and
    // events with `my_crate` targets:
    .and(LevelFilter::INFO)
    // If a span or event *doesn't* have a target beginning with
    // `my_crate`, enable it if it has the `WARN` level or lower:
    .or(LevelFilter::WARN);

tracing_subscriber::registry()
    .with(tracing_subscriber::fmt::layer().with_filter(filter))
    .init();
```

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/filter/layer_filters/mod.rs.html#373-375)

#### fn [not](#method.not)(self) -> [Not](combinator/struct.Not.html "struct bevy::log::tracing_subscriber::filter::combinator::Not")<Self, S>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Inverts `self`, returning a filter that enables spans and events only if `self` would _not_ enable them.

This inverts the values returned by the [`enabled`](../layer/trait.Filter.html#tymethod.enabled "method bevy::log::tracing_subscriber::layer::Filter::enabled") and [`callsite_enabled`](../layer/trait.Filter.html#method.callsite_enabled "method bevy::log::tracing_subscriber::layer::Filter::callsite_enabled") methods on the wrapped filter; it does _not_ invert [`event_enabled`](../layer/trait.Filter.html#method.event_enabled "method bevy::log::tracing_subscriber::layer::Filter::event_enabled"), as filters which do not implement filtering on event field values will return the default `true` even for events that their [`enabled`](../layer/trait.Filter.html#tymethod.enabled "method bevy::log::tracing_subscriber::layer::Filter::enabled") method disables.

Consider a normal filter defined as:

[ⓘ](# "This example is not tested")

```rust
// for spans
match callsite_enabled() {
    ALWAYS => on_span(),
    SOMETIMES => if enabled() { on_span() },
    NEVER => (),
}
// for events
match callsite_enabled() {
   ALWAYS => on_event(),
   SOMETIMES => if enabled() && event_enabled() { on_event() },
   NEVER => (),
}
```

and an inverted filter defined as:

[ⓘ](# "This example is not tested")

```rust
// for spans
match callsite_enabled() {
    ALWAYS => (),
    SOMETIMES => if !enabled() { on_span() },
    NEVER => on_span(),
}
// for events
match callsite_enabled() {
    ALWAYS => (),
    SOMETIMES => if !enabled() { on_event() },
    NEVER => on_event(),
}
```

A proper inversion would do `!(enabled() && event_enabled())` (or `!enabled() || !event_enabled()`), but because of the implicit `&&` relation between `enabled` and `event_enabled`, it is difficult to short circuit and not call the wrapped `event_enabled`.

A combinator which remembers the result of `enabled` in order to call `event_enabled` only when `enabled() == true` is possible, but requires additional thread-local mutable state to support a very niche use case.

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/filter/layer_filters/mod.rs.html#450-452)

#### fn [boxed](#method.boxed)(self) -> [Box](../../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Filter](../layer/trait.Filter.html "trait bevy::log::tracing_subscriber::layer::Filter")<S> + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync")\>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Boxes](https://doc.rust-lang.org/nightly/alloc/boxed/index.html "mod alloc::boxed") `self`, erasing its concrete type.

This is equivalent to calling [`Box::new`](../../../prelude/struct.Box.html#method.new "associated function bevy::prelude::Box::new"), but in method form, so that it can be used when chaining combinator methods.

##### Examples

When different combinations of filters are used conditionally, they may have different types. For example, the following code won’t compile, since the `if` and `else` clause produce filters of different types:

[ⓘ](# "This example deliberately fails to compile")

```rust
use tracing_subscriber::{
    filter::{filter_fn, LevelFilter, FilterExt},
    prelude::*,
};

let enable_bar_target: bool = // ...

let filter = if enable_bar_target {
    filter_fn(|meta| meta.target().starts_with("foo"))
        // If `enable_bar_target` is true, add a `filter_fn` enabling
        // spans and events with the target `bar`:
        .or(filter_fn(|meta| meta.target().starts_with("bar")))
        .and(LevelFilter::INFO)
} else {
    filter_fn(|meta| meta.target().starts_with("foo"))
        .and(LevelFilter::INFO)
};

tracing_subscriber::registry()
    .with(tracing_subscriber::fmt::layer().with_filter(filter))
    .init();
```

By using `boxed`, the types of the two different branches can be erased, so the assignment to the `filter` variable is valid (as both branches have the type `Box<dyn Filter<S> + Send + Sync + 'static>`). The following code _does_ compile:

```rust
use tracing_subscriber::{
    filter::{filter_fn, LevelFilter, FilterExt},
    prelude::*,
};

let enable_bar_target: bool = // ...

let filter = if enable_bar_target {
    filter_fn(|meta| meta.target().starts_with("foo"))
        .or(filter_fn(|meta| meta.target().starts_with("bar")))
        .and(LevelFilter::INFO)
        // Boxing the filter erases its type, so both branches now
        // have the same type.
        .boxed()
} else {
    filter_fn(|meta| meta.target().starts_with("foo"))
        .and(LevelFilter::INFO)
        .boxed()
};

tracing_subscriber::registry()
    .with(tracing_subscriber::fmt::layer().with_filter(filter))
    .init();
```

## Dyn Compatibility

This trait **is** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/filter/layer_filters/mod.rs.html#1041)

### impl<F, S> [FilterExt](trait.FilterExt.html "trait bevy::log::tracing_subscriber::filter::FilterExt")<S> for F

where F: [Filter](../layer/trait.Filter.html "trait bevy::log::tracing_subscriber::layer::Filter")<S>,