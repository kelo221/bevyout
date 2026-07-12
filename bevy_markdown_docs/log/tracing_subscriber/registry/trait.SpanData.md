[bevy](../../../index.html)::[log](../../index.html)::[tracing\_subscriber](../index.html)::[registry](index.html)

# Trait SpanData 

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/registry/mod.rs.html#157)

```rust
pub trait SpanData<'a> {
    // Required methods
    fn id(&self) -> Id;
    fn metadata(&self) -> &'static Metadata<'static>;
    fn parent(&self) -> Option<&Id>;
    fn extensions(&self) -> Extensions<'_>;
    fn extensions_mut(&self) -> ExtensionsMut<'_>;

    // Provided method
    fn is_enabled_for(&self, filter: FilterId) -> bool { ... }
}
```

A stored representation of data associated with a span.

## Required Methods

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/registry/mod.rs.html#159)

#### fn [id](#tymethod.id)(&self) -> [Id](../../tracing/struct.Id.html "struct bevy::log::tracing::Id")

Returns this span’s ID.

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/registry/mod.rs.html#162)

#### fn [metadata](#tymethod.metadata)(&self) -> &'static [Metadata](../../tracing/struct.Metadata.html "struct bevy::log::tracing::Metadata")<'static>

Returns a reference to the span’s `Metadata`.

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/registry/mod.rs.html#165)

#### fn [parent](#tymethod.parent)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&[Id](../../tracing/struct.Id.html "struct bevy::log::tracing::Id")\>

Returns a reference to the ID

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/registry/mod.rs.html#173)

#### fn [extensions](#tymethod.extensions)(&self) -> [Extensions](struct.Extensions.html "struct bevy::log::tracing_subscriber::registry::Extensions")<'\_>

Available on **crate feature `std`** only.

Returns a reference to this span’s `Extensions`.

The extensions may be used by `Layer`s to store additional data describing the span.

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/registry/mod.rs.html#181)

#### fn [extensions\_mut](#tymethod.extensions_mut)(&self) -> [ExtensionsMut](struct.ExtensionsMut.html "struct bevy::log::tracing_subscriber::registry::ExtensionsMut")<'\_>

Available on **crate feature `std`** only.

Returns a mutable reference to this span’s `Extensions`.

The extensions may be used by `Layer`s to store additional data describing the span.

## Provided Methods

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/registry/mod.rs.html#195)

#### fn [is\_enabled\_for](#method.is_enabled_for)(&self, filter: [FilterId](../filter/struct.FilterId.html "struct bevy::log::tracing_subscriber::filter::FilterId")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Available on **crate feature `registry`** only.

Returns `true` if this span is enabled for the [per-layer filter](../trait.Layer.html#per-layer-filtering "trait bevy::log::tracing_subscriber::Layer") corresponding to the provided [`FilterId`](../filter/struct.FilterId.html "struct bevy::log::tracing_subscriber::filter::FilterId").

###### Default Implementation

By default, this method assumes that the [`LookupSpan`](trait.LookupSpan.html "trait bevy::log::tracing_subscriber::registry::LookupSpan") implementation does not support [per-layer filtering](../trait.Layer.html#per-layer-filtering "trait bevy::log::tracing_subscriber::Layer"), and always returns `true`.

## Dyn Compatibility

This trait **is** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/registry/sharded.rs.html#414)

### impl<'a> [SpanData](trait.SpanData.html "trait bevy::log::tracing_subscriber::registry::SpanData")<'a> for [Data](struct.Data.html "struct bevy::log::tracing_subscriber::registry::Data")<'a>