[bevy](../../index.html)::[render](../index.html)::[extract\_resource](index.html)

# Trait ExtractResource 

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/extract_resource.rs.html#18)

```rust
pub trait ExtractResource<F = ()>: Resource {
    type Source: Resource;

    // Required method
    fn extract_resource(source: &Self::Source) -> Self;
}
```

Describes how a resource gets extracted for rendering.

Therefore the resource is transferred from the “main world” into the “render world” in the [`ExtractSchedule`](../../prelude/struct.ExtractSchedule.html "struct bevy::prelude::ExtractSchedule") step.

The marker type `F` is only used as a way to bypass the orphan rules. To implement the trait for a foreign type you can use a local type as the marker, e.g. the type of the plugin that calls [`ExtractResourcePlugin`](struct.ExtractResourcePlugin.html "struct bevy::render::extract_resource::ExtractResourcePlugin").

## Required Associated Types

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/extract_resource.rs.html#19)

#### type [Source](#associatedtype.Source): [Resource](../../prelude/trait.Resource.html "trait bevy::prelude::Resource")

## Required Methods

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/extract_resource.rs.html#22)

#### fn [extract\_resource](#tymethod.extract_resource)(source: &Self::[Source](trait.ExtractResource.html#associatedtype.Source "type bevy::render::extract_resource::ExtractResource::Source")) -> Self

Defines how the resource is transferred into the “render world”.

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/camera.rs.html#125)

### impl [ExtractResource](trait.ExtractResource.html "trait bevy::render::extract_resource::ExtractResource") for [ClearColor](../../prelude/struct.ClearColor.html "struct bevy::prelude::ClearColor")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/camera.rs.html#126)

#### type [Source](#associatedtype.Source) = [ClearColor](../../prelude/struct.ClearColor.html "struct bevy::prelude::ClearColor")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/material.rs.html#1358)

### impl [ExtractResource](trait.ExtractResource.html "trait bevy::render::extract_resource::ExtractResource") for [DefaultOpaqueRendererMethod](../../pbr/struct.DefaultOpaqueRendererMethod.html "struct bevy::pbr::DefaultOpaqueRendererMethod")

where [DefaultOpaqueRendererMethod](../../pbr/struct.DefaultOpaqueRendererMethod.html "struct bevy::pbr::DefaultOpaqueRendererMethod"): [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/material.rs.html#1358)

#### type [Source](#associatedtype.Source) = [DefaultOpaqueRendererMethod](../../pbr/struct.DefaultOpaqueRendererMethod.html "struct bevy::pbr::DefaultOpaqueRendererMethod")

[Source](https://docs.rs/bevy_dev_tools/0.19.0/x86_64-unknown-linux-gnu/src/bevy_dev_tools/render_debug.rs.html#296)

### impl [ExtractResource](trait.ExtractResource.html "trait bevy::render::extract_resource::ExtractResource") for [GlobalRenderDebugOverlay](../../dev_tools/render_debug/struct.GlobalRenderDebugOverlay.html "struct bevy::dev_tools::render_debug::GlobalRenderDebugOverlay")

where [GlobalRenderDebugOverlay](../../dev_tools/render_debug/struct.GlobalRenderDebugOverlay.html "struct bevy::dev_tools::render_debug::GlobalRenderDebugOverlay"): [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

[Source](https://docs.rs/bevy_dev_tools/0.19.0/x86_64-unknown-linux-gnu/src/bevy_dev_tools/render_debug.rs.html#296)

#### type [Source](#associatedtype.Source) = [GlobalRenderDebugOverlay](../../dev_tools/render_debug/struct.GlobalRenderDebugOverlay.html "struct bevy::dev_tools::render_debug::GlobalRenderDebugOverlay")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/globals.rs.html#42)

### impl [ExtractResource](trait.ExtractResource.html "trait bevy::render::extract_resource::ExtractResource") for [GlobalsUniform](../globals/struct.GlobalsUniform.html "struct bevy::render::globals::GlobalsUniform")

where [GlobalsUniform](../globals/struct.GlobalsUniform.html "struct bevy::render::globals::GlobalsUniform"): [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/globals.rs.html#42)

#### type [Source](#associatedtype.Source) = [GlobalsUniform](../globals/struct.GlobalsUniform.html "struct bevy::render::globals::GlobalsUniform")

[Source](https://docs.rs/bevy_gizmos_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos_render/lib.rs.html#622)

### impl [ExtractResource](trait.ExtractResource.html "trait bevy::render::extract_resource::ExtractResource") for [LineGizmoEntities](../../gizmos_render/struct.LineGizmoEntities.html "struct bevy::gizmos_render::LineGizmoEntities")

where [LineGizmoEntities](../../gizmos_render/struct.LineGizmoEntities.html "struct bevy::gizmos_render::LineGizmoEntities"): [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

[Source](https://docs.rs/bevy_gizmos_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos_render/lib.rs.html#622)

#### type [Source](#associatedtype.Source) = [LineGizmoEntities](../../gizmos_render/struct.LineGizmoEntities.html "struct bevy::gizmos_render::LineGizmoEntities")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/texture/manual_texture_view.rs.html#52)

### impl [ExtractResource](trait.ExtractResource.html "trait bevy::render::extract_resource::ExtractResource") for [ManualTextureViews](../../prelude/struct.ManualTextureViews.html "struct bevy::prelude::ManualTextureViews")

where [ManualTextureViews](../../prelude/struct.ManualTextureViews.html "struct bevy::prelude::ManualTextureViews"): [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/texture/manual_texture_view.rs.html#52)

#### type [Source](#associatedtype.Source) = [ManualTextureViews](../../prelude/struct.ManualTextureViews.html "struct bevy::prelude::ManualTextureViews")

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/tonemapping/mod.rs.html#35)

### impl [ExtractResource](trait.ExtractResource.html "trait bevy::render::extract_resource::ExtractResource") for [TonemappingLuts](../../core_pipeline/tonemapping/struct.TonemappingLuts.html "struct bevy::core_pipeline::tonemapping::TonemappingLuts")

where [TonemappingLuts](../../core_pipeline/tonemapping/struct.TonemappingLuts.html "struct bevy::core_pipeline::tonemapping::TonemappingLuts"): [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/tonemapping/mod.rs.html#35)

#### type [Source](#associatedtype.Source) = [TonemappingLuts](../../core_pipeline/tonemapping/struct.TonemappingLuts.html "struct bevy::core_pipeline::tonemapping::TonemappingLuts")

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/mesh2d/wireframe2d.rs.html#422)

### impl [ExtractResource](trait.ExtractResource.html "trait bevy::render::extract_resource::ExtractResource") for [Wireframe2dConfig](../../sprite_render/struct.Wireframe2dConfig.html "struct bevy::sprite_render::Wireframe2dConfig")

where [Wireframe2dConfig](../../sprite_render/struct.Wireframe2dConfig.html "struct bevy::sprite_render::Wireframe2dConfig"): [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/mesh2d/wireframe2d.rs.html#422)

#### type [Source](#associatedtype.Source) = [Wireframe2dConfig](../../sprite_render/struct.Wireframe2dConfig.html "struct bevy::sprite_render::Wireframe2dConfig")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/wireframe.rs.html#883)

### impl [ExtractResource](trait.ExtractResource.html "trait bevy::render::extract_resource::ExtractResource") for [WireframeConfig](../../pbr/wireframe/struct.WireframeConfig.html "struct bevy::pbr::wireframe::WireframeConfig")

where [WireframeConfig](../../pbr/wireframe/struct.WireframeConfig.html "struct bevy::pbr::wireframe::WireframeConfig"): [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/wireframe.rs.html#883)

#### type [Source](#associatedtype.Source) = [WireframeConfig](../../pbr/wireframe/struct.WireframeConfig.html "struct bevy::pbr::wireframe::WireframeConfig")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/cluster/gpu.rs.html#1728)

### impl [ExtractResource](trait.ExtractResource.html "trait bevy::render::extract_resource::ExtractResource")<GpuClusteringPlugin> for [GlobalClusterSettings](../../light/cluster/struct.GlobalClusterSettings.html "struct bevy::light::cluster::GlobalClusterSettings")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/cluster/gpu.rs.html#1729)

#### type [Source](#associatedtype.Source) = [GlobalClusterSettings](../../light/cluster/struct.GlobalClusterSettings.html "struct bevy::light::cluster::GlobalClusterSettings")