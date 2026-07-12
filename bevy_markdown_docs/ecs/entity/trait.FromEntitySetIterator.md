[bevy](../../index.html)::[ecs](../index.html)::[entity](index.html)

# Trait FromEntitySetIterator 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/entity_set.rs.html#337)

```rust
pub trait FromEntitySetIterator<A>: FromIterator<A>where
    A: EntityEquivalent,{
    // Required method
    fn from_entity_set_iter<T>(set_iter: T) -> Self
       where T: EntitySet<Item = A>;
}
```

Conversion from an `EntitySetIterator`.

Some collections, while they can be constructed from plain iterators, benefit strongly from the additional uniqueness guarantee [`EntitySetIterator`](trait.EntitySetIterator.html "trait bevy::ecs::entity::EntitySetIterator") offers. Mirroring [`Iterator::collect`](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.collect "method core::iter::traits::iterator::Iterator::collect")/[`FromIterator::from_iter`](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.FromIterator.html#tymethod.from_iter "associated function core::iter::traits::collect::FromIterator::from_iter"), [`EntitySetIterator::collect_set`](trait.EntitySetIterator.html#method.collect_set "method bevy::ecs::entity::EntitySetIterator::collect_set") and `FromEntitySetIterator::from_entity_set_iter` can be used for construction.

See also: [`EntitySet`](trait.EntitySet.html "trait bevy::ecs::entity::EntitySet").

## Required Methods

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/entity_set.rs.html#339)

#### fn [from\_entity\_set\_iter](#tymethod.from_entity_set_iter)<T>(set\_iter: T) -> Self

where T: [EntitySet](trait.EntitySet.html "trait bevy::ecs::entity::EntitySet")<Item = A>,

Creates a value from an [`EntitySetIterator`](trait.EntitySetIterator.html "trait bevy::ecs::entity::EntitySetIterator").

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/hash_set.rs.html#199)

### impl [FromEntitySetIterator](trait.FromEntitySetIterator.html "trait bevy::ecs::entity::FromEntitySetIterator")<[Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity")\> for [EntityHashSet](struct.EntityHashSet.html "struct bevy::ecs::entity::EntityHashSet")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/entity_set.rs.html#342-343)

### impl<T, S> [FromEntitySetIterator](trait.FromEntitySetIterator.html "trait bevy::ecs::entity::FromEntitySetIterator")<T> for [HashSet](../../platform/collections/struct.HashSet.html "struct bevy::platform::collections::HashSet")<T, S>

where T: [EntityEquivalent](trait.EntityEquivalent.html "trait bevy::ecs::entity::EntityEquivalent") + [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash"), S: [BuildHasher](https://doc.rust-lang.org/nightly/core/hash/trait.BuildHasher.html "trait core::hash::BuildHasher") + [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#1097)

### impl<T> [FromEntitySetIterator](trait.FromEntitySetIterator.html "trait bevy::ecs::entity::FromEntitySetIterator")<T> for [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<[UniqueEntityEquivalentSlice](struct.UniqueEntityEquivalentSlice.html "struct bevy::ecs::entity::UniqueEntityEquivalentSlice")<T>>

where T: [EntityEquivalent](trait.EntityEquivalent.html "trait bevy::ecs::entity::EntityEquivalent"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_vec.rs.html#930)

### impl<T> [FromEntitySetIterator](trait.FromEntitySetIterator.html "trait bevy::ecs::entity::FromEntitySetIterator")<T> for [UniqueEntityEquivalentVec](struct.UniqueEntityEquivalentVec.html "struct bevy::ecs::entity::UniqueEntityEquivalentVec")<T>

where T: [EntityEquivalent](trait.EntityEquivalent.html "trait bevy::ecs::entity::EntityEquivalent"),