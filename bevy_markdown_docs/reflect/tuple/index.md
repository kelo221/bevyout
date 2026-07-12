[bevy](../../index.html)::[reflect](../index.html)

# Module tuple 

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/lib.rs.html#614)

Traits and types used to power [tuple-like](https://doc.rust-lang.org/book/ch03-02-data-types.html#the-tuple-type) operations via reflection.

## Structs

[DynamicTuple](struct.DynamicTuple.html "struct bevy::reflect::tuple::DynamicTuple")

A tuple which allows fields to be added at runtime.

[TupleFieldIter](struct.TupleFieldIter.html "struct bevy::reflect::tuple::TupleFieldIter")

An iterator over the field values of a tuple.

[TupleInfo](struct.TupleInfo.html "struct bevy::reflect::tuple::TupleInfo")

A container for compile-time tuple info.

## Traits

[GetTupleField](trait.GetTupleField.html "trait bevy::reflect::tuple::GetTupleField")

A convenience trait which combines fetching and downcasting of tuple fields.

[Tuple](trait.Tuple.html "trait bevy::reflect::tuple::Tuple")

A trait used to power [tuple-like](https://doc.rust-lang.org/book/ch03-02-data-types.html#the-tuple-type) operations via [reflection](../index.html "mod bevy::reflect").

## Functions

[tuple\_apply](fn.tuple_apply.html "fn bevy::reflect::tuple::tuple_apply")

Applies the elements of `b` to the corresponding elements of `a`.

[tuple\_debug](fn.tuple_debug.html "fn bevy::reflect::tuple::tuple_debug")

The default debug formatter for [`Tuple`](trait.Tuple.html "trait bevy::reflect::tuple::Tuple") types.

[tuple\_partial\_cmp](fn.tuple_partial_cmp.html "fn bevy::reflect::tuple::tuple_partial_cmp")

Lexicographically compares two [`Tuple`](trait.Tuple.html "trait bevy::reflect::tuple::Tuple") values and returns their ordering.

[tuple\_partial\_eq](fn.tuple_partial_eq.html "fn bevy::reflect::tuple::tuple_partial_eq")

Compares a [`Tuple`](trait.Tuple.html "trait bevy::reflect::tuple::Tuple") with a [`PartialReflect`](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") value.

[tuple\_try\_apply](fn.tuple_try_apply.html "fn bevy::reflect::tuple::tuple_try_apply")

Tries to apply the elements of `b` to the corresponding elements of `a` and returns a Result.