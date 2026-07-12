[bevy](../../../index.html)::[remote](../../index.html)::[schemas](../index.html)

# Module json\_schema 

[Source](https://docs.rs/bevy_remote/0.19.0/x86_64-unknown-linux-gnu/src/bevy_remote/schemas/mod.rs.html#13)

Module with JSON Schema type for Bevy Registry Types. It tries to follow this standard: [https://json-schema.org/specification](https://json-schema.org/specification)

## Structs

[ComponentMetadata](struct.ComponentMetadata.html "struct bevy::remote::schemas::json_schema::ComponentMetadata")

Component-specific metadata. Related to [`ComponentInfo`](../../../ecs/component/struct.ComponentInfo.html "struct bevy::ecs::component::ComponentInfo").

[JsonSchemaBevyType](struct.JsonSchemaBevyType.html "struct bevy::remote::schemas::json_schema::JsonSchemaBevyType")

JSON Schema type for Bevy Registry Types. It tries to follow this standard: [https://json-schema.org/specification](https://json-schema.org/specification)

## Enums

[RelationshipKind](enum.RelationshipKind.html "enum bevy::remote::schemas::json_schema::RelationshipKind")

Kind of relationship.

[SchemaKind](enum.SchemaKind.html "enum bevy::remote::schemas::json_schema::SchemaKind")

Kind of json schema, maps [`TypeInfo`](../../../reflect/enum.TypeInfo.html "enum bevy::reflect::TypeInfo") type

[SchemaType](enum.SchemaType.html "enum bevy::remote::schemas::json_schema::SchemaType")

Type of json schema

[StorageKind](enum.StorageKind.html "enum bevy::remote::schemas::json_schema::StorageKind")

The storage used for a specific component type.

## Traits

[SchemaJsonType](trait.SchemaJsonType.html "trait bevy::remote::schemas::json_schema::SchemaJsonType")

Helper trait for mapping bevy type path into json schema type

[TypeRegistrySchemaReader](trait.TypeRegistrySchemaReader.html "trait bevy::remote::schemas::json_schema::TypeRegistrySchemaReader")

Helper trait for converting `TypeRegistration` to `JsonSchemaBevyType`

## Functions

[export\_type](fn.export_type.html "fn bevy::remote::schemas::json_schema::export_type")

Exports schema info for a given type