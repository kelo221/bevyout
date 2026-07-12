[bevy](../../index.html)::[ecs](../index.html)::[query](index.html)

# Trait ReleaseStateQueryData 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#486)

```rust
pub trait ReleaseStateQueryData: QueryData {
    // Required method
    fn release_state<'w>(item: Self::Item<'w, '_>) -> Self::Item<'w, 'static>;
}
```

A [`QueryData`](trait.QueryData.html "trait bevy::ecs::query::QueryData") that does not borrow from its [`QueryState`](../../prelude/struct.QueryState.html "struct bevy::prelude::QueryState").

This is implemented by most `QueryData` types. The main exceptions are [`FilteredEntityRef`](../world/struct.FilteredEntityRef.html "struct bevy::ecs::world::FilteredEntityRef"), [`FilteredEntityMut`](../world/struct.FilteredEntityMut.html "struct bevy::ecs::world::FilteredEntityMut"), [`EntityRefExcept`](../world/struct.EntityRefExcept.html "struct bevy::ecs::world::EntityRefExcept"), and [`EntityMutExcept`](../world/struct.EntityMutExcept.html "struct bevy::ecs::world::EntityMutExcept"), which borrow an access list from their query state. Consider using a full [`EntityRef`](../../prelude/struct.EntityRef.html "struct bevy::prelude::EntityRef") or [`EntityMut`](../../prelude/struct.EntityMut.html "struct bevy::prelude::EntityMut") if you would need those.

## Required Methods

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#488)

#### fn [release\_state](#tymethod.release_state)<'w>(item: Self::[Item](trait.QueryData.html#associatedtype.Item "type bevy::ecs::query::QueryData::Item")<'w, '\_>) -> Self::[Item](trait.QueryData.html#associatedtype.Item "type bevy::ecs::query::QueryData::Item")<'w, 'static>

Releases the borrow from the query state by converting an item to have a `'static` state lifetime.

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementations on Foreign Types

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#3788-3796)

### impl [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData") for [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#3788-3796)

#### fn [release\_state](#tymethod.release_state)<'w>( \_: <[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html) as [QueryData](trait.QueryData.html "trait bevy::ecs::query::QueryData")\>::[Item](trait.QueryData.html#associatedtype.Item "type bevy::ecs::query::QueryData::Item")<'w, '\_>, ) -> <[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html) as [QueryData](trait.QueryData.html "trait bevy::ecs::query::QueryData")\>::[Item](trait.QueryData.html#associatedtype.Item "type bevy::ecs::query::QueryData::Item")<'w, 'static>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#3788-3796)

### impl<F> [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData") for [(F₁, F₂, …, Fₙ)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html#trait-implementations-1)

where F: [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData"),

This trait is implemented for tuples up to 16 items long.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#3788-3796)

#### fn [release\_state](#tymethod.release_state)<'w>( \_: <[(F,)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html) as [QueryData](trait.QueryData.html "trait bevy::ecs::query::QueryData")\>::[Item](trait.QueryData.html#associatedtype.Item "type bevy::ecs::query::QueryData::Item")<'w, '\_>, ) -> <[(F,)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html) as [QueryData](trait.QueryData.html "trait bevy::ecs::query::QueryData")\>::[Item](trait.QueryData.html#associatedtype.Item "type bevy::ecs::query::QueryData::Item")<'w, 'static>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#1953)

### impl<T> [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData") for [&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

where T: [Component](../../prelude/trait.Component.html "trait bevy::prelude::Component"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#1954)

#### fn [release\_state](#tymethod.release_state)<'w>( item: <[&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html) as [QueryData](trait.QueryData.html "trait bevy::ecs::query::QueryData")\>::[Item](trait.QueryData.html#associatedtype.Item "type bevy::ecs::query::QueryData::Item")<'w, '\_>, ) -> <[&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html) as [QueryData](trait.QueryData.html "trait bevy::ecs::query::QueryData")\>::[Item](trait.QueryData.html#associatedtype.Item "type bevy::ecs::query::QueryData::Item")<'w, 'static>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#2440)

### impl<T> [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData") for [&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

where T: [Component](../../prelude/trait.Component.html "trait bevy::prelude::Component")<Mutability = [Mutable](../component/struct.Mutable.html "struct bevy::ecs::component::Mutable")\>,

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#2441)

#### fn [release\_state](#tymethod.release_state)<'w>( item: <[&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html) as [QueryData](trait.QueryData.html "trait bevy::ecs::query::QueryData")\>::[Item](trait.QueryData.html#associatedtype.Item "type bevy::ecs::query::QueryData::Item")<'w, '\_>, ) -> <[&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html) as [QueryData](trait.QueryData.html "trait bevy::ecs::query::QueryData")\>::[Item](trait.QueryData.html#associatedtype.Item "type bevy::ecs::query::QueryData::Item")<'w, 'static>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#3175)

### impl<T> [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData") for [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<T>

where T: [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#3176)

#### fn [release\_state](#tymethod.release_state)<'w>( item: <[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<T> as [QueryData](trait.QueryData.html "trait bevy::ecs::query::QueryData")\>::[Item](trait.QueryData.html#associatedtype.Item "type bevy::ecs::query::QueryData::Item")<'w, '\_>, ) -> <[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<T> as [QueryData](trait.QueryData.html "trait bevy::ecs::query::QueryData")\>::[Item](trait.QueryData.html#associatedtype.Item "type bevy::ecs::query::QueryData::Item")<'w, 'static>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#3998)

### impl<T> [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData") for [PhantomData](https://doc.rust-lang.org/nightly/core/marker/struct.PhantomData.html "struct core::marker::PhantomData")<T>

where T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#3999)

#### fn [release\_state](#tymethod.release_state)<'w>( \_item: <[PhantomData](https://doc.rust-lang.org/nightly/core/marker/struct.PhantomData.html "struct core::marker::PhantomData")<T> as [QueryData](trait.QueryData.html "trait bevy::ecs::query::QueryData")\>::[Item](trait.QueryData.html#associatedtype.Item "type bevy::ecs::query::QueryData::Item")<'w, '\_>, ) -> <[PhantomData](https://doc.rust-lang.org/nightly/core/marker/struct.PhantomData.html "struct core::marker::PhantomData")<T> as [QueryData](trait.QueryData.html "trait bevy::ecs::query::QueryData")\>::[Item](trait.QueryData.html#associatedtype.Item "type bevy::ecs::query::QueryData::Item")<'w, 'static>

## Implementors

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#1746)

### impl [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData") for &[Archetype](../archetype/struct.Archetype.html "struct bevy::ecs::archetype::Archetype")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#3797-3805)

### impl [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData") for [AnyOf](../../prelude/struct.AnyOf.html "struct bevy::prelude::AnyOf")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)\>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#589)

### impl [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData") for [Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#704)

### impl [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData") for [EntityLocation](../entity/struct.EntityLocation.html "struct bevy::ecs::entity::EntityLocation")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#1139)

### impl [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData") for [EntityMut](../../prelude/struct.EntityMut.html "struct bevy::prelude::EntityMut")<'\_>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#1026)

### impl [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData") for [EntityRef](../../prelude/struct.EntityRef.html "struct bevy::prelude::EntityRef")<'\_>

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/sync_world.rs.html#523)

### impl [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData") for [MainEntity](../../render/sync_world/struct.MainEntity.html "struct bevy::render::sync_world::MainEntity")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/name.rs.html#155)

### impl [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData") for [NameOrEntity](../../prelude/struct.NameOrEntity.html "struct bevy::prelude::NameOrEntity")

where [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [Name](../../prelude/struct.Name.html "struct bevy::prelude::Name")\>: for<'\_\_a> [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData"), [Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity"): for<'\_\_a> [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData"),

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/picking_backend.rs.html#85)

### impl [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData") for bevy::ui::picking\_backend::[NodeQuery](../../ui/picking_backend/struct.NodeQuery.html "struct bevy::ui::picking_backend::NodeQuery")

where [Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity"): for<'\_\_a> [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData"), &'static [ComputedNode](../../prelude/struct.ComputedNode.html "struct bevy::prelude::ComputedNode"): for<'\_\_a> [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData"), &'static [UiGlobalTransform](../../prelude/struct.UiGlobalTransform.html "struct bevy::prelude::UiGlobalTransform"): for<'\_\_a> [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData"), [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [Pickable](../../prelude/struct.Pickable.html "struct bevy::prelude::Pickable")\>: for<'\_\_a> [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData"), [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [InheritedVisibility](../../prelude/struct.InheritedVisibility.html "struct bevy::prelude::InheritedVisibility")\>: for<'\_\_a> [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData"), &'static [ComputedUiTargetCamera](../../prelude/struct.ComputedUiTargetCamera.html "struct bevy::prelude::ComputedUiTargetCamera"): for<'\_\_a> [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData"), [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<(&'static [TextLayoutInfo](../../text/struct.TextLayoutInfo.html "struct bevy::text::TextLayoutInfo"), &'static [ComputedTextBlock](../../text/struct.ComputedTextBlock.html "struct bevy::text::ComputedTextBlock"))>: for<'\_\_a> [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData"),

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/focus.rs.html#132)

### impl [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData") for bevy::ui::[NodeQuery](../../ui/struct.NodeQuery.html "struct bevy::ui::NodeQuery")

where [Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity"): for<'\_\_a> [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData"), &'static [ComputedNode](../../prelude/struct.ComputedNode.html "struct bevy::prelude::ComputedNode"): for<'\_\_a> [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData"), &'static [UiGlobalTransform](../../prelude/struct.UiGlobalTransform.html "struct bevy::prelude::UiGlobalTransform"): for<'\_\_a> [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData"), [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static mut [Interaction](../../prelude/enum.Interaction.html "enum bevy::prelude::Interaction")\>: for<'\_\_a> [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData"), [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static mut [RelativeCursorPosition](../../ui/struct.RelativeCursorPosition.html "struct bevy::ui::RelativeCursorPosition")\>: for<'\_\_a> [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData"), [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [FocusPolicy](../../ui/enum.FocusPolicy.html "enum bevy::ui::FocusPolicy")\>: for<'\_\_a> [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData"), [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [InheritedVisibility](../../prelude/struct.InheritedVisibility.html "struct bevy::prelude::InheritedVisibility")\>: for<'\_\_a> [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData"), &'static [ComputedUiTargetCamera](../../prelude/struct.ComputedUiTargetCamera.html "struct bevy::prelude::ComputedUiTargetCamera"): for<'\_\_a> [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData"),

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/picking_backend.rs.html#85)

### impl [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData") for bevy::ui::picking\_backend::[NodeQueryReadOnly](../../ui/picking_backend/struct.NodeQueryReadOnly.html "struct bevy::ui::picking_backend::NodeQueryReadOnly")

where [Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity"): for<'\_\_a> [QueryData](trait.QueryData.html "trait bevy::ecs::query::QueryData"), <[Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity") as [QueryData](trait.QueryData.html "trait bevy::ecs::query::QueryData")\>::[ReadOnly](trait.QueryData.html#associatedtype.ReadOnly "type bevy::ecs::query::QueryData::ReadOnly"): for<'\_\_a> [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData"), &'static [ComputedNode](../../prelude/struct.ComputedNode.html "struct bevy::prelude::ComputedNode"): for<'\_\_a> [QueryData](trait.QueryData.html "trait bevy::ecs::query::QueryData"), <&'static [ComputedNode](../../prelude/struct.ComputedNode.html "struct bevy::prelude::ComputedNode") as [QueryData](trait.QueryData.html "trait bevy::ecs::query::QueryData")\>::[ReadOnly](trait.QueryData.html#associatedtype.ReadOnly "type bevy::ecs::query::QueryData::ReadOnly"): for<'\_\_a> [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData"), &'static [UiGlobalTransform](../../prelude/struct.UiGlobalTransform.html "struct bevy::prelude::UiGlobalTransform"): for<'\_\_a> [QueryData](trait.QueryData.html "trait bevy::ecs::query::QueryData"), <&'static [UiGlobalTransform](../../prelude/struct.UiGlobalTransform.html "struct bevy::prelude::UiGlobalTransform") as [QueryData](trait.QueryData.html "trait bevy::ecs::query::QueryData")\>::[ReadOnly](trait.QueryData.html#associatedtype.ReadOnly "type bevy::ecs::query::QueryData::ReadOnly"): for<'\_\_a> [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData"), [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [Pickable](../../prelude/struct.Pickable.html "struct bevy::prelude::Pickable")\>: for<'\_\_a> [QueryData](trait.QueryData.html "trait bevy::ecs::query::QueryData"), <[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [Pickable](../../prelude/struct.Pickable.html "struct bevy::prelude::Pickable")\> as [QueryData](trait.QueryData.html "trait bevy::ecs::query::QueryData")\>::[ReadOnly](trait.QueryData.html#associatedtype.ReadOnly "type bevy::ecs::query::QueryData::ReadOnly"): for<'\_\_a> [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData"), [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [InheritedVisibility](../../prelude/struct.InheritedVisibility.html "struct bevy::prelude::InheritedVisibility")\>: for<'\_\_a> [QueryData](trait.QueryData.html "trait bevy::ecs::query::QueryData"), <[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [InheritedVisibility](../../prelude/struct.InheritedVisibility.html "struct bevy::prelude::InheritedVisibility")\> as [QueryData](trait.QueryData.html "trait bevy::ecs::query::QueryData")\>::[ReadOnly](trait.QueryData.html#associatedtype.ReadOnly "type bevy::ecs::query::QueryData::ReadOnly"): for<'\_\_a> [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData"), &'static [ComputedUiTargetCamera](../../prelude/struct.ComputedUiTargetCamera.html "struct bevy::prelude::ComputedUiTargetCamera"): for<'\_\_a> [QueryData](trait.QueryData.html "trait bevy::ecs::query::QueryData"), <&'static [ComputedUiTargetCamera](../../prelude/struct.ComputedUiTargetCamera.html "struct bevy::prelude::ComputedUiTargetCamera") as [QueryData](trait.QueryData.html "trait bevy::ecs::query::QueryData")\>::[ReadOnly](trait.QueryData.html#associatedtype.ReadOnly "type bevy::ecs::query::QueryData::ReadOnly"): for<'\_\_a> [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData"), [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<(&'static [TextLayoutInfo](../../text/struct.TextLayoutInfo.html "struct bevy::text::TextLayoutInfo"), &'static [ComputedTextBlock](../../text/struct.ComputedTextBlock.html "struct bevy::text::ComputedTextBlock"))>: for<'\_\_a> [QueryData](trait.QueryData.html "trait bevy::ecs::query::QueryData"), <[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<(&'static [TextLayoutInfo](../../text/struct.TextLayoutInfo.html "struct bevy::text::TextLayoutInfo"), &'static [ComputedTextBlock](../../text/struct.ComputedTextBlock.html "struct bevy::text::ComputedTextBlock"))> as [QueryData](trait.QueryData.html "trait bevy::ecs::query::QueryData")\>::[ReadOnly](trait.QueryData.html#associatedtype.ReadOnly "type bevy::ecs::query::QueryData::ReadOnly"): for<'\_\_a> [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData"),

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/focus.rs.html#132)

### impl [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData") for bevy::ui::[NodeQueryReadOnly](../../ui/struct.NodeQueryReadOnly.html "struct bevy::ui::NodeQueryReadOnly")

where [Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity"): for<'\_\_a> [QueryData](trait.QueryData.html "trait bevy::ecs::query::QueryData"), <[Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity") as [QueryData](trait.QueryData.html "trait bevy::ecs::query::QueryData")\>::[ReadOnly](trait.QueryData.html#associatedtype.ReadOnly "type bevy::ecs::query::QueryData::ReadOnly"): for<'\_\_a> [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData"), &'static [ComputedNode](../../prelude/struct.ComputedNode.html "struct bevy::prelude::ComputedNode"): for<'\_\_a> [QueryData](trait.QueryData.html "trait bevy::ecs::query::QueryData"), <&'static [ComputedNode](../../prelude/struct.ComputedNode.html "struct bevy::prelude::ComputedNode") as [QueryData](trait.QueryData.html "trait bevy::ecs::query::QueryData")\>::[ReadOnly](trait.QueryData.html#associatedtype.ReadOnly "type bevy::ecs::query::QueryData::ReadOnly"): for<'\_\_a> [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData"), &'static [UiGlobalTransform](../../prelude/struct.UiGlobalTransform.html "struct bevy::prelude::UiGlobalTransform"): for<'\_\_a> [QueryData](trait.QueryData.html "trait bevy::ecs::query::QueryData"), <&'static [UiGlobalTransform](../../prelude/struct.UiGlobalTransform.html "struct bevy::prelude::UiGlobalTransform") as [QueryData](trait.QueryData.html "trait bevy::ecs::query::QueryData")\>::[ReadOnly](trait.QueryData.html#associatedtype.ReadOnly "type bevy::ecs::query::QueryData::ReadOnly"): for<'\_\_a> [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData"), [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static mut [Interaction](../../prelude/enum.Interaction.html "enum bevy::prelude::Interaction")\>: for<'\_\_a> [QueryData](trait.QueryData.html "trait bevy::ecs::query::QueryData"), <[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static mut [Interaction](../../prelude/enum.Interaction.html "enum bevy::prelude::Interaction")\> as [QueryData](trait.QueryData.html "trait bevy::ecs::query::QueryData")\>::[ReadOnly](trait.QueryData.html#associatedtype.ReadOnly "type bevy::ecs::query::QueryData::ReadOnly"): for<'\_\_a> [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData"), [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static mut [RelativeCursorPosition](../../ui/struct.RelativeCursorPosition.html "struct bevy::ui::RelativeCursorPosition")\>: for<'\_\_a> [QueryData](trait.QueryData.html "trait bevy::ecs::query::QueryData"), <[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static mut [RelativeCursorPosition](../../ui/struct.RelativeCursorPosition.html "struct bevy::ui::RelativeCursorPosition")\> as [QueryData](trait.QueryData.html "trait bevy::ecs::query::QueryData")\>::[ReadOnly](trait.QueryData.html#associatedtype.ReadOnly "type bevy::ecs::query::QueryData::ReadOnly"): for<'\_\_a> [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData"), [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [FocusPolicy](../../ui/enum.FocusPolicy.html "enum bevy::ui::FocusPolicy")\>: for<'\_\_a> [QueryData](trait.QueryData.html "trait bevy::ecs::query::QueryData"), <[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [FocusPolicy](../../ui/enum.FocusPolicy.html "enum bevy::ui::FocusPolicy")\> as [QueryData](trait.QueryData.html "trait bevy::ecs::query::QueryData")\>::[ReadOnly](trait.QueryData.html#associatedtype.ReadOnly "type bevy::ecs::query::QueryData::ReadOnly"): for<'\_\_a> [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData"), [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [InheritedVisibility](../../prelude/struct.InheritedVisibility.html "struct bevy::prelude::InheritedVisibility")\>: for<'\_\_a> [QueryData](trait.QueryData.html "trait bevy::ecs::query::QueryData"), <[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [InheritedVisibility](../../prelude/struct.InheritedVisibility.html "struct bevy::prelude::InheritedVisibility")\> as [QueryData](trait.QueryData.html "trait bevy::ecs::query::QueryData")\>::[ReadOnly](trait.QueryData.html#associatedtype.ReadOnly "type bevy::ecs::query::QueryData::ReadOnly"): for<'\_\_a> [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData"), &'static [ComputedUiTargetCamera](../../prelude/struct.ComputedUiTargetCamera.html "struct bevy::prelude::ComputedUiTargetCamera"): for<'\_\_a> [QueryData](trait.QueryData.html "trait bevy::ecs::query::QueryData"), <&'static [ComputedUiTargetCamera](../../prelude/struct.ComputedUiTargetCamera.html "struct bevy::prelude::ComputedUiTargetCamera") as [QueryData](trait.QueryData.html "trait bevy::ecs::query::QueryData")\>::[ReadOnly](trait.QueryData.html#associatedtype.ReadOnly "type bevy::ecs::query::QueryData::ReadOnly"): for<'\_\_a> [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData"),

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/events.rs.html#94)

### impl [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData") for [PointerTraversal](../../prelude/struct.PointerTraversal.html "struct bevy::prelude::PointerTraversal")

where [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [ChildOf](../../prelude/struct.ChildOf.html "struct bevy::prelude::ChildOf")\>: for<'\_\_a> [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData"), [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [Window](../../prelude/struct.Window.html "struct bevy::prelude::Window")\>: for<'\_\_a> [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData"),

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/sync_world.rs.html#402)

### impl [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData") for [RenderEntity](../../render/sync_world/struct.RenderEntity.html "struct bevy::render::sync_world::RenderEntity")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#897)

### impl [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData") for [SpawnDetails](struct.SpawnDetails.html "struct bevy::ecs::query::SpawnDetails")

[Source](https://docs.rs/bevy_input_focus/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input_focus/lib.rs.html#219)

### impl [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData") for [WindowTraversal](../../input_focus/struct.WindowTraversal.html "struct bevy::input_focus::WindowTraversal")

where [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [ChildOf](../../prelude/struct.ChildOf.html "struct bevy::prelude::ChildOf")\>: for<'\_\_a> [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData"), [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [Window](../../prelude/struct.Window.html "struct bevy::prelude::Window")\>: for<'\_\_a> [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#3797-3805)

### impl<F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14> [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData") for [AnyOf](../../prelude/struct.AnyOf.html "struct bevy::prelude::AnyOf")<[(F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)\>

where F0: [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData"), F1: [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData"), F2: [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData"), F3: [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData"), F4: [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData"), F5: [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData"), F6: [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData"), F7: [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData"), F8: [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData"), F9: [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData"), F10: [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData"), F11: [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData"), F12: [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData"), F13: [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData"), F14: [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#3797-3805)

### impl<F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13> [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData") for [AnyOf](../../prelude/struct.AnyOf.html "struct bevy::prelude::AnyOf")<[(F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)\>

where F0: [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData"), F1: [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData"), F2: [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData"), F3: [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData"), F4: [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData"), F5: [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData"), F6: [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData"), F7: [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData"), F8: [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData"), F9: [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData"), F10: [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData"), F11: [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData"), F12: [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData"), F13: [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#3797-3805)

### impl<F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12> [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData") for [AnyOf](../../prelude/struct.AnyOf.html "struct bevy::prelude::AnyOf")<[(F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)\>

where F0: [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData"), F1: [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData"), F2: [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData"), F3: [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData"), F4: [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData"), F5: [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData"), F6: [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData"), F7: [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData"), F8: [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData"), F9: [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData"), F10: [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData"), F11: [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData"), F12: [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#3797-3805)

### impl<F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11> [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData") for [AnyOf](../../prelude/struct.AnyOf.html "struct bevy::prelude::AnyOf")<[(F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)\>

where F0: [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData"), F1: [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData"), F2: [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData"), F3: [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData"), F4: [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData"), F5: [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData"), F6: [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData"), F7: [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData"), F8: [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData"), F9: [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData"), F10: [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData"), F11: [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#3797-3805)

### impl<F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10> [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData") for [AnyOf](../../prelude/struct.AnyOf.html "struct bevy::prelude::AnyOf")<[(F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)\>

where F0: [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData"), F1: [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData"), F2: [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData"), F3: [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData"), F4: [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData"), F5: [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData"), F6: [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData"), F7: [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData"), F8: [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData"), F9: [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData"), F10: [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#3797-3805)

### impl<F0, F1, F2, F3, F4, F5, F6, F7, F8, F9> [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData") for [AnyOf](../../prelude/struct.AnyOf.html "struct bevy::prelude::AnyOf")<[(F0, F1, F2, F3, F4, F5, F6, F7, F8, F9)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)\>

where F0: [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData"), F1: [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData"), F2: [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData"), F3: [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData"), F4: [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData"), F5: [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData"), F6: [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData"), F7: [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData"), F8: [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData"), F9: [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#3797-3805)

### impl<F0, F1, F2, F3, F4, F5, F6, F7, F8> [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData") for [AnyOf](../../prelude/struct.AnyOf.html "struct bevy::prelude::AnyOf")<[(F0, F1, F2, F3, F4, F5, F6, F7, F8)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)\>

where F0: [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData"), F1: [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData"), F2: [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData"), F3: [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData"), F4: [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData"), F5: [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData"), F6: [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData"), F7: [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData"), F8: [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#3797-3805)

### impl<F0, F1, F2, F3, F4, F5, F6, F7> [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData") for [AnyOf](../../prelude/struct.AnyOf.html "struct bevy::prelude::AnyOf")<[(F0, F1, F2, F3, F4, F5, F6, F7)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)\>

where F0: [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData"), F1: [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData"), F2: [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData"), F3: [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData"), F4: [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData"), F5: [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData"), F6: [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData"), F7: [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#3797-3805)

### impl<F0, F1, F2, F3, F4, F5, F6> [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData") for [AnyOf](../../prelude/struct.AnyOf.html "struct bevy::prelude::AnyOf")<[(F0, F1, F2, F3, F4, F5, F6)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)\>

where F0: [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData"), F1: [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData"), F2: [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData"), F3: [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData"), F4: [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData"), F5: [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData"), F6: [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#3797-3805)

### impl<F0, F1, F2, F3, F4, F5> [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData") for [AnyOf](../../prelude/struct.AnyOf.html "struct bevy::prelude::AnyOf")<[(F0, F1, F2, F3, F4, F5)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)\>

where F0: [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData"), F1: [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData"), F2: [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData"), F3: [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData"), F4: [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData"), F5: [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#3797-3805)

### impl<F0, F1, F2, F3, F4> [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData") for [AnyOf](../../prelude/struct.AnyOf.html "struct bevy::prelude::AnyOf")<[(F0, F1, F2, F3, F4)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)\>

where F0: [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData"), F1: [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData"), F2: [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData"), F3: [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData"), F4: [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#3797-3805)

### impl<F0, F1, F2, F3> [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData") for [AnyOf](../../prelude/struct.AnyOf.html "struct bevy::prelude::AnyOf")<[(F0, F1, F2, F3)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)\>

where F0: [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData"), F1: [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData"), F2: [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData"), F3: [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#3797-3805)

### impl<F0, F1, F2> [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData") for [AnyOf](../../prelude/struct.AnyOf.html "struct bevy::prelude::AnyOf")<[(F0, F1, F2)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)\>

where F0: [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData"), F1: [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData"), F2: [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#3797-3805)

### impl<F0, F1> [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData") for [AnyOf](../../prelude/struct.AnyOf.html "struct bevy::prelude::AnyOf")<[(F0, F1)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)\>

where F0: [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData"), F1: [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#3797-3805)

### impl<F> [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData") for [AnyOf](../../prelude/struct.AnyOf.html "struct bevy::prelude::AnyOf")<[(F,)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)\>

where F: [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#3376)

### impl<T> [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData") for [Has](../../prelude/struct.Has.html "struct bevy::prelude::Has")<T>

where T: [Component](../../prelude/trait.Component.html "trait bevy::prelude::Component"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#2609)

### impl<T> [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData") for [Mut](../../prelude/struct.Mut.html "struct bevy::prelude::Mut")<'\_, T>

where T: [Component](../../prelude/trait.Component.html "trait bevy::prelude::Component")<Mutability = [Mutable](../component/struct.Mutable.html "struct bevy::ecs::component::Mutable")\>,

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/fetch.rs.html#2176)

### impl<T> [ReleaseStateQueryData](trait.ReleaseStateQueryData.html "trait bevy::ecs::query::ReleaseStateQueryData") for [Ref](../../prelude/struct.Ref.html "struct bevy::prelude::Ref")<'\_, T>

where T: [Component](../../prelude/trait.Component.html "trait bevy::prelude::Component"),