[bevy](../../index.html)::[render](../index.html)::[batching](index.html)

# Trait GetFullBatchData 

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/batching/mod.rs.html#119)

```rust
pub trait GetFullBatchData: GetBatchData {
    type BufferInputData: AtomicPod;

    // Required methods
    fn get_binned_batch_data(
        param: &<Self::Param as SystemParam>::Item<'_, '_>,
        query_item: MainEntity,
    ) -> Option<Self::BufferData>;
    fn get_index_and_compare_data(
        param: &<Self::Param as SystemParam>::Item<'_, '_>,
        query_item: MainEntity,
    ) -> Option<(NonMaxU32, Option<(Self::BatchSetCompareData, Self::BatchCompareData)>)>;
    fn get_binned_index(
        param: &<Self::Param as SystemParam>::Item<'_, '_>,
        query_item: MainEntity,
    ) -> Option<NonMaxU32>;
    fn write_batch_indirect_parameters_metadata(
        indexed: bool,
        base_output_index: u32,
        batch_set_index: Option<NonMaxU32>,
        indirect_parameters_buffers: &mut UntypedPhaseIndirectParametersBuffers,
        indirect_parameters_offset: u32,
    );
}
```

A trait to support getting data used for batching draw commands via phase items.

This version allows for binning and GPU preprocessing.

## Required Associated Types

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/batching/mod.rs.html#122)

#### type [BufferInputData](#associatedtype.BufferInputData): [AtomicPod](../render_resource/trait.AtomicPod.html "trait bevy::render::render_resource::AtomicPod")

The per-instance data that was inserted into the [`crate::render_resource::BufferVec`](../render_resource/struct.BufferVec.html "struct bevy::render::render_resource::BufferVec") during extraction.

## Required Methods

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/batching/mod.rs.html#130-133)

#### fn [get\_binned\_batch\_data](#tymethod.get_binned_batch_data)( param: &<Self::[Param](trait.GetBatchData.html#associatedtype.Param "type bevy::render::batching::GetBatchData::Param") as [SystemParam](../../ecs/system/trait.SystemParam.html "trait bevy::ecs::system::SystemParam")\>::[Item](../../ecs/system/trait.SystemParam.html#associatedtype.Item "type bevy::ecs::system::SystemParam::Item")<'\_, '\_>, query\_item: [MainEntity](../sync_world/struct.MainEntity.html "struct bevy::render::sync_world::MainEntity"), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<Self::[BufferData](trait.GetBatchData.html#associatedtype.BufferData "type bevy::render::batching::GetBatchData::BufferData")\>

Get the per-instance data to be inserted into the [`crate::render_resource::GpuArrayBuffer`](../render_resource/enum.GpuArrayBuffer.html "enum bevy::render::render_resource::GpuArrayBuffer").

This is only called when building uniforms on CPU. In the GPU instance buffer building path, we use [`GetFullBatchData::get_index_and_compare_data`](trait.GetFullBatchData.html#tymethod.get_index_and_compare_data "associated function bevy::render::batching::GetFullBatchData::get_index_and_compare_data") instead.

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/batching/mod.rs.html#142-148)

#### fn [get\_index\_and\_compare\_data](#tymethod.get_index_and_compare_data)( param: &<Self::[Param](trait.GetBatchData.html#associatedtype.Param "type bevy::render::batching::GetBatchData::Param") as [SystemParam](../../ecs/system/trait.SystemParam.html "trait bevy::ecs::system::SystemParam")\>::[Item](../../ecs/system/trait.SystemParam.html#associatedtype.Item "type bevy::ecs::system::SystemParam::Item")<'\_, '\_>, query\_item: [MainEntity](../sync_world/struct.MainEntity.html "struct bevy::render::sync_world::MainEntity"), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<([NonMaxU32](https://docs.rs/nonmax/0.5.5/x86_64-unknown-linux-gnu/nonmax/struct.NonMaxU32.html "struct nonmax::NonMaxU32"), [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<(Self::[BatchSetCompareData](trait.GetBatchData.html#associatedtype.BatchSetCompareData "type bevy::render::batching::GetBatchData::BatchSetCompareData"), Self::[BatchCompareData](trait.GetBatchData.html#associatedtype.BatchCompareData "type bevy::render::batching::GetBatchData::BatchCompareData"))>)>

Returns the index of the [`GetFullBatchData::BufferInputData`](trait.GetFullBatchData.html#associatedtype.BufferInputData "associated type bevy::render::batching::GetFullBatchData::BufferInputData") that the GPU preprocessing phase will use.

We already inserted the [`GetFullBatchData::BufferInputData`](trait.GetFullBatchData.html#associatedtype.BufferInputData "associated type bevy::render::batching::GetFullBatchData::BufferInputData") during the extraction phase before we got here, so this function shouldn’t need to look up any render data. If CPU instance buffer building is in use, this function will never be called.

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/batching/mod.rs.html#162-165)

#### fn [get\_binned\_index](#tymethod.get_binned_index)( param: &<Self::[Param](trait.GetBatchData.html#associatedtype.Param "type bevy::render::batching::GetBatchData::Param") as [SystemParam](../../ecs/system/trait.SystemParam.html "trait bevy::ecs::system::SystemParam")\>::[Item](../../ecs/system/trait.SystemParam.html#associatedtype.Item "type bevy::ecs::system::SystemParam::Item")<'\_, '\_>, query\_item: [MainEntity](../sync_world/struct.MainEntity.html "struct bevy::render::sync_world::MainEntity"), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[NonMaxU32](https://docs.rs/nonmax/0.5.5/x86_64-unknown-linux-gnu/nonmax/struct.NonMaxU32.html "struct nonmax::NonMaxU32")\>

Returns the index of the [`GetFullBatchData::BufferInputData`](trait.GetFullBatchData.html#associatedtype.BufferInputData "associated type bevy::render::batching::GetFullBatchData::BufferInputData") that the GPU preprocessing phase will use.

We already inserted the [`GetFullBatchData::BufferInputData`](trait.GetFullBatchData.html#associatedtype.BufferInputData "associated type bevy::render::batching::GetFullBatchData::BufferInputData") during the extraction phase before we got here, so this function shouldn’t need to look up any render data.

This function is currently only called for unbatchable entities when GPU instance buffer building is in use. For batchable entities, the uniform index is written during queuing (e.g. in `queue_material_meshes`). In the case of CPU instance buffer building, the CPU writes the uniforms, so there’s no index to return.

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/batching/mod.rs.html#188-194)

#### fn [write\_batch\_indirect\_parameters\_metadata](#tymethod.write_batch_indirect_parameters_metadata)( indexed: [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html), base\_output\_index: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html), batch\_set\_index: [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[NonMaxU32](https://docs.rs/nonmax/0.5.5/x86_64-unknown-linux-gnu/nonmax/struct.NonMaxU32.html "struct nonmax::NonMaxU32")\>, indirect\_parameters\_buffers: &mut [UntypedPhaseIndirectParametersBuffers](gpu_preprocessing/struct.UntypedPhaseIndirectParametersBuffers.html "struct bevy::render::batching::gpu_preprocessing::UntypedPhaseIndirectParametersBuffers"), indirect\_parameters\_offset: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html), )

Writes the [`gpu_preprocessing::IndirectParametersGpuMetadata`](gpu_preprocessing/struct.IndirectParametersGpuMetadata.html "struct bevy::render::batching::gpu_preprocessing::IndirectParametersGpuMetadata") necessary to draw this batch into the given metadata buffer at the given index.

This is only used if GPU culling is enabled (which requires GPU preprocessing).

*   `indexed` is true if the mesh is indexed or false if it’s non-indexed.
    
*   `base_output_index` is the index of the first mesh instance in this batch in the `MeshUniform` output buffer.
    
*   `batch_set_index` is the index of the batch set in the [`gpu_preprocessing::IndirectBatchSet`](gpu_preprocessing/struct.IndirectBatchSet.html "struct bevy::render::batching::gpu_preprocessing::IndirectBatchSet") buffer, if this batch belongs to a batch set.
    
*   `indirect_parameters_buffers` is the buffer in which to write the metadata.
    
*   `indirect_parameters_offset` is the index in that buffer at which to write the metadata.
    

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/mesh2d/mesh.rs.html#387)

### impl [GetFullBatchData](trait.GetFullBatchData.html "trait bevy::render::batching::GetFullBatchData") for [Mesh2dPipeline](../../sprite_render/struct.Mesh2dPipeline.html "struct bevy::sprite_render::Mesh2dPipeline")

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/mesh2d/mesh.rs.html#388)

#### type [BufferInputData](#associatedtype.BufferInputData) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#2868)

### impl [GetFullBatchData](trait.GetFullBatchData.html "trait bevy::render::batching::GetFullBatchData") for [MeshPipeline](../../pbr/struct.MeshPipeline.html "struct bevy::pbr::MeshPipeline")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#2869)

#### type [BufferInputData](#associatedtype.BufferInputData) = [MeshInputUniform](../../pbr/struct.MeshInputUniform.html "struct bevy::pbr::MeshInputUniform")