[bevy](../../../index.html)::[remote](../../index.html)::[schemas](../index.html)::[json\_schema](index.html)

# Function export\_type 

[Source](https://docs.rs/bevy_remote/0.19.0/x86_64-unknown-linux-gnu/src/bevy_remote/schemas/json_schema.rs.html#49-53)

```rust
pub fn export_type(
    reg: &TypeRegistration,
    metadata: &SchemaTypesMetadata,
    components: &Components,
) -> (Cow<'static, str>, JsonSchemaBevyType)
```

Exports schema info for a given type