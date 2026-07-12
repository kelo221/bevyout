[bevy](../../../index.html)::[remote](../../index.html)::[schemas](../index.html)::[json\_schema](index.html)

# Trait TypeRegistrySchemaReader 

[Source](https://docs.rs/bevy_remote/0.19.0/x86_64-unknown-linux-gnu/src/bevy_remote/schemas/json_schema.rs.html#18)

```rust
pub trait TypeRegistrySchemaReader {
    // Required method
    fn export_type_json_schema_for_id(
        &self,
        extra_info: &SchemaTypesMetadata,
        type_id: TypeId,
        components: &Components,
    ) -> Option<JsonSchemaBevyType>;

    // Provided method
    fn export_type_json_schema<T>(
        &self,
        extra_info: &SchemaTypesMetadata,
        components: &Components,
    ) -> Option<JsonSchemaBevyType>
       where T: GetTypeRegistration + 'static { ... }
}
```

Helper trait for converting `TypeRegistration` to `JsonSchemaBevyType`

## Required Methods

[Source](https://docs.rs/bevy_remote/0.19.0/x86_64-unknown-linux-gnu/src/bevy_remote/schemas/json_schema.rs.html#28-33)

#### fn [export\_type\_json\_schema\_for\_id](#tymethod.export_type_json_schema_for_id)( &self, extra\_info: &[SchemaTypesMetadata](../struct.SchemaTypesMetadata.html "struct bevy::remote::schemas::SchemaTypesMetadata"), type\_id: [TypeId](https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html "struct core::any::TypeId"), components: &[Components](../../../ecs/component/struct.Components.html "struct bevy::ecs::component::Components"), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[JsonSchemaBevyType](struct.JsonSchemaBevyType.html "struct bevy::remote::schemas::json_schema::JsonSchemaBevyType")\>

Export type JSON Schema.

## Provided Methods

[Source](https://docs.rs/bevy_remote/0.19.0/x86_64-unknown-linux-gnu/src/bevy_remote/schemas/json_schema.rs.html#20-24)

#### fn [export\_type\_json\_schema](#method.export_type_json_schema)<T>( &self, extra\_info: &[SchemaTypesMetadata](../struct.SchemaTypesMetadata.html "struct bevy::remote::schemas::SchemaTypesMetadata"), components: &[Components](../../../ecs/component/struct.Components.html "struct bevy::ecs::component::Components"), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[JsonSchemaBevyType](struct.JsonSchemaBevyType.html "struct bevy::remote::schemas::json_schema::JsonSchemaBevyType")\>

where T: [GetTypeRegistration](../../../reflect/trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration") + 'static,

Export type JSON Schema.

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_remote/0.19.0/x86_64-unknown-linux-gnu/src/bevy_remote/schemas/json_schema.rs.html#36)

### impl [TypeRegistrySchemaReader](trait.TypeRegistrySchemaReader.html "trait bevy::remote::schemas::json_schema::TypeRegistrySchemaReader") for [TypeRegistry](../../../reflect/struct.TypeRegistry.html "struct bevy::reflect::TypeRegistry")