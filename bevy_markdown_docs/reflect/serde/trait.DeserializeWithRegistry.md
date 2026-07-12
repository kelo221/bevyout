[bevy](../../index.html)::[reflect](../index.html)::[serde](index.html)

# Trait DeserializeWithRegistry 

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/serde/de/deserialize_with_registry.rs.html#44)

```rust
pub trait DeserializeWithRegistry<'de>: Sized {
    // Required method
    fn deserialize<D>(
        deserializer: D,
        registry: &TypeRegistry,
    ) -> Result<Self, <D as Deserializer<'de>>::Error>
       where D: Deserializer<'de>;
}
```

Trait used to provide finer control when deserializing a reflected type with one of the reflection deserializers.

This trait is the reflection equivalent of `serde`’s [`Deserialize`](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserialize.html "trait serde_core::de::Deserialize") trait. The main difference is that this trait provides access to the [`TypeRegistry`](../struct.TypeRegistry.html "struct bevy::reflect::TypeRegistry"), which means that we can use the registry and all its stored type information to deserialize our type.

This can be useful when writing a custom reflection deserializer where we may want to handle parts of the deserialization process, but temporarily pass control to the standard reflection deserializer for other parts.

For the serialization equivalent of this trait, see [`SerializeWithRegistry`](trait.SerializeWithRegistry.html "trait bevy::reflect::serde::SerializeWithRegistry").

## Rationale

Without this trait and its associated [type data](struct.ReflectDeserializeWithRegistry.html "struct bevy::reflect::serde::ReflectDeserializeWithRegistry"), such a deserializer would have to write out all of the deserialization logic itself, possibly including unnecessary code duplication and trivial implementations.

This is because a normal [`Deserialize`](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserialize.html "trait serde_core::de::Deserialize") implementation has no knowledge of the [`TypeRegistry`](../struct.TypeRegistry.html "struct bevy::reflect::TypeRegistry") and therefore cannot create a reflection-based deserializer for nested items.

## Implementors

In order for this to work with the reflection deserializers like [`TypedReflectDeserializer`](struct.TypedReflectDeserializer.html "struct bevy::reflect::serde::TypedReflectDeserializer") and [`ReflectDeserializer`](struct.ReflectDeserializer.html "struct bevy::reflect::serde::ReflectDeserializer"), implementors should be sure to register the [`ReflectDeserializeWithRegistry`](struct.ReflectDeserializeWithRegistry.html "struct bevy::reflect::serde::ReflectDeserializeWithRegistry") type data. This can be done [via the registry](../struct.TypeRegistry.html#method.register_type_data "method bevy::reflect::TypeRegistry::register_type_data") or by adding `#[reflect(DeserializeWithRegistry)]` to the type definition.

## Required Methods

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/serde/de/deserialize_with_registry.rs.html#48-50)

#### fn [deserialize](#tymethod.deserialize)<D>( deserializer: D, registry: &[TypeRegistry](../struct.TypeRegistry.html "struct bevy::reflect::TypeRegistry"), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Self, <D as [Deserializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html "trait serde_core::de::Deserializer")<'de>>::[Error](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#associatedtype.Error "type serde_core::de::Deserializer::Error")\>

where D: [Deserializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html "trait serde_core::de::Deserializer")<'de>,

Deserialize this value using the given [Deserializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html "trait serde_core::de::Deserializer") and [`TypeRegistry`](../struct.TypeRegistry.html "struct bevy::reflect::TypeRegistry").

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors