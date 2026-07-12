[bevy](../../index.html)::[reflect](../index.html)::[utility](index.html)

# Trait TypedProperty 

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/utility.rs.html#18)

```rust
pub trait TypedProperty: Sealed {
    type Stored: 'static;
}
```

A type that can be stored in a ([`Non`](struct.NonGenericTypeCell.html "struct bevy::reflect::utility::NonGenericTypeCell"))[`GenericTypeCell`](struct.GenericTypeCell.html "struct bevy::reflect::utility::GenericTypeCell").

## Required Associated Types

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/utility.rs.html#20)

#### type [Stored](#associatedtype.Stored): 'static

The type of the value stored in [`GenericTypeCell`](struct.GenericTypeCell.html "struct bevy::reflect::utility::GenericTypeCell").

## Dyn Compatibility

This trait **is** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/utility.rs.html#38)

### impl [TypedProperty](trait.TypedProperty.html "trait bevy::reflect::utility::TypedProperty") for [TypeInfo](../enum.TypeInfo.html "enum bevy::reflect::TypeInfo")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/utility.rs.html#39)

#### type [Stored](#associatedtype.Stored) = [TypeInfo](../enum.TypeInfo.html "enum bevy::reflect::TypeInfo")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/utility.rs.html#42)

### impl [TypedProperty](trait.TypedProperty.html "trait bevy::reflect::utility::TypedProperty") for [TypePathComponent](struct.TypePathComponent.html "struct bevy::reflect::utility::TypePathComponent")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/utility.rs.html#43)

#### type [Stored](#associatedtype.Stored) = [String](../../prelude/struct.String.html "struct bevy::prelude::String")