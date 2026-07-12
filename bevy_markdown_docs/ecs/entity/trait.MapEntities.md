[bevy](../../index.html)::[ecs](../index.html)::[entity](index.html)

# Trait MapEntities 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/map_entities.rs.html#54)

```rust
pub trait MapEntities {
    // Required method
    fn map_entities<E>(&mut self, entity_mapper: &mut E)
       where E: EntityMapper;
}
```

Operation to map all contained [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity") fields in a type to new values.

As entity IDs are valid only for the [`World`](../../prelude/struct.World.html "struct bevy::prelude::World") they’re sourced from, using [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity") as references in components copied from another world will be invalid. This trait allows defining custom mappings for these references via [`EntityMappers`](../../prelude/trait.EntityMapper.html "trait bevy::prelude::EntityMapper"), which inject the entity mapping strategy between your `MapEntities` type and the current world (usually by using an [`EntityHashMap<Entity>`](struct.EntityHashMap.html "struct bevy::ecs::entity::EntityHashMap") between source entities and entities in the current world).

Components use [`Component::map_entities`](../../prelude/trait.Component.html#method.map_entities "associated function bevy::prelude::Component::map_entities") to map entities in the context of scenes and entity cloning, which generally uses [`MapEntities`](trait.MapEntities.html "trait bevy::ecs::entity::MapEntities") internally to map each field (see those docs for usage).

### Example

```rust
use bevy_ecs::prelude::*;
use bevy_ecs::entity::MapEntities;

#[derive(Component)]
struct Spring {
    a: Entity,
    b: Entity,
}

impl MapEntities for Spring {
    fn map_entities<M: EntityMapper>(&mut self, entity_mapper: &mut M) {
        self.a = entity_mapper.get_mapped(self.a);
        self.b = entity_mapper.get_mapped(self.b);
    }
}
```

## Required Methods

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/map_entities.rs.html#59)

#### fn [map\_entities](#tymethod.map_entities)<E>(&mut self, entity\_mapper: [&mut E](https://doc.rust-lang.org/nightly/std/primitive.reference.html))

where E: [EntityMapper](../../prelude/trait.EntityMapper.html "trait bevy::prelude::EntityMapper"),

Updates all [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity") references stored inside using `entity_mapper`.

Implementers should look up any and all [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity") values stored within `self` and update them to the mapped values via `entity_mapper`.

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementations on Foreign Types

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/map_entities.rs.html#196)

### impl [MapEntities](trait.MapEntities.html "trait bevy::ecs::entity::MapEntities") for [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/map_entities.rs.html#197)

#### fn [map\_entities](#tymethod.map_entities)<E>(&mut self, \_entity\_mapper: [&mut E](https://doc.rust-lang.org/nightly/std/primitive.reference.html))

where E: [EntityMapper](../../prelude/trait.EntityMapper.html "trait bevy::prelude::EntityMapper"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/map_entities.rs.html#103-104)

### impl<K, V, S> [MapEntities](trait.MapEntities.html "trait bevy::ecs::entity::MapEntities") for [IndexMap](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/map/struct.IndexMap.html "struct indexmap::map::IndexMap")<K, V, S>

where K: [MapEntities](trait.MapEntities.html "trait bevy::ecs::entity::MapEntities") + [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash"), V: [MapEntities](trait.MapEntities.html "trait bevy::ecs::entity::MapEntities"), S: [BuildHasher](https://doc.rust-lang.org/nightly/core/hash/trait.BuildHasher.html "trait core::hash::BuildHasher") + [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/map_entities.rs.html#106)

#### fn [map\_entities](#tymethod.map_entities)<E>(&mut self, entity\_mapper: [&mut E](https://doc.rust-lang.org/nightly/std/primitive.reference.html))

where E: [EntityMapper](../../prelude/trait.EntityMapper.html "trait bevy::prelude::EntityMapper"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/map_entities.rs.html#139)

### impl<K, V> [MapEntities](trait.MapEntities.html "trait bevy::ecs::entity::MapEntities") for [BTreeMap](https://doc.rust-lang.org/nightly/alloc/collections/btree/map/struct.BTreeMap.html "struct alloc::collections::btree::map::BTreeMap")<K, V>

where K: [MapEntities](trait.MapEntities.html "trait bevy::ecs::entity::MapEntities") + [Ord](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html "trait core::cmp::Ord"), V: [MapEntities](trait.MapEntities.html "trait bevy::ecs::entity::MapEntities"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/map_entities.rs.html#140)

#### fn [map\_entities](#tymethod.map_entities)<E>(&mut self, entity\_mapper: [&mut E](https://doc.rust-lang.org/nightly/std/primitive.reference.html))

where E: [EntityMapper](../../prelude/trait.EntityMapper.html "trait bevy::prelude::EntityMapper"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/map_entities.rs.html#188)

### impl<T, A> [MapEntities](trait.MapEntities.html "trait bevy::ecs::entity::MapEntities") for [SmallVec](https://docs.rs/smallvec/1.15.1/x86_64-unknown-linux-gnu/smallvec/struct.SmallVec.html "struct smallvec::SmallVec")<A>

where T: [MapEntities](trait.MapEntities.html "trait bevy::ecs::entity::MapEntities"), A: [Array](https://docs.rs/smallvec/1.15.1/x86_64-unknown-linux-gnu/smallvec/trait.Array.html "trait smallvec::Array")<Item = T>,

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/map_entities.rs.html#189)

#### fn [map\_entities](#tymethod.map_entities)<E>(&mut self, entity\_mapper: [&mut E](https://doc.rust-lang.org/nightly/std/primitive.reference.html))

where E: [EntityMapper](../../prelude/trait.EntityMapper.html "trait bevy::prelude::EntityMapper"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/map_entities.rs.html#118)

### impl<T, S> [MapEntities](trait.MapEntities.html "trait bevy::ecs::entity::MapEntities") for [IndexSet](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/set/struct.IndexSet.html "struct indexmap::set::IndexSet")<T, S>

where T: [MapEntities](trait.MapEntities.html "trait bevy::ecs::entity::MapEntities") + [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash"), S: [BuildHasher](https://doc.rust-lang.org/nightly/core/hash/trait.BuildHasher.html "trait core::hash::BuildHasher") + [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/map_entities.rs.html#119)

#### fn [map\_entities](#tymethod.map_entities)<E>(&mut self, entity\_mapper: [&mut E](https://doc.rust-lang.org/nightly/std/primitive.reference.html))

where E: [EntityMapper](../../prelude/trait.EntityMapper.html "trait bevy::prelude::EntityMapper"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/map_entities.rs.html#164)

### impl<T, const N: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\> [MapEntities](trait.MapEntities.html "trait bevy::ecs::entity::MapEntities") for [\[T; N\]](https://doc.rust-lang.org/nightly/std/primitive.array.html)

where T: [MapEntities](trait.MapEntities.html "trait bevy::ecs::entity::MapEntities"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/map_entities.rs.html#165)

#### fn [map\_entities](#tymethod.map_entities)<E>(&mut self, entity\_mapper: [&mut E](https://doc.rust-lang.org/nightly/std/primitive.reference.html))

where E: [EntityMapper](../../prelude/trait.EntityMapper.html "trait bevy::prelude::EntityMapper"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/map_entities.rs.html#152)

### impl<T> [MapEntities](trait.MapEntities.html "trait bevy::ecs::entity::MapEntities") for [BTreeSet](https://doc.rust-lang.org/nightly/alloc/collections/btree/set/struct.BTreeSet.html "struct alloc::collections::btree::set::BTreeSet")<T>

where T: [MapEntities](trait.MapEntities.html "trait bevy::ecs::entity::MapEntities") + [Ord](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html "trait core::cmp::Ord"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/map_entities.rs.html#153)

#### fn [map\_entities](#tymethod.map_entities)<E>(&mut self, entity\_mapper: [&mut E](https://doc.rust-lang.org/nightly/std/primitive.reference.html))

where E: [EntityMapper](../../prelude/trait.EntityMapper.html "trait bevy::prelude::EntityMapper"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/map_entities.rs.html#68)

### impl<T> [MapEntities](trait.MapEntities.html "trait bevy::ecs::entity::MapEntities") for [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<T>

where T: [MapEntities](trait.MapEntities.html "trait bevy::ecs::entity::MapEntities"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/map_entities.rs.html#69)

#### fn [map\_entities](#tymethod.map_entities)<E>(&mut self, entity\_mapper: [&mut E](https://doc.rust-lang.org/nightly/std/primitive.reference.html))

where E: [EntityMapper](../../prelude/trait.EntityMapper.html "trait bevy::prelude::EntityMapper"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/map_entities.rs.html#180)

### impl<T> [MapEntities](trait.MapEntities.html "trait bevy::ecs::entity::MapEntities") for [VecDeque](https://doc.rust-lang.org/nightly/alloc/collections/vec_deque/struct.VecDeque.html "struct alloc::collections::vec_deque::VecDeque")<T>

where T: [MapEntities](trait.MapEntities.html "trait bevy::ecs::entity::MapEntities"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/map_entities.rs.html#181)

#### fn [map\_entities](#tymethod.map_entities)<E>(&mut self, entity\_mapper: [&mut E](https://doc.rust-lang.org/nightly/std/primitive.reference.html))

where E: [EntityMapper](../../prelude/trait.EntityMapper.html "trait bevy::prelude::EntityMapper"),

## Implementors

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/map_entities.rs.html#62)

### impl [MapEntities](trait.MapEntities.html "trait bevy::ecs::entity::MapEntities") for [Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/map_entities.rs.html#130)

### impl [MapEntities](trait.MapEntities.html "trait bevy::ecs::entity::MapEntities") for [EntityIndexSet](struct.EntityIndexSet.html "struct bevy::ecs::entity::EntityIndexSet")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/map_entities.rs.html#76-77)

### impl<K, V, S> [MapEntities](trait.MapEntities.html "trait bevy::ecs::entity::MapEntities") for [HashMap](../../platform/collections/struct.HashMap.html "struct bevy::platform::collections::HashMap")<K, V, S>

where K: [MapEntities](trait.MapEntities.html "trait bevy::ecs::entity::MapEntities") + [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash"), V: [MapEntities](trait.MapEntities.html "trait bevy::ecs::entity::MapEntities"), S: [BuildHasher](https://doc.rust-lang.org/nightly/core/hash/trait.BuildHasher.html "trait core::hash::BuildHasher") + [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/map_entities.rs.html#91)

### impl<T, S> [MapEntities](trait.MapEntities.html "trait bevy::ecs::entity::MapEntities") for [HashSet](../../platform/collections/struct.HashSet.html "struct bevy::platform::collections::HashSet")<T, S>

where T: [MapEntities](trait.MapEntities.html "trait bevy::ecs::entity::MapEntities") + [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash"), S: [BuildHasher](https://doc.rust-lang.org/nightly/core/hash/trait.BuildHasher.html "trait core::hash::BuildHasher") + [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/map_entities.rs.html#172)

### impl<T> [MapEntities](trait.MapEntities.html "trait bevy::ecs::entity::MapEntities") for [Vec](../../prelude/struct.Vec.html "struct bevy::prelude::Vec")<T>

where T: [MapEntities](trait.MapEntities.html "trait bevy::ecs::entity::MapEntities"),