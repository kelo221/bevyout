[bevy](../../index.html)::[reflect](../index.html)

# Module set 

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/lib.rs.html#612)

A trait used to power [set-like](https://doc.rust-lang.org/stable/std/collections/struct.HashSet.html) operations via reflection.

## Structs

[DynamicSet](struct.DynamicSet.html "struct bevy::reflect::set::DynamicSet")

An unordered set of reflected values.

[SetInfo](struct.SetInfo.html "struct bevy::reflect::set::SetInfo")

A container for compile-time set info.

## Traits

[Set](trait.Set.html "trait bevy::reflect::set::Set")

A trait used to power [set-like](https://doc.rust-lang.org/stable/std/collections/struct.HashSet.html) operations via [reflection](../index.html "mod bevy::reflect").

## Functions

[set\_apply](fn.set_apply.html "fn bevy::reflect::set::set_apply")

Applies the elements of reflected set `b` to the corresponding elements of set `a`.

[set\_debug](fn.set_debug.html "fn bevy::reflect::set::set_debug")

The default debug formatter for [`Set`](trait.Set.html "trait bevy::reflect::set::Set") types.

[set\_partial\_eq](fn.set_partial_eq.html "fn bevy::reflect::set::set_partial_eq")

Compares a [`Set`](trait.Set.html "trait bevy::reflect::set::Set") with a [`PartialReflect`](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") value.

[set\_try\_apply](fn.set_try_apply.html "fn bevy::reflect::set::set_try_apply")

Tries to apply the elements of reflected set `b` to the corresponding elements of set `a` and returns a Result.