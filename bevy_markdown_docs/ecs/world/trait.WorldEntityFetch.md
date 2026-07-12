[bevy](../../index.html)::[ecs](../index.html)::[world](index.html)

# Trait WorldEntityFetch 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_fetch.rs.html#123)

```rust
pub unsafe trait WorldEntityFetch {
    type Ref<'w>;
    type Mut<'w>;
    type DeferredMut<'w>;

    // Required methods
    unsafe fn fetch_ref(
        self,
        cell: UnsafeWorldCell<'_>,
    ) -> Result<Self::Ref<'_>, EntityNotSpawnedError>;
    unsafe fn fetch_mut(
        self,
        cell: UnsafeWorldCell<'_>,
    ) -> Result<Self::Mut<'_>, EntityMutableFetchError>;
    unsafe fn fetch_deferred_mut(
        self,
        cell: UnsafeWorldCell<'_>,
    ) -> Result<Self::DeferredMut<'_>, EntityMutableFetchError>;
}
```

Types that can be used to fetch [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity") references from a [`World`](../../prelude/struct.World.html "struct bevy::prelude::World").

Provided implementations are:

*   [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity"): Fetch a single entity.
*   `[Entity; N]`/`&[Entity; N]`: Fetch multiple entities, receiving a same-sized array of references.
*   `&[Entity]`: Fetch multiple entities, receiving a vector of references.
*   [`&EntityHashSet`](../entity/struct.EntityHashSet.html "struct bevy::ecs::entity::EntityHashSet"): Fetch multiple entities, receiving a hash map of [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity") IDs to references.

## Performance

*   The slice and array implementations perform an aliased mutability check in [`WorldEntityFetch::fetch_mut`](trait.WorldEntityFetch.html#tymethod.fetch_mut "method bevy::ecs::world::WorldEntityFetch::fetch_mut") that is `O(N^2)`.
*   The single [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity") implementation performs no such check as only one reference is returned.

## Safety

Implementor must ensure that:

*   No aliased mutability is caused by the returned references.
*   [`WorldEntityFetch::fetch_ref`](trait.WorldEntityFetch.html#tymethod.fetch_ref "method bevy::ecs::world::WorldEntityFetch::fetch_ref") returns only read-only references.
*   [`WorldEntityFetch::fetch_deferred_mut`](trait.WorldEntityFetch.html#tymethod.fetch_deferred_mut "method bevy::ecs::world::WorldEntityFetch::fetch_deferred_mut") returns only non-structurally-mutable references.

## Required Associated Types

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_fetch.rs.html#125)

#### type [Ref](#associatedtype.Ref)<'w>

The read-only reference type returned by [`WorldEntityFetch::fetch_ref`](trait.WorldEntityFetch.html#tymethod.fetch_ref "method bevy::ecs::world::WorldEntityFetch::fetch_ref").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_fetch.rs.html#128)

#### type [Mut](#associatedtype.Mut)<'w>

The mutable reference type returned by [`WorldEntityFetch::fetch_mut`](trait.WorldEntityFetch.html#tymethod.fetch_mut "method bevy::ecs::world::WorldEntityFetch::fetch_mut").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_fetch.rs.html#132)

#### type [DeferredMut](#associatedtype.DeferredMut)<'w>

The mutable reference type returned by [`WorldEntityFetch::fetch_deferred_mut`](trait.WorldEntityFetch.html#tymethod.fetch_deferred_mut "method bevy::ecs::world::WorldEntityFetch::fetch_deferred_mut"), but without structural mutability.

## Required Methods

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_fetch.rs.html#146-149)

#### unsafe fn [fetch\_ref](#tymethod.fetch_ref)( self, cell: [UnsafeWorldCell](unsafe_world_cell/struct.UnsafeWorldCell.html "struct bevy::ecs::world::unsafe_world_cell::UnsafeWorldCell")<'\_>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Self::[Ref](trait.WorldEntityFetch.html#associatedtype.Ref "type bevy::ecs::world::WorldEntityFetch::Ref")<'\_>, [EntityNotSpawnedError](../entity/enum.EntityNotSpawnedError.html "enum bevy::ecs::entity::EntityNotSpawnedError")\>

Returns read-only reference(s) to the entities with the given [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity") IDs, as determined by `self`.

##### Safety

It is the caller’s responsibility to ensure that:

*   The given [`UnsafeWorldCell`](unsafe_world_cell/struct.UnsafeWorldCell.html "struct bevy::ecs::world::unsafe_world_cell::UnsafeWorldCell") has read-only access to the fetched entities.
*   No other mutable references to the fetched entities exist at the same time.

##### Errors

*   Returns [`EntityNotSpawnedError`](../entity/enum.EntityNotSpawnedError.html "enum bevy::ecs::entity::EntityNotSpawnedError") if the entity does not exist.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_fetch.rs.html#165-168)

#### unsafe fn [fetch\_mut](#tymethod.fetch_mut)( self, cell: [UnsafeWorldCell](unsafe_world_cell/struct.UnsafeWorldCell.html "struct bevy::ecs::world::unsafe_world_cell::UnsafeWorldCell")<'\_>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Self::[Mut](trait.WorldEntityFetch.html#associatedtype.Mut "type bevy::ecs::world::WorldEntityFetch::Mut")<'\_>, [EntityMutableFetchError](error/enum.EntityMutableFetchError.html "enum bevy::ecs::world::error::EntityMutableFetchError")\>

Returns mutable reference(s) to the entities with the given [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity") IDs, as determined by `self`.

##### Safety

It is the caller’s responsibility to ensure that:

*   The given [`UnsafeWorldCell`](unsafe_world_cell/struct.UnsafeWorldCell.html "struct bevy::ecs::world::unsafe_world_cell::UnsafeWorldCell") has mutable access to the fetched entities.
*   No other references to the fetched entities exist at the same time.

##### Errors

*   Returns [`EntityMutableFetchError::NotSpawned`](error/enum.EntityMutableFetchError.html#variant.NotSpawned "variant bevy::ecs::world::error::EntityMutableFetchError::NotSpawned") if the entity does not exist.
*   Returns [`EntityMutableFetchError::AliasedMutability`](error/enum.EntityMutableFetchError.html#variant.AliasedMutability "variant bevy::ecs::world::error::EntityMutableFetchError::AliasedMutability") if the entity was requested mutably more than once.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_fetch.rs.html#188-191)

#### unsafe fn [fetch\_deferred\_mut](#tymethod.fetch_deferred_mut)( self, cell: [UnsafeWorldCell](unsafe_world_cell/struct.UnsafeWorldCell.html "struct bevy::ecs::world::unsafe_world_cell::UnsafeWorldCell")<'\_>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Self::[DeferredMut](trait.WorldEntityFetch.html#associatedtype.DeferredMut "type bevy::ecs::world::WorldEntityFetch::DeferredMut")<'\_>, [EntityMutableFetchError](error/enum.EntityMutableFetchError.html "enum bevy::ecs::world::error::EntityMutableFetchError")\>

Returns mutable reference(s) to the entities with the given [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity") IDs, as determined by `self`, but without structural mutability.

No structural mutability means components cannot be removed from the entity, new components cannot be added to the entity, and the entity cannot be despawned.

##### Safety

It is the caller’s responsibility to ensure that:

*   The given [`UnsafeWorldCell`](unsafe_world_cell/struct.UnsafeWorldCell.html "struct bevy::ecs::world::unsafe_world_cell::UnsafeWorldCell") has mutable access to the fetched entities.
*   No other references to the fetched entities exist at the same time.

##### Errors

*   Returns [`EntityMutableFetchError::NotSpawned`](error/enum.EntityMutableFetchError.html#variant.NotSpawned "variant bevy::ecs::world::error::EntityMutableFetchError::NotSpawned") if the entity does not exist.
*   Returns [`EntityMutableFetchError::AliasedMutability`](error/enum.EntityMutableFetchError.html#variant.AliasedMutability "variant bevy::ecs::world::error::EntityMutableFetchError::AliasedMutability") if the entity was requested mutably more than once.

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementations on Foreign Types

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_fetch.rs.html#342)

### impl [WorldEntityFetch](trait.WorldEntityFetch.html "trait bevy::ecs::world::WorldEntityFetch") for &\[[Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity")\]

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_fetch.rs.html#343)

#### type [Ref](#associatedtype.Ref)<'w> = [Vec](../../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[EntityRef](../../prelude/struct.EntityRef.html "struct bevy::prelude::EntityRef")<'w>>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_fetch.rs.html#344)

#### type [Mut](#associatedtype.Mut)<'w> = [Vec](../../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[EntityMut](../../prelude/struct.EntityMut.html "struct bevy::prelude::EntityMut")<'w>>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_fetch.rs.html#345)

#### type [DeferredMut](#associatedtype.DeferredMut)<'w> = [Vec](../../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[EntityMut](../../prelude/struct.EntityMut.html "struct bevy::prelude::EntityMut")<'w>>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_fetch.rs.html#348-351)

#### unsafe fn [fetch\_ref](#tymethod.fetch_ref)( self, cell: [UnsafeWorldCell](unsafe_world_cell/struct.UnsafeWorldCell.html "struct bevy::ecs::world::unsafe_world_cell::UnsafeWorldCell")<'\_>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<&\[[Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity")\] as [WorldEntityFetch](trait.WorldEntityFetch.html "trait bevy::ecs::world::WorldEntityFetch")\>::[Ref](trait.WorldEntityFetch.html#associatedtype.Ref "type bevy::ecs::world::WorldEntityFetch::Ref")<'\_>, [EntityNotSpawnedError](../entity/enum.EntityNotSpawnedError.html "enum bevy::ecs::entity::EntityNotSpawnedError")\>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_fetch.rs.html#363-366)

#### unsafe fn [fetch\_mut](#tymethod.fetch_mut)( self, cell: [UnsafeWorldCell](unsafe_world_cell/struct.UnsafeWorldCell.html "struct bevy::ecs::world::unsafe_world_cell::UnsafeWorldCell")<'\_>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<&\[[Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity")\] as [WorldEntityFetch](trait.WorldEntityFetch.html "trait bevy::ecs::world::WorldEntityFetch")\>::[Mut](trait.WorldEntityFetch.html#associatedtype.Mut "type bevy::ecs::world::WorldEntityFetch::Mut")<'\_>, [EntityMutableFetchError](error/enum.EntityMutableFetchError.html "enum bevy::ecs::world::error::EntityMutableFetchError")\>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_fetch.rs.html#387-390)

#### unsafe fn [fetch\_deferred\_mut](#tymethod.fetch_deferred_mut)( self, cell: [UnsafeWorldCell](unsafe_world_cell/struct.UnsafeWorldCell.html "struct bevy::ecs::world::unsafe_world_cell::UnsafeWorldCell")<'\_>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<&\[[Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity")\] as [WorldEntityFetch](trait.WorldEntityFetch.html "trait bevy::ecs::world::WorldEntityFetch")\>::[DeferredMut](trait.WorldEntityFetch.html#associatedtype.DeferredMut "type bevy::ecs::world::WorldEntityFetch::DeferredMut")<'\_>, [EntityMutableFetchError](error/enum.EntityMutableFetchError.html "enum bevy::ecs::world::error::EntityMutableFetchError")\>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_fetch.rs.html#277)

### impl<const N: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\> [WorldEntityFetch](trait.WorldEntityFetch.html "trait bevy::ecs::world::WorldEntityFetch") for &\[[Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity"); [N](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_fetch.rs.html#278)

#### type [Ref](#associatedtype.Ref)<'w> = \[[EntityRef](../../prelude/struct.EntityRef.html "struct bevy::prelude::EntityRef")<'w>; [N](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_fetch.rs.html#279)

#### type [Mut](#associatedtype.Mut)<'w> = \[[EntityMut](../../prelude/struct.EntityMut.html "struct bevy::prelude::EntityMut")<'w>; [N](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_fetch.rs.html#280)

#### type [DeferredMut](#associatedtype.DeferredMut)<'w> = \[[EntityMut](../../prelude/struct.EntityMut.html "struct bevy::prelude::EntityMut")<'w>; [N](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_fetch.rs.html#283-286)

#### unsafe fn [fetch\_ref](#tymethod.fetch_ref)( self, cell: [UnsafeWorldCell](unsafe_world_cell/struct.UnsafeWorldCell.html "struct bevy::ecs::world::unsafe_world_cell::UnsafeWorldCell")<'\_>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<&\[[Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity"); [N](https://doc.rust-lang.org/nightly/std/primitive.array.html)\] as [WorldEntityFetch](trait.WorldEntityFetch.html "trait bevy::ecs::world::WorldEntityFetch")\>::[Ref](trait.WorldEntityFetch.html#associatedtype.Ref "type bevy::ecs::world::WorldEntityFetch::Ref")<'\_>, [EntityNotSpawnedError](../entity/enum.EntityNotSpawnedError.html "enum bevy::ecs::entity::EntityNotSpawnedError")\>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_fetch.rs.html#301-304)

#### unsafe fn [fetch\_mut](#tymethod.fetch_mut)( self, cell: [UnsafeWorldCell](unsafe_world_cell/struct.UnsafeWorldCell.html "struct bevy::ecs::world::unsafe_world_cell::UnsafeWorldCell")<'\_>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<&\[[Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity"); [N](https://doc.rust-lang.org/nightly/std/primitive.array.html)\] as [WorldEntityFetch](trait.WorldEntityFetch.html "trait bevy::ecs::world::WorldEntityFetch")\>::[Mut](trait.WorldEntityFetch.html#associatedtype.Mut "type bevy::ecs::world::WorldEntityFetch::Mut")<'\_>, [EntityMutableFetchError](error/enum.EntityMutableFetchError.html "enum bevy::ecs::world::error::EntityMutableFetchError")\>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_fetch.rs.html#328-331)

#### unsafe fn [fetch\_deferred\_mut](#tymethod.fetch_deferred_mut)( self, cell: [UnsafeWorldCell](unsafe_world_cell/struct.UnsafeWorldCell.html "struct bevy::ecs::world::unsafe_world_cell::UnsafeWorldCell")<'\_>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<&\[[Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity"); [N](https://doc.rust-lang.org/nightly/std/primitive.array.html)\] as [WorldEntityFetch](trait.WorldEntityFetch.html "trait bevy::ecs::world::WorldEntityFetch")\>::[DeferredMut](trait.WorldEntityFetch.html#associatedtype.DeferredMut "type bevy::ecs::world::WorldEntityFetch::DeferredMut")<'\_>, [EntityMutableFetchError](error/enum.EntityMutableFetchError.html "enum bevy::ecs::world::error::EntityMutableFetchError")\>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_fetch.rs.html#240)

### impl<const N: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\> [WorldEntityFetch](trait.WorldEntityFetch.html "trait bevy::ecs::world::WorldEntityFetch") for \[[Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity"); [N](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_fetch.rs.html#241)

#### type [Ref](#associatedtype.Ref)<'w> = \[[EntityRef](../../prelude/struct.EntityRef.html "struct bevy::prelude::EntityRef")<'w>; [N](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_fetch.rs.html#242)

#### type [Mut](#associatedtype.Mut)<'w> = \[[EntityMut](../../prelude/struct.EntityMut.html "struct bevy::prelude::EntityMut")<'w>; [N](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_fetch.rs.html#243)

#### type [DeferredMut](#associatedtype.DeferredMut)<'w> = \[[EntityMut](../../prelude/struct.EntityMut.html "struct bevy::prelude::EntityMut")<'w>; [N](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_fetch.rs.html#246-249)

#### unsafe fn [fetch\_ref](#tymethod.fetch_ref)( self, cell: [UnsafeWorldCell](unsafe_world_cell/struct.UnsafeWorldCell.html "struct bevy::ecs::world::unsafe_world_cell::UnsafeWorldCell")<'\_>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<\[[Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity"); [N](https://doc.rust-lang.org/nightly/std/primitive.array.html)\] as [WorldEntityFetch](trait.WorldEntityFetch.html "trait bevy::ecs::world::WorldEntityFetch")\>::[Ref](trait.WorldEntityFetch.html#associatedtype.Ref "type bevy::ecs::world::WorldEntityFetch::Ref")<'\_>, [EntityNotSpawnedError](../entity/enum.EntityNotSpawnedError.html "enum bevy::ecs::entity::EntityNotSpawnedError")\>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_fetch.rs.html#255-258)

#### unsafe fn [fetch\_mut](#tymethod.fetch_mut)( self, cell: [UnsafeWorldCell](unsafe_world_cell/struct.UnsafeWorldCell.html "struct bevy::ecs::world::unsafe_world_cell::UnsafeWorldCell")<'\_>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<\[[Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity"); [N](https://doc.rust-lang.org/nightly/std/primitive.array.html)\] as [WorldEntityFetch](trait.WorldEntityFetch.html "trait bevy::ecs::world::WorldEntityFetch")\>::[Mut](trait.WorldEntityFetch.html#associatedtype.Mut "type bevy::ecs::world::WorldEntityFetch::Mut")<'\_>, [EntityMutableFetchError](error/enum.EntityMutableFetchError.html "enum bevy::ecs::world::error::EntityMutableFetchError")\>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_fetch.rs.html#264-267)

#### unsafe fn [fetch\_deferred\_mut](#tymethod.fetch_deferred_mut)( self, cell: [UnsafeWorldCell](unsafe_world_cell/struct.UnsafeWorldCell.html "struct bevy::ecs::world::unsafe_world_cell::UnsafeWorldCell")<'\_>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<\[[Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity"); [N](https://doc.rust-lang.org/nightly/std/primitive.array.html)\] as [WorldEntityFetch](trait.WorldEntityFetch.html "trait bevy::ecs::world::WorldEntityFetch")\>::[DeferredMut](trait.WorldEntityFetch.html#associatedtype.DeferredMut "type bevy::ecs::world::WorldEntityFetch::DeferredMut")<'\_>, [EntityMutableFetchError](error/enum.EntityMutableFetchError.html "enum bevy::ecs::world::error::EntityMutableFetchError")\>

## Implementors

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_fetch.rs.html#401)

### impl [WorldEntityFetch](trait.WorldEntityFetch.html "trait bevy::ecs::world::WorldEntityFetch") for &[EntityHashSet](../entity/struct.EntityHashSet.html "struct bevy::ecs::entity::EntityHashSet")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_fetch.rs.html#402)

#### type [Ref](#associatedtype.Ref)<'w> = [EntityHashMap](../entity/struct.EntityHashMap.html "struct bevy::ecs::entity::EntityHashMap")<[EntityRef](../../prelude/struct.EntityRef.html "struct bevy::prelude::EntityRef")<'w>>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_fetch.rs.html#403)

#### type [Mut](#associatedtype.Mut)<'w> = [EntityHashMap](../entity/struct.EntityHashMap.html "struct bevy::ecs::entity::EntityHashMap")<[EntityMut](../../prelude/struct.EntityMut.html "struct bevy::prelude::EntityMut")<'w>>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_fetch.rs.html#404)

#### type [DeferredMut](#associatedtype.DeferredMut)<'w> = [EntityHashMap](../entity/struct.EntityHashMap.html "struct bevy::ecs::entity::EntityHashMap")<[EntityMut](../../prelude/struct.EntityMut.html "struct bevy::prelude::EntityMut")<'w>>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_fetch.rs.html#198)

### impl [WorldEntityFetch](trait.WorldEntityFetch.html "trait bevy::ecs::world::WorldEntityFetch") for [Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_fetch.rs.html#199)

#### type [Ref](#associatedtype.Ref)<'w> = [EntityRef](../../prelude/struct.EntityRef.html "struct bevy::prelude::EntityRef")<'w>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_fetch.rs.html#200)

#### type [Mut](#associatedtype.Mut)<'w> = [EntityWorldMut](../../prelude/struct.EntityWorldMut.html "struct bevy::prelude::EntityWorldMut")<'w>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_fetch.rs.html#201)

#### type [DeferredMut](#associatedtype.DeferredMut)<'w> = [EntityMut](../../prelude/struct.EntityMut.html "struct bevy::prelude::EntityMut")<'w>