[bevy](../../index.html)::[ecs](../index.html)::[query](index.html)

# Trait SingleEntityQueryData 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#473)

```rust
pub unsafe trait SingleEntityQueryData: IterQueryData { }
```

A [`QueryData`](trait.QueryData.html "trait bevy::ecs::query::QueryData") that only accesses data from the current entity, the one passed to [`QueryData::fetch`](trait.QueryData.html#tymethod.fetch "associated function bevy::ecs::query::QueryData::fetch").

This is used as a bound in [`EntityRef::get_components`](../../prelude/struct.EntityRef.html#method.get_components "method bevy::prelude::EntityRef::get_components") and related APIs, since they only have access to a single entity.

## Safety

This [`QueryData`](trait.QueryData.html "trait bevy::ecs::query::QueryData") must only access data from the current entity, and not any other entities.

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementations on Foreign Types

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#3788-3796)

### impl [SingleEntityQueryData](trait.SingleEntityQueryData.html "trait bevy::ecs::query::SingleEntityQueryData") for [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#3788-3796)

### impl<F> [SingleEntityQueryData](trait.SingleEntityQueryData.html "trait bevy::ecs::query::SingleEntityQueryData") for [(F₁, F₂, …, Fₙ)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html#trait-implementations-1)

where F: [SingleEntityQueryData](trait.SingleEntityQueryData.html "trait bevy::ecs::query::SingleEntityQueryData"),

This trait is implemented for tuples up to 16 items long.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#1951)

### impl<T> [SingleEntityQueryData](trait.SingleEntityQueryData.html "trait bevy::ecs::query::SingleEntityQueryData") for [&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

where T: [Component](../../prelude/trait.Component.html "trait bevy::prelude::Component"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#2438)

### impl<T> [SingleEntityQueryData](trait.SingleEntityQueryData.html "trait bevy::ecs::query::SingleEntityQueryData") for [&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

where T: [Component](../../prelude/trait.Component.html "trait bevy::prelude::Component")<Mutability = [Mutable](../component/struct.Mutable.html "struct bevy::ecs::component::Mutable")\>,

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#3173)

### impl<T> [SingleEntityQueryData](trait.SingleEntityQueryData.html "trait bevy::ecs::query::SingleEntityQueryData") for [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<T>

where T: [SingleEntityQueryData](trait.SingleEntityQueryData.html "trait bevy::ecs::query::SingleEntityQueryData"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#3996)

### impl<T> [SingleEntityQueryData](trait.SingleEntityQueryData.html "trait bevy::ecs::query::SingleEntityQueryData") for [PhantomData](https://doc.rust-lang.org/nightly/core/marker/struct.PhantomData.html "struct core::marker::PhantomData")<T>

where T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

## Implementors

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#1744)

### impl [SingleEntityQueryData](trait.SingleEntityQueryData.html "trait bevy::ecs::query::SingleEntityQueryData") for &[Archetype](../archetype/struct.Archetype.html "struct bevy::ecs::archetype::Archetype")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#3797-3805)

### impl [SingleEntityQueryData](trait.SingleEntityQueryData.html "trait bevy::ecs::query::SingleEntityQueryData") for [AnyOf](../../prelude/struct.AnyOf.html "struct bevy::prelude::AnyOf")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)\>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#587)

### impl [SingleEntityQueryData](trait.SingleEntityQueryData.html "trait bevy::ecs::query::SingleEntityQueryData") for [Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#702)

### impl [SingleEntityQueryData](trait.SingleEntityQueryData.html "trait bevy::ecs::query::SingleEntityQueryData") for [EntityLocation](../entity/struct.EntityLocation.html "struct bevy::ecs::entity::EntityLocation")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#1137)

### impl [SingleEntityQueryData](trait.SingleEntityQueryData.html "trait bevy::ecs::query::SingleEntityQueryData") for [EntityMut](../../prelude/struct.EntityMut.html "struct bevy::prelude::EntityMut")<'\_>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#1024)

### impl [SingleEntityQueryData](trait.SingleEntityQueryData.html "trait bevy::ecs::query::SingleEntityQueryData") for [EntityRef](../../prelude/struct.EntityRef.html "struct bevy::prelude::EntityRef")<'\_>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#1400)

### impl [SingleEntityQueryData](trait.SingleEntityQueryData.html "trait bevy::ecs::query::SingleEntityQueryData") for [FilteredEntityMut](../world/struct.FilteredEntityMut.html "struct bevy::ecs::world::FilteredEntityMut")<'\_, '\_>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#1274)

### impl [SingleEntityQueryData](trait.SingleEntityQueryData.html "trait bevy::ecs::query::SingleEntityQueryData") for [FilteredEntityRef](../world/struct.FilteredEntityRef.html "struct bevy::ecs::world::FilteredEntityRef")<'\_, '\_>

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/sync_world.rs.html#519)

### impl [SingleEntityQueryData](trait.SingleEntityQueryData.html "trait bevy::ecs::query::SingleEntityQueryData") for [MainEntity](../../render/sync_world/struct.MainEntity.html "struct bevy::render::sync_world::MainEntity")

SAFETY: access is only on the current entity

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/name.rs.html#155)

### impl [SingleEntityQueryData](trait.SingleEntityQueryData.html "trait bevy::ecs::query::SingleEntityQueryData") for [NameOrEntity](../../prelude/struct.NameOrEntity.html "struct bevy::prelude::NameOrEntity")

where [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [Name](../../prelude/struct.Name.html "struct bevy::prelude::Name")\>: for<'\_\_a> [SingleEntityQueryData](trait.SingleEntityQueryData.html "trait bevy::ecs::query::SingleEntityQueryData"), [Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity"): for<'\_\_a> [SingleEntityQueryData](trait.SingleEntityQueryData.html "trait bevy::ecs::query::SingleEntityQueryData"),

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/picking_backend.rs.html#85)

### impl [SingleEntityQueryData](trait.SingleEntityQueryData.html "trait bevy::ecs::query::SingleEntityQueryData") for bevy::ui::picking\_backend::[NodeQuery](../../ui/picking_backend/struct.NodeQuery.html "struct bevy::ui::picking_backend::NodeQuery")

where [Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity"): for<'\_\_a> [SingleEntityQueryData](trait.SingleEntityQueryData.html "trait bevy::ecs::query::SingleEntityQueryData"), &'static [ComputedNode](../../prelude/struct.ComputedNode.html "struct bevy::prelude::ComputedNode"): for<'\_\_a> [SingleEntityQueryData](trait.SingleEntityQueryData.html "trait bevy::ecs::query::SingleEntityQueryData"), &'static [UiGlobalTransform](../../prelude/struct.UiGlobalTransform.html "struct bevy::prelude::UiGlobalTransform"): for<'\_\_a> [SingleEntityQueryData](trait.SingleEntityQueryData.html "trait bevy::ecs::query::SingleEntityQueryData"), [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [Pickable](../../prelude/struct.Pickable.html "struct bevy::prelude::Pickable")\>: for<'\_\_a> [SingleEntityQueryData](trait.SingleEntityQueryData.html "trait bevy::ecs::query::SingleEntityQueryData"), [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [InheritedVisibility](../../prelude/struct.InheritedVisibility.html "struct bevy::prelude::InheritedVisibility")\>: for<'\_\_a> [SingleEntityQueryData](trait.SingleEntityQueryData.html "trait bevy::ecs::query::SingleEntityQueryData"), &'static [ComputedUiTargetCamera](../../prelude/struct.ComputedUiTargetCamera.html "struct bevy::prelude::ComputedUiTargetCamera"): for<'\_\_a> [SingleEntityQueryData](trait.SingleEntityQueryData.html "trait bevy::ecs::query::SingleEntityQueryData"), [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<(&'static [TextLayoutInfo](../../text/struct.TextLayoutInfo.html "struct bevy::text::TextLayoutInfo"), &'static [ComputedTextBlock](../../text/struct.ComputedTextBlock.html "struct bevy::text::ComputedTextBlock"))>: for<'\_\_a> [SingleEntityQueryData](trait.SingleEntityQueryData.html "trait bevy::ecs::query::SingleEntityQueryData"),

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/focus.rs.html#132)

### impl [SingleEntityQueryData](trait.SingleEntityQueryData.html "trait bevy::ecs::query::SingleEntityQueryData") for bevy::ui::[NodeQuery](../../ui/struct.NodeQuery.html "struct bevy::ui::NodeQuery")

where [Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity"): for<'\_\_a> [SingleEntityQueryData](trait.SingleEntityQueryData.html "trait bevy::ecs::query::SingleEntityQueryData"), &'static [ComputedNode](../../prelude/struct.ComputedNode.html "struct bevy::prelude::ComputedNode"): for<'\_\_a> [SingleEntityQueryData](trait.SingleEntityQueryData.html "trait bevy::ecs::query::SingleEntityQueryData"), &'static [UiGlobalTransform](../../prelude/struct.UiGlobalTransform.html "struct bevy::prelude::UiGlobalTransform"): for<'\_\_a> [SingleEntityQueryData](trait.SingleEntityQueryData.html "trait bevy::ecs::query::SingleEntityQueryData"), [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static mut [Interaction](../../prelude/enum.Interaction.html "enum bevy::prelude::Interaction")\>: for<'\_\_a> [SingleEntityQueryData](trait.SingleEntityQueryData.html "trait bevy::ecs::query::SingleEntityQueryData"), [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static mut [RelativeCursorPosition](../../ui/struct.RelativeCursorPosition.html "struct bevy::ui::RelativeCursorPosition")\>: for<'\_\_a> [SingleEntityQueryData](trait.SingleEntityQueryData.html "trait bevy::ecs::query::SingleEntityQueryData"), [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [FocusPolicy](../../ui/enum.FocusPolicy.html "enum bevy::ui::FocusPolicy")\>: for<'\_\_a> [SingleEntityQueryData](trait.SingleEntityQueryData.html "trait bevy::ecs::query::SingleEntityQueryData"), [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [InheritedVisibility](../../prelude/struct.InheritedVisibility.html "struct bevy::prelude::InheritedVisibility")\>: for<'\_\_a> [SingleEntityQueryData](trait.SingleEntityQueryData.html "trait bevy::ecs::query::SingleEntityQueryData"), &'static [ComputedUiTargetCamera](../../prelude/struct.ComputedUiTargetCamera.html "struct bevy::prelude::ComputedUiTargetCamera"): for<'\_\_a> [SingleEntityQueryData](trait.SingleEntityQueryData.html "trait bevy::ecs::query::SingleEntityQueryData"),

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/picking_backend.rs.html#85)

### impl [SingleEntityQueryData](trait.SingleEntityQueryData.html "trait bevy::ecs::query::SingleEntityQueryData") for bevy::ui::picking\_backend::[NodeQueryReadOnly](../../ui/picking_backend/struct.NodeQueryReadOnly.html "struct bevy::ui::picking_backend::NodeQueryReadOnly")

where [Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity"): for<'\_\_a> [QueryData](trait.QueryData.html "trait bevy::ecs::query::QueryData"), <[Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity") as [QueryData](trait.QueryData.html "trait bevy::ecs::query::QueryData")\>::[ReadOnly](trait.QueryData.html#associatedtype.ReadOnly "type bevy::ecs::query::QueryData::ReadOnly"): for<'\_\_a> [SingleEntityQueryData](trait.SingleEntityQueryData.html "trait bevy::ecs::query::SingleEntityQueryData"), &'static [ComputedNode](../../prelude/struct.ComputedNode.html "struct bevy::prelude::ComputedNode"): for<'\_\_a> [QueryData](trait.QueryData.html "trait bevy::ecs::query::QueryData"), <&'static [ComputedNode](../../prelude/struct.ComputedNode.html "struct bevy::prelude::ComputedNode") as [QueryData](trait.QueryData.html "trait bevy::ecs::query::QueryData")\>::[ReadOnly](trait.QueryData.html#associatedtype.ReadOnly "type bevy::ecs::query::QueryData::ReadOnly"): for<'\_\_a> [SingleEntityQueryData](trait.SingleEntityQueryData.html "trait bevy::ecs::query::SingleEntityQueryData"), &'static [UiGlobalTransform](../../prelude/struct.UiGlobalTransform.html "struct bevy::prelude::UiGlobalTransform"): for<'\_\_a> [QueryData](trait.QueryData.html "trait bevy::ecs::query::QueryData"), <&'static [UiGlobalTransform](../../prelude/struct.UiGlobalTransform.html "struct bevy::prelude::UiGlobalTransform") as [QueryData](trait.QueryData.html "trait bevy::ecs::query::QueryData")\>::[ReadOnly](trait.QueryData.html#associatedtype.ReadOnly "type bevy::ecs::query::QueryData::ReadOnly"): for<'\_\_a> [SingleEntityQueryData](trait.SingleEntityQueryData.html "trait bevy::ecs::query::SingleEntityQueryData"), [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [Pickable](../../prelude/struct.Pickable.html "struct bevy::prelude::Pickable")\>: for<'\_\_a> [QueryData](trait.QueryData.html "trait bevy::ecs::query::QueryData"), <[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [Pickable](../../prelude/struct.Pickable.html "struct bevy::prelude::Pickable")\> as [QueryData](trait.QueryData.html "trait bevy::ecs::query::QueryData")\>::[ReadOnly](trait.QueryData.html#associatedtype.ReadOnly "type bevy::ecs::query::QueryData::ReadOnly"): for<'\_\_a> [SingleEntityQueryData](trait.SingleEntityQueryData.html "trait bevy::ecs::query::SingleEntityQueryData"), [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [InheritedVisibility](../../prelude/struct.InheritedVisibility.html "struct bevy::prelude::InheritedVisibility")\>: for<'\_\_a> [QueryData](trait.QueryData.html "trait bevy::ecs::query::QueryData"), <[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [InheritedVisibility](../../prelude/struct.InheritedVisibility.html "struct bevy::prelude::InheritedVisibility")\> as [QueryData](trait.QueryData.html "trait bevy::ecs::query::QueryData")\>::[ReadOnly](trait.QueryData.html#associatedtype.ReadOnly "type bevy::ecs::query::QueryData::ReadOnly"): for<'\_\_a> [SingleEntityQueryData](trait.SingleEntityQueryData.html "trait bevy::ecs::query::SingleEntityQueryData"), &'static [ComputedUiTargetCamera](../../prelude/struct.ComputedUiTargetCamera.html "struct bevy::prelude::ComputedUiTargetCamera"): for<'\_\_a> [QueryData](trait.QueryData.html "trait bevy::ecs::query::QueryData"), <&'static [ComputedUiTargetCamera](../../prelude/struct.ComputedUiTargetCamera.html "struct bevy::prelude::ComputedUiTargetCamera") as [QueryData](trait.QueryData.html "trait bevy::ecs::query::QueryData")\>::[ReadOnly](trait.QueryData.html#associatedtype.ReadOnly "type bevy::ecs::query::QueryData::ReadOnly"): for<'\_\_a> [SingleEntityQueryData](trait.SingleEntityQueryData.html "trait bevy::ecs::query::SingleEntityQueryData"), [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<(&'static [TextLayoutInfo](../../text/struct.TextLayoutInfo.html "struct bevy::text::TextLayoutInfo"), &'static [ComputedTextBlock](../../text/struct.ComputedTextBlock.html "struct bevy::text::ComputedTextBlock"))>: for<'\_\_a> [QueryData](trait.QueryData.html "trait bevy::ecs::query::QueryData"), <[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<(&'static [TextLayoutInfo](../../text/struct.TextLayoutInfo.html "struct bevy::text::TextLayoutInfo"), &'static [ComputedTextBlock](../../text/struct.ComputedTextBlock.html "struct bevy::text::ComputedTextBlock"))> as [QueryData](trait.QueryData.html "trait bevy::ecs::query::QueryData")\>::[ReadOnly](trait.QueryData.html#associatedtype.ReadOnly "type bevy::ecs::query::QueryData::ReadOnly"): for<'\_\_a> [SingleEntityQueryData](trait.SingleEntityQueryData.html "trait bevy::ecs::query::SingleEntityQueryData"),

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/focus.rs.html#132)

### impl [SingleEntityQueryData](trait.SingleEntityQueryData.html "trait bevy::ecs::query::SingleEntityQueryData") for bevy::ui::[NodeQueryReadOnly](../../ui/struct.NodeQueryReadOnly.html "struct bevy::ui::NodeQueryReadOnly")

where [Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity"): for<'\_\_a> [QueryData](trait.QueryData.html "trait bevy::ecs::query::QueryData"), <[Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity") as [QueryData](trait.QueryData.html "trait bevy::ecs::query::QueryData")\>::[ReadOnly](trait.QueryData.html#associatedtype.ReadOnly "type bevy::ecs::query::QueryData::ReadOnly"): for<'\_\_a> [SingleEntityQueryData](trait.SingleEntityQueryData.html "trait bevy::ecs::query::SingleEntityQueryData"), &'static [ComputedNode](../../prelude/struct.ComputedNode.html "struct bevy::prelude::ComputedNode"): for<'\_\_a> [QueryData](trait.QueryData.html "trait bevy::ecs::query::QueryData"), <&'static [ComputedNode](../../prelude/struct.ComputedNode.html "struct bevy::prelude::ComputedNode") as [QueryData](trait.QueryData.html "trait bevy::ecs::query::QueryData")\>::[ReadOnly](trait.QueryData.html#associatedtype.ReadOnly "type bevy::ecs::query::QueryData::ReadOnly"): for<'\_\_a> [SingleEntityQueryData](trait.SingleEntityQueryData.html "trait bevy::ecs::query::SingleEntityQueryData"), &'static [UiGlobalTransform](../../prelude/struct.UiGlobalTransform.html "struct bevy::prelude::UiGlobalTransform"): for<'\_\_a> [QueryData](trait.QueryData.html "trait bevy::ecs::query::QueryData"), <&'static [UiGlobalTransform](../../prelude/struct.UiGlobalTransform.html "struct bevy::prelude::UiGlobalTransform") as [QueryData](trait.QueryData.html "trait bevy::ecs::query::QueryData")\>::[ReadOnly](trait.QueryData.html#associatedtype.ReadOnly "type bevy::ecs::query::QueryData::ReadOnly"): for<'\_\_a> [SingleEntityQueryData](trait.SingleEntityQueryData.html "trait bevy::ecs::query::SingleEntityQueryData"), [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static mut [Interaction](../../prelude/enum.Interaction.html "enum bevy::prelude::Interaction")\>: for<'\_\_a> [QueryData](trait.QueryData.html "trait bevy::ecs::query::QueryData"), <[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static mut [Interaction](../../prelude/enum.Interaction.html "enum bevy::prelude::Interaction")\> as [QueryData](trait.QueryData.html "trait bevy::ecs::query::QueryData")\>::[ReadOnly](trait.QueryData.html#associatedtype.ReadOnly "type bevy::ecs::query::QueryData::ReadOnly"): for<'\_\_a> [SingleEntityQueryData](trait.SingleEntityQueryData.html "trait bevy::ecs::query::SingleEntityQueryData"), [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static mut [RelativeCursorPosition](../../ui/struct.RelativeCursorPosition.html "struct bevy::ui::RelativeCursorPosition")\>: for<'\_\_a> [QueryData](trait.QueryData.html "trait bevy::ecs::query::QueryData"), <[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static mut [RelativeCursorPosition](../../ui/struct.RelativeCursorPosition.html "struct bevy::ui::RelativeCursorPosition")\> as [QueryData](trait.QueryData.html "trait bevy::ecs::query::QueryData")\>::[ReadOnly](trait.QueryData.html#associatedtype.ReadOnly "type bevy::ecs::query::QueryData::ReadOnly"): for<'\_\_a> [SingleEntityQueryData](trait.SingleEntityQueryData.html "trait bevy::ecs::query::SingleEntityQueryData"), [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [FocusPolicy](../../ui/enum.FocusPolicy.html "enum bevy::ui::FocusPolicy")\>: for<'\_\_a> [QueryData](trait.QueryData.html "trait bevy::ecs::query::QueryData"), <[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [FocusPolicy](../../ui/enum.FocusPolicy.html "enum bevy::ui::FocusPolicy")\> as [QueryData](trait.QueryData.html "trait bevy::ecs::query::QueryData")\>::[ReadOnly](trait.QueryData.html#associatedtype.ReadOnly "type bevy::ecs::query::QueryData::ReadOnly"): for<'\_\_a> [SingleEntityQueryData](trait.SingleEntityQueryData.html "trait bevy::ecs::query::SingleEntityQueryData"), [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [InheritedVisibility](../../prelude/struct.InheritedVisibility.html "struct bevy::prelude::InheritedVisibility")\>: for<'\_\_a> [QueryData](trait.QueryData.html "trait bevy::ecs::query::QueryData"), <[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [InheritedVisibility](../../prelude/struct.InheritedVisibility.html "struct bevy::prelude::InheritedVisibility")\> as [QueryData](trait.QueryData.html "trait bevy::ecs::query::QueryData")\>::[ReadOnly](trait.QueryData.html#associatedtype.ReadOnly "type bevy::ecs::query::QueryData::ReadOnly"): for<'\_\_a> [SingleEntityQueryData](trait.SingleEntityQueryData.html "trait bevy::ecs::query::SingleEntityQueryData"), &'static [ComputedUiTargetCamera](../../prelude/struct.ComputedUiTargetCamera.html "struct bevy::prelude::ComputedUiTargetCamera"): for<'\_\_a> [QueryData](trait.QueryData.html "trait bevy::ecs::query::QueryData"), <&'static [ComputedUiTargetCamera](../../prelude/struct.ComputedUiTargetCamera.html "struct bevy::prelude::ComputedUiTargetCamera") as [QueryData](trait.QueryData.html "trait bevy::ecs::query::QueryData")\>::[ReadOnly](trait.QueryData.html#associatedtype.ReadOnly "type bevy::ecs::query::QueryData::ReadOnly"): for<'\_\_a> [SingleEntityQueryData](trait.SingleEntityQueryData.html "trait bevy::ecs::query::SingleEntityQueryData"),

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/events.rs.html#94)

### impl [SingleEntityQueryData](trait.SingleEntityQueryData.html "trait bevy::ecs::query::SingleEntityQueryData") for [PointerTraversal](../../prelude/struct.PointerTraversal.html "struct bevy::prelude::PointerTraversal")

where [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [ChildOf](../../prelude/struct.ChildOf.html "struct bevy::prelude::ChildOf")\>: for<'\_\_a> [SingleEntityQueryData](trait.SingleEntityQueryData.html "trait bevy::ecs::query::SingleEntityQueryData"), [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [Window](../../prelude/struct.Window.html "struct bevy::prelude::Window")\>: for<'\_\_a> [SingleEntityQueryData](trait.SingleEntityQueryData.html "trait bevy::ecs::query::SingleEntityQueryData"),

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/sync_world.rs.html#398)

### impl [SingleEntityQueryData](trait.SingleEntityQueryData.html "trait bevy::ecs::query::SingleEntityQueryData") for [RenderEntity](../../render/sync_world/struct.RenderEntity.html "struct bevy::render::sync_world::RenderEntity")

SAFETY: access is only on the current entity

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#895)

### impl [SingleEntityQueryData](trait.SingleEntityQueryData.html "trait bevy::ecs::query::SingleEntityQueryData") for [SpawnDetails](struct.SpawnDetails.html "struct bevy::ecs::query::SpawnDetails")

[Source](https://docs.rs/bevy_input_focus/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input_focus/lib.rs.html#219)

### impl [SingleEntityQueryData](trait.SingleEntityQueryData.html "trait bevy::ecs::query::SingleEntityQueryData") for [WindowTraversal](../../input_focus/struct.WindowTraversal.html "struct bevy::input_focus::WindowTraversal")

where [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [ChildOf](../../prelude/struct.ChildOf.html "struct bevy::prelude::ChildOf")\>: for<'\_\_a> [SingleEntityQueryData](trait.SingleEntityQueryData.html "trait bevy::ecs::query::SingleEntityQueryData"), [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [Window](../../prelude/struct.Window.html "struct bevy::prelude::Window")\>: for<'\_\_a> [SingleEntityQueryData](trait.SingleEntityQueryData.html "trait bevy::ecs::query::SingleEntityQueryData"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#2174)

### impl<'\_\_w, T> [SingleEntityQueryData](trait.SingleEntityQueryData.html "trait bevy::ecs::query::SingleEntityQueryData") for [Ref](../../prelude/struct.Ref.html "struct bevy::prelude::Ref")<'\_\_w, T>

where T: [Component](../../prelude/trait.Component.html "trait bevy::prelude::Component"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#1644)

### impl<B> [SingleEntityQueryData](trait.SingleEntityQueryData.html "trait bevy::ecs::query::SingleEntityQueryData") for [EntityMutExcept](../world/struct.EntityMutExcept.html "struct bevy::ecs::world::EntityMutExcept")<'\_, '\_, B>

where B: [Bundle](../../prelude/trait.Bundle.html "trait bevy::prelude::Bundle"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#1523)

### impl<B> [SingleEntityQueryData](trait.SingleEntityQueryData.html "trait bevy::ecs::query::SingleEntityQueryData") for [EntityRefExcept](../world/struct.EntityRefExcept.html "struct bevy::ecs::world::EntityRefExcept")<'\_, '\_, B>

where B: [Bundle](../../prelude/trait.Bundle.html "trait bevy::prelude::Bundle"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#3797-3805)

### impl<F> [SingleEntityQueryData](trait.SingleEntityQueryData.html "trait bevy::ecs::query::SingleEntityQueryData") for [AnyOf](../../prelude/struct.AnyOf.html "struct bevy::prelude::AnyOf")<[(F₁, F₂, …, Fₙ)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html#trait-implementations-1)\>

where F: [SingleEntityQueryData](trait.SingleEntityQueryData.html "trait bevy::ecs::query::SingleEntityQueryData"),

This trait is implemented for tuples up to 16 items long.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#3374)

### impl<T> [SingleEntityQueryData](trait.SingleEntityQueryData.html "trait bevy::ecs::query::SingleEntityQueryData") for [Has](../../prelude/struct.Has.html "struct bevy::prelude::Has")<T>

where T: [Component](../../prelude/trait.Component.html "trait bevy::prelude::Component"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#2607)

### impl<T> [SingleEntityQueryData](trait.SingleEntityQueryData.html "trait bevy::ecs::query::SingleEntityQueryData") for [Mut](../../prelude/struct.Mut.html "struct bevy::prelude::Mut")<'\_, T>

where T: [Component](../../prelude/trait.Component.html "trait bevy::prelude::Component")<Mutability = [Mutable](../component/struct.Mutable.html "struct bevy::ecs::component::Mutable")\>,