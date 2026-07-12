[bevy](../../index.html)::[reflect](../index.html)::[tuple](index.html)

# Trait Tuple 

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#43)

```rust
pub trait Tuple: PartialReflect {
    // Required methods
    fn field(&self, index: usize) -> Option<&(dyn PartialReflect + 'static)>;
    fn field_mut(
        &mut self,
        index: usize,
    ) -> Option<&mut (dyn PartialReflect + 'static)>;
    fn field_len(&self) -> usize;
    fn iter_fields(&self) -> TupleFieldIter<'_> ⓘ;
    fn drain(self: Box<Self>) -> Vec<Box<dyn PartialReflect>>;

    // Provided methods
    fn to_dynamic_tuple(&self) -> DynamicTuple { ... }
    fn get_represented_tuple_info(&self) -> Option<&'static TupleInfo> { ... }
}
```

A trait used to power [tuple-like](https://doc.rust-lang.org/book/ch03-02-data-types.html#the-tuple-type) operations via [reflection](../index.html "mod bevy::reflect").

This trait uses the [`Reflect`](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") trait to allow implementors to have their fields be dynamically addressed by index.

This trait is automatically implemented for arbitrary tuples of up to 12 elements, provided that each element implements [`Reflect`](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect").

## Example

```rust
use bevy_reflect::{PartialReflect, tuple::Tuple};

let foo = (123_u32, true);
assert_eq!(foo.field_len(), 2);

let field: &dyn PartialReflect = foo.field(0).unwrap();
assert_eq!(field.try_downcast_ref::<u32>(), Some(&123));
```

## Required Methods

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#46)

#### fn [field](#tymethod.field)(&self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&(dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)>

Returns a reference to the value of the field with index `index` as a `&dyn Reflect`.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#50)

#### fn [field\_mut](#tymethod.field_mut)( &mut self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&mut (dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)>

Returns a mutable reference to the value of the field with index `index` as a `&mut dyn Reflect`.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#53)

#### fn [field\_len](#tymethod.field_len)(&self) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

Returns the number of fields in the tuple.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#56)

#### fn [iter\_fields](#tymethod.iter_fields)(&self) -> [TupleFieldIter](struct.TupleFieldIter.html "struct bevy::reflect::tuple::TupleFieldIter")<'\_> [ⓘ](#)

Returns an iterator over the values of the tuple’s fields.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#59)

#### fn [drain](#tymethod.drain)(self: [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<Self>) -> [Vec](../../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>>

Drain the fields of this tuple to get a vector of owned values.

## Provided Methods

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#62)

#### fn [to\_dynamic\_tuple](#method.to_dynamic_tuple)(&self) -> [DynamicTuple](struct.DynamicTuple.html "struct bevy::reflect::tuple::DynamicTuple")

Creates a new [`DynamicTuple`](struct.DynamicTuple.html "struct bevy::reflect::tuple::DynamicTuple") from this tuple.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#70)

#### fn [get\_represented\_tuple\_info](#method.get_represented_tuple_info)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [TupleInfo](struct.TupleInfo.html "struct bevy::reflect::tuple::TupleInfo")\>

Will return `None` if [`TypeInfo`](../enum.TypeInfo.html "enum bevy::reflect::TypeInfo") is not available.

## Trait Implementations

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#145)

### impl [GetTupleField](trait.GetTupleField.html "trait bevy::reflect::tuple::GetTupleField") for dyn [Tuple](trait.Tuple.html "trait bevy::reflect::tuple::Tuple")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#146)

#### fn [get\_field](trait.GetTupleField.html#tymethod.get_field)<T>(&self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)\>

where T: [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect"),

Returns a reference to the value of the field with index `index`, downcast to `T`.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#151)

#### fn [get\_field\_mut](trait.GetTupleField.html#tymethod.get_field_mut)<T>(&mut self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)\>

where T: [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect"),

Returns a mutable reference to the value of the field with index `index`, downcast to `T`.

## Dyn Compatibility

This trait **is** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementations on Foreign Types

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#688)

### impl [Tuple](trait.Tuple.html "trait bevy::reflect::tuple::Tuple") for [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#688)

#### fn [field](#tymethod.field)(&self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&(dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#688)

#### fn [field\_mut](#tymethod.field_mut)( &mut self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&mut (dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#688)

#### fn [field\_len](#tymethod.field_len)(&self) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#688)

#### fn [iter\_fields](#tymethod.iter_fields)(&self) -> [TupleFieldIter](struct.TupleFieldIter.html "struct bevy::reflect::tuple::TupleFieldIter")<'\_> [ⓘ](#)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#688)

#### fn [drain](#tymethod.drain)(self: [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)\>) -> [Vec](../../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#712)

### impl<A, B, C, D, E, F, G, H, I, J, K, L> [Tuple](trait.Tuple.html "trait bevy::reflect::tuple::Tuple") for [(A, B, C, D, E, F, G, H, I, J, K, L)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)

where A: [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + MaybeTyped + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), B: [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + MaybeTyped + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), C: [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + MaybeTyped + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), D: [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + MaybeTyped + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), E: [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + MaybeTyped + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), F: [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + MaybeTyped + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), G: [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + MaybeTyped + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), H: [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + MaybeTyped + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), I: [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + MaybeTyped + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), J: [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + MaybeTyped + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), K: [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + MaybeTyped + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), L: [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + MaybeTyped + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#712)

#### fn [field](#tymethod.field)(&self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&(dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#712)

#### fn [field\_mut](#tymethod.field_mut)( &mut self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&mut (dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#712)

#### fn [field\_len](#tymethod.field_len)(&self) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#712)

#### fn [iter\_fields](#tymethod.iter_fields)(&self) -> [TupleFieldIter](struct.TupleFieldIter.html "struct bevy::reflect::tuple::TupleFieldIter")<'\_> [ⓘ](#)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#712)

#### fn [drain](#tymethod.drain)( self: [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<[(A, B, C, D, E, F, G, H, I, J, K, L)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)\>, ) -> [Vec](../../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#710)

### impl<A, B, C, D, E, F, G, H, I, J, K> [Tuple](trait.Tuple.html "trait bevy::reflect::tuple::Tuple") for [(A, B, C, D, E, F, G, H, I, J, K)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)

where A: [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + MaybeTyped + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), B: [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + MaybeTyped + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), C: [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + MaybeTyped + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), D: [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + MaybeTyped + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), E: [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + MaybeTyped + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), F: [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + MaybeTyped + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), G: [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + MaybeTyped + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), H: [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + MaybeTyped + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), I: [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + MaybeTyped + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), J: [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + MaybeTyped + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), K: [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + MaybeTyped + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#710)

#### fn [field](#tymethod.field)(&self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&(dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#710)

#### fn [field\_mut](#tymethod.field_mut)( &mut self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&mut (dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#710)

#### fn [field\_len](#tymethod.field_len)(&self) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#710)

#### fn [iter\_fields](#tymethod.iter_fields)(&self) -> [TupleFieldIter](struct.TupleFieldIter.html "struct bevy::reflect::tuple::TupleFieldIter")<'\_> [ⓘ](#)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#710)

#### fn [drain](#tymethod.drain)( self: [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<[(A, B, C, D, E, F, G, H, I, J, K)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)\>, ) -> [Vec](../../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#708)

### impl<A, B, C, D, E, F, G, H, I, J> [Tuple](trait.Tuple.html "trait bevy::reflect::tuple::Tuple") for [(A, B, C, D, E, F, G, H, I, J)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)

where A: [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + MaybeTyped + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), B: [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + MaybeTyped + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), C: [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + MaybeTyped + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), D: [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + MaybeTyped + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), E: [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + MaybeTyped + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), F: [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + MaybeTyped + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), G: [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + MaybeTyped + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), H: [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + MaybeTyped + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), I: [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + MaybeTyped + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), J: [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + MaybeTyped + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#708)

#### fn [field](#tymethod.field)(&self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&(dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#708)

#### fn [field\_mut](#tymethod.field_mut)( &mut self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&mut (dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#708)

#### fn [field\_len](#tymethod.field_len)(&self) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#708)

#### fn [iter\_fields](#tymethod.iter_fields)(&self) -> [TupleFieldIter](struct.TupleFieldIter.html "struct bevy::reflect::tuple::TupleFieldIter")<'\_> [ⓘ](#)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#708)

#### fn [drain](#tymethod.drain)( self: [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<[(A, B, C, D, E, F, G, H, I, J)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)\>, ) -> [Vec](../../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#706)

### impl<A, B, C, D, E, F, G, H, I> [Tuple](trait.Tuple.html "trait bevy::reflect::tuple::Tuple") for [(A, B, C, D, E, F, G, H, I)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)

where A: [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + MaybeTyped + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), B: [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + MaybeTyped + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), C: [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + MaybeTyped + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), D: [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + MaybeTyped + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), E: [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + MaybeTyped + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), F: [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + MaybeTyped + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), G: [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + MaybeTyped + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), H: [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + MaybeTyped + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), I: [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + MaybeTyped + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#706)

#### fn [field](#tymethod.field)(&self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&(dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#706)

#### fn [field\_mut](#tymethod.field_mut)( &mut self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&mut (dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#706)

#### fn [field\_len](#tymethod.field_len)(&self) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#706)

#### fn [iter\_fields](#tymethod.iter_fields)(&self) -> [TupleFieldIter](struct.TupleFieldIter.html "struct bevy::reflect::tuple::TupleFieldIter")<'\_> [ⓘ](#)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#706)

#### fn [drain](#tymethod.drain)(self: [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<[(A, B, C, D, E, F, G, H, I)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)\>) -> [Vec](../../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#704)

### impl<A, B, C, D, E, F, G, H> [Tuple](trait.Tuple.html "trait bevy::reflect::tuple::Tuple") for [(A, B, C, D, E, F, G, H)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)

where A: [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + MaybeTyped + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), B: [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + MaybeTyped + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), C: [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + MaybeTyped + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), D: [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + MaybeTyped + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), E: [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + MaybeTyped + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), F: [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + MaybeTyped + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), G: [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + MaybeTyped + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), H: [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + MaybeTyped + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#704)

#### fn [field](#tymethod.field)(&self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&(dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#704)

#### fn [field\_mut](#tymethod.field_mut)( &mut self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&mut (dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#704)

#### fn [field\_len](#tymethod.field_len)(&self) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#704)

#### fn [iter\_fields](#tymethod.iter_fields)(&self) -> [TupleFieldIter](struct.TupleFieldIter.html "struct bevy::reflect::tuple::TupleFieldIter")<'\_> [ⓘ](#)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#704)

#### fn [drain](#tymethod.drain)(self: [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<[(A, B, C, D, E, F, G, H)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)\>) -> [Vec](../../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#702)

### impl<A, B, C, D, E, F, G> [Tuple](trait.Tuple.html "trait bevy::reflect::tuple::Tuple") for [(A, B, C, D, E, F, G)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)

where A: [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + MaybeTyped + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), B: [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + MaybeTyped + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), C: [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + MaybeTyped + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), D: [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + MaybeTyped + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), E: [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + MaybeTyped + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), F: [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + MaybeTyped + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), G: [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + MaybeTyped + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#702)

#### fn [field](#tymethod.field)(&self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&(dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#702)

#### fn [field\_mut](#tymethod.field_mut)( &mut self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&mut (dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#702)

#### fn [field\_len](#tymethod.field_len)(&self) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#702)

#### fn [iter\_fields](#tymethod.iter_fields)(&self) -> [TupleFieldIter](struct.TupleFieldIter.html "struct bevy::reflect::tuple::TupleFieldIter")<'\_> [ⓘ](#)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#702)

#### fn [drain](#tymethod.drain)(self: [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<[(A, B, C, D, E, F, G)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)\>) -> [Vec](../../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#700)

### impl<A, B, C, D, E, F> [Tuple](trait.Tuple.html "trait bevy::reflect::tuple::Tuple") for [(A, B, C, D, E, F)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)

where A: [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + MaybeTyped + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), B: [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + MaybeTyped + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), C: [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + MaybeTyped + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), D: [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + MaybeTyped + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), E: [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + MaybeTyped + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), F: [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + MaybeTyped + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#700)

#### fn [field](#tymethod.field)(&self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&(dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#700)

#### fn [field\_mut](#tymethod.field_mut)( &mut self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&mut (dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#700)

#### fn [field\_len](#tymethod.field_len)(&self) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#700)

#### fn [iter\_fields](#tymethod.iter_fields)(&self) -> [TupleFieldIter](struct.TupleFieldIter.html "struct bevy::reflect::tuple::TupleFieldIter")<'\_> [ⓘ](#)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#700)

#### fn [drain](#tymethod.drain)(self: [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<[(A, B, C, D, E, F)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)\>) -> [Vec](../../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#698)

### impl<A, B, C, D, E> [Tuple](trait.Tuple.html "trait bevy::reflect::tuple::Tuple") for [(A, B, C, D, E)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)

where A: [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + MaybeTyped + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), B: [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + MaybeTyped + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), C: [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + MaybeTyped + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), D: [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + MaybeTyped + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), E: [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + MaybeTyped + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#698)

#### fn [field](#tymethod.field)(&self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&(dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#698)

#### fn [field\_mut](#tymethod.field_mut)( &mut self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&mut (dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#698)

#### fn [field\_len](#tymethod.field_len)(&self) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#698)

#### fn [iter\_fields](#tymethod.iter_fields)(&self) -> [TupleFieldIter](struct.TupleFieldIter.html "struct bevy::reflect::tuple::TupleFieldIter")<'\_> [ⓘ](#)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#698)

#### fn [drain](#tymethod.drain)(self: [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<[(A, B, C, D, E)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)\>) -> [Vec](../../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#696)

### impl<A, B, C, D> [Tuple](trait.Tuple.html "trait bevy::reflect::tuple::Tuple") for [(A, B, C, D)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)

where A: [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + MaybeTyped + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), B: [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + MaybeTyped + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), C: [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + MaybeTyped + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), D: [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + MaybeTyped + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#696)

#### fn [field](#tymethod.field)(&self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&(dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#696)

#### fn [field\_mut](#tymethod.field_mut)( &mut self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&mut (dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#696)

#### fn [field\_len](#tymethod.field_len)(&self) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#696)

#### fn [iter\_fields](#tymethod.iter_fields)(&self) -> [TupleFieldIter](struct.TupleFieldIter.html "struct bevy::reflect::tuple::TupleFieldIter")<'\_> [ⓘ](#)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#696)

#### fn [drain](#tymethod.drain)(self: [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<[(A, B, C, D)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)\>) -> [Vec](../../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#694)

### impl<A, B, C> [Tuple](trait.Tuple.html "trait bevy::reflect::tuple::Tuple") for [(A, B, C)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)

where A: [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + MaybeTyped + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), B: [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + MaybeTyped + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), C: [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + MaybeTyped + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#694)

#### fn [field](#tymethod.field)(&self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&(dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#694)

#### fn [field\_mut](#tymethod.field_mut)( &mut self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&mut (dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#694)

#### fn [field\_len](#tymethod.field_len)(&self) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#694)

#### fn [iter\_fields](#tymethod.iter_fields)(&self) -> [TupleFieldIter](struct.TupleFieldIter.html "struct bevy::reflect::tuple::TupleFieldIter")<'\_> [ⓘ](#)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#694)

#### fn [drain](#tymethod.drain)(self: [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<[(A, B, C)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)\>) -> [Vec](../../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#692)

### impl<A, B> [Tuple](trait.Tuple.html "trait bevy::reflect::tuple::Tuple") for [(A, B)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)

where A: [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + MaybeTyped + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), B: [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + MaybeTyped + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#692)

#### fn [field](#tymethod.field)(&self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&(dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#692)

#### fn [field\_mut](#tymethod.field_mut)( &mut self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&mut (dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#692)

#### fn [field\_len](#tymethod.field_len)(&self) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#692)

#### fn [iter\_fields](#tymethod.iter_fields)(&self) -> [TupleFieldIter](struct.TupleFieldIter.html "struct bevy::reflect::tuple::TupleFieldIter")<'\_> [ⓘ](#)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#692)

#### fn [drain](#tymethod.drain)(self: [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<[(A, B)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)\>) -> [Vec](../../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#690)

### impl<A> [Tuple](trait.Tuple.html "trait bevy::reflect::tuple::Tuple") for [(A,)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)

where A: [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + MaybeTyped + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#690)

#### fn [field](#tymethod.field)(&self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&(dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#690)

#### fn [field\_mut](#tymethod.field_mut)( &mut self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&mut (dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#690)

#### fn [field\_len](#tymethod.field_len)(&self) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#690)

#### fn [iter\_fields](#tymethod.iter_fields)(&self) -> [TupleFieldIter](struct.TupleFieldIter.html "struct bevy::reflect::tuple::TupleFieldIter")<'\_> [ⓘ](#)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#690)

#### fn [drain](#tymethod.drain)(self: [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<[(A,)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)\>) -> [Vec](../../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>>

## Implementors

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#253)

### impl [Tuple](trait.Tuple.html "trait bevy::reflect::tuple::Tuple") for [DynamicTuple](struct.DynamicTuple.html "struct bevy::reflect::tuple::DynamicTuple")

{"TupleFieldIter<'\_>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"struct.TupleFieldIter.html\\" title=\\"struct bevy::reflect::tuple::TupleFieldIter\\">TupleFieldIter</a>&lt;'a&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"struct.TupleFieldIter.html\\" title=\\"struct bevy::reflect::tuple::TupleFieldIter\\">TupleFieldIter</a>&lt;'a&gt;</div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &amp;'a (dyn <a class=\\"trait\\" href=\\"../../prelude/trait.PartialReflect.html\\" title=\\"trait bevy::prelude::PartialReflect\\">PartialReflect</a> + 'static);</div>"}