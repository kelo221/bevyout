[bevy](../../index.html)::[reflect](../index.html)

# Module structs 

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/lib.rs.html#613)

Traits and types used to power [struct-like](https://doc.rust-lang.org/book/ch05-01-defining-structs.html) operations via reflection.

## Structs

[DynamicStruct](struct.DynamicStruct.html "struct bevy::reflect::structs::DynamicStruct")

A struct type which allows fields to be added at runtime.

[FieldIter](struct.FieldIter.html "struct bevy::reflect::structs::FieldIter")

An iterator over the names and fields of a struct.

[StructInfo](struct.StructInfo.html "struct bevy::reflect::structs::StructInfo")

A container for compile-time named struct info.

## Traits

[GetField](trait.GetField.html "trait bevy::reflect::structs::GetField")

A convenience trait which combines fetching and downcasting of struct fields.

[Struct](trait.Struct.html "trait bevy::reflect::structs::Struct")

A trait used to power [struct-like](https://doc.rust-lang.org/book/ch05-01-defining-structs.html) operations via [reflection](../index.html "mod bevy::reflect").

## Functions

[struct\_debug](fn.struct_debug.html "fn bevy::reflect::structs::struct_debug")

The default debug formatter for [`Struct`](../../prelude/trait.Struct.html "trait bevy::prelude::Struct") types.

[struct\_partial\_cmp](fn.struct_partial_cmp.html "fn bevy::reflect::structs::struct_partial_cmp")

Lexicographically compares two [`Struct`](../../prelude/trait.Struct.html "trait bevy::prelude::Struct") values and returns their ordering.

[struct\_partial\_eq](fn.struct_partial_eq.html "fn bevy::reflect::structs::struct_partial_eq")

Compares a [`Struct`](../../prelude/trait.Struct.html "trait bevy::prelude::Struct") with a [`PartialReflect`](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") value.