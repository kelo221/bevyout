[bevy](../../index.html)::[reflect](../index.html)

# Crate erased\_serde 

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/lib.rs.html#1-144)

  [![github](https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github)](https://github.com/dtolnay/erased-serde) [![crates-io](https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust)](https://crates.io/crates/erased-serde) [![docs-rs](https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs)](https://docs.rs/erased-serde)

  

This crate provides type-erased versions of Serde’s `Serialize`, `Serializer` and `Deserializer` traits that can be used as [trait objects](https://doc.rust-lang.org/book/trait-objects.html).

The usual Serde `Serialize`, `Serializer` and `Deserializer` traits cannot be used as trait objects like `&dyn Serialize` or boxed trait objects like `Box<dyn Serialize>` because of Rust’s [“object safety” rules](http://huonw.github.io/blog/2015/01/object-safety/). In particular, all three traits contain generic methods which cannot be made into a trait object.

This library should be considered a low-level building block for interacting with Serde APIs in an object-safe way. Most use cases will require higher level functionality such as provided by [`typetag`](https://github.com/dtolnay/typetag) which uses this crate internally.

**The traits in this crate work seamlessly with any existing Serde `Serialize` and `Deserialize` type and any existing Serde `Serializer` and `Deserializer` format.**

### Serialization

```rust
use erased_serde::{Serialize, Serializer};
use std::collections::BTreeMap as Map;
use std::io;

fn main() {
    // Construct some serializers.
    let json = &mut serde_json::Serializer::new(io::stdout());
    let cbor = &mut serde_cbor::Serializer::new(serde_cbor::ser::IoWrite::new(io::stdout()));

    // The values in this map are boxed trait objects. Ordinarily this would not
    // be possible with serde::Serializer because of object safety, but type
    // erasure makes it possible with erased_serde::Serializer.
    let mut formats: Map<&str, Box<dyn Serializer>> = Map::new();
    formats.insert("json", Box::new(<dyn Serializer>::erase(json)));
    formats.insert("cbor", Box::new(<dyn Serializer>::erase(cbor)));

    // These are boxed trait objects as well. Same thing here - type erasure
    // makes this possible.
    let mut values: Map<&str, Box<dyn Serialize>> = Map::new();
    values.insert("vec", Box::new(vec!["a", "b"]));
    values.insert("int", Box::new(65536));

    // Pick a Serializer out of the formats map.
    let format = formats.get_mut("json").unwrap();

    // Pick a Serialize out of the values map.
    let value = values.get("vec").unwrap();

    // This line prints `["a","b"]` to stdout.
    value.erased_serialize(format).unwrap();
}
```

### Deserialization

```rust
use erased_serde::Deserializer;
use std::collections::BTreeMap as Map;

fn main() {
    static JSON: &'static [u8] = br#"{"A": 65, "B": 66}"#;
    static CBOR: &'static [u8] = &[162, 97, 65, 24, 65, 97, 66, 24, 66];

    // Construct some deserializers.
    let json = &mut serde_json::Deserializer::from_slice(JSON);
    let cbor = &mut serde_cbor::Deserializer::from_slice(CBOR);

    // The values in this map are boxed trait objects, which is not possible
    // with the normal serde::Deserializer because of object safety.
    let mut formats: Map<&str, Box<dyn Deserializer>> = Map::new();
    formats.insert("json", Box::new(<dyn Deserializer>::erase(json)));
    formats.insert("cbor", Box::new(<dyn Deserializer>::erase(cbor)));

    // Pick a Deserializer out of the formats map.
    let format = formats.get_mut("json").unwrap();

    let data: Map<String, usize> = erased_serde::deserialize(format).unwrap();

    println!("{}", data["A"] + data["B"]);
}
```

## Macros

[serialize\_trait\_object](macro.serialize_trait_object.html "macro bevy::reflect::erased_serde::serialize_trait_object")

Implement `serde::Serialize` for a trait object that has `erased_serde::Serialize` as a supertrait.

## Structs

[Error](struct.Error.html "struct bevy::reflect::erased_serde::Error")

Error when a `Serializer` or `Deserializer` trait object fails.

## Traits

[Deserializer](trait.Deserializer.html "trait bevy::reflect::erased_serde::Deserializer")

An object-safe equivalent of Serde’s `Deserializer` trait.

[Serialize](trait.Serialize.html "trait bevy::reflect::erased_serde::Serialize")

An object-safe equivalent of Serde’s `Serialize` trait.

[Serializer](trait.Serializer.html "trait bevy::reflect::erased_serde::Serializer")

An object-safe equivalent of Serde’s `Serializer` trait.

## Functions

[deserialize](fn.deserialize.html "fn bevy::reflect::erased_serde::deserialize")

Deserialize a value of type `T` from the given trait object.

[serialize](fn.serialize.html "fn bevy::reflect::erased_serde::serialize")

Serialize the given type-erased serializable value.

## Type Aliases

[Result](type.Result.html "type bevy::reflect::erased_serde::Result")

Result type alias where the error is `erased_serde::Error`.