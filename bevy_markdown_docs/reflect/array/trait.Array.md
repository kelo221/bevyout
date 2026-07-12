[bevy](../../index.html)::[reflect](../index.html)::[array](index.html)

# Trait Array 

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/array.rs.html#53)

```rust
pub trait Array: PartialReflect {
    // Required methods
    fn get(&self, index: usize) -> Option<&(dyn PartialReflect + 'static)>;
    fn get_mut(
        &mut self,
        index: usize,
    ) -> Option<&mut (dyn PartialReflect + 'static)>;
    fn len(&self) -> usize;
    fn iter(&self) -> ArrayIter<'_> ⓘ;
    fn drain(self: Box<Self>) -> Vec<Box<dyn PartialReflect>>;

    // Provided methods
    fn is_empty(&self) -> bool { ... }
    fn to_dynamic_array(&self) -> DynamicArray { ... }
    fn get_represented_array_info(&self) -> Option<&'static ArrayInfo> { ... }
}
```

A trait used to power [array-like](https://doc.rust-lang.org/book/ch03-02-data-types.html#the-array-type) operations via [reflection](../index.html "mod bevy::reflect").

This corresponds to true Rust arrays like `[T; N]`, but also to any fixed-size linear sequence types. It is expected that implementors of this trait uphold this contract and maintain a fixed size as returned by the [`Array::len`](trait.Array.html#tymethod.len "method bevy::reflect::array::Array::len") method.

Due to the [type-erasing](https://doc.rust-lang.org/book/ch17-02-trait-objects.html) nature of the reflection API as a whole, this trait does not make any guarantees that the implementor’s elements are homogeneous (i.e. all the same type).

This trait has a blanket implementation over Rust arrays of up to 32 items. This implementation can technically contain more than 32, but the blanket [`GetTypeRegistration`](../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration") is only implemented up to the 32 item limit due to a [limitation](https://github.com/serde-rs/serde/issues/1937) on [`Deserialize`](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserialize.html "trait serde_core::de::Deserialize").

## Example

```rust
use bevy_reflect::{PartialReflect, array::Array};

let foo: &dyn Array = &[123_u32, 456_u32, 789_u32];
assert_eq!(foo.len(), 3);

let field: &dyn PartialReflect = foo.get(0).unwrap();
assert_eq!(field.try_downcast_ref::<u32>(), Some(&123));
```

## Required Methods

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/array.rs.html#55)

#### fn [get](#tymethod.get)(&self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&(dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)>

Returns a reference to the element at `index`, or `None` if out of bounds.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/array.rs.html#58)

#### fn [get\_mut](#tymethod.get_mut)( &mut self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&mut (dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)>

Returns a mutable reference to the element at `index`, or `None` if out of bounds.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/array.rs.html#61)

#### fn [len](#tymethod.len)(&self) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

Returns the number of elements in the array.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/array.rs.html#69)

#### fn [iter](#tymethod.iter)(&self) -> [ArrayIter](struct.ArrayIter.html "struct bevy::reflect::array::ArrayIter")<'\_> [ⓘ](#)

Returns an iterator over the array.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/array.rs.html#72)

#### fn [drain](#tymethod.drain)(self: [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<Self>) -> [Vec](../../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>>

Drain the elements of this array to get a vector of owned values.

## Provided Methods

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/array.rs.html#64)

#### fn [is\_empty](#method.is_empty)(&self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Returns `true` if the collection contains no elements.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/array.rs.html#75)

#### fn [to\_dynamic\_array](#method.to_dynamic_array)(&self) -> [DynamicArray](struct.DynamicArray.html "struct bevy::reflect::array::DynamicArray")

Creates a new [`DynamicArray`](struct.DynamicArray.html "struct bevy::reflect::array::DynamicArray") from this array.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/array.rs.html#83)

#### fn [get\_represented\_array\_info](#method.get_represented_array_info)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [ArrayInfo](struct.ArrayInfo.html "struct bevy::reflect::array::ArrayInfo")\>

Will return `None` if [`TypeInfo`](../enum.TypeInfo.html "enum bevy::reflect::TypeInfo") is not available.

## Dyn Compatibility

This trait **is** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementations on Foreign Types

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#460)

### impl<T, const N: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\> [Array](trait.Array.html "trait bevy::reflect::array::Array") for [\[T; N\]](https://doc.rust-lang.org/nightly/std/primitive.array.html)

where T: [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + MaybeTyped + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#462)

#### fn [get](#tymethod.get)(&self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&(dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#467)

#### fn [get\_mut](#tymethod.get_mut)( &mut self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&mut (dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#472)

#### fn [len](#tymethod.len)(&self) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#477)

#### fn [iter](#tymethod.iter)(&self) -> [ArrayIter](struct.ArrayIter.html "struct bevy::reflect::array::ArrayIter")<'\_> [ⓘ](#)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#482)

#### fn [drain](#tymethod.drain)(self: [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<[\[T; N\]](https://doc.rust-lang.org/nightly/std/primitive.array.html)\>) -> [Vec](../../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>>

## Implementors

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/array.rs.html#287)

### impl [Array](trait.Array.html "trait bevy::reflect::array::Array") for [DynamicArray](struct.DynamicArray.html "struct bevy::reflect::array::DynamicArray")

{"ArrayIter<'\_>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"struct.ArrayIter.html\\" title=\\"struct bevy::reflect::array::ArrayIter\\">ArrayIter</a>&lt;'a&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"struct.ArrayIter.html\\" title=\\"struct bevy::reflect::array::ArrayIter\\">ArrayIter</a>&lt;'a&gt;</div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &amp;'a (dyn <a class=\\"trait\\" href=\\"../../prelude/trait.PartialReflect.html\\" title=\\"trait bevy::prelude::PartialReflect\\">PartialReflect</a> + 'static);</div>"}