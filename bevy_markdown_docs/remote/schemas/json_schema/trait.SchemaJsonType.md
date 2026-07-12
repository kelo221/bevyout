[bevy](../../../index.html)::[remote](../../index.html)::[schemas](../index.html)::[json\_schema](index.html)

# Trait SchemaJsonType 

[Source](https://docs.rs/bevy_remote/0.19.0/x86_64-unknown-linux-gnu/src/bevy_remote/schemas/json_schema.rs.html#414)

```rust
pub trait SchemaJsonType {
    // Required method
    fn get_type_path(&self) -> &'static str;

    // Provided method
    fn map_json_type(&self) -> SchemaType { ... }
}
```

Helper trait for mapping bevy type path into json schema type

## Required Methods

[Source](https://docs.rs/bevy_remote/0.19.0/x86_64-unknown-linux-gnu/src/bevy_remote/schemas/json_schema.rs.html#416)

#### fn [get\_type\_path](#tymethod.get_type_path)(&self) -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

Bevy Reflect type path

## Provided Methods

[Source](https://docs.rs/bevy_remote/0.19.0/x86_64-unknown-linux-gnu/src/bevy_remote/schemas/json_schema.rs.html#419)

#### fn [map\_json\_type](#method.map_json_type)(&self) -> [SchemaType](enum.SchemaType.html "enum bevy::remote::schemas::json_schema::SchemaType")

JSON Schema type keyword from Bevy reflect type path into

## Dyn Compatibility

This trait **is** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_remote/0.19.0/x86_64-unknown-linux-gnu/src/bevy_remote/schemas/json_schema.rs.html#431)

### impl [SchemaJsonType](trait.SchemaJsonType.html "trait bevy::remote::schemas::json_schema::SchemaJsonType") for [OpaqueInfo](../../../reflect/struct.OpaqueInfo.html "struct bevy::reflect::OpaqueInfo")