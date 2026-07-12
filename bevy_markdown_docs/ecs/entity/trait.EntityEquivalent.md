[bevy](../../index.html)::[ecs](../index.html)::[entity](index.html)

# Trait EntityEquivalent 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/entity_set.rs.html#68)

```rust
pub unsafe trait EntityEquivalent: ContainsEntity + Eq { }
```

A trait for types that represent an [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity").

Comparison trait behavior between an [`EntityEquivalent`](trait.EntityEquivalent.html "trait bevy::ecs::entity::EntityEquivalent") type and its underlying entity will match. This property includes [`PartialEq`](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq"), [`Eq`](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq"), [`PartialOrd`](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html "trait core::cmp::PartialOrd"), [`Ord`](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html "trait core::cmp::Ord") and [`Hash`](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash"), and remains even after [`Clone`](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") and/or [`Borrow`](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html "trait core::borrow::Borrow") calls.

## Safety

Any [`PartialEq`](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq"), [`Eq`](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq"), [`PartialOrd`](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html "trait core::cmp::PartialOrd"), and [`Ord`](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html "trait core::cmp::Ord") impls must evaluate the same for `Self` and its underlying entity. `x.entity() == y.entity()` must be equivalent to `x == y`.

The above equivalence must also hold through and between calls to any [`Clone`](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") and [`Borrow`](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html "trait core::borrow::Borrow")/[`BorrowMut`](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html "trait core::borrow::BorrowMut") impls in place of [`entity()`](../../prelude/trait.ContainsEntity.html#tymethod.entity "method bevy::prelude::ContainsEntity::entity").

The result of [`entity()`](../../prelude/trait.ContainsEntity.html#tymethod.entity "method bevy::prelude::ContainsEntity::entity") must be unaffected by any interior mutability.

The aforementioned properties imply determinism in both [`entity()`](../../prelude/trait.ContainsEntity.html#tymethod.entity "method bevy::prelude::ContainsEntity::entity") calls and comparison trait behavior.

All [`Hash`](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash") impls except that for [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity") must delegate to the [`Hash`](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash") impl of another [`EntityEquivalent`](trait.EntityEquivalent.html "trait bevy::ecs::entity::EntityEquivalent") type. All conversions to the delegatee within the [`Hash`](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash") impl must follow [`entity()`](../../prelude/trait.ContainsEntity.html#tymethod.entity "method bevy::prelude::ContainsEntity::entity") equivalence.

It should be noted that [`Hash`](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash") is _not_ a comparison trait, and with [`Hash::hash`](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash "method core::hash::Hash::hash") being forcibly generic over all [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher")s, **cannot** guarantee determinism or uniqueness of any final hash values on its own. To obtain hash values forming the same total order as [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity"), any [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher") used must be deterministic and concerning [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity"), collisionless. Standard library hash collections handle collisions with an [`Eq`](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") fallback, but do not account for determinism when [`BuildHasher`](https://doc.rust-lang.org/nightly/core/hash/trait.BuildHasher.html "trait core::hash::BuildHasher") is unspecified.

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementations on Foreign Types

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/entity_set.rs.html#90)

### impl<T> [EntityEquivalent](trait.EntityEquivalent.html "trait bevy::ecs::entity::EntityEquivalent") for [&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

where T: [EntityEquivalent](trait.EntityEquivalent.html "trait bevy::ecs::entity::EntityEquivalent"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/entity_set.rs.html#102)

### impl<T> [EntityEquivalent](trait.EntityEquivalent.html "trait bevy::ecs::entity::EntityEquivalent") for [&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

where T: [EntityEquivalent](trait.EntityEquivalent.html "trait bevy::ecs::entity::EntityEquivalent"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/entity_set.rs.html#126)

### impl<T> [EntityEquivalent](trait.EntityEquivalent.html "trait bevy::ecs::entity::EntityEquivalent") for [Rc](https://doc.rust-lang.org/nightly/alloc/rc/struct.Rc.html "struct alloc::rc::Rc")<T>

where T: [EntityEquivalent](trait.EntityEquivalent.html "trait bevy::ecs::entity::EntityEquivalent"),

## Implementors

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/entity_set.rs.html#78)

### impl [EntityEquivalent](trait.EntityEquivalent.html "trait bevy::ecs::entity::EntityEquivalent") for [Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/entity_mut.rs.html#871)

### impl [EntityEquivalent](trait.EntityEquivalent.html "trait bevy::ecs::entity::EntityEquivalent") for [EntityMut](../../prelude/struct.EntityMut.html "struct bevy::prelude::EntityMut")<'\_>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/entity_ref.rs.html#361)

### impl [EntityEquivalent](trait.EntityEquivalent.html "trait bevy::ecs::entity::EntityEquivalent") for [EntityRef](../../prelude/struct.EntityRef.html "struct bevy::prelude::EntityRef")<'\_>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/filtered.rs.html#798)

### impl [EntityEquivalent](trait.EntityEquivalent.html "trait bevy::ecs::entity::EntityEquivalent") for [FilteredEntityMut](../world/struct.FilteredEntityMut.html "struct bevy::ecs::world::FilteredEntityMut")<'\_, '\_>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/filtered.rs.html#281)

### impl [EntityEquivalent](trait.EntityEquivalent.html "trait bevy::ecs::entity::EntityEquivalent") for [FilteredEntityRef](../world/struct.FilteredEntityRef.html "struct bevy::ecs::world::FilteredEntityRef")<'\_, '\_>

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/sync_world.rs.html#181)

### impl [EntityEquivalent](trait.EntityEquivalent.html "trait bevy::ecs::entity::EntityEquivalent") for [MainEntity](../../render/sync_world/struct.MainEntity.html "struct bevy::render::sync_world::MainEntity")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/sync_world.rs.html#153)

### impl [EntityEquivalent](trait.EntityEquivalent.html "trait bevy::ecs::entity::EntityEquivalent") for [RenderEntity](../../render/sync_world/struct.RenderEntity.html "struct bevy::render::sync_world::RenderEntity")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/except.rs.html#543)

### impl<B> [EntityEquivalent](trait.EntityEquivalent.html "trait bevy::ecs::entity::EntityEquivalent") for [EntityMutExcept](../world/struct.EntityMutExcept.html "struct bevy::ecs::world::EntityMutExcept")<'\_, '\_, B>

where B: [Bundle](../../prelude/trait.Bundle.html "trait bevy::prelude::Bundle"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/except.rs.html#259)

### impl<B> [EntityEquivalent](trait.EntityEquivalent.html "trait bevy::ecs::entity::EntityEquivalent") for [EntityRefExcept](../world/struct.EntityRefExcept.html "struct bevy::ecs::world::EntityRefExcept")<'\_, '\_, B>

where B: [Bundle](../../prelude/trait.Bundle.html "trait bevy::prelude::Bundle"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/entity_set.rs.html#138)

### impl<T> [EntityEquivalent](trait.EntityEquivalent.html "trait bevy::ecs::entity::EntityEquivalent") for [Arc](../../platform/sync/struct.Arc.html "struct bevy::platform::sync::Arc")<T>

where T: [EntityEquivalent](trait.EntityEquivalent.html "trait bevy::ecs::entity::EntityEquivalent"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/entity_set.rs.html#114)

### impl<T> [EntityEquivalent](trait.EntityEquivalent.html "trait bevy::ecs::entity::EntityEquivalent") for [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<T>

where T: [EntityEquivalent](trait.EntityEquivalent.html "trait bevy::ecs::entity::EntityEquivalent"),