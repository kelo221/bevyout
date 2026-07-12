[bevy](../../index.html)::[render](../index.html)::[render\_asset](index.html)

# Trait RenderAsset 

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_asset.rs.html#47)

```rust
pub trait RenderAsset:
    Sized
    + Send
    + Sync
    + 'static {
    type SourceAsset: Asset + Clone;
    type Param: SystemParam;

    // Required method
    fn prepare_asset(
        source_asset: Self::SourceAsset,
        asset_id: AssetId<Self::SourceAsset>,
        param: &mut <Self::Param as SystemParam>::Item<'_, '_>,
        previous_asset: Option<&Self>,
    ) -> Result<Self, PrepareAssetError<Self::SourceAsset>>;

    // Provided methods
    fn asset_usage(_source_asset: &Self::SourceAsset) -> RenderAssetUsages { ... }
    fn byte_len(source_asset: &Self::SourceAsset) -> Option<usize> { ... }
    fn unload_asset(
        _source_asset: AssetId<Self::SourceAsset>,
        _param: &mut <Self::Param as SystemParam>::Item<'_, '_>,
    ) { ... }
    fn take_gpu_data(
        _source: &mut Self::SourceAsset,
        _previous_gpu_asset: Option<&Self>,
    ) -> Result<Self::SourceAsset, AssetExtractionError> { ... }
}
```

Describes how an asset gets extracted and prepared for rendering.

In the [`ExtractSchedule`](../../prelude/struct.ExtractSchedule.html "struct bevy::prelude::ExtractSchedule") step the [`RenderAsset::SourceAsset`](trait.RenderAsset.html#associatedtype.SourceAsset "associated type bevy::render::render_asset::RenderAsset::SourceAsset") is transferred from the “main world” into the “render world”.

After that in the [`RenderSystems::PrepareAssets`](../enum.RenderSystems.html#variant.PrepareAssets "variant bevy::render::RenderSystems::PrepareAssets") step the extracted asset is transformed into its GPU-representation of type [`RenderAsset`](trait.RenderAsset.html "trait bevy::render::render_asset::RenderAsset").

## Required Associated Types

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_asset.rs.html#49)

#### type [SourceAsset](#associatedtype.SourceAsset): [Asset](../../prelude/trait.Asset.html "trait bevy::prelude::Asset") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone")

The representation of the asset in the “main world”.

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_asset.rs.html#54)

#### type [Param](#associatedtype.Param): [SystemParam](../../ecs/system/trait.SystemParam.html "trait bevy::ecs::system::SystemParam")

Specifies all ECS data required by [`RenderAsset::prepare_asset`](trait.RenderAsset.html#tymethod.prepare_asset "associated function bevy::render::render_asset::RenderAsset::prepare_asset").

For convenience use the [`lifetimeless`](../../ecs/system/lifetimeless/index.html "mod bevy::ecs::system::lifetimeless") [`SystemParam`](../../ecs/system/trait.SystemParam.html "trait bevy::ecs::system::SystemParam").

## Required Methods

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_asset.rs.html#76-81)

#### fn [prepare\_asset](#tymethod.prepare_asset)( source\_asset: Self::[SourceAsset](trait.RenderAsset.html#associatedtype.SourceAsset "type bevy::render::render_asset::RenderAsset::SourceAsset"), asset\_id: [AssetId](../../prelude/enum.AssetId.html "enum bevy::prelude::AssetId")<Self::[SourceAsset](trait.RenderAsset.html#associatedtype.SourceAsset "type bevy::render::render_asset::RenderAsset::SourceAsset")\>, param: &mut <Self::[Param](trait.RenderAsset.html#associatedtype.Param "type bevy::render::render_asset::RenderAsset::Param") as [SystemParam](../../ecs/system/trait.SystemParam.html "trait bevy::ecs::system::SystemParam")\>::[Item](../../ecs/system/trait.SystemParam.html#associatedtype.Item "type bevy::ecs::system::SystemParam::Item")<'\_, '\_>, previous\_asset: [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&Self>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Self, [PrepareAssetError](enum.PrepareAssetError.html "enum bevy::render::render_asset::PrepareAssetError")<Self::[SourceAsset](trait.RenderAsset.html#associatedtype.SourceAsset "type bevy::render::render_asset::RenderAsset::SourceAsset")\>>

Prepares the [`RenderAsset::SourceAsset`](trait.RenderAsset.html#associatedtype.SourceAsset "associated type bevy::render::render_asset::RenderAsset::SourceAsset") for the GPU by transforming it into a [`RenderAsset`](trait.RenderAsset.html "trait bevy::render::render_asset::RenderAsset").

ECS data may be accessed via `param`.

## Provided Methods

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_asset.rs.html#58)

#### fn [asset\_usage](#method.asset_usage)(\_source\_asset: &Self::[SourceAsset](trait.RenderAsset.html#associatedtype.SourceAsset "type bevy::render::render_asset::RenderAsset::SourceAsset")) -> [RenderAssetUsages](../../asset/struct.RenderAssetUsages.html "struct bevy::asset::RenderAssetUsages")

Whether or not to unload the asset after extracting it to the render world.

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_asset.rs.html#69)

#### fn [byte\_len](#method.byte_len)(source\_asset: &Self::[SourceAsset](trait.RenderAsset.html#associatedtype.SourceAsset "type bevy::render::render_asset::RenderAsset::SourceAsset")) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>

Size of the data the asset will upload to the gpu. Specifying a return value will allow the asset to be throttled via [`RenderAssetBytesPerFrame`](struct.RenderAssetBytesPerFrame.html "struct bevy::render::render_asset::RenderAssetBytesPerFrame").

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_asset.rs.html#89-92)

#### fn [unload\_asset](#method.unload_asset)( \_source\_asset: [AssetId](../../prelude/enum.AssetId.html "enum bevy::prelude::AssetId")<Self::[SourceAsset](trait.RenderAsset.html#associatedtype.SourceAsset "type bevy::render::render_asset::RenderAsset::SourceAsset")\>, \_param: &mut <Self::[Param](trait.RenderAsset.html#associatedtype.Param "type bevy::render::render_asset::RenderAsset::Param") as [SystemParam](../../ecs/system/trait.SystemParam.html "trait bevy::ecs::system::SystemParam")\>::[Item](../../ecs/system/trait.SystemParam.html#associatedtype.Item "type bevy::ecs::system::SystemParam::Item")<'\_, '\_>, )

Called whenever the [`RenderAsset::SourceAsset`](trait.RenderAsset.html#associatedtype.SourceAsset "associated type bevy::render::render_asset::RenderAsset::SourceAsset") has been removed.

You can implement this method if you need to access ECS data (via `_param`) in order to perform cleanup tasks when the asset is removed.

The default implementation does nothing.

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_asset.rs.html#100-103)

#### fn [take\_gpu\_data](#method.take_gpu_data)( \_source: &mut Self::[SourceAsset](trait.RenderAsset.html#associatedtype.SourceAsset "type bevy::render::render_asset::RenderAsset::SourceAsset"), \_previous\_gpu\_asset: [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&Self>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Self::[SourceAsset](trait.RenderAsset.html#associatedtype.SourceAsset "type bevy::render::render_asset::RenderAsset::SourceAsset"), [AssetExtractionError](enum.AssetExtractionError.html "enum bevy::render::render_asset::AssetExtractionError")\>

Make a copy of the asset to be moved to the `RenderWorld` / gpu. Heavy internal data (pixels, vertex attributes) should be moved into the copy, leaving this asset with only metadata. An error may be returned to indicate that the asset has already been extracted, and should not have been modified on the CPU side (as it cannot be transferred to GPU again). The previous GPU asset is also provided, which can be used to check if the modification is valid.

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/texture/gpu_image.rs.html#26)

### impl [RenderAsset](trait.RenderAsset.html "trait bevy::render::render_asset::RenderAsset") for [GpuImage](../texture/struct.GpuImage.html "struct bevy::render::texture::GpuImage")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/texture/gpu_image.rs.html#27)

#### type [SourceAsset](#associatedtype.SourceAsset) = [Image](../../prelude/struct.Image.html "struct bevy::prelude::Image")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/texture/gpu_image.rs.html#28)

#### type [Param](#associatedtype.Param) = ([Res](../../prelude/struct.Res.html "struct bevy::prelude::Res")<'static, [RenderDevice](../renderer/struct.RenderDevice.html "struct bevy::render::renderer::RenderDevice")\>, [Res](../../prelude/struct.Res.html "struct bevy::prelude::Res")<'static, [RenderQueue](../renderer/struct.RenderQueue.html "struct bevy::render::renderer::RenderQueue")\>, [Res](../../prelude/struct.Res.html "struct bevy::prelude::Res")<'static, [DefaultImageSampler](../render_resource/struct.DefaultImageSampler.html "struct bevy::render::render_resource::DefaultImageSampler")\>)

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/medium.rs.html#64)

### impl [RenderAsset](trait.RenderAsset.html "trait bevy::render::render_asset::RenderAsset") for [GpuScatteringMedium](../../pbr/struct.GpuScatteringMedium.html "struct bevy::pbr::GpuScatteringMedium")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/medium.rs.html#65)

#### type [SourceAsset](#associatedtype.SourceAsset) = [ScatteringMedium](../../light/atmosphere/struct.ScatteringMedium.html "struct bevy::light::atmosphere::ScatteringMedium")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/medium.rs.html#67)

#### type [Param](#associatedtype.Param) = ([Res](../../prelude/struct.Res.html "struct bevy::prelude::Res")<'static, [RenderDevice](../renderer/struct.RenderDevice.html "struct bevy::render::renderer::RenderDevice")\>, [Res](../../prelude/struct.Res.html "struct bevy::prelude::Res")<'static, [RenderQueue](../renderer/struct.RenderQueue.html "struct bevy::render::renderer::RenderQueue")\>)

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/storage.rs.html#135)

### impl [RenderAsset](trait.RenderAsset.html "trait bevy::render::render_asset::RenderAsset") for [GpuShaderBuffer](../storage/struct.GpuShaderBuffer.html "struct bevy::render::storage::GpuShaderBuffer")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/storage.rs.html#136)

#### type [SourceAsset](#associatedtype.SourceAsset) = [ShaderBuffer](../storage/struct.ShaderBuffer.html "struct bevy::render::storage::ShaderBuffer")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/storage.rs.html#137)

#### type [Param](#associatedtype.Param) = ([Res](../../prelude/struct.Res.html "struct bevy::prelude::Res")<'static, [RenderDevice](../renderer/struct.RenderDevice.html "struct bevy::render::renderer::RenderDevice")\>, [Res](../../prelude/struct.Res.html "struct bevy::prelude::Res")<'static, [RenderQueue](../renderer/struct.RenderQueue.html "struct bevy::render::renderer::RenderQueue")\>)

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/mesh/mod.rs.html#120)

### impl [RenderAsset](trait.RenderAsset.html "trait bevy::render::render_asset::RenderAsset") for [RenderMesh](../mesh/struct.RenderMesh.html "struct bevy::render::mesh::RenderMesh")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/mesh/mod.rs.html#121)

#### type [SourceAsset](#associatedtype.SourceAsset) = [Mesh](../../prelude/struct.Mesh.html "struct bevy::prelude::Mesh")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/mesh/mod.rs.html#131)

#### type [Param](#associatedtype.Param) = ([Res](../../prelude/struct.Res.html "struct bevy::prelude::Res")<'static, [RenderDevice](../renderer/struct.RenderDevice.html "struct bevy::render::renderer::RenderDevice")\>, [Res](../../prelude/struct.Res.html "struct bevy::prelude::Res")<'static, [RenderQueue](../renderer/struct.RenderQueue.html "struct bevy::render::renderer::RenderQueue")\>, [ResMut](../../prelude/struct.ResMut.html "struct bevy::prelude::ResMut")<'static, [MeshVertexBufferLayouts](../../mesh/struct.MeshVertexBufferLayouts.html "struct bevy::mesh::MeshVertexBufferLayouts")\>, [ResMut](../../prelude/struct.ResMut.html "struct bevy::prelude::ResMut")<'static, [RenderMorphTargetAllocator](../mesh/morph/enum.RenderMorphTargetAllocator.html "enum bevy::render::mesh::morph::RenderMorphTargetAllocator")\>)

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/wireframe.rs.html#948)

### impl [RenderAsset](trait.RenderAsset.html "trait bevy::render::render_asset::RenderAsset") for bevy::pbr::wireframe::[RenderWireframeMaterial](../../pbr/wireframe/struct.RenderWireframeMaterial.html "struct bevy::pbr::wireframe::RenderWireframeMaterial")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/wireframe.rs.html#949)

#### type [SourceAsset](#associatedtype.SourceAsset) = [WireframeMaterial](../../pbr/wireframe/struct.WireframeMaterial.html "struct bevy::pbr::wireframe::WireframeMaterial")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/wireframe.rs.html#950)

#### type [Param](#associatedtype.Param) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/mesh2d/wireframe2d.rs.html#458)

### impl [RenderAsset](trait.RenderAsset.html "trait bevy::render::render_asset::RenderAsset") for bevy::sprite\_render::[RenderWireframeMaterial](../../sprite_render/struct.RenderWireframeMaterial.html "struct bevy::sprite_render::RenderWireframeMaterial")

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/mesh2d/wireframe2d.rs.html#459)

#### type [SourceAsset](#associatedtype.SourceAsset) = [Wireframe2dMaterial](../../sprite_render/struct.Wireframe2dMaterial.html "struct bevy::sprite_render::Wireframe2dMaterial")

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/mesh2d/wireframe2d.rs.html#460)

#### type [Param](#associatedtype.Param) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/mesh2d/material.rs.html#1100)

### impl<M> [RenderAsset](trait.RenderAsset.html "trait bevy::render::render_asset::RenderAsset") for [PreparedMaterial2d](../../sprite_render/struct.PreparedMaterial2d.html "struct bevy::sprite_render::PreparedMaterial2d")<M>

where M: [Material2d](../../sprite_render/trait.Material2d.html "trait bevy::sprite_render::Material2d"),

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/mesh2d/material.rs.html#1101)

#### type [SourceAsset](#associatedtype.SourceAsset) = M

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/mesh2d/material.rs.html#1103)

#### type [Param](#associatedtype.Param) = ([Res](../../prelude/struct.Res.html "struct bevy::prelude::Res")<'static, [RenderDevice](../renderer/struct.RenderDevice.html "struct bevy::render::renderer::RenderDevice")\>, [Res](../../prelude/struct.Res.html "struct bevy::prelude::Res")<'static, [PipelineCache](../render_resource/struct.PipelineCache.html "struct bevy::render::render_resource::PipelineCache")\>, [Res](../../prelude/struct.Res.html "struct bevy::prelude::Res")<'static, [Material2dPipeline](../../sprite_render/struct.Material2dPipeline.html "struct bevy::sprite_render::Material2dPipeline")<M>>, [Res](../../prelude/struct.Res.html "struct bevy::prelude::Res")<'static, [DrawFunctions](../render_phase/struct.DrawFunctions.html "struct bevy::render::render_phase::DrawFunctions")<[Opaque2d](../../core_pipeline/core_2d/struct.Opaque2d.html "struct bevy::core_pipeline::core_2d::Opaque2d")\>>, [Res](../../prelude/struct.Res.html "struct bevy::prelude::Res")<'static, [DrawFunctions](../render_phase/struct.DrawFunctions.html "struct bevy::render::render_phase::DrawFunctions")<[AlphaMask2d](../../core_pipeline/core_2d/struct.AlphaMask2d.html "struct bevy::core_pipeline::core_2d::AlphaMask2d")\>>, [Res](../../prelude/struct.Res.html "struct bevy::prelude::Res")<'static, [DrawFunctions](../render_phase/struct.DrawFunctions.html "struct bevy::render::render_phase::DrawFunctions")<[Transparent2d](../../core_pipeline/core_2d/struct.Transparent2d.html "struct bevy::core_pipeline::core_2d::Transparent2d")\>>, [ResMut](../../prelude/struct.ResMut.html "struct bevy::prelude::ResMut")<'static, [RenderMaterial2dBindGroupIds](../../sprite_render/struct.RenderMaterial2dBindGroupIds.html "struct bevy::sprite_render::RenderMaterial2dBindGroupIds")\>, <M as [AsBindGroup](../render_resource/trait.AsBindGroup.html "trait bevy::render::render_resource::AsBindGroup")\>::[Param](../render_resource/trait.AsBindGroup.html#associatedtype.Param "type bevy::render::render_resource::AsBindGroup::Param"))

[Source](https://docs.rs/bevy_ui_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_render/ui_material_pipeline.rs.html#556)

### impl<M> [RenderAsset](trait.RenderAsset.html "trait bevy::render::render_asset::RenderAsset") for [PreparedUiMaterial](../../ui_render/struct.PreparedUiMaterial.html "struct bevy::ui_render::PreparedUiMaterial")<M>

where M: [UiMaterial](../../prelude/trait.UiMaterial.html "trait bevy::prelude::UiMaterial"),

[Source](https://docs.rs/bevy_ui_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_render/ui_material_pipeline.rs.html#557)

#### type [SourceAsset](#associatedtype.SourceAsset) = M

[Source](https://docs.rs/bevy_ui_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_render/ui_material_pipeline.rs.html#559)

#### type [Param](#associatedtype.Param) = ([Res](../../prelude/struct.Res.html "struct bevy::prelude::Res")<'static, [RenderDevice](../renderer/struct.RenderDevice.html "struct bevy::render::renderer::RenderDevice")\>, [Res](../../prelude/struct.Res.html "struct bevy::prelude::Res")<'static, [PipelineCache](../render_resource/struct.PipelineCache.html "struct bevy::render::render_resource::PipelineCache")\>, [Res](../../prelude/struct.Res.html "struct bevy::prelude::Res")<'static, [UiMaterialPipeline](../../ui_render/struct.UiMaterialPipeline.html "struct bevy::ui_render::UiMaterialPipeline")<M>>, <M as [AsBindGroup](../render_resource/trait.AsBindGroup.html "trait bevy::render::render_resource::AsBindGroup")\>::[Param](../render_resource/trait.AsBindGroup.html#associatedtype.Param "type bevy::render::render_resource::AsBindGroup::Param"))