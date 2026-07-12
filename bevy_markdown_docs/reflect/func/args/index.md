[bevy](../../../index.html)::[reflect](../../index.html)::[func](../index.html)

# Module args 

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/func/mod.rs.html#174)

Available on **crate feature `functions`** only.

Argument types and utilities for working with [`DynamicFunction`](../struct.DynamicFunction.html "struct bevy::reflect::func::DynamicFunction") and [`DynamicFunctionMut`](../struct.DynamicFunctionMut.html "struct bevy::reflect::func::DynamicFunctionMut").

## Structs

[Arg](struct.Arg.html "struct bevy::reflect::func::args::Arg")

Represents an argument that can be passed to a [`DynamicFunction`](../struct.DynamicFunction.html "struct bevy::reflect::func::DynamicFunction") or [`DynamicFunctionMut`](../struct.DynamicFunctionMut.html "struct bevy::reflect::func::DynamicFunctionMut").

[ArgCount](struct.ArgCount.html "struct bevy::reflect::func::args::ArgCount")

A container for zero or more argument counts for a function.

[ArgCountIter](struct.ArgCountIter.html "struct bevy::reflect::func::args::ArgCountIter")

An iterator for the argument counts in an [`ArgCount`](struct.ArgCount.html "struct bevy::reflect::func::args::ArgCount").

[ArgCountOutOfBoundsError](struct.ArgCountOutOfBoundsError.html "struct bevy::reflect::func::args::ArgCountOutOfBoundsError")

The given argument count is out of bounds.

[ArgInfo](struct.ArgInfo.html "struct bevy::reflect::func::args::ArgInfo")

Type information for an [`Arg`](struct.Arg.html "struct bevy::reflect::func::args::Arg") used in a [`DynamicFunction`](../struct.DynamicFunction.html "struct bevy::reflect::func::DynamicFunction") or [`DynamicFunctionMut`](../struct.DynamicFunctionMut.html "struct bevy::reflect::func::DynamicFunctionMut").

[ArgList](struct.ArgList.html "struct bevy::reflect::func::args::ArgList")

A list of arguments that can be passed to a [`DynamicFunction`](../struct.DynamicFunction.html "struct bevy::reflect::func::DynamicFunction") or [`DynamicFunctionMut`](../struct.DynamicFunctionMut.html "struct bevy::reflect::func::DynamicFunctionMut").

## Enums

[ArgError](enum.ArgError.html "enum bevy::reflect::func::args::ArgError")

An error that occurs when converting an [argument](struct.Arg.html "struct bevy::reflect::func::args::Arg").

[ArgId](enum.ArgId.html "enum bevy::reflect::func::args::ArgId")

A representation of an argument.

[ArgValue](enum.ArgValue.html "enum bevy::reflect::func::args::ArgValue")

Represents an argument that can be passed to a [`DynamicFunction`](../struct.DynamicFunction.html "struct bevy::reflect::func::DynamicFunction") or [`DynamicFunctionMut`](../struct.DynamicFunctionMut.html "struct bevy::reflect::func::DynamicFunctionMut").

[Ownership](enum.Ownership.html "enum bevy::reflect::func::args::Ownership")

The ownership of a type.

## Traits

[FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg")

A trait for types that can be created from an [`Arg`](struct.Arg.html "struct bevy::reflect::func::args::Arg").

[GetOwnership](trait.GetOwnership.html "trait bevy::reflect::func::args::GetOwnership")

A trait for getting the ownership of a type.