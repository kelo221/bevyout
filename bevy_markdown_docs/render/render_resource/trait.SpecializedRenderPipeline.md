[bevy](../../index.html)::[render](../index.html)::[render\_resource](index.html)

# Trait SpecializedRenderPipeline 

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_resource/pipeline_specializer.rs.html#30)

```rust
pub trait SpecializedRenderPipeline {
    type Key: Clone + Hash + PartialEq + Eq;

    // Required method
    fn specialize(&self, key: Self::Key) -> RenderPipelineDescriptor;
}
```

A trait that allows constructing different variants of a render pipeline from a key.

Note: This is intended for modifying your pipeline descriptor on the basis of a key. If your key contains no data then you don’t need to specialize. For example, if you are using the [`AsBindGroup`](trait.AsBindGroup.html "trait bevy::render::render_resource::AsBindGroup") without the `#[bind_group_data]` attribute, you don’t need to specialize. Instead, create the pipeline directly from [`PipelineCache`](struct.PipelineCache.html "struct bevy::render::render_resource::PipelineCache") and store its ID.

See [`SpecializedRenderPipelines`](struct.SpecializedRenderPipelines.html "struct bevy::render::render_resource::SpecializedRenderPipelines") for more info.

## Required Associated Types

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_resource/pipeline_specializer.rs.html#32)

#### type [Key](#associatedtype.Key): [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash") + [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq") + [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq")

The key that defines each “variant” of the render pipeline.

## Required Methods

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_resource/pipeline_specializer.rs.html#35)

#### fn [specialize](#tymethod.specialize)(&self, key: Self::[Key](trait.SpecializedRenderPipeline.html#associatedtype.Key "type bevy::render::render_resource::SpecializedRenderPipeline::Key")) -> [RenderPipelineDescriptor](../../material/descriptor/struct.RenderPipelineDescriptor.html "struct bevy::material::descriptor::RenderPipelineDescriptor")

Construct a new render pipeline based on the provided key.

## Dyn Compatibility

This trait **is** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/blit/mod.rs.html#95)

### impl [SpecializedRenderPipeline](trait.SpecializedRenderPipeline.html "trait bevy::render::render_resource::SpecializedRenderPipeline") for [BlitPipeline](../../core_pipeline/blit/struct.BlitPipeline.html "struct bevy::core_pipeline::blit::BlitPipeline")

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/blit/mod.rs.html#96)

#### type [Key](#associatedtype.Key) = [BlitPipelineKey](../../core_pipeline/blit/struct.BlitPipelineKey.html "struct bevy::core_pipeline::blit::BlitPipelineKey")

[Source](https://docs.rs/bevy_ui_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_render/box_shadow.rs.html#133)

### impl [SpecializedRenderPipeline](trait.SpecializedRenderPipeline.html "trait bevy::render::render_resource::SpecializedRenderPipeline") for [BoxShadowPipeline](../../ui_render/box_shadow/struct.BoxShadowPipeline.html "struct bevy::ui_render::box_shadow::BoxShadowPipeline")

[Source](https://docs.rs/bevy_ui_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_render/box_shadow.rs.html#134)

#### type [Key](#associatedtype.Key) = [BoxShadowPipelineKey](../../ui_render/box_shadow/struct.BoxShadowPipelineKey.html "struct bevy::ui_render::box_shadow::BoxShadowPipelineKey")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/deferred/mod.rs.html#200)

### impl [SpecializedRenderPipeline](trait.SpecializedRenderPipeline.html "trait bevy::render::render_resource::SpecializedRenderPipeline") for [DeferredLightingLayout](../../pbr/deferred/struct.DeferredLightingLayout.html "struct bevy::pbr::deferred::DeferredLightingLayout")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/deferred/mod.rs.html#201)

#### type [Key](#associatedtype.Key) = [MeshPipelineKey](../../pbr/struct.MeshPipelineKey.html "struct bevy::pbr::MeshPipelineKey")

[Source](https://docs.rs/bevy_post_process/0.19.0/x86_64-unknown-linux-gnu/src/bevy_post_process/dof/mod.rs.html#586)

### impl [SpecializedRenderPipeline](trait.SpecializedRenderPipeline.html "trait bevy::render::render_resource::SpecializedRenderPipeline") for [DepthOfFieldPipeline](../../post_process/dof/struct.DepthOfFieldPipeline.html "struct bevy::post_process::dof::DepthOfFieldPipeline")

[Source](https://docs.rs/bevy_post_process/0.19.0/x86_64-unknown-linux-gnu/src/bevy_post_process/dof/mod.rs.html#587)

#### type [Key](#associatedtype.Key) = [DepthOfFieldPipelineKey](../../post_process/dof/struct.DepthOfFieldPipelineKey.html "struct bevy::post_process::dof::DepthOfFieldPipelineKey")

[Source](https://docs.rs/bevy_anti_alias/0.19.0/x86_64-unknown-linux-gnu/src/bevy_anti_alias/fxaa/mod.rs.html#165)

### impl [SpecializedRenderPipeline](trait.SpecializedRenderPipeline.html "trait bevy::render::render_resource::SpecializedRenderPipeline") for [FxaaPipeline](../../anti_alias/fxaa/struct.FxaaPipeline.html "struct bevy::anti_alias::fxaa::FxaaPipeline")

[Source](https://docs.rs/bevy_anti_alias/0.19.0/x86_64-unknown-linux-gnu/src/bevy_anti_alias/fxaa/mod.rs.html#166)

#### type [Key](#associatedtype.Key) = [FxaaPipelineKey](../../anti_alias/fxaa/struct.FxaaPipelineKey.html "struct bevy::anti_alias::fxaa::FxaaPipelineKey")

[Source](https://docs.rs/bevy_post_process/0.19.0/x86_64-unknown-linux-gnu/src/bevy_post_process/motion_blur/pipeline.rs.html#118)

### impl [SpecializedRenderPipeline](trait.SpecializedRenderPipeline.html "trait bevy::render::render_resource::SpecializedRenderPipeline") for [MotionBlurPipeline](../../post_process/motion_blur/pipeline/struct.MotionBlurPipeline.html "struct bevy::post_process::motion_blur::pipeline::MotionBlurPipeline")

[Source](https://docs.rs/bevy_post_process/0.19.0/x86_64-unknown-linux-gnu/src/bevy_post_process/motion_blur/pipeline.rs.html#119)

#### type [Key](#associatedtype.Key) = [MotionBlurPipelineKey](../../post_process/motion_blur/pipeline/struct.MotionBlurPipelineKey.html "struct bevy::post_process::motion_blur::pipeline::MotionBlurPipelineKey")

[Source](https://docs.rs/bevy_post_process/0.19.0/x86_64-unknown-linux-gnu/src/bevy_post_process/effect_stack/mod.rs.html#221)

### impl [SpecializedRenderPipeline](trait.SpecializedRenderPipeline.html "trait bevy::render::render_resource::SpecializedRenderPipeline") for [PostProcessingPipeline](../../post_process/effect_stack/struct.PostProcessingPipeline.html "struct bevy::post_process::effect_stack::PostProcessingPipeline")

[Source](https://docs.rs/bevy_post_process/0.19.0/x86_64-unknown-linux-gnu/src/bevy_post_process/effect_stack/mod.rs.html#222)

#### type [Key](#associatedtype.Key) = [PostProcessingPipelineKey](../../post_process/effect_stack/struct.PostProcessingPipelineKey.html "struct bevy::post_process::effect_stack::PostProcessingPipelineKey")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/ssr/mod.rs.html#474)

### impl [SpecializedRenderPipeline](trait.SpecializedRenderPipeline.html "trait bevy::render::render_resource::SpecializedRenderPipeline") for [ScreenSpaceReflectionsPipeline](../../pbr/struct.ScreenSpaceReflectionsPipeline.html "struct bevy::pbr::ScreenSpaceReflectionsPipeline")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/ssr/mod.rs.html#475)

#### type [Key](#associatedtype.Key) = [ScreenSpaceReflectionsPipelineKey](../../pbr/struct.ScreenSpaceReflectionsPipelineKey.html "struct bevy::pbr::ScreenSpaceReflectionsPipelineKey")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/view/window/screenshot.rs.html#468)

### impl [SpecializedRenderPipeline](trait.SpecializedRenderPipeline.html "trait bevy::render::render_resource::SpecializedRenderPipeline") for [ScreenshotToScreenPipeline](../view/window/screenshot/struct.ScreenshotToScreenPipeline.html "struct bevy::render::view::window::screenshot::ScreenshotToScreenPipeline")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/view/window/screenshot.rs.html#469)

#### type [Key](#associatedtype.Key) = [TextureFormat](enum.TextureFormat.html "enum bevy::render::render_resource::TextureFormat")

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/render/mod.rs.html#161)

### impl [SpecializedRenderPipeline](trait.SpecializedRenderPipeline.html "trait bevy::render::render_resource::SpecializedRenderPipeline") for [SpritePipeline](../../sprite_render/struct.SpritePipeline.html "struct bevy::sprite_render::SpritePipeline")

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/render/mod.rs.html#162)

#### type [Key](#associatedtype.Key) = [SpritePipelineKey](../../sprite_render/struct.SpritePipelineKey.html "struct bevy::sprite_render::SpritePipelineKey")

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/tonemapping/mod.rs.html#201)

### impl [SpecializedRenderPipeline](trait.SpecializedRenderPipeline.html "trait bevy::render::render_resource::SpecializedRenderPipeline") for [TonemappingPipeline](../../core_pipeline/tonemapping/struct.TonemappingPipeline.html "struct bevy::core_pipeline::tonemapping::TonemappingPipeline")

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/tonemapping/mod.rs.html#202)

#### type [Key](#associatedtype.Key) = [TonemappingPipelineKey](../../core_pipeline/tonemapping/struct.TonemappingPipelineKey.html "struct bevy::core_pipeline::tonemapping::TonemappingPipelineKey")

[Source](https://docs.rs/bevy_ui_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_render/pipeline.rs.html#54)

### impl [SpecializedRenderPipeline](trait.SpecializedRenderPipeline.html "trait bevy::render::render_resource::SpecializedRenderPipeline") for [UiPipeline](../../ui_render/struct.UiPipeline.html "struct bevy::ui_render::UiPipeline")

[Source](https://docs.rs/bevy_ui_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_render/pipeline.rs.html#55)

#### type [Key](#associatedtype.Key) = [UiPipelineKey](../../ui_render/struct.UiPipelineKey.html "struct bevy::ui_render::UiPipelineKey")

[Source](https://docs.rs/bevy_ui_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_render/ui_texture_slice_pipeline.rs.html#144)

### impl [SpecializedRenderPipeline](trait.SpecializedRenderPipeline.html "trait bevy::render::render_resource::SpecializedRenderPipeline") for [UiTextureSlicePipeline](../../ui_render/ui_texture_slice_pipeline/struct.UiTextureSlicePipeline.html "struct bevy::ui_render::ui_texture_slice_pipeline::UiTextureSlicePipeline")

[Source](https://docs.rs/bevy_ui_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_render/ui_texture_slice_pipeline.rs.html#145)

#### type [Key](#associatedtype.Key) = [UiTextureSlicePipelineKey](../../ui_render/ui_texture_slice_pipeline/struct.UiTextureSlicePipelineKey.html "struct bevy::ui_render::ui_texture_slice_pipeline::UiTextureSlicePipelineKey")

[Source](https://docs.rs/bevy_ui_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_render/ui_material_pipeline.rs.html#123-125)

### impl<M> [SpecializedRenderPipeline](trait.SpecializedRenderPipeline.html "trait bevy::render::render_resource::SpecializedRenderPipeline") for [UiMaterialPipeline](../../ui_render/struct.UiMaterialPipeline.html "struct bevy::ui_render::UiMaterialPipeline")<M>

where M: [UiMaterial](../../prelude/trait.UiMaterial.html "trait bevy::prelude::UiMaterial"), <M as [AsBindGroup](trait.AsBindGroup.html "trait bevy::render::render_resource::AsBindGroup")\>::[Data](trait.AsBindGroup.html#associatedtype.Data "type bevy::render::render_resource::AsBindGroup::Data"): [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq") + [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

[Source](https://docs.rs/bevy_ui_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_render/ui_material_pipeline.rs.html#127)

#### type [Key](#associatedtype.Key) = [UiMaterialKey](../../prelude/struct.UiMaterialKey.html "struct bevy::prelude::UiMaterialKey")<M>