[bevy](../../index.html)::[reflect](../index.html)

# Module tuple\_struct 

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/lib.rs.html#615)

Traits and types used to power [tuple-struct-like](https://doc.rust-lang.org/book/ch05-01-defining-structs.html#using-tuple-structs-without-named-fields-to-create-different-types) operations via reflection.

## Structs

[DynamicTupleStruct](struct.DynamicTupleStruct.html "struct bevy::reflect::tuple_struct::DynamicTupleStruct")

A tuple struct which allows fields to be added at runtime.

[TupleStructFieldIter](struct.TupleStructFieldIter.html "struct bevy::reflect::tuple_struct::TupleStructFieldIter")

An iterator over the field values of a tuple struct.

[TupleStructInfo](struct.TupleStructInfo.html "struct bevy::reflect::tuple_struct::TupleStructInfo")

A container for compile-time tuple struct info.

## Traits

[GetTupleStructField](trait.GetTupleStructField.html "trait bevy::reflect::tuple_struct::GetTupleStructField")

A convenience trait which combines fetching and downcasting of tuple struct fields.

[TupleStruct](trait.TupleStruct.html "trait bevy::reflect::tuple_struct::TupleStruct")

A trait used to power [tuple struct-like](https://doc.rust-lang.org/book/ch05-01-defining-structs.html#using-tuple-structs-without-named-fields-to-create-different-types) operations via [reflection](../index.html "mod bevy::reflect").

## Functions

[tuple\_struct\_debug](fn.tuple_struct_debug.html "fn bevy::reflect::tuple_struct::tuple_struct_debug")

The default debug formatter for [`TupleStruct`](../../prelude/trait.TupleStruct.html "trait bevy::prelude::TupleStruct") types.

[tuple\_struct\_partial\_cmp](fn.tuple_struct_partial_cmp.html "fn bevy::reflect::tuple_struct::tuple_struct_partial_cmp")

Lexicographically compares two [`TupleStruct`](../../prelude/trait.TupleStruct.html "trait bevy::prelude::TupleStruct") values and returns their ordering.

[tuple\_struct\_partial\_eq](fn.tuple_struct_partial_eq.html "fn bevy::reflect::tuple_struct::tuple_struct_partial_eq")

Compares a [`TupleStruct`](../../prelude/trait.TupleStruct.html "trait bevy::prelude::TupleStruct") with a [`PartialReflect`](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") value.