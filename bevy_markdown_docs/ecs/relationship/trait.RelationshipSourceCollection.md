[bevy](../../index.html)::[ecs](../index.html)::[relationship](index.html)

# Trait RelationshipSourceCollection 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/relationship/relationship_source_collection.rs.html#14)

```rust
pub trait RelationshipSourceCollection {
    type SourceIter<'a>: Iterator<Item = Entity>
       where Self: 'a;

    // Required methods
    fn new() -> Self;
    fn with_capacity(capacity: usize) -> Self;
    fn reserve(&mut self, additional: usize);
    fn add(&mut self, entity: Entity) -> bool;
    fn remove(&mut self, entity: Entity) -> bool;
    fn iter(&self) -> Self::SourceIter<'_>;
    fn len(&self) -> usize;
    fn clear(&mut self);
    fn shrink_to_fit(&mut self);
    fn extend_from_iter(&mut self, entities: impl IntoIterator<Item = Entity>);

    // Provided methods
    fn is_empty(&self) -> bool { ... }
    fn source_to_remove_before_add(&self) -> Option<Entity> { ... }
}
```

The internal [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity") collection used by a [`RelationshipTarget`](../../prelude/trait.RelationshipTarget.html "trait bevy::prelude::RelationshipTarget") component. This is not intended to be modified directly by users, as it could invalidate the correctness of relationships.

## Required Associated Types

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/relationship/relationship_source_collection.rs.html#22)

#### type [SourceIter](#associatedtype.SourceIter)<'a>: [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = [Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity")\> where Self: 'a

The type of iterator returned by the `iter` method.

This is an associated type (rather than using a method that returns an opaque return-position impl trait) to ensure that all methods and traits (like [`DoubleEndedIterator`](https://doc.rust-lang.org/nightly/core/iter/traits/double_ended/trait.DoubleEndedIterator.html "trait core::iter::traits::double_ended::DoubleEndedIterator")) of the underlying collection’s iterator are available to the user when implemented without unduly restricting the possible collections.

The [`SourceIter`](type.SourceIter.html "type bevy::ecs::relationship::SourceIter") type alias can be helpful to reduce confusion when working with this associated type.

## Required Methods

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/relationship/relationship_source_collection.rs.html#27)

#### fn [new](#tymethod.new)() -> Self

Creates a new empty instance.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/relationship/relationship_source_collection.rs.html#32)

#### fn [with\_capacity](#tymethod.with_capacity)(capacity: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> Self

Returns an instance with the given pre-allocated entity `capacity`.

Some collections will ignore the provided `capacity` and return a default instance.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/relationship/relationship_source_collection.rs.html#37)

#### fn [reserve](#tymethod.reserve)(&mut self, additional: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html))

Reserves capacity for at least `additional` more entities to be inserted.

Not all collections support this operation, in which case it is a no-op.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/relationship/relationship_source_collection.rs.html#44)

#### fn [add](#tymethod.add)(&mut self, entity: [Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Adds the given `entity` to the collection.

Returns whether the entity was added to the collection. Mainly useful when dealing with collections that don’t allow multiple instances of the same entity ([`EntityHashSet`](../entity/struct.EntityHashSet.html "struct bevy::ecs::entity::EntityHashSet")).

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/relationship/relationship_source_collection.rs.html#50)

#### fn [remove](#tymethod.remove)(&mut self, entity: [Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Removes the given `entity` from the collection.

Returns whether the collection actually contained the entity.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/relationship/relationship_source_collection.rs.html#53)

#### fn [iter](#tymethod.iter)(&self) -> Self::[SourceIter](trait.RelationshipSourceCollection.html#associatedtype.SourceIter "type bevy::ecs::relationship::RelationshipSourceCollection::SourceIter")<'\_>

Iterates all entities in the collection.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/relationship/relationship_source_collection.rs.html#56)

#### fn [len](#tymethod.len)(&self) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

Returns the current length of the collection.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/relationship/relationship_source_collection.rs.html#59)

#### fn [clear](#tymethod.clear)(&mut self)

Clears the collection.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/relationship/relationship_source_collection.rs.html#64)

#### fn [shrink\_to\_fit](#tymethod.shrink_to_fit)(&mut self)

Attempts to save memory by shrinking the capacity to fit the current length.

This operation is a no-op for collections that do not support it.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/relationship/relationship_source_collection.rs.html#81)

#### fn [extend\_from\_iter](#tymethod.extend_from_iter)(&mut self, entities: impl [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")<Item = [Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity")\>)

Add multiple entities to collection at once.

May be faster than repeatedly calling [`Self::add`](trait.RelationshipSourceCollection.html#tymethod.add "method bevy::ecs::relationship::RelationshipSourceCollection::add").

## Provided Methods

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/relationship/relationship_source_collection.rs.html#68)

#### fn [is\_empty](#method.is_empty)(&self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Returns true if the collection contains no entities.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/relationship/relationship_source_collection.rs.html#74)

#### fn [source\_to\_remove\_before\_add](#method.source_to_remove_before_add)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity")\>

For one-to-one relationships, returns the entity that should be removed before adding a new one. Returns `None` for one-to-many relationships or when no entity needs to be removed.

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementations on Foreign Types

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/relationship/relationship_source_collection.rs.html#540)

### impl [RelationshipSourceCollection](trait.RelationshipSourceCollection.html "trait bevy::ecs::relationship::RelationshipSourceCollection") for [BTreeSet](https://doc.rust-lang.org/nightly/alloc/collections/btree/set/struct.BTreeSet.html "struct alloc::collections::btree::set::BTreeSet")<[Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity")\>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/relationship/relationship_source_collection.rs.html#541)

#### type [SourceIter](#associatedtype.SourceIter)<'a> = [Copied](https://doc.rust-lang.org/nightly/core/iter/adapters/copied/struct.Copied.html "struct core::iter::adapters::copied::Copied")<[Iter](https://doc.rust-lang.org/nightly/alloc/collections/btree/set/struct.Iter.html "struct alloc::collections::btree::set::Iter")<'a, [Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity")\>>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/relationship/relationship_source_collection.rs.html#543)

#### fn [new](#tymethod.new)() -> [BTreeSet](https://doc.rust-lang.org/nightly/alloc/collections/btree/set/struct.BTreeSet.html "struct alloc::collections::btree::set::BTreeSet")<[Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity")\>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/relationship/relationship_source_collection.rs.html#547)

#### fn [with\_capacity](#tymethod.with_capacity)(\_: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [BTreeSet](https://doc.rust-lang.org/nightly/alloc/collections/btree/set/struct.BTreeSet.html "struct alloc::collections::btree::set::BTreeSet")<[Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity")\>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/relationship/relationship_source_collection.rs.html#552)

#### fn [reserve](#tymethod.reserve)(&mut self, \_: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html))

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/relationship/relationship_source_collection.rs.html#556)

#### fn [add](#tymethod.add)(&mut self, entity: [Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/relationship/relationship_source_collection.rs.html#560)

#### fn [remove](#tymethod.remove)(&mut self, entity: [Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/relationship/relationship_source_collection.rs.html#564)

#### fn [iter](#tymethod.iter)( &self, ) -> <[BTreeSet](https://doc.rust-lang.org/nightly/alloc/collections/btree/set/struct.BTreeSet.html "struct alloc::collections::btree::set::BTreeSet")<[Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity")\> as [RelationshipSourceCollection](trait.RelationshipSourceCollection.html "trait bevy::ecs::relationship::RelationshipSourceCollection")\>::[SourceIter](trait.RelationshipSourceCollection.html#associatedtype.SourceIter "type bevy::ecs::relationship::RelationshipSourceCollection::SourceIter")<'\_>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/relationship/relationship_source_collection.rs.html#568)

#### fn [len](#tymethod.len)(&self) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/relationship/relationship_source_collection.rs.html#572)

#### fn [clear](#tymethod.clear)(&mut self)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/relationship/relationship_source_collection.rs.html#576)

#### fn [shrink\_to\_fit](#tymethod.shrink_to_fit)(&mut self)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/relationship/relationship_source_collection.rs.html#580)

#### fn [extend\_from\_iter](#tymethod.extend_from_iter)(&mut self, entities: impl [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")<Item = [Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity")\>)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/relationship/relationship_source_collection.rs.html#449)

### impl<S> [RelationshipSourceCollection](trait.RelationshipSourceCollection.html "trait bevy::ecs::relationship::RelationshipSourceCollection") for [IndexSet](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/set/struct.IndexSet.html "struct indexmap::set::IndexSet")<[Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity"), S>

where S: [BuildHasher](https://doc.rust-lang.org/nightly/core/hash/trait.BuildHasher.html "trait core::hash::BuildHasher") + [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/relationship/relationship_source_collection.rs.html#450)

#### type [SourceIter](#associatedtype.SourceIter)<'a> = [Copied](https://doc.rust-lang.org/nightly/core/iter/adapters/copied/struct.Copied.html "struct core::iter::adapters::copied::Copied")<[Iter](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/set/iter/struct.Iter.html "struct indexmap::set::iter::Iter")<'a, [Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity")\>> where S: 'a

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/relationship/relationship_source_collection.rs.html#455)

#### fn [new](#tymethod.new)() -> [IndexSet](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/set/struct.IndexSet.html "struct indexmap::set::IndexSet")<[Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity"), S>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/relationship/relationship_source_collection.rs.html#459)

#### fn [reserve](#tymethod.reserve)(&mut self, additional: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html))

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/relationship/relationship_source_collection.rs.html#463)

#### fn [with\_capacity](#tymethod.with_capacity)(capacity: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [IndexSet](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/set/struct.IndexSet.html "struct indexmap::set::IndexSet")<[Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity"), S>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/relationship/relationship_source_collection.rs.html#467)

#### fn [add](#tymethod.add)(&mut self, entity: [Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/relationship/relationship_source_collection.rs.html#471)

#### fn [remove](#tymethod.remove)(&mut self, entity: [Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/relationship/relationship_source_collection.rs.html#475)

#### fn [iter](#tymethod.iter)( &self, ) -> <[IndexSet](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/set/struct.IndexSet.html "struct indexmap::set::IndexSet")<[Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity"), S> as [RelationshipSourceCollection](trait.RelationshipSourceCollection.html "trait bevy::ecs::relationship::RelationshipSourceCollection")\>::[SourceIter](trait.RelationshipSourceCollection.html#associatedtype.SourceIter "type bevy::ecs::relationship::RelationshipSourceCollection::SourceIter")<'\_>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/relationship/relationship_source_collection.rs.html#479)

#### fn [len](#tymethod.len)(&self) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/relationship/relationship_source_collection.rs.html#483)

#### fn [clear](#tymethod.clear)(&mut self)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/relationship/relationship_source_collection.rs.html#487)

#### fn [shrink\_to\_fit](#tymethod.shrink_to_fit)(&mut self)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/relationship/relationship_source_collection.rs.html#491)

#### fn [extend\_from\_iter](#tymethod.extend_from_iter)(&mut self, entities: impl [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")<Item = [Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity")\>)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/relationship/relationship_source_collection.rs.html#283)

### impl<const N: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\> [RelationshipSourceCollection](trait.RelationshipSourceCollection.html "trait bevy::ecs::relationship::RelationshipSourceCollection") for [SmallVec](https://docs.rs/smallvec/1.15.1/x86_64-unknown-linux-gnu/smallvec/struct.SmallVec.html "struct smallvec::SmallVec")<\[[Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity"); [N](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/relationship/relationship_source_collection.rs.html#284)

#### type [SourceIter](#associatedtype.SourceIter)<'a> = [Copied](https://doc.rust-lang.org/nightly/core/iter/adapters/copied/struct.Copied.html "struct core::iter::adapters::copied::Copied")<[Iter](https://doc.rust-lang.org/nightly/core/slice/iter/struct.Iter.html "struct core::slice::iter::Iter")<'a, [Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity")\>>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/relationship/relationship_source_collection.rs.html#286)

#### fn [new](#tymethod.new)() -> [SmallVec](https://docs.rs/smallvec/1.15.1/x86_64-unknown-linux-gnu/smallvec/struct.SmallVec.html "struct smallvec::SmallVec")<\[[Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity"); [N](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/relationship/relationship_source_collection.rs.html#290)

#### fn [reserve](#tymethod.reserve)(&mut self, additional: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html))

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/relationship/relationship_source_collection.rs.html#294)

#### fn [with\_capacity](#tymethod.with_capacity)(capacity: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [SmallVec](https://docs.rs/smallvec/1.15.1/x86_64-unknown-linux-gnu/smallvec/struct.SmallVec.html "struct smallvec::SmallVec")<\[[Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity"); [N](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/relationship/relationship_source_collection.rs.html#298)

#### fn [add](#tymethod.add)(&mut self, entity: [Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/relationship/relationship_source_collection.rs.html#304)

#### fn [remove](#tymethod.remove)(&mut self, entity: [Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/relationship/relationship_source_collection.rs.html#313)

#### fn [iter](#tymethod.iter)( &self, ) -> <[SmallVec](https://docs.rs/smallvec/1.15.1/x86_64-unknown-linux-gnu/smallvec/struct.SmallVec.html "struct smallvec::SmallVec")<\[[Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity"); [N](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]> as [RelationshipSourceCollection](trait.RelationshipSourceCollection.html "trait bevy::ecs::relationship::RelationshipSourceCollection")\>::[SourceIter](trait.RelationshipSourceCollection.html#associatedtype.SourceIter "type bevy::ecs::relationship::RelationshipSourceCollection::SourceIter")<'\_>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/relationship/relationship_source_collection.rs.html#317)

#### fn [len](#tymethod.len)(&self) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/relationship/relationship_source_collection.rs.html#321)

#### fn [clear](#tymethod.clear)(&mut self)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/relationship/relationship_source_collection.rs.html#325)

#### fn [shrink\_to\_fit](#tymethod.shrink_to_fit)(&mut self)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/relationship/relationship_source_collection.rs.html#329)

#### fn [extend\_from\_iter](#tymethod.extend_from_iter)(&mut self, entities: impl [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")<Item = [Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity")\>)

## Implementors

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/relationship/relationship_source_collection.rs.html#334)

### impl [RelationshipSourceCollection](trait.RelationshipSourceCollection.html "trait bevy::ecs::relationship::RelationshipSourceCollection") for [Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/relationship/relationship_source_collection.rs.html#335)

#### type [SourceIter](#associatedtype.SourceIter)<'a> = [IntoIter](https://doc.rust-lang.org/nightly/core/option/struct.IntoIter.html "struct core::option::IntoIter")<[Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity")\>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/relationship/relationship_source_collection.rs.html#239)

### impl [RelationshipSourceCollection](trait.RelationshipSourceCollection.html "trait bevy::ecs::relationship::RelationshipSourceCollection") for [EntityHashSet](../entity/struct.EntityHashSet.html "struct bevy::ecs::entity::EntityHashSet")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/relationship/relationship_source_collection.rs.html#240)

#### type [SourceIter](#associatedtype.SourceIter)<'a> = [Copied](https://doc.rust-lang.org/nightly/core/iter/adapters/copied/struct.Copied.html "struct core::iter::adapters::copied::Copied")<[Iter](../entity/hash_set/struct.Iter.html "struct bevy::ecs::entity::hash_set::Iter")<'a>>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/relationship/relationship_source_collection.rs.html#496)

### impl [RelationshipSourceCollection](trait.RelationshipSourceCollection.html "trait bevy::ecs::relationship::RelationshipSourceCollection") for [EntityIndexSet](../entity/struct.EntityIndexSet.html "struct bevy::ecs::entity::EntityIndexSet")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/relationship/relationship_source_collection.rs.html#497)

#### type [SourceIter](#associatedtype.SourceIter)<'a> = [Copied](https://doc.rust-lang.org/nightly/core/iter/adapters/copied/struct.Copied.html "struct core::iter::adapters::copied::Copied")<[Iter](../entity/index_set/struct.Iter.html "struct bevy::ecs::entity::index_set::Iter")<'a>>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/relationship/relationship_source_collection.rs.html#136)

### impl [RelationshipSourceCollection](trait.RelationshipSourceCollection.html "trait bevy::ecs::relationship::RelationshipSourceCollection") for [Vec](../../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity")\>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/relationship/relationship_source_collection.rs.html#137)

#### type [SourceIter](#associatedtype.SourceIter)<'a> = [Copied](https://doc.rust-lang.org/nightly/core/iter/adapters/copied/struct.Copied.html "struct core::iter::adapters::copied::Copied")<[Iter](https://doc.rust-lang.org/nightly/core/slice/iter/struct.Iter.html "struct core::slice::iter::Iter")<'a, [Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity")\>>