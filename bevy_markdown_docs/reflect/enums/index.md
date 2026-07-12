[bevy](../../index.html)::[reflect](../index.html)

# Module enums 

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/lib.rs.html#649)

Traits and types used to power [enum-like](https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html) operations via reflection.

## Structs

[DynamicEnum](struct.DynamicEnum.html "struct bevy::reflect::enums::DynamicEnum")

A dynamic representation of an enum.

[EnumInfo](struct.EnumInfo.html "struct bevy::reflect::enums::EnumInfo")

A container for compile-time enum info, used by [`TypeInfo`](../enum.TypeInfo.html "enum bevy::reflect::TypeInfo").

[StructVariantInfo](struct.StructVariantInfo.html "struct bevy::reflect::enums::StructVariantInfo")

Type info for struct variants.

[TupleVariantInfo](struct.TupleVariantInfo.html "struct bevy::reflect::enums::TupleVariantInfo")

Type info for tuple variants.

[UnitVariantInfo](struct.UnitVariantInfo.html "struct bevy::reflect::enums::UnitVariantInfo")

Type info for unit variants.

[VariantFieldIter](struct.VariantFieldIter.html "struct bevy::reflect::enums::VariantFieldIter")

An iterator over the fields in the current enum variant.

## Enums

[DynamicVariant](enum.DynamicVariant.html "enum bevy::reflect::enums::DynamicVariant")

A dynamic representation of an enum variant.

[VariantField](enum.VariantField.html "enum bevy::reflect::enums::VariantField")

A field in the current enum variant.

[VariantInfo](enum.VariantInfo.html "enum bevy::reflect::enums::VariantInfo")

A container for compile-time enum variant info.

[VariantInfoError](enum.VariantInfoError.html "enum bevy::reflect::enums::VariantInfoError")

A [`VariantInfo`](enum.VariantInfo.html "enum bevy::reflect::enums::VariantInfo")\-specific error.

[VariantType](enum.VariantType.html "enum bevy::reflect::enums::VariantType")

Describes the form of an enum variant.

## Traits

[Enum](trait.Enum.html "trait bevy::reflect::enums::Enum")

A trait used to power [enum-like](https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html) operations via [reflection](../index.html "mod bevy::reflect").

## Functions

[enum\_debug](fn.enum_debug.html "fn bevy::reflect::enums::enum_debug")

The default debug formatter for [`Enum`](trait.Enum.html "trait bevy::reflect::enums::Enum") types.

[enum\_hash](fn.enum_hash.html "fn bevy::reflect::enums::enum_hash")

Returns the `u64` hash of the given [enum](trait.Enum.html "trait bevy::reflect::enums::Enum").

[enum\_partial\_cmp](fn.enum_partial_cmp.html "fn bevy::reflect::enums::enum_partial_cmp")

Compares two [`Enum`](trait.Enum.html "trait bevy::reflect::enums::Enum") values (by variant) and returns their ordering.

[enum\_partial\_eq](fn.enum_partial_eq.html "fn bevy::reflect::enums::enum_partial_eq")

Compares an [`Enum`](trait.Enum.html "trait bevy::reflect::enums::Enum") with a [`PartialReflect`](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") value.