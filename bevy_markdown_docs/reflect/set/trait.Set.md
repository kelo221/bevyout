[bevy](../../index.html)::[reflect](../index.html)::[set](index.html)

# Trait Set 

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/set.rs.html#51)

```rust
pub trait Set: PartialReflect {
    // Required methods
    fn get(
        &self,
        value: &(dyn PartialReflect + 'static),
    ) -> Option<&(dyn PartialReflect + 'static)>;
    fn len(&self) -> usize;
    fn iter(
        &self,
    ) -> Box<dyn Iterator<Item = &(dyn PartialReflect + 'static)> + '_>;
    fn drain(&mut self) -> Vec<Box<dyn PartialReflect>>;
    fn retain(
        &mut self,
        f: &mut dyn FnMut(&(dyn PartialReflect + 'static)) -> bool,
    );
    fn insert_boxed(&mut self, value: Box<dyn PartialReflect>) -> bool;
    fn remove(&mut self, value: &(dyn PartialReflect + 'static)) -> bool;
    fn contains(&self, value: &(dyn PartialReflect + 'static)) -> bool;

    // Provided methods
    fn is_empty(&self) -> bool { ... }
    fn to_dynamic_set(&self) -> DynamicSet { ... }
}
```

A trait used to power [set-like](https://doc.rust-lang.org/stable/std/collections/struct.HashSet.html) operations via [reflection](../index.html "mod bevy::reflect").

Sets contain zero or more entries of a fixed type, and correspond to types like [`HashSet`](https://doc.rust-lang.org/nightly/std/collections/hash/set/struct.HashSet.html "struct std::collections::hash::set::HashSet") and [`BTreeSet`](https://doc.rust-lang.org/nightly/alloc/collections/btree/set/struct.BTreeSet.html "struct alloc::collections::btree::set::BTreeSet"). The order of these entries is not guaranteed by this trait.

## Hashing and equality

All values are expected to return a valid hash value from [`PartialReflect::reflect_hash`](../../prelude/trait.PartialReflect.html#method.reflect_hash "method bevy::prelude::PartialReflect::reflect_hash") and be comparable using [`PartialReflect::reflect_partial_eq`](../../prelude/trait.PartialReflect.html#method.reflect_partial_eq "method bevy::prelude::PartialReflect::reflect_partial_eq"). If using the [`#[derive(Reflect)]`](../../prelude/derive.Reflect.html "derive bevy::prelude::Reflect") macro, this can be done by adding `#[reflect(Hash, PartialEq)]` to the entire struct or enum. The ordering is expected to be total, that is as if the reflected type implements the [`Eq`](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") trait. This is true even for manual implementors who do not hash or compare values, as it is still relied on by [`DynamicSet`](struct.DynamicSet.html "struct bevy::reflect::set::DynamicSet").

## Example

```rust
use bevy_reflect::{PartialReflect, set::Set};
use std::collections::HashSet;


let foo: &mut dyn Set = &mut HashSet::<u32>::new();
foo.insert_boxed(Box::new(123_u32));
assert_eq!(foo.len(), 1);

let field: &dyn PartialReflect = foo.get(&123_u32).unwrap();
assert_eq!(field.try_downcast_ref::<u32>(), Some(&123_u32));
```

## Required Methods

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/set.rs.html#55)

#### fn [get](#tymethod.get)( &self, value: &(dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&(dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)>

Returns a reference to the value.

If no value is contained, returns `None`.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/set.rs.html#58)

#### fn [len](#tymethod.len)(&self) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

Returns the number of elements in the set.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/set.rs.html#66)

#### fn [iter](#tymethod.iter)(&self) -> [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = &(dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)> + '\_>

Returns an iterator over the values of the set.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/set.rs.html#71)

#### fn [drain](#tymethod.drain)(&mut self) -> [Vec](../../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>>

Drain the values of this set to get a vector of owned values.

After calling this function, `self` will be empty.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/set.rs.html#76)

#### fn [retain](#tymethod.retain)(&mut self, f: &mut dyn [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(&(dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html))

Retain only the elements specified by the predicate.

In other words, remove all elements `e` for which `f(&e)` returns `false`.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/set.rs.html#92)

#### fn [insert\_boxed](#tymethod.insert_boxed)(&mut self, value: [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Inserts a value into the set.

If the set did not have this value present, `true` is returned. If the set did have this value present, `false` is returned.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/set.rs.html#98)

#### fn [remove](#tymethod.remove)(&mut self, value: &(dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Removes a value from the set.

If the set did have this value present, `true` is returned. If the set did not have this value present, `false` is returned.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/set.rs.html#101)

#### fn [contains](#tymethod.contains)(&self, value: &(dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Checks if the given value is contained in the set

## Provided Methods

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/set.rs.html#61)

#### fn [is\_empty](#method.is_empty)(&self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Returns `true` if the list contains no elements.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/set.rs.html#79)

#### fn [to\_dynamic\_set](#method.to_dynamic_set)(&self) -> [DynamicSet](struct.DynamicSet.html "struct bevy::reflect::set::DynamicSet")

Creates a new [`DynamicSet`](struct.DynamicSet.html "struct bevy::reflect::set::DynamicSet") from this set.

## Dyn Compatibility

This trait **is** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementations on Foreign Types

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/indexmap.rs.html#282-285)

### impl<T, S> [Set](trait.Set.html "trait bevy::reflect::set::Set") for [IndexSet](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/set/struct.IndexSet.html "struct indexmap::set::IndexSet")<T, S>

where T: [FromReflect](../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration") + [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash"), S: [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [BuildHasher](https://doc.rust-lang.org/nightly/core/hash/trait.BuildHasher.html "trait core::hash::BuildHasher") + [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/indexmap.rs.html#287)

#### fn [get](#tymethod.get)( &self, value: &(dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&(dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/indexmap.rs.html#294)

#### fn [len](#tymethod.len)(&self) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/indexmap.rs.html#298)

#### fn [iter](#tymethod.iter)(&self) -> [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = &(dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)> + '\_>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/indexmap.rs.html#303)

#### fn [drain](#tymethod.drain)(&mut self) -> [Vec](../../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/indexmap.rs.html#309)

#### fn [retain](#tymethod.retain)(&mut self, f: &mut dyn [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(&(dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html))

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/indexmap.rs.html#313)

#### fn [insert\_boxed](#tymethod.insert_boxed)(&mut self, value: [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/indexmap.rs.html#323)

#### fn [remove](#tymethod.remove)(&mut self, value: &(dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/indexmap.rs.html#334)

#### fn [contains](#tymethod.contains)(&self, value: &(dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/std/collections/hash_set.rs.html#9)

### impl<V, S> [Set](trait.Set.html "trait bevy::reflect::set::Set") for [HashSet](https://doc.rust-lang.org/nightly/std/collections/hash/set/struct.HashSet.html "struct std::collections::hash::set::HashSet")<V, S>

where V: [FromReflect](../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration") + [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash"), S: [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [BuildHasher](https://doc.rust-lang.org/nightly/core/hash/trait.BuildHasher.html "trait core::hash::BuildHasher") + [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/std/collections/hash_set.rs.html#9)

#### fn [get](#tymethod.get)( &self, value: &(dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&(dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/std/collections/hash_set.rs.html#9)

#### fn [len](#tymethod.len)(&self) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/std/collections/hash_set.rs.html#9)

#### fn [iter](#tymethod.iter)(&self) -> [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = &(dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)> + '\_>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/std/collections/hash_set.rs.html#9)

#### fn [drain](#tymethod.drain)(&mut self) -> [Vec](../../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/std/collections/hash_set.rs.html#9)

#### fn [retain](#tymethod.retain)(&mut self, f: &mut dyn [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(&(dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html))

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/std/collections/hash_set.rs.html#9)

#### fn [insert\_boxed](#tymethod.insert_boxed)(&mut self, value: [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/std/collections/hash_set.rs.html#9)

#### fn [remove](#tymethod.remove)(&mut self, value: &(dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/std/collections/hash_set.rs.html#9)

#### fn [contains](#tymethod.contains)(&self, value: &(dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

## Implementors

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/set.rs.html#196)

### impl [Set](trait.Set.html "trait bevy::reflect::set::Set") for [DynamicSet](struct.DynamicSet.html "struct bevy::reflect::set::DynamicSet")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/bevy_platform/collections/hash_set.rs.html#9)

### impl<V, S> [Set](trait.Set.html "trait bevy::reflect::set::Set") for bevy::platform::collections::[HashSet](../../platform/collections/struct.HashSet.html "struct bevy::platform::collections::HashSet")<V, S>

where V: [FromReflect](../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration") + [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash"), S: [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [BuildHasher](https://doc.rust-lang.org/nightly/core/hash/trait.BuildHasher.html "trait core::hash::BuildHasher") + [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),