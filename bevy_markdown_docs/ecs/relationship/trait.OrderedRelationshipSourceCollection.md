[bevy](../../index.html)::[ecs](../index.html)::[relationship](index.html)

# Trait OrderedRelationshipSourceCollection 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/relationship/relationship_source_collection.rs.html#85)

```rust
pub trait OrderedRelationshipSourceCollection: RelationshipSourceCollection {
    // Required methods
    fn insert(&mut self, index: usize, entity: Entity);
    fn remove_at(&mut self, index: usize) -> Option<Entity>;
    fn insert_stable(&mut self, index: usize, entity: Entity);
    fn remove_at_stable(&mut self, index: usize) -> Option<Entity>;
    fn sort(&mut self);
    fn insert_sorted(&mut self, entity: Entity);
    fn place_most_recent(&mut self, index: usize);
    fn place(&mut self, entity: Entity, index: usize);

    // Provided methods
    fn push_front(&mut self, entity: Entity) { ... }
    fn push_back(&mut self, entity: Entity) { ... }
    fn pop_front(&mut self) -> Option<Entity> { ... }
    fn pop_back(&mut self) -> Option<Entity> { ... }
}
```

This trait signals that a [`RelationshipSourceCollection`](trait.RelationshipSourceCollection.html "trait bevy::ecs::relationship::RelationshipSourceCollection") is ordered.

## Required Methods

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/relationship/relationship_source_collection.rs.html#88)

#### fn [insert](#tymethod.insert)(&mut self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), entity: [Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity"))

Inserts the entity at a specific index. If the index is too large, the entity will be added to the end of the collection.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/relationship/relationship_source_collection.rs.html#90)

#### fn [remove\_at](#tymethod.remove_at)(&mut self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity")\>

Removes the entity at the specified index if it exists.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/relationship/relationship_source_collection.rs.html#94)

#### fn [insert\_stable](#tymethod.insert_stable)(&mut self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), entity: [Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity"))

Inserts the entity at a specific index. This will never reorder other entities. If the index is too large, the entity will be added to the end of the collection.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/relationship/relationship_source_collection.rs.html#97)

#### fn [remove\_at\_stable](#tymethod.remove_at_stable)(&mut self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity")\>

Removes the entity at the specified index if it exists. This will never reorder other entities.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/relationship/relationship_source_collection.rs.html#99)

#### fn [sort](#tymethod.sort)(&mut self)

Sorts the source collection.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/relationship/relationship_source_collection.rs.html#101)

#### fn [insert\_sorted](#tymethod.insert_sorted)(&mut self, entity: [Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity"))

Inserts the entity at the proper place to maintain sorting.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/relationship/relationship_source_collection.rs.html#104)

#### fn [place\_most\_recent](#tymethod.place_most_recent)(&mut self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html))

This places the most recently added entity at the particular index.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/relationship/relationship_source_collection.rs.html#109)

#### fn [place](#tymethod.place)(&mut self, entity: [Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity"), index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html))

This places the given entity at the particular index. This will do nothing if the entity is not in the collection. If the index is out of bounds, this will put the entity at the end.

## Provided Methods

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/relationship/relationship_source_collection.rs.html#112)

#### fn [push\_front](#method.push_front)(&mut self, entity: [Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity"))

Adds the entity at index 0.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/relationship/relationship_source_collection.rs.html#117)

#### fn [push\_back](#method.push_back)(&mut self, entity: [Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity"))

Adds the entity to the back of the collection.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/relationship/relationship_source_collection.rs.html#122)

#### fn [pop\_front](#method.pop_front)(&mut self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity")\>

Removes the first entity.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/relationship/relationship_source_collection.rs.html#127)

#### fn [pop\_back](#method.pop_back)(&mut self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity")\>

Removes the last entity.

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementations on Foreign Types

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/relationship/relationship_source_collection.rs.html#398)

### impl<const N: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\> [OrderedRelationshipSourceCollection](trait.OrderedRelationshipSourceCollection.html "trait bevy::ecs::relationship::OrderedRelationshipSourceCollection") for [SmallVec](https://docs.rs/smallvec/1.15.1/x86_64-unknown-linux-gnu/smallvec/struct.SmallVec.html "struct smallvec::SmallVec")<\[[Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity"); [N](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/relationship/relationship_source_collection.rs.html#399)

#### fn [insert](#tymethod.insert)(&mut self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), entity: [Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity"))

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/relationship/relationship_source_collection.rs.html#407)

#### fn [remove\_at](#tymethod.remove_at)(&mut self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity")\>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/relationship/relationship_source_collection.rs.html#411)

#### fn [insert\_stable](#tymethod.insert_stable)(&mut self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), entity: [Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity"))

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/relationship/relationship_source_collection.rs.html#419)

#### fn [remove\_at\_stable](#tymethod.remove_at_stable)(&mut self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity")\>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/relationship/relationship_source_collection.rs.html#423)

#### fn [sort](#tymethod.sort)(&mut self)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/relationship/relationship_source_collection.rs.html#427)

#### fn [insert\_sorted](#tymethod.insert_sorted)(&mut self, entity: [Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity"))

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/relationship/relationship_source_collection.rs.html#432)

#### fn [place\_most\_recent](#tymethod.place_most_recent)(&mut self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html))

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/relationship/relationship_source_collection.rs.html#439)

#### fn [place](#tymethod.place)(&mut self, entity: [Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity"), index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html))

## Implementors

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/relationship/relationship_source_collection.rs.html#189)

### impl [OrderedRelationshipSourceCollection](trait.OrderedRelationshipSourceCollection.html "trait bevy::ecs::relationship::OrderedRelationshipSourceCollection") for [Vec](../../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity")\>