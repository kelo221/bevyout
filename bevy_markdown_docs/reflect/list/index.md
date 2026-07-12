[bevy](../../index.html)::[reflect](../index.html)

# Module list 

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/lib.rs.html#606)

Traits and types used to power [list-like](https://doc.rust-lang.org/book/ch08-01-vectors.html) operations via reflection.

## Structs

[DynamicList](struct.DynamicList.html "struct bevy::reflect::list::DynamicList")

A list of reflected values.

[ListInfo](struct.ListInfo.html "struct bevy::reflect::list::ListInfo")

A container for compile-time list info.

[ListIter](struct.ListIter.html "struct bevy::reflect::list::ListIter")

An iterator over an [`List`](trait.List.html "trait bevy::reflect::list::List").

## Traits

[List](trait.List.html "trait bevy::reflect::list::List")

A trait used to power [list-like](https://doc.rust-lang.org/book/ch08-01-vectors.html) operations via [reflection](../index.html "mod bevy::reflect").

## Functions

[list\_apply](fn.list_apply.html "fn bevy::reflect::list::list_apply")

Applies the elements of `b` to the corresponding elements of `a`.

[list\_debug](fn.list_debug.html "fn bevy::reflect::list::list_debug")

The default debug formatter for [`List`](trait.List.html "trait bevy::reflect::list::List") types.

[list\_hash](fn.list_hash.html "fn bevy::reflect::list::list_hash")

Returns the `u64` hash of the given [list](trait.List.html "trait bevy::reflect::list::List").

[list\_partial\_cmp](fn.list_partial_cmp.html "fn bevy::reflect::list::list_partial_cmp")

Lexicographically compares two [List](trait.List.html "trait bevy::reflect::list::List") values and returns their ordering.

[list\_partial\_eq](fn.list_partial_eq.html "fn bevy::reflect::list::list_partial_eq")

Compares a [`List`](trait.List.html "trait bevy::reflect::list::List") with a [`Reflect`](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") value.

[list\_try\_apply](fn.list_try_apply.html "fn bevy::reflect::list::list_try_apply")

Tries to apply the elements of `b` to the corresponding elements of `a` and returns a Result.