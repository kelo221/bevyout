[bevy](../../../index.html)::[log](../../index.html)::[tracing\_subscriber](../index.html)::[registry](index.html)

# Trait LookupSpan 

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/registry/mod.rs.html#92)

```rust
pub trait LookupSpan<'a> {
    type Data: SpanData<'a>;

    // Required method
    fn span_data(&'a self, id: &Id) -> Option<Self::Data>;

    // Provided methods
    fn span(&'a self, id: &Id) -> Option<SpanRef<'a, Self>>
       where Self: Sized { ... }
    fn register_filter(&mut self) -> FilterId { ... }
}
```

Provides access to stored span data.

Subscribers which store span data and associate it with span IDs should implement this trait; if they do, any [`Layer`](../trait.Layer.html "trait bevy::log::tracing_subscriber::Layer")s wrapping them can look up metadata via the [`Context`](../layer/struct.Context.html "struct bevy::log::tracing_subscriber::layer::Context") type’s [`span()`](../layer/struct.Context.html#method.span "method bevy::log::tracing_subscriber::layer::Context::span") method.

## Required Associated Types

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/registry/mod.rs.html#94)

#### type [Data](#associatedtype.Data): [SpanData](trait.SpanData.html "trait bevy::log::tracing_subscriber::registry::SpanData")<'a>

The type of span data stored in this registry.

## Required Methods

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/registry/mod.rs.html#106)

#### fn [span\_data](#tymethod.span_data)(&'a self, id: &[Id](../../tracing/struct.Id.html "struct bevy::log::tracing::Id")) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<Self::[Data](trait.LookupSpan.html#associatedtype.Data "type bevy::log::tracing_subscriber::registry::LookupSpan::Data")\>

Returns the [`SpanData`](trait.SpanData.html "trait bevy::log::tracing_subscriber::registry::SpanData") for a given `Id`, if it exists.

**Note**: users of the `LookupSpan` trait should
typically call the [`span`](#method.span) method rather
than this method. The `span` method is implemented by
_calling_ `span_data`, but returns a reference which is
capable of performing more sophisiticated queries.

## Provided Methods

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/registry/mod.rs.html#118-120)

#### fn [span](#method.span)(&'a self, id: &[Id](../../tracing/struct.Id.html "struct bevy::log::tracing::Id")) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[SpanRef](struct.SpanRef.html "struct bevy::log::tracing_subscriber::registry::SpanRef")<'a, Self>>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Returns a [`SpanRef`](struct.SpanRef.html "struct bevy::log::tracing_subscriber::registry::SpanRef") for the span with the given `Id`, if it exists.

A `SpanRef` is similar to [`SpanData`](trait.SpanData.html "trait bevy::log::tracing_subscriber::registry::SpanData"), but it allows performing additional lookups against the registryr that stores the wrapped data.

In general, _users_ of the `LookupSpan` trait should use this method rather than the [`span_data`](trait.LookupSpan.html#tymethod.span_data "method bevy::log::tracing_subscriber::registry::LookupSpan::span_data") method; while _implementors_ of this trait should only implement `span_data`.

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/registry/mod.rs.html#148)

#### fn [register\_filter](#method.register_filter)(&mut self) -> [FilterId](../filter/struct.FilterId.html "struct bevy::log::tracing_subscriber::filter::FilterId")

Available on **crate feature `registry`** only.

Registers a [`Filter`](../layer/trait.Filter.html "trait bevy::log::tracing_subscriber::layer::Filter") for [per-layer filtering](../trait.Layer.html#per-layer-filtering "trait bevy::log::tracing_subscriber::Layer") with this [`Subscriber`](../../tracing/trait.Subscriber.html "trait bevy::log::tracing::Subscriber").

The [`Filter`](../layer/trait.Filter.html "trait bevy::log::tracing_subscriber::layer::Filter") can then use the returned [`FilterId`](../filter/struct.FilterId.html "struct bevy::log::tracing_subscriber::filter::FilterId") to [check if it previously enabled a span](trait.SpanData.html#method.is_enabled_for "method bevy::log::tracing_subscriber::registry::SpanData::is_enabled_for").

##### Panics

If this `Subscriber` does not support [per-layer filtering](../trait.Layer.html#per-layer-filtering "trait bevy::log::tracing_subscriber::Layer").

## Dyn Compatibility

This trait **is** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/layer/layered.rs.html#386-388)

### impl<'a, L, S> [LookupSpan](trait.LookupSpan.html "trait bevy::log::tracing_subscriber::registry::LookupSpan")<'a> for [Layered](../layer/struct.Layered.html "struct bevy::log::tracing_subscriber::layer::Layered")<L, S>

where S: [Subscriber](../../tracing/trait.Subscriber.html "trait bevy::log::tracing::Subscriber") + [LookupSpan](trait.LookupSpan.html "trait bevy::log::tracing_subscriber::registry::LookupSpan")<'a>,

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/layer/layered.rs.html#390)

#### type [Data](#associatedtype.Data) = <S as [LookupSpan](trait.LookupSpan.html "trait bevy::log::tracing_subscriber::registry::LookupSpan")<'a>>::[Data](trait.LookupSpan.html#associatedtype.Data "type bevy::log::tracing_subscriber::registry::LookupSpan::Data")

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/fmt/mod.rs.html#454-456)

### impl<'a, N, E, F, W> [LookupSpan](trait.LookupSpan.html "trait bevy::log::tracing_subscriber::registry::LookupSpan")<'a> for [Subscriber](../struct.FmtSubscriber.html "struct bevy::log::tracing_subscriber::FmtSubscriber")<N, E, F, W>

where [Layered](../layer/struct.Layered.html "struct bevy::log::tracing_subscriber::layer::Layered")<F, [Layered](../layer/struct.Layered.html "struct bevy::log::tracing_subscriber::layer::Layered")<[Layer](../fmt/struct.Layer.html "struct bevy::log::tracing_subscriber::fmt::Layer")<[Registry](../struct.Registry.html "struct bevy::log::tracing_subscriber::Registry"), N, E, W>, [Registry](../struct.Registry.html "struct bevy::log::tracing_subscriber::Registry")\>>: [LookupSpan](trait.LookupSpan.html "trait bevy::log::tracing_subscriber::registry::LookupSpan")<'a>,

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/fmt/mod.rs.html#458)

#### type [Data](#associatedtype.Data) = <[Layered](../layer/struct.Layered.html "struct bevy::log::tracing_subscriber::layer::Layered")<F, [Layered](../layer/struct.Layered.html "struct bevy::log::tracing_subscriber::layer::Layered")<[Layer](../fmt/struct.Layer.html "struct bevy::log::tracing_subscriber::fmt::Layer")<[Registry](../struct.Registry.html "struct bevy::log::tracing_subscriber::Registry"), N, E, W>, [Registry](../struct.Registry.html "struct bevy::log::tracing_subscriber::Registry")\>> as [LookupSpan](trait.LookupSpan.html "trait bevy::log::tracing_subscriber::registry::LookupSpan")<'a>>::[Data](trait.LookupSpan.html#associatedtype.Data "type bevy::log::tracing_subscriber::registry::LookupSpan::Data")

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/registry/sharded.rs.html#364)

### impl<'a> [LookupSpan](trait.LookupSpan.html "trait bevy::log::tracing_subscriber::registry::LookupSpan")<'a> for [Registry](../struct.Registry.html "struct bevy::log::tracing_subscriber::Registry")

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/registry/sharded.rs.html#365)

#### type [Data](#associatedtype.Data) = [Data](struct.Data.html "struct bevy::log::tracing_subscriber::registry::Data")<'a>