[bevy](../../index.html)::[reflect](../index.html)

# Module prelude 

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/lib.rs.html#660)

The reflect prelude.

This includes the most common types in this crate, re-exported for your convenience.

## Structs

[ReflectAdd](struct.ReflectAdd.html "struct bevy::reflect::prelude::ReflectAdd")

A struct used to perform addition on reflected values.

[ReflectAddAssign](struct.ReflectAddAssign.html "struct bevy::reflect::prelude::ReflectAddAssign")

A struct used to perform addition assignment on reflected values.

[ReflectDefault](struct.ReflectDefault.html "struct bevy::reflect::prelude::ReflectDefault")

A struct used to provide the default value of a type.

[ReflectDeserialize](struct.ReflectDeserialize.html "struct bevy::reflect::prelude::ReflectDeserialize")

A struct used to deserialize reflected instances of a type.

[ReflectDiv](struct.ReflectDiv.html "struct bevy::reflect::prelude::ReflectDiv")

A struct used to perform division on reflected values.

[ReflectDivAssign](struct.ReflectDivAssign.html "struct bevy::reflect::prelude::ReflectDivAssign")

A struct used to perform division assignment on reflected values.

[ReflectFromReflect](struct.ReflectFromReflect.html "struct bevy::reflect::prelude::ReflectFromReflect")

Type data that represents the [`FromReflect`](../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") trait and allows it to be used dynamically.

[ReflectMul](struct.ReflectMul.html "struct bevy::reflect::prelude::ReflectMul")

A struct used to perform multiplication on reflected values.

[ReflectMulAssign](struct.ReflectMulAssign.html "struct bevy::reflect::prelude::ReflectMulAssign")

A struct used to perform multiplication assignment on reflected values.

[ReflectRem](struct.ReflectRem.html "struct bevy::reflect::prelude::ReflectRem")

A struct used to perform remainder on reflected values.

[ReflectRemAssign](struct.ReflectRemAssign.html "struct bevy::reflect::prelude::ReflectRemAssign")

A struct used to perform remainder assignment on reflected values.

[ReflectSerialize](struct.ReflectSerialize.html "struct bevy::reflect::prelude::ReflectSerialize")

A struct used to serialize reflected instances of a type.

[ReflectSub](struct.ReflectSub.html "struct bevy::reflect::prelude::ReflectSub")

A struct used to perform subtraction on reflected values.

[ReflectSubAssign](struct.ReflectSubAssign.html "struct bevy::reflect::prelude::ReflectSubAssign")

A struct used to perform subtraction assignment on reflected values.

## Traits

[FromReflect](trait.FromReflect.html "trait bevy::reflect::prelude::FromReflect")

A trait that enables types to be dynamically constructed from reflected data.

[Function](trait.Function.html "trait bevy::reflect::prelude::Function")

A trait used to power [function-like](../func/index.html "mod bevy::reflect::func") operations via [reflection](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect").

[GetField](trait.GetField.html "trait bevy::reflect::prelude::GetField")

A convenience trait which combines fetching and downcasting of struct fields.

[GetPath](trait.GetPath.html "trait bevy::reflect::prelude::GetPath")

A trait which allows nested [`Reflect`](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") values to be retrieved with path strings.

[GetTupleStructField](trait.GetTupleStructField.html "trait bevy::reflect::prelude::GetTupleStructField")

A convenience trait which combines fetching and downcasting of tuple struct fields.

[IntoFunction](trait.IntoFunction.html "trait bevy::reflect::prelude::IntoFunction")

A trait for types that can be converted into a [`DynamicFunction`](../func/struct.DynamicFunction.html "struct bevy::reflect::func::DynamicFunction").

[IntoFunctionMut](trait.IntoFunctionMut.html "trait bevy::reflect::prelude::IntoFunctionMut")

A trait for types that can be converted into a [`DynamicFunctionMut`](../func/struct.DynamicFunctionMut.html "struct bevy::reflect::func::DynamicFunctionMut").

[PartialReflect](trait.PartialReflect.html "trait bevy::reflect::prelude::PartialReflect")

The foundational trait of [`bevy_reflect`](../index.html "mod bevy::reflect"), used for accessing and modifying data dynamically.

[Reflect](trait.Reflect.html "trait bevy::reflect::prelude::Reflect")

A core trait of [`bevy_reflect`](../index.html "mod bevy::reflect"), used for downcasting to concrete types.

[ReflectPath](trait.ReflectPath.html "trait bevy::reflect::prelude::ReflectPath")

Something that can be interpreted as a reflection path in [`GetPath`](../../prelude/trait.GetPath.html "trait bevy::prelude::GetPath").

[Struct](trait.Struct.html "trait bevy::reflect::prelude::Struct")

A trait used to power [struct-like](https://doc.rust-lang.org/book/ch05-01-defining-structs.html) operations via [reflection](../index.html "mod bevy::reflect").

[TupleStruct](trait.TupleStruct.html "trait bevy::reflect::prelude::TupleStruct")

A trait used to power [tuple struct-like](https://doc.rust-lang.org/book/ch05-01-defining-structs.html#using-tuple-structs-without-named-fields-to-create-different-types) operations via [reflection](../index.html "mod bevy::reflect").

[TypePath](trait.TypePath.html "trait bevy::reflect::prelude::TypePath")

A static accessor to type paths and names.

## Attribute Macros

[reflect\_trait](attr.reflect_trait.html "attr bevy::reflect::prelude::reflect_trait")

A macro that automatically generates type data for traits, which their implementors can then register.

## Derive Macros

[FromReflect](derive.FromReflect.html "derive bevy::reflect::prelude::FromReflect")

Derives the `FromReflect` trait.

[Reflect](derive.Reflect.html "derive bevy::reflect::prelude::Reflect")

The main derive macro used by `bevy_reflect` for deriving its `Reflect` trait.

[TypePath](derive.TypePath.html "derive bevy::reflect::prelude::TypePath")

Derives the `TypePath` trait, providing a stable alternative to \[`std::any::type_name`\].