[bevy](../index.html)::[utils](index.html)

# Trait TypeIdMapExt 

[Source](https://docs.rs/bevy_utils/0.19.0/x86_64-unknown-linux-gnu/src/bevy_utils/map.rs.html#69)

```rust
pub trait TypeIdMapExt<V> {
    // Required methods
    fn insert_type<T>(&mut self, v: V) -> Option<V>
       where T: 'static + ?Sized;
    fn get_type<T>(&self) -> Option<&V>
       where T: 'static + ?Sized;
    fn get_type_mut<T>(&mut self) -> Option<&mut V>
       where T: 'static + ?Sized;
    fn remove_type<T>(&mut self) -> Option<V>
       where T: 'static + ?Sized;
    fn entry_type<T>(&mut self) -> Entry<'_, TypeId, V>
       where T: 'static + ?Sized;
}
```

Extension trait to make use of [`TypeIdMap`](type.TypeIdMap.html "type bevy::utils::TypeIdMap") more ergonomic.

Each function on this trait is a trivial wrapper for a function on [`IndexMap`](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/map/struct.IndexMap.html "struct indexmap::map::IndexMap"), replacing a `TypeId` key with a generic parameter `T`.

## Examples

```rust
use bevy_utils::TypeIdMapExt;

struct MyType;

// Using the built-in `HashMap` functions requires manually looking up `TypeId`s.
let mut map = TypeIdMap::default();
map.insert(TypeId::of::<MyType>(), 7);
assert_eq!(map.get(&TypeId::of::<MyType>()), Some(&7));

// Using `TypeIdMapExt` functions does the lookup for you.
map.insert_type::<MyType>(7);
assert_eq!(map.get_type::<MyType>(), Some(&7));
```

## Required Methods

[Source](https://docs.rs/bevy_utils/0.19.0/x86_64-unknown-linux-gnu/src/bevy_utils/map.rs.html#74)

#### fn [insert\_type](#tymethod.insert_type)<T>(&mut self, v: V) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<V>

where T: 'static + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Inserts a value for the type `T`.

If the map did not previously contain this key then [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") is returned, otherwise the value for this key is updated and the old value returned.

[Source](https://docs.rs/bevy_utils/0.19.0/x86_64-unknown-linux-gnu/src/bevy_utils/map.rs.html#77)

#### fn [get\_type](#tymethod.get_type)<T>(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[&V](https://doc.rust-lang.org/nightly/std/primitive.reference.html)\>

where T: 'static + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Returns a reference to the value for type `T`, if one exists.

[Source](https://docs.rs/bevy_utils/0.19.0/x86_64-unknown-linux-gnu/src/bevy_utils/map.rs.html#80)

#### fn [get\_type\_mut](#tymethod.get_type_mut)<T>(&mut self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[&mut V](https://doc.rust-lang.org/nightly/std/primitive.reference.html)\>

where T: 'static + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Returns a mutable reference to the value for type `T`, if one exists.

[Source](https://docs.rs/bevy_utils/0.19.0/x86_64-unknown-linux-gnu/src/bevy_utils/map.rs.html#84)

#### fn [remove\_type](#tymethod.remove_type)<T>(&mut self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<V>

where T: 'static + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Removes type `T` from the map, returning the value for this key if it was previously present.

[Source](https://docs.rs/bevy_utils/0.19.0/x86_64-unknown-linux-gnu/src/bevy_utils/map.rs.html#87)

#### fn [entry\_type](#tymethod.entry_type)<T>(&mut self) -> [Entry](enum.TypeIdMapEntry.html "enum bevy::utils::TypeIdMapEntry")<'\_, [TypeId](https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html "struct core::any::TypeId"), V>

where T: 'static + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Gets the type `T`’s entry in the map for in-place manipulation.

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementations on Foreign Types

[Source](https://docs.rs/bevy_utils/0.19.0/x86_64-unknown-linux-gnu/src/bevy_utils/map.rs.html#90)

### impl<V> [TypeIdMapExt](trait.TypeIdMapExt.html "trait bevy::utils::TypeIdMapExt")<V> for [IndexMap](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/map/struct.IndexMap.html "struct indexmap::map::IndexMap")<[TypeId](https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html "struct core::any::TypeId"), V, [NoOpHash](../platform/hash/struct.NoOpHash.html "struct bevy::platform::hash::NoOpHash")\>

[Source](https://docs.rs/bevy_utils/0.19.0/x86_64-unknown-linux-gnu/src/bevy_utils/map.rs.html#92)

#### fn [insert\_type](#tymethod.insert_type)<T>(&mut self, v: V) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<V>

where T: 'static + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/bevy_utils/0.19.0/x86_64-unknown-linux-gnu/src/bevy_utils/map.rs.html#97)

#### fn [get\_type](#tymethod.get_type)<T>(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[&V](https://doc.rust-lang.org/nightly/std/primitive.reference.html)\>

where T: 'static + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/bevy_utils/0.19.0/x86_64-unknown-linux-gnu/src/bevy_utils/map.rs.html#102)

#### fn [get\_type\_mut](#tymethod.get_type_mut)<T>(&mut self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[&mut V](https://doc.rust-lang.org/nightly/std/primitive.reference.html)\>

where T: 'static + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/bevy_utils/0.19.0/x86_64-unknown-linux-gnu/src/bevy_utils/map.rs.html#107)

#### fn [remove\_type](#tymethod.remove_type)<T>(&mut self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<V>

where T: 'static + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/bevy_utils/0.19.0/x86_64-unknown-linux-gnu/src/bevy_utils/map.rs.html#112)

#### fn [entry\_type](#tymethod.entry_type)<T>(&mut self) -> [Entry](enum.TypeIdMapEntry.html "enum bevy::utils::TypeIdMapEntry")<'\_, [TypeId](https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html "struct core::any::TypeId"), V>

where T: 'static + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

## Implementors