[bevy](../../index.html)::[reflect](../index.html)

# Module utility 

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/lib.rs.html#655)

Helpers for working with Bevy reflection.

## Structs

[GenericTypeCell](struct.GenericTypeCell.html "struct bevy::reflect::utility::GenericTypeCell")

A container for [`TypedProperty`](trait.TypedProperty.html "trait bevy::reflect::utility::TypedProperty") over generic types, allowing instances to be stored statically.

[NonGenericTypeCell](struct.NonGenericTypeCell.html "struct bevy::reflect::utility::NonGenericTypeCell")

A container for [`TypeInfo`](../enum.TypeInfo.html "enum bevy::reflect::TypeInfo") over non-generic types, allowing instances to be stored statically.

[TypePathComponent](struct.TypePathComponent.html "struct bevy::reflect::utility::TypePathComponent")

Used to store a [`String`](../../prelude/struct.String.html "struct bevy::prelude::String") in a [`GenericTypePathCell`](type.GenericTypePathCell.html "type bevy::reflect::utility::GenericTypePathCell") as part of a [`TypePath`](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") implementation.

## Traits

[TypedProperty](trait.TypedProperty.html "trait bevy::reflect::utility::TypedProperty")

A type that can be stored in a ([`Non`](struct.NonGenericTypeCell.html "struct bevy::reflect::utility::NonGenericTypeCell"))[`GenericTypeCell`](struct.GenericTypeCell.html "struct bevy::reflect::utility::GenericTypeCell").

## Functions

[reflect\_hasher](fn.reflect_hasher.html "fn bevy::reflect::utility::reflect_hasher")

Deterministic fixed state hasher to be used by implementors of [`Reflect::reflect_hash`](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect").

## Type Aliases

[GenericTypeInfoCell](type.GenericTypeInfoCell.html "type bevy::reflect::utility::GenericTypeInfoCell")

See [`GenericTypeCell`](struct.GenericTypeCell.html "struct bevy::reflect::utility::GenericTypeCell").

[GenericTypePathCell](type.GenericTypePathCell.html "type bevy::reflect::utility::GenericTypePathCell")

See [`GenericTypeCell`](struct.GenericTypeCell.html "struct bevy::reflect::utility::GenericTypeCell").

[NonGenericTypeInfoCell](type.NonGenericTypeInfoCell.html "type bevy::reflect::utility::NonGenericTypeInfoCell")

See [`NonGenericTypeCell`](struct.NonGenericTypeCell.html "struct bevy::reflect::utility::NonGenericTypeCell").