[bevy](../../index.html)::[reflect](../index.html)

# Module map 

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/lib.rs.html#607)

Traits and types used to power [map-like](https://doc.rust-lang.org/book/ch08-03-hash-maps.html) operations via reflection.

## Structs

[DynamicMap](struct.DynamicMap.html "struct bevy::reflect::map::DynamicMap")

An unordered mapping between reflected values.

[MapInfo](struct.MapInfo.html "struct bevy::reflect::map::MapInfo")

A container for compile-time map info.

## Traits

[Map](trait.Map.html "trait bevy::reflect::map::Map")

A trait used to power [map-like](https://doc.rust-lang.org/book/ch08-03-hash-maps.html) operations via [reflection](../index.html "mod bevy::reflect").

## Functions

[map\_apply](fn.map_apply.html "fn bevy::reflect::map::map_apply")

Applies the elements of reflected map `b` to the corresponding elements of map `a`.

[map\_debug](fn.map_debug.html "fn bevy::reflect::map::map_debug")

The default debug formatter for [`Map`](trait.Map.html "trait bevy::reflect::map::Map") types.

[map\_partial\_cmp](fn.map_partial_cmp.html "fn bevy::reflect::map::map_partial_cmp")

Lexicographically compares two [`Map`](trait.Map.html "trait bevy::reflect::map::Map") values according to their iteration order (suitable for ordered maps like `BTreeMap`).

[map\_partial\_eq](fn.map_partial_eq.html "fn bevy::reflect::map::map_partial_eq")

Compares a [`Map`](trait.Map.html "trait bevy::reflect::map::Map") with a [`PartialReflect`](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") value.

[map\_try\_apply](fn.map_try_apply.html "fn bevy::reflect::map::map_try_apply")

Tries to apply the elements of reflected map `b` to the corresponding elements of map `a` and returns a Result.