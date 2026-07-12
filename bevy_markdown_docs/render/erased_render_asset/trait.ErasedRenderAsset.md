[bevy](../../index.html)::[render](../index.html)::[erased\_render\_asset](index.html)

# Trait ErasedRenderAsset 

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/erased_render_asset.rs.html#39)

```rust
pub trait ErasedRenderAsset:
    Send
    + Sync
    + 'static {
    type SourceAsset: Asset + Clone;
    type ErasedAsset: Send + Sync + 'static;
    type Param: SystemParam;

    // Required method
    fn prepare_asset(
        source_asset: Self::SourceAsset,
        asset_id: AssetId<Self::SourceAsset>,
        param: &mut <Self::Param as SystemParam>::Item<'_, '_>,
    ) -> Result<Self::ErasedAsset, PrepareAssetError<Self::SourceAsset>>;

    // Provided methods
    fn asset_usage(_source_asset: &Self::SourceAsset) -> RenderAssetUsages { ... }
    fn byte_len(erased_asset: &Self::SourceAsset) -> Option<usize> { ... }
    fn unload_asset(
        _source_asset: AssetId<Self::SourceAsset>,
        _param: &mut <Self::Param as SystemParam>::Item<'_, '_>,
    ) { ... }
}
```

Describes how an asset gets extracted and prepared for rendering.

In the [`ExtractSchedule`](../../prelude/struct.ExtractSchedule.html "struct bevy::prelude::ExtractSchedule") step the [`ErasedRenderAsset::SourceAsset`](trait.ErasedRenderAsset.html#associatedtype.SourceAsset "associated type bevy::render::erased_render_asset::ErasedRenderAsset::SourceAsset") is transferred from the “main world” into the “render world”.

After that in the [`RenderSystems::PrepareAssets`](../enum.RenderSystems.html#variant.PrepareAssets "variant bevy::render::RenderSystems::PrepareAssets") step the extracted asset is transformed into its GPU-representation of type [`ErasedRenderAsset`](trait.ErasedRenderAsset.html "trait bevy::render::erased_render_asset::ErasedRenderAsset").

## Required Associated Types

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/erased_render_asset.rs.html#41)

#### type [SourceAsset](#associatedtype.SourceAsset): [Asset](../../prelude/trait.Asset.html "trait bevy::prelude::Asset") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone")

The representation of the asset in the “main world”.

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/erased_render_asset.rs.html#43)

#### type [ErasedAsset](#associatedtype.ErasedAsset): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static

The target representation of the asset in the “render world”.

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/erased_render_asset.rs.html#48)

#### type [Param](#associatedtype.Param): [SystemParam](../../ecs/system/trait.SystemParam.html "trait bevy::ecs::system::SystemParam")

Specifies all ECS data required by [`ErasedRenderAsset::prepare_asset`](trait.ErasedRenderAsset.html#tymethod.prepare_asset "associated function bevy::render::erased_render_asset::ErasedRenderAsset::prepare_asset").

For convenience use the [`lifetimeless`](../../ecs/system/lifetimeless/index.html "mod bevy::ecs::system::lifetimeless") [`SystemParam`](../../ecs/system/trait.SystemParam.html "trait bevy::ecs::system::SystemParam").

## Required Methods

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/erased_render_asset.rs.html#70-74)

#### fn [prepare\_asset](#tymethod.prepare_asset)( source\_asset: Self::[SourceAsset](trait.ErasedRenderAsset.html#associatedtype.SourceAsset "type bevy::render::erased_render_asset::ErasedRenderAsset::SourceAsset"), asset\_id: [AssetId](../../prelude/enum.AssetId.html "enum bevy::prelude::AssetId")<Self::[SourceAsset](trait.ErasedRenderAsset.html#associatedtype.SourceAsset "type bevy::render::erased_render_asset::ErasedRenderAsset::SourceAsset")\>, param: &mut <Self::[Param](trait.ErasedRenderAsset.html#associatedtype.Param "type bevy::render::erased_render_asset::ErasedRenderAsset::Param") as [SystemParam](../../ecs/system/trait.SystemParam.html "trait bevy::ecs::system::SystemParam")\>::[Item](../../ecs/system/trait.SystemParam.html#associatedtype.Item "type bevy::ecs::system::SystemParam::Item")<'\_, '\_>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Self::[ErasedAsset](trait.ErasedRenderAsset.html#associatedtype.ErasedAsset "type bevy::render::erased_render_asset::ErasedRenderAsset::ErasedAsset"), [PrepareAssetError](enum.PrepareAssetError.html "enum bevy::render::erased_render_asset::PrepareAssetError")<Self::[SourceAsset](trait.ErasedRenderAsset.html#associatedtype.SourceAsset "type bevy::render::erased_render_asset::ErasedRenderAsset::SourceAsset")\>>

Prepares the [`ErasedRenderAsset::SourceAsset`](trait.ErasedRenderAsset.html#associatedtype.SourceAsset "associated type bevy::render::erased_render_asset::ErasedRenderAsset::SourceAsset") for the GPU by transforming it into a [`ErasedRenderAsset`](trait.ErasedRenderAsset.html "trait bevy::render::erased_render_asset::ErasedRenderAsset").

ECS data may be accessed via `param`.

## Provided Methods

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/erased_render_asset.rs.html#52)

#### fn [asset\_usage](#method.asset_usage)(\_source\_asset: &Self::[SourceAsset](trait.ErasedRenderAsset.html#associatedtype.SourceAsset "type bevy::render::erased_render_asset::ErasedRenderAsset::SourceAsset")) -> [RenderAssetUsages](../../asset/struct.RenderAssetUsages.html "struct bevy::asset::RenderAssetUsages")

Whether or not to unload the asset after extracting it to the render world.

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/erased_render_asset.rs.html#63)

#### fn [byte\_len](#method.byte_len)(erased\_asset: &Self::[SourceAsset](trait.ErasedRenderAsset.html#associatedtype.SourceAsset "type bevy::render::erased_render_asset::ErasedRenderAsset::SourceAsset")) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>

Size of the data the asset will upload to the gpu. Specifying a return value will allow the asset to be throttled via [`RenderAssetBytesPerFrameLimiter`](../render_asset/struct.RenderAssetBytesPerFrameLimiter.html "struct bevy::render::render_asset::RenderAssetBytesPerFrameLimiter").

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/erased_render_asset.rs.html#82-85)

#### fn [unload\_asset](#method.unload_asset)( \_source\_asset: [AssetId](../../prelude/enum.AssetId.html "enum bevy::prelude::AssetId")<Self::[SourceAsset](trait.ErasedRenderAsset.html#associatedtype.SourceAsset "type bevy::render::erased_render_asset::ErasedRenderAsset::SourceAsset")\>, \_param: &mut <Self::[Param](trait.ErasedRenderAsset.html#associatedtype.Param "type bevy::render::erased_render_asset::ErasedRenderAsset::Param") as [SystemParam](../../ecs/system/trait.SystemParam.html "trait bevy::ecs::system::SystemParam")\>::[Item](../../ecs/system/trait.SystemParam.html#associatedtype.Item "type bevy::ecs::system::SystemParam::Item")<'\_, '\_>, )

Called whenever the [`ErasedRenderAsset::SourceAsset`](trait.ErasedRenderAsset.html#associatedtype.SourceAsset "associated type bevy::render::erased_render_asset::ErasedRenderAsset::SourceAsset") has been removed.

You can implement this method if you need to access ECS data (via `_param`) in order to perform cleanup tasks when the asset is removed.

The default implementation does nothing.

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/material.rs.html#1512-1514)

### impl<M> [ErasedRenderAsset](trait.ErasedRenderAsset.html "trait bevy::render::erased_render_asset::ErasedRenderAsset") for [MeshMaterial3d](../../prelude/struct.MeshMaterial3d.html "struct bevy::prelude::MeshMaterial3d")<M>

where M: [Material](../../prelude/trait.Material.html "trait bevy::prelude::Material"), <M as [AsBindGroup](../render_resource/trait.AsBindGroup.html "trait bevy::render::render_resource::AsBindGroup")\>::[Data](../render_resource/trait.AsBindGroup.html#associatedtype.Data "type bevy::render::render_resource::AsBindGroup::Data"): [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq") + [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/material.rs.html#1516)

#### type [SourceAsset](#associatedtype.SourceAsset) = M

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/material.rs.html#1517)

#### type [ErasedAsset](#associatedtype.ErasedAsset) = [PreparedMaterial](../../pbr/struct.PreparedMaterial.html "struct bevy::pbr::PreparedMaterial")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/material.rs.html#1519)

#### type [Param](#associatedtype.Param) = ([Res](../../prelude/struct.Res.html "struct bevy::prelude::Res")<'static, [RenderDevice](../renderer/struct.RenderDevice.html "struct bevy::render::renderer::RenderDevice")\>, [Res](../../prelude/struct.Res.html "struct bevy::prelude::Res")<'static, [PipelineCache](../render_resource/struct.PipelineCache.html "struct bevy::render::render_resource::PipelineCache")\>, [Res](../../prelude/struct.Res.html "struct bevy::prelude::Res")<'static, [DefaultOpaqueRendererMethod](../../pbr/struct.DefaultOpaqueRendererMethod.html "struct bevy::pbr::DefaultOpaqueRendererMethod")\>, [ResMut](../../prelude/struct.ResMut.html "struct bevy::prelude::ResMut")<'static, [MaterialBindGroupAllocators](../../pbr/struct.MaterialBindGroupAllocators.html "struct bevy::pbr::MaterialBindGroupAllocators")\>, [ResMut](../../prelude/struct.ResMut.html "struct bevy::prelude::ResMut")<'static, [RenderMaterialBindings](../../pbr/struct.RenderMaterialBindings.html "struct bevy::pbr::RenderMaterialBindings")\>, [Res](../../prelude/struct.Res.html "struct bevy::prelude::Res")<'static, [DrawFunctions](../render_phase/struct.DrawFunctions.html "struct bevy::render::render_phase::DrawFunctions")<[Opaque3d](../../core_pipeline/core_3d/struct.Opaque3d.html "struct bevy::core_pipeline::core_3d::Opaque3d")\>>, [Res](../../prelude/struct.Res.html "struct bevy::prelude::Res")<'static, [DrawFunctions](../render_phase/struct.DrawFunctions.html "struct bevy::render::render_phase::DrawFunctions")<[AlphaMask3d](../../core_pipeline/core_3d/struct.AlphaMask3d.html "struct bevy::core_pipeline::core_3d::AlphaMask3d")\>>, [Res](../../prelude/struct.Res.html "struct bevy::prelude::Res")<'static, [DrawFunctions](../render_phase/struct.DrawFunctions.html "struct bevy::render::render_phase::DrawFunctions")<[Transmissive3d](../../pbr/struct.Transmissive3d.html "struct bevy::pbr::Transmissive3d")\>>, [Res](../../prelude/struct.Res.html "struct bevy::prelude::Res")<'static, [DrawFunctions](../render_phase/struct.DrawFunctions.html "struct bevy::render::render_phase::DrawFunctions")<[Transparent3d](../../core_pipeline/core_3d/struct.Transparent3d.html "struct bevy::core_pipeline::core_3d::Transparent3d")\>>, [Res](../../prelude/struct.Res.html "struct bevy::prelude::Res")<'static, [DrawFunctions](../render_phase/struct.DrawFunctions.html "struct bevy::render::render_phase::DrawFunctions")<[Opaque3dPrepass](../../core_pipeline/prepass/struct.Opaque3dPrepass.html "struct bevy::core_pipeline::prepass::Opaque3dPrepass")\>>, [Res](../../prelude/struct.Res.html "struct bevy::prelude::Res")<'static, [DrawFunctions](../render_phase/struct.DrawFunctions.html "struct bevy::render::render_phase::DrawFunctions")<[AlphaMask3dPrepass](../../core_pipeline/prepass/struct.AlphaMask3dPrepass.html "struct bevy::core_pipeline::prepass::AlphaMask3dPrepass")\>>, [Res](../../prelude/struct.Res.html "struct bevy::prelude::Res")<'static, [DrawFunctions](../render_phase/struct.DrawFunctions.html "struct bevy::render::render_phase::DrawFunctions")<[Opaque3dDeferred](../../core_pipeline/deferred/struct.Opaque3dDeferred.html "struct bevy::core_pipeline::deferred::Opaque3dDeferred")\>>, [Res](../../prelude/struct.Res.html "struct bevy::prelude::Res")<'static, [DrawFunctions](../render_phase/struct.DrawFunctions.html "struct bevy::render::render_phase::DrawFunctions")<[AlphaMask3dDeferred](../../core_pipeline/deferred/struct.AlphaMask3dDeferred.html "struct bevy::core_pipeline::deferred::AlphaMask3dDeferred")\>>, [Res](../../prelude/struct.Res.html "struct bevy::prelude::Res")<'static, [DrawFunctions](../render_phase/struct.DrawFunctions.html "struct bevy::render::render_phase::DrawFunctions")<[Shadow](../../pbr/struct.Shadow.html "struct bevy::pbr::Shadow")\>>, [Res](../../prelude/struct.Res.html "struct bevy::prelude::Res")<'static, [AssetServer](../../prelude/struct.AssetServer.html "struct bevy::prelude::AssetServer")\>, <M as [AsBindGroup](../render_resource/trait.AsBindGroup.html "trait bevy::render::render_resource::AsBindGroup")\>::[Param](../render_resource/trait.AsBindGroup.html#associatedtype.Param "type bevy::render::render_resource::AsBindGroup::Param"))