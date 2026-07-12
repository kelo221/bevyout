[bevy](../../index.html)::[reflect](../index.html)

# Module serde 

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/lib.rs.html#651)

Serde integration for reflected types.

## Structs

[ReflectDeserializeWithRegistry](struct.ReflectDeserializeWithRegistry.html "struct bevy::reflect::serde::ReflectDeserializeWithRegistry")

Type data used to deserialize a [`PartialReflect`](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") type with a custom [`DeserializeWithRegistry`](trait.DeserializeWithRegistry.html "trait bevy::reflect::serde::DeserializeWithRegistry") implementation.

[ReflectDeserializer](struct.ReflectDeserializer.html "struct bevy::reflect::serde::ReflectDeserializer")

A general purpose deserializer for reflected types.

[ReflectSerializeWithRegistry](struct.ReflectSerializeWithRegistry.html "struct bevy::reflect::serde::ReflectSerializeWithRegistry")

Type data used to serialize a [`Reflect`](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") type with a custom [`SerializeWithRegistry`](trait.SerializeWithRegistry.html "trait bevy::reflect::serde::SerializeWithRegistry") implementation.

[ReflectSerializer](struct.ReflectSerializer.html "struct bevy::reflect::serde::ReflectSerializer")

A general purpose serializer for reflected types.

[SerializationData](struct.SerializationData.html "struct bevy::reflect::serde::SerializationData")

Contains data relevant to the automatic reflect powered (de)serialization of a type.

[SkippedField](struct.SkippedField.html "struct bevy::reflect::serde::SkippedField")

Data needed for (de)serialization of a skipped field.

[TypeRegistrationDeserializer](struct.TypeRegistrationDeserializer.html "struct bevy::reflect::serde::TypeRegistrationDeserializer")

A deserializer for type registrations.

[TypedReflectDeserializer](struct.TypedReflectDeserializer.html "struct bevy::reflect::serde::TypedReflectDeserializer")

A deserializer for reflected types whose [`TypeRegistration`](../struct.TypeRegistration.html "struct bevy::reflect::TypeRegistration") is known.

[TypedReflectSerializer](struct.TypedReflectSerializer.html "struct bevy::reflect::serde::TypedReflectSerializer")

A serializer for reflected types whose type will be known during deserialization.

## Enums

[Serializable](enum.Serializable.html "enum bevy::reflect::serde::Serializable")

A type-erased serializable value.

## Traits

[DeserializeWithRegistry](trait.DeserializeWithRegistry.html "trait bevy::reflect::serde::DeserializeWithRegistry")

Trait used to provide finer control when deserializing a reflected type with one of the reflection deserializers.

[ReflectDeserializerProcessor](trait.ReflectDeserializerProcessor.html "trait bevy::reflect::serde::ReflectDeserializerProcessor")

Allows overriding the default deserialization behavior of [`ReflectDeserializer`](struct.ReflectDeserializer.html "struct bevy::reflect::serde::ReflectDeserializer") and [`TypedReflectDeserializer`](struct.TypedReflectDeserializer.html "struct bevy::reflect::serde::TypedReflectDeserializer") for specific [`TypeRegistration`](../struct.TypeRegistration.html "struct bevy::reflect::TypeRegistration")s.

[ReflectSerializerProcessor](trait.ReflectSerializerProcessor.html "trait bevy::reflect::serde::ReflectSerializerProcessor")

Allows overriding the default serialization behavior of [`ReflectSerializer`](struct.ReflectSerializer.html "struct bevy::reflect::serde::ReflectSerializer") and [`TypedReflectSerializer`](struct.TypedReflectSerializer.html "struct bevy::reflect::serde::TypedReflectSerializer") for specific values.

[SerializeWithRegistry](trait.SerializeWithRegistry.html "trait bevy::reflect::serde::SerializeWithRegistry")

Trait used to provide finer control when serializing a reflected type with one of the reflection serializers.