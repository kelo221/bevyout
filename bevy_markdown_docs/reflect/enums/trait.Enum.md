[bevy](../../index.html)::[reflect](../index.html)::[enums](index.html)

# Trait Enum 

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/enums/enum_trait.rs.html#97)

```rust
pub trait Enum: PartialReflect {
    // Required methods
    fn field(&self, name: &str) -> Option<&(dyn PartialReflect + 'static)>;
    fn field_at(&self, index: usize) -> Option<&(dyn PartialReflect + 'static)>;
    fn field_mut(
        &mut self,
        name: &str,
    ) -> Option<&mut (dyn PartialReflect + 'static)>;
    fn field_at_mut(
        &mut self,
        index: usize,
    ) -> Option<&mut (dyn PartialReflect + 'static)>;
    fn index_of(&self, name: &str) -> Option<usize>;
    fn name_at(&self, index: usize) -> Option<&str>;
    fn iter_fields(&self) -> VariantFieldIter<'_> ⓘ;
    fn field_len(&self) -> usize;
    fn variant_name(&self) -> &str;
    fn variant_index(&self) -> usize;
    fn variant_type(&self) -> VariantType;

    // Provided methods
    fn to_dynamic_enum(&self) -> DynamicEnum { ... }
    fn is_variant(&self, variant_type: VariantType) -> bool { ... }
    fn variant_path(&self) -> String { ... }
    fn get_represented_enum_info(&self) -> Option<&'static EnumInfo> { ... }
}
```

A trait used to power [enum-like](https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html) operations via [reflection](../index.html "mod bevy::reflect").

This allows enums to be processed and modified dynamically at runtime without necessarily knowing the actual type. Enums are much more complex than their struct counterparts. As a result, users will need to be mindful of conventions, considerations, and complications when working with this trait.

## Variants

An enum is a set of choices called _variants_. An instance of an enum can only exist as one of these choices at any given time. Consider Rust’s [`Option<T>`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option"). It’s an enum with two variants: [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") and [`Some`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.Some "variant core::option::Option::Some"). If you’re `None`, you can’t be `Some` and vice versa.

> ⚠️ **This is very important:** The [`Enum`](trait.Enum.html "trait bevy::reflect::enums::Enum") trait represents an enum _as one of its variants_. It does not represent the entire enum since that’s not true to how enums work.

Variants come in a few [flavors](enum.VariantType.html "enum bevy::reflect::enums::VariantType"):

| Variant Type | Syntax |
| --- | --- |
| Unit | `MyEnum::Foo` |
| Tuple | `MyEnum::Foo( i32, i32 )` |
| Struct | `MyEnum::Foo{ value: String }` |

As you can see, a unit variant contains no fields, while tuple and struct variants can contain one or more fields. The fields in a tuple variant is defined by their _order_ within the variant. Index `0` represents the first field in the variant and so on. Fields in struct variants (excluding tuple structs), on the other hand, are represented by a _name_.

## Implementation

> 💡 This trait can be automatically implemented using [`#[derive(Reflect)]`](../../prelude/derive.Reflect.html "derive bevy::prelude::Reflect") on an enum definition.

Despite the fact that enums can represent multiple states, traits only exist in one state and must be applied to the entire enum rather than a particular variant. Because of this limitation, the [`Enum`](trait.Enum.html "trait bevy::reflect::enums::Enum") trait must not only _represent_ any of the three variant types, but also define the _methods_ for all three as well.

What does this mean? It means that even though a unit variant contains no fields, a representation of that variant using the [`Enum`](trait.Enum.html "trait bevy::reflect::enums::Enum") trait will still contain methods for accessing fields! Again, this is to account for _all three_ variant types.

We recommend using the built-in [`#[derive(Reflect)]`](../../prelude/derive.Reflect.html "derive bevy::prelude::Reflect") macro to automatically handle all the implementation details for you. However, if you _must_ implement this trait manually, there are a few things to keep in mind…

### Field Order

While tuple variants identify their fields by the order in which they are defined, struct variants identify fields by their name. However, both should allow access to fields by their defined order.

The reason all fields, regardless of variant type, need to be accessible by their order is due to field iteration. We need a way to iterate through each field in a variant, and the easiest way of achieving that is through the use of field order.

The derive macro adds proper struct variant handling for [`Enum::index_of`](trait.Enum.html#tymethod.index_of "method bevy::reflect::enums::Enum::index_of"), [`Enum::name_at`](trait.Enum.html#tymethod.name_at "method bevy::reflect::enums::Enum::name_at") and [`Enum::field_at[_mut]`](trait.Enum.html#tymethod.field_at "method bevy::reflect::enums::Enum::field_at") methods. The first two methods are **required** for all struct variant types. By convention, implementors should also handle the last method as well, but this is not a strict requirement.

### Field Names

Implementors may choose to handle [`Enum::index_of`](trait.Enum.html#tymethod.index_of "method bevy::reflect::enums::Enum::index_of"), [`Enum::name_at`](trait.Enum.html#tymethod.name_at "method bevy::reflect::enums::Enum::name_at"), and [`Enum::field[_mut]`](trait.Enum.html#tymethod.field "method bevy::reflect::enums::Enum::field") for tuple variants by considering stringified `usize`s to be valid names (such as `"3"`). This isn’t wrong to do, but the convention set by the derive macro is that it isn’t supported. It’s preferred that these strings be converted to their proper `usize` representations and the [`Enum::field_at[_mut]`](trait.Enum.html#tymethod.field_at "method bevy::reflect::enums::Enum::field_at") methods be used instead.

## Required Methods

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/enums/enum_trait.rs.html#101)

#### fn [field](#tymethod.field)(&self, name: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&(dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)>

Returns a reference to the value of the field (in the current variant) with the given name.

For non-[`VariantType::Struct`](enum.VariantType.html#variant.Struct "variant bevy::reflect::enums::VariantType::Struct") variants, this should return `None`.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/enums/enum_trait.rs.html#103)

#### fn [field\_at](#tymethod.field_at)(&self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&(dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)>

Returns a reference to the value of the field (in the current variant) at the given index.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/enums/enum_trait.rs.html#107)

#### fn [field\_mut](#tymethod.field_mut)( &mut self, name: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&mut (dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)>

Returns a mutable reference to the value of the field (in the current variant) with the given name.

For non-[`VariantType::Struct`](enum.VariantType.html#variant.Struct "variant bevy::reflect::enums::VariantType::Struct") variants, this should return `None`.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/enums/enum_trait.rs.html#109)

#### fn [field\_at\_mut](#tymethod.field_at_mut)( &mut self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&mut (dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)>

Returns a mutable reference to the value of the field (in the current variant) at the given index.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/enums/enum_trait.rs.html#113)

#### fn [index\_of](#tymethod.index_of)(&self, name: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>

Returns the index of the field (in the current variant) with the given name.

For non-[`VariantType::Struct`](enum.VariantType.html#variant.Struct "variant bevy::reflect::enums::VariantType::Struct") variants, this should return `None`.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/enums/enum_trait.rs.html#117)

#### fn [name\_at](#tymethod.name_at)(&self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

Returns the name of the field (in the current variant) with the given index.

For non-[`VariantType::Struct`](enum.VariantType.html#variant.Struct "variant bevy::reflect::enums::VariantType::Struct") variants, this should return `None`.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/enums/enum_trait.rs.html#119)

#### fn [iter\_fields](#tymethod.iter_fields)(&self) -> [VariantFieldIter](struct.VariantFieldIter.html "struct bevy::reflect::enums::VariantFieldIter")<'\_> [ⓘ](#)

Returns an iterator over the values of the current variant’s fields.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/enums/enum_trait.rs.html#121)

#### fn [field\_len](#tymethod.field_len)(&self) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

Returns the number of fields in the current variant.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/enums/enum_trait.rs.html#123)

#### fn [variant\_name](#tymethod.variant_name)(&self) -> &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

The name of the current variant.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/enums/enum_trait.rs.html#125)

#### fn [variant\_index](#tymethod.variant_index)(&self) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

The index of the current variant.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/enums/enum_trait.rs.html#127)

#### fn [variant\_type](#tymethod.variant_type)(&self) -> [VariantType](enum.VariantType.html "enum bevy::reflect::enums::VariantType")

The type of the current variant.

## Provided Methods

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/enums/enum_trait.rs.html#129)

#### fn [to\_dynamic\_enum](#method.to_dynamic_enum)(&self) -> [DynamicEnum](struct.DynamicEnum.html "struct bevy::reflect::enums::DynamicEnum")

Creates a new [`DynamicEnum`](struct.DynamicEnum.html "struct bevy::reflect::enums::DynamicEnum") from this enum.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/enums/enum_trait.rs.html#133)

#### fn [is\_variant](#method.is_variant)(&self, variant\_type: [VariantType](enum.VariantType.html "enum bevy::reflect::enums::VariantType")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Returns true if the current variant’s type matches the given one.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/enums/enum_trait.rs.html#137)

#### fn [variant\_path](#method.variant_path)(&self) -> [String](../../prelude/struct.String.html "struct bevy::prelude::String")

Returns the full path to the current variant.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/enums/enum_trait.rs.html#144)

#### fn [get\_represented\_enum\_info](#method.get_represented_enum_info)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [EnumInfo](struct.EnumInfo.html "struct bevy::reflect::enums::EnumInfo")\>

Will return `None` if [`TypeInfo`](../enum.TypeInfo.html "enum bevy::reflect::TypeInfo") is not available.

## Dyn Compatibility

This trait **is** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementations on Foreign Types

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/result.rs.html#8-14)

### impl<T, E> [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, E>

where [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, E>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), T: [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [FromReflect](../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + RegisterForReflection, E: [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [FromReflect](../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/result.rs.html#8-14)

#### fn [field](#tymethod.field)(&self, \_\_name\_param: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&(dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/result.rs.html#8-14)

#### fn [field\_at](#tymethod.field_at)( &self, \_\_index\_param: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&(dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/result.rs.html#8-14)

#### fn [field\_mut](#tymethod.field_mut)( &mut self, \_\_name\_param: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&mut (dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/result.rs.html#8-14)

#### fn [field\_at\_mut](#tymethod.field_at_mut)( &mut self, \_\_index\_param: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&mut (dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/result.rs.html#8-14)

#### fn [index\_of](#tymethod.index_of)(&self, \_\_name\_param: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/result.rs.html#8-14)

#### fn [name\_at](#tymethod.name_at)(&self, \_\_index\_param: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/result.rs.html#8-14)

#### fn [iter\_fields](#tymethod.iter_fields)(&self) -> [VariantFieldIter](struct.VariantFieldIter.html "struct bevy::reflect::enums::VariantFieldIter")<'\_> [ⓘ](#)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/result.rs.html#8-14)

#### fn [field\_len](#tymethod.field_len)(&self) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/result.rs.html#8-14)

#### fn [variant\_name](#tymethod.variant_name)(&self) -> &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/result.rs.html#8-14)

#### fn [variant\_index](#tymethod.variant_index)(&self) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/result.rs.html#8-14)

#### fn [variant\_type](#tymethod.variant_type)(&self) -> [VariantType](enum.VariantType.html "enum bevy::reflect::enums::VariantType")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/result.rs.html#8-14)

#### fn [to\_dynamic\_enum](#method.to_dynamic_enum)(&self) -> [DynamicEnum](struct.DynamicEnum.html "struct bevy::reflect::enums::DynamicEnum")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/option.rs.html#8-14)

### impl<T> [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<T>

where [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<T>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), T: [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [FromReflect](../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/option.rs.html#8-14)

#### fn [field](#tymethod.field)(&self, \_\_name\_param: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&(dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/option.rs.html#8-14)

#### fn [field\_at](#tymethod.field_at)( &self, \_\_index\_param: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&(dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/option.rs.html#8-14)

#### fn [field\_mut](#tymethod.field_mut)( &mut self, \_\_name\_param: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&mut (dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/option.rs.html#8-14)

#### fn [field\_at\_mut](#tymethod.field_at_mut)( &mut self, \_\_index\_param: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&mut (dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/option.rs.html#8-14)

#### fn [index\_of](#tymethod.index_of)(&self, \_\_name\_param: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/option.rs.html#8-14)

#### fn [name\_at](#tymethod.name_at)(&self, \_\_index\_param: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/option.rs.html#8-14)

#### fn [iter\_fields](#tymethod.iter_fields)(&self) -> [VariantFieldIter](struct.VariantFieldIter.html "struct bevy::reflect::enums::VariantFieldIter")<'\_> [ⓘ](#)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/option.rs.html#8-14)

#### fn [field\_len](#tymethod.field_len)(&self) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/option.rs.html#8-14)

#### fn [variant\_name](#tymethod.variant_name)(&self) -> &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/option.rs.html#8-14)

#### fn [variant\_index](#tymethod.variant_index)(&self) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/option.rs.html#8-14)

#### fn [variant\_type](#tymethod.variant_type)(&self) -> [VariantType](enum.VariantType.html "enum bevy::reflect::enums::VariantType")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/option.rs.html#8-14)

#### fn [to\_dynamic\_enum](#method.to_dynamic_enum)(&self) -> [DynamicEnum](struct.DynamicEnum.html "struct bevy::reflect::enums::DynamicEnum")

## Implementors

[Source](https://docs.rs/bevy_a11y/0.19.0/x86_64-unknown-linux-gnu/src/bevy_a11y/lib.rs.html#251)

### impl [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [AccessibilitySystems](../../a11y/enum.AccessibilitySystems.html "enum bevy::a11y::AccessibilitySystems")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1055)

### impl [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [AlignContent](../../prelude/enum.AlignContent.html "enum bevy::prelude::AlignContent")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#895)

### impl [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [AlignItems](../../prelude/enum.AlignItems.html "enum bevy::prelude::AlignItems")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#975)

### impl [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [AlignSelf](../../prelude/enum.AlignSelf.html "enum bevy::prelude::AlignSelf")

[Source](https://docs.rs/bevy_material/0.19.0/x86_64-unknown-linux-gnu/src/bevy_material/alpha.rs.html#7)

### impl [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [AlphaMode](../../prelude/enum.AlphaMode.html "enum bevy::prelude::AlphaMode")

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/mesh2d/material.rs.html#245)

### impl [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [AlphaMode2d](../../sprite_render/enum.AlphaMode2d.html "enum bevy::sprite_render::AlphaMode2d")

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/graph.rs.html#211)

### impl [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [AnimationNodeType](../../prelude/enum.AnimationNodeType.html "enum bevy::prelude::AnimationNodeType")

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/app.rs.html#1565)

### impl [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [AppExit](../../prelude/enum.AppExit.html "enum bevy::prelude::AppExit")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/event.rs.html#453)

### impl [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [AppLifecycle](../../window/enum.AppLifecycle.html "enum bevy::window::AppLifecycle")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/atmosphere/mod.rs.html#414)

### impl [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [AtmosphereMode](../../pbr/enum.AtmosphereMode.html "enum bevy::pbr::AtmosphereMode")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/mesh_picking/ray_cast/mod.rs.html#95)

### impl [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [Backfaces](../../picking/mesh_picking/ray_cast/enum.Backfaces.html "enum bevy::picking::mesh_picking::ray_cast::Backfaces")

[Source](https://docs.rs/bevy_post_process/0.19.0/x86_64-unknown-linux-gnu/src/bevy_post_process/bloom/settings.rs.html#216)

### impl [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [BloomCompositeMode](../../post_process/bloom/enum.BloomCompositeMode.html "enum bevy::post_process::bloom::BloomCompositeMode")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1181)

### impl [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [BoxSizing](../../prelude/enum.BoxSizing.html "enum bevy::prelude::BoxSizing")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/lib.rs.html#172)

### impl [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [ButtonState](../../input/enum.ButtonState.html "enum bevy::input::ButtonState")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/button.rs.html#34)

### impl [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [ButtonVariant](../../feathers/controls/enum.ButtonVariant.html "enum bevy::feathers::controls::ButtonVariant")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/components.rs.html#58)

### impl [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [Camera3dDepthLoadOp](../../camera/enum.Camera3dDepthLoadOp.html "enum bevy::camera::Camera3dDepthLoadOp")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/camera.rs.html#860)

### impl [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [CameraOutputMode](../../camera/enum.CameraOutputMode.html "enum bevy::camera::CameraOutputMode")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim3/capsule.rs.html#7)

### impl [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [CapsuleUvProfile](../../mesh/enum.CapsuleUvProfile.html "enum bevy::mesh::CapsuleUvProfile")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim2.rs.html#106)

### impl [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [CircularMeshUvMode](../../mesh/enum.CircularMeshUvMode.html "enum bevy::mesh::CircularMeshUvMode")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/clear_color.rs.html#11)

### impl [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [ClearColorConfig](../../prelude/enum.ClearColorConfig.html "enum bevy::prelude::ClearColorConfig")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/cluster/mod.rs.html#105)

### impl [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [ClusterConfig](../../light/cluster/enum.ClusterConfig.html "enum bevy::light::cluster::ClusterConfig")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/cluster/mod.rs.html#82)

### impl [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [ClusterFarZMode](../../light/cluster/enum.ClusterFarZMode.html "enum bevy::light::cluster::ClusterFarZMode")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#48)

### impl [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [Color](../../prelude/enum.Color.html "enum bevy::prelude::Color")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/color_slider.rs.html#47)

### impl [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [ColorChannel](../../feathers/controls/enum.ColorChannel.html "enum bevy::feathers::controls::ColorChannel")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/compass.rs.html#132)

### impl [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [CompassOctant](../../math/enum.CompassOctant.html "enum bevy::math::CompassOctant")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/compass.rs.html#25)

### impl [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [CompassQuadrant](../../math/enum.CompassQuadrant.html "enum bevy::math::CompassQuadrant")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/window.rs.html#1295)

### impl [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [CompositeAlphaMode](../../window/enum.CompositeAlphaMode.html "enum bevy::window::CompositeAlphaMode")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/components.rs.html#92)

### impl [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [CompositingSpace](../../prelude/enum.CompositingSpace.html "enum bevy::prelude::CompositingSpace")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim3/cone.rs.html#7)

### impl [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [ConeAnchor](../../mesh/enum.ConeAnchor.html "enum bevy::mesh::ConeAnchor")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/scrollbar.rs.html#27)

### impl [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [ControlOrientation](../../ui_widgets/enum.ControlOrientation.html "enum bevy::ui_widgets::ControlOrientation")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/primitives.rs.html#408)

### impl [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [CubemapLayout](../../camera/primitives/enum.CubemapLayout.html "enum bevy::camera::primitives::CubemapLayout")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/window.rs.html#1076)

### impl [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [CursorGrabMode](../../window/enum.CursorGrabMode.html "enum bevy::window::CursorGrabMode")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/cursor/mod.rs.html#24)

### impl [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [CursorIcon](../../window/enum.CursorIcon.html "enum bevy::window::CursorIcon")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/cursor/custom_cursor.rs.html#71)

### impl [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [CustomCursor](../../window/enum.CustomCursor.html "enum bevy::window::CustomCursor")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim3/cylinder.rs.html#7)

### impl [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [CylinderAnchor](../../mesh/enum.CylinderAnchor.html "enum bevy::mesh::CylinderAnchor")

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/tonemapping/mod.rs.html#379)

### impl [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [DebandDither](../../core_pipeline/tonemapping/enum.DebandDither.html "enum bevy::core_pipeline::tonemapping::DebandDither")

[Source](https://docs.rs/bevy_post_process/0.19.0/x86_64-unknown-linux-gnu/src/bevy_post_process/dof/mod.rs.html#119)

### impl [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [DepthOfFieldMode](../../post_process/dof/enum.DepthOfFieldMode.html "enum bevy::post_process::dof::DepthOfFieldMode")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1147)

### impl [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [Display](../../prelude/enum.Display.html "enum bevy::prelude::Display")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/enums/dynamic_enum.rs.html#214)

### impl [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [DynamicEnum](struct.DynamicEnum.html "struct bevy::reflect::enums::DynamicEnum")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/easing.rs.html#431)

### impl [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [EaseFunction](../../prelude/enum.EaseFunction.html "enum bevy::prelude::EaseFunction")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/cursor.rs.html#32)

### impl [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [EntityCursor](../../feathers/cursor/enum.EntityCursor.html "enum bevy::feathers::cursor::EntityCursor")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#471-500)

### impl [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [EulerRot](../../prelude/enum.EulerRot.html "enum bevy::prelude::EulerRot")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/color_plane.rs.html#47)

### impl [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [FeathersColorPlane](../../feathers/controls/enum.FeathersColorPlane.html "enum bevy::feathers::controls::FeathersColorPlane")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/event.rs.html#376)

### impl [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [FileDragAndDrop](../../prelude/enum.FileDragAndDrop.html "enum bevy::prelude::FileDragAndDrop")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1206)

### impl [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [FlexDirection](../../prelude/enum.FlexDirection.html "enum bevy::prelude::FlexDirection")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1478)

### impl [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [FlexWrap](../../prelude/enum.FlexWrap.html "enum bevy::prelude::FlexWrap")

[Source](https://docs.rs/bevy_input_focus/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input_focus/gained_and_lost.rs.html#15)

### impl [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [FocusCause](../../input_focus/enum.FocusCause.html "enum bevy::input_focus::FocusCause")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/focus.rs.html#101)

### impl [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [FocusPolicy](../../ui/enum.FocusPolicy.html "enum bevy::ui::FocusPolicy")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/fog.rs.html#100)

### impl [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [FogFalloff](../../prelude/enum.FogFalloff.html "enum bevy::prelude::FogFalloff")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#1199)

### impl [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [FontHinting](../../prelude/enum.FontHinting.html "enum bevy::prelude::FontHinting")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#486)

### impl [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [FontSize](../../prelude/enum.FontSize.html "enum bevy::prelude::FontSize")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#1179)

### impl [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [FontSmoothing](../../prelude/enum.FontSmoothing.html "enum bevy::prelude::FontSmoothing")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#267)

### impl [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [FontSource](../../prelude/enum.FontSource.html "enum bevy::prelude::FontSource")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#704)

### impl [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [FontStyle](../../prelude/enum.FontStyle.html "enum bevy::prelude::FontStyle")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/touch.rs.html#73)

### impl [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [ForceTouch](../../input/touch/enum.ForceTouch.html "enum bevy::input::touch::ForceTouch")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/gamepad.rs.html#664)

### impl [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [GamepadAxis](../../prelude/enum.GamepadAxis.html "enum bevy::prelude::GamepadAxis")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/gamepad.rs.html#572)

### impl [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [GamepadButton](../../prelude/enum.GamepadButton.html "enum bevy::prelude::GamepadButton")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/gamepad.rs.html#1554)

### impl [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [GamepadConnection](../../input/gamepad/enum.GamepadConnection.html "enum bevy::input::gamepad::GamepadConnection")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/gamepad.rs.html#38)

### impl [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [GamepadEvent](../../input/gamepad/enum.GamepadEvent.html "enum bevy::input::gamepad::GamepadEvent")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/gamepad.rs.html#710)

### impl [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [GamepadInput](../../input/gamepad/enum.GamepadInput.html "enum bevy::input::gamepad::GamepadInput")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/gamepad.rs.html#1778)

### impl [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [GamepadRumbleRequest](../../input/gamepad/enum.GamepadRumbleRequest.html "enum bevy::input::gamepad::GamepadRumbleRequest")

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/config.rs.html#19)

### impl [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [GizmoLineJoint](../../prelude/enum.GizmoLineJoint.html "enum bevy::prelude::GizmoLineJoint")

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/config.rs.html#37)

### impl [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [GizmoLineStyle](../../prelude/enum.GizmoLineStyle.html "enum bevy::prelude::GizmoLineStyle")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/gradients.rs.html#457)

### impl [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [Gradient](../../prelude/enum.Gradient.html "enum bevy::prelude::Gradient")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1512)

### impl [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [GridAutoFlow](../../prelude/enum.GridAutoFlow.html "enum bevy::prelude::GridAutoFlow")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1768)

### impl [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [GridTrackRepetition](../../prelude/enum.GridTrackRepetition.html "enum bevy::prelude::GridTrackRepetition")

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/image.rs.html#723)

### impl [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [ImageAddressMode](../../image/enum.ImageAddressMode.html "enum bevy::image::ImageAddressMode")

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/image.rs.html#776)

### impl [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [ImageCompareFunction](../../image/enum.ImageCompareFunction.html "enum bevy::image::ImageCompareFunction")

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/image.rs.html#757)

### impl [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [ImageFilterMode](../../image/enum.ImageFilterMode.html "enum bevy::image::ImageFilterMode")

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/image.rs.html#673)

### impl [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [ImageSampler](../../image/enum.ImageSampler.html "enum bevy::image::ImageSampler")

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/image.rs.html#804)

### impl [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [ImageSamplerBorderColor](../../image/enum.ImageSamplerBorderColor.html "enum bevy::image::ImageSamplerBorderColor")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/event.rs.html#247)

### impl [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [Ime](../../prelude/enum.Ime.html "enum bevy::prelude::Ime")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/index.rs.html#83)

### impl [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [Indices](../../mesh/enum.Indices.html "enum bevy::mesh::Indices")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#875)

### impl [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [InlineDirection](../../prelude/enum.InlineDirection.html "enum bevy::prelude::InlineDirection")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/focus.rs.html#44)

### impl [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [Interaction](../../prelude/enum.Interaction.html "enum bevy::prelude::Interaction")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/gradients.rs.html#634)

### impl [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [InterpolationColorSpace](../../prelude/enum.InterpolationColorSpace.html "enum bevy::prelude::InterpolationColorSpace")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/easing.rs.html#346)

### impl [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [JumpAt](../../prelude/enum.JumpAt.html "enum bevy::prelude::JumpAt")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#230)

### impl [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [Justify](../../prelude/enum.Justify.html "enum bevy::prelude::Justify")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1102)

### impl [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [JustifyContent](../../prelude/enum.JustifyContent.html "enum bevy::prelude::JustifyContent")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#938)

### impl [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [JustifyItems](../../prelude/enum.JustifyItems.html "enum bevy::prelude::JustifyItems")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1018)

### impl [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [JustifySelf](../../prelude/enum.JustifySelf.html "enum bevy::prelude::JustifySelf")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/keyboard.rs.html#804)

### impl [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [Key](../../input/keyboard/enum.Key.html "enum bevy::input::keyboard::Key")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/keyboard.rs.html#262)

### impl [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [KeyCode](../../prelude/enum.KeyCode.html "enum bevy::prelude::KeyCode")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#1039)

### impl [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [LetterSpacing](../../text/enum.LetterSpacing.html "enum bevy::text::LetterSpacing")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/gizmos.rs.html#151)

### impl [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [LightGizmoColor](../../prelude/enum.LightGizmoColor.html "enum bevy::prelude::LightGizmoColor")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#1112)

### impl [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [LineBreak](../../prelude/enum.LineBreak.html "enum bevy::prelude::LineBreak")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#1011)

### impl [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [LineHeight](../../text/enum.LineHeight.html "enum bevy::text::LineHeight")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1569)

### impl [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [MaxTrackSizingFunction](../../prelude/enum.MaxTrackSizingFunction.html "enum bevy::prelude::MaxTrackSizingFunction")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/menu.rs.html#61)

### impl [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [MenuAction](../../ui_widgets/enum.MenuAction.html "enum bevy::ui_widgets::MenuAction")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/menu.rs.html#139)

### impl [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [MenuFocusState](../../ui_widgets/enum.MenuFocusState.html "enum bevy::ui_widgets::MenuFocusState")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/menu.rs.html#91)

### impl [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [MenuLayout](../../ui_widgets/enum.MenuLayout.html "enum bevy::ui_widgets::MenuLayout")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/morph.rs.html#118)

### impl [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [MeshMorphWeights](../../mesh/morph/enum.MeshMorphWeights.html "enum bevy::mesh::morph::MeshMorphWeights")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1540)

### impl [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [MinTrackSizingFunction](../../prelude/enum.MinTrackSizingFunction.html "enum bevy::prelude::MinTrackSizingFunction")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/window.rs.html#1147)

### impl [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [MonitorSelection](../../prelude/enum.MonitorSelection.html "enum bevy::prelude::MonitorSelection")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/mouse.rs.html#64)

### impl [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [MouseButton](../../prelude/enum.MouseButton.html "enum bevy::prelude::MouseButton")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/mouse.rs.html#121)

### impl [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [MouseScrollUnit](../../input/mouse/enum.MouseScrollUnit.html "enum bevy::input::mouse::MouseScrollUnit")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/view/mod.rs.html#231)

### impl [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [Msaa](../../prelude/enum.Msaa.html "enum bevy::prelude::Msaa")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/clear_color.rs.html#29)

### impl [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [MsaaWriteback](../../prelude/enum.MsaaWriteback.html "enum bevy::prelude::MsaaWriteback")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/keyboard.rs.html#758)

### impl [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [NativeKey](../../input/keyboard/enum.NativeKey.html "enum bevy::input::keyboard::NativeKey")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/keyboard.rs.html#220)

### impl [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [NativeKeyCode](../../input/keyboard/enum.NativeKeyCode.html "enum bevy::input::keyboard::NativeKeyCode")

[Source](https://docs.rs/bevy_input_focus/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input_focus/tab_navigation.rs.html#107)

### impl [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [NavAction](../../input_focus/tab_navigation/enum.NavAction.html "enum bevy::input_focus::tab_navigation::NavAction")

[Source](https://docs.rs/bevy_input_focus/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input_focus/directional_navigation.rs.html#159)

### impl [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [NavNeighbor](../../input_focus/directional_navigation/enum.NavNeighbor.html "enum bevy::input_focus::directional_navigation::NavNeighbor")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/widget/image.rs.html#156)

### impl [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [NodeImageMode](../../prelude/enum.NodeImageMode.html "enum bevy::prelude::NodeImageMode")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/camera.rs.html#940)

### impl [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [NormalizedRenderTarget](../../camera/enum.NormalizedRenderTarget.html "enum bevy::camera::NormalizedRenderTarget")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/number_input.rs.html#131)

### impl [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [NumberFormat](../../feathers/controls/enum.NumberFormat.html "enum bevy::feathers::controls::NumberFormat")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/number_input.rs.html#146)

### impl [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [NumberInputValue](../../feathers/controls/enum.NumberInputValue.html "enum bevy::feathers::controls::NumberInputValue")

[Source](https://docs.rs/bevy_material/0.19.0/x86_64-unknown-linux-gnu/src/bevy_material/opaque.rs.html#21)

### impl [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [OpaqueRendererMethod](../../material/enum.OpaqueRendererMethod.html "enum bevy::material::OpaqueRendererMethod")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1347)

### impl [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [OverflowAxis](../../prelude/enum.OverflowAxis.html "enum bevy::prelude::OverflowAxis")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/probe.rs.html#410)

### impl [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [ParallaxCorrection](../../light/enum.ParallaxCorrection.html "enum bevy::light::ParallaxCorrection")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/parallax.rs.html#14)

### impl [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [ParallaxMappingMethod](../../prelude/enum.ParallaxMappingMethod.html "enum bevy::prelude::ParallaxMappingMethod")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/hover.rs.html#224)

### impl [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [PickingInteraction](../../picking/hover/enum.PickingInteraction.html "enum bevy::picking::hover::PickingInteraction")

[Source](https://docs.rs/bevy_audio/0.19.0/x86_64-unknown-linux-gnu/src/bevy_audio/audio.rs.html#9)

### impl [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [PlaybackMode](../../audio/enum.PlaybackMode.html "enum bevy::audio::PlaybackMode")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/pointer.rs.html#248)

### impl [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [PointerAction](../../picking/pointer/enum.PointerAction.html "enum bevy::picking::pointer::PointerAction")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/pointer.rs.html#159)

### impl [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [PointerButton](../../prelude/enum.PointerButton.html "enum bevy::prelude::PointerButton")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/pointer.rs.html#31)

### impl [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [PointerId](../../picking/pointer/enum.PointerId.html "enum bevy::picking::pointer::PointerId")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/popover.rs.html#52)

### impl [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [PopoverAlign](../../ui_widgets/popover/enum.PopoverAlign.html "enum bevy::ui_widgets::popover::PopoverAlign")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/popover.rs.html#23)

### impl [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [PopoverSide](../../ui_widgets/popover/enum.PopoverSide.html "enum bevy::ui_widgets::popover::PopoverSide")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1453)

### impl [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [PositionType](../../prelude/enum.PositionType.html "enum bevy::prelude::PositionType")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/window.rs.html#1214)

### impl [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [PresentMode](../../window/enum.PresentMode.html "enum bevy::window::PresentMode")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/pointer.rs.html#149)

### impl [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [PressDirection](../../picking/pointer/enum.PressDirection.html "enum bevy::picking::pointer::PressDirection")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/projection.rs.html#214)

### impl [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [Projection](../../prelude/enum.Projection.html "enum bevy::prelude::Projection")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/gradients.rs.html#558)

### impl [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [RadialGradientShape](../../prelude/enum.RadialGradientShape.html "enum bevy::prelude::RadialGradientShape")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/gamepad.rs.html#65)

### impl [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [RawGamepadEvent](../../input/gamepad/enum.RawGamepadEvent.html "enum bevy::input::gamepad::RawGamepadEvent")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/mesh_picking/ray_cast/mod.rs.html#27)

### impl [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [RayCastVisibility](../../prelude/enum.RayCastVisibility.html "enum bevy::prelude::RayCastVisibility")

[Source](https://docs.rs/bevy_dev_tools/0.19.0/x86_64-unknown-linux-gnu/src/bevy_dev_tools/render_debug.rs.html#319)

### impl [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [RenderDebugMode](../../dev_tools/render_debug/enum.RenderDebugMode.html "enum bevy::dev_tools::render_debug::RenderDebugMode")

[Source](https://docs.rs/bevy_dev_tools/0.19.0/x86_64-unknown-linux-gnu/src/bevy_dev_tools/render_debug.rs.html#262)

### impl [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [RenderDebugOverlayEvent](../../dev_tools/render_debug/enum.RenderDebugOverlayEvent.html "enum bevy::dev_tools::render_debug::RenderDebugOverlayEvent")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/camera.rs.html#890)

### impl [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [RenderTarget](../../camera/enum.RenderTarget.html "enum bevy::camera::RenderTarget")

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/lib.rs.html#469)

### impl [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [RepeatAnimation](../../animation/enum.RepeatAnimation.html "enum bevy::animation::RepeatAnimation")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/projection.rs.html#521)

### impl [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [ScalingMode](../../camera/enum.ScalingMode.html "enum bevy::camera::ScalingMode")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/window.rs.html#1478)

### impl [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [ScreenEdge](../../window/enum.ScreenEdge.html "enum bevy::window::ScreenEdge")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/ssao/mod.rs.html#135)

### impl [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [ScreenSpaceAmbientOcclusionQualityLevel](../../pbr/enum.ScreenSpaceAmbientOcclusionQualityLevel.html "enum bevy::pbr::ScreenSpaceAmbientOcclusionQualityLevel")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/transmission/mod.rs.html#110)

### impl [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [ScreenSpaceTransmissionQuality](../../pbr/enum.ScreenSpaceTransmissionQuality.html "enum bevy::pbr::ScreenSpaceTransmissionQuality")

[Source](https://docs.rs/bevy_anti_alias/0.19.0/x86_64-unknown-linux-gnu/src/bevy_anti_alias/fxaa/mod.rs.html#29)

### impl [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [Sensitivity](../../anti_alias/fxaa/enum.Sensitivity.html "enum bevy::anti_alias::fxaa::Sensitivity")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/lib.rs.html#283)

### impl [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [ShadowFilteringMethod](../../light/enum.ShadowFilteringMethod.html "enum bevy::light::ShadowFilteringMethod")

[Source](https://docs.rs/bevy_sprite/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite/texture_slice/slicer.rs.html#27)

### impl [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [SliceScaleMode](../../prelude/enum.SliceScaleMode.html "enum bevy::prelude::SliceScaleMode")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/slider.rs.html#35)

### impl [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [SliderOrientation](../../ui_widgets/enum.SliderOrientation.html "enum bevy::ui_widgets::SliderOrientation")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/slider.rs.html#683)

### impl [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [SliderValueChange](../../ui_widgets/enum.SliderValueChange.html "enum bevy::ui_widgets::SliderValueChange")

[Source](https://docs.rs/bevy_anti_alias/0.19.0/x86_64-unknown-linux-gnu/src/bevy_anti_alias/smaa/mod.rs.html#106)

### impl [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [SmaaPreset](../../anti_alias/smaa/enum.SmaaPreset.html "enum bevy::anti_alias::smaa::SmaaPreset")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim3/sphere.rs.html#23)

### impl [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [SphereKind](../../mesh/enum.SphereKind.html "enum bevy::mesh::SphereKind")

[Source](https://docs.rs/bevy_sprite/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite/sprite_mesh.rs.html#178)

### impl [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [SpriteAlphaMode](../../sprite/enum.SpriteAlphaMode.html "enum bevy::sprite::SpriteAlphaMode")

[Source](https://docs.rs/bevy_sprite/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite/sprite.rs.html#166)

### impl [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [SpriteImageMode](../../prelude/enum.SpriteImageMode.html "enum bevy::prelude::SpriteImageMode")

[Source](https://docs.rs/bevy_sprite/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite/picking_backend.rs.html#39)

### impl [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [SpritePickingMode](../../prelude/enum.SpritePickingMode.html "enum bevy::prelude::SpritePickingMode")

[Source](https://docs.rs/bevy_sprite/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite/sprite.rs.html#214)

### impl [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [SpriteScalingMode](../../prelude/enum.SpriteScalingMode.html "enum bevy::prelude::SpriteScalingMode")

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/systems.rs.html#88)

### impl [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [StaticTransformOptimizations](../../prelude/enum.StaticTransformOptimizations.html "enum bevy::prelude::StaticTransformOptimizations")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/cursor/system_cursor.rs.html#89)

### impl [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [SystemCursorIcon](../../window/enum.SystemCursorIcon.html "enum bevy::window::SystemCursorIcon")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text_edit.rs.html#25)

### impl [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [TextEdit](../../text/enum.TextEdit.html "enum bevy::text::TextEdit")

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/tilemap_chunk/tile_orientation.rs.html#37)

### impl [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [TileOrientation](../../sprite_render/enum.TileOrientation.html "enum bevy::sprite_render::TileOrientation")

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/timer.rs.html#492)

### impl [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [TimerMode](../../prelude/enum.TimerMode.html "enum bevy::prelude::TimerMode")

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/tonemapping/mod.rs.html#115)

### impl [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [Tonemapping](../../core_pipeline/tonemapping/enum.Tonemapping.html "enum bevy::core_pipeline::tonemapping::Tonemapping")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/touch.rs.html#123)

### impl [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [TouchPhase](../../input/touch/enum.TouchPhase.html "enum bevy::input::touch::TouchPhase")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/slider.rs.html#61)

### impl [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [TrackClick](../../ui_widgets/enum.TrackClick.html "enum bevy::ui_widgets::TrackClick")

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/transform_gizmo.rs.html#123)

### impl [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [TransformGizmoAxis](../../prelude/enum.TransformGizmoAxis.html "enum bevy::prelude::TransformGizmoAxis")

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/transform_gizmo.rs.html#101)

### impl [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [TransformGizmoMode](../../prelude/enum.TransformGizmoMode.html "enum bevy::prelude::TransformGizmoMode")

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/transform_gizmo.rs.html#113)

### impl [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [TransformGizmoSpace](../../prelude/enum.TransformGizmoSpace.html "enum bevy::prelude::TransformGizmoSpace")

[Source](https://docs.rs/bevy_ui_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_render/lib.rs.html#159)

### impl [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [UiAntiAlias](../../prelude/enum.UiAntiAlias.html "enum bevy::prelude::UiAntiAlias")

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/id.rs.html#167)

### impl [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [UntypedAssetId](../../asset/enum.UntypedAssetId.html "enum bevy::asset::UntypedAssetId")

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/handle.rs.html#474)

### impl [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [UntypedHandle](../../prelude/enum.UntypedHandle.html "enum bevy::prelude::UntypedHandle")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/mesh.rs.html#2531)

### impl [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [UvChannel](../../mesh/enum.UvChannel.html "enum bevy::mesh::UvChannel")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#25)

### impl [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [Val](../../prelude/enum.Val.html "enum bevy::prelude::Val")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/window.rs.html#1175)

### impl [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [VideoModeSelection](../../prelude/enum.VideoModeSelection.html "enum bevy::prelude::VideoModeSelection")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/visibility/mod.rs.html#80)

### impl [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [Visibility](../../prelude/enum.Visibility.html "enum bevy::prelude::Visibility")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1435)

### impl [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [VisualBox](../../prelude/enum.VisualBox.html "enum bevy::prelude::VisualBox")

[Source](https://docs.rs/bevy_audio/0.19.0/x86_64-unknown-linux-gnu/src/bevy_audio/volume.rs.html#34)

### impl [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [Volume](../../audio/enum.Volume.html "enum bevy::audio::Volume")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/event.rs.html#496)

### impl [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [WindowEvent](../../window/enum.WindowEvent.html "enum bevy::window::WindowEvent")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/window.rs.html#1382)

### impl [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [WindowLevel](../../window/enum.WindowLevel.html "enum bevy::window::WindowLevel")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/window.rs.html#1334)

### impl [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [WindowMode](../../window/enum.WindowMode.html "enum bevy::window::WindowMode")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/window.rs.html#796)

### impl [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [WindowPosition](../../prelude/enum.WindowPosition.html "enum bevy::prelude::WindowPosition")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/window.rs.html#64)

### impl [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [WindowRef](../../window/enum.WindowRef.html "enum bevy::window::WindowRef")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/window.rs.html#1406)

### impl [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [WindowTheme](../../window/enum.WindowTheme.html "enum bevy::window::WindowTheme")

[Source](https://docs.rs/bevy_winit/0.19.0/x86_64-unknown-linux-gnu/src/bevy_winit/lib.rs.html#175)

### impl [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [WinitUserEvent](../../winit/enum.WinitUserEvent.html "enum bevy::winit::WinitUserEvent")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/wireframe.rs.html#873)

### impl [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [WireframeTopology](../../pbr/wireframe/enum.WireframeTopology.html "enum bevy::pbr::wireframe::WireframeTopology")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/path/access.rs.html#16)

### impl<'a> [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [Access](../enum.Access.html "enum bevy::reflect::Access")<'a>

where [Access](../enum.Access.html "enum bevy::reflect::Access")<'a>: 'static,

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/event.rs.html#49)

### impl<A> [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [AssetEvent](../../prelude/enum.AssetEvent.html "enum bevy::prelude::AssetEvent")<A>

where A: [Asset](../../prelude/trait.Asset.html "trait bevy::prelude::Asset") + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [AssetEvent](../../prelude/enum.AssetEvent.html "enum bevy::prelude::AssetEvent")<A>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), [AssetId](../../prelude/enum.AssetId.html "enum bevy::prelude::AssetId")<A>: [FromReflect](../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/id.rs.html#21)

### impl<A> [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [AssetId](../../prelude/enum.AssetId.html "enum bevy::prelude::AssetId")<A>

where A: [Asset](../../prelude/trait.Asset.html "trait bevy::prelude::Asset") + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [AssetId](../../prelude/enum.AssetId.html "enum bevy::prelude::AssetId")<A>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/handle.rs.html#132)

### impl<A> [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [Handle](../../prelude/enum.Handle.html "enum bevy::prelude::Handle")<A>

where A: [Asset](../../prelude/trait.Asset.html "trait bevy::prelude::Asset") + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [Handle](../../prelude/enum.Handle.html "enum bevy::prelude::Handle")<A>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_state/0.19.0/x86_64-unknown-linux-gnu/src/bevy_state/state/resources.rs.html#178)

### impl<S> [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [NextState](../../prelude/enum.NextState.html "enum bevy::prelude::NextState")<S>

where S: [FreelyMutableState](../../state/state/trait.FreelyMutableState.html "trait bevy::state::state::FreelyMutableState") + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [FromReflect](../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + RegisterForReflection, [NextState](../../prelude/enum.NextState.html "enum bevy::prelude::NextState")<S>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/handle.rs.html#272)

### impl<T> [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [HandleTemplate](../../asset/enum.HandleTemplate.html "enum bevy::asset::HandleTemplate")<T>

where T: [Asset](../../prelude/trait.Asset.html "trait bevy::prelude::Asset") + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [HandleTemplate](../../asset/enum.HandleTemplate.html "enum bevy::asset::HandleTemplate")<T>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), [Handle](../../prelude/enum.Handle.html "enum bevy::prelude::Handle")<T>: [FromReflect](../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + MaybeTyped + RegisterForReflection, [ArcMutexValue](../../asset/struct.ArcMutexValue.html "struct bevy::asset::ArcMutexValue")<T>: [FromReflect](../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/cores.rs.html#25)

### impl<T> [Enum](trait.Enum.html "trait bevy::reflect::enums::Enum") for [InterpolationDatum](../../prelude/cores/enum.InterpolationDatum.html "enum bevy::prelude::cores::InterpolationDatum")<T>

where [InterpolationDatum](../../prelude/cores/enum.InterpolationDatum.html "enum bevy::prelude::cores::InterpolationDatum")<T>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), T: [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [FromReflect](../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + RegisterForReflection,

{"VariantFieldIter<'\_>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"struct.VariantFieldIter.html\\" title=\\"struct bevy::reflect::enums::VariantFieldIter\\">VariantFieldIter</a>&lt;'a&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"struct.VariantFieldIter.html\\" title=\\"struct bevy::reflect::enums::VariantFieldIter\\">VariantFieldIter</a>&lt;'a&gt;</div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = <a class=\\"enum\\" href=\\"enum.VariantField.html\\" title=\\"enum bevy::reflect::enums::VariantField\\">VariantField</a>&lt;'a&gt;;</div>"}