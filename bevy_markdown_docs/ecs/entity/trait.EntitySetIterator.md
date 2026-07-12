[bevy](../../index.html)::[ecs](../index.html)::[entity](index.html)

# Trait EntitySetIterator 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/entity_set.rs.html#170)

```rust
pub unsafe trait EntitySetIterator: Iteratorwhere
    Self::Item: EntityEquivalent,{
    // Provided method
    fn collect_set<B>(self) -> B
       where B: FromEntitySetIterator<Self::Item>,
             Self: Sized { ... }
}
```

An iterator over a set of unique entities.

Every `EntitySetIterator` is also [`EntitySet`](trait.EntitySet.html "trait bevy::ecs::entity::EntitySet").

## Safety

`x != y` must hold for any 2 elements returned by the iterator. This is always true for iterators that cannot return more than one element.

## Provided Methods

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/entity_set.rs.html#178-180)

#### fn [collect\_set](#method.collect_set)<B>(self) -> B

where B: [FromEntitySetIterator](trait.FromEntitySetIterator.html "trait bevy::ecs::entity::FromEntitySetIterator")<Self::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")\>, Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Transforms an `EntitySetIterator` into a collection.

This is a specialized form of [`collect`](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.collect "method core::iter::traits::iterator::Iterator::collect"), for collections which benefit from the uniqueness guarantee. When present, this should always be preferred over [`collect`](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.collect "method core::iter::traits::iterator::Iterator::collect").

## Dyn Compatibility

This trait **is** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementations on Foreign Types

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_set.rs.html#824)

### impl [EntitySetIterator](trait.EntitySetIterator.html "trait bevy::ecs::entity::EntitySetIterator") for [Difference](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/set/iter/struct.Difference.html "struct indexmap::set::iter::Difference")<'\_, [Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity"), [EntityHash](struct.EntityHash.html "struct bevy::ecs::entity::EntityHash")\>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_set.rs.html#827)

### impl [EntitySetIterator](trait.EntitySetIterator.html "trait bevy::ecs::entity::EntitySetIterator") for [Intersection](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/set/iter/struct.Intersection.html "struct indexmap::set::iter::Intersection")<'\_, [Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity"), [EntityHash](struct.EntityHash.html "struct bevy::ecs::entity::EntityHash")\>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_set.rs.html#830)

### impl [EntitySetIterator](trait.EntitySetIterator.html "trait bevy::ecs::entity::EntitySetIterator") for [SymmetricDifference](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/set/iter/struct.SymmetricDifference.html "struct indexmap::set::iter::SymmetricDifference")<'\_, [Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity"), [EntityHash](struct.EntityHash.html "struct bevy::ecs::entity::EntityHash"), [EntityHash](struct.EntityHash.html "struct bevy::ecs::entity::EntityHash")\>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_set.rs.html#833)

### impl [EntitySetIterator](trait.EntitySetIterator.html "trait bevy::ecs::entity::EntitySetIterator") for [Union](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/set/iter/struct.Union.html "struct indexmap::set::iter::Union")<'\_, [Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity"), [EntityHash](struct.EntityHash.html "struct bevy::ecs::entity::EntityHash")\>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/entity_set.rs.html#282-283)

### impl<'a, T, I> [EntitySetIterator](trait.EntitySetIterator.html "trait bevy::ecs::entity::EntitySetIterator") for [Cloned](https://doc.rust-lang.org/nightly/core/iter/adapters/cloned/struct.Cloned.html "struct core::iter::adapters::cloned::Cloned")<I>

where T: 'a + [EntityEquivalent](trait.EntityEquivalent.html "trait bevy::ecs::entity::EntityEquivalent") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"), I: [EntitySetIterator](trait.EntitySetIterator.html "trait bevy::ecs::entity::EntitySetIterator")<Item = [&'a T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)\>,

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/entity_set.rs.html#276-277)

### impl<'a, T, I> [EntitySetIterator](trait.EntitySetIterator.html "trait bevy::ecs::entity::EntitySetIterator") for [Copied](https://doc.rust-lang.org/nightly/core/iter/adapters/copied/struct.Copied.html "struct core::iter::adapters::copied::Copied")<I>

where T: 'a + [EntityEquivalent](trait.EntityEquivalent.html "trait bevy::ecs::entity::EntityEquivalent") + [Copy](https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html "trait core::marker::Copy"), I: [EntitySetIterator](trait.EntitySetIterator.html "trait bevy::ecs::entity::EntitySetIterator")<Item = [&'a T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)\>,

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/entity_set.rs.html#299-300)

### impl<I, F> [EntitySetIterator](trait.EntitySetIterator.html "trait bevy::ecs::entity::EntitySetIterator") for [Inspect](https://doc.rust-lang.org/nightly/core/iter/adapters/inspect/struct.Inspect.html "struct core::iter::adapters::inspect::Inspect")<I, F>

where I: [EntitySetIterator](trait.EntitySetIterator.html "trait bevy::ecs::entity::EntitySetIterator"), F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(&<I as [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")\>::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/entity_set.rs.html#288-289)

### impl<I, P> [EntitySetIterator](trait.EntitySetIterator.html "trait bevy::ecs::entity::EntitySetIterator") for [Filter](https://doc.rust-lang.org/nightly/core/iter/adapters/filter/struct.Filter.html "struct core::iter::adapters::filter::Filter")<I, P>

where I: [EntitySetIterator](trait.EntitySetIterator.html "trait bevy::ecs::entity::EntitySetIterator"), P: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(&<I as [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")\>::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/entity_set.rs.html#311-312)

### impl<I, P> [EntitySetIterator](trait.EntitySetIterator.html "trait bevy::ecs::entity::EntitySetIterator") for [SkipWhile](https://doc.rust-lang.org/nightly/core/iter/adapters/skip_while/struct.SkipWhile.html "struct core::iter::adapters::skip_while::SkipWhile")<I, P>

where I: [EntitySetIterator](trait.EntitySetIterator.html "trait bevy::ecs::entity::EntitySetIterator"), P: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(&<I as [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")\>::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/entity_set.rs.html#320-321)

### impl<I, P> [EntitySetIterator](trait.EntitySetIterator.html "trait bevy::ecs::entity::EntitySetIterator") for [TakeWhile](https://doc.rust-lang.org/nightly/core/iter/adapters/take_while/struct.TakeWhile.html "struct core::iter::adapters::take_while::TakeWhile")<I, P>

where I: [EntitySetIterator](trait.EntitySetIterator.html "trait bevy::ecs::entity::EntitySetIterator"), P: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(&<I as [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")\>::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/entity_set.rs.html#270)

### impl<I> [EntitySetIterator](trait.EntitySetIterator.html "trait bevy::ecs::entity::EntitySetIterator") for [&mut I](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

where I: [EntitySetIterator](trait.EntitySetIterator.html "trait bevy::ecs::entity::EntitySetIterator") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/entity_set.rs.html#294)

### impl<I> [EntitySetIterator](trait.EntitySetIterator.html "trait bevy::ecs::entity::EntitySetIterator") for [Fuse](https://doc.rust-lang.org/nightly/core/iter/adapters/fuse/struct.Fuse.html "struct core::iter::adapters::fuse::Fuse")<I>

where I: [EntitySetIterator](trait.EntitySetIterator.html "trait bevy::ecs::entity::EntitySetIterator"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/entity_set.rs.html#305)

### impl<I> [EntitySetIterator](trait.EntitySetIterator.html "trait bevy::ecs::entity::EntitySetIterator") for [Rev](https://doc.rust-lang.org/nightly/core/iter/adapters/rev/struct.Rev.html "struct core::iter::adapters::rev::Rev")<I>

where I: [DoubleEndedIterator](https://doc.rust-lang.org/nightly/core/iter/traits/double_ended/trait.DoubleEndedIterator.html "trait core::iter::traits::double_ended::DoubleEndedIterator") + [EntitySetIterator](trait.EntitySetIterator.html "trait bevy::ecs::entity::EntitySetIterator"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/entity_set.rs.html#308)

### impl<I> [EntitySetIterator](trait.EntitySetIterator.html "trait bevy::ecs::entity::EntitySetIterator") for [Skip](https://doc.rust-lang.org/nightly/core/iter/adapters/skip/struct.Skip.html "struct core::iter::adapters::skip::Skip")<I>

where I: [EntitySetIterator](trait.EntitySetIterator.html "trait bevy::ecs::entity::EntitySetIterator"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_set.rs.html#836-837)

### impl<I> [EntitySetIterator](trait.EntitySetIterator.html "trait bevy::ecs::entity::EntitySetIterator") for [Splice](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/set/iter/struct.Splice.html "struct indexmap::set::iter::Splice")<'\_, I, [Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity"), [EntityHash](struct.EntityHash.html "struct bevy::ecs::entity::EntityHash")\>

where I: [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = [Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity")\>,

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/entity_set.rs.html#326)

### impl<I> [EntitySetIterator](trait.EntitySetIterator.html "trait bevy::ecs::entity::EntitySetIterator") for [StepBy](https://doc.rust-lang.org/nightly/core/iter/adapters/step_by/struct.StepBy.html "struct core::iter::adapters::step_by::StepBy")<I>

where I: [EntitySetIterator](trait.EntitySetIterator.html "trait bevy::ecs::entity::EntitySetIterator"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/entity_set.rs.html#317)

### impl<I> [EntitySetIterator](trait.EntitySetIterator.html "trait bevy::ecs::entity::EntitySetIterator") for [Take](https://doc.rust-lang.org/nightly/core/iter/adapters/take/struct.Take.html "struct core::iter::adapters::take::Take")<I>

where I: [EntitySetIterator](trait.EntitySetIterator.html "trait bevy::ecs::entity::EntitySetIterator"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/entity_set.rs.html#194)

### impl<K, V> [EntitySetIterator](trait.EntitySetIterator.html "trait bevy::ecs::entity::EntitySetIterator") for [IntoKeys](https://doc.rust-lang.org/nightly/alloc/collections/btree/map/struct.IntoKeys.html "struct alloc::collections::btree::map::IntoKeys")<K, V>

where K: [EntityEquivalent](trait.EntityEquivalent.html "trait bevy::ecs::entity::EntityEquivalent"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/entity_set.rs.html#189)

### impl<K, V> [EntitySetIterator](trait.EntitySetIterator.html "trait bevy::ecs::entity::EntitySetIterator") for [Keys](https://doc.rust-lang.org/nightly/alloc/collections/btree/map/struct.Keys.html "struct alloc::collections::btree::map::Keys")<'\_, K, V>

where K: [EntityEquivalent](trait.EntityEquivalent.html "trait bevy::ecs::entity::EntityEquivalent"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/entity_set.rs.html#261)

### impl<T, F> [EntitySetIterator](trait.EntitySetIterator.html "trait bevy::ecs::entity::EntitySetIterator") for [OnceWith](https://doc.rust-lang.org/nightly/core/iter/sources/once_with/struct.OnceWith.html "struct core::iter::sources::once_with::OnceWith")<F>

where T: [EntityEquivalent](trait.EntityEquivalent.html "trait bevy::ecs::entity::EntityEquivalent"), F: [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")() -> T,

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/entity_set.rs.html#218)

### impl<T> [EntitySetIterator](trait.EntitySetIterator.html "trait bevy::ecs::entity::EntitySetIterator") for [Difference](https://doc.rust-lang.org/nightly/alloc/collections/btree/set/struct.Difference.html "struct alloc::collections::btree::set::Difference")<'\_, T>

where T: [EntityEquivalent](trait.EntityEquivalent.html "trait bevy::ecs::entity::EntityEquivalent") + [Ord](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html "trait core::cmp::Ord"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/entity_set.rs.html#267)

### impl<T> [EntitySetIterator](trait.EntitySetIterator.html "trait bevy::ecs::entity::EntitySetIterator") for [Empty](https://doc.rust-lang.org/nightly/core/iter/sources/empty/struct.Empty.html "struct core::iter::sources::empty::Empty")<T>

where T: [EntityEquivalent](trait.EntityEquivalent.html "trait bevy::ecs::entity::EntityEquivalent"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/entity_set.rs.html#206)

### impl<T> [EntitySetIterator](trait.EntitySetIterator.html "trait bevy::ecs::entity::EntitySetIterator") for [Intersection](https://doc.rust-lang.org/nightly/alloc/collections/btree/set/struct.Intersection.html "struct alloc::collections::btree::set::Intersection")<'\_, T>

where T: [EntityEquivalent](trait.EntityEquivalent.html "trait bevy::ecs::entity::EntityEquivalent") + [Ord](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html "trait core::cmp::Ord"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/entity_set.rs.html#258)

### impl<T> [EntitySetIterator](trait.EntitySetIterator.html "trait bevy::ecs::entity::EntitySetIterator") for [IntoIter](https://doc.rust-lang.org/nightly/core/array/iter/struct.IntoIter.html "struct core::array::iter::IntoIter")<T, 0>

where T: [EntityEquivalent](trait.EntityEquivalent.html "trait bevy::ecs::entity::EntityEquivalent"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/entity_set.rs.html#255)

### impl<T> [EntitySetIterator](trait.EntitySetIterator.html "trait bevy::ecs::entity::EntitySetIterator") for [IntoIter](https://doc.rust-lang.org/nightly/core/array/iter/struct.IntoIter.html "struct core::array::iter::IntoIter")<T, 1>

where T: [EntityEquivalent](trait.EntityEquivalent.html "trait bevy::ecs::entity::EntityEquivalent"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/entity_set.rs.html#234)

### impl<T> [EntitySetIterator](trait.EntitySetIterator.html "trait bevy::ecs::entity::EntitySetIterator") for [IntoIter](https://doc.rust-lang.org/nightly/alloc/collections/btree/set/struct.IntoIter.html "struct alloc::collections::btree::set::IntoIter")<T>

where T: [EntityEquivalent](trait.EntityEquivalent.html "trait bevy::ecs::entity::EntityEquivalent"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/entity_set.rs.html#243)

### impl<T> [EntitySetIterator](trait.EntitySetIterator.html "trait bevy::ecs::entity::EntitySetIterator") for [IntoIter](https://doc.rust-lang.org/nightly/core/option/struct.IntoIter.html "struct core::option::IntoIter")<T>

where T: [EntityEquivalent](trait.EntityEquivalent.html "trait bevy::ecs::entity::EntityEquivalent"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/entity_set.rs.html#252)

### impl<T> [EntitySetIterator](trait.EntitySetIterator.html "trait bevy::ecs::entity::EntitySetIterator") for [IntoIter](https://doc.rust-lang.org/nightly/core/result/struct.IntoIter.html "struct core::result::IntoIter")<T>

where T: [EntityEquivalent](trait.EntityEquivalent.html "trait bevy::ecs::entity::EntityEquivalent"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/entity_set.rs.html#229)

### impl<T> [EntitySetIterator](trait.EntitySetIterator.html "trait bevy::ecs::entity::EntitySetIterator") for [Iter](https://doc.rust-lang.org/nightly/alloc/collections/btree/set/struct.Iter.html "struct alloc::collections::btree::set::Iter")<'\_, T>

where T: [EntityEquivalent](trait.EntityEquivalent.html "trait bevy::ecs::entity::EntityEquivalent"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/entity_set.rs.html#237)

### impl<T> [EntitySetIterator](trait.EntitySetIterator.html "trait bevy::ecs::entity::EntitySetIterator") for [Iter](https://doc.rust-lang.org/nightly/core/option/struct.Iter.html "struct core::option::Iter")<'\_, T>

where T: [EntityEquivalent](trait.EntityEquivalent.html "trait bevy::ecs::entity::EntityEquivalent"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/entity_set.rs.html#246)

### impl<T> [EntitySetIterator](trait.EntitySetIterator.html "trait bevy::ecs::entity::EntitySetIterator") for [Iter](https://doc.rust-lang.org/nightly/core/result/struct.Iter.html "struct core::result::Iter")<'\_, T>

where T: [EntityEquivalent](trait.EntityEquivalent.html "trait bevy::ecs::entity::EntityEquivalent"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/entity_set.rs.html#264)

### impl<T> [EntitySetIterator](trait.EntitySetIterator.html "trait bevy::ecs::entity::EntitySetIterator") for [Once](https://doc.rust-lang.org/nightly/core/iter/sources/once/struct.Once.html "struct core::iter::sources::once::Once")<T>

where T: [EntityEquivalent](trait.EntityEquivalent.html "trait bevy::ecs::entity::EntityEquivalent"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/entity_set.rs.html#200)

### impl<T> [EntitySetIterator](trait.EntitySetIterator.html "trait bevy::ecs::entity::EntitySetIterator") for [Range](https://doc.rust-lang.org/nightly/alloc/collections/btree/set/struct.Range.html "struct alloc::collections::btree::set::Range")<'\_, T>

where T: [EntityEquivalent](trait.EntityEquivalent.html "trait bevy::ecs::entity::EntityEquivalent"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/entity_set.rs.html#224)

### impl<T> [EntitySetIterator](trait.EntitySetIterator.html "trait bevy::ecs::entity::EntitySetIterator") for [SymmetricDifference](https://doc.rust-lang.org/nightly/alloc/collections/btree/set/struct.SymmetricDifference.html "struct alloc::collections::btree::set::SymmetricDifference")<'\_, T>

where T: [EntityEquivalent](trait.EntityEquivalent.html "trait bevy::ecs::entity::EntityEquivalent") + [Ord](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html "trait core::cmp::Ord"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/entity_set.rs.html#212)

### impl<T> [EntitySetIterator](trait.EntitySetIterator.html "trait bevy::ecs::entity::EntitySetIterator") for [Union](https://doc.rust-lang.org/nightly/alloc/collections/btree/set/struct.Union.html "struct alloc::collections::btree::set::Union")<'\_, T>

where T: [EntityEquivalent](trait.EntityEquivalent.html "trait bevy::ecs::entity::EntityEquivalent") + [Ord](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html "trait core::cmp::Ord"),

## Implementors

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/mod.rs.html#822)

### impl [EntitySetIterator](trait.EntitySetIterator.html "trait bevy::ecs::entity::EntitySetIterator") for [AllocEntitiesIterator](struct.AllocEntitiesIterator.html "struct bevy::ecs::entity::AllocEntitiesIterator")<'\_>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/hash_set.rs.html#482)

### impl [EntitySetIterator](trait.EntitySetIterator.html "trait bevy::ecs::entity::EntitySetIterator") for bevy::platform::collections::hash\_set::[Difference](../../platform/collections/hash_set/struct.Difference.html "struct bevy::platform::collections::hash_set::Difference")<'\_, [Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity"), [EntityHash](struct.EntityHash.html "struct bevy::ecs::entity::EntityHash")\>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/hash_set.rs.html#419)

### impl [EntitySetIterator](trait.EntitySetIterator.html "trait bevy::ecs::entity::EntitySetIterator") for bevy::ecs::entity::hash\_set::[Drain](hash_set/struct.Drain.html "struct bevy::ecs::entity::hash_set::Drain")<'\_>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_set.rs.html#821)

### impl [EntitySetIterator](trait.EntitySetIterator.html "trait bevy::ecs::entity::EntitySetIterator") for bevy::ecs::entity::index\_set::[Drain](index_set/struct.Drain.html "struct bevy::ecs::entity::index_set::Drain")<'\_>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/hash_set.rs.html#485)

### impl [EntitySetIterator](trait.EntitySetIterator.html "trait bevy::ecs::entity::EntitySetIterator") for bevy::platform::collections::hash\_set::[Intersection](../../platform/collections/hash_set/struct.Intersection.html "struct bevy::platform::collections::hash_set::Intersection")<'\_, [Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity"), [EntityHash](struct.EntityHash.html "struct bevy::ecs::entity::EntityHash")\>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/hash_set.rs.html#357)

### impl [EntitySetIterator](trait.EntitySetIterator.html "trait bevy::ecs::entity::EntitySetIterator") for bevy::ecs::entity::hash\_set::[IntoIter](hash_set/struct.IntoIter.html "struct bevy::ecs::entity::hash_set::IntoIter")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_set.rs.html#747)

### impl [EntitySetIterator](trait.EntitySetIterator.html "trait bevy::ecs::entity::EntitySetIterator") for bevy::ecs::entity::index\_set::[IntoIter](index_set/struct.IntoIter.html "struct bevy::ecs::entity::index_set::IntoIter")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/hash_set.rs.html#288)

### impl [EntitySetIterator](trait.EntitySetIterator.html "trait bevy::ecs::entity::EntitySetIterator") for bevy::ecs::entity::hash\_set::[Iter](hash_set/struct.Iter.html "struct bevy::ecs::entity::hash_set::Iter")<'\_>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_set.rs.html#657)

### impl [EntitySetIterator](trait.EntitySetIterator.html "trait bevy::ecs::entity::EntitySetIterator") for bevy::ecs::entity::index\_set::[Iter](index_set/struct.Iter.html "struct bevy::ecs::entity::index_set::Iter")<'\_>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/hash_set.rs.html#488)

### impl [EntitySetIterator](trait.EntitySetIterator.html "trait bevy::ecs::entity::EntitySetIterator") for bevy::platform::collections::hash\_set::[SymmetricDifference](../../platform/collections/hash_set/struct.SymmetricDifference.html "struct bevy::platform::collections::hash_set::SymmetricDifference")<'\_, [Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity"), [EntityHash](struct.EntityHash.html "struct bevy::ecs::entity::EntityHash")\>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/hash_set.rs.html#491)

### impl [EntitySetIterator](trait.EntitySetIterator.html "trait bevy::ecs::entity::EntitySetIterator") for bevy::platform::collections::hash\_set::[Union](../../platform/collections/hash_set/struct.Union.html "struct bevy::platform::collections::hash_set::Union")<'\_, [Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity"), [EntityHash](struct.EntityHash.html "struct bevy::ecs::entity::EntityHash")\>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/iter.rs.html#1121-1122)

### impl<'w, 's, F, B> [EntitySetIterator](trait.EntitySetIterator.html "trait bevy::ecs::entity::EntitySetIterator") for [QueryIter](../query/struct.QueryIter.html "struct bevy::ecs::query::QueryIter")<'w, 's, [EntityMutExcept](../world/struct.EntityMutExcept.html "struct bevy::ecs::world::EntityMutExcept")<'\_, '\_, B>, F>

where F: [QueryFilter](../query/trait.QueryFilter.html "trait bevy::ecs::query::QueryFilter"), B: [Bundle](../../prelude/trait.Bundle.html "trait bevy::prelude::Bundle"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/iter.rs.html#1115-1116)

### impl<'w, 's, F, B> [EntitySetIterator](trait.EntitySetIterator.html "trait bevy::ecs::entity::EntitySetIterator") for [QueryIter](../query/struct.QueryIter.html "struct bevy::ecs::query::QueryIter")<'w, 's, [EntityRefExcept](../world/struct.EntityRefExcept.html "struct bevy::ecs::world::EntityRefExcept")<'\_, '\_, B>, F>

where F: [QueryFilter](../query/trait.QueryFilter.html "trait bevy::ecs::query::QueryFilter"), B: [Bundle](../../prelude/trait.Bundle.html "trait bevy::prelude::Bundle"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/iter.rs.html#2197-2198)

### impl<'w, 's, F, I> [EntitySetIterator](trait.EntitySetIterator.html "trait bevy::ecs::entity::EntitySetIterator") for [QueryManyIter](../query/struct.QueryManyIter.html "struct bevy::ecs::query::QueryManyIter")<'w, 's, [Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity"), F, I>

where F: [QueryFilter](../query/trait.QueryFilter.html "trait bevy::ecs::query::QueryFilter"), I: [EntitySetIterator](trait.EntitySetIterator.html "trait bevy::ecs::entity::EntitySetIterator"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/iter.rs.html#2286-2287)

### impl<'w, 's, F, I> [EntitySetIterator](trait.EntitySetIterator.html "trait bevy::ecs::entity::EntitySetIterator") for [QueryManyUniqueIter](../query/struct.QueryManyUniqueIter.html "struct bevy::ecs::query::QueryManyUniqueIter")<'w, 's, [Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity"), F, I>

where F: [QueryFilter](../query/trait.QueryFilter.html "trait bevy::ecs::query::QueryFilter"), I: [EntitySetIterator](trait.EntitySetIterator.html "trait bevy::ecs::entity::EntitySetIterator"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/iter.rs.html#1444-1445)

### impl<'w, 's, F, I> [EntitySetIterator](trait.EntitySetIterator.html "trait bevy::ecs::entity::EntitySetIterator") for [QuerySortedIter](../query/struct.QuerySortedIter.html "struct bevy::ecs::query::QuerySortedIter")<'w, 's, [Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity"), F, I>

where F: [QueryFilter](../query/trait.QueryFilter.html "trait bevy::ecs::query::QueryFilter"), I: [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = [Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity")\>,

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/iter.rs.html#1094)

### impl<'w, 's, F> [EntitySetIterator](trait.EntitySetIterator.html "trait bevy::ecs::entity::EntitySetIterator") for [QueryIter](../query/struct.QueryIter.html "struct bevy::ecs::query::QueryIter")<'w, 's, [Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity"), F>

where F: [QueryFilter](../query/trait.QueryFilter.html "trait bevy::ecs::query::QueryFilter"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/iter.rs.html#1100)

### impl<'w, 's, F> [EntitySetIterator](trait.EntitySetIterator.html "trait bevy::ecs::entity::EntitySetIterator") for [QueryIter](../query/struct.QueryIter.html "struct bevy::ecs::query::QueryIter")<'w, 's, [EntityMut](../../prelude/struct.EntityMut.html "struct bevy::prelude::EntityMut")<'\_>, F>

where F: [QueryFilter](../query/trait.QueryFilter.html "trait bevy::ecs::query::QueryFilter"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/iter.rs.html#1097)

### impl<'w, 's, F> [EntitySetIterator](trait.EntitySetIterator.html "trait bevy::ecs::entity::EntitySetIterator") for [QueryIter](../query/struct.QueryIter.html "struct bevy::ecs::query::QueryIter")<'w, 's, [EntityRef](../../prelude/struct.EntityRef.html "struct bevy::prelude::EntityRef")<'\_>, F>

where F: [QueryFilter](../query/trait.QueryFilter.html "trait bevy::ecs::query::QueryFilter"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/iter.rs.html#1109-1110)

### impl<'w, 's, F> [EntitySetIterator](trait.EntitySetIterator.html "trait bevy::ecs::entity::EntitySetIterator") for [QueryIter](../query/struct.QueryIter.html "struct bevy::ecs::query::QueryIter")<'w, 's, [FilteredEntityMut](../world/struct.FilteredEntityMut.html "struct bevy::ecs::world::FilteredEntityMut")<'\_, '\_>, F>

where F: [QueryFilter](../query/trait.QueryFilter.html "trait bevy::ecs::query::QueryFilter"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/iter.rs.html#1103-1104)

### impl<'w, 's, F> [EntitySetIterator](trait.EntitySetIterator.html "trait bevy::ecs::entity::EntitySetIterator") for [QueryIter](../query/struct.QueryIter.html "struct bevy::ecs::query::QueryIter")<'w, 's, [FilteredEntityRef](../world/struct.FilteredEntityRef.html "struct bevy::ecs::world::FilteredEntityRef")<'\_, '\_>, F>

where F: [QueryFilter](../query/trait.QueryFilter.html "trait bevy::ecs::query::QueryFilter"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/hash_set.rs.html#479)

### impl<F> [EntitySetIterator](trait.EntitySetIterator.html "trait bevy::ecs::entity::EntitySetIterator") for [ExtractIf](hash_set/struct.ExtractIf.html "struct bevy::ecs::entity::hash_set::ExtractIf")<'\_, F>

where F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(&[Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/spawn_batch.rs.html#116-119)

### impl<I, T> [EntitySetIterator](trait.EntitySetIterator.html "trait bevy::ecs::entity::EntitySetIterator") for [SpawnBatchIter](../world/struct.SpawnBatchIter.html "struct bevy::ecs::world::SpawnBatchIter")<'\_, I>

where I: [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = T> + [FusedIterator](https://doc.rust-lang.org/nightly/core/iter/traits/marker/trait.FusedIterator.html "trait core::iter::traits::marker::FusedIterator"), T: [Bundle](../../prelude/trait.Bundle.html "trait bevy::prelude::Bundle"), <T as [DynamicBundle](../bundle/trait.DynamicBundle.html "trait bevy::ecs::bundle::DynamicBundle")\>::[Effect](../bundle/trait.DynamicBundle.html#associatedtype.Effect "type bevy::ecs::bundle::DynamicBundle::Effect"): [NoBundleEffect](../bundle/trait.NoBundleEffect.html "trait bevy::ecs::bundle::NoBundleEffect"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/entity_set.rs.html#273)

### impl<I> [EntitySetIterator](trait.EntitySetIterator.html "trait bevy::ecs::entity::EntitySetIterator") for [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<I>

where I: [EntitySetIterator](trait.EntitySetIterator.html "trait bevy::ecs::entity::EntitySetIterator") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/entity_set.rs.html#454)

### impl<I> [EntitySetIterator](trait.EntitySetIterator.html "trait bevy::ecs::entity::EntitySetIterator") for [UniqueEntityIter](struct.UniqueEntityIter.html "struct bevy::ecs::entity::UniqueEntityIter")<I>

where I: [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator"), <I as [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")\>::[Item](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "type core::iter::traits::iterator::Iterator::Item"): [EntityEquivalent](trait.EntityEquivalent.html "trait bevy::ecs::entity::EntityEquivalent"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/hash_map.rs.html#306)

### impl<V> [EntitySetIterator](trait.EntitySetIterator.html "trait bevy::ecs::entity::EntitySetIterator") for bevy::ecs::entity::hash\_map::[IntoKeys](hash_map/struct.IntoKeys.html "struct bevy::ecs::entity::hash_map::IntoKeys")<V>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#1345)

### impl<V> [EntitySetIterator](trait.EntitySetIterator.html "trait bevy::ecs::entity::EntitySetIterator") for bevy::ecs::entity::index\_map::[IntoKeys](index_map/struct.IntoKeys.html "struct bevy::ecs::entity::index_map::IntoKeys")<V>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/hash_map.rs.html#234)

### impl<V> [EntitySetIterator](trait.EntitySetIterator.html "trait bevy::ecs::entity::EntitySetIterator") for bevy::ecs::entity::hash\_map::[Keys](hash_map/struct.Keys.html "struct bevy::ecs::entity::hash_map::Keys")<'\_, V>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#1271)

### impl<V> [EntitySetIterator](trait.EntitySetIterator.html "trait bevy::ecs::entity::EntitySetIterator") for bevy::ecs::entity::index\_map::[Keys](index_map/struct.Keys.html "struct bevy::ecs::entity::index_map::Keys")<'\_, V>