[bevy](../index.html)::[prelude](index.html)

# Trait FromWorld 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#3995)

```rust
pub trait FromWorld {
    // Required method
    fn from_world(world: &mut World) -> Self;
}
```

Creates an instance of the type this trait is implemented for using data from the supplied [`World`](struct.World.html "struct bevy::prelude::World").

This can be helpful for complex initialization or context-aware defaults.

[`FromWorld`](trait.FromWorld.html "trait bevy::prelude::FromWorld") is automatically implemented for any type implementing [`Default`](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") and may also be derived for:

*   any struct whose fields all implement `FromWorld`
*   any enum where one variant has the attribute `#[from_world]`

```
#[derive(Default)]
struct A;

#[derive(Default)]
struct B(Option<u32>)

struct C;

impl FromWorld for C {
    fn from_world(_world: &mut World) -> Self {
        Self
    }
}

#[derive(FromWorld)]
struct D(A, B, C);

#[derive(FromWorld)]
enum E {
    #[from_world]
    F,
    G
}
```

## Required Methods

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#3997)

#### fn [from\_world](#tymethod.from_world)(world: &mut [World](struct.World.html "struct bevy::prelude::World")) -> Self

Creates `Self` using data from the given [`World`](struct.World.html "struct bevy::prelude::World").

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/atmosphere/resources.rs.html#234)

### impl [FromWorld](trait.FromWorld.html "trait bevy::prelude::FromWorld") for [AtmosphereSampler](../pbr/resources/struct.AtmosphereSampler.html "struct bevy::pbr::resources::AtmosphereSampler")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/hierarchy.rs.html#121)

### impl [FromWorld](trait.FromWorld.html "trait bevy::prelude::FromWorld") for [ChildOf](struct.ChildOf.html "struct bevy::prelude::ChildOf")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_resource/texture.rs.html#178)

### impl [FromWorld](trait.FromWorld.html "trait bevy::prelude::FromWorld") for [DefaultImageSampler](../render/render_resource/struct.DefaultImageSampler.html "struct bevy::render::render_resource::DefaultImageSampler")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity_disabling.rs.html#181)

### impl [FromWorld](trait.FromWorld.html "trait bevy::prelude::FromWorld") for [DefaultQueryFilters](../ecs/entity_disabling/struct.DefaultQueryFilters.html "struct bevy::ecs::entity_disabling::DefaultQueryFilters")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/diagnostic/mod.rs.html#97)

### impl [FromWorld](trait.FromWorld.html "trait bevy::prelude::FromWorld") for [DiagnosticsRecorder](../render/diagnostic/struct.DiagnosticsRecorder.html "struct bevy::render::diagnostic::DiagnosticsRecorder")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/texture/fallback_image.rs.html#144)

### impl [FromWorld](trait.FromWorld.html "trait bevy::prelude::FromWorld") for [FallbackImage](../render/texture/struct.FallbackImage.html "struct bevy::render::texture::FallbackImage")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/texture/fallback_image.rs.html#225)

### impl [FromWorld](trait.FromWorld.html "trait bevy::prelude::FromWorld") for [FallbackImageCubemap](../render/texture/struct.FallbackImageCubemap.html "struct bevy::render::texture::FallbackImageCubemap")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/texture/fallback_image.rs.html#208)

### impl [FromWorld](trait.FromWorld.html "trait bevy::prelude::FromWorld") for [FallbackImageZero](../render/texture/struct.FallbackImageZero.html "struct bevy::render::texture::FallbackImageZero")

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/fullscreen_vertex_shader/mod.rs.html#10)

### impl [FromWorld](trait.FromWorld.html "trait bevy::prelude::FromWorld") for [FullscreenShader](../core_pipeline/struct.FullscreenShader.html "struct bevy::core_pipeline::FullscreenShader")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/batching/gpu_preprocessing.rs.html#1322)

### impl [FromWorld](trait.FromWorld.html "trait bevy::prelude::FromWorld") for [GpuPreprocessingSupport](../render/batching/gpu_preprocessing/struct.GpuPreprocessingSupport.html "struct bevy::render::batching::gpu_preprocessing::GpuPreprocessingSupport")

[Source](https://docs.rs/bevy_gizmos_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos_render/lib.rs.html#633)

### impl [FromWorld](trait.FromWorld.html "trait bevy::prelude::FromWorld") for [LineGizmoEntities](../gizmos_render/struct.LineGizmoEntities.html "struct bevy::gizmos_render::LineGizmoEntities")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/mesh/allocator.rs.html#218)

### impl [FromWorld](trait.FromWorld.html "trait bevy::prelude::FromWorld") for [MeshAllocator](../render/mesh/allocator/struct.MeshAllocator.html "struct bevy::render::mesh::allocator::MeshAllocator")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/morph.rs.html#78)

### impl [FromWorld](trait.FromWorld.html "trait bevy::prelude::FromWorld") for [MorphIndices](../pbr/enum.MorphIndices.html "enum bevy::pbr::MorphIndices")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/morph.rs.html#118)

### impl [FromWorld](trait.FromWorld.html "trait bevy::prelude::FromWorld") for [MorphUniforms](../pbr/struct.MorphUniforms.html "struct bevy::pbr::MorphUniforms")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/projection.rs.html#760)

### impl [FromWorld](trait.FromWorld.html "trait bevy::prelude::FromWorld") for [OrthographicProjection](struct.OrthographicProjection.html "struct bevy::prelude::OrthographicProjection")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/gpu_preprocess.rs.html#1261)

### impl [FromWorld](trait.FromWorld.html "trait bevy::prelude::FromWorld") for [PreprocessPipelines](../pbr/struct.PreprocessPipelines.html "struct bevy::pbr::PreprocessPipelines")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/mesh/morph.rs.html#134)

### impl [FromWorld](trait.FromWorld.html "trait bevy::prelude::FromWorld") for [RenderMorphTargetAllocator](../render/mesh/morph/enum.RenderMorphTargetAllocator.html "enum bevy::render::mesh::morph::RenderMorphTargetAllocator")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_resource/sparse_buffer_vec.rs.html#837)

### impl [FromWorld](trait.FromWorld.html "trait bevy::prelude::FromWorld") for [SparseBufferUpdateBindGroups](../render/render_resource/struct.SparseBufferUpdateBindGroups.html "struct bevy::render::render_resource::SparseBufferUpdateBindGroups")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_resource/sparse_buffer_vec.rs.html#277)

### impl [FromWorld](trait.FromWorld.html "trait bevy::prelude::FromWorld") for [SparseBufferUpdatePipelines](../render/render_resource/struct.SparseBufferUpdatePipelines.html "struct bevy::render::render_resource::SparseBufferUpdatePipelines")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/view/mod.rs.html#676)

### impl [FromWorld](trait.FromWorld.html "trait bevy::prelude::FromWorld") for [ViewUniforms](../render/view/struct.ViewUniforms.html "struct bevy::render::view::ViewUniforms")

[Source](https://docs.rs/bevy_world_serialization/0.19.0/x86_64-unknown-linux-gnu/src/bevy_world_serialization/world_asset_loader.rs.html#27)

### impl [FromWorld](trait.FromWorld.html "trait bevy::prelude::FromWorld") for [WorldAssetLoader](../world_serialization/struct.WorldAssetLoader.html "struct bevy::world_serialization::WorldAssetLoader")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/identifier.rs.html#44)

### impl [FromWorld](trait.FromWorld.html "trait bevy::prelude::FromWorld") for [WorldId](../ecs/world/struct.WorldId.html "struct bevy::ecs::world::WorldId")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/batching/no_gpu_preprocessing.rs.html#33-35)

### impl<BD> [FromWorld](trait.FromWorld.html "trait bevy::prelude::FromWorld") for [BatchedInstanceBuffer](../render/batching/no_gpu_preprocessing/struct.BatchedInstanceBuffer.html "struct bevy::render::batching::no_gpu_preprocessing::BatchedInstanceBuffer")<BD>

where BD: [GpuArrayBufferable](../render/render_resource/trait.GpuArrayBufferable.html "trait bevy::render::render_resource::GpuArrayBufferable") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + 'static,

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/query/state.rs.html#116)

### impl<D, F> [FromWorld](trait.FromWorld.html "trait bevy::prelude::FromWorld") for [QueryState](struct.QueryState.html "struct bevy::prelude::QueryState")<D, F>

where D: [QueryData](../ecs/query/trait.QueryData.html "trait bevy::ecs::query::QueryData"), F: [QueryFilter](../ecs/query/trait.QueryFilter.html "trait bevy::ecs::query::QueryFilter"),

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/batching/gpu_preprocessing.rs.html#990-992)

### impl<PI> [FromWorld](trait.FromWorld.html "trait bevy::prelude::FromWorld") for [PhaseIndirectParametersBuffers](../render/batching/gpu_preprocessing/struct.PhaseIndirectParametersBuffers.html "struct bevy::render::batching::gpu_preprocessing::PhaseIndirectParametersBuffers")<PI>

where PI: [PhaseItem](../render/render_phase/trait.PhaseItem.html "trait bevy::render::render_phase::PhaseItem"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/function_system.rs.html#487)

### impl<Param> [FromWorld](trait.FromWorld.html "trait bevy::prelude::FromWorld") for [SystemState](../ecs/system/struct.SystemState.html "struct bevy::ecs::system::SystemState")<Param>

where Param: [SystemParam](../ecs/system/trait.SystemParam.html "trait bevy::ecs::system::SystemParam"),

[Source](https://docs.rs/bevy_state/0.19.0/x86_64-unknown-linux-gnu/src/bevy_state/state/resources.rs.html#74)

### impl<S> [FromWorld](trait.FromWorld.html "trait bevy::prelude::FromWorld") for [State](struct.State.html "struct bevy::prelude::State")<S>

where S: [States](trait.States.html "trait bevy::prelude::States") + [FromWorld](trait.FromWorld.html "trait bevy::prelude::FromWorld"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#4000)

### impl<T> [FromWorld](trait.FromWorld.html "trait bevy::prelude::FromWorld") for T

where T: [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default"),