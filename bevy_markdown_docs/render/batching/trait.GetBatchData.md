[bevy](../../index.html)::[render](../index.html)::[batching](index.html)

# Trait GetBatchData 

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/batching/mod.rs.html#77)

```rust
pub trait GetBatchData {
    type Param: SystemParam + 'static;
    type BatchCompareData: PartialEq;
    type BatchSetCompareData: PartialEq;
    type BufferData: GpuArrayBufferable + Sync + Send + 'static;

    // Required method
    fn get_batch_data(
        param: &<Self::Param as SystemParam>::Item<'_, '_>,
        query_item: (Entity, MainEntity),
    ) -> Option<(Self::BufferData, Option<(Self::BatchSetCompareData, Self::BatchCompareData)>)>;
}
```

A trait to support getting data used for batching draw commands via phase items.

This is a simple version that only allows for sorting, not binning, as well as only CPU processing, not GPU preprocessing. For these fancier features, see [`GetFullBatchData`](trait.GetFullBatchData.html "trait bevy::render::batching::GetFullBatchData").

## Required Associated Types

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/batching/mod.rs.html#80)

#### type [Param](#associatedtype.Param): [SystemParam](../../ecs/system/trait.SystemParam.html "trait bevy::ecs::system::SystemParam") + 'static

The system parameters [`GetBatchData::get_batch_data`](trait.GetBatchData.html#tymethod.get_batch_data "associated function bevy::render::batching::GetBatchData::get_batch_data") needs in order to compute the batch data.

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/batching/mod.rs.html#86)

#### type [BatchCompareData](#associatedtype.BatchCompareData): [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq")

Data used for comparison between phase items to decide whether items can be batched.

If this data, and the [`Self::BatchSetCompareData`](trait.GetBatchData.html#associatedtype.BatchSetCompareData "associated type bevy::render::batching::GetBatchData::BatchSetCompareData"), are identical to those of the previous phase item, the items can be batched together.

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/batching/mod.rs.html#93)

#### type [BatchSetCompareData](#associatedtype.BatchSetCompareData): [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq")

Data used for comparison between phase items to decide whether items can be grouped in the same batch set (i.e. multi-drawn).

If this data is identical to that of the previous phase items, and the current platform supports multi-draw, the items can be multi-drawn together.

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/batching/mod.rs.html#97)

#### type [BufferData](#associatedtype.BufferData): [GpuArrayBufferable](../render_resource/trait.GpuArrayBufferable.html "trait bevy::render::render_resource::GpuArrayBufferable") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + 'static

The per-instance data to be inserted into the [`crate::render_resource::GpuArrayBuffer`](../render_resource/enum.GpuArrayBuffer.html "enum bevy::render::render_resource::GpuArrayBuffer") containing these data for all instances.

## Required Methods

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/batching/mod.rs.html#106-112)

#### fn [get\_batch\_data](#tymethod.get_batch_data)( param: &<Self::[Param](trait.GetBatchData.html#associatedtype.Param "type bevy::render::batching::GetBatchData::Param") as [SystemParam](../../ecs/system/trait.SystemParam.html "trait bevy::ecs::system::SystemParam")\>::[Item](../../ecs/system/trait.SystemParam.html#associatedtype.Item "type bevy::ecs::system::SystemParam::Item")<'\_, '\_>, query\_item: ([Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity"), [MainEntity](../sync_world/struct.MainEntity.html "struct bevy::render::sync_world::MainEntity")), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<(Self::[BufferData](trait.GetBatchData.html#associatedtype.BufferData "type bevy::render::batching::GetBatchData::BufferData"), [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<(Self::[BatchSetCompareData](trait.GetBatchData.html#associatedtype.BatchSetCompareData "type bevy::render::batching::GetBatchData::BatchSetCompareData"), Self::[BatchCompareData](trait.GetBatchData.html#associatedtype.BatchCompareData "type bevy::render::batching::GetBatchData::BatchCompareData"))>)>

Get the per-instance data to be inserted into the [`crate::render_resource::GpuArrayBuffer`](../render_resource/enum.GpuArrayBuffer.html "enum bevy::render::render_resource::GpuArrayBuffer"). If the instance can be batched, also return the data used for comparison when deciding whether draws can be batched, else return None for the `CompareData`.

This is only called when building instance data on CPU. In the GPU instance data building path, we use [`GetFullBatchData::get_index_and_compare_data`](trait.GetFullBatchData.html#tymethod.get_index_and_compare_data "associated function bevy::render::batching::GetFullBatchData::get_index_and_compare_data") instead.

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/mesh2d/mesh.rs.html#356)

### impl [GetBatchData](trait.GetBatchData.html "trait bevy::render::batching::GetBatchData") for [Mesh2dPipeline](../../sprite_render/struct.Mesh2dPipeline.html "struct bevy::sprite_render::Mesh2dPipeline")

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/mesh2d/mesh.rs.html#357)

#### type [Param](#associatedtype.Param) = ([Res](../../prelude/struct.Res.html "struct bevy::prelude::Res")<'static, [RenderMesh2dInstances](../../sprite_render/struct.RenderMesh2dInstances.html "struct bevy::sprite_render::RenderMesh2dInstances")\>, [Res](../../prelude/struct.Res.html "struct bevy::prelude::Res")<'static, [RenderAssets](../render_asset/struct.RenderAssets.html "struct bevy::render::render_asset::RenderAssets")<[RenderMesh](../mesh/struct.RenderMesh.html "struct bevy::render::mesh::RenderMesh")\>>, [Res](../../prelude/struct.Res.html "struct bevy::prelude::Res")<'static, [MeshAllocator](../mesh/allocator/struct.MeshAllocator.html "struct bevy::render::mesh::allocator::MeshAllocator")\>)

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/mesh2d/mesh.rs.html#362)

#### type [BatchSetCompareData](#associatedtype.BatchSetCompareData) = ([Material2dBindGroupId](../../sprite_render/struct.Material2dBindGroupId.html "struct bevy::sprite_render::Material2dBindGroupId"), [AssetId](../../prelude/enum.AssetId.html "enum bevy::prelude::AssetId")<[Mesh](../../prelude/struct.Mesh.html "struct bevy::prelude::Mesh")\>)

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/mesh2d/mesh.rs.html#363)

#### type [BatchCompareData](#associatedtype.BatchCompareData) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/mesh2d/mesh.rs.html#364)

#### type [BufferData](#associatedtype.BufferData) = [Mesh2dUniform](../../sprite_render/struct.Mesh2dUniform.html "struct bevy::sprite_render::Mesh2dUniform")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#2803)

### impl [GetBatchData](trait.GetBatchData.html "trait bevy::render::batching::GetBatchData") for [MeshPipeline](../../pbr/struct.MeshPipeline.html "struct bevy::pbr::MeshPipeline")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#2804)

#### type [Param](#associatedtype.Param) = ([Res](../../prelude/struct.Res.html "struct bevy::prelude::Res")<'static, [RenderMeshInstances](../../pbr/enum.RenderMeshInstances.html "enum bevy::pbr::RenderMeshInstances")\>, [Res](../../prelude/struct.Res.html "struct bevy::prelude::Res")<'static, [RenderLightmaps](../../pbr/struct.RenderLightmaps.html "struct bevy::pbr::RenderLightmaps")\>, [Res](../../prelude/struct.Res.html "struct bevy::prelude::Res")<'static, [RenderAssets](../render_asset/struct.RenderAssets.html "struct bevy::render::render_asset::RenderAssets")<[RenderMesh](../mesh/struct.RenderMesh.html "struct bevy::render::mesh::RenderMesh")\>>, [Res](../../prelude/struct.Res.html "struct bevy::prelude::Res")<'static, [MeshAllocator](../mesh/allocator/struct.MeshAllocator.html "struct bevy::render::mesh::allocator::MeshAllocator")\>, [Res](../../prelude/struct.Res.html "struct bevy::prelude::Res")<'static, [SkinUniforms](../../pbr/struct.SkinUniforms.html "struct bevy::pbr::SkinUniforms")\>, [Res](../../prelude/struct.Res.html "struct bevy::prelude::Res")<'static, [MorphIndices](../../pbr/enum.MorphIndices.html "enum bevy::pbr::MorphIndices")\>)

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#2812)

#### type [BatchSetCompareData](#associatedtype.BatchSetCompareData) = [MeshBatchSetCompareData](../../pbr/struct.MeshBatchSetCompareData.html "struct bevy::pbr::MeshBatchSetCompareData")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#2813)

#### type [BatchCompareData](#associatedtype.BatchCompareData) = [AssetId](../../prelude/enum.AssetId.html "enum bevy::prelude::AssetId")<[Mesh](../../prelude/struct.Mesh.html "struct bevy::prelude::Mesh")\>

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#2815)

#### type [BufferData](#associatedtype.BufferData) = [MeshUniform](../../pbr/struct.MeshUniform.html "struct bevy::pbr::MeshUniform")