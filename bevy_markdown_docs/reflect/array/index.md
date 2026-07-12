[bevy](../../index.html)::[reflect](../index.html)

# Module array 

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/lib.rs.html#598)

Traits and types used to power [array-like](https://doc.rust-lang.org/book/ch03-02-data-types.html#the-array-type) operations via reflection.

## Structs

[ArrayInfo](struct.ArrayInfo.html "struct bevy::reflect::array::ArrayInfo")

A container for compile-time array info.

[ArrayIter](struct.ArrayIter.html "struct bevy::reflect::array::ArrayIter")

An iterator over an [`Array`](trait.Array.html "trait bevy::reflect::array::Array").

[DynamicArray](struct.DynamicArray.html "struct bevy::reflect::array::DynamicArray")

A fixed-size list of reflected values.

## Traits

[Array](trait.Array.html "trait bevy::reflect::array::Array")

A trait used to power [array-like](https://doc.rust-lang.org/book/ch03-02-data-types.html#the-array-type) operations via [reflection](../index.html "mod bevy::reflect").

## Functions

[array\_apply](fn.array_apply.html "fn bevy::reflect::array::array_apply")

Applies the reflected [array](trait.Array.html "trait bevy::reflect::array::Array") data to the given [array](trait.Array.html "trait bevy::reflect::array::Array").

[array\_debug](fn.array_debug.html "fn bevy::reflect::array::array_debug")

The default debug formatter for [`Array`](trait.Array.html "trait bevy::reflect::array::Array") types.

[array\_hash](fn.array_hash.html "fn bevy::reflect::array::array_hash")

Returns the `u64` hash of the given [array](trait.Array.html "trait bevy::reflect::array::Array").

[array\_partial\_cmp](fn.array_partial_cmp.html "fn bevy::reflect::array::array_partial_cmp")

Lexicographically compares two [arrays](trait.Array.html "trait bevy::reflect::array::Array") and returns their ordering.

[array\_partial\_eq](fn.array_partial_eq.html "fn bevy::reflect::array::array_partial_eq")

Compares two [arrays](trait.Array.html "trait bevy::reflect::array::Array") (one concrete and one reflected) to see if they are equal.

[array\_try\_apply](fn.array_try_apply.html "fn bevy::reflect::array::array_try_apply")

Tries to apply the reflected [array](trait.Array.html "trait bevy::reflect::array::Array") data to the given [array](trait.Array.html "trait bevy::reflect::array::Array") and returns a Result.