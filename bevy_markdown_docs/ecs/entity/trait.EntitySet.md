[bevy](../../index.html)::[ecs](../index.html)::[entity](index.html)

# Trait EntitySet 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/entity_set.rs.html#158)

```rust
pub trait EntitySet: IntoIteratorwhere
    Self::IntoIter: EntitySetIterator,{ }
```

A set of unique entities.

Any element returned by [`Self::IntoIter`](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.IntoIter "associated type core::iter::traits::collect::IntoIterator::IntoIter") will compare non-equal to every other element in the iterator. As a consequence, [`into_iter()`](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#tymethod.into_iter "method core::iter::traits::collect::IntoIterator::into_iter") on `EntitySet` will always produce another `EntitySet`.

Implementing this trait allows for unique query iteration over a list of entities. See [`iter_many_unique`](../../prelude/struct.Query.html#method.iter_many_unique "method bevy::prelude::Query::iter_many_unique") and [`iter_many_unique_mut`](../../prelude/struct.Query.html#method.iter_many_unique_mut "method bevy::prelude::Query::iter_many_unique_mut").

Note that there is no guarantee of the [`IntoIterator`](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator") impl being deterministic, it might return different iterators when called multiple times. Neither is there a guarantee that the comparison trait impls of `EntitySet` match that of the respective [`EntitySetIterator`](trait.EntitySetIterator.html "trait bevy::ecs::entity::EntitySetIterator") (or of a [`Vec`](../../prelude/struct.Vec.html "struct bevy::prelude::Vec") collected from its elements).

## Dyn Compatibility

This trait **is** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/entity_set.rs.html#160)

### impl<T> [EntitySet](trait.EntitySet.html "trait bevy::ecs::entity::EntitySet") for T

where T: [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator"), <T as [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")\>::[IntoIter](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.IntoIter "type core::iter::traits::collect::IntoIterator::IntoIter"): [EntitySetIterator](trait.EntitySetIterator.html "trait bevy::ecs::entity::EntitySetIterator"),