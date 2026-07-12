[bevy](../../index.html)::[ecs](../index.html)::[query](index.html)

# Trait ArchetypeQueryData 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#497)

```rust
pub trait ArchetypeQueryData: QueryData { }
```

A marker trait to indicate that the query data filters at an archetype level.

This is needed to implement [`ExactSizeIterator`](https://doc.rust-lang.org/nightly/core/iter/traits/exact_size/trait.ExactSizeIterator.html "trait core::iter::traits::exact_size::ExactSizeIterator") for [`QueryIter`](struct.QueryIter.html "struct bevy::ecs::query::QueryIter") that contains archetype-level filters.

The trait must only be implemented for query data where its corresponding [`QueryData::IS_ARCHETYPAL`](trait.QueryData.html#associatedconstant.IS_ARCHETYPAL "associated constant bevy::ecs::query::QueryData::IS_ARCHETYPAL") is [`true`](https://doc.rust-lang.org/nightly/std/primitive.bool.html "primitive bool").

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementations on Foreign Types

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#3788-3796)

### impl [ArchetypeQueryData](trait.ArchetypeQueryData.html "trait bevy::ecs::query::ArchetypeQueryData") for [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#3788-3796)

### impl<F> [ArchetypeQueryData](trait.ArchetypeQueryData.html "trait bevy::ecs::query::ArchetypeQueryData") for [(F₁, F₂, …, Fₙ)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html#trait-implementations-1)

where F: [ArchetypeQueryData](trait.ArchetypeQueryData.html "trait bevy::ecs::query::ArchetypeQueryData"),

This trait is implemented for tuples up to 16 items long.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#1959)

### impl<T> [ArchetypeQueryData](trait.ArchetypeQueryData.html "trait bevy::ecs::query::ArchetypeQueryData") for [&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

where T: [Component](../../prelude/trait.Component.html "trait bevy::prelude::Component"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#2446)

### impl<T> [ArchetypeQueryData](trait.ArchetypeQueryData.html "trait bevy::ecs::query::ArchetypeQueryData") for [&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

where T: [Component](../../prelude/trait.Component.html "trait bevy::prelude::Component")<Mutability = [Mutable](../component/struct.Mutable.html "struct bevy::ecs::component::Mutable")\>,

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#3183)

### impl<T> [ArchetypeQueryData](trait.ArchetypeQueryData.html "trait bevy::ecs::query::ArchetypeQueryData") for [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<T>

where T: [QueryData](trait.QueryData.html "trait bevy::ecs::query::QueryData"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#4002)

### impl<T> [ArchetypeQueryData](trait.ArchetypeQueryData.html "trait bevy::ecs::query::ArchetypeQueryData") for [PhantomData](https://doc.rust-lang.org/nightly/core/marker/struct.PhantomData.html "struct core::marker::PhantomData")<T>

where T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

## Implementors

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#1752)

### impl [ArchetypeQueryData](trait.ArchetypeQueryData.html "trait bevy::ecs::query::ArchetypeQueryData") for &[Archetype](../archetype/struct.Archetype.html "struct bevy::ecs::archetype::Archetype")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#3797-3805)

### impl [ArchetypeQueryData](trait.ArchetypeQueryData.html "trait bevy::ecs::query::ArchetypeQueryData") for [AnyOf](../../prelude/struct.AnyOf.html "struct bevy::prelude::AnyOf")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)\>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#595)

### impl [ArchetypeQueryData](trait.ArchetypeQueryData.html "trait bevy::ecs::query::ArchetypeQueryData") for [Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#710)

### impl [ArchetypeQueryData](trait.ArchetypeQueryData.html "trait bevy::ecs::query::ArchetypeQueryData") for [EntityLocation](../entity/struct.EntityLocation.html "struct bevy::ecs::entity::EntityLocation")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#1145)

### impl [ArchetypeQueryData](trait.ArchetypeQueryData.html "trait bevy::ecs::query::ArchetypeQueryData") for [EntityMut](../../prelude/struct.EntityMut.html "struct bevy::prelude::EntityMut")<'\_>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#1032)

### impl [ArchetypeQueryData](trait.ArchetypeQueryData.html "trait bevy::ecs::query::ArchetypeQueryData") for [EntityRef](../../prelude/struct.EntityRef.html "struct bevy::prelude::EntityRef")<'\_>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#1402)

### impl [ArchetypeQueryData](trait.ArchetypeQueryData.html "trait bevy::ecs::query::ArchetypeQueryData") for [FilteredEntityMut](../world/struct.FilteredEntityMut.html "struct bevy::ecs::world::FilteredEntityMut")<'\_, '\_>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#1276)

### impl [ArchetypeQueryData](trait.ArchetypeQueryData.html "trait bevy::ecs::query::ArchetypeQueryData") for [FilteredEntityRef](../world/struct.FilteredEntityRef.html "struct bevy::ecs::world::FilteredEntityRef")<'\_, '\_>

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/sync_world.rs.html#521)

### impl [ArchetypeQueryData](trait.ArchetypeQueryData.html "trait bevy::ecs::query::ArchetypeQueryData") for [MainEntity](../../render/sync_world/struct.MainEntity.html "struct bevy::render::sync_world::MainEntity")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/name.rs.html#155)

### impl [ArchetypeQueryData](trait.ArchetypeQueryData.html "trait bevy::ecs::query::ArchetypeQueryData") for [NameOrEntity](../../prelude/struct.NameOrEntity.html "struct bevy::prelude::NameOrEntity")

where [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [Name](../../prelude/struct.Name.html "struct bevy::prelude::Name")\>: for<'\_\_a> [ArchetypeQueryData](trait.ArchetypeQueryData.html "trait bevy::ecs::query::ArchetypeQueryData"), [Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity"): for<'\_\_a> [ArchetypeQueryData](trait.ArchetypeQueryData.html "trait bevy::ecs::query::ArchetypeQueryData"),

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/picking_backend.rs.html#85)

### impl [ArchetypeQueryData](trait.ArchetypeQueryData.html "trait bevy::ecs::query::ArchetypeQueryData") for bevy::ui::picking\_backend::[NodeQuery](../../ui/picking_backend/struct.NodeQuery.html "struct bevy::ui::picking_backend::NodeQuery")

where [Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity"): for<'\_\_a> [ArchetypeQueryData](trait.ArchetypeQueryData.html "trait bevy::ecs::query::ArchetypeQueryData"), &'static [ComputedNode](../../prelude/struct.ComputedNode.html "struct bevy::prelude::ComputedNode"): for<'\_\_a> [ArchetypeQueryData](trait.ArchetypeQueryData.html "trait bevy::ecs::query::ArchetypeQueryData"), &'static [UiGlobalTransform](../../prelude/struct.UiGlobalTransform.html "struct bevy::prelude::UiGlobalTransform"): for<'\_\_a> [ArchetypeQueryData](trait.ArchetypeQueryData.html "trait bevy::ecs::query::ArchetypeQueryData"), [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [Pickable](../../prelude/struct.Pickable.html "struct bevy::prelude::Pickable")\>: for<'\_\_a> [ArchetypeQueryData](trait.ArchetypeQueryData.html "trait bevy::ecs::query::ArchetypeQueryData"), [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [InheritedVisibility](../../prelude/struct.InheritedVisibility.html "struct bevy::prelude::InheritedVisibility")\>: for<'\_\_a> [ArchetypeQueryData](trait.ArchetypeQueryData.html "trait bevy::ecs::query::ArchetypeQueryData"), &'static [ComputedUiTargetCamera](../../prelude/struct.ComputedUiTargetCamera.html "struct bevy::prelude::ComputedUiTargetCamera"): for<'\_\_a> [ArchetypeQueryData](trait.ArchetypeQueryData.html "trait bevy::ecs::query::ArchetypeQueryData"), [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<(&'static [TextLayoutInfo](../../text/struct.TextLayoutInfo.html "struct bevy::text::TextLayoutInfo"), &'static [ComputedTextBlock](../../text/struct.ComputedTextBlock.html "struct bevy::text::ComputedTextBlock"))>: for<'\_\_a> [ArchetypeQueryData](trait.ArchetypeQueryData.html "trait bevy::ecs::query::ArchetypeQueryData"),

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/focus.rs.html#132)

### impl [ArchetypeQueryData](trait.ArchetypeQueryData.html "trait bevy::ecs::query::ArchetypeQueryData") for bevy::ui::[NodeQuery](../../ui/struct.NodeQuery.html "struct bevy::ui::NodeQuery")

where [Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity"): for<'\_\_a> [ArchetypeQueryData](trait.ArchetypeQueryData.html "trait bevy::ecs::query::ArchetypeQueryData"), &'static [ComputedNode](../../prelude/struct.ComputedNode.html "struct bevy::prelude::ComputedNode"): for<'\_\_a> [ArchetypeQueryData](trait.ArchetypeQueryData.html "trait bevy::ecs::query::ArchetypeQueryData"), &'static [UiGlobalTransform](../../prelude/struct.UiGlobalTransform.html "struct bevy::prelude::UiGlobalTransform"): for<'\_\_a> [ArchetypeQueryData](trait.ArchetypeQueryData.html "trait bevy::ecs::query::ArchetypeQueryData"), [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static mut [Interaction](../../prelude/enum.Interaction.html "enum bevy::prelude::Interaction")\>: for<'\_\_a> [ArchetypeQueryData](trait.ArchetypeQueryData.html "trait bevy::ecs::query::ArchetypeQueryData"), [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static mut [RelativeCursorPosition](../../ui/struct.RelativeCursorPosition.html "struct bevy::ui::RelativeCursorPosition")\>: for<'\_\_a> [ArchetypeQueryData](trait.ArchetypeQueryData.html "trait bevy::ecs::query::ArchetypeQueryData"), [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [FocusPolicy](../../ui/enum.FocusPolicy.html "enum bevy::ui::FocusPolicy")\>: for<'\_\_a> [ArchetypeQueryData](trait.ArchetypeQueryData.html "trait bevy::ecs::query::ArchetypeQueryData"), [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [InheritedVisibility](../../prelude/struct.InheritedVisibility.html "struct bevy::prelude::InheritedVisibility")\>: for<'\_\_a> [ArchetypeQueryData](trait.ArchetypeQueryData.html "trait bevy::ecs::query::ArchetypeQueryData"), &'static [ComputedUiTargetCamera](../../prelude/struct.ComputedUiTargetCamera.html "struct bevy::prelude::ComputedUiTargetCamera"): for<'\_\_a> [ArchetypeQueryData](trait.ArchetypeQueryData.html "trait bevy::ecs::query::ArchetypeQueryData"),

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/picking_backend.rs.html#85)

### impl [ArchetypeQueryData](trait.ArchetypeQueryData.html "trait bevy::ecs::query::ArchetypeQueryData") for bevy::ui::picking\_backend::[NodeQueryReadOnly](../../ui/picking_backend/struct.NodeQueryReadOnly.html "struct bevy::ui::picking_backend::NodeQueryReadOnly")

where [Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity"): for<'\_\_a> [ArchetypeQueryData](trait.ArchetypeQueryData.html "trait bevy::ecs::query::ArchetypeQueryData"), &'static [ComputedNode](../../prelude/struct.ComputedNode.html "struct bevy::prelude::ComputedNode"): for<'\_\_a> [ArchetypeQueryData](trait.ArchetypeQueryData.html "trait bevy::ecs::query::ArchetypeQueryData"), &'static [UiGlobalTransform](../../prelude/struct.UiGlobalTransform.html "struct bevy::prelude::UiGlobalTransform"): for<'\_\_a> [ArchetypeQueryData](trait.ArchetypeQueryData.html "trait bevy::ecs::query::ArchetypeQueryData"), [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [Pickable](../../prelude/struct.Pickable.html "struct bevy::prelude::Pickable")\>: for<'\_\_a> [ArchetypeQueryData](trait.ArchetypeQueryData.html "trait bevy::ecs::query::ArchetypeQueryData"), [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [InheritedVisibility](../../prelude/struct.InheritedVisibility.html "struct bevy::prelude::InheritedVisibility")\>: for<'\_\_a> [ArchetypeQueryData](trait.ArchetypeQueryData.html "trait bevy::ecs::query::ArchetypeQueryData"), &'static [ComputedUiTargetCamera](../../prelude/struct.ComputedUiTargetCamera.html "struct bevy::prelude::ComputedUiTargetCamera"): for<'\_\_a> [ArchetypeQueryData](trait.ArchetypeQueryData.html "trait bevy::ecs::query::ArchetypeQueryData"), [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<(&'static [TextLayoutInfo](../../text/struct.TextLayoutInfo.html "struct bevy::text::TextLayoutInfo"), &'static [ComputedTextBlock](../../text/struct.ComputedTextBlock.html "struct bevy::text::ComputedTextBlock"))>: for<'\_\_a> [ArchetypeQueryData](trait.ArchetypeQueryData.html "trait bevy::ecs::query::ArchetypeQueryData"),

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/focus.rs.html#132)

### impl [ArchetypeQueryData](trait.ArchetypeQueryData.html "trait bevy::ecs::query::ArchetypeQueryData") for bevy::ui::[NodeQueryReadOnly](../../ui/struct.NodeQueryReadOnly.html "struct bevy::ui::NodeQueryReadOnly")

where [Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity"): for<'\_\_a> [ArchetypeQueryData](trait.ArchetypeQueryData.html "trait bevy::ecs::query::ArchetypeQueryData"), &'static [ComputedNode](../../prelude/struct.ComputedNode.html "struct bevy::prelude::ComputedNode"): for<'\_\_a> [ArchetypeQueryData](trait.ArchetypeQueryData.html "trait bevy::ecs::query::ArchetypeQueryData"), &'static [UiGlobalTransform](../../prelude/struct.UiGlobalTransform.html "struct bevy::prelude::UiGlobalTransform"): for<'\_\_a> [ArchetypeQueryData](trait.ArchetypeQueryData.html "trait bevy::ecs::query::ArchetypeQueryData"), [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static mut [Interaction](../../prelude/enum.Interaction.html "enum bevy::prelude::Interaction")\>: for<'\_\_a> [ArchetypeQueryData](trait.ArchetypeQueryData.html "trait bevy::ecs::query::ArchetypeQueryData"), [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static mut [RelativeCursorPosition](../../ui/struct.RelativeCursorPosition.html "struct bevy::ui::RelativeCursorPosition")\>: for<'\_\_a> [ArchetypeQueryData](trait.ArchetypeQueryData.html "trait bevy::ecs::query::ArchetypeQueryData"), [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [FocusPolicy](../../ui/enum.FocusPolicy.html "enum bevy::ui::FocusPolicy")\>: for<'\_\_a> [ArchetypeQueryData](trait.ArchetypeQueryData.html "trait bevy::ecs::query::ArchetypeQueryData"), [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [InheritedVisibility](../../prelude/struct.InheritedVisibility.html "struct bevy::prelude::InheritedVisibility")\>: for<'\_\_a> [ArchetypeQueryData](trait.ArchetypeQueryData.html "trait bevy::ecs::query::ArchetypeQueryData"), &'static [ComputedUiTargetCamera](../../prelude/struct.ComputedUiTargetCamera.html "struct bevy::prelude::ComputedUiTargetCamera"): for<'\_\_a> [ArchetypeQueryData](trait.ArchetypeQueryData.html "trait bevy::ecs::query::ArchetypeQueryData"),

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/events.rs.html#94)

### impl [ArchetypeQueryData](trait.ArchetypeQueryData.html "trait bevy::ecs::query::ArchetypeQueryData") for [PointerTraversal](../../prelude/struct.PointerTraversal.html "struct bevy::prelude::PointerTraversal")

where [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [ChildOf](../../prelude/struct.ChildOf.html "struct bevy::prelude::ChildOf")\>: for<'\_\_a> [ArchetypeQueryData](trait.ArchetypeQueryData.html "trait bevy::ecs::query::ArchetypeQueryData"), [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [Window](../../prelude/struct.Window.html "struct bevy::prelude::Window")\>: for<'\_\_a> [ArchetypeQueryData](trait.ArchetypeQueryData.html "trait bevy::ecs::query::ArchetypeQueryData"),

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/sync_world.rs.html#400)

### impl [ArchetypeQueryData](trait.ArchetypeQueryData.html "trait bevy::ecs::query::ArchetypeQueryData") for [RenderEntity](../../render/sync_world/struct.RenderEntity.html "struct bevy::render::sync_world::RenderEntity")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#903)

### impl [ArchetypeQueryData](trait.ArchetypeQueryData.html "trait bevy::ecs::query::ArchetypeQueryData") for [SpawnDetails](struct.SpawnDetails.html "struct bevy::ecs::query::SpawnDetails")

[Source](https://docs.rs/bevy_input_focus/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input_focus/lib.rs.html#219)

### impl [ArchetypeQueryData](trait.ArchetypeQueryData.html "trait bevy::ecs::query::ArchetypeQueryData") for [WindowTraversal](../../input_focus/struct.WindowTraversal.html "struct bevy::input_focus::WindowTraversal")

where [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [ChildOf](../../prelude/struct.ChildOf.html "struct bevy::prelude::ChildOf")\>: for<'\_\_a> [ArchetypeQueryData](trait.ArchetypeQueryData.html "trait bevy::ecs::query::ArchetypeQueryData"), [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [Window](../../prelude/struct.Window.html "struct bevy::prelude::Window")\>: for<'\_\_a> [ArchetypeQueryData](trait.ArchetypeQueryData.html "trait bevy::ecs::query::ArchetypeQueryData"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#1646)

### impl<B> [ArchetypeQueryData](trait.ArchetypeQueryData.html "trait bevy::ecs::query::ArchetypeQueryData") for [EntityMutExcept](../world/struct.EntityMutExcept.html "struct bevy::ecs::world::EntityMutExcept")<'\_, '\_, B>

where B: [Bundle](../../prelude/trait.Bundle.html "trait bevy::prelude::Bundle"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#1525)

### impl<B> [ArchetypeQueryData](trait.ArchetypeQueryData.html "trait bevy::ecs::query::ArchetypeQueryData") for [EntityRefExcept](../world/struct.EntityRefExcept.html "struct bevy::ecs::world::EntityRefExcept")<'\_, '\_, B>

where B: [Bundle](../../prelude/trait.Bundle.html "trait bevy::prelude::Bundle"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#3007)

### impl<D, F> [ArchetypeQueryData](trait.ArchetypeQueryData.html "trait bevy::ecs::query::ArchetypeQueryData") for [NestedQuery](struct.NestedQuery.html "struct bevy::ecs::query::NestedQuery")<D, F>

where D: [ReadOnlyQueryData](trait.ReadOnlyQueryData.html "trait bevy::ecs::query::ReadOnlyQueryData"), F: [QueryFilter](trait.QueryFilter.html "trait bevy::ecs::query::QueryFilter"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#3797-3805)

### impl<F> [ArchetypeQueryData](trait.ArchetypeQueryData.html "trait bevy::ecs::query::ArchetypeQueryData") for [AnyOf](../../prelude/struct.AnyOf.html "struct bevy::prelude::AnyOf")<[(F₁, F₂, …, Fₙ)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html#trait-implementations-1)\>

where F: [ArchetypeQueryData](trait.ArchetypeQueryData.html "trait bevy::ecs::query::ArchetypeQueryData"),

This trait is implemented for tuples up to 16 items long.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#3382)

### impl<T> [ArchetypeQueryData](trait.ArchetypeQueryData.html "trait bevy::ecs::query::ArchetypeQueryData") for [Has](../../prelude/struct.Has.html "struct bevy::prelude::Has")<T>

where T: [Component](../../prelude/trait.Component.html "trait bevy::prelude::Component"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#2615)

### impl<T> [ArchetypeQueryData](trait.ArchetypeQueryData.html "trait bevy::ecs::query::ArchetypeQueryData") for [Mut](../../prelude/struct.Mut.html "struct bevy::prelude::Mut")<'\_, T>

where T: [Component](../../prelude/trait.Component.html "trait bevy::prelude::Component")<Mutability = [Mutable](../component/struct.Mutable.html "struct bevy::ecs::component::Mutable")\>,

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#2182)

### impl<T> [ArchetypeQueryData](trait.ArchetypeQueryData.html "trait bevy::ecs::query::ArchetypeQueryData") for [Ref](../../prelude/struct.Ref.html "struct bevy::prelude::Ref")<'\_, T>

where T: [Component](../../prelude/trait.Component.html "trait bevy::prelude::Component"),