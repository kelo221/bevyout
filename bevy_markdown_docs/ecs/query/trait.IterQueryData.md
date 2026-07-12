[bevy](../../index.html)::[ecs](../index.html)::[query](index.html)

# Trait IterQueryData 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#456)

```rust
pub unsafe trait IterQueryData: QueryData { }
```

A [`QueryData`](trait.QueryData.html "trait bevy::ecs::query::QueryData") for which instances may be alive for different entities concurrently.

Rust [`Iterator`](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")s don’t connect the lifetime in [`Iterator::next`](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#tymethod.next "method core::iter::traits::iterator::Iterator::next") to anything in [`Iterator::Item`](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item "associated type core::iter::traits::iterator::Iterator::Item"), so later calls don’t invalidate earlier items. This is how methods like [`Iterator::collect`](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#method.collect "method core::iter::traits::iterator::Iterator::collect") work. It is therefore unsound to offer an [`Iterator`](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator") for a [`QueryData`](trait.QueryData.html "trait bevy::ecs::query::QueryData") for which only one instance may be alive concurrently.

To iterate over a [`QueryData`](trait.QueryData.html "trait bevy::ecs::query::QueryData") that does not implement [`IterQueryData`](trait.IterQueryData.html "trait bevy::ecs::query::IterQueryData"), use the [`QueryIter::fetch_next()`](struct.QueryIter.html#method.fetch_next "method bevy::ecs::query::QueryIter::fetch_next") method.

For `QueryData` that implement this trait, [`QueryData::fetch`](trait.QueryData.html#tymethod.fetch "associated function bevy::ecs::query::QueryData::fetch") may be called for one entity while an item is still alive for a different entity.

All [`SingleEntityQueryData`](trait.SingleEntityQueryData.html "trait bevy::ecs::query::SingleEntityQueryData") types are [`IterQueryData`](trait.IterQueryData.html "trait bevy::ecs::query::IterQueryData"). They only access data on the current entity, the one passed to [`QueryData::fetch`](trait.QueryData.html#tymethod.fetch "associated function bevy::ecs::query::QueryData::fetch"), so the access for different entities will always be disjoint.

All [`ReadOnlyQueryData`](trait.ReadOnlyQueryData.html "trait bevy::ecs::query::ReadOnlyQueryData") types are [`IterQueryData`](trait.IterQueryData.html "trait bevy::ecs::query::IterQueryData"). Even if they access data on entities other than the current one, that access is read-only and it’s sound for it to alias.

Queries with a nested query that performs mutable access should generally _not_ be [`IterQueryData`](trait.IterQueryData.html "trait bevy::ecs::query::IterQueryData"), although they can be if they have a way to prove that all accesses through the nested query are disjoint.

## Safety

This [`QueryData`](trait.QueryData.html "trait bevy::ecs::query::QueryData") must not perform conflicting access when fetched for different entities.

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementations on Foreign Types

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#3788-3796)

### impl [IterQueryData](trait.IterQueryData.html "trait bevy::ecs::query::IterQueryData") for [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#3788-3796)

### impl<F> [IterQueryData](trait.IterQueryData.html "trait bevy::ecs::query::IterQueryData") for [(F₁, F₂, …, Fₙ)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html#trait-implementations-1)

where F: [IterQueryData](trait.IterQueryData.html "trait bevy::ecs::query::IterQueryData"),

This trait is implemented for tuples up to 16 items long.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#1945)

### impl<T> [IterQueryData](trait.IterQueryData.html "trait bevy::ecs::query::IterQueryData") for [&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

where T: [Component](../../prelude/trait.Component.html "trait bevy::prelude::Component"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#2435)

### impl<T> [IterQueryData](trait.IterQueryData.html "trait bevy::ecs::query::IterQueryData") for [&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

where T: [Component](../../prelude/trait.Component.html "trait bevy::prelude::Component")<Mutability = [Mutable](../component/struct.Mutable.html "struct bevy::ecs::component::Mutable")\>,

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#3167)

### impl<T> [IterQueryData](trait.IterQueryData.html "trait bevy::ecs::query::IterQueryData") for [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<T>

where T: [IterQueryData](trait.IterQueryData.html "trait bevy::ecs::query::IterQueryData"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#3990)

### impl<T> [IterQueryData](trait.IterQueryData.html "trait bevy::ecs::query::IterQueryData") for [PhantomData](https://doc.rust-lang.org/nightly/core/marker/struct.PhantomData.html "struct core::marker::PhantomData")<T>

where T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

## Implementors

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#1738)

### impl [IterQueryData](trait.IterQueryData.html "trait bevy::ecs::query::IterQueryData") for &[Archetype](../archetype/struct.Archetype.html "struct bevy::ecs::archetype::Archetype")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#3797-3805)

### impl [IterQueryData](trait.IterQueryData.html "trait bevy::ecs::query::IterQueryData") for [AnyOf](../../prelude/struct.AnyOf.html "struct bevy::prelude::AnyOf")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)\>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#581)

### impl [IterQueryData](trait.IterQueryData.html "trait bevy::ecs::query::IterQueryData") for [Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#696)

### impl [IterQueryData](trait.IterQueryData.html "trait bevy::ecs::query::IterQueryData") for [EntityLocation](../entity/struct.EntityLocation.html "struct bevy::ecs::entity::EntityLocation")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#1134)

### impl [IterQueryData](trait.IterQueryData.html "trait bevy::ecs::query::IterQueryData") for [EntityMut](../../prelude/struct.EntityMut.html "struct bevy::prelude::EntityMut")<'\_>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#1018)

### impl [IterQueryData](trait.IterQueryData.html "trait bevy::ecs::query::IterQueryData") for [EntityRef](../../prelude/struct.EntityRef.html "struct bevy::prelude::EntityRef")<'\_>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#1397)

### impl [IterQueryData](trait.IterQueryData.html "trait bevy::ecs::query::IterQueryData") for [FilteredEntityMut](../world/struct.FilteredEntityMut.html "struct bevy::ecs::world::FilteredEntityMut")<'\_, '\_>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#1268)

### impl [IterQueryData](trait.IterQueryData.html "trait bevy::ecs::query::IterQueryData") for [FilteredEntityRef](../world/struct.FilteredEntityRef.html "struct bevy::ecs::world::FilteredEntityRef")<'\_, '\_>

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/sync_world.rs.html#513)

### impl [IterQueryData](trait.IterQueryData.html "trait bevy::ecs::query::IterQueryData") for [MainEntity](../../render/sync_world/struct.MainEntity.html "struct bevy::render::sync_world::MainEntity")

SAFETY: access is read only and only on the current entity

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/name.rs.html#155)

### impl [IterQueryData](trait.IterQueryData.html "trait bevy::ecs::query::IterQueryData") for [NameOrEntity](../../prelude/struct.NameOrEntity.html "struct bevy::prelude::NameOrEntity")

where [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [Name](../../prelude/struct.Name.html "struct bevy::prelude::Name")\>: for<'\_\_a> [IterQueryData](trait.IterQueryData.html "trait bevy::ecs::query::IterQueryData"), [Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity"): for<'\_\_a> [IterQueryData](trait.IterQueryData.html "trait bevy::ecs::query::IterQueryData"),

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/picking_backend.rs.html#85)

### impl [IterQueryData](trait.IterQueryData.html "trait bevy::ecs::query::IterQueryData") for bevy::ui::picking\_backend::[NodeQuery](../../ui/picking_backend/struct.NodeQuery.html "struct bevy::ui::picking_backend::NodeQuery")

where [Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity"): for<'\_\_a> [IterQueryData](trait.IterQueryData.html "trait bevy::ecs::query::IterQueryData"), &'static [ComputedNode](../../prelude/struct.ComputedNode.html "struct bevy::prelude::ComputedNode"): for<'\_\_a> [IterQueryData](trait.IterQueryData.html "trait bevy::ecs::query::IterQueryData"), &'static [UiGlobalTransform](../../prelude/struct.UiGlobalTransform.html "struct bevy::prelude::UiGlobalTransform"): for<'\_\_a> [IterQueryData](trait.IterQueryData.html "trait bevy::ecs::query::IterQueryData"), [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [Pickable](../../prelude/struct.Pickable.html "struct bevy::prelude::Pickable")\>: for<'\_\_a> [IterQueryData](trait.IterQueryData.html "trait bevy::ecs::query::IterQueryData"), [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [InheritedVisibility](../../prelude/struct.InheritedVisibility.html "struct bevy::prelude::InheritedVisibility")\>: for<'\_\_a> [IterQueryData](trait.IterQueryData.html "trait bevy::ecs::query::IterQueryData"), &'static [ComputedUiTargetCamera](../../prelude/struct.ComputedUiTargetCamera.html "struct bevy::prelude::ComputedUiTargetCamera"): for<'\_\_a> [IterQueryData](trait.IterQueryData.html "trait bevy::ecs::query::IterQueryData"), [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<(&'static [TextLayoutInfo](../../text/struct.TextLayoutInfo.html "struct bevy::text::TextLayoutInfo"), &'static [ComputedTextBlock](../../text/struct.ComputedTextBlock.html "struct bevy::text::ComputedTextBlock"))>: for<'\_\_a> [IterQueryData](trait.IterQueryData.html "trait bevy::ecs::query::IterQueryData"),

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/focus.rs.html#132)

### impl [IterQueryData](trait.IterQueryData.html "trait bevy::ecs::query::IterQueryData") for bevy::ui::[NodeQuery](../../ui/struct.NodeQuery.html "struct bevy::ui::NodeQuery")

where [Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity"): for<'\_\_a> [IterQueryData](trait.IterQueryData.html "trait bevy::ecs::query::IterQueryData"), &'static [ComputedNode](../../prelude/struct.ComputedNode.html "struct bevy::prelude::ComputedNode"): for<'\_\_a> [IterQueryData](trait.IterQueryData.html "trait bevy::ecs::query::IterQueryData"), &'static [UiGlobalTransform](../../prelude/struct.UiGlobalTransform.html "struct bevy::prelude::UiGlobalTransform"): for<'\_\_a> [IterQueryData](trait.IterQueryData.html "trait bevy::ecs::query::IterQueryData"), [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static mut [Interaction](../../prelude/enum.Interaction.html "enum bevy::prelude::Interaction")\>: for<'\_\_a> [IterQueryData](trait.IterQueryData.html "trait bevy::ecs::query::IterQueryData"), [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static mut [RelativeCursorPosition](../../ui/struct.RelativeCursorPosition.html "struct bevy::ui::RelativeCursorPosition")\>: for<'\_\_a> [IterQueryData](trait.IterQueryData.html "trait bevy::ecs::query::IterQueryData"), [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [FocusPolicy](../../ui/enum.FocusPolicy.html "enum bevy::ui::FocusPolicy")\>: for<'\_\_a> [IterQueryData](trait.IterQueryData.html "trait bevy::ecs::query::IterQueryData"), [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [InheritedVisibility](../../prelude/struct.InheritedVisibility.html "struct bevy::prelude::InheritedVisibility")\>: for<'\_\_a> [IterQueryData](trait.IterQueryData.html "trait bevy::ecs::query::IterQueryData"), &'static [ComputedUiTargetCamera](../../prelude/struct.ComputedUiTargetCamera.html "struct bevy::prelude::ComputedUiTargetCamera"): for<'\_\_a> [IterQueryData](trait.IterQueryData.html "trait bevy::ecs::query::IterQueryData"),

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/picking_backend.rs.html#85)

### impl [IterQueryData](trait.IterQueryData.html "trait bevy::ecs::query::IterQueryData") for bevy::ui::picking\_backend::[NodeQueryReadOnly](../../ui/picking_backend/struct.NodeQueryReadOnly.html "struct bevy::ui::picking_backend::NodeQueryReadOnly")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/focus.rs.html#132)

### impl [IterQueryData](trait.IterQueryData.html "trait bevy::ecs::query::IterQueryData") for bevy::ui::[NodeQueryReadOnly](../../ui/struct.NodeQueryReadOnly.html "struct bevy::ui::NodeQueryReadOnly")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/events.rs.html#94)

### impl [IterQueryData](trait.IterQueryData.html "trait bevy::ecs::query::IterQueryData") for [PointerTraversal](../../prelude/struct.PointerTraversal.html "struct bevy::prelude::PointerTraversal")

where [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [ChildOf](../../prelude/struct.ChildOf.html "struct bevy::prelude::ChildOf")\>: for<'\_\_a> [IterQueryData](trait.IterQueryData.html "trait bevy::ecs::query::IterQueryData"), [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [Window](../../prelude/struct.Window.html "struct bevy::prelude::Window")\>: for<'\_\_a> [IterQueryData](trait.IterQueryData.html "trait bevy::ecs::query::IterQueryData"),

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/sync_world.rs.html#392)

### impl [IterQueryData](trait.IterQueryData.html "trait bevy::ecs::query::IterQueryData") for [RenderEntity](../../render/sync_world/struct.RenderEntity.html "struct bevy::render::sync_world::RenderEntity")

SAFETY: access is read only and only on the current entity

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#889)

### impl [IterQueryData](trait.IterQueryData.html "trait bevy::ecs::query::IterQueryData") for [SpawnDetails](struct.SpawnDetails.html "struct bevy::ecs::query::SpawnDetails")

[Source](https://docs.rs/bevy_input_focus/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input_focus/lib.rs.html#219)

### impl [IterQueryData](trait.IterQueryData.html "trait bevy::ecs::query::IterQueryData") for [WindowTraversal](../../input_focus/struct.WindowTraversal.html "struct bevy::input_focus::WindowTraversal")

where [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [ChildOf](../../prelude/struct.ChildOf.html "struct bevy::prelude::ChildOf")\>: for<'\_\_a> [IterQueryData](trait.IterQueryData.html "trait bevy::ecs::query::IterQueryData"), [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [Window](../../prelude/struct.Window.html "struct bevy::prelude::Window")\>: for<'\_\_a> [IterQueryData](trait.IterQueryData.html "trait bevy::ecs::query::IterQueryData"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#2168)

### impl<'\_\_w, T> [IterQueryData](trait.IterQueryData.html "trait bevy::ecs::query::IterQueryData") for [Ref](../../prelude/struct.Ref.html "struct bevy::prelude::Ref")<'\_\_w, T>

where T: [Component](../../prelude/trait.Component.html "trait bevy::prelude::Component"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#1641)

### impl<B> [IterQueryData](trait.IterQueryData.html "trait bevy::ecs::query::IterQueryData") for [EntityMutExcept](../world/struct.EntityMutExcept.html "struct bevy::ecs::world::EntityMutExcept")<'\_, '\_, B>

where B: [Bundle](../../prelude/trait.Bundle.html "trait bevy::prelude::Bundle"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#1517)

### impl<B> [IterQueryData](trait.IterQueryData.html "trait bevy::ecs::query::IterQueryData") for [EntityRefExcept](../world/struct.EntityRefExcept.html "struct bevy::ecs::world::EntityRefExcept")<'\_, '\_, B>

where B: [Bundle](../../prelude/trait.Bundle.html "trait bevy::prelude::Bundle"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#3001)

### impl<D, F> [IterQueryData](trait.IterQueryData.html "trait bevy::ecs::query::IterQueryData") for [NestedQuery](struct.NestedQuery.html "struct bevy::ecs::query::NestedQuery")<D, F>

where D: [ReadOnlyQueryData](trait.ReadOnlyQueryData.html "trait bevy::ecs::query::ReadOnlyQueryData"), F: [QueryFilter](trait.QueryFilter.html "trait bevy::ecs::query::QueryFilter"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#3797-3805)

### impl<F> [IterQueryData](trait.IterQueryData.html "trait bevy::ecs::query::IterQueryData") for [AnyOf](../../prelude/struct.AnyOf.html "struct bevy::prelude::AnyOf")<[(F₁, F₂, …, Fₙ)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html#trait-implementations-1)\>

where F: [IterQueryData](trait.IterQueryData.html "trait bevy::ecs::query::IterQueryData"),

This trait is implemented for tuples up to 16 items long.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#3368)

### impl<T> [IterQueryData](trait.IterQueryData.html "trait bevy::ecs::query::IterQueryData") for [Has](../../prelude/struct.Has.html "struct bevy::prelude::Has")<T>

where T: [Component](../../prelude/trait.Component.html "trait bevy::prelude::Component"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#2604)

### impl<T> [IterQueryData](trait.IterQueryData.html "trait bevy::ecs::query::IterQueryData") for [Mut](../../prelude/struct.Mut.html "struct bevy::prelude::Mut")<'\_, T>

where T: [Component](../../prelude/trait.Component.html "trait bevy::prelude::Component")<Mutability = [Mutable](../component/struct.Mutable.html "struct bevy::ecs::component::Mutable")\>,