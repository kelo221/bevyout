[bevy](../../../index.html)::[asset](../../index.html)::[uuid](../index.html)

# Module serde 

[Source](https://docs.rs/uuid/1.23.2/x86_64-unknown-linux-gnu/src/uuid/lib.rs.html#994)

Available on **crate feature `serde`** only.

Adapters for alternative `serde` formats.

This module contains adapters you can use with [`#[serde(with)]`](https://serde.rs/field-attrs.html#with) to change the way a [`Uuid`](../struct.Uuid.html) is serialized and deserialized.

## Modules

[braced](braced/index.html "mod bevy::asset::uuid::serde::braced")

Serialize a [`Uuid`](../../struct.Uuid.html) as \[`uuid::fmt::Braced`\].

[compact](compact/index.html "mod bevy::asset::uuid::serde::compact")

Serialize a [`Uuid`](../../struct.Uuid.html) as a `[u8; 16]`.

[hyphenated](hyphenated/index.html "mod bevy::asset::uuid::serde::hyphenated")

Serialize a [`Uuid`](../../struct.Uuid.html) as \[`uuid::fmt::Hyphenated`\].

[simple](simple/index.html "mod bevy::asset::uuid::serde::simple")

Serialize a [`Uuid`](../../struct.Uuid.html) as \[`uuid::fmt::Simple`\].

[urn](urn/index.html "mod bevy::asset::uuid::serde::urn")

Serialize a [`Uuid`](../../struct.Uuid.html) as \[`uuid::fmt::Urn`\].