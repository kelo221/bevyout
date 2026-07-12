[bevy](../../index.html)::[ecs](../index.html)::[query](index.html)

# Trait ReadOnlyQueryData 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#463)

```rust
pub unsafe trait ReadOnlyQueryData: IterQueryData<ReadOnly = Self> { }
```

A [`QueryData`](trait.QueryData.html "trait bevy::ecs::query::QueryData") that is read only.

## Safety

This must only be implemented for read-only [`QueryData`](trait.QueryData.html "trait bevy::ecs::query::QueryData")’s.

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementations on Foreign Types

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#3788-3796)

### impl [ReadOnlyQueryData](trait.ReadOnlyQueryData.html "trait bevy::ecs::query::ReadOnlyQueryData") for [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#3788-3796)

### impl<F> [ReadOnlyQueryData](trait.ReadOnlyQueryData.html "trait bevy::ecs::query::ReadOnlyQueryData") for [(F₁, F₂, …, Fₙ)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html#trait-implementations-1)

where F: [ReadOnlyQueryData](trait.ReadOnlyQueryData.html "trait bevy::ecs::query::ReadOnlyQueryData"),

This trait is implemented for tuples up to 16 items long.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#1948)

### impl<T> [ReadOnlyQueryData](trait.ReadOnlyQueryData.html "trait bevy::ecs::query::ReadOnlyQueryData") for [&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

where T: [Component](../../prelude/trait.Component.html "trait bevy::prelude::Component"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#3170)

### impl<T> [ReadOnlyQueryData](trait.ReadOnlyQueryData.html "trait bevy::ecs::query::ReadOnlyQueryData") for [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<T>

where T: [ReadOnlyQueryData](trait.ReadOnlyQueryData.html "trait bevy::ecs::query::ReadOnlyQueryData"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#3993)

### impl<T> [ReadOnlyQueryData](trait.ReadOnlyQueryData.html "trait bevy::ecs::query::ReadOnlyQueryData") for [PhantomData](https://doc.rust-lang.org/nightly/core/marker/struct.PhantomData.html "struct core::marker::PhantomData")<T>

where T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

## Implementors

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#1741)

### impl [ReadOnlyQueryData](trait.ReadOnlyQueryData.html "trait bevy::ecs::query::ReadOnlyQueryData") for &[Archetype](../archetype/struct.Archetype.html "struct bevy::ecs::archetype::Archetype")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#3797-3805)

### impl [ReadOnlyQueryData](trait.ReadOnlyQueryData.html "trait bevy::ecs::query::ReadOnlyQueryData") for [AnyOf](../../prelude/struct.AnyOf.html "struct bevy::prelude::AnyOf")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)\>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#584)

### impl [ReadOnlyQueryData](trait.ReadOnlyQueryData.html "trait bevy::ecs::query::ReadOnlyQueryData") for [Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#699)

### impl [ReadOnlyQueryData](trait.ReadOnlyQueryData.html "trait bevy::ecs::query::ReadOnlyQueryData") for [EntityLocation](../entity/struct.EntityLocation.html "struct bevy::ecs::entity::EntityLocation")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#1021)

### impl [ReadOnlyQueryData](trait.ReadOnlyQueryData.html "trait bevy::ecs::query::ReadOnlyQueryData") for [EntityRef](../../prelude/struct.EntityRef.html "struct bevy::prelude::EntityRef")<'\_>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#1271)

### impl [ReadOnlyQueryData](trait.ReadOnlyQueryData.html "trait bevy::ecs::query::ReadOnlyQueryData") for [FilteredEntityRef](../world/struct.FilteredEntityRef.html "struct bevy::ecs::world::FilteredEntityRef")<'\_, '\_>

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/sync_world.rs.html#516)

### impl [ReadOnlyQueryData](trait.ReadOnlyQueryData.html "trait bevy::ecs::query::ReadOnlyQueryData") for [MainEntity](../../render/sync_world/struct.MainEntity.html "struct bevy::render::sync_world::MainEntity")

SAFETY: access is read only

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/name.rs.html#155)

### impl [ReadOnlyQueryData](trait.ReadOnlyQueryData.html "trait bevy::ecs::query::ReadOnlyQueryData") for [NameOrEntity](../../prelude/struct.NameOrEntity.html "struct bevy::prelude::NameOrEntity")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/picking_backend.rs.html#85)

### impl [ReadOnlyQueryData](trait.ReadOnlyQueryData.html "trait bevy::ecs::query::ReadOnlyQueryData") for bevy::ui::picking\_backend::[NodeQueryReadOnly](../../ui/picking_backend/struct.NodeQueryReadOnly.html "struct bevy::ui::picking_backend::NodeQueryReadOnly")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/focus.rs.html#132)

### impl [ReadOnlyQueryData](trait.ReadOnlyQueryData.html "trait bevy::ecs::query::ReadOnlyQueryData") for bevy::ui::[NodeQueryReadOnly](../../ui/struct.NodeQueryReadOnly.html "struct bevy::ui::NodeQueryReadOnly")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/events.rs.html#94)

### impl [ReadOnlyQueryData](trait.ReadOnlyQueryData.html "trait bevy::ecs::query::ReadOnlyQueryData") for [PointerTraversal](../../prelude/struct.PointerTraversal.html "struct bevy::prelude::PointerTraversal")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/sync_world.rs.html#395)

### impl [ReadOnlyQueryData](trait.ReadOnlyQueryData.html "trait bevy::ecs::query::ReadOnlyQueryData") for [RenderEntity](../../render/sync_world/struct.RenderEntity.html "struct bevy::render::sync_world::RenderEntity")

SAFETY: access is read only

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#892)

### impl [ReadOnlyQueryData](trait.ReadOnlyQueryData.html "trait bevy::ecs::query::ReadOnlyQueryData") for [SpawnDetails](struct.SpawnDetails.html "struct bevy::ecs::query::SpawnDetails")

[Source](https://docs.rs/bevy_input_focus/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input_focus/lib.rs.html#219)

### impl [ReadOnlyQueryData](trait.ReadOnlyQueryData.html "trait bevy::ecs::query::ReadOnlyQueryData") for [WindowTraversal](../../input_focus/struct.WindowTraversal.html "struct bevy::input_focus::WindowTraversal")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#2171)

### impl<'\_\_w, T> [ReadOnlyQueryData](trait.ReadOnlyQueryData.html "trait bevy::ecs::query::ReadOnlyQueryData") for [Ref](../../prelude/struct.Ref.html "struct bevy::prelude::Ref")<'\_\_w, T>

where T: [Component](../../prelude/trait.Component.html "trait bevy::prelude::Component"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#1520)

### impl<B> [ReadOnlyQueryData](trait.ReadOnlyQueryData.html "trait bevy::ecs::query::ReadOnlyQueryData") for [EntityRefExcept](../world/struct.EntityRefExcept.html "struct bevy::ecs::world::EntityRefExcept")<'\_, '\_, B>

where B: [Bundle](../../prelude/trait.Bundle.html "trait bevy::prelude::Bundle"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#2996)

### impl<D, F> [ReadOnlyQueryData](trait.ReadOnlyQueryData.html "trait bevy::ecs::query::ReadOnlyQueryData") for [NestedQuery](struct.NestedQuery.html "struct bevy::ecs::query::NestedQuery")<D, F>

where D: [ReadOnlyQueryData](trait.ReadOnlyQueryData.html "trait bevy::ecs::query::ReadOnlyQueryData"), F: [QueryFilter](trait.QueryFilter.html "trait bevy::ecs::query::QueryFilter"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#3797-3805)

### impl<F> [ReadOnlyQueryData](trait.ReadOnlyQueryData.html "trait bevy::ecs::query::ReadOnlyQueryData") for [AnyOf](../../prelude/struct.AnyOf.html "struct bevy::prelude::AnyOf")<[(F₁, F₂, …, Fₙ)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html#trait-implementations-1)\>

where F: [ReadOnlyQueryData](trait.ReadOnlyQueryData.html "trait bevy::ecs::query::ReadOnlyQueryData"),

This trait is implemented for tuples up to 16 items long.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#3371)

### impl<T> [ReadOnlyQueryData](trait.ReadOnlyQueryData.html "trait bevy::ecs::query::ReadOnlyQueryData") for [Has](../../prelude/struct.Has.html "struct bevy::prelude::Has")<T>

where T: [Component](../../prelude/trait.Component.html "trait bevy::prelude::Component"),