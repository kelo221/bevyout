[bevy](../../index.html)::[reflect](../index.html)::[map](index.html)

# Trait Map 

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/map.rs.html#51)

```rust
pub trait Map: PartialReflect {
    // Required methods
    fn get(
        &self,
        key: &(dyn PartialReflect + 'static),
    ) -> Option<&(dyn PartialReflect + 'static)>;
    fn get_mut(
        &mut self,
        key: &(dyn PartialReflect + 'static),
    ) -> Option<&mut (dyn PartialReflect + 'static)>;
    fn len(&self) -> usize;
    fn iter(
        &self,
    ) -> Box<dyn Iterator<Item = (&(dyn PartialReflect + 'static), &(dyn PartialReflect + 'static))> + '_>;
    fn drain(
        &mut self,
    ) -> Vec<(Box<dyn PartialReflect>, Box<dyn PartialReflect>)>;
    fn retain(
        &mut self,
        f: &mut dyn FnMut(&(dyn PartialReflect + 'static), &mut (dyn PartialReflect + 'static)) -> bool,
    );
    fn insert_boxed(
        &mut self,
        key: Box<dyn PartialReflect>,
        value: Box<dyn PartialReflect>,
    ) -> Option<Box<dyn PartialReflect>>;
    fn remove(
        &mut self,
        key: &(dyn PartialReflect + 'static),
    ) -> Option<Box<dyn PartialReflect>>;

    // Provided methods
    fn is_empty(&self) -> bool { ... }
    fn to_dynamic_map(&self) -> DynamicMap { ... }
    fn get_represented_map_info(&self) -> Option<&'static MapInfo> { ... }
}
```

A trait used to power [map-like](https://doc.rust-lang.org/book/ch08-03-hash-maps.html) operations via [reflection](../index.html "mod bevy::reflect").

Maps contain zero or more entries of a key and its associated value, and correspond to types like [`HashMap`](https://doc.rust-lang.org/nightly/std/collections/hash/map/struct.HashMap.html "struct std::collections::hash::map::HashMap") and [`BTreeMap`](https://doc.rust-lang.org/nightly/alloc/collections/btree/map/struct.BTreeMap.html "struct alloc::collections::btree::map::BTreeMap"). The order of these entries is not guaranteed by this trait.

## Hashing and equality

All keys are expected to return a valid hash value from [`PartialReflect::reflect_hash`](../../prelude/trait.PartialReflect.html#method.reflect_hash "method bevy::prelude::PartialReflect::reflect_hash") and be comparable using [`PartialReflect::reflect_partial_eq`](../../prelude/trait.PartialReflect.html#method.reflect_partial_eq "method bevy::prelude::PartialReflect::reflect_partial_eq"). If using the [`#[derive(Reflect)]`](../../prelude/derive.Reflect.html "derive bevy::prelude::Reflect") macro, this can be done by adding `#[reflect(Hash, PartialEq)]` to the entire struct or enum. The ordering is expected to be total, that is as if the reflected type implements the [`Eq`](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") trait. This is true even for manual implementors who do not hash or compare values, as it is still relied on by [`DynamicMap`](struct.DynamicMap.html "struct bevy::reflect::map::DynamicMap").

## Example

```rust
use bevy_reflect::{PartialReflect, Reflect, map::Map};
use std::collections::HashMap;


let foo: &mut dyn Map = &mut HashMap::<u32, bool>::new();
foo.insert_boxed(Box::new(123_u32), Box::new(true));
assert_eq!(foo.len(), 1);

let field: &dyn PartialReflect = foo.get(&123_u32).unwrap();
assert_eq!(field.try_downcast_ref::<bool>(), Some(&true));
```

## Required Methods

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/map.rs.html#55)

#### fn [get](#tymethod.get)( &self, key: &(dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&(dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)>

Returns a reference to the value associated with the given key.

If no value is associated with `key`, returns `None`.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/map.rs.html#60)

#### fn [get\_mut](#tymethod.get_mut)( &mut self, key: &(dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&mut (dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)>

Returns a mutable reference to the value associated with the given key.

If no value is associated with `key`, returns `None`.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/map.rs.html#63)

#### fn [len](#tymethod.len)(&self) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

Returns the number of elements in the map.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/map.rs.html#71)

#### fn [iter](#tymethod.iter)( &self, ) -> [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = (&(dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static), &(dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static))> + '\_>

Returns an iterator over the key-value pairs of the map.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/map.rs.html#76)

#### fn [drain](#tymethod.drain)(&mut self) -> [Vec](../../prelude/struct.Vec.html "struct bevy::prelude::Vec")<([Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>, [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>)>

Drain the key-value pairs of this map to get a vector of owned values.

After calling this function, `self` will be empty.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/map.rs.html#81)

#### fn [retain](#tymethod.retain)( &mut self, f: &mut dyn [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(&(dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static), &mut (dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html), )

Retain only the elements specified by the predicate.

In other words, remove all pairs `(k, v)` such that `f(&k, &mut v)` returns `false`.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/map.rs.html#97-101)

#### fn [insert\_boxed](#tymethod.insert_boxed)( &mut self, key: [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>, value: [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>, ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>>

Inserts a key-value pair into the map.

If the map did not have this key present, `None` is returned. If the map did have this key present, the value is updated, and the old value is returned.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/map.rs.html#107)

#### fn [remove](#tymethod.remove)( &mut self, key: &(dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>>

Removes an entry from the map.

If the map did not have this key present, `None` is returned. If the map did have this key present, the removed value is returned.

## Provided Methods

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/map.rs.html#66)

#### fn [is\_empty](#method.is_empty)(&self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Returns `true` if the list contains no elements.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/map.rs.html#84)

#### fn [to\_dynamic\_map](#method.to_dynamic_map)(&self) -> [DynamicMap](struct.DynamicMap.html "struct bevy::reflect::map::DynamicMap")

Creates a new [`DynamicMap`](struct.DynamicMap.html "struct bevy::reflect::map::DynamicMap") from this map.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/map.rs.html#110)

#### fn [get\_represented\_map\_info](#method.get_represented_map_info)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [MapInfo](struct.MapInfo.html "struct bevy::reflect::map::MapInfo")\>

Will return `None` if [`TypeInfo`](../enum.TypeInfo.html "enum bevy::reflect::TypeInfo") is not available.

## Dyn Compatibility

This trait **is** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementations on Foreign Types

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/std/collections/hash_map.rs.html#12)

### impl<K, V, S> [Map](trait.Map.html "trait bevy::reflect::map::Map") for [HashMap](https://doc.rust-lang.org/nightly/std/collections/hash/map/struct.HashMap.html "struct std::collections::hash::map::HashMap")<K, V, S>

where K: [FromReflect](../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration") + [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash"), V: [FromReflect](../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), S: [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [BuildHasher](https://doc.rust-lang.org/nightly/core/hash/trait.BuildHasher.html "trait core::hash::BuildHasher") + [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/std/collections/hash_map.rs.html#12)

#### fn [get](#tymethod.get)( &self, key: &(dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&(dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/std/collections/hash_map.rs.html#12)

#### fn [get\_mut](#tymethod.get_mut)( &mut self, key: &(dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&mut (dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/std/collections/hash_map.rs.html#12)

#### fn [len](#tymethod.len)(&self) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/std/collections/hash_map.rs.html#12)

#### fn [iter](#tymethod.iter)( &self, ) -> [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = (&(dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static), &(dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static))> + '\_>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/std/collections/hash_map.rs.html#12)

#### fn [drain](#tymethod.drain)(&mut self) -> [Vec](../../prelude/struct.Vec.html "struct bevy::prelude::Vec")<([Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>, [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>)>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/std/collections/hash_map.rs.html#12)

#### fn [retain](#tymethod.retain)( &mut self, f: &mut dyn [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(&(dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static), &mut (dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html), )

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/std/collections/hash_map.rs.html#12)

#### fn [to\_dynamic\_map](#method.to_dynamic_map)(&self) -> [DynamicMap](struct.DynamicMap.html "struct bevy::reflect::map::DynamicMap")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/std/collections/hash_map.rs.html#12)

#### fn [insert\_boxed](#tymethod.insert_boxed)( &mut self, key: [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>, value: [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>, ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/std/collections/hash_map.rs.html#12)

#### fn [remove](#tymethod.remove)( &mut self, key: &(dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/indexmap.rs.html#17-21)

### impl<K, V, S> [Map](trait.Map.html "trait bevy::reflect::map::Map") for [IndexMap](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/map/struct.IndexMap.html "struct indexmap::map::IndexMap")<K, V, S>

where K: [FromReflect](../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration") + [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash"), V: [FromReflect](../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), S: [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [BuildHasher](https://doc.rust-lang.org/nightly/core/hash/trait.BuildHasher.html "trait core::hash::BuildHasher") + [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/indexmap.rs.html#23)

#### fn [get](#tymethod.get)( &self, key: &(dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&(dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/indexmap.rs.html#29)

#### fn [get\_mut](#tymethod.get_mut)( &mut self, key: &(dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&mut (dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/indexmap.rs.html#35)

#### fn [len](#tymethod.len)(&self) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/indexmap.rs.html#39)

#### fn [iter](#tymethod.iter)( &self, ) -> [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = (&(dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static), &(dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static))> + '\_>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/indexmap.rs.html#46)

#### fn [drain](#tymethod.drain)(&mut self) -> [Vec](../../prelude/struct.Vec.html "struct bevy::prelude::Vec")<([Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>, [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>)>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/indexmap.rs.html#57)

#### fn [retain](#tymethod.retain)( &mut self, f: &mut dyn [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(&(dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static), &mut (dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html), )

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/indexmap.rs.html#61)

#### fn [to\_dynamic\_map](#method.to_dynamic_map)(&self) -> [DynamicMap](struct.DynamicMap.html "struct bevy::reflect::map::DynamicMap")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/indexmap.rs.html#76-80)

#### fn [insert\_boxed](#tymethod.insert_boxed)( &mut self, key: [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>, value: [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>, ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/indexmap.rs.html#97)

#### fn [remove](#tymethod.remove)( &mut self, key: &(dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/collections/btree/map.rs.html#16-19)

### impl<K, V> [Map](trait.Map.html "trait bevy::reflect::map::Map") for [BTreeMap](https://doc.rust-lang.org/nightly/alloc/collections/btree/map/struct.BTreeMap.html "struct alloc::collections::btree::map::BTreeMap")<K, V>

where K: [FromReflect](../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration") + [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + [Ord](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html "trait core::cmp::Ord"), V: [FromReflect](../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/collections/btree/map.rs.html#21)

#### fn [get](#tymethod.get)( &self, key: &(dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&(dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/collections/btree/map.rs.html#27)

#### fn [get\_mut](#tymethod.get_mut)( &mut self, key: &(dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&mut (dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/collections/btree/map.rs.html#33)

#### fn [len](#tymethod.len)(&self) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/collections/btree/map.rs.html#37)

#### fn [iter](#tymethod.iter)( &self, ) -> [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = (&(dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static), &(dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static))> + '\_>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/collections/btree/map.rs.html#44)

#### fn [drain](#tymethod.drain)(&mut self) -> [Vec](../../prelude/struct.Vec.html "struct bevy::prelude::Vec")<([Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>, [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>)>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/collections/btree/map.rs.html#58)

#### fn [retain](#tymethod.retain)( &mut self, f: &mut dyn [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(&(dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static), &mut (dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html), )

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/collections/btree/map.rs.html#62-66)

#### fn [insert\_boxed](#tymethod.insert_boxed)( &mut self, key: [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>, value: [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>, ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/collections/btree/map.rs.html#83)

#### fn [remove](#tymethod.remove)( &mut self, key: &(dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>>

## Implementors

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/map.rs.html#266)

### impl [Map](trait.Map.html "trait bevy::reflect::map::Map") for [DynamicMap](struct.DynamicMap.html "struct bevy::reflect::map::DynamicMap")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/bevy_platform/collections/hash_map.rs.html#12)

### impl<K, V, S> [Map](trait.Map.html "trait bevy::reflect::map::Map") for bevy::platform::collections::[HashMap](../../platform/collections/struct.HashMap.html "struct bevy::platform::collections::HashMap")<K, V, S>

where K: [FromReflect](../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration") + [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash"), V: [FromReflect](../../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](../trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), S: [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [BuildHasher](https://doc.rust-lang.org/nightly/core/hash/trait.BuildHasher.html "trait core::hash::BuildHasher") + [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),