[bevy](../../index.html)::[reflect](../index.html)::[serde](index.html)

# Trait ReflectDeserializerProcessor 

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/serde/de/processor.rs.html#141)

```rust
pub trait ReflectDeserializerProcessor {
    // Required method
    fn try_deserialize<'de, D>(
        &mut self,
        registration: &TypeRegistration,
        registry: &TypeRegistry,
        deserializer: D,
    ) -> Result<Result<Box<dyn PartialReflect>, D>, <D as Deserializer<'de>>::Error>
       where D: Deserializer<'de>;
}
```

Allows overriding the default deserialization behavior of [`ReflectDeserializer`](struct.ReflectDeserializer.html "struct bevy::reflect::serde::ReflectDeserializer") and [`TypedReflectDeserializer`](struct.TypedReflectDeserializer.html "struct bevy::reflect::serde::TypedReflectDeserializer") for specific [`TypeRegistration`](../struct.TypeRegistration.html "struct bevy::reflect::TypeRegistration")s.

When deserializing a reflected value, you may want to override the default behavior and use your own logic for deserialization. This logic may also be context-dependent, and only apply for a single use of your [`ReflectDeserializer`](struct.ReflectDeserializer.html "struct bevy::reflect::serde::ReflectDeserializer"). To achieve this, you can create a processor and pass it in to your deserializer.

Whenever the deserializer attempts to deserialize a value, it will first call [`try_deserialize`](trait.ReflectDeserializerProcessor.html#tymethod.try_deserialize "method bevy::reflect::serde::ReflectDeserializerProcessor::try_deserialize") on your processor, which may take ownership of the deserializer and give back a [`Box<dyn PartialReflect>`](../../prelude/struct.Box.html "struct bevy::prelude::Box"), or return ownership of the deserializer back, and continue with the default logic.

The serialization equivalent of this is [`ReflectSerializerProcessor`](trait.ReflectSerializerProcessor.html "trait bevy::reflect::serde::ReflectSerializerProcessor").

## Compared to [`DeserializeWithRegistry`](trait.DeserializeWithRegistry.html "trait bevy::reflect::serde::DeserializeWithRegistry")

[`DeserializeWithRegistry`](trait.DeserializeWithRegistry.html "trait bevy::reflect::serde::DeserializeWithRegistry") allows you to define how your type will be deserialized by a [`TypedReflectDeserializer`](struct.TypedReflectDeserializer.html "struct bevy::reflect::serde::TypedReflectDeserializer"), given the extra context of the [`TypeRegistry`](../struct.TypeRegistry.html "struct bevy::reflect::TypeRegistry"). If your type can be deserialized entirely from that, then you should prefer implementing that trait instead of using a processor.

However, you may need more context-dependent data which is only present in the scope where you create the [`TypedReflectDeserializer`](struct.TypedReflectDeserializer.html "struct bevy::reflect::serde::TypedReflectDeserializer"). For example, in an asset loader, the `&mut LoadContext` you get is only valid from within the `load` function. This is where a processor is useful, as the processor can capture local variables.

A [`ReflectDeserializerProcessor`](trait.ReflectDeserializerProcessor.html "trait bevy::reflect::serde::ReflectDeserializerProcessor") always takes priority over a [`DeserializeWithRegistry`](trait.DeserializeWithRegistry.html "trait bevy::reflect::serde::DeserializeWithRegistry") implementation, so this is also useful for overriding deserialization behavior if you need to do something custom.

## Examples

Deserializing a reflected value in an asset loader, and replacing asset handles with a loaded equivalent:

```rust
#[derive(Debug, Clone, Reflect)]
struct MyAsset {
    name: String,
    mesh: Handle<Mesh>,
}

fn load(
    asset_bytes: &[u8],
    type_registry: &TypeRegistry,
    load_context: &mut LoadContext,
) -> Result<MyAsset, AssetError> {
    struct HandleProcessor<'a> {
        load_context: &'a mut LoadContext,
    }

    impl ReflectDeserializerProcessor for HandleProcessor<'_> {
        fn try_deserialize<'de, D>(
            &mut self,
            registration: &TypeRegistration,
            _registry: &TypeRegistry,
            deserializer: D,
        ) -> Result<Result<Box<dyn PartialReflect>, D>, D::Error>
        where
            D: Deserializer<'de>,
        {
            let Some(reflect_handle) = registration.data::<ReflectHandle>() else {
                // we don't want to deserialize this - give the deserializer back
                return Ok(Err(deserializer));
            };

            let asset_type_id = reflect_handle.asset_type_id();
            let asset_path = deserializer.deserialize_str(AssetPathVisitor)?;

            let handle: Handle<LoadedUntypedAsset> = self.load_context
                .load()
                .with_asset_type_id(asset_type_id)
                .untyped()
                .load_asset(asset_path);
            Ok(Box::new(handle))
        }
    }

    let mut ron_deserializer = ron::Deserializer::from_bytes(asset_bytes)?;
    let mut processor = HandleProcessor { load_context };
    let reflect_deserializer =
        ReflectDeserializer::with_processor(type_registry, &mut processor);
    let asset = reflect_deserializer.deserialize(&mut ron_deserializer)?;
}
```

## Required Methods

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/serde/de/processor.rs.html#195-202)

#### fn [try\_deserialize](#tymethod.try_deserialize)<'de, D>( &mut self, registration: &[TypeRegistration](../struct.TypeRegistration.html "struct bevy::reflect::TypeRegistration"), registry: &[TypeRegistry](../struct.TypeRegistry.html "struct bevy::reflect::TypeRegistry"), deserializer: D, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>, D>, <D as [Deserializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html "trait serde_core::de::Deserializer")<'de>>::[Error](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#associatedtype.Error "type serde_core::de::Deserializer::Error")\>

where D: [Deserializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html "trait serde_core::de::Deserializer")<'de>,

Attempts to deserialize the value which a [`TypedReflectDeserializer`](struct.TypedReflectDeserializer.html "struct bevy::reflect::serde::TypedReflectDeserializer") is currently looking at, and knows the type of.

If you’ve read the `registration` and want to override the default deserialization, return `Ok(Ok(value))` with the boxed reflected value that you want to assign this value to. The type inside the box must be the same one as the `registration` is for, otherwise future reflection operations (such as using [`FromReflect`](../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") to convert the resulting [`Box<dyn PartialReflect>`](../../prelude/struct.Box.html "struct bevy::prelude::Box") into a concrete type) will fail.

If you don’t want to override the deserialization, return ownership of the deserializer back via `Ok(Err(deserializer))`.

Note that, if you do want to return a value, you _must_ read from the deserializer passed to this function (you are free to ignore the result though). Otherwise, the deserializer will be in an inconsistent state, and future value parsing will fail.

##### Examples

Correct way to return a constant value (not using any output from the deserializer):

```rust
use serde::de::IgnoredAny;

struct ConstantI32Processor;

impl ReflectDeserializerProcessor for ConstantI32Processor {
    fn try_deserialize<'de, D>(
        &mut self,
        registration: &TypeRegistration,
        _registry: &TypeRegistry,
        deserializer: D,
    ) -> Result<Result<Box<dyn PartialReflect>, D>, D::Error>
    where
        D: serde::Deserializer<'de>
    {
        if registration.type_id() == TypeId::of::<i32>() {
            _ = deserializer.deserialize_ignored_any(IgnoredAny);
            Ok(Ok(Box::new(42_i32)))
        } else {
            Ok(Err(deserializer))
        }
    }
}
```

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementations on Foreign Types

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/serde/de/processor.rs.html#205)

### impl [ReflectDeserializerProcessor](trait.ReflectDeserializerProcessor.html "trait bevy::reflect::serde::ReflectDeserializerProcessor") for [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/serde/de/processor.rs.html#206-213)

#### fn [try\_deserialize](#tymethod.try_deserialize)<'de, D>( &mut self, \_registration: &[TypeRegistration](../struct.TypeRegistration.html "struct bevy::reflect::TypeRegistration"), \_registry: &[TypeRegistry](../struct.TypeRegistry.html "struct bevy::reflect::TypeRegistry"), deserializer: D, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>, D>, <D as [Deserializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html "trait serde_core::de::Deserializer")<'de>>::[Error](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html#associatedtype.Error "type serde_core::de::Deserializer::Error")\>

where D: [Deserializer](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserializer.html "trait serde_core::de::Deserializer")<'de>,

## Implementors

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/reflect.rs.html#450)

### impl [ReflectDeserializerProcessor](trait.ReflectDeserializerProcessor.html "trait bevy::reflect::serde::ReflectDeserializerProcessor") for [HandleDeserializeProcessor](../../asset/struct.HandleDeserializeProcessor.html "struct bevy::asset::HandleDeserializeProcessor")<'\_>