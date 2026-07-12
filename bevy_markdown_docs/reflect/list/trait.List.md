[bevy](../../index.html)::[reflect](../index.html)::[list](index.html)

# Trait List 

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/list.rs.html#57)

```rust
pub trait List: PartialReflect {
    // Required methods
    fn get(&self, index: usize) -> Option<&(dyn PartialReflect + 'static)>;
    fn get_mut(
        &mut self,
        index: usize,
    ) -> Option<&mut (dyn PartialReflect + 'static)>;
    fn insert(&mut self, index: usize, element: Box<dyn PartialReflect>);
    fn remove(&mut self, index: usize) -> Box<dyn PartialReflect>;
    fn len(&self) -> usize;
    fn iter(&self) -> ListIter<'_> ⓘ;
    fn drain(&mut self) -> Vec<Box<dyn PartialReflect>>;

    // Provided methods
    fn push(&mut self, value: Box<dyn PartialReflect>) { ... }
    fn pop(&mut self) -> Option<Box<dyn PartialReflect>> { ... }
    fn is_empty(&self) -> bool { ... }
    fn to_dynamic_list(&self) -> DynamicList { ... }
    fn get_represented_list_info(&self) -> Option<&'static ListInfo> { ... }
}
```

A trait used to power [list-like](https://doc.rust-lang.org/book/ch08-01-vectors.html) operations via [reflection](../index.html "mod bevy::reflect").

This corresponds to types, like [`Vec`](../../prelude/struct.Vec.html "struct bevy::prelude::Vec"), which contain an ordered sequence of elements that implement [`Reflect`](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect").

Unlike the [`Array`](../array/trait.Array.html "trait bevy::reflect::array::Array") trait, implementors of this trait are not expected to maintain a constant length. Methods like [insertion](trait.List.html#tymethod.insert "method bevy::reflect::list::List::insert") and [removal](trait.List.html#tymethod.remove "method bevy::reflect::list::List::remove") explicitly allow for their internal size to change.

[`push`](trait.List.html#method.push "method bevy::reflect::list::List::push") and [`pop`](trait.List.html#method.pop "method bevy::reflect::list::List::pop") have default implementations, however it will generally be more performant to implement them manually as the default implementation uses a very naive approach to find the correct position.

This trait expects its elements to be ordered linearly from front to back. The _front_ element starts at index 0 with the _back_ element ending at the largest index. This contract above should be upheld by any manual implementors.

Due to the [type-erasing](https://doc.rust-lang.org/book/ch17-02-trait-objects.html) nature of the reflection API as a whole, this trait does not make any guarantees that the implementor’s elements are homogeneous (i.e. all the same type).

## Example

```rust
use bevy_reflect::{PartialReflect, Reflect, list::List};

let foo: &mut dyn List = &mut vec![123_u32, 456_u32, 789_u32];
assert_eq!(foo.len(), 3);

let last_field: Box<dyn PartialReflect> = foo.pop().unwrap();
assert_eq!(last_field.try_downcast_ref::<u32>(), Some(&789));
```

## Required Methods

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/list.rs.html#59)

#### fn [get](#tymethod.get)(&self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&(dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)>

Returns a reference to the element at `index`, or `None` if out of bounds.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/list.rs.html#62)

#### fn [get\_mut](#tymethod.get_mut)( &mut self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&mut (dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)>

Returns a mutable reference to the element at `index`, or `None` if out of bounds.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/list.rs.html#69)

#### fn [insert](#tymethod.insert)(&mut self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), element: [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>)

Inserts an element at position `index` within the list, shifting all elements after it towards the back of the list.

##### Panics

Panics if `index > len`.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/list.rs.html#76)

#### fn [remove](#tymethod.remove)(&mut self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>

Removes and returns the element at position `index` within the list, shifting all elements before it towards the front of the list.

##### Panics

Panics if `index` is out of bounds.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/list.rs.html#93)

#### fn [len](#tymethod.len)(&self) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

Returns the number of elements in the list.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/list.rs.html#101)

#### fn [iter](#tymethod.iter)(&self) -> [ListIter](struct.ListIter.html "struct bevy::reflect::list::ListIter")<'\_> [ⓘ](#)

Returns an iterator over the list.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/list.rs.html#107)

#### fn [drain](#tymethod.drain)(&mut self) -> [Vec](../../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>>

Drain the elements of this list to get a vector of owned values.

After calling this function, `self` will be empty. The order of items in the returned [`Vec`](../../prelude/struct.Vec.html "struct bevy::prelude::Vec") will match the order of items in `self`.

## Provided Methods

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/list.rs.html#79)

#### fn [push](#method.push)(&mut self, value: [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>)

Appends an element to the _back_ of the list.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/list.rs.html#84)

#### fn [pop](#method.pop)(&mut self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>>

Removes the _back_ element from the list and returns it, or [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") if it is empty.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/list.rs.html#96)

#### fn [is\_empty](#method.is_empty)(&self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Returns `true` if the collection contains no elements.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/list.rs.html#110)

#### fn [to\_dynamic\_list](#method.to_dynamic_list)(&self) -> [DynamicList](struct.DynamicList.html "struct bevy::reflect::list::DynamicList")

Creates a new [`DynamicList`](struct.DynamicList.html "struct bevy::reflect::list::DynamicList") from this list.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/list.rs.html#118)

#### fn [get\_represented\_list\_info](#method.get_represented_list_info)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [ListInfo](struct.ListInfo.html "struct bevy::reflect::list::ListInfo")\>

Will return `None` if [`TypeInfo`](../enum.TypeInfo.html "enum bevy::reflect::TypeInfo") is not available.

## Dyn Compatibility

This trait **is** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementations on Foreign Types

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/borrow.rs.html#144-145)

### impl<T> [List](trait.List.html "trait bevy::reflect::list::List") for [Cow](https://doc.rust-lang.org/nightly/alloc/borrow/enum.Cow.html "enum alloc::borrow::Cow")<'static, [\[T\]](https://doc.rust-lang.org/nightly/std/primitive.slice.html)\>

where T: [FromReflect](../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/borrow.rs.html#147)

#### fn [get](#tymethod.get)(&self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&(dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/borrow.rs.html#151)

#### fn [get\_mut](#tymethod.get_mut)( &mut self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&mut (dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/borrow.rs.html#157)

#### fn [insert](#tymethod.insert)(&mut self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), element: [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/borrow.rs.html#167)

#### fn [remove](#tymethod.remove)(&mut self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/borrow.rs.html#171)

#### fn [push](#method.push)(&mut self, value: [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/borrow.rs.html#181)

#### fn [pop](#method.pop)(&mut self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/borrow.rs.html#187)

#### fn [len](#tymethod.len)(&self) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/borrow.rs.html#191)

#### fn [iter](#tymethod.iter)(&self) -> [ListIter](struct.ListIter.html "struct bevy::reflect::list::ListIter")<'\_> [ⓘ](#)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/borrow.rs.html#195)

#### fn [drain](#tymethod.drain)(&mut self) -> [Vec](../../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/smallvec.rs.html#14-16)

### impl<T> [List](trait.List.html "trait bevy::reflect::list::List") for [SmallVec](https://docs.rs/smallvec/1.15.1/x86_64-unknown-linux-gnu/smallvec/struct.SmallVec.html "struct smallvec::SmallVec")<T>

where T: [Array](https://docs.rs/smallvec/1.15.1/x86_64-unknown-linux-gnu/smallvec/trait.Array.html "trait smallvec::Array") + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), <T as [Array](https://docs.rs/smallvec/1.15.1/x86_64-unknown-linux-gnu/smallvec/trait.Array.html "trait smallvec::Array")\>::[Item](https://docs.rs/smallvec/1.15.1/x86_64-unknown-linux-gnu/smallvec/trait.Array.html#associatedtype.Item "type smallvec::Array::Item"): [FromReflect](../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/smallvec.rs.html#18)

#### fn [get](#tymethod.get)(&self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&(dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/smallvec.rs.html#26)

#### fn [get\_mut](#tymethod.get_mut)( &mut self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&mut (dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/smallvec.rs.html#34)

#### fn [insert](#tymethod.insert)(&mut self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), value: [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/smallvec.rs.html#46)

#### fn [remove](#tymethod.remove)(&mut self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/smallvec.rs.html#50)

#### fn [push](#method.push)(&mut self, value: [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/smallvec.rs.html#62)

#### fn [pop](#method.pop)(&mut self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/smallvec.rs.html#67)

#### fn [len](#tymethod.len)(&self) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/smallvec.rs.html#71)

#### fn [iter](#tymethod.iter)(&self) -> [ListIter](struct.ListIter.html "struct bevy::reflect::list::ListIter")<'\_> [ⓘ](#)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/smallvec.rs.html#75)

#### fn [drain](#tymethod.drain)(&mut self) -> [Vec](../../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/collections/vec_deque.rs.html#10-17)

### impl<T> [List](trait.List.html "trait bevy::reflect::list::List") for [VecDeque](https://doc.rust-lang.org/nightly/alloc/collections/vec_deque/struct.VecDeque.html "struct alloc::collections::vec_deque::VecDeque")<T>

where T: [FromReflect](../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/collections/vec_deque.rs.html#10-17)

#### fn [get](#tymethod.get)(&self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&(dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/collections/vec_deque.rs.html#10-17)

#### fn [get\_mut](#tymethod.get_mut)( &mut self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&mut (dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/collections/vec_deque.rs.html#10-17)

#### fn [insert](#tymethod.insert)(&mut self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), value: [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/collections/vec_deque.rs.html#10-17)

#### fn [remove](#tymethod.remove)(&mut self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/collections/vec_deque.rs.html#10-17)

#### fn [push](#method.push)(&mut self, value: [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/collections/vec_deque.rs.html#10-17)

#### fn [pop](#method.pop)(&mut self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/collections/vec_deque.rs.html#10-17)

#### fn [len](#tymethod.len)(&self) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/collections/vec_deque.rs.html#10-17)

#### fn [iter](#tymethod.iter)(&self) -> [ListIter](struct.ListIter.html "struct bevy::reflect::list::ListIter")<'\_> [ⓘ](#)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/collections/vec_deque.rs.html#10-17)

#### fn [drain](#tymethod.drain)(&mut self) -> [Vec](../../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>>

## Implementors

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/list.rs.html#215)

### impl [List](trait.List.html "trait bevy::reflect::list::List") for [DynamicList](struct.DynamicList.html "struct bevy::reflect::list::DynamicList")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/vec.rs.html#10-17)

### impl<T> [List](trait.List.html "trait bevy::reflect::list::List") for [Vec](../../prelude/struct.Vec.html "struct bevy::prelude::Vec")<T>

where T: [FromReflect](../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"),

{"ListIter<'\_>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"struct.ListIter.html\\" title=\\"struct bevy::reflect::list::ListIter\\">ListIter</a>&lt;'a&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"struct.ListIter.html\\" title=\\"struct bevy::reflect::list::ListIter\\">ListIter</a>&lt;'a&gt;</div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &amp;'a (dyn <a class=\\"trait\\" href=\\"../../prelude/trait.PartialReflect.html\\" title=\\"trait bevy::prelude::PartialReflect\\">PartialReflect</a> + 'static);</div>"}