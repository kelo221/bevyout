[bevy](../../index.html)::[render](../index.html)::[render\_phase](index.html)

# Trait RenderCommand 

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_phase/draw.rs.html#181)

```rust
pub trait RenderCommand<P>where
    P: PhaseItem,{
    type Param: SystemParam + 'static;
    type ViewQuery: ReadOnlyQueryData;
    type ItemQuery: ReadOnlyQueryData;

    // Required method
    fn render<'w>(
        item: &P,
        view: <<Self::ViewQuery as QueryData>::ReadOnly as QueryData>::Item<'w, '_>,
        entity: Option<<<Self::ItemQuery as QueryData>::ReadOnly as QueryData>::Item<'w, '_>>,
        param: <Self::Param as SystemParam>::Item<'w, '_>,
        pass: &mut TrackedRenderPass<'w>,
    ) -> RenderCommandResult;
}
```

[`RenderCommand`](trait.RenderCommand.html "trait bevy::render::render_phase::RenderCommand")s are modular standardized pieces of render logic that can be composed into [`Draw`](trait.Draw.html "trait bevy::render::render_phase::Draw") functions.

To turn a stateless render command into a usable draw function it has to be wrapped by a [`RenderCommandState`](struct.RenderCommandState.html "struct bevy::render::render_phase::RenderCommandState"). This is done automatically when registering a render command as a [`Draw`](trait.Draw.html "trait bevy::render::render_phase::Draw") function via the [`AddRenderCommand::add_render_command`](trait.AddRenderCommand.html#tymethod.add_render_command "method bevy::render::render_phase::AddRenderCommand::add_render_command") method.

Compared to the draw function the required ECS data is fetched automatically (by the [`RenderCommandState`](struct.RenderCommandState.html "struct bevy::render::render_phase::RenderCommandState")) from the render world. Therefore the three types [`Param`](trait.RenderCommand.html#associatedtype.Param "associated type bevy::render::render_phase::RenderCommand::Param"), [`ViewQuery`](trait.RenderCommand.html#associatedtype.ViewQuery "associated type bevy::render::render_phase::RenderCommand::ViewQuery") and [`ItemQuery`](trait.RenderCommand.html#associatedtype.ItemQuery "associated type bevy::render::render_phase::RenderCommand::ItemQuery") are used. They specify which information is required to execute the render command.

Multiple render commands can be combined together by wrapping them in a tuple.

## Example

The `DrawMaterial` draw function is created from the following render command tuple. Const generics are used to set specific bind group locations:

```rust
pub type DrawMaterial<M> = (
    SetItemPipeline,
    SetMeshViewBindGroup<0>,
    SetMeshViewBindingArrayBindGroup<1>,
    SetMeshBindGroup<2>,
    SetMaterialBindGroup<M, 3>,
    DrawMesh,
);
```

## Required Associated Types

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_phase/draw.rs.html#193)

#### type [Param](#associatedtype.Param): [SystemParam](../../ecs/system/trait.SystemParam.html "trait bevy::ecs::system::SystemParam") + 'static

Specifies the general ECS data (e.g. resources) required by [`RenderCommand::render`](trait.RenderCommand.html#tymethod.render "associated function bevy::render::render_phase::RenderCommand::render").

When fetching resources, note that, due to lifetime limitations of the `Deref` trait, [`SRes::into_inner`](../../prelude/struct.Res.html#method.into_inner "method bevy::prelude::Res::into_inner") must be called on each [`SRes`](../../ecs/system/lifetimeless/type.SRes.html "type bevy::ecs::system::lifetimeless::SRes") reference in the [`RenderCommand::render`](trait.RenderCommand.html#tymethod.render "associated function bevy::render::render_phase::RenderCommand::render") method, instead of being automatically dereferenced as is the case in normal `systems`.

All parameters have to be read only.

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_phase/draw.rs.html#199)

#### type [ViewQuery](#associatedtype.ViewQuery): [ReadOnlyQueryData](../../ecs/query/trait.ReadOnlyQueryData.html "trait bevy::ecs::query::ReadOnlyQueryData")

Specifies the ECS data of the view entity required by [`RenderCommand::render`](trait.RenderCommand.html#tymethod.render "associated function bevy::render::render_phase::RenderCommand::render").

The view entity refers to the camera, or shadow-casting light, etc. from which the phase item will be rendered from. All components have to be accessed read only.

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_phase/draw.rs.html#209)

#### type [ItemQuery](#associatedtype.ItemQuery): [ReadOnlyQueryData](../../ecs/query/trait.ReadOnlyQueryData.html "trait bevy::ecs::query::ReadOnlyQueryData")

Specifies the ECS data of the item entity required by [`RenderCommand::render`](trait.RenderCommand.html#tymethod.render "associated function bevy::render::render_phase::RenderCommand::render").

The item is the entity that will be rendered for the corresponding view. All components have to be accessed read only.

For efficiency reasons, Bevy doesn’t always extract entities to the render world; for instance, entities that simply consist of meshes are often not extracted. If the entity doesn’t exist in the render world, the supplied query data will be `None`.

## Required Methods

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_phase/draw.rs.html#213-219)

#### fn [render](#tymethod.render)<'w>( item: [&P](https://doc.rust-lang.org/nightly/std/primitive.reference.html), view: <<Self::[ViewQuery](trait.RenderCommand.html#associatedtype.ViewQuery "type bevy::render::render_phase::RenderCommand::ViewQuery") as [QueryData](../../ecs/query/trait.QueryData.html "trait bevy::ecs::query::QueryData")\>::[ReadOnly](../../ecs/query/trait.QueryData.html#associatedtype.ReadOnly "type bevy::ecs::query::QueryData::ReadOnly") as [QueryData](../../ecs/query/trait.QueryData.html "trait bevy::ecs::query::QueryData")\>::[Item](../../ecs/query/trait.QueryData.html#associatedtype.Item "type bevy::ecs::query::QueryData::Item")<'w, '\_>, entity: [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<<<Self::[ItemQuery](trait.RenderCommand.html#associatedtype.ItemQuery "type bevy::render::render_phase::RenderCommand::ItemQuery") as [QueryData](../../ecs/query/trait.QueryData.html "trait bevy::ecs::query::QueryData")\>::[ReadOnly](../../ecs/query/trait.QueryData.html#associatedtype.ReadOnly "type bevy::ecs::query::QueryData::ReadOnly") as [QueryData](../../ecs/query/trait.QueryData.html "trait bevy::ecs::query::QueryData")\>::[Item](../../ecs/query/trait.QueryData.html#associatedtype.Item "type bevy::ecs::query::QueryData::Item")<'w, '\_>>, param: <Self::[Param](trait.RenderCommand.html#associatedtype.Param "type bevy::render::render_phase::RenderCommand::Param") as [SystemParam](../../ecs/system/trait.SystemParam.html "trait bevy::ecs::system::SystemParam")\>::[Item](../../ecs/system/trait.SystemParam.html#associatedtype.Item "type bevy::ecs::system::SystemParam::Item")<'w, '\_>, pass: &mut [TrackedRenderPass](struct.TrackedRenderPass.html "struct bevy::render::render_phase::TrackedRenderPass")<'w>, ) -> [RenderCommandResult](enum.RenderCommandResult.html "enum bevy::render::render_phase::RenderCommandResult")

Renders a [`PhaseItem`](trait.PhaseItem.html "trait bevy::render::render_phase::PhaseItem") by recording commands (e.g. setting pipelines, binding bind groups, issuing draw calls, etc.) via the [`TrackedRenderPass`](struct.TrackedRenderPass.html "struct bevy::render::render_phase::TrackedRenderPass").

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementations on Foreign Types

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_phase/draw.rs.html#279-287)

### impl<P, C> [RenderCommand](trait.RenderCommand.html "trait bevy::render::render_phase::RenderCommand")<P> for [(C₁, C₂, …, Cₙ)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html#trait-implementations-1)

where P: [PhaseItem](trait.PhaseItem.html "trait bevy::render::render_phase::PhaseItem"), C: [RenderCommand](trait.RenderCommand.html "trait bevy::render::render_phase::RenderCommand")<P>,

This trait is implemented for tuples up to 16 items long.

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_phase/draw.rs.html#279-287)

#### type [Param](#associatedtype.Param) = (<C as [RenderCommand](trait.RenderCommand.html "trait bevy::render::render_phase::RenderCommand")<P>>::[Param](trait.RenderCommand.html#associatedtype.Param "type bevy::render::render_phase::RenderCommand::Param"),)

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_phase/draw.rs.html#279-287)

#### type [ViewQuery](#associatedtype.ViewQuery) = (<C as [RenderCommand](trait.RenderCommand.html "trait bevy::render::render_phase::RenderCommand")<P>>::[ViewQuery](trait.RenderCommand.html#associatedtype.ViewQuery "type bevy::render::render_phase::RenderCommand::ViewQuery"),)

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_phase/draw.rs.html#279-287)

#### type [ItemQuery](#associatedtype.ItemQuery) = (<C as [RenderCommand](trait.RenderCommand.html "trait bevy::render::render_phase::RenderCommand")<P>>::[ItemQuery](trait.RenderCommand.html#associatedtype.ItemQuery "type bevy::render::render_phase::RenderCommand::ItemQuery"),)

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_phase/draw.rs.html#279-287)

#### fn [render](#tymethod.render)<'w>( \_item: [&P](https://doc.rust-lang.org/nightly/std/primitive.reference.html), \_: <<<[(C,)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html) as [RenderCommand](trait.RenderCommand.html "trait bevy::render::render_phase::RenderCommand")<P>>::[ViewQuery](trait.RenderCommand.html#associatedtype.ViewQuery "type bevy::render::render_phase::RenderCommand::ViewQuery") as [QueryData](../../ecs/query/trait.QueryData.html "trait bevy::ecs::query::QueryData")\>::[ReadOnly](../../ecs/query/trait.QueryData.html#associatedtype.ReadOnly "type bevy::ecs::query::QueryData::ReadOnly") as [QueryData](../../ecs/query/trait.QueryData.html "trait bevy::ecs::query::QueryData")\>::[Item](../../ecs/query/trait.QueryData.html#associatedtype.Item "type bevy::ecs::query::QueryData::Item")<'w, '\_>, maybe\_entities: [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<<<<[(C,)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html) as [RenderCommand](trait.RenderCommand.html "trait bevy::render::render_phase::RenderCommand")<P>>::[ItemQuery](trait.RenderCommand.html#associatedtype.ItemQuery "type bevy::render::render_phase::RenderCommand::ItemQuery") as [QueryData](../../ecs/query/trait.QueryData.html "trait bevy::ecs::query::QueryData")\>::[ReadOnly](../../ecs/query/trait.QueryData.html#associatedtype.ReadOnly "type bevy::ecs::query::QueryData::ReadOnly") as [QueryData](../../ecs/query/trait.QueryData.html "trait bevy::ecs::query::QueryData")\>::[Item](../../ecs/query/trait.QueryData.html#associatedtype.Item "type bevy::ecs::query::QueryData::Item")<'w, '\_>>, \_: <<[(C,)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html) as [RenderCommand](trait.RenderCommand.html "trait bevy::render::render_phase::RenderCommand")<P>>::[Param](trait.RenderCommand.html#associatedtype.Param "type bevy::render::render_phase::RenderCommand::Param") as [SystemParam](../../ecs/system/trait.SystemParam.html "trait bevy::ecs::system::SystemParam")\>::[Item](../../ecs/system/trait.SystemParam.html#associatedtype.Item "type bevy::ecs::system::SystemParam::Item")<'w, '\_>, \_pass: &mut [TrackedRenderPass](struct.TrackedRenderPass.html "struct bevy::render::render_phase::TrackedRenderPass")<'w>, ) -> [RenderCommandResult](enum.RenderCommandResult.html "enum bevy::render::render_phase::RenderCommandResult")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_phase/draw.rs.html#279-287)

### impl<P> [RenderCommand](trait.RenderCommand.html "trait bevy::render::render_phase::RenderCommand")<P> for [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

where P: [PhaseItem](trait.PhaseItem.html "trait bevy::render::render_phase::PhaseItem"),

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_phase/draw.rs.html#279-287)

#### type [Param](#associatedtype.Param) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_phase/draw.rs.html#279-287)

#### type [ViewQuery](#associatedtype.ViewQuery) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_phase/draw.rs.html#279-287)

#### type [ItemQuery](#associatedtype.ItemQuery) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_phase/draw.rs.html#279-287)

#### fn [render](#tymethod.render)<'w>( \_item: [&P](https://doc.rust-lang.org/nightly/std/primitive.reference.html), \_: <<<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html) as [RenderCommand](trait.RenderCommand.html "trait bevy::render::render_phase::RenderCommand")<P>>::[ViewQuery](trait.RenderCommand.html#associatedtype.ViewQuery "type bevy::render::render_phase::RenderCommand::ViewQuery") as [QueryData](../../ecs/query/trait.QueryData.html "trait bevy::ecs::query::QueryData")\>::[ReadOnly](../../ecs/query/trait.QueryData.html#associatedtype.ReadOnly "type bevy::ecs::query::QueryData::ReadOnly") as [QueryData](../../ecs/query/trait.QueryData.html "trait bevy::ecs::query::QueryData")\>::[Item](../../ecs/query/trait.QueryData.html#associatedtype.Item "type bevy::ecs::query::QueryData::Item")<'w, '\_>, maybe\_entities: [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<<<<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html) as [RenderCommand](trait.RenderCommand.html "trait bevy::render::render_phase::RenderCommand")<P>>::[ItemQuery](trait.RenderCommand.html#associatedtype.ItemQuery "type bevy::render::render_phase::RenderCommand::ItemQuery") as [QueryData](../../ecs/query/trait.QueryData.html "trait bevy::ecs::query::QueryData")\>::[ReadOnly](../../ecs/query/trait.QueryData.html#associatedtype.ReadOnly "type bevy::ecs::query::QueryData::ReadOnly") as [QueryData](../../ecs/query/trait.QueryData.html "trait bevy::ecs::query::QueryData")\>::[Item](../../ecs/query/trait.QueryData.html#associatedtype.Item "type bevy::ecs::query::QueryData::Item")<'w, '\_>>, \_: <<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html) as [RenderCommand](trait.RenderCommand.html "trait bevy::render::render_phase::RenderCommand")<P>>::[Param](trait.RenderCommand.html#associatedtype.Param "type bevy::render::render_phase::RenderCommand::Param") as [SystemParam](../../ecs/system/trait.SystemParam.html "trait bevy::ecs::system::SystemParam")\>::[Item](../../ecs/system/trait.SystemParam.html#associatedtype.Item "type bevy::ecs::system::SystemParam::Item")<'w, '\_>, \_pass: &mut [TrackedRenderPass](struct.TrackedRenderPass.html "struct bevy::render::render_phase::TrackedRenderPass")<'w>, ) -> [RenderCommandResult](enum.RenderCommandResult.html "enum bevy::render::render_phase::RenderCommandResult")

## Implementors

[Source](https://docs.rs/bevy_ui_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_render/ui_material_pipeline.rs.html#221)

### impl<P, M, const I: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\> [RenderCommand](trait.RenderCommand.html "trait bevy::render::render_phase::RenderCommand")<P> for [SetMatUiViewBindGroup](../../ui_render/struct.SetMatUiViewBindGroup.html "struct bevy::ui_render::SetMatUiViewBindGroup")<M, I>

where P: [PhaseItem](trait.PhaseItem.html "trait bevy::render::render_phase::PhaseItem"), M: [UiMaterial](../../prelude/trait.UiMaterial.html "trait bevy::prelude::UiMaterial"),

[Source](https://docs.rs/bevy_ui_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_render/ui_material_pipeline.rs.html#222)

#### type [Param](#associatedtype.Param) = [Res](../../prelude/struct.Res.html "struct bevy::prelude::Res")<'static, [UiMaterialMeta](../../ui_render/struct.UiMaterialMeta.html "struct bevy::ui_render::UiMaterialMeta")<M>>

[Source](https://docs.rs/bevy_ui_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_render/ui_material_pipeline.rs.html#223)

#### type [ViewQuery](#associatedtype.ViewQuery) = &'static [ViewUniformOffset](../view/struct.ViewUniformOffset.html "struct bevy::render::view::ViewUniformOffset")

[Source](https://docs.rs/bevy_ui_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_render/ui_material_pipeline.rs.html#224)

#### type [ItemQuery](#associatedtype.ItemQuery) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/mesh2d/material.rs.html#531-532)

### impl<P, M, const I: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\> [RenderCommand](trait.RenderCommand.html "trait bevy::render::render_phase::RenderCommand")<P> for [SetMaterial2dBindGroup](../../sprite_render/struct.SetMaterial2dBindGroup.html "struct bevy::sprite_render::SetMaterial2dBindGroup")<M, I>

where P: [PhaseItem](trait.PhaseItem.html "trait bevy::render::render_phase::PhaseItem"), M: [Material2d](../../sprite_render/trait.Material2d.html "trait bevy::sprite_render::Material2d"),

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/mesh2d/material.rs.html#534)

#### type [Param](#associatedtype.Param) = ([Res](../../prelude/struct.Res.html "struct bevy::prelude::Res")<'static, [RenderAssets](../render_asset/struct.RenderAssets.html "struct bevy::render::render_asset::RenderAssets")<[PreparedMaterial2d](../../sprite_render/struct.PreparedMaterial2d.html "struct bevy::sprite_render::PreparedMaterial2d")<M>>>, [Res](../../prelude/struct.Res.html "struct bevy::prelude::Res")<'static, [RenderMaterial2dInstances](../../sprite_render/struct.RenderMaterial2dInstances.html "struct bevy::sprite_render::RenderMaterial2dInstances")<M>>)

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/mesh2d/material.rs.html#538)

#### type [ViewQuery](#associatedtype.ViewQuery) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/mesh2d/material.rs.html#539)

#### type [ItemQuery](#associatedtype.ItemQuery) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_ui_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_render/ui_material_pipeline.rs.html#243-244)

### impl<P, M, const I: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\> [RenderCommand](trait.RenderCommand.html "trait bevy::render::render_phase::RenderCommand")<P> for [SetUiMaterialBindGroup](../../ui_render/struct.SetUiMaterialBindGroup.html "struct bevy::ui_render::SetUiMaterialBindGroup")<M, I>

where P: [PhaseItem](trait.PhaseItem.html "trait bevy::render::render_phase::PhaseItem"), M: [UiMaterial](../../prelude/trait.UiMaterial.html "trait bevy::prelude::UiMaterial"),

[Source](https://docs.rs/bevy_ui_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_render/ui_material_pipeline.rs.html#246)

#### type [Param](#associatedtype.Param) = [Res](../../prelude/struct.Res.html "struct bevy::prelude::Res")<'static, [RenderAssets](../render_asset/struct.RenderAssets.html "struct bevy::render::render_asset::RenderAssets")<[PreparedUiMaterial](../../ui_render/struct.PreparedUiMaterial.html "struct bevy::ui_render::PreparedUiMaterial")<M>>>

[Source](https://docs.rs/bevy_ui_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_render/ui_material_pipeline.rs.html#247)

#### type [ViewQuery](#associatedtype.ViewQuery) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_ui_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_render/ui_material_pipeline.rs.html#248)

#### type [ItemQuery](#associatedtype.ItemQuery) = &'static [UiMaterialBatch](../../ui_render/struct.UiMaterialBatch.html "struct bevy::ui_render::UiMaterialBatch")<M>

[Source](https://docs.rs/bevy_ui_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_render/ui_material_pipeline.rs.html#269)

### impl<P, M> [RenderCommand](trait.RenderCommand.html "trait bevy::render::render_phase::RenderCommand")<P> for [DrawUiMaterialNode](../../ui_render/struct.DrawUiMaterialNode.html "struct bevy::ui_render::DrawUiMaterialNode")<M>

where P: [PhaseItem](trait.PhaseItem.html "trait bevy::render::render_phase::PhaseItem"), M: [UiMaterial](../../prelude/trait.UiMaterial.html "trait bevy::prelude::UiMaterial"),

[Source](https://docs.rs/bevy_ui_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_render/ui_material_pipeline.rs.html#270)

#### type [Param](#associatedtype.Param) = [Res](../../prelude/struct.Res.html "struct bevy::prelude::Res")<'static, [UiMaterialMeta](../../ui_render/struct.UiMaterialMeta.html "struct bevy::ui_render::UiMaterialMeta")<M>>

[Source](https://docs.rs/bevy_ui_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_render/ui_material_pipeline.rs.html#271)

#### type [ViewQuery](#associatedtype.ViewQuery) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_ui_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_render/ui_material_pipeline.rs.html#272)

#### type [ItemQuery](#associatedtype.ItemQuery) = &'static [UiMaterialBatch](../../ui_render/struct.UiMaterialBatch.html "struct bevy::ui_render::UiMaterialBatch")<M>

[Source](https://docs.rs/bevy_ui_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_render/box_shadow.rs.html#507)

### impl<P, const I: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\> [RenderCommand](trait.RenderCommand.html "trait bevy::render::render_phase::RenderCommand")<P> for [SetBoxShadowViewBindGroup](../../ui_render/box_shadow/struct.SetBoxShadowViewBindGroup.html "struct bevy::ui_render::box_shadow::SetBoxShadowViewBindGroup")<I>

where P: [PhaseItem](trait.PhaseItem.html "trait bevy::render::render_phase::PhaseItem"),

[Source](https://docs.rs/bevy_ui_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_render/box_shadow.rs.html#508)

#### type [Param](#associatedtype.Param) = [Res](../../prelude/struct.Res.html "struct bevy::prelude::Res")<'static, [BoxShadowMeta](../../ui_render/box_shadow/struct.BoxShadowMeta.html "struct bevy::ui_render::box_shadow::BoxShadowMeta")\>

[Source](https://docs.rs/bevy_ui_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_render/box_shadow.rs.html#509)

#### type [ViewQuery](#associatedtype.ViewQuery) = &'static [ViewUniformOffset](../view/struct.ViewUniformOffset.html "struct bevy::render::view::ViewUniformOffset")

[Source](https://docs.rs/bevy_ui_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_render/box_shadow.rs.html#510)

#### type [ItemQuery](#associatedtype.ItemQuery) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/material.rs.html#521)

### impl<P, const I: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\> [RenderCommand](trait.RenderCommand.html "trait bevy::render::render_phase::RenderCommand")<P> for [SetMaterialBindGroup](../../pbr/struct.SetMaterialBindGroup.html "struct bevy::pbr::SetMaterialBindGroup")<I>

where P: [PhaseItem](trait.PhaseItem.html "trait bevy::render::render_phase::PhaseItem"),

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/material.rs.html#522)

#### type [Param](#associatedtype.Param) = ([Res](../../prelude/struct.Res.html "struct bevy::prelude::Res")<'static, [ErasedRenderAssets](../erased_render_asset/struct.ErasedRenderAssets.html "struct bevy::render::erased_render_asset::ErasedRenderAssets")<[PreparedMaterial](../../pbr/struct.PreparedMaterial.html "struct bevy::pbr::PreparedMaterial")\>>, [Res](../../prelude/struct.Res.html "struct bevy::prelude::Res")<'static, [RenderMaterialInstances](../../pbr/struct.RenderMaterialInstances.html "struct bevy::pbr::RenderMaterialInstances")\>, [Res](../../prelude/struct.Res.html "struct bevy::prelude::Res")<'static, [MaterialBindGroupAllocators](../../pbr/struct.MaterialBindGroupAllocators.html "struct bevy::pbr::MaterialBindGroupAllocators")\>)

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/material.rs.html#527)

#### type [ViewQuery](#associatedtype.ViewQuery) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/material.rs.html#528)

#### type [ItemQuery](#associatedtype.ItemQuery) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/mesh2d/mesh.rs.html#834)

### impl<P, const I: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\> [RenderCommand](trait.RenderCommand.html "trait bevy::render::render_phase::RenderCommand")<P> for [SetMesh2dBindGroup](../../sprite_render/struct.SetMesh2dBindGroup.html "struct bevy::sprite_render::SetMesh2dBindGroup")<I>

where P: [PhaseItem](trait.PhaseItem.html "trait bevy::render::render_phase::PhaseItem"),

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/mesh2d/mesh.rs.html#835)

#### type [Param](#associatedtype.Param) = [Res](../../prelude/struct.Res.html "struct bevy::prelude::Res")<'static, [Mesh2dBindGroup](../../sprite_render/struct.Mesh2dBindGroup.html "struct bevy::sprite_render::Mesh2dBindGroup")\>

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/mesh2d/mesh.rs.html#836)

#### type [ViewQuery](#associatedtype.ViewQuery) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/mesh2d/mesh.rs.html#837)

#### type [ItemQuery](#associatedtype.ItemQuery) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/mesh2d/mesh.rs.html#814)

### impl<P, const I: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\> [RenderCommand](trait.RenderCommand.html "trait bevy::render::render_phase::RenderCommand")<P> for [SetMesh2dViewBindGroup](../../sprite_render/struct.SetMesh2dViewBindGroup.html "struct bevy::sprite_render::SetMesh2dViewBindGroup")<I>

where P: [PhaseItem](trait.PhaseItem.html "trait bevy::render::render_phase::PhaseItem"),

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/mesh2d/mesh.rs.html#815)

#### type [Param](#associatedtype.Param) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/mesh2d/mesh.rs.html#816)

#### type [ViewQuery](#associatedtype.ViewQuery) = (&'static [ViewUniformOffset](../view/struct.ViewUniformOffset.html "struct bevy::render::view::ViewUniformOffset"), &'static [Mesh2dViewBindGroup](../../sprite_render/struct.Mesh2dViewBindGroup.html "struct bevy::sprite_render::Mesh2dViewBindGroup"))

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/mesh2d/mesh.rs.html#817)

#### type [ItemQuery](#associatedtype.ItemQuery) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#4250)

### impl<P, const I: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\> [RenderCommand](trait.RenderCommand.html "trait bevy::render::render_phase::RenderCommand")<P> for [SetMeshBindGroup](../../pbr/struct.SetMeshBindGroup.html "struct bevy::pbr::SetMeshBindGroup")<I>

where P: [PhaseItem](trait.PhaseItem.html "trait bevy::render::render_phase::PhaseItem"),

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#4251)

#### type [Param](#associatedtype.Param) = ([Res](../../prelude/struct.Res.html "struct bevy::prelude::Res")<'static, [RenderDevice](../renderer/struct.RenderDevice.html "struct bevy::render::renderer::RenderDevice")\>, [Res](../../prelude/struct.Res.html "struct bevy::prelude::Res")<'static, [MeshBindGroups](../../pbr/enum.MeshBindGroups.html "enum bevy::pbr::MeshBindGroups")\>, [Res](../../prelude/struct.Res.html "struct bevy::prelude::Res")<'static, [RenderMeshInstances](../../pbr/enum.RenderMeshInstances.html "enum bevy::pbr::RenderMeshInstances")\>, [Res](../../prelude/struct.Res.html "struct bevy::prelude::Res")<'static, [SkinUniforms](../../pbr/struct.SkinUniforms.html "struct bevy::pbr::SkinUniforms")\>, [Res](../../prelude/struct.Res.html "struct bevy::prelude::Res")<'static, [MorphIndices](../../pbr/enum.MorphIndices.html "enum bevy::pbr::MorphIndices")\>, [Res](../../prelude/struct.Res.html "struct bevy::prelude::Res")<'static, [MeshAllocator](../mesh/allocator/struct.MeshAllocator.html "struct bevy::render::mesh::allocator::MeshAllocator")\>, [Res](../../prelude/struct.Res.html "struct bevy::prelude::Res")<'static, [RenderLightmaps](../../pbr/struct.RenderLightmaps.html "struct bevy::pbr::RenderLightmaps")\>)

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#4260)

#### type [ViewQuery](#associatedtype.ViewQuery) = [Has](../../prelude/struct.Has.html "struct bevy::prelude::Has")<[MotionVectorPrepass](../../core_pipeline/prepass/struct.MotionVectorPrepass.html "struct bevy::core_pipeline::prepass::MotionVectorPrepass")\>

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#4261)

#### type [ItemQuery](#associatedtype.ItemQuery) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#4186)

### impl<P, const I: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\> [RenderCommand](trait.RenderCommand.html "trait bevy::render::render_phase::RenderCommand")<P> for [SetMeshViewBindGroup](../../pbr/struct.SetMeshViewBindGroup.html "struct bevy::pbr::SetMeshViewBindGroup")<I>

where P: [PhaseItem](trait.PhaseItem.html "trait bevy::render::render_phase::PhaseItem"),

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#4187)

#### type [Param](#associatedtype.Param) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#4188)

#### type [ViewQuery](#associatedtype.ViewQuery) = (&'static [MeshViewBindGroup](../../pbr/struct.MeshViewBindGroup.html "struct bevy::pbr::MeshViewBindGroup"),)

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#4189)

#### type [ItemQuery](#associatedtype.ItemQuery) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#4210)

### impl<P, const I: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\> [RenderCommand](trait.RenderCommand.html "trait bevy::render::render_phase::RenderCommand")<P> for [SetMeshViewBindingArrayBindGroup](../../pbr/struct.SetMeshViewBindingArrayBindGroup.html "struct bevy::pbr::SetMeshViewBindingArrayBindGroup")<I>

where P: [PhaseItem](trait.PhaseItem.html "trait bevy::render::render_phase::PhaseItem"),

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#4211)

#### type [Param](#associatedtype.Param) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#4212)

#### type [ViewQuery](#associatedtype.ViewQuery) = (&'static [MeshViewBindGroup](../../pbr/struct.MeshViewBindGroup.html "struct bevy::pbr::MeshViewBindGroup"),)

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#4213)

#### type [ItemQuery](#associatedtype.ItemQuery) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#4230)

### impl<P, const I: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\> [RenderCommand](trait.RenderCommand.html "trait bevy::render::render_phase::RenderCommand")<P> for [SetMeshViewEmptyBindGroup](../../pbr/struct.SetMeshViewEmptyBindGroup.html "struct bevy::pbr::SetMeshViewEmptyBindGroup")<I>

where P: [PhaseItem](trait.PhaseItem.html "trait bevy::render::render_phase::PhaseItem"),

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#4231)

#### type [Param](#associatedtype.Param) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#4232)

#### type [ViewQuery](#associatedtype.ViewQuery) = (&'static [MeshViewBindGroup](../../pbr/struct.MeshViewBindGroup.html "struct bevy::pbr::MeshViewBindGroup"),)

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#4233)

#### type [ItemQuery](#associatedtype.ItemQuery) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/prepass/mod.rs.html#1458)

### impl<P, const I: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\> [RenderCommand](trait.RenderCommand.html "trait bevy::render::render_phase::RenderCommand")<P> for [SetPrepassEmptyMaterialBindGroup](../../pbr/struct.SetPrepassEmptyMaterialBindGroup.html "struct bevy::pbr::SetPrepassEmptyMaterialBindGroup")<I>

where P: [PhaseItem](trait.PhaseItem.html "trait bevy::render::render_phase::PhaseItem"),

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/prepass/mod.rs.html#1459)

#### type [Param](#associatedtype.Param) = [Res](../../prelude/struct.Res.html "struct bevy::prelude::Res")<'static, [PrepassViewBindGroup](../../pbr/struct.PrepassViewBindGroup.html "struct bevy::pbr::PrepassViewBindGroup")\>

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/prepass/mod.rs.html#1460)

#### type [ViewQuery](#associatedtype.ViewQuery) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/prepass/mod.rs.html#1461)

#### type [ItemQuery](#associatedtype.ItemQuery) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/prepass/mod.rs.html#1391)

### impl<P, const I: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\> [RenderCommand](trait.RenderCommand.html "trait bevy::render::render_phase::RenderCommand")<P> for [SetPrepassViewBindGroup](../../pbr/struct.SetPrepassViewBindGroup.html "struct bevy::pbr::SetPrepassViewBindGroup")<I>

where P: [PhaseItem](trait.PhaseItem.html "trait bevy::render::render_phase::PhaseItem"),

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/prepass/mod.rs.html#1392)

#### type [Param](#associatedtype.Param) = [Res](../../prelude/struct.Res.html "struct bevy::prelude::Res")<'static, [PrepassViewBindGroup](../../pbr/struct.PrepassViewBindGroup.html "struct bevy::pbr::PrepassViewBindGroup")\>

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/prepass/mod.rs.html#1393)

#### type [ViewQuery](#associatedtype.ViewQuery) = (&'static [ViewUniformOffset](../view/struct.ViewUniformOffset.html "struct bevy::render::view::ViewUniformOffset"), [Has](../../prelude/struct.Has.html "struct bevy::prelude::Has")<[MotionVectorPrepass](../../core_pipeline/prepass/struct.MotionVectorPrepass.html "struct bevy::core_pipeline::prepass::MotionVectorPrepass")\>, [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [PreviousViewUniformOffset](../../core_pipeline/prepass/struct.PreviousViewUniformOffset.html "struct bevy::core_pipeline::prepass::PreviousViewUniformOffset")\>)

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/prepass/mod.rs.html#1398)

#### type [ItemQuery](#associatedtype.ItemQuery) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/prepass/mod.rs.html#1438)

### impl<P, const I: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\> [RenderCommand](trait.RenderCommand.html "trait bevy::render::render_phase::RenderCommand")<P> for [SetPrepassViewEmptyBindGroup](../../pbr/struct.SetPrepassViewEmptyBindGroup.html "struct bevy::pbr::SetPrepassViewEmptyBindGroup")<I>

where P: [PhaseItem](trait.PhaseItem.html "trait bevy::render::render_phase::PhaseItem"),

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/prepass/mod.rs.html#1439)

#### type [Param](#associatedtype.Param) = [Res](../../prelude/struct.Res.html "struct bevy::prelude::Res")<'static, [PrepassViewBindGroup](../../pbr/struct.PrepassViewBindGroup.html "struct bevy::pbr::PrepassViewBindGroup")\>

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/prepass/mod.rs.html#1440)

#### type [ViewQuery](#associatedtype.ViewQuery) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/prepass/mod.rs.html#1441)

#### type [ItemQuery](#associatedtype.ItemQuery) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_ui_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_render/ui_texture_slice_pipeline.rs.html#667)

### impl<P, const I: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\> [RenderCommand](trait.RenderCommand.html "trait bevy::render::render_phase::RenderCommand")<P> for [SetSlicerTextureBindGroup](../../ui_render/ui_texture_slice_pipeline/struct.SetSlicerTextureBindGroup.html "struct bevy::ui_render::ui_texture_slice_pipeline::SetSlicerTextureBindGroup")<I>

where P: [PhaseItem](trait.PhaseItem.html "trait bevy::render::render_phase::PhaseItem"),

[Source](https://docs.rs/bevy_ui_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_render/ui_texture_slice_pipeline.rs.html#668)

#### type [Param](#associatedtype.Param) = [Res](../../prelude/struct.Res.html "struct bevy::prelude::Res")<'static, [UiTextureSliceImageBindGroups](../../ui_render/ui_texture_slice_pipeline/struct.UiTextureSliceImageBindGroups.html "struct bevy::ui_render::ui_texture_slice_pipeline::UiTextureSliceImageBindGroups")\>

[Source](https://docs.rs/bevy_ui_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_render/ui_texture_slice_pipeline.rs.html#669)

#### type [ViewQuery](#associatedtype.ViewQuery) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_ui_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_render/ui_texture_slice_pipeline.rs.html#670)

#### type [ItemQuery](#associatedtype.ItemQuery) = &'static [UiTextureSlicerBatch](../../ui_render/ui_texture_slice_pipeline/struct.UiTextureSlicerBatch.html "struct bevy::ui_render::ui_texture_slice_pipeline::UiTextureSlicerBatch")

[Source](https://docs.rs/bevy_ui_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_render/ui_texture_slice_pipeline.rs.html#647)

### impl<P, const I: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\> [RenderCommand](trait.RenderCommand.html "trait bevy::render::render_phase::RenderCommand")<P> for [SetSlicerViewBindGroup](../../ui_render/ui_texture_slice_pipeline/struct.SetSlicerViewBindGroup.html "struct bevy::ui_render::ui_texture_slice_pipeline::SetSlicerViewBindGroup")<I>

where P: [PhaseItem](trait.PhaseItem.html "trait bevy::render::render_phase::PhaseItem"),

[Source](https://docs.rs/bevy_ui_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_render/ui_texture_slice_pipeline.rs.html#648)

#### type [Param](#associatedtype.Param) = [Res](../../prelude/struct.Res.html "struct bevy::prelude::Res")<'static, [UiTextureSliceMeta](../../ui_render/ui_texture_slice_pipeline/struct.UiTextureSliceMeta.html "struct bevy::ui_render::ui_texture_slice_pipeline::UiTextureSliceMeta")\>

[Source](https://docs.rs/bevy_ui_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_render/ui_texture_slice_pipeline.rs.html#649)

#### type [ViewQuery](#associatedtype.ViewQuery) = &'static [ViewUniformOffset](../view/struct.ViewUniformOffset.html "struct bevy::render::view::ViewUniformOffset")

[Source](https://docs.rs/bevy_ui_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_render/ui_texture_slice_pipeline.rs.html#650)

#### type [ItemQuery](#associatedtype.ItemQuery) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/render/mod.rs.html#909)

### impl<P, const I: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\> [RenderCommand](trait.RenderCommand.html "trait bevy::render::render_phase::RenderCommand")<P> for [SetSpriteTextureBindGroup](../../sprite_render/struct.SetSpriteTextureBindGroup.html "struct bevy::sprite_render::SetSpriteTextureBindGroup")<I>

where P: [PhaseItem](trait.PhaseItem.html "trait bevy::render::render_phase::PhaseItem"),

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/render/mod.rs.html#910)

#### type [Param](#associatedtype.Param) = ([Res](../../prelude/struct.Res.html "struct bevy::prelude::Res")<'static, [ImageBindGroups](../../sprite_render/struct.ImageBindGroups.html "struct bevy::sprite_render::ImageBindGroups")\>, [Res](../../prelude/struct.Res.html "struct bevy::prelude::Res")<'static, [SpriteBatches](../../sprite_render/struct.SpriteBatches.html "struct bevy::sprite_render::SpriteBatches")\>)

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/render/mod.rs.html#911)

#### type [ViewQuery](#associatedtype.ViewQuery) = &'static [ExtractedView](../view/struct.ExtractedView.html "struct bevy::render::view::ExtractedView")

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/render/mod.rs.html#912)

#### type [ItemQuery](#associatedtype.ItemQuery) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/render/mod.rs.html#892)

### impl<P, const I: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\> [RenderCommand](trait.RenderCommand.html "trait bevy::render::render_phase::RenderCommand")<P> for [SetSpriteViewBindGroup](../../sprite_render/struct.SetSpriteViewBindGroup.html "struct bevy::sprite_render::SetSpriteViewBindGroup")<I>

where P: [PhaseItem](trait.PhaseItem.html "trait bevy::render::render_phase::PhaseItem"),

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/render/mod.rs.html#893)

#### type [Param](#associatedtype.Param) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/render/mod.rs.html#894)

#### type [ViewQuery](#associatedtype.ViewQuery) = (&'static [ViewUniformOffset](../view/struct.ViewUniformOffset.html "struct bevy::render::view::ViewUniformOffset"), &'static [SpriteViewBindGroup](../../sprite_render/struct.SpriteViewBindGroup.html "struct bevy::sprite_render::SpriteViewBindGroup"))

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/render/mod.rs.html#895)

#### type [ItemQuery](#associatedtype.ItemQuery) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_ui_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_render/render_pass.rs.html#185)

### impl<P, const I: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\> [RenderCommand](trait.RenderCommand.html "trait bevy::render::render_phase::RenderCommand")<P> for [SetUiTextureBindGroup](../../ui_render/struct.SetUiTextureBindGroup.html "struct bevy::ui_render::SetUiTextureBindGroup")<I>

where P: [PhaseItem](trait.PhaseItem.html "trait bevy::render::render_phase::PhaseItem"),

[Source](https://docs.rs/bevy_ui_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_render/render_pass.rs.html#186)

#### type [Param](#associatedtype.Param) = [Res](../../prelude/struct.Res.html "struct bevy::prelude::Res")<'static, [ImageNodeBindGroups](../../ui_render/struct.ImageNodeBindGroups.html "struct bevy::ui_render::ImageNodeBindGroups")\>

[Source](https://docs.rs/bevy_ui_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_render/render_pass.rs.html#187)

#### type [ViewQuery](#associatedtype.ViewQuery) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_ui_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_render/render_pass.rs.html#188)

#### type [ItemQuery](#associatedtype.ItemQuery) = &'static [UiBatch](../../ui_render/struct.UiBatch.html "struct bevy::ui_render::UiBatch")

[Source](https://docs.rs/bevy_ui_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_render/render_pass.rs.html#165)

### impl<P, const I: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\> [RenderCommand](trait.RenderCommand.html "trait bevy::render::render_phase::RenderCommand")<P> for [SetUiViewBindGroup](../../ui_render/struct.SetUiViewBindGroup.html "struct bevy::ui_render::SetUiViewBindGroup")<I>

where P: [PhaseItem](trait.PhaseItem.html "trait bevy::render::render_phase::PhaseItem"),

[Source](https://docs.rs/bevy_ui_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_render/render_pass.rs.html#166)

#### type [Param](#associatedtype.Param) = [Res](../../prelude/struct.Res.html "struct bevy::prelude::Res")<'static, [UiMeta](../../ui_render/struct.UiMeta.html "struct bevy::ui_render::UiMeta")\>

[Source](https://docs.rs/bevy_ui_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_render/render_pass.rs.html#167)

#### type [ViewQuery](#associatedtype.ViewQuery) = &'static [ViewUniformOffset](../view/struct.ViewUniformOffset.html "struct bevy::render::view::ViewUniformOffset")

[Source](https://docs.rs/bevy_ui_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_render/render_pass.rs.html#168)

#### type [ItemQuery](#associatedtype.ItemQuery) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_ui_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_render/box_shadow.rs.html#528)

### impl<P> [RenderCommand](trait.RenderCommand.html "trait bevy::render::render_phase::RenderCommand")<P> for [DrawBoxShadow](../../ui_render/box_shadow/struct.DrawBoxShadow.html "struct bevy::ui_render::box_shadow::DrawBoxShadow")

where P: [PhaseItem](trait.PhaseItem.html "trait bevy::render::render_phase::PhaseItem"),

[Source](https://docs.rs/bevy_ui_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_render/box_shadow.rs.html#529)

#### type [Param](#associatedtype.Param) = [Res](../../prelude/struct.Res.html "struct bevy::prelude::Res")<'static, [BoxShadowMeta](../../ui_render/box_shadow/struct.BoxShadowMeta.html "struct bevy::ui_render::box_shadow::BoxShadowMeta")\>

[Source](https://docs.rs/bevy_ui_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_render/box_shadow.rs.html#530)

#### type [ViewQuery](#associatedtype.ViewQuery) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_ui_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_render/box_shadow.rs.html#531)

#### type [ItemQuery](#associatedtype.ItemQuery) = &'static [UiShadowsBatch](../../ui_render/box_shadow/struct.UiShadowsBatch.html "struct bevy::ui_render::box_shadow::UiShadowsBatch")

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/mesh2d/mesh.rs.html#863)

### impl<P> [RenderCommand](trait.RenderCommand.html "trait bevy::render::render_phase::RenderCommand")<P> for [DrawMesh2d](../../sprite_render/struct.DrawMesh2d.html "struct bevy::sprite_render::DrawMesh2d")

where P: [PhaseItem](trait.PhaseItem.html "trait bevy::render::render_phase::PhaseItem"),

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/mesh2d/mesh.rs.html#864)

#### type [Param](#associatedtype.Param) = ([Res](../../prelude/struct.Res.html "struct bevy::prelude::Res")<'static, [RenderAssets](../render_asset/struct.RenderAssets.html "struct bevy::render::render_asset::RenderAssets")<[RenderMesh](../mesh/struct.RenderMesh.html "struct bevy::render::mesh::RenderMesh")\>>, [Res](../../prelude/struct.Res.html "struct bevy::prelude::Res")<'static, [RenderMesh2dInstances](../../sprite_render/struct.RenderMesh2dInstances.html "struct bevy::sprite_render::RenderMesh2dInstances")\>, [Res](../../prelude/struct.Res.html "struct bevy::prelude::Res")<'static, [MeshAllocator](../mesh/allocator/struct.MeshAllocator.html "struct bevy::render::mesh::allocator::MeshAllocator")\>)

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/mesh2d/mesh.rs.html#869)

#### type [ViewQuery](#associatedtype.ViewQuery) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/mesh2d/mesh.rs.html#870)

#### type [ItemQuery](#associatedtype.ItemQuery) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#4406)

### impl<P> [RenderCommand](trait.RenderCommand.html "trait bevy::render::render_phase::RenderCommand")<P> for [DrawMesh](../../pbr/struct.DrawMesh.html "struct bevy::pbr::DrawMesh")

where P: [PhaseItem](trait.PhaseItem.html "trait bevy::render::render_phase::PhaseItem"),

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#4407)

#### type [Param](#associatedtype.Param) = ([Res](../../prelude/struct.Res.html "struct bevy::prelude::Res")<'static, [RenderAssets](../render_asset/struct.RenderAssets.html "struct bevy::render::render_asset::RenderAssets")<[RenderMesh](../mesh/struct.RenderMesh.html "struct bevy::render::mesh::RenderMesh")\>>, [Res](../../prelude/struct.Res.html "struct bevy::prelude::Res")<'static, [RenderMeshInstances](../../pbr/enum.RenderMeshInstances.html "enum bevy::pbr::RenderMeshInstances")\>, [Res](../../prelude/struct.Res.html "struct bevy::prelude::Res")<'static, [IndirectParametersBuffers](../batching/gpu_preprocessing/struct.IndirectParametersBuffers.html "struct bevy::render::batching::gpu_preprocessing::IndirectParametersBuffers")\>, [Res](../../prelude/struct.Res.html "struct bevy::prelude::Res")<'static, [PipelineCache](../render_resource/struct.PipelineCache.html "struct bevy::render::render_resource::PipelineCache")\>, [Res](../../prelude/struct.Res.html "struct bevy::prelude::Res")<'static, [MeshAllocator](../mesh/allocator/struct.MeshAllocator.html "struct bevy::render::mesh::allocator::MeshAllocator")\>, [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Res](../../prelude/struct.Res.html "struct bevy::prelude::Res")<'static, [PreprocessPipelines](../../pbr/struct.PreprocessPipelines.html "struct bevy::pbr::PreprocessPipelines")\>>, [Res](../../prelude/struct.Res.html "struct bevy::prelude::Res")<'static, [GpuPreprocessingSupport](../batching/gpu_preprocessing/struct.GpuPreprocessingSupport.html "struct bevy::render::batching::gpu_preprocessing::GpuPreprocessingSupport")\>)

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#4416)

#### type [ViewQuery](#associatedtype.ViewQuery) = [Has](../../prelude/struct.Has.html "struct bevy::prelude::Has")<[PreprocessBindGroups](../../pbr/struct.PreprocessBindGroups.html "struct bevy::pbr::PreprocessBindGroups")\>

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#4417)

#### type [ItemQuery](#associatedtype.ItemQuery) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_ui_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_render/ui_texture_slice_pipeline.rs.html#690)

### impl<P> [RenderCommand](trait.RenderCommand.html "trait bevy::render::render_phase::RenderCommand")<P> for [DrawSlicer](../../ui_render/ui_texture_slice_pipeline/struct.DrawSlicer.html "struct bevy::ui_render::ui_texture_slice_pipeline::DrawSlicer")

where P: [PhaseItem](trait.PhaseItem.html "trait bevy::render::render_phase::PhaseItem"),

[Source](https://docs.rs/bevy_ui_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_render/ui_texture_slice_pipeline.rs.html#691)

#### type [Param](#associatedtype.Param) = [Res](../../prelude/struct.Res.html "struct bevy::prelude::Res")<'static, [UiTextureSliceMeta](../../ui_render/ui_texture_slice_pipeline/struct.UiTextureSliceMeta.html "struct bevy::ui_render::ui_texture_slice_pipeline::UiTextureSliceMeta")\>

[Source](https://docs.rs/bevy_ui_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_render/ui_texture_slice_pipeline.rs.html#692)

#### type [ViewQuery](#associatedtype.ViewQuery) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_ui_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_render/ui_texture_slice_pipeline.rs.html#693)

#### type [ItemQuery](#associatedtype.ItemQuery) = &'static [UiTextureSlicerBatch](../../ui_render/ui_texture_slice_pipeline/struct.UiTextureSlicerBatch.html "struct bevy::ui_render::ui_texture_slice_pipeline::UiTextureSlicerBatch")

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/render/mod.rs.html#939)

### impl<P> [RenderCommand](trait.RenderCommand.html "trait bevy::render::render_phase::RenderCommand")<P> for [DrawSpriteBatch](../../sprite_render/struct.DrawSpriteBatch.html "struct bevy::sprite_render::DrawSpriteBatch")

where P: [PhaseItem](trait.PhaseItem.html "trait bevy::render::render_phase::PhaseItem"),

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/render/mod.rs.html#940)

#### type [Param](#associatedtype.Param) = ([Res](../../prelude/struct.Res.html "struct bevy::prelude::Res")<'static, [SpriteMeta](../../sprite_render/struct.SpriteMeta.html "struct bevy::sprite_render::SpriteMeta")\>, [Res](../../prelude/struct.Res.html "struct bevy::prelude::Res")<'static, [SpriteBatches](../../sprite_render/struct.SpriteBatches.html "struct bevy::sprite_render::SpriteBatches")\>)

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/render/mod.rs.html#941)

#### type [ViewQuery](#associatedtype.ViewQuery) = &'static [ExtractedView](../view/struct.ExtractedView.html "struct bevy::render::view::ExtractedView")

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/render/mod.rs.html#942)

#### type [ItemQuery](#associatedtype.ItemQuery) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_ui_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_render/render_pass.rs.html#209)

### impl<P> [RenderCommand](trait.RenderCommand.html "trait bevy::render::render_phase::RenderCommand")<P> for [DrawUiNode](../../ui_render/struct.DrawUiNode.html "struct bevy::ui_render::DrawUiNode")

where P: [PhaseItem](trait.PhaseItem.html "trait bevy::render::render_phase::PhaseItem"),

[Source](https://docs.rs/bevy_ui_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_render/render_pass.rs.html#210)

#### type [Param](#associatedtype.Param) = [Res](../../prelude/struct.Res.html "struct bevy::prelude::Res")<'static, [UiMeta](../../ui_render/struct.UiMeta.html "struct bevy::ui_render::UiMeta")\>

[Source](https://docs.rs/bevy_ui_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_render/render_pass.rs.html#211)

#### type [ViewQuery](#associatedtype.ViewQuery) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_ui_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_render/render_pass.rs.html#212)

#### type [ItemQuery](#associatedtype.ItemQuery) = &'static [UiBatch](../../ui_render/struct.UiBatch.html "struct bevy::ui_render::UiBatch")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/wireframe.rs.html#558)

### impl<P> [RenderCommand](trait.RenderCommand.html "trait bevy::render::render_phase::RenderCommand")<P> for [DrawWireframeMeshPulled](../../pbr/wireframe/struct.DrawWireframeMeshPulled.html "struct bevy::pbr::wireframe::DrawWireframeMeshPulled")

where P: [PhaseItem](trait.PhaseItem.html "trait bevy::render::render_phase::PhaseItem"),

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/wireframe.rs.html#559)

#### type [Param](#associatedtype.Param) = ([Res](../../prelude/struct.Res.html "struct bevy::prelude::Res")<'static, [RenderMeshInstances](../../pbr/enum.RenderMeshInstances.html "enum bevy::pbr::RenderMeshInstances")\>, [Res](../../prelude/struct.Res.html "struct bevy::prelude::Res")<'static, [RenderAssets](../render_asset/struct.RenderAssets.html "struct bevy::render::render_asset::RenderAssets")<[RenderMesh](../mesh/struct.RenderMesh.html "struct bevy::render::mesh::RenderMesh")\>>, [Res](../../prelude/struct.Res.html "struct bevy::prelude::Res")<'static, [MeshAllocator](../mesh/allocator/struct.MeshAllocator.html "struct bevy::render::mesh::allocator::MeshAllocator")\>, [Res](../../prelude/struct.Res.html "struct bevy::prelude::Res")<'static, [IndirectParametersBuffers](../batching/gpu_preprocessing/struct.IndirectParametersBuffers.html "struct bevy::render::batching::gpu_preprocessing::IndirectParametersBuffers")\>, [Res](../../prelude/struct.Res.html "struct bevy::prelude::Res")<'static, [PipelineCache](../render_resource/struct.PipelineCache.html "struct bevy::render::render_resource::PipelineCache")\>, [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Res](../../prelude/struct.Res.html "struct bevy::prelude::Res")<'static, [PreprocessPipelines](../../pbr/struct.PreprocessPipelines.html "struct bevy::pbr::PreprocessPipelines")\>>, [Res](../../prelude/struct.Res.html "struct bevy::prelude::Res")<'static, [GpuPreprocessingSupport](../batching/gpu_preprocessing/struct.GpuPreprocessingSupport.html "struct bevy::render::batching::gpu_preprocessing::GpuPreprocessingSupport")\>)

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/wireframe.rs.html#568)

#### type [ViewQuery](#associatedtype.ViewQuery) = [Has](../../prelude/struct.Has.html "struct bevy::prelude::Has")<[PreprocessBindGroups](../../pbr/struct.PreprocessBindGroups.html "struct bevy::pbr::PreprocessBindGroups")\>

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/wireframe.rs.html#569)

#### type [ItemQuery](#associatedtype.ItemQuery) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_phase/mod.rs.html#2148)

### impl<P> [RenderCommand](trait.RenderCommand.html "trait bevy::render::render_phase::RenderCommand")<P> for [SetItemPipeline](struct.SetItemPipeline.html "struct bevy::render::render_phase::SetItemPipeline")

where P: [CachedRenderPipelinePhaseItem](trait.CachedRenderPipelinePhaseItem.html "trait bevy::render::render_phase::CachedRenderPipelinePhaseItem"),

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_phase/mod.rs.html#2149)

#### type [Param](#associatedtype.Param) = [Res](../../prelude/struct.Res.html "struct bevy::prelude::Res")<'static, [PipelineCache](../render_resource/struct.PipelineCache.html "struct bevy::render::render_resource::PipelineCache")\>

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_phase/mod.rs.html#2150)

#### type [ViewQuery](#associatedtype.ViewQuery) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_phase/mod.rs.html#2151)

#### type [ItemQuery](#associatedtype.ItemQuery) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/mesh2d/wireframe2d.rs.html#282)

### impl<P> [RenderCommand](trait.RenderCommand.html "trait bevy::render::render_phase::RenderCommand")<P> for [SetWireframe2dImmediates](../../sprite_render/struct.SetWireframe2dImmediates.html "struct bevy::sprite_render::SetWireframe2dImmediates")

where P: [PhaseItem](trait.PhaseItem.html "trait bevy::render::render_phase::PhaseItem"),

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/mesh2d/wireframe2d.rs.html#283)

#### type [Param](#associatedtype.Param) = ([Res](../../prelude/struct.Res.html "struct bevy::prelude::Res")<'static, [RenderWireframeInstances](../../sprite_render/struct.RenderWireframeInstances.html "struct bevy::sprite_render::RenderWireframeInstances")\>, [Res](../../prelude/struct.Res.html "struct bevy::prelude::Res")<'static, [RenderAssets](../render_asset/struct.RenderAssets.html "struct bevy::render::render_asset::RenderAssets")<[RenderWireframeMaterial](../../sprite_render/struct.RenderWireframeMaterial.html "struct bevy::sprite_render::RenderWireframeMaterial")\>>)

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/mesh2d/wireframe2d.rs.html#287)

#### type [ViewQuery](#associatedtype.ViewQuery) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/mesh2d/wireframe2d.rs.html#288)

#### type [ItemQuery](#associatedtype.ItemQuery) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/wireframe.rs.html#320)

### impl<P> [RenderCommand](trait.RenderCommand.html "trait bevy::render::render_phase::RenderCommand")<P> for [SetWireframe3dThinImmediates](../../pbr/wireframe/struct.SetWireframe3dThinImmediates.html "struct bevy::pbr::wireframe::SetWireframe3dThinImmediates")

where P: [PhaseItem](trait.PhaseItem.html "trait bevy::render::render_phase::PhaseItem"),

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/wireframe.rs.html#321)

#### type [Param](#associatedtype.Param) = ([Res](../../prelude/struct.Res.html "struct bevy::prelude::Res")<'static, [RenderWireframeInstances](../../pbr/wireframe/struct.RenderWireframeInstances.html "struct bevy::pbr::wireframe::RenderWireframeInstances")\>, [Res](../../prelude/struct.Res.html "struct bevy::prelude::Res")<'static, [RenderAssets](../render_asset/struct.RenderAssets.html "struct bevy::render::render_asset::RenderAssets")<[RenderWireframeMaterial](../../pbr/wireframe/struct.RenderWireframeMaterial.html "struct bevy::pbr::wireframe::RenderWireframeMaterial")\>>)

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/wireframe.rs.html#325)

#### type [ViewQuery](#associatedtype.ViewQuery) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/wireframe.rs.html#326)

#### type [ItemQuery](#associatedtype.ItemQuery) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/wireframe.rs.html#526)

### impl<P> [RenderCommand](trait.RenderCommand.html "trait bevy::render::render_phase::RenderCommand")<P> for [SetWireframe3dWideBindGroup](../../pbr/wireframe/struct.SetWireframe3dWideBindGroup.html "struct bevy::pbr::wireframe::SetWireframe3dWideBindGroup")

where P: [PhaseItem](trait.PhaseItem.html "trait bevy::render::render_phase::PhaseItem"),

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/wireframe.rs.html#527)

#### type [Param](#associatedtype.Param) = ([Res](../../prelude/struct.Res.html "struct bevy::prelude::Res")<'static, [RenderMeshInstances](../../pbr/enum.RenderMeshInstances.html "enum bevy::pbr::RenderMeshInstances")\>, [Res](../../prelude/struct.Res.html "struct bevy::prelude::Res")<'static, [WireframeWideBindGroups](../../pbr/wireframe/struct.WireframeWideBindGroups.html "struct bevy::pbr::wireframe::WireframeWideBindGroups")\>)

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/wireframe.rs.html#528)

#### type [ViewQuery](#associatedtype.ViewQuery) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/wireframe.rs.html#529)

#### type [ItemQuery](#associatedtype.ItemQuery) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/wireframe.rs.html#360)

### impl<P> [RenderCommand](trait.RenderCommand.html "trait bevy::render::render_phase::RenderCommand")<P> for [SetWireframe3dWideImmediates](../../pbr/wireframe/struct.SetWireframe3dWideImmediates.html "struct bevy::pbr::wireframe::SetWireframe3dWideImmediates")

where P: [PhaseItem](trait.PhaseItem.html "trait bevy::render::render_phase::PhaseItem"),

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/wireframe.rs.html#361)

#### type [Param](#associatedtype.Param) = ([Res](../../prelude/struct.Res.html "struct bevy::prelude::Res")<'static, [RenderWireframeInstances](../../pbr/wireframe/struct.RenderWireframeInstances.html "struct bevy::pbr::wireframe::RenderWireframeInstances")\>, [Res](../../prelude/struct.Res.html "struct bevy::prelude::Res")<'static, [RenderAssets](../render_asset/struct.RenderAssets.html "struct bevy::render::render_asset::RenderAssets")<[RenderWireframeMaterial](../../pbr/wireframe/struct.RenderWireframeMaterial.html "struct bevy::pbr::wireframe::RenderWireframeMaterial")\>>)

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/wireframe.rs.html#365)

#### type [ViewQuery](#associatedtype.ViewQuery) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/wireframe.rs.html#366)

#### type [ItemQuery](#associatedtype.ItemQuery) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)