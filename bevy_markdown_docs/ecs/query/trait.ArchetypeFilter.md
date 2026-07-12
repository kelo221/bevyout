[bevy](../../index.html)::[ecs](../index.html)::[query](index.html)

# Trait ArchetypeFilter 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/filter.rs.html#1273)

```rust
pub trait ArchetypeFilter: QueryFilter { }
```

A marker trait to indicate that the filter works at an archetype level.

This is needed to:

*   implement [`ExactSizeIterator`](https://doc.rust-lang.org/nightly/core/iter/traits/exact_size/trait.ExactSizeIterator.html "trait core::iter::traits::exact_size::ExactSizeIterator") for [`QueryIter`](struct.QueryIter.html "struct bevy::ecs::query::QueryIter") that contains archetype-level filters.
*   ensure table filtering for [`QueryContiguousIter`](struct.QueryContiguousIter.html "struct bevy::ecs::query::QueryContiguousIter").

The trait must only be implemented for filters where its corresponding [`QueryFilter::IS_ARCHETYPAL`](trait.QueryFilter.html#associatedconstant.IS_ARCHETYPAL "associated constant bevy::ecs::query::QueryFilter::IS_ARCHETYPAL") is [`true`](https://doc.rust-lang.org/nightly/std/primitive.bool.html "primitive bool"). As such, only the [`With`](../../prelude/struct.With.html "struct bevy::prelude::With") and [`Without`](../../prelude/struct.Without.html "struct bevy::prelude::Without") filters can implement the trait. [Tuples](https://doc.rust-lang.org/nightly/std/primitive.tuple.html "primitive tuple") and [`Or`](../../prelude/struct.Or.html "struct bevy::prelude::Or") filters are automatically implemented with the trait only if its containing types also implement the same trait.

[`Added`](../../prelude/struct.Added.html "struct bevy::prelude::Added"), [`Changed`](../../prelude/struct.Changed.html "struct bevy::prelude::Changed") and [`Spawned`](struct.Spawned.html "struct bevy::ecs::query::Spawned") work with entities, and therefore are not archetypal. As such they do not implement [`ArchetypeFilter`](trait.ArchetypeFilter.html "trait bevy::ecs::query::ArchetypeFilter").

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementations on Foreign Types

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/filter.rs.html#1293-1299)

### impl [ArchetypeFilter](trait.ArchetypeFilter.html "trait bevy::ecs::query::ArchetypeFilter") for [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/filter.rs.html#1293-1299)

### impl<F> [ArchetypeFilter](trait.ArchetypeFilter.html "trait bevy::ecs::query::ArchetypeFilter") for [(F₁, F₂, …, Fₙ)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html#trait-implementations-1)

where F: [ArchetypeFilter](trait.ArchetypeFilter.html "trait bevy::ecs::query::ArchetypeFilter"),

This trait is implemented for tuples up to 16 items long.

## Implementors

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/filter.rs.html#1301-1307)

### impl [ArchetypeFilter](trait.ArchetypeFilter.html "trait bevy::ecs::query::ArchetypeFilter") for [Or](../../prelude/struct.Or.html "struct bevy::prelude::Or")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)\>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/filter.rs.html#1301-1307)

### impl<F> [ArchetypeFilter](trait.ArchetypeFilter.html "trait bevy::ecs::query::ArchetypeFilter") for [Or](../../prelude/struct.Or.html "struct bevy::prelude::Or")<[(F₁, F₂, …, Fₙ)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html#trait-implementations-1)\>

where F: [ArchetypeFilter](trait.ArchetypeFilter.html "trait bevy::ecs::query::ArchetypeFilter"),

This trait is implemented for tuples up to 16 items long.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/filter.rs.html#1275)

### impl<T> [ArchetypeFilter](trait.ArchetypeFilter.html "trait bevy::ecs::query::ArchetypeFilter") for [With](../../prelude/struct.With.html "struct bevy::prelude::With")<T>

where T: [Component](../../prelude/trait.Component.html "trait bevy::prelude::Component"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/filter.rs.html#1277)

### impl<T> [ArchetypeFilter](trait.ArchetypeFilter.html "trait bevy::ecs::query::ArchetypeFilter") for [Without](../../prelude/struct.Without.html "struct bevy::prelude::Without")<T>

where T: [Component](../../prelude/trait.Component.html "trait bevy::prelude::Component"),