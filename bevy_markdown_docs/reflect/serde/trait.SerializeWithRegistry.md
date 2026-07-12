[bevy](../../index.html)::[reflect](../index.html)::[serde](index.html)

# Trait SerializeWithRegistry 

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/serde/ser/serialize_with_registry.rs.html#42)

```rust
pub trait SerializeWithRegistry {
    // Required method
    fn serialize<S>(
        &self,
        serializer: S,
        registry: &TypeRegistry,
    ) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
       where S: Serializer;
}
```

Trait used to provide finer control when serializing a reflected type with one of the reflection serializers.

This trait is the reflection equivalent of `serde`’s [`Serialize`](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serialize.html "trait serde_core::ser::Serialize") trait. The main difference is that this trait provides access to the [`TypeRegistry`](../struct.TypeRegistry.html "struct bevy::reflect::TypeRegistry"), which means that we can use the registry and all its stored type information to serialize our type.

This can be useful when writing a custom reflection serializer where we may want to handle parts of the serialization process, but temporarily pass control to the standard reflection serializer for other parts.

For the deserialization equivalent of this trait, see [`DeserializeWithRegistry`](trait.DeserializeWithRegistry.html "trait bevy::reflect::serde::DeserializeWithRegistry").

## Rationale

Without this trait and its associated [type data](struct.ReflectSerializeWithRegistry.html "struct bevy::reflect::serde::ReflectSerializeWithRegistry"), such a serializer would have to write out all of the serialization logic itself, possibly including unnecessary code duplication and trivial implementations.

This is because a normal [`Serialize`](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serialize.html "trait serde_core::ser::Serialize") implementation has no knowledge of the [`TypeRegistry`](../struct.TypeRegistry.html "struct bevy::reflect::TypeRegistry") and therefore cannot create a reflection-based serializer for nested items.

## Implementors

In order for this to work with the reflection serializers like [`TypedReflectSerializer`](struct.TypedReflectSerializer.html "struct bevy::reflect::serde::TypedReflectSerializer") and [`ReflectSerializer`](struct.ReflectSerializer.html "struct bevy::reflect::serde::ReflectSerializer"), implementors should be sure to register the [`ReflectSerializeWithRegistry`](struct.ReflectSerializeWithRegistry.html "struct bevy::reflect::serde::ReflectSerializeWithRegistry") type data. This can be done [via the registry](../struct.TypeRegistry.html#method.register_type_data "method bevy::reflect::TypeRegistry::register_type_data") or by adding `#[reflect(SerializeWithRegistry)]` to the type definition.

## Required Methods

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/serde/ser/serialize_with_registry.rs.html#46-48)

#### fn [serialize](#tymethod.serialize)<S>( &self, serializer: S, registry: &[TypeRegistry](../struct.TypeRegistry.html "struct bevy::reflect::TypeRegistry"), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<S as [Serializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html "trait serde_core::ser::Serializer")\>::[Ok](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html#associatedtype.Ok "type serde_core::ser::Serializer::Ok"), <S as [Serializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html "trait serde_core::ser::Serializer")\>::[Error](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html#associatedtype.Error "type serde_core::ser::Serializer::Error")\>

where S: [Serializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html "trait serde_core::ser::Serializer"),

Serialize this value using the given [Serializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html "trait serde_core::ser::Serializer") and [`TypeRegistry`](../struct.TypeRegistry.html "struct bevy::reflect::TypeRegistry").

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors