[bevy](../../index.html)::[ecs](../index.html)::[world](index.html)

# Trait DynamicComponentFetch 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/component_fetch.rs.html#35)

```rust
pub unsafe trait DynamicComponentFetch {
    type Ref<'w>;
    type Mut<'w>;

    // Required methods
    unsafe fn fetch_ref(
        self,
        cell: UnsafeEntityCell<'_>,
    ) -> Result<Self::Ref<'_>, EntityComponentError>;
    unsafe fn fetch_mut(
        self,
        cell: UnsafeEntityCell<'_>,
    ) -> Result<Self::Mut<'_>, EntityComponentError>;
    unsafe fn fetch_mut_assume_mutable(
        self,
        cell: UnsafeEntityCell<'_>,
    ) -> Result<Self::Mut<'_>, EntityComponentError>;
}
```

Types that can be used to fetch components from an entity dynamically by [`ComponentId`](../component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId")s.

Provided implementations are:

*   [`ComponentId`](../component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId"): Returns a single untyped reference.
*   `[ComponentId; N]` and `&[ComponentId; N]`: Returns a same-sized array of untyped references.
*   `&[ComponentId]`: Returns a [`Vec`](../../prelude/struct.Vec.html "struct bevy::prelude::Vec") of untyped references.
*   [`&HashSet<ComponentId>`](../../platform/collections/struct.HashSet.html "struct bevy::platform::collections::HashSet"): Returns a [`HashMap`](../../platform/collections/struct.HashMap.html "struct bevy::platform::collections::HashMap") of IDs to untyped references.

## Performance

*   The slice and array implementations perform an aliased mutability check in [`DynamicComponentFetch::fetch_mut`](trait.DynamicComponentFetch.html#tymethod.fetch_mut "method bevy::ecs::world::DynamicComponentFetch::fetch_mut") that is `O(N^2)`.
*   The [`HashSet`](../../platform/collections/struct.HashSet.html "struct bevy::platform::collections::HashSet") implementation performs no such check as the type itself guarantees unique IDs.
*   The single [`ComponentId`](../component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId") implementation performs no such check as only one reference is returned.

## Safety

Implementor must ensure that:

*   No aliased mutability is caused by the returned references.
*   [`DynamicComponentFetch::fetch_ref`](trait.DynamicComponentFetch.html#tymethod.fetch_ref "method bevy::ecs::world::DynamicComponentFetch::fetch_ref") returns only read-only references.

## Required Associated Types

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/component_fetch.rs.html#37)

#### type [Ref](#associatedtype.Ref)<'w>

The read-only reference type returned by [`DynamicComponentFetch::fetch_ref`](trait.DynamicComponentFetch.html#tymethod.fetch_ref "method bevy::ecs::world::DynamicComponentFetch::fetch_ref").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/component_fetch.rs.html#40)

#### type [Mut](#associatedtype.Mut)<'w>

The mutable reference type returned by [`DynamicComponentFetch::fetch_mut`](trait.DynamicComponentFetch.html#tymethod.fetch_mut "method bevy::ecs::world::DynamicComponentFetch::fetch_mut").

## Required Methods

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/component_fetch.rs.html#54-57)

#### unsafe fn [fetch\_ref](#tymethod.fetch_ref)( self, cell: [UnsafeEntityCell](unsafe_world_cell/struct.UnsafeEntityCell.html "struct bevy::ecs::world::unsafe_world_cell::UnsafeEntityCell")<'\_>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Self::[Ref](trait.DynamicComponentFetch.html#associatedtype.Ref "type bevy::ecs::world::DynamicComponentFetch::Ref")<'\_>, [EntityComponentError](error/enum.EntityComponentError.html "enum bevy::ecs::world::error::EntityComponentError")\>

Returns untyped read-only reference(s) to the component(s) with the given [`ComponentId`](../component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId")s, as determined by `self`.

##### Safety

It is the caller’s responsibility to ensure that:

*   The given [`UnsafeEntityCell`](unsafe_world_cell/struct.UnsafeEntityCell.html "struct bevy::ecs::world::unsafe_world_cell::UnsafeEntityCell") has read-only access to the fetched components.
*   No other mutable references to the fetched components exist at the same time.

##### Errors

*   Returns [`EntityComponentError::MissingComponent`](error/enum.EntityComponentError.html#variant.MissingComponent "variant bevy::ecs::world::error::EntityComponentError::MissingComponent") if a component is missing from the entity.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/component_fetch.rs.html#72-75)

#### unsafe fn [fetch\_mut](#tymethod.fetch_mut)( self, cell: [UnsafeEntityCell](unsafe_world_cell/struct.UnsafeEntityCell.html "struct bevy::ecs::world::unsafe_world_cell::UnsafeEntityCell")<'\_>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Self::[Mut](trait.DynamicComponentFetch.html#associatedtype.Mut "type bevy::ecs::world::DynamicComponentFetch::Mut")<'\_>, [EntityComponentError](error/enum.EntityComponentError.html "enum bevy::ecs::world::error::EntityComponentError")\>

Returns untyped mutable reference(s) to the component(s) with the given [`ComponentId`](../component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId")s, as determined by `self`.

##### Safety

It is the caller’s responsibility to ensure that:

*   The given [`UnsafeEntityCell`](unsafe_world_cell/struct.UnsafeEntityCell.html "struct bevy::ecs::world::unsafe_world_cell::UnsafeEntityCell") has mutable access to the fetched components.
*   No other references to the fetched components exist at the same time.

##### Errors

*   Returns [`EntityComponentError::MissingComponent`](error/enum.EntityComponentError.html#variant.MissingComponent "variant bevy::ecs::world::error::EntityComponentError::MissingComponent") if a component is missing from the entity.
*   Returns [`EntityComponentError::AliasedMutability`](error/enum.EntityComponentError.html#variant.AliasedMutability "variant bevy::ecs::world::error::EntityComponentError::AliasedMutability") if a component is requested multiple times.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/component_fetch.rs.html#92-95)

#### unsafe fn [fetch\_mut\_assume\_mutable](#tymethod.fetch_mut_assume_mutable)( self, cell: [UnsafeEntityCell](unsafe_world_cell/struct.UnsafeEntityCell.html "struct bevy::ecs::world::unsafe_world_cell::UnsafeEntityCell")<'\_>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Self::[Mut](trait.DynamicComponentFetch.html#associatedtype.Mut "type bevy::ecs::world::DynamicComponentFetch::Mut")<'\_>, [EntityComponentError](error/enum.EntityComponentError.html "enum bevy::ecs::world::error::EntityComponentError")\>

Returns untyped mutable reference(s) to the component(s) with the given [`ComponentId`](../component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId")s, as determined by `self`. Assumes all [`ComponentId`](../component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId")s refer to mutable components.

##### Safety

It is the caller’s responsibility to ensure that:

*   The given [`UnsafeEntityCell`](unsafe_world_cell/struct.UnsafeEntityCell.html "struct bevy::ecs::world::unsafe_world_cell::UnsafeEntityCell") has mutable access to the fetched components.
*   No other references to the fetched components exist at the same time.
*   The requested components are all mutable.

##### Errors

*   Returns [`EntityComponentError::MissingComponent`](error/enum.EntityComponentError.html#variant.MissingComponent "variant bevy::ecs::world::error::EntityComponentError::MissingComponent") if a component is missing from the entity.
*   Returns [`EntityComponentError::AliasedMutability`](error/enum.EntityComponentError.html#variant.AliasedMutability "variant bevy::ecs::world::error::EntityComponentError::AliasedMutability") if a component is requested multiple times.

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementations on Foreign Types

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/component_fetch.rs.html#249)

### impl [DynamicComponentFetch](trait.DynamicComponentFetch.html "trait bevy::ecs::world::DynamicComponentFetch") for &\[[ComponentId](../component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId")\]

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/component_fetch.rs.html#250)

#### type [Ref](#associatedtype.Ref)<'w> = [Vec](../../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[Ptr](../ptr/struct.Ptr.html "struct bevy::ecs::ptr::Ptr")<'w>>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/component_fetch.rs.html#251)

#### type [Mut](#associatedtype.Mut)<'w> = [Vec](../../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[MutUntyped](../change_detection/struct.MutUntyped.html "struct bevy::ecs::change_detection::MutUntyped")<'w>>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/component_fetch.rs.html#253-256)

#### unsafe fn [fetch\_ref](#tymethod.fetch_ref)( self, cell: [UnsafeEntityCell](unsafe_world_cell/struct.UnsafeEntityCell.html "struct bevy::ecs::world::unsafe_world_cell::UnsafeEntityCell")<'\_>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<&\[[ComponentId](../component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId")\] as [DynamicComponentFetch](trait.DynamicComponentFetch.html "trait bevy::ecs::world::DynamicComponentFetch")\>::[Ref](trait.DynamicComponentFetch.html#associatedtype.Ref "type bevy::ecs::world::DynamicComponentFetch::Ref")<'\_>, [EntityComponentError](error/enum.EntityComponentError.html "enum bevy::ecs::world::error::EntityComponentError")\>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/component_fetch.rs.html#267-270)

#### unsafe fn [fetch\_mut](#tymethod.fetch_mut)( self, cell: [UnsafeEntityCell](unsafe_world_cell/struct.UnsafeEntityCell.html "struct bevy::ecs::world::unsafe_world_cell::UnsafeEntityCell")<'\_>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<&\[[ComponentId](../component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId")\] as [DynamicComponentFetch](trait.DynamicComponentFetch.html "trait bevy::ecs::world::DynamicComponentFetch")\>::[Mut](trait.DynamicComponentFetch.html#associatedtype.Mut "type bevy::ecs::world::DynamicComponentFetch::Mut")<'\_>, [EntityComponentError](error/enum.EntityComponentError.html "enum bevy::ecs::world::error::EntityComponentError")\>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/component_fetch.rs.html#291-294)

#### unsafe fn [fetch\_mut\_assume\_mutable](#tymethod.fetch_mut_assume_mutable)( self, cell: [UnsafeEntityCell](unsafe_world_cell/struct.UnsafeEntityCell.html "struct bevy::ecs::world::unsafe_world_cell::UnsafeEntityCell")<'\_>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<&\[[ComponentId](../component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId")\] as [DynamicComponentFetch](trait.DynamicComponentFetch.html "trait bevy::ecs::world::DynamicComponentFetch")\>::[Mut](trait.DynamicComponentFetch.html#associatedtype.Mut "type bevy::ecs::world::DynamicComponentFetch::Mut")<'\_>, [EntityComponentError](error/enum.EntityComponentError.html "enum bevy::ecs::world::error::EntityComponentError")\>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/component_fetch.rs.html#167)

### impl<const N: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\> [DynamicComponentFetch](trait.DynamicComponentFetch.html "trait bevy::ecs::world::DynamicComponentFetch") for &\[[ComponentId](../component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId"); [N](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/component_fetch.rs.html#168)

#### type [Ref](#associatedtype.Ref)<'w> = \[[Ptr](../ptr/struct.Ptr.html "struct bevy::ecs::ptr::Ptr")<'w>; [N](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/component_fetch.rs.html#169)

#### type [Mut](#associatedtype.Mut)<'w> = \[[MutUntyped](../change_detection/struct.MutUntyped.html "struct bevy::ecs::change_detection::MutUntyped")<'w>; [N](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/component_fetch.rs.html#171-174)

#### unsafe fn [fetch\_ref](#tymethod.fetch_ref)( self, cell: [UnsafeEntityCell](unsafe_world_cell/struct.UnsafeEntityCell.html "struct bevy::ecs::world::unsafe_world_cell::UnsafeEntityCell")<'\_>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<&\[[ComponentId](../component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId"); [N](https://doc.rust-lang.org/nightly/std/primitive.array.html)\] as [DynamicComponentFetch](trait.DynamicComponentFetch.html "trait bevy::ecs::world::DynamicComponentFetch")\>::[Ref](trait.DynamicComponentFetch.html#associatedtype.Ref "type bevy::ecs::world::DynamicComponentFetch::Ref")<'\_>, [EntityComponentError](error/enum.EntityComponentError.html "enum bevy::ecs::world::error::EntityComponentError")\>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/component_fetch.rs.html#189-192)

#### unsafe fn [fetch\_mut](#tymethod.fetch_mut)( self, cell: [UnsafeEntityCell](unsafe_world_cell/struct.UnsafeEntityCell.html "struct bevy::ecs::world::unsafe_world_cell::UnsafeEntityCell")<'\_>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<&\[[ComponentId](../component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId"); [N](https://doc.rust-lang.org/nightly/std/primitive.array.html)\] as [DynamicComponentFetch](trait.DynamicComponentFetch.html "trait bevy::ecs::world::DynamicComponentFetch")\>::[Mut](trait.DynamicComponentFetch.html#associatedtype.Mut "type bevy::ecs::world::DynamicComponentFetch::Mut")<'\_>, [EntityComponentError](error/enum.EntityComponentError.html "enum bevy::ecs::world::error::EntityComponentError")\>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/component_fetch.rs.html#217-220)

#### unsafe fn [fetch\_mut\_assume\_mutable](#tymethod.fetch_mut_assume_mutable)( self, cell: [UnsafeEntityCell](unsafe_world_cell/struct.UnsafeEntityCell.html "struct bevy::ecs::world::unsafe_world_cell::UnsafeEntityCell")<'\_>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<&\[[ComponentId](../component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId"); [N](https://doc.rust-lang.org/nightly/std/primitive.array.html)\] as [DynamicComponentFetch](trait.DynamicComponentFetch.html "trait bevy::ecs::world::DynamicComponentFetch")\>::[Mut](trait.DynamicComponentFetch.html#associatedtype.Mut "type bevy::ecs::world::DynamicComponentFetch::Mut")<'\_>, [EntityComponentError](error/enum.EntityComponentError.html "enum bevy::ecs::world::error::EntityComponentError")\>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/component_fetch.rs.html#135)

### impl<const N: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\> [DynamicComponentFetch](trait.DynamicComponentFetch.html "trait bevy::ecs::world::DynamicComponentFetch") for \[[ComponentId](../component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId"); [N](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/component_fetch.rs.html#136)

#### type [Ref](#associatedtype.Ref)<'w> = \[[Ptr](../ptr/struct.Ptr.html "struct bevy::ecs::ptr::Ptr")<'w>; [N](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/component_fetch.rs.html#137)

#### type [Mut](#associatedtype.Mut)<'w> = \[[MutUntyped](../change_detection/struct.MutUntyped.html "struct bevy::ecs::change_detection::MutUntyped")<'w>; [N](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/component_fetch.rs.html#139-142)

#### unsafe fn [fetch\_ref](#tymethod.fetch_ref)( self, cell: [UnsafeEntityCell](unsafe_world_cell/struct.UnsafeEntityCell.html "struct bevy::ecs::world::unsafe_world_cell::UnsafeEntityCell")<'\_>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<\[[ComponentId](../component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId"); [N](https://doc.rust-lang.org/nightly/std/primitive.array.html)\] as [DynamicComponentFetch](trait.DynamicComponentFetch.html "trait bevy::ecs::world::DynamicComponentFetch")\>::[Ref](trait.DynamicComponentFetch.html#associatedtype.Ref "type bevy::ecs::world::DynamicComponentFetch::Ref")<'\_>, [EntityComponentError](error/enum.EntityComponentError.html "enum bevy::ecs::world::error::EntityComponentError")\>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/component_fetch.rs.html#147-150)

#### unsafe fn [fetch\_mut](#tymethod.fetch_mut)( self, cell: [UnsafeEntityCell](unsafe_world_cell/struct.UnsafeEntityCell.html "struct bevy::ecs::world::unsafe_world_cell::UnsafeEntityCell")<'\_>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<\[[ComponentId](../component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId"); [N](https://doc.rust-lang.org/nightly/std/primitive.array.html)\] as [DynamicComponentFetch](trait.DynamicComponentFetch.html "trait bevy::ecs::world::DynamicComponentFetch")\>::[Mut](trait.DynamicComponentFetch.html#associatedtype.Mut "type bevy::ecs::world::DynamicComponentFetch::Mut")<'\_>, [EntityComponentError](error/enum.EntityComponentError.html "enum bevy::ecs::world::error::EntityComponentError")\>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/component_fetch.rs.html#155-158)

#### unsafe fn [fetch\_mut\_assume\_mutable](#tymethod.fetch_mut_assume_mutable)( self, cell: [UnsafeEntityCell](unsafe_world_cell/struct.UnsafeEntityCell.html "struct bevy::ecs::world::unsafe_world_cell::UnsafeEntityCell")<'\_>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<\[[ComponentId](../component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId"); [N](https://doc.rust-lang.org/nightly/std/primitive.array.html)\] as [DynamicComponentFetch](trait.DynamicComponentFetch.html "trait bevy::ecs::world::DynamicComponentFetch")\>::[Mut](trait.DynamicComponentFetch.html#associatedtype.Mut "type bevy::ecs::world::DynamicComponentFetch::Mut")<'\_>, [EntityComponentError](error/enum.EntityComponentError.html "enum bevy::ecs::world::error::EntityComponentError")\>

## Implementors

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/component_fetch.rs.html#319)

### impl [DynamicComponentFetch](trait.DynamicComponentFetch.html "trait bevy::ecs::world::DynamicComponentFetch") for &[HashSet](../../platform/collections/struct.HashSet.html "struct bevy::platform::collections::HashSet")<[ComponentId](../component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId")\>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/component_fetch.rs.html#320)

#### type [Ref](#associatedtype.Ref)<'w> = [HashMap](../../platform/collections/struct.HashMap.html "struct bevy::platform::collections::HashMap")<[ComponentId](../component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId"), [Ptr](../ptr/struct.Ptr.html "struct bevy::ecs::ptr::Ptr")<'w>>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/component_fetch.rs.html#321)

#### type [Mut](#associatedtype.Mut)<'w> = [HashMap](../../platform/collections/struct.HashMap.html "struct bevy::platform::collections::HashMap")<[ComponentId](../component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId"), [MutUntyped](../change_detection/struct.MutUntyped.html "struct bevy::ecs::change_detection::MutUntyped")<'w>>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/component_fetch.rs.html#101)

### impl [DynamicComponentFetch](trait.DynamicComponentFetch.html "trait bevy::ecs::world::DynamicComponentFetch") for [ComponentId](../component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/component_fetch.rs.html#102)

#### type [Ref](#associatedtype.Ref)<'w> = [Ptr](../ptr/struct.Ptr.html "struct bevy::ecs::ptr::Ptr")<'w>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/entity_access/component_fetch.rs.html#103)

#### type [Mut](#associatedtype.Mut)<'w> = [MutUntyped](../change_detection/struct.MutUntyped.html "struct bevy::ecs::change_detection::MutUntyped")<'w>