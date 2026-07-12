[bevy](../../../index.html)::[log](../../index.html)::[tracing](../index.html)::[span](index.html)

# Trait AsId 

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/span.rs.html#336)

```rust
pub trait AsId: Sealed {
    // Required method
    fn as_id(&self) -> Option<&Id>;
}
```

Trait implemented by types which have a span `Id`.

## Required Methods

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/span.rs.html#339)

#### fn [as\_id](#tymethod.as_id)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&[Id](../struct.Id.html "struct bevy::log::tracing::Id")\>

Returns the `Id` of the span that `self` corresponds to, or `None` if this corresponds to a disabled span.

## Dyn Compatibility

This trait **is** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors