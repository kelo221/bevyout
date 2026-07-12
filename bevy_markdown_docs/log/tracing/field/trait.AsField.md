[bevy](../../../index.html)::[log](../../index.html)::[tracing](../index.html)::[field](index.html)

# Trait AsField 

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/field.rs.html#129)

```rust
pub trait AsField: Sealed {
    // Required method
    fn as_field(&self, metadata: &Metadata<'_>) -> Option<Field>;
}
```

Trait implemented to allow a type to be used as a field key.

**Note**: Although this is implemented for both the
[`Field`](./struct.Field.html) type _and_ any
type that can be borrowed as an `&str`, only `Field`
allows _O_(1) access.
Indexing a field with a string results in an iterative search that performs
string comparisons. Thus, if possible, once the key for a field is known, it
should be used whenever possible.

## Required Methods

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/field.rs.html#134)

#### fn [as\_field](#tymethod.as_field)(&self, metadata: &[Metadata](../struct.Metadata.html "struct bevy::log::tracing::Metadata")<'\_>) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Field](struct.Field.html "struct bevy::log::tracing::field::Field")\>

Attempts to convert `&self` into a `Field` with the specified `metadata`.

If `metadata` defines this field, then the field is returned. Otherwise, this returns `None`.

## Dyn Compatibility

This trait **is** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementations on Foreign Types

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/field.rs.html#161)

### impl [AsField](trait.AsField.html "trait bevy::log::tracing::field::AsField") for [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/field.rs.html#163)

#### fn [as\_field](#tymethod.as_field)(&self, metadata: &[Metadata](../struct.Metadata.html "struct bevy::log::tracing::Metadata")<'\_>) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Field](struct.Field.html "struct bevy::log::tracing::field::Field")\>

## Implementors

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/field.rs.html#150)

### impl [AsField](trait.AsField.html "trait bevy::log::tracing::field::AsField") for &[Field](struct.Field.html "struct bevy::log::tracing::field::Field")

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/field.rs.html#139)

### impl [AsField](trait.AsField.html "trait bevy::log::tracing::field::AsField") for [Field](struct.Field.html "struct bevy::log::tracing::field::Field")