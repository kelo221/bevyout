[bevy](../../index.html)::[render](../index.html)::[render\_phase](index.html)

# Trait CachedRenderPipelinePhaseItem 

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_phase/mod.rs.html#2139)

```rust
pub trait CachedRenderPipelinePhaseItem: PhaseItem {
    // Required method
    fn cached_pipeline(&self) -> CachedRenderPipelineId;
}
```

A [`PhaseItem`](trait.PhaseItem.html "trait bevy::render::render_phase::PhaseItem") item, that automatically sets the appropriate render pipeline, cached in the [`PipelineCache`](../render_resource/struct.PipelineCache.html "struct bevy::render::render_resource::PipelineCache").

You can use the [`SetItemPipeline`](struct.SetItemPipeline.html "struct bevy::render::render_phase::SetItemPipeline") render command to set the pipeline for this item.

## Required Methods

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_phase/mod.rs.html#2142)

#### fn [cached\_pipeline](#tymethod.cached_pipeline)(&self) -> [CachedRenderPipelineId](../../material/descriptor/struct.CachedRenderPipelineId.html "struct bevy::material::descriptor::CachedRenderPipelineId")

The id of the render pipeline, cached in the [`PipelineCache`](../render_resource/struct.PipelineCache.html "struct bevy::render::render_resource::PipelineCache"), that will be used to draw this phase item.

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/core_2d/mod.rs.html#303)

### impl [CachedRenderPipelinePhaseItem](trait.CachedRenderPipelinePhaseItem.html "trait bevy::render::render_phase::CachedRenderPipelinePhaseItem") for [AlphaMask2d](../../core_pipeline/core_2d/struct.AlphaMask2d.html "struct bevy::core_pipeline::core_2d::AlphaMask2d")

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/core_3d/mod.rs.html#370)

### impl [CachedRenderPipelinePhaseItem](trait.CachedRenderPipelinePhaseItem.html "trait bevy::render::render_phase::CachedRenderPipelinePhaseItem") for [AlphaMask3d](../../core_pipeline/core_3d/struct.AlphaMask3d.html "struct bevy::core_pipeline::core_3d::AlphaMask3d")

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/deferred/mod.rs.html#181)

### impl [CachedRenderPipelinePhaseItem](trait.CachedRenderPipelinePhaseItem.html "trait bevy::render::render_phase::CachedRenderPipelinePhaseItem") for [AlphaMask3dDeferred](../../core_pipeline/deferred/struct.AlphaMask3dDeferred.html "struct bevy::core_pipeline::deferred::AlphaMask3dDeferred")

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/prepass/mod.rs.html#379)

### impl [CachedRenderPipelinePhaseItem](trait.CachedRenderPipelinePhaseItem.html "trait bevy::render::render_phase::CachedRenderPipelinePhaseItem") for [AlphaMask3dPrepass](../../core_pipeline/prepass/struct.AlphaMask3dPrepass.html "struct bevy::core_pipeline::prepass::AlphaMask3dPrepass")

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/core_2d/mod.rs.html#202)

### impl [CachedRenderPipelinePhaseItem](trait.CachedRenderPipelinePhaseItem.html "trait bevy::render::render_phase::CachedRenderPipelinePhaseItem") for [Opaque2d](../../core_pipeline/core_2d/struct.Opaque2d.html "struct bevy::core_pipeline::core_2d::Opaque2d")

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/core_3d/mod.rs.html#292)

### impl [CachedRenderPipelinePhaseItem](trait.CachedRenderPipelinePhaseItem.html "trait bevy::render::render_phase::CachedRenderPipelinePhaseItem") for [Opaque3d](../../core_pipeline/core_3d/struct.Opaque3d.html "struct bevy::core_pipeline::core_3d::Opaque3d")

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/deferred/mod.rs.html#98)

### impl [CachedRenderPipelinePhaseItem](trait.CachedRenderPipelinePhaseItem.html "trait bevy::render::render_phase::CachedRenderPipelinePhaseItem") for [Opaque3dDeferred](../../core_pipeline/deferred/struct.Opaque3dDeferred.html "struct bevy::core_pipeline::deferred::Opaque3dDeferred")

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/prepass/mod.rs.html#296)

### impl [CachedRenderPipelinePhaseItem](trait.CachedRenderPipelinePhaseItem.html "trait bevy::render::render_phase::CachedRenderPipelinePhaseItem") for [Opaque3dPrepass](../../core_pipeline/prepass/struct.Opaque3dPrepass.html "struct bevy::core_pipeline::prepass::Opaque3dPrepass")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/light.rs.html#2749)

### impl [CachedRenderPipelinePhaseItem](trait.CachedRenderPipelinePhaseItem.html "trait bevy::render::render_phase::CachedRenderPipelinePhaseItem") for [Shadow](../../pbr/struct.Shadow.html "struct bevy::pbr::Shadow")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/transmission/phase.rs.html#116)

### impl [CachedRenderPipelinePhaseItem](trait.CachedRenderPipelinePhaseItem.html "trait bevy::render::render_phase::CachedRenderPipelinePhaseItem") for [Transmissive3d](../../pbr/struct.Transmissive3d.html "struct bevy::pbr::Transmissive3d")

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/core_2d/mod.rs.html#386)

### impl [CachedRenderPipelinePhaseItem](trait.CachedRenderPipelinePhaseItem.html "trait bevy::render::render_phase::CachedRenderPipelinePhaseItem") for [Transparent2d](../../core_pipeline/core_2d/struct.Transparent2d.html "struct bevy::core_pipeline::core_2d::Transparent2d")

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/core_3d/mod.rs.html#457)

### impl [CachedRenderPipelinePhaseItem](trait.CachedRenderPipelinePhaseItem.html "trait bevy::render::render_phase::CachedRenderPipelinePhaseItem") for [Transparent3d](../../core_pipeline/core_3d/struct.Transparent3d.html "struct bevy::core_pipeline::core_3d::Transparent3d")

[Source](https://docs.rs/bevy_ui_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_render/render_pass.rs.html#150)

### impl [CachedRenderPipelinePhaseItem](trait.CachedRenderPipelinePhaseItem.html "trait bevy::render::render_phase::CachedRenderPipelinePhaseItem") for [TransparentUi](../../ui_render/struct.TransparentUi.html "struct bevy::ui_render::TransparentUi")

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/mesh2d/wireframe2d.rs.html#215)

### impl [CachedRenderPipelinePhaseItem](trait.CachedRenderPipelinePhaseItem.html "trait bevy::render::render_phase::CachedRenderPipelinePhaseItem") for [Wireframe2dPhaseItem](../../sprite_render/struct.Wireframe2dPhaseItem.html "struct bevy::sprite_render::Wireframe2dPhaseItem")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/wireframe.rs.html#251)

### impl [CachedRenderPipelinePhaseItem](trait.CachedRenderPipelinePhaseItem.html "trait bevy::render::render_phase::CachedRenderPipelinePhaseItem") for [Wireframe3d](../../pbr/wireframe/struct.Wireframe3d.html "struct bevy::pbr::wireframe::Wireframe3d")