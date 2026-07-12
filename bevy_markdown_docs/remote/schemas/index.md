[bevy](../../index.html)::[remote](../index.html)

# Module schemas 

[Source](https://docs.rs/bevy_remote/0.19.0/x86_64-unknown-linux-gnu/src/bevy_remote/lib.rs.html#562)

Module with schemas used for various BRP endpoints

## Modules

[json\_schema](json_schema/index.html "mod bevy::remote::schemas::json_schema")

Module with JSON Schema type for Bevy Registry Types. It tries to follow this standard: [https://json-schema.org/specification](https://json-schema.org/specification)

[open\_rpc](open_rpc/index.html "mod bevy::remote::schemas::open_rpc")

Module with trimmed down `OpenRPC` document structs. It tries to follow this standard: [https://spec.open-rpc.org](https://spec.open-rpc.org)

## Structs

[SchemaTypesMetadata](struct.SchemaTypesMetadata.html "struct bevy::remote::schemas::SchemaTypesMetadata")

Holds mapping of reflect [type data](../../reflect/trait.TypeData.html "trait bevy::reflect::TypeData") to strings, later on used in Bevy Json Schema.