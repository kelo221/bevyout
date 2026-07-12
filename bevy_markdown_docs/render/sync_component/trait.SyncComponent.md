[bevy](../../index.html)::[render](../index.html)::[sync\_component](index.html)

# Trait SyncComponent 

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/sync_component.rs.html#40)

```rust
pub trait SyncComponent<F = ()>: Component {
    type Target: Bundle
       where <Self::Target as DynamicBundle>::Effect: NoBundleEffect;
}
```

Trait that links components from the main world with output components in the render world. It is used by [`SyncComponentPlugin`](struct.SyncComponentPlugin.html "struct bevy::render::sync_component::SyncComponentPlugin").

The marker type `F` is only used as a way to bypass the orphan rules. To implement the trait for a foreign type you can use a local type as the marker, e.g. the type of the plugin that calls [`SyncComponentPlugin`](struct.SyncComponentPlugin.html "struct bevy::render::sync_component::SyncComponentPlugin").

## Required Associated Types

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/sync_component.rs.html#43)

#### type [Target](#associatedtype.Target): [Bundle](../../prelude/trait.Bundle.html "trait bevy::prelude::Bundle") where <Self::[Target](trait.SyncComponent.html#associatedtype.Target "type bevy::render::sync_component::SyncComponent::Target") as [DynamicBundle](../../ecs/bundle/trait.DynamicBundle.html "trait bevy::ecs::bundle::DynamicBundle")\>::[Effect](../../ecs/bundle/trait.DynamicBundle.html#associatedtype.Effect "type bevy::ecs::bundle::DynamicBundle::Effect"): [NoBundleEffect](../../ecs/bundle/trait.NoBundleEffect.html "trait bevy::ecs::bundle::NoBundleEffect")

Describes what components should be removed from the render world if the implementing component is removed.

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/atmosphere/mod.rs.html#399)

### impl [SyncComponent](trait.SyncComponent.html "trait bevy::render::sync_component::SyncComponent") for [AtmosphereSettings](../../pbr/struct.AtmosphereSettings.html "struct bevy::pbr::AtmosphereSettings")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/atmosphere/mod.rs.html#400)

#### type [Target](#associatedtype.Target) = [GpuAtmosphereSettings](../../pbr/struct.GpuAtmosphereSettings.html "struct bevy::pbr::GpuAtmosphereSettings")

[Source](https://docs.rs/bevy_post_process/0.19.0/x86_64-unknown-linux-gnu/src/bevy_post_process/auto_exposure/settings.rs.html#27)

### impl [SyncComponent](trait.SyncComponent.html "trait bevy::render::sync_component::SyncComponent") for [AutoExposure](../../post_process/auto_exposure/struct.AutoExposure.html "struct bevy::post_process::auto_exposure::AutoExposure")

where [AutoExposure](../../post_process/auto_exposure/struct.AutoExposure.html "struct bevy::post_process::auto_exposure::AutoExposure"): [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

[Source](https://docs.rs/bevy_post_process/0.19.0/x86_64-unknown-linux-gnu/src/bevy_post_process/auto_exposure/settings.rs.html#27)

#### type [Target](#associatedtype.Target) = [AutoExposure](../../post_process/auto_exposure/struct.AutoExposure.html "struct bevy::post_process::auto_exposure::AutoExposure")

[Source](https://docs.rs/bevy_post_process/0.19.0/x86_64-unknown-linux-gnu/src/bevy_post_process/bloom/settings.rs.html#223)

### impl [SyncComponent](trait.SyncComponent.html "trait bevy::render::sync_component::SyncComponent") for [Bloom](../../post_process/bloom/struct.Bloom.html "struct bevy::post_process::bloom::Bloom")

[Source](https://docs.rs/bevy_post_process/0.19.0/x86_64-unknown-linux-gnu/src/bevy_post_process/bloom/settings.rs.html#224)

#### type [Target](#associatedtype.Target) = ([Bloom](../../post_process/bloom/struct.Bloom.html "struct bevy::post_process::bloom::Bloom"), BloomUniforms)

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/camera.rs.html#147)

### impl [SyncComponent](trait.SyncComponent.html "trait bevy::render::sync_component::SyncComponent") for [Camera2d](../../prelude/struct.Camera2d.html "struct bevy::prelude::Camera2d")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/camera.rs.html#148)

#### type [Target](#associatedtype.Target) = [Camera2d](../../prelude/struct.Camera2d.html "struct bevy::prelude::Camera2d")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/camera.rs.html#161)

### impl [SyncComponent](trait.SyncComponent.html "trait bevy::render::sync_component::SyncComponent") for [Camera3d](../../prelude/struct.Camera3d.html "struct bevy::prelude::Camera3d")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/camera.rs.html#162)

#### type [Target](#associatedtype.Target) = [Camera3d](../../prelude/struct.Camera3d.html "struct bevy::prelude::Camera3d")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/camera.rs.html#133)

### impl [SyncComponent](trait.SyncComponent.html "trait bevy::render::sync_component::SyncComponent") for [CameraMainTextureUsages](../../camera/struct.CameraMainTextureUsages.html "struct bevy::camera::CameraMainTextureUsages")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/camera.rs.html#134)

#### type [Target](#associatedtype.Target) = [CameraMainTextureUsages](../../camera/struct.CameraMainTextureUsages.html "struct bevy::camera::CameraMainTextureUsages")

[Source](https://docs.rs/bevy_post_process/0.19.0/x86_64-unknown-linux-gnu/src/bevy_post_process/effect_stack/chromatic_aberration.rs.html#81)

### impl [SyncComponent](trait.SyncComponent.html "trait bevy::render::sync_component::SyncComponent") for [ChromaticAberration](../../post_process/effect_stack/struct.ChromaticAberration.html "struct bevy::post_process::effect_stack::ChromaticAberration")

[Source](https://docs.rs/bevy_post_process/0.19.0/x86_64-unknown-linux-gnu/src/bevy_post_process/effect_stack/chromatic_aberration.rs.html#82)

#### type [Target](#associatedtype.Target) = [ChromaticAberration](../../post_process/effect_stack/struct.ChromaticAberration.html "struct bevy::post_process::effect_stack::ChromaticAberration")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/contact_shadows.rs.html#82)

### impl [SyncComponent](trait.SyncComponent.html "trait bevy::render::sync_component::SyncComponent") for [ContactShadows](../../pbr/struct.ContactShadows.html "struct bevy::pbr::ContactShadows")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/contact_shadows.rs.html#83)

#### type [Target](#associatedtype.Target) = ([ContactShadows](../../pbr/struct.ContactShadows.html "struct bevy::pbr::ContactShadows"), [ViewContactShadowsUniformOffset](../../pbr/struct.ViewContactShadowsUniformOffset.html "struct bevy::pbr::ViewContactShadowsUniformOffset"))

[Source](https://docs.rs/bevy_anti_alias/0.19.0/x86_64-unknown-linux-gnu/src/bevy_anti_alias/contrast_adaptive_sharpening/mod.rs.html#78)

### impl [SyncComponent](trait.SyncComponent.html "trait bevy::render::sync_component::SyncComponent") for [ContrastAdaptiveSharpening](../../anti_alias/contrast_adaptive_sharpening/struct.ContrastAdaptiveSharpening.html "struct bevy::anti_alias::contrast_adaptive_sharpening::ContrastAdaptiveSharpening")

[Source](https://docs.rs/bevy_anti_alias/0.19.0/x86_64-unknown-linux-gnu/src/bevy_anti_alias/contrast_adaptive_sharpening/mod.rs.html#79)

#### type [Target](#associatedtype.Target) = ([DenoiseCas](../../anti_alias/contrast_adaptive_sharpening/struct.DenoiseCas.html "struct bevy::anti_alias::contrast_adaptive_sharpening::DenoiseCas"), CasUniform)

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/tonemapping/mod.rs.html#379)

### impl [SyncComponent](trait.SyncComponent.html "trait bevy::render::sync_component::SyncComponent") for [DebandDither](../../core_pipeline/tonemapping/enum.DebandDither.html "enum bevy::core_pipeline::tonemapping::DebandDither")

where [DebandDither](../../core_pipeline/tonemapping/enum.DebandDither.html "enum bevy::core_pipeline::tonemapping::DebandDither"): [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/tonemapping/mod.rs.html#379)

#### type [Target](#associatedtype.Target) = [DebandDither](../../core_pipeline/tonemapping/enum.DebandDither.html "enum bevy::core_pipeline::tonemapping::DebandDither")

[Source](https://docs.rs/bevy_post_process/0.19.0/x86_64-unknown-linux-gnu/src/bevy_post_process/dof/mod.rs.html#652)

### impl [SyncComponent](trait.SyncComponent.html "trait bevy::render::sync_component::SyncComponent") for [DepthOfField](../../post_process/dof/struct.DepthOfField.html "struct bevy::post_process::dof::DepthOfField")

[Source](https://docs.rs/bevy_post_process/0.19.0/x86_64-unknown-linux-gnu/src/bevy_post_process/dof/mod.rs.html#653)

#### type [Target](#associatedtype.Target) = ([DepthOfField](../../post_process/dof/struct.DepthOfField.html "struct bevy::post_process::dof::DepthOfField"), [DepthOfFieldUniform](../../post_process/dof/struct.DepthOfFieldUniform.html "struct bevy::post_process::dof::DepthOfFieldUniform"), [DepthOfFieldPipelines](../../post_process/dof/enum.DepthOfFieldPipelines.html "enum bevy::post_process::dof::DepthOfFieldPipelines"), [AuxiliaryDepthOfFieldTexture](../../post_process/dof/struct.AuxiliaryDepthOfFieldTexture.html "struct bevy::post_process::dof::AuxiliaryDepthOfFieldTexture"), [ViewDepthOfFieldBindGroupLayouts](../../post_process/dof/struct.ViewDepthOfFieldBindGroupLayouts.html "struct bevy::post_process::dof::ViewDepthOfFieldBindGroupLayouts"))

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/fog.rs.html#51)

### impl [SyncComponent](trait.SyncComponent.html "trait bevy::render::sync_component::SyncComponent") for [DistanceFog](../../prelude/struct.DistanceFog.html "struct bevy::prelude::DistanceFog")

where [DistanceFog](../../prelude/struct.DistanceFog.html "struct bevy::prelude::DistanceFog"): [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/fog.rs.html#51)

#### type [Target](#associatedtype.Target) = ([DistanceFog](../../prelude/struct.DistanceFog.html "struct bevy::prelude::DistanceFog"), [ViewFogUniformOffset](../../pbr/struct.ViewFogUniformOffset.html "struct bevy::pbr::ViewFogUniformOffset"))

[Source](https://docs.rs/bevy_anti_alias/0.19.0/x86_64-unknown-linux-gnu/src/bevy_anti_alias/fxaa/mod.rs.html#53)

### impl [SyncComponent](trait.SyncComponent.html "trait bevy::render::sync_component::SyncComponent") for [Fxaa](../../anti_alias/fxaa/struct.Fxaa.html "struct bevy::anti_alias::fxaa::Fxaa")

where [Fxaa](../../anti_alias/fxaa/struct.Fxaa.html "struct bevy::anti_alias::fxaa::Fxaa"): [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

[Source](https://docs.rs/bevy_anti_alias/0.19.0/x86_64-unknown-linux-gnu/src/bevy_anti_alias/fxaa/mod.rs.html#53)

#### type [Target](#associatedtype.Target) = [Fxaa](../../anti_alias/fxaa/struct.Fxaa.html "struct bevy::anti_alias::fxaa::Fxaa")

[Source](https://docs.rs/bevy_dev_tools/0.19.0/x86_64-unknown-linux-gnu/src/bevy_dev_tools/render_debug.rs.html#296)

### impl [SyncComponent](trait.SyncComponent.html "trait bevy::render::sync_component::SyncComponent") for [GlobalRenderDebugOverlay](../../dev_tools/render_debug/struct.GlobalRenderDebugOverlay.html "struct bevy::dev_tools::render_debug::GlobalRenderDebugOverlay")

where [GlobalRenderDebugOverlay](../../dev_tools/render_debug/struct.GlobalRenderDebugOverlay.html "struct bevy::dev_tools::render_debug::GlobalRenderDebugOverlay"): [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

[Source](https://docs.rs/bevy_dev_tools/0.19.0/x86_64-unknown-linux-gnu/src/bevy_dev_tools/render_debug.rs.html#296)

#### type [Target](#associatedtype.Target) = [GlobalRenderDebugOverlay](../../dev_tools/render_debug/struct.GlobalRenderDebugOverlay.html "struct bevy::dev_tools::render_debug::GlobalRenderDebugOverlay")

[Source](https://docs.rs/bevy_post_process/0.19.0/x86_64-unknown-linux-gnu/src/bevy_post_process/effect_stack/lens_distortion.rs.html#81)

### impl [SyncComponent](trait.SyncComponent.html "trait bevy::render::sync_component::SyncComponent") for [LensDistortion](../../post_process/effect_stack/struct.LensDistortion.html "struct bevy::post_process::effect_stack::LensDistortion")

[Source](https://docs.rs/bevy_post_process/0.19.0/x86_64-unknown-linux-gnu/src/bevy_post_process/effect_stack/lens_distortion.rs.html#82)

#### type [Target](#associatedtype.Target) = [LensDistortion](../../post_process/effect_stack/struct.LensDistortion.html "struct bevy::post_process::effect_stack::LensDistortion")

[Source](https://docs.rs/bevy_post_process/0.19.0/x86_64-unknown-linux-gnu/src/bevy_post_process/motion_blur/mod.rs.html#118)

### impl [SyncComponent](trait.SyncComponent.html "trait bevy::render::sync_component::SyncComponent") for [MotionBlur](../../post_process/motion_blur/struct.MotionBlur.html "struct bevy::post_process::motion_blur::MotionBlur")

[Source](https://docs.rs/bevy_post_process/0.19.0/x86_64-unknown-linux-gnu/src/bevy_post_process/motion_blur/mod.rs.html#119)

#### type [Target](#associatedtype.Target) = MotionBlurUniform

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/view/mod.rs.html#230)

### impl [SyncComponent](trait.SyncComponent.html "trait bevy::render::sync_component::SyncComponent") for [Msaa](../../prelude/enum.Msaa.html "enum bevy::prelude::Msaa")

where [Msaa](../../prelude/enum.Msaa.html "enum bevy::prelude::Msaa"): [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/view/mod.rs.html#230)

#### type [Target](#associatedtype.Target) = [Msaa](../../prelude/enum.Msaa.html "enum bevy::prelude::Msaa")

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/prepass/background_motion_vectors.rs.html#59)

### impl [SyncComponent](trait.SyncComponent.html "trait bevy::render::sync_component::SyncComponent") for [NoBackgroundMotionVectors](../../core_pipeline/prepass/struct.NoBackgroundMotionVectors.html "struct bevy::core_pipeline::prepass::NoBackgroundMotionVectors")

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/prepass/background_motion_vectors.rs.html#60)

#### type [Target](#associatedtype.Target) = [NoBackgroundMotionVectors](../../core_pipeline/prepass/struct.NoBackgroundMotionVectors.html "struct bevy::core_pipeline::prepass::NoBackgroundMotionVectors")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/occlusion_culling/mod.rs.html#70)

### impl [SyncComponent](trait.SyncComponent.html "trait bevy::render::sync_component::SyncComponent") for [OcclusionCulling](../occlusion_culling/struct.OcclusionCulling.html "struct bevy::render::occlusion_culling::OcclusionCulling")

where [OcclusionCulling](../occlusion_culling/struct.OcclusionCulling.html "struct bevy::render::occlusion_culling::OcclusionCulling"): [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/occlusion_culling/mod.rs.html#70)

#### type [Target](#associatedtype.Target) = [OcclusionCulling](../occlusion_culling/struct.OcclusionCulling.html "struct bevy::render::occlusion_culling::OcclusionCulling")

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/oit/mod.rs.html#39)

### impl [SyncComponent](trait.SyncComponent.html "trait bevy::render::sync_component::SyncComponent") for [OrderIndependentTransparencySettings](../../core_pipeline/oit/struct.OrderIndependentTransparencySettings.html "struct bevy::core_pipeline::oit::OrderIndependentTransparencySettings")

where [OrderIndependentTransparencySettings](../../core_pipeline/oit/struct.OrderIndependentTransparencySettings.html "struct bevy::core_pipeline::oit::OrderIndependentTransparencySettings"): [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/oit/mod.rs.html#39)

#### type [Target](#associatedtype.Target) = ([OrderIndependentTransparencySettings](../../core_pipeline/oit/struct.OrderIndependentTransparencySettings.html "struct bevy::core_pipeline::oit::OrderIndependentTransparencySettings"), [OrderIndependentTransparencySettingsOffset](../../core_pipeline/oit/struct.OrderIndependentTransparencySettingsOffset.html "struct bevy::core_pipeline::oit::OrderIndependentTransparencySettingsOffset"), [OitResolvePipelineId](../../core_pipeline/oit/resolve/struct.OitResolvePipelineId.html "struct bevy::core_pipeline::oit::resolve::OitResolvePipelineId"))

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/deferred/mod.rs.html#42)

### impl [SyncComponent](trait.SyncComponent.html "trait bevy::render::sync_component::SyncComponent") for [PbrDeferredLightingDepthId](../../pbr/deferred/struct.PbrDeferredLightingDepthId.html "struct bevy::pbr::deferred::PbrDeferredLightingDepthId")

where [PbrDeferredLightingDepthId](../../pbr/deferred/struct.PbrDeferredLightingDepthId.html "struct bevy::pbr::deferred::PbrDeferredLightingDepthId"): [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/deferred/mod.rs.html#42)

#### type [Target](#associatedtype.Target) = [PbrDeferredLightingDepthId](../../pbr/deferred/struct.PbrDeferredLightingDepthId.html "struct bevy::pbr::deferred::PbrDeferredLightingDepthId")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/gpu_readback.rs.html#76)

### impl [SyncComponent](trait.SyncComponent.html "trait bevy::render::sync_component::SyncComponent") for [Readback](../gpu_readback/enum.Readback.html "enum bevy::render::gpu_readback::Readback")

where [Readback](../gpu_readback/enum.Readback.html "enum bevy::render::gpu_readback::Readback"): [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/gpu_readback.rs.html#76)

#### type [Target](#associatedtype.Target) = [Readback](../gpu_readback/enum.Readback.html "enum bevy::render::gpu_readback::Readback")

[Source](https://docs.rs/bevy_dev_tools/0.19.0/x86_64-unknown-linux-gnu/src/bevy_dev_tools/render_debug.rs.html#273)

### impl [SyncComponent](trait.SyncComponent.html "trait bevy::render::sync_component::SyncComponent") for [RenderDebugOverlay](../../dev_tools/render_debug/struct.RenderDebugOverlay.html "struct bevy::dev_tools::render_debug::RenderDebugOverlay")

where [RenderDebugOverlay](../../dev_tools/render_debug/struct.RenderDebugOverlay.html "struct bevy::dev_tools::render_debug::RenderDebugOverlay"): [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

[Source](https://docs.rs/bevy_dev_tools/0.19.0/x86_64-unknown-linux-gnu/src/bevy_dev_tools/render_debug.rs.html#273)

#### type [Target](#associatedtype.Target) = [RenderDebugOverlay](../../dev_tools/render_debug/struct.RenderDebugOverlay.html "struct bevy::dev_tools::render_debug::RenderDebugOverlay")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/ssao/mod.rs.html#111)

### impl [SyncComponent](trait.SyncComponent.html "trait bevy::render::sync_component::SyncComponent") for [ScreenSpaceAmbientOcclusion](../../pbr/struct.ScreenSpaceAmbientOcclusion.html "struct bevy::pbr::ScreenSpaceAmbientOcclusion")

where [ScreenSpaceAmbientOcclusion](../../pbr/struct.ScreenSpaceAmbientOcclusion.html "struct bevy::pbr::ScreenSpaceAmbientOcclusion"): [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/ssao/mod.rs.html#111)

#### type [Target](#associatedtype.Target) = ([ScreenSpaceAmbientOcclusion](../../pbr/struct.ScreenSpaceAmbientOcclusion.html "struct bevy::pbr::ScreenSpaceAmbientOcclusion"), [ScreenSpaceAmbientOcclusionResources](../../pbr/struct.ScreenSpaceAmbientOcclusionResources.html "struct bevy::pbr::ScreenSpaceAmbientOcclusionResources"), [SsaoPipelineId](../../pbr/struct.SsaoPipelineId.html "struct bevy::pbr::SsaoPipelineId"), [SsaoBindGroups](../../pbr/struct.SsaoBindGroups.html "struct bevy::pbr::SsaoBindGroups"))

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/ssr/mod.rs.html#448)

### impl [SyncComponent](trait.SyncComponent.html "trait bevy::render::sync_component::SyncComponent") for [ScreenSpaceReflections](../../pbr/struct.ScreenSpaceReflections.html "struct bevy::pbr::ScreenSpaceReflections")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/ssr/mod.rs.html#449)

#### type [Target](#associatedtype.Target) = ([ScreenSpaceReflectionsUniform](../../pbr/struct.ScreenSpaceReflectionsUniform.html "struct bevy::pbr::ScreenSpaceReflectionsUniform"), [ViewScreenSpaceReflectionsUniformOffset](../../pbr/struct.ViewScreenSpaceReflectionsUniformOffset.html "struct bevy::pbr::ViewScreenSpaceReflectionsUniformOffset"), [ScreenSpaceReflectionsPipelineId](../../pbr/struct.ScreenSpaceReflectionsPipelineId.html "struct bevy::pbr::ScreenSpaceReflectionsPipelineId"))

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/transmission/mod.rs.html#65)

### impl [SyncComponent](trait.SyncComponent.html "trait bevy::render::sync_component::SyncComponent") for [ScreenSpaceTransmission](../../pbr/struct.ScreenSpaceTransmission.html "struct bevy::pbr::ScreenSpaceTransmission")

where [ScreenSpaceTransmission](../../pbr/struct.ScreenSpaceTransmission.html "struct bevy::pbr::ScreenSpaceTransmission"): [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/transmission/mod.rs.html#65)

#### type [Target](#associatedtype.Target) = [ScreenSpaceTransmission](../../pbr/struct.ScreenSpaceTransmission.html "struct bevy::pbr::ScreenSpaceTransmission")

[Source](https://docs.rs/bevy_anti_alias/0.19.0/x86_64-unknown-linux-gnu/src/bevy_anti_alias/smaa/mod.rs.html#84)

### impl [SyncComponent](trait.SyncComponent.html "trait bevy::render::sync_component::SyncComponent") for [Smaa](../../anti_alias/smaa/struct.Smaa.html "struct bevy::anti_alias::smaa::Smaa")

where [Smaa](../../anti_alias/smaa/struct.Smaa.html "struct bevy::anti_alias::smaa::Smaa"): [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

[Source](https://docs.rs/bevy_anti_alias/0.19.0/x86_64-unknown-linux-gnu/src/bevy_anti_alias/smaa/mod.rs.html#84)

#### type [Target](#associatedtype.Target) = ([Smaa](../../anti_alias/smaa/struct.Smaa.html "struct bevy::anti_alias::smaa::Smaa"), [SmaaTextures](../../anti_alias/smaa/struct.SmaaTextures.html "struct bevy::anti_alias::smaa::SmaaTextures"), [SmaaPipelines](../../anti_alias/smaa/struct.SmaaPipelines.html "struct bevy::anti_alias::smaa::SmaaPipelines"), [SmaaBindGroups](../../anti_alias/smaa/struct.SmaaBindGroups.html "struct bevy::anti_alias::smaa::SmaaBindGroups"), [ViewSmaaPipelines](../../anti_alias/smaa/struct.ViewSmaaPipelines.html "struct bevy::anti_alias::smaa::ViewSmaaPipelines"))

[Source](https://docs.rs/bevy_anti_alias/0.19.0/x86_64-unknown-linux-gnu/src/bevy_anti_alias/taa/mod.rs.html#132)

### impl [SyncComponent](trait.SyncComponent.html "trait bevy::render::sync_component::SyncComponent") for [TemporalAntiAliasing](../../anti_alias/taa/struct.TemporalAntiAliasing.html "struct bevy::anti_alias::taa::TemporalAntiAliasing")

[Source](https://docs.rs/bevy_anti_alias/0.19.0/x86_64-unknown-linux-gnu/src/bevy_anti_alias/taa/mod.rs.html#133)

#### type [Target](#associatedtype.Target) = [TemporalAntiAliasing](../../anti_alias/taa/struct.TemporalAntiAliasing.html "struct bevy::anti_alias::taa::TemporalAntiAliasing")

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/tonemapping/mod.rs.html#115)

### impl [SyncComponent](trait.SyncComponent.html "trait bevy::render::sync_component::SyncComponent") for [Tonemapping](../../core_pipeline/tonemapping/enum.Tonemapping.html "enum bevy::core_pipeline::tonemapping::Tonemapping")

where [Tonemapping](../../core_pipeline/tonemapping/enum.Tonemapping.html "enum bevy::core_pipeline::tonemapping::Tonemapping"): [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/tonemapping/mod.rs.html#115)

#### type [Target](#associatedtype.Target) = [Tonemapping](../../core_pipeline/tonemapping/enum.Tonemapping.html "enum bevy::core_pipeline::tonemapping::Tonemapping")

[Source](https://docs.rs/bevy_post_process/0.19.0/x86_64-unknown-linux-gnu/src/bevy_post_process/effect_stack/vignette.rs.html#90)

### impl [SyncComponent](trait.SyncComponent.html "trait bevy::render::sync_component::SyncComponent") for [Vignette](../../post_process/effect_stack/struct.Vignette.html "struct bevy::post_process::effect_stack::Vignette")

[Source](https://docs.rs/bevy_post_process/0.19.0/x86_64-unknown-linux-gnu/src/bevy_post_process/effect_stack/vignette.rs.html#91)

#### type [Target](#associatedtype.Target) = [Vignette](../../post_process/effect_stack/struct.Vignette.html "struct bevy::post_process::effect_stack::Vignette")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/decal/clustered.rs.html#187)

### impl [SyncComponent](trait.SyncComponent.html "trait bevy::render::sync_component::SyncComponent")<[ClusteredDecalPlugin](../../pbr/struct.ClusteredDecalPlugin.html "struct bevy::pbr::ClusteredDecalPlugin")\> for [ClusteredDecal](../../light/struct.ClusteredDecal.html "struct bevy::light::ClusteredDecal")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/decal/clustered.rs.html#188)

#### type [Target](#associatedtype.Target) = [ClusteredDecal](../../light/struct.ClusteredDecal.html "struct bevy::light::ClusteredDecal")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/light_probe/generate.rs.html#1109)

### impl [SyncComponent](trait.SyncComponent.html "trait bevy::render::sync_component::SyncComponent")<[EnvironmentMapGenerationPlugin](../../pbr/generate/struct.EnvironmentMapGenerationPlugin.html "struct bevy::pbr::generate::EnvironmentMapGenerationPlugin")\> for [GeneratedEnvironmentMapLight](../../prelude/struct.GeneratedEnvironmentMapLight.html "struct bevy::prelude::GeneratedEnvironmentMapLight")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/light_probe/generate.rs.html#1110)

#### type [Target](#associatedtype.Target) = [RenderEnvironmentMap](../../pbr/generate/struct.RenderEnvironmentMap.html "struct bevy::pbr::generate::RenderEnvironmentMap")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/lib.rs.html#540)

### impl [SyncComponent](trait.SyncComponent.html "trait bevy::render::sync_component::SyncComponent")<[PbrPlugin](../../pbr/struct.PbrPlugin.html "struct bevy::pbr::PbrPlugin")\> for [AmbientLight](../../prelude/struct.AmbientLight.html "struct bevy::prelude::AmbientLight")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/lib.rs.html#541)

#### type [Target](#associatedtype.Target) = [AmbientLight](../../prelude/struct.AmbientLight.html "struct bevy::prelude::AmbientLight")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/lib.rs.html#510)

### impl [SyncComponent](trait.SyncComponent.html "trait bevy::render::sync_component::SyncComponent")<[PbrPlugin](../../pbr/struct.PbrPlugin.html "struct bevy::pbr::PbrPlugin")\> for [DirectionalLight](../../prelude/struct.DirectionalLight.html "struct bevy::prelude::DirectionalLight")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/lib.rs.html#511)

#### type [Target](#associatedtype.Target) = ([DirectionalLight](../../prelude/struct.DirectionalLight.html "struct bevy::prelude::DirectionalLight"), [ExtractedDirectionalLight](../../pbr/struct.ExtractedDirectionalLight.html "struct bevy::pbr::ExtractedDirectionalLight"), [RenderExtractedShadowMapVisibleEntities](../view/struct.RenderExtractedShadowMapVisibleEntities.html "struct bevy::render::view::RenderExtractedShadowMapVisibleEntities"), [RenderShadowMapVisibleEntities](../view/struct.RenderShadowMapVisibleEntities.html "struct bevy::render::view::RenderShadowMapVisibleEntities"), [DirectionalLightViewEntities](../../pbr/struct.DirectionalLightViewEntities.html "struct bevy::pbr::DirectionalLightViewEntities"))

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/lib.rs.html#519)

### impl [SyncComponent](trait.SyncComponent.html "trait bevy::render::sync_component::SyncComponent")<[PbrPlugin](../../pbr/struct.PbrPlugin.html "struct bevy::pbr::PbrPlugin")\> for [PointLight](../../prelude/struct.PointLight.html "struct bevy::prelude::PointLight")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/lib.rs.html#520)

#### type [Target](#associatedtype.Target) = ([PointLight](../../prelude/struct.PointLight.html "struct bevy::prelude::PointLight"), [ExtractedPointLight](../../pbr/struct.ExtractedPointLight.html "struct bevy::pbr::ExtractedPointLight"), [RenderExtractedShadowMapVisibleEntities](../view/struct.RenderExtractedShadowMapVisibleEntities.html "struct bevy::render::view::RenderExtractedShadowMapVisibleEntities"), [RenderShadowMapVisibleEntities](../view/struct.RenderShadowMapVisibleEntities.html "struct bevy::render::view::RenderShadowMapVisibleEntities"), [PointAndSpotLightViewEntities](../../pbr/struct.PointAndSpotLightViewEntities.html "struct bevy::pbr::PointAndSpotLightViewEntities"))

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/lib.rs.html#537)

### impl [SyncComponent](trait.SyncComponent.html "trait bevy::render::sync_component::SyncComponent")<[PbrPlugin](../../pbr/struct.PbrPlugin.html "struct bevy::pbr::PbrPlugin")\> for [RectLight](../../prelude/struct.RectLight.html "struct bevy::prelude::RectLight")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/lib.rs.html#538)

#### type [Target](#associatedtype.Target) = ([RectLight](../../prelude/struct.RectLight.html "struct bevy::prelude::RectLight"), [ExtractedRectLight](../../pbr/struct.ExtractedRectLight.html "struct bevy::pbr::ExtractedRectLight"))

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/lib.rs.html#543)

### impl [SyncComponent](trait.SyncComponent.html "trait bevy::render::sync_component::SyncComponent")<[PbrPlugin](../../pbr/struct.PbrPlugin.html "struct bevy::pbr::PbrPlugin")\> for [ShadowFilteringMethod](../../light/enum.ShadowFilteringMethod.html "enum bevy::light::ShadowFilteringMethod")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/lib.rs.html#544)

#### type [Target](#associatedtype.Target) = [ShadowFilteringMethod](../../light/enum.ShadowFilteringMethod.html "enum bevy::light::ShadowFilteringMethod")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/lib.rs.html#528)

### impl [SyncComponent](trait.SyncComponent.html "trait bevy::render::sync_component::SyncComponent")<[PbrPlugin](../../pbr/struct.PbrPlugin.html "struct bevy::pbr::PbrPlugin")\> for [SpotLight](../../prelude/struct.SpotLight.html "struct bevy::prelude::SpotLight")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/lib.rs.html#529)

#### type [Target](#associatedtype.Target) = ([SpotLight](../../prelude/struct.SpotLight.html "struct bevy::prelude::SpotLight"), [ExtractedPointLight](../../pbr/struct.ExtractedPointLight.html "struct bevy::pbr::ExtractedPointLight"), [RenderExtractedShadowMapVisibleEntities](../view/struct.RenderExtractedShadowMapVisibleEntities.html "struct bevy::render::view::RenderExtractedShadowMapVisibleEntities"), [RenderShadowMapVisibleEntities](../view/struct.RenderShadowMapVisibleEntities.html "struct bevy::render::view::RenderShadowMapVisibleEntities"), [PointAndSpotLightViewEntities](../../pbr/struct.PointAndSpotLightViewEntities.html "struct bevy::pbr::PointAndSpotLightViewEntities"))

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/skybox/mod.rs.html#62)

### impl [SyncComponent](trait.SyncComponent.html "trait bevy::render::sync_component::SyncComponent")<[SkyboxPlugin](../../core_pipeline/skybox/struct.SkyboxPlugin.html "struct bevy::core_pipeline::skybox::SkyboxPlugin")\> for [Skybox](../../core_pipeline/struct.Skybox.html "struct bevy::core_pipeline::Skybox")

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/skybox/mod.rs.html#63)

#### type [Target](#associatedtype.Target) = ([Skybox](../../core_pipeline/struct.Skybox.html "struct bevy::core_pipeline::Skybox"), [SkyboxUniforms](../../core_pipeline/skybox/struct.SkyboxUniforms.html "struct bevy::core_pipeline::skybox::SkyboxUniforms"), [SkyboxPipelineId](../../core_pipeline/skybox/struct.SkyboxPipelineId.html "struct bevy::core_pipeline::skybox::SkyboxPipelineId"), [SkyboxBindGroup](../../core_pipeline/skybox/struct.SkyboxBindGroup.html "struct bevy::core_pipeline::skybox::SkyboxBindGroup"))

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/volumetric_fog/mod.rs.html#110)

### impl [SyncComponent](trait.SyncComponent.html "trait bevy::render::sync_component::SyncComponent")<[VolumetricFogPlugin](../../pbr/struct.VolumetricFogPlugin.html "struct bevy::pbr::VolumetricFogPlugin")\> for [FogVolume](../../light/struct.FogVolume.html "struct bevy::light::FogVolume")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/volumetric_fog/mod.rs.html#111)

#### type [Target](#associatedtype.Target) = [FogVolume](../../light/struct.FogVolume.html "struct bevy::light::FogVolume")

[Source](https://docs.rs/bevy_ui_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_render/ui_material.rs.html#175)

### impl<M> [SyncComponent](trait.SyncComponent.html "trait bevy::render::sync_component::SyncComponent") for [MaterialNode](../../prelude/struct.MaterialNode.html "struct bevy::prelude::MaterialNode")<M>

where M: [UiMaterial](../../prelude/trait.UiMaterial.html "trait bevy::prelude::UiMaterial"), [MaterialNode](../../prelude/struct.MaterialNode.html "struct bevy::prelude::MaterialNode")<M>: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

[Source](https://docs.rs/bevy_ui_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_render/ui_material.rs.html#175)

#### type [Target](#associatedtype.Target) = [MaterialNode](../../prelude/struct.MaterialNode.html "struct bevy::prelude::MaterialNode")<M>