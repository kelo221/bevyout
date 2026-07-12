[bevy](../../index.html)::[reflect](../index.html)::[serde](index.html)

# Trait ReflectSerializerProcessor 

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/serde/ser/processor.rs.html#132)

```rust
pub trait ReflectSerializerProcessor {
    // Required method
    fn try_serialize<S>(
        &self,
        value: &(dyn PartialReflect + 'static),
        registry: &TypeRegistry,
        serializer: S,
    ) -> Result<Result<<S as Serializer>::Ok, S>, <S as Serializer>::Error>
       where S: Serializer;
}
```

Allows overriding the default serialization behavior of [`ReflectSerializer`](struct.ReflectSerializer.html "struct bevy::reflect::serde::ReflectSerializer") and [`TypedReflectSerializer`](struct.TypedReflectSerializer.html "struct bevy::reflect::serde::TypedReflectSerializer") for specific values.

When serializing a reflected value, you may want to override the default behavior and use your own logic for serialization. This logic may also be context-dependent, and only apply for a single use of your [`ReflectSerializer`](struct.ReflectSerializer.html "struct bevy::reflect::serde::ReflectSerializer"). To achieve this, you can create a processor and pass it into your serializer.

Whenever the serializer attempts to serialize a value, it will first call [`try_serialize`](trait.ReflectSerializerProcessor.html#tymethod.try_serialize "method bevy::reflect::serde::ReflectSerializerProcessor::try_serialize") on your processor, which may take ownership of the serializer and write into the serializer (successfully or not), or return ownership of the serializer back, and continue with the default logic.

The deserialization equivalent of this is [`ReflectDeserializerProcessor`](trait.ReflectDeserializerProcessor.html "trait bevy::reflect::serde::ReflectDeserializerProcessor").

## Compared to [`SerializeWithRegistry`](trait.SerializeWithRegistry.html "trait bevy::reflect::serde::SerializeWithRegistry")

[`SerializeWithRegistry`](trait.SerializeWithRegistry.html "trait bevy::reflect::serde::SerializeWithRegistry") allows you to define how your type will be serialized by a [`TypedReflectSerializer`](struct.TypedReflectSerializer.html "struct bevy::reflect::serde::TypedReflectSerializer"), given the extra context of the [`TypeRegistry`](../struct.TypeRegistry.html "struct bevy::reflect::TypeRegistry"). If your type can be serialized entirely using that, then you should prefer implementing that trait instead of using a processor.

However, you may need more context-dependent data which is only present in the scope where you create the [`TypedReflectSerializer`](struct.TypedReflectSerializer.html "struct bevy::reflect::serde::TypedReflectSerializer"). For example, if you need to use a reference to a value while serializing, then there is no way to do this with [`SerializeWithRegistry`](trait.SerializeWithRegistry.html "trait bevy::reflect::serde::SerializeWithRegistry") as you can’t pass that reference into anywhere. This is where a processor is useful, as the processor can capture local variables.

A [`ReflectSerializerProcessor`](trait.ReflectSerializerProcessor.html "trait bevy::reflect::serde::ReflectSerializerProcessor") always takes priority over a [`SerializeWithRegistry`](trait.SerializeWithRegistry.html "trait bevy::reflect::serde::SerializeWithRegistry") implementation, so this is also useful for overriding serialization behavior if you need to do something custom.

## Examples

Serializing a reflected value when saving an asset to disk, and replacing asset handles with the handle path (if it has one):

```rust
#[derive(Debug, Clone, Reflect)]
struct MyAsset {
    name: String,
    mesh: Handle<Mesh>,
}

struct HandleProcessor;

impl ReflectSerializerProcessor for HandleProcessor {
    fn try_serialize<S>(
        &self,
        value: &dyn PartialReflect,
        registry: &TypeRegistry,
        serializer: S,
    ) -> Result<Result<S::Ok, S>, S::Error>
    where
        S: serde::Serializer,
    {
        let Some(value) = value.try_as_reflect() else {
            // we don't have any info on this type; do the default serialization logic
            return Ok(Err(serializer));
        };
        let type_id = value.reflect_type_info().type_id();
        let Some(reflect_handle) = registry.get_type_data::<ReflectHandle>(type_id) else {
            // this isn't a `Handle<T>`
            return Ok(Err(serializer));
        };

        let untyped_handle = reflect_handle
            .downcast_handle_untyped(value.as_any())
            .unwrap();
        if let Some(path) = untyped_handle.path() {
            Ok(Ok(serializer.serialize_str(path)?))
        } else {
            Ok(Ok(serializer.serialize_unit()?))
        }
    }
}

fn save(type_registry: &TypeRegistry, asset: &MyAsset) -> Result<String, AssetError> {
    let mut asset_string = String::new();

    let processor = HandleProcessor;
    let serializer = ReflectSerializer::with_processor(asset, type_registry, &processor);
    let mut ron_serializer = ron::Serializer::new(&mut asset_string, None)?;

    serializer.serialize(&mut ron_serializer)?;
    Ok(asset_string)
}
```

## Required Methods

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/serde/ser/processor.rs.html#174-181)

#### fn [try\_serialize](#tymethod.try_serialize)<S>( &self, value: &(dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static), registry: &[TypeRegistry](../struct.TypeRegistry.html "struct bevy::reflect::TypeRegistry"), serializer: S, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<S as [Serializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html "trait serde_core::ser::Serializer")\>::[Ok](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html#associatedtype.Ok "type serde_core::ser::Serializer::Ok"), S>, <S as [Serializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html "trait serde_core::ser::Serializer")\>::[Error](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html#associatedtype.Error "type serde_core::ser::Serializer::Error")\>

where S: [Serializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html "trait serde_core::ser::Serializer"),

Attempts to serialize the value which a [`TypedReflectSerializer`](struct.TypedReflectSerializer.html "struct bevy::reflect::serde::TypedReflectSerializer") is currently looking at.

If you want to override the default serialization, return `Ok(Ok(value))` with an `Ok` output from the serializer.

If you don’t want to override the serialization, return ownership of the serializer back via `Ok(Err(serializer))`.

You can use the type registry to read info about the type you’re serializing, or just try to downcast the value directly:

```rust
struct I32AsStringProcessor;

impl ReflectSerializerProcessor for I32AsStringProcessor {
    fn try_serialize<S>(
        &self,
        value: &dyn PartialReflect,
        registry: &TypeRegistry,
        serializer: S,
    ) -> Result<Result<S::Ok, S>, S::Error>
    where
        S: serde::Serializer
    {
        if let Some(value) = value.try_downcast_ref::<i32>() {
            let value_as_string = format!("{value:?}");
            Ok(Ok(serializer.serialize_str(&value_as_string)?))
        } else {
            // Not an `i32`, just do the default serialization
            Ok(Err(serializer))
        }
    }
}
```

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementations on Foreign Types

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/serde/ser/processor.rs.html#184)

### impl [ReflectSerializerProcessor](trait.ReflectSerializerProcessor.html "trait bevy::reflect::serde::ReflectSerializerProcessor") for [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/serde/ser/processor.rs.html#185-192)

#### fn [try\_serialize](#tymethod.try_serialize)<S>( &self, \_value: &(dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static), \_registry: &[TypeRegistry](../struct.TypeRegistry.html "struct bevy::reflect::TypeRegistry"), serializer: S, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<S as [Serializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html "trait serde_core::ser::Serializer")\>::[Ok](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html#associatedtype.Ok "type serde_core::ser::Serializer::Ok"), S>, <S as [Serializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html "trait serde_core::ser::Serializer")\>::[Error](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html#associatedtype.Error "type serde_core::ser::Serializer::Error")\>

where S: [Serializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serializer.html "trait serde_core::ser::Serializer"),

## Implementors

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/reflect.rs.html#297)

### impl [ReflectSerializerProcessor](trait.ReflectSerializerProcessor.html "trait bevy::reflect::serde::ReflectSerializerProcessor") for [HandleSerializeProcessor](../../asset/struct.HandleSerializeProcessor.html "struct bevy::asset::HandleSerializeProcessor")