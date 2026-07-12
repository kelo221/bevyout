[bevy](../../index.html)::[render](../index.html)::[extract\_component](index.html)

# Trait ExtractComponent 

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/extract_component.rs.html#30)

```rust
pub trait ExtractComponent<F = ()>: SyncComponent<F> {
    type QueryData: ReadOnlyQueryData;
    type QueryFilter: QueryFilter;
    type Out: Bundle
       where <Self::Out as DynamicBundle>::Effect: NoBundleEffect;

    // Required method
    fn extract_component(
        item: <Self::QueryData as QueryData>::Item<'_, '_>,
    ) -> Option<Self::Out>;
}
```

Describes how a component gets extracted for rendering.

Therefore the component is transferred from the “app world” into the “render world” in the [`ExtractSchedule`](../../prelude/struct.ExtractSchedule.html "struct bevy::prelude::ExtractSchedule") step. This functionality is enabled by adding [`ExtractComponentPlugin`](struct.ExtractComponentPlugin.html "struct bevy::render::extract_component::ExtractComponentPlugin") with the component type.

The Out type is defined in [`SyncComponent`](../sync_component/trait.SyncComponent.html "trait bevy::render::sync_component::SyncComponent").

The marker type `F` is only used as a way to bypass the orphan rules. To implement the trait for a foreign type you can use a local type as the marker, e.g. the type of the plugin that calls [`ExtractComponentPlugin`](struct.ExtractComponentPlugin.html "struct bevy::render::extract_component::ExtractComponentPlugin").

## Required Associated Types

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/extract_component.rs.html#32)

#### type [QueryData](#associatedtype.QueryData): [ReadOnlyQueryData](../../ecs/query/trait.ReadOnlyQueryData.html "trait bevy::ecs::query::ReadOnlyQueryData")

ECS [`ReadOnlyQueryData`](../../ecs/query/trait.ReadOnlyQueryData.html "trait bevy::ecs::query::ReadOnlyQueryData") to fetch the components to extract.

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/extract_component.rs.html#34)

#### type [QueryFilter](#associatedtype.QueryFilter): [QueryFilter](../../ecs/query/trait.QueryFilter.html "trait bevy::ecs::query::QueryFilter")

Filters the entities with additional constraints.

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/extract_component.rs.html#39)

#### type [Out](#associatedtype.Out): [Bundle](../../prelude/trait.Bundle.html "trait bevy::prelude::Bundle") where <Self::[Out](trait.ExtractComponent.html#associatedtype.Out "type bevy::render::extract_component::ExtractComponent::Out") as [DynamicBundle](../../ecs/bundle/trait.DynamicBundle.html "trait bevy::ecs::bundle::DynamicBundle")\>::[Effect](../../ecs/bundle/trait.DynamicBundle.html#associatedtype.Effect "type bevy::ecs::bundle::DynamicBundle::Effect"): [NoBundleEffect](../../ecs/bundle/trait.NoBundleEffect.html "trait bevy::ecs::bundle::NoBundleEffect")

The output from extraction, i.e. [`ExtractComponent::extract_component`](trait.ExtractComponent.html#tymethod.extract_component "associated function bevy::render::extract_component::ExtractComponent::extract_component").

The output components won’t be removed automatically from the render world if the implementing component is removed, unless you set them in the [`SyncComponent::Target`](../sync_component/trait.SyncComponent.html#associatedtype.Target "associated type bevy::render::sync_component::SyncComponent::Target").

## Required Methods

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/extract_component.rs.html#47)

#### fn [extract\_component](#tymethod.extract_component)( item: <Self::[QueryData](trait.ExtractComponent.html#associatedtype.QueryData "type bevy::render::extract_component::ExtractComponent::QueryData") as [QueryData](../../ecs/query/trait.QueryData.html "trait bevy::ecs::query::QueryData")\>::[Item](../../ecs/query/trait.QueryData.html#associatedtype.Item "type bevy::ecs::query::QueryData::Item")<'\_, '\_>, ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<Self::[Out](trait.ExtractComponent.html#associatedtype.Out "type bevy::render::extract_component::ExtractComponent::Out")\>

Defines how the component is transferred into the “render world”.

Returning `None` based on the queried item will remove the [`SyncComponent::Target`](../sync_component/trait.SyncComponent.html#associatedtype.Target "associated type bevy::render::sync_component::SyncComponent::Target") from the entity in the render world.

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_post_process/0.19.0/x86_64-unknown-linux-gnu/src/bevy_post_process/auto_exposure/settings.rs.html#27)

### impl [ExtractComponent](trait.ExtractComponent.html "trait bevy::render::extract_component::ExtractComponent") for [AutoExposure](../../post_process/auto_exposure/struct.AutoExposure.html "struct bevy::post_process::auto_exposure::AutoExposure")

where [AutoExposure](../../post_process/auto_exposure/struct.AutoExposure.html "struct bevy::post_process::auto_exposure::AutoExposure"): [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

[Source](https://docs.rs/bevy_post_process/0.19.0/x86_64-unknown-linux-gnu/src/bevy_post_process/auto_exposure/settings.rs.html#27)

#### type [QueryData](#associatedtype.QueryData) = &'static [AutoExposure](../../post_process/auto_exposure/struct.AutoExposure.html "struct bevy::post_process::auto_exposure::AutoExposure")

[Source](https://docs.rs/bevy_post_process/0.19.0/x86_64-unknown-linux-gnu/src/bevy_post_process/auto_exposure/settings.rs.html#27)

#### type [QueryFilter](#associatedtype.QueryFilter) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_post_process/0.19.0/x86_64-unknown-linux-gnu/src/bevy_post_process/auto_exposure/settings.rs.html#27)

#### type [Out](#associatedtype.Out) = [AutoExposure](../../post_process/auto_exposure/struct.AutoExposure.html "struct bevy::post_process::auto_exposure::AutoExposure")

[Source](https://docs.rs/bevy_post_process/0.19.0/x86_64-unknown-linux-gnu/src/bevy_post_process/bloom/settings.rs.html#227)

### impl [ExtractComponent](trait.ExtractComponent.html "trait bevy::render::extract_component::ExtractComponent") for [Bloom](../../post_process/bloom/struct.Bloom.html "struct bevy::post_process::bloom::Bloom")

[Source](https://docs.rs/bevy_post_process/0.19.0/x86_64-unknown-linux-gnu/src/bevy_post_process/bloom/settings.rs.html#228)

#### type [QueryData](#associatedtype.QueryData) = (&'static [Bloom](../../post_process/bloom/struct.Bloom.html "struct bevy::post_process::bloom::Bloom"), &'static [Camera](../../prelude/struct.Camera.html "struct bevy::prelude::Camera"))

[Source](https://docs.rs/bevy_post_process/0.19.0/x86_64-unknown-linux-gnu/src/bevy_post_process/bloom/settings.rs.html#229)

#### type [QueryFilter](#associatedtype.QueryFilter) = [With](../../prelude/struct.With.html "struct bevy::prelude::With")<[Hdr](../../camera/struct.Hdr.html "struct bevy::camera::Hdr")\>

[Source](https://docs.rs/bevy_post_process/0.19.0/x86_64-unknown-linux-gnu/src/bevy_post_process/bloom/settings.rs.html#230)

#### type [Out](#associatedtype.Out) = ([Bloom](../../post_process/bloom/struct.Bloom.html "struct bevy::post_process::bloom::Bloom"), BloomUniforms)

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/camera.rs.html#151)

### impl [ExtractComponent](trait.ExtractComponent.html "trait bevy::render::extract_component::ExtractComponent") for [Camera2d](../../prelude/struct.Camera2d.html "struct bevy::prelude::Camera2d")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/camera.rs.html#152)

#### type [QueryData](#associatedtype.QueryData) = &'static [Camera2d](../../prelude/struct.Camera2d.html "struct bevy::prelude::Camera2d")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/camera.rs.html#153)

#### type [QueryFilter](#associatedtype.QueryFilter) = [With](../../prelude/struct.With.html "struct bevy::prelude::With")<[Camera](../../prelude/struct.Camera.html "struct bevy::prelude::Camera")\>

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/camera.rs.html#154)

#### type [Out](#associatedtype.Out) = [Camera2d](../../prelude/struct.Camera2d.html "struct bevy::prelude::Camera2d")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/camera.rs.html#165)

### impl [ExtractComponent](trait.ExtractComponent.html "trait bevy::render::extract_component::ExtractComponent") for [Camera3d](../../prelude/struct.Camera3d.html "struct bevy::prelude::Camera3d")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/camera.rs.html#166)

#### type [QueryData](#associatedtype.QueryData) = &'static [Camera3d](../../prelude/struct.Camera3d.html "struct bevy::prelude::Camera3d")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/camera.rs.html#167)

#### type [QueryFilter](#associatedtype.QueryFilter) = [With](../../prelude/struct.With.html "struct bevy::prelude::With")<[Camera](../../prelude/struct.Camera.html "struct bevy::prelude::Camera")\>

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/camera.rs.html#168)

#### type [Out](#associatedtype.Out) = [Camera3d](../../prelude/struct.Camera3d.html "struct bevy::prelude::Camera3d")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/camera.rs.html#137)

### impl [ExtractComponent](trait.ExtractComponent.html "trait bevy::render::extract_component::ExtractComponent") for [CameraMainTextureUsages](../../camera/struct.CameraMainTextureUsages.html "struct bevy::camera::CameraMainTextureUsages")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/camera.rs.html#138)

#### type [QueryData](#associatedtype.QueryData) = &'static [CameraMainTextureUsages](../../camera/struct.CameraMainTextureUsages.html "struct bevy::camera::CameraMainTextureUsages")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/camera.rs.html#139)

#### type [QueryFilter](#associatedtype.QueryFilter) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/camera.rs.html#140)

#### type [Out](#associatedtype.Out) = [CameraMainTextureUsages](../../camera/struct.CameraMainTextureUsages.html "struct bevy::camera::CameraMainTextureUsages")

[Source](https://docs.rs/bevy_post_process/0.19.0/x86_64-unknown-linux-gnu/src/bevy_post_process/effect_stack/chromatic_aberration.rs.html#85)

### impl [ExtractComponent](trait.ExtractComponent.html "trait bevy::render::extract_component::ExtractComponent") for [ChromaticAberration](../../post_process/effect_stack/struct.ChromaticAberration.html "struct bevy::post_process::effect_stack::ChromaticAberration")

[Source](https://docs.rs/bevy_post_process/0.19.0/x86_64-unknown-linux-gnu/src/bevy_post_process/effect_stack/chromatic_aberration.rs.html#86)

#### type [QueryData](#associatedtype.QueryData) = &'static [ChromaticAberration](../../post_process/effect_stack/struct.ChromaticAberration.html "struct bevy::post_process::effect_stack::ChromaticAberration")

[Source](https://docs.rs/bevy_post_process/0.19.0/x86_64-unknown-linux-gnu/src/bevy_post_process/effect_stack/chromatic_aberration.rs.html#87)

#### type [QueryFilter](#associatedtype.QueryFilter) = [With](../../prelude/struct.With.html "struct bevy::prelude::With")<[Camera](../../prelude/struct.Camera.html "struct bevy::prelude::Camera")\>

[Source](https://docs.rs/bevy_post_process/0.19.0/x86_64-unknown-linux-gnu/src/bevy_post_process/effect_stack/chromatic_aberration.rs.html#88)

#### type [Out](#associatedtype.Out) = [ChromaticAberration](../../post_process/effect_stack/struct.ChromaticAberration.html "struct bevy::post_process::effect_stack::ChromaticAberration")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/contact_shadows.rs.html#86)

### impl [ExtractComponent](trait.ExtractComponent.html "trait bevy::render::extract_component::ExtractComponent") for [ContactShadows](../../pbr/struct.ContactShadows.html "struct bevy::pbr::ContactShadows")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/contact_shadows.rs.html#87)

#### type [QueryData](#associatedtype.QueryData) = &'static [ContactShadows](../../pbr/struct.ContactShadows.html "struct bevy::pbr::ContactShadows")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/contact_shadows.rs.html#88)

#### type [QueryFilter](#associatedtype.QueryFilter) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/contact_shadows.rs.html#89)

#### type [Out](#associatedtype.Out) = [ContactShadows](../../pbr/struct.ContactShadows.html "struct bevy::pbr::ContactShadows")

[Source](https://docs.rs/bevy_anti_alias/0.19.0/x86_64-unknown-linux-gnu/src/bevy_anti_alias/contrast_adaptive_sharpening/mod.rs.html#82)

### impl [ExtractComponent](trait.ExtractComponent.html "trait bevy::render::extract_component::ExtractComponent") for [ContrastAdaptiveSharpening](../../anti_alias/contrast_adaptive_sharpening/struct.ContrastAdaptiveSharpening.html "struct bevy::anti_alias::contrast_adaptive_sharpening::ContrastAdaptiveSharpening")

[Source](https://docs.rs/bevy_anti_alias/0.19.0/x86_64-unknown-linux-gnu/src/bevy_anti_alias/contrast_adaptive_sharpening/mod.rs.html#83)

#### type [QueryData](#associatedtype.QueryData) = &'static [ContrastAdaptiveSharpening](../../anti_alias/contrast_adaptive_sharpening/struct.ContrastAdaptiveSharpening.html "struct bevy::anti_alias::contrast_adaptive_sharpening::ContrastAdaptiveSharpening")

[Source](https://docs.rs/bevy_anti_alias/0.19.0/x86_64-unknown-linux-gnu/src/bevy_anti_alias/contrast_adaptive_sharpening/mod.rs.html#84)

#### type [QueryFilter](#associatedtype.QueryFilter) = [With](../../prelude/struct.With.html "struct bevy::prelude::With")<[Camera](../../prelude/struct.Camera.html "struct bevy::prelude::Camera")\>

[Source](https://docs.rs/bevy_anti_alias/0.19.0/x86_64-unknown-linux-gnu/src/bevy_anti_alias/contrast_adaptive_sharpening/mod.rs.html#85)

#### type [Out](#associatedtype.Out) = ([DenoiseCas](../../anti_alias/contrast_adaptive_sharpening/struct.DenoiseCas.html "struct bevy::anti_alias::contrast_adaptive_sharpening::DenoiseCas"), CasUniform)

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/tonemapping/mod.rs.html#379)

### impl [ExtractComponent](trait.ExtractComponent.html "trait bevy::render::extract_component::ExtractComponent") for [DebandDither](../../core_pipeline/tonemapping/enum.DebandDither.html "enum bevy::core_pipeline::tonemapping::DebandDither")

where [DebandDither](../../core_pipeline/tonemapping/enum.DebandDither.html "enum bevy::core_pipeline::tonemapping::DebandDither"): [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/tonemapping/mod.rs.html#379)

#### type [QueryData](#associatedtype.QueryData) = &'static [DebandDither](../../core_pipeline/tonemapping/enum.DebandDither.html "enum bevy::core_pipeline::tonemapping::DebandDither")

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/tonemapping/mod.rs.html#379)

#### type [QueryFilter](#associatedtype.QueryFilter) = [With](../../prelude/struct.With.html "struct bevy::prelude::With")<[Camera](../../prelude/struct.Camera.html "struct bevy::prelude::Camera")\>

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/tonemapping/mod.rs.html#379)

#### type [Out](#associatedtype.Out) = [DebandDither](../../core_pipeline/tonemapping/enum.DebandDither.html "enum bevy::core_pipeline::tonemapping::DebandDither")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/fog.rs.html#51)

### impl [ExtractComponent](trait.ExtractComponent.html "trait bevy::render::extract_component::ExtractComponent") for [DistanceFog](../../prelude/struct.DistanceFog.html "struct bevy::prelude::DistanceFog")

where [DistanceFog](../../prelude/struct.DistanceFog.html "struct bevy::prelude::DistanceFog"): [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/fog.rs.html#51)

#### type [QueryData](#associatedtype.QueryData) = &'static [DistanceFog](../../prelude/struct.DistanceFog.html "struct bevy::prelude::DistanceFog")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/fog.rs.html#51)

#### type [QueryFilter](#associatedtype.QueryFilter) = [With](../../prelude/struct.With.html "struct bevy::prelude::With")<[Camera](../../prelude/struct.Camera.html "struct bevy::prelude::Camera")\>

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/fog.rs.html#51)

#### type [Out](#associatedtype.Out) = [DistanceFog](../../prelude/struct.DistanceFog.html "struct bevy::prelude::DistanceFog")

[Source](https://docs.rs/bevy_anti_alias/0.19.0/x86_64-unknown-linux-gnu/src/bevy_anti_alias/fxaa/mod.rs.html#53)

### impl [ExtractComponent](trait.ExtractComponent.html "trait bevy::render::extract_component::ExtractComponent") for [Fxaa](../../anti_alias/fxaa/struct.Fxaa.html "struct bevy::anti_alias::fxaa::Fxaa")

where [Fxaa](../../anti_alias/fxaa/struct.Fxaa.html "struct bevy::anti_alias::fxaa::Fxaa"): [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

[Source](https://docs.rs/bevy_anti_alias/0.19.0/x86_64-unknown-linux-gnu/src/bevy_anti_alias/fxaa/mod.rs.html#53)

#### type [QueryData](#associatedtype.QueryData) = &'static [Fxaa](../../anti_alias/fxaa/struct.Fxaa.html "struct bevy::anti_alias::fxaa::Fxaa")

[Source](https://docs.rs/bevy_anti_alias/0.19.0/x86_64-unknown-linux-gnu/src/bevy_anti_alias/fxaa/mod.rs.html#53)

#### type [QueryFilter](#associatedtype.QueryFilter) = [With](../../prelude/struct.With.html "struct bevy::prelude::With")<[Camera](../../prelude/struct.Camera.html "struct bevy::prelude::Camera")\>

[Source](https://docs.rs/bevy_anti_alias/0.19.0/x86_64-unknown-linux-gnu/src/bevy_anti_alias/fxaa/mod.rs.html#53)

#### type [Out](#associatedtype.Out) = [Fxaa](../../anti_alias/fxaa/struct.Fxaa.html "struct bevy::anti_alias::fxaa::Fxaa")

[Source](https://docs.rs/bevy_dev_tools/0.19.0/x86_64-unknown-linux-gnu/src/bevy_dev_tools/render_debug.rs.html#296)

### impl [ExtractComponent](trait.ExtractComponent.html "trait bevy::render::extract_component::ExtractComponent") for [GlobalRenderDebugOverlay](../../dev_tools/render_debug/struct.GlobalRenderDebugOverlay.html "struct bevy::dev_tools::render_debug::GlobalRenderDebugOverlay")

where [GlobalRenderDebugOverlay](../../dev_tools/render_debug/struct.GlobalRenderDebugOverlay.html "struct bevy::dev_tools::render_debug::GlobalRenderDebugOverlay"): [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

[Source](https://docs.rs/bevy_dev_tools/0.19.0/x86_64-unknown-linux-gnu/src/bevy_dev_tools/render_debug.rs.html#296)

#### type [QueryData](#associatedtype.QueryData) = &'static [GlobalRenderDebugOverlay](../../dev_tools/render_debug/struct.GlobalRenderDebugOverlay.html "struct bevy::dev_tools::render_debug::GlobalRenderDebugOverlay")

[Source](https://docs.rs/bevy_dev_tools/0.19.0/x86_64-unknown-linux-gnu/src/bevy_dev_tools/render_debug.rs.html#296)

#### type [QueryFilter](#associatedtype.QueryFilter) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_dev_tools/0.19.0/x86_64-unknown-linux-gnu/src/bevy_dev_tools/render_debug.rs.html#296)

#### type [Out](#associatedtype.Out) = [GlobalRenderDebugOverlay](../../dev_tools/render_debug/struct.GlobalRenderDebugOverlay.html "struct bevy::dev_tools::render_debug::GlobalRenderDebugOverlay")

[Source](https://docs.rs/bevy_post_process/0.19.0/x86_64-unknown-linux-gnu/src/bevy_post_process/effect_stack/lens_distortion.rs.html#85)

### impl [ExtractComponent](trait.ExtractComponent.html "trait bevy::render::extract_component::ExtractComponent") for [LensDistortion](../../post_process/effect_stack/struct.LensDistortion.html "struct bevy::post_process::effect_stack::LensDistortion")

[Source](https://docs.rs/bevy_post_process/0.19.0/x86_64-unknown-linux-gnu/src/bevy_post_process/effect_stack/lens_distortion.rs.html#86)

#### type [QueryData](#associatedtype.QueryData) = &'static [LensDistortion](../../post_process/effect_stack/struct.LensDistortion.html "struct bevy::post_process::effect_stack::LensDistortion")

[Source](https://docs.rs/bevy_post_process/0.19.0/x86_64-unknown-linux-gnu/src/bevy_post_process/effect_stack/lens_distortion.rs.html#87)

#### type [QueryFilter](#associatedtype.QueryFilter) = [With](../../prelude/struct.With.html "struct bevy::prelude::With")<[LensDistortion](../../post_process/effect_stack/struct.LensDistortion.html "struct bevy::post_process::effect_stack::LensDistortion")\>

[Source](https://docs.rs/bevy_post_process/0.19.0/x86_64-unknown-linux-gnu/src/bevy_post_process/effect_stack/lens_distortion.rs.html#88)

#### type [Out](#associatedtype.Out) = [LensDistortion](../../post_process/effect_stack/struct.LensDistortion.html "struct bevy::post_process::effect_stack::LensDistortion")

[Source](https://docs.rs/bevy_post_process/0.19.0/x86_64-unknown-linux-gnu/src/bevy_post_process/motion_blur/mod.rs.html#122)

### impl [ExtractComponent](trait.ExtractComponent.html "trait bevy::render::extract_component::ExtractComponent") for [MotionBlur](../../post_process/motion_blur/struct.MotionBlur.html "struct bevy::post_process::motion_blur::MotionBlur")

[Source](https://docs.rs/bevy_post_process/0.19.0/x86_64-unknown-linux-gnu/src/bevy_post_process/motion_blur/mod.rs.html#123)

#### type [QueryData](#associatedtype.QueryData) = &'static [MotionBlur](../../post_process/motion_blur/struct.MotionBlur.html "struct bevy::post_process::motion_blur::MotionBlur")

[Source](https://docs.rs/bevy_post_process/0.19.0/x86_64-unknown-linux-gnu/src/bevy_post_process/motion_blur/mod.rs.html#124)

#### type [QueryFilter](#associatedtype.QueryFilter) = [With](../../prelude/struct.With.html "struct bevy::prelude::With")<[Camera](../../prelude/struct.Camera.html "struct bevy::prelude::Camera")\>

[Source](https://docs.rs/bevy_post_process/0.19.0/x86_64-unknown-linux-gnu/src/bevy_post_process/motion_blur/mod.rs.html#125)

#### type [Out](#associatedtype.Out) = MotionBlurUniform

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/view/mod.rs.html#230)

### impl [ExtractComponent](trait.ExtractComponent.html "trait bevy::render::extract_component::ExtractComponent") for [Msaa](../../prelude/enum.Msaa.html "enum bevy::prelude::Msaa")

where [Msaa](../../prelude/enum.Msaa.html "enum bevy::prelude::Msaa"): [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/view/mod.rs.html#230)

#### type [QueryData](#associatedtype.QueryData) = &'static [Msaa](../../prelude/enum.Msaa.html "enum bevy::prelude::Msaa")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/view/mod.rs.html#230)

#### type [QueryFilter](#associatedtype.QueryFilter) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/view/mod.rs.html#230)

#### type [Out](#associatedtype.Out) = [Msaa](../../prelude/enum.Msaa.html "enum bevy::prelude::Msaa")

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/prepass/background_motion_vectors.rs.html#63)

### impl [ExtractComponent](trait.ExtractComponent.html "trait bevy::render::extract_component::ExtractComponent") for [NoBackgroundMotionVectors](../../core_pipeline/prepass/struct.NoBackgroundMotionVectors.html "struct bevy::core_pipeline::prepass::NoBackgroundMotionVectors")

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/prepass/background_motion_vectors.rs.html#64)

#### type [QueryData](#associatedtype.QueryData) = &'static [NoBackgroundMotionVectors](../../core_pipeline/prepass/struct.NoBackgroundMotionVectors.html "struct bevy::core_pipeline::prepass::NoBackgroundMotionVectors")

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/prepass/background_motion_vectors.rs.html#65)

#### type [QueryFilter](#associatedtype.QueryFilter) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/prepass/background_motion_vectors.rs.html#66)

#### type [Out](#associatedtype.Out) = [NoBackgroundMotionVectors](../../core_pipeline/prepass/struct.NoBackgroundMotionVectors.html "struct bevy::core_pipeline::prepass::NoBackgroundMotionVectors")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/occlusion_culling/mod.rs.html#70)

### impl [ExtractComponent](trait.ExtractComponent.html "trait bevy::render::extract_component::ExtractComponent") for [OcclusionCulling](../occlusion_culling/struct.OcclusionCulling.html "struct bevy::render::occlusion_culling::OcclusionCulling")

where [OcclusionCulling](../occlusion_culling/struct.OcclusionCulling.html "struct bevy::render::occlusion_culling::OcclusionCulling"): [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/occlusion_culling/mod.rs.html#70)

#### type [QueryData](#associatedtype.QueryData) = &'static [OcclusionCulling](../occlusion_culling/struct.OcclusionCulling.html "struct bevy::render::occlusion_culling::OcclusionCulling")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/occlusion_culling/mod.rs.html#70)

#### type [QueryFilter](#associatedtype.QueryFilter) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/occlusion_culling/mod.rs.html#70)

#### type [Out](#associatedtype.Out) = [OcclusionCulling](../occlusion_culling/struct.OcclusionCulling.html "struct bevy::render::occlusion_culling::OcclusionCulling")

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/oit/mod.rs.html#39)

### impl [ExtractComponent](trait.ExtractComponent.html "trait bevy::render::extract_component::ExtractComponent") for [OrderIndependentTransparencySettings](../../core_pipeline/oit/struct.OrderIndependentTransparencySettings.html "struct bevy::core_pipeline::oit::OrderIndependentTransparencySettings")

where [OrderIndependentTransparencySettings](../../core_pipeline/oit/struct.OrderIndependentTransparencySettings.html "struct bevy::core_pipeline::oit::OrderIndependentTransparencySettings"): [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/oit/mod.rs.html#39)

#### type [QueryData](#associatedtype.QueryData) = &'static [OrderIndependentTransparencySettings](../../core_pipeline/oit/struct.OrderIndependentTransparencySettings.html "struct bevy::core_pipeline::oit::OrderIndependentTransparencySettings")

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/oit/mod.rs.html#39)

#### type [QueryFilter](#associatedtype.QueryFilter) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/oit/mod.rs.html#39)

#### type [Out](#associatedtype.Out) = [OrderIndependentTransparencySettings](../../core_pipeline/oit/struct.OrderIndependentTransparencySettings.html "struct bevy::core_pipeline::oit::OrderIndependentTransparencySettings")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/deferred/mod.rs.html#42)

### impl [ExtractComponent](trait.ExtractComponent.html "trait bevy::render::extract_component::ExtractComponent") for [PbrDeferredLightingDepthId](../../pbr/deferred/struct.PbrDeferredLightingDepthId.html "struct bevy::pbr::deferred::PbrDeferredLightingDepthId")

where [PbrDeferredLightingDepthId](../../pbr/deferred/struct.PbrDeferredLightingDepthId.html "struct bevy::pbr::deferred::PbrDeferredLightingDepthId"): [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/deferred/mod.rs.html#42)

#### type [QueryData](#associatedtype.QueryData) = &'static [PbrDeferredLightingDepthId](../../pbr/deferred/struct.PbrDeferredLightingDepthId.html "struct bevy::pbr::deferred::PbrDeferredLightingDepthId")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/deferred/mod.rs.html#42)

#### type [QueryFilter](#associatedtype.QueryFilter) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/deferred/mod.rs.html#42)

#### type [Out](#associatedtype.Out) = [PbrDeferredLightingDepthId](../../pbr/deferred/struct.PbrDeferredLightingDepthId.html "struct bevy::pbr::deferred::PbrDeferredLightingDepthId")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/gpu_readback.rs.html#76)

### impl [ExtractComponent](trait.ExtractComponent.html "trait bevy::render::extract_component::ExtractComponent") for [Readback](../gpu_readback/enum.Readback.html "enum bevy::render::gpu_readback::Readback")

where [Readback](../gpu_readback/enum.Readback.html "enum bevy::render::gpu_readback::Readback"): [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/gpu_readback.rs.html#76)

#### type [QueryData](#associatedtype.QueryData) = &'static [Readback](../gpu_readback/enum.Readback.html "enum bevy::render::gpu_readback::Readback")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/gpu_readback.rs.html#76)

#### type [QueryFilter](#associatedtype.QueryFilter) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/gpu_readback.rs.html#76)

#### type [Out](#associatedtype.Out) = [Readback](../gpu_readback/enum.Readback.html "enum bevy::render::gpu_readback::Readback")

[Source](https://docs.rs/bevy_dev_tools/0.19.0/x86_64-unknown-linux-gnu/src/bevy_dev_tools/render_debug.rs.html#273)

### impl [ExtractComponent](trait.ExtractComponent.html "trait bevy::render::extract_component::ExtractComponent") for [RenderDebugOverlay](../../dev_tools/render_debug/struct.RenderDebugOverlay.html "struct bevy::dev_tools::render_debug::RenderDebugOverlay")

where [RenderDebugOverlay](../../dev_tools/render_debug/struct.RenderDebugOverlay.html "struct bevy::dev_tools::render_debug::RenderDebugOverlay"): [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

[Source](https://docs.rs/bevy_dev_tools/0.19.0/x86_64-unknown-linux-gnu/src/bevy_dev_tools/render_debug.rs.html#273)

#### type [QueryData](#associatedtype.QueryData) = &'static [RenderDebugOverlay](../../dev_tools/render_debug/struct.RenderDebugOverlay.html "struct bevy::dev_tools::render_debug::RenderDebugOverlay")

[Source](https://docs.rs/bevy_dev_tools/0.19.0/x86_64-unknown-linux-gnu/src/bevy_dev_tools/render_debug.rs.html#273)

#### type [QueryFilter](#associatedtype.QueryFilter) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_dev_tools/0.19.0/x86_64-unknown-linux-gnu/src/bevy_dev_tools/render_debug.rs.html#273)

#### type [Out](#associatedtype.Out) = [RenderDebugOverlay](../../dev_tools/render_debug/struct.RenderDebugOverlay.html "struct bevy::dev_tools::render_debug::RenderDebugOverlay")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/ssao/mod.rs.html#111)

### impl [ExtractComponent](trait.ExtractComponent.html "trait bevy::render::extract_component::ExtractComponent") for [ScreenSpaceAmbientOcclusion](../../pbr/struct.ScreenSpaceAmbientOcclusion.html "struct bevy::pbr::ScreenSpaceAmbientOcclusion")

where [ScreenSpaceAmbientOcclusion](../../pbr/struct.ScreenSpaceAmbientOcclusion.html "struct bevy::pbr::ScreenSpaceAmbientOcclusion"): [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/ssao/mod.rs.html#111)

#### type [QueryData](#associatedtype.QueryData) = &'static [ScreenSpaceAmbientOcclusion](../../pbr/struct.ScreenSpaceAmbientOcclusion.html "struct bevy::pbr::ScreenSpaceAmbientOcclusion")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/ssao/mod.rs.html#111)

#### type [QueryFilter](#associatedtype.QueryFilter) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/ssao/mod.rs.html#111)

#### type [Out](#associatedtype.Out) = [ScreenSpaceAmbientOcclusion](../../pbr/struct.ScreenSpaceAmbientOcclusion.html "struct bevy::pbr::ScreenSpaceAmbientOcclusion")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/ssr/mod.rs.html#456)

### impl [ExtractComponent](trait.ExtractComponent.html "trait bevy::render::extract_component::ExtractComponent") for [ScreenSpaceReflections](../../pbr/struct.ScreenSpaceReflections.html "struct bevy::pbr::ScreenSpaceReflections")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/ssr/mod.rs.html#457)

#### type [QueryData](#associatedtype.QueryData) = &'static [ScreenSpaceReflections](../../pbr/struct.ScreenSpaceReflections.html "struct bevy::pbr::ScreenSpaceReflections")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/ssr/mod.rs.html#458)

#### type [QueryFilter](#associatedtype.QueryFilter) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/ssr/mod.rs.html#459)

#### type [Out](#associatedtype.Out) = [ScreenSpaceReflectionsUniform](../../pbr/struct.ScreenSpaceReflectionsUniform.html "struct bevy::pbr::ScreenSpaceReflectionsUniform")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/transmission/mod.rs.html#65)

### impl [ExtractComponent](trait.ExtractComponent.html "trait bevy::render::extract_component::ExtractComponent") for [ScreenSpaceTransmission](../../pbr/struct.ScreenSpaceTransmission.html "struct bevy::pbr::ScreenSpaceTransmission")

where [ScreenSpaceTransmission](../../pbr/struct.ScreenSpaceTransmission.html "struct bevy::pbr::ScreenSpaceTransmission"): [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/transmission/mod.rs.html#65)

#### type [QueryData](#associatedtype.QueryData) = &'static [ScreenSpaceTransmission](../../pbr/struct.ScreenSpaceTransmission.html "struct bevy::pbr::ScreenSpaceTransmission")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/transmission/mod.rs.html#65)

#### type [QueryFilter](#associatedtype.QueryFilter) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/transmission/mod.rs.html#65)

#### type [Out](#associatedtype.Out) = [ScreenSpaceTransmission](../../pbr/struct.ScreenSpaceTransmission.html "struct bevy::pbr::ScreenSpaceTransmission")

[Source](https://docs.rs/bevy_anti_alias/0.19.0/x86_64-unknown-linux-gnu/src/bevy_anti_alias/smaa/mod.rs.html#84)

### impl [ExtractComponent](trait.ExtractComponent.html "trait bevy::render::extract_component::ExtractComponent") for [Smaa](../../anti_alias/smaa/struct.Smaa.html "struct bevy::anti_alias::smaa::Smaa")

where [Smaa](../../anti_alias/smaa/struct.Smaa.html "struct bevy::anti_alias::smaa::Smaa"): [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

[Source](https://docs.rs/bevy_anti_alias/0.19.0/x86_64-unknown-linux-gnu/src/bevy_anti_alias/smaa/mod.rs.html#84)

#### type [QueryData](#associatedtype.QueryData) = &'static [Smaa](../../anti_alias/smaa/struct.Smaa.html "struct bevy::anti_alias::smaa::Smaa")

[Source](https://docs.rs/bevy_anti_alias/0.19.0/x86_64-unknown-linux-gnu/src/bevy_anti_alias/smaa/mod.rs.html#84)

#### type [QueryFilter](#associatedtype.QueryFilter) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_anti_alias/0.19.0/x86_64-unknown-linux-gnu/src/bevy_anti_alias/smaa/mod.rs.html#84)

#### type [Out](#associatedtype.Out) = [Smaa](../../anti_alias/smaa/struct.Smaa.html "struct bevy::anti_alias::smaa::Smaa")

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/tonemapping/mod.rs.html#115)

### impl [ExtractComponent](trait.ExtractComponent.html "trait bevy::render::extract_component::ExtractComponent") for [Tonemapping](../../core_pipeline/tonemapping/enum.Tonemapping.html "enum bevy::core_pipeline::tonemapping::Tonemapping")

where [Tonemapping](../../core_pipeline/tonemapping/enum.Tonemapping.html "enum bevy::core_pipeline::tonemapping::Tonemapping"): [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/tonemapping/mod.rs.html#115)

#### type [QueryData](#associatedtype.QueryData) = &'static [Tonemapping](../../core_pipeline/tonemapping/enum.Tonemapping.html "enum bevy::core_pipeline::tonemapping::Tonemapping")

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/tonemapping/mod.rs.html#115)

#### type [QueryFilter](#associatedtype.QueryFilter) = [With](../../prelude/struct.With.html "struct bevy::prelude::With")<[Camera](../../prelude/struct.Camera.html "struct bevy::prelude::Camera")\>

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/tonemapping/mod.rs.html#115)

#### type [Out](#associatedtype.Out) = [Tonemapping](../../core_pipeline/tonemapping/enum.Tonemapping.html "enum bevy::core_pipeline::tonemapping::Tonemapping")

[Source](https://docs.rs/bevy_post_process/0.19.0/x86_64-unknown-linux-gnu/src/bevy_post_process/effect_stack/vignette.rs.html#94)

### impl [ExtractComponent](trait.ExtractComponent.html "trait bevy::render::extract_component::ExtractComponent") for [Vignette](../../post_process/effect_stack/struct.Vignette.html "struct bevy::post_process::effect_stack::Vignette")

[Source](https://docs.rs/bevy_post_process/0.19.0/x86_64-unknown-linux-gnu/src/bevy_post_process/effect_stack/vignette.rs.html#95)

#### type [QueryData](#associatedtype.QueryData) = &'static [Vignette](../../post_process/effect_stack/struct.Vignette.html "struct bevy::post_process::effect_stack::Vignette")

[Source](https://docs.rs/bevy_post_process/0.19.0/x86_64-unknown-linux-gnu/src/bevy_post_process/effect_stack/vignette.rs.html#96)

#### type [QueryFilter](#associatedtype.QueryFilter) = [With](../../prelude/struct.With.html "struct bevy::prelude::With")<[Camera](../../prelude/struct.Camera.html "struct bevy::prelude::Camera")\>

[Source](https://docs.rs/bevy_post_process/0.19.0/x86_64-unknown-linux-gnu/src/bevy_post_process/effect_stack/vignette.rs.html#97)

#### type [Out](#associatedtype.Out) = [Vignette](../../post_process/effect_stack/struct.Vignette.html "struct bevy::post_process::effect_stack::Vignette")

[Source](https://docs.rs/bevy_ui_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_render/ui_material.rs.html#175)

### impl<M> [ExtractComponent](trait.ExtractComponent.html "trait bevy::render::extract_component::ExtractComponent") for [MaterialNode](../../prelude/struct.MaterialNode.html "struct bevy::prelude::MaterialNode")<M>

where M: [UiMaterial](../../prelude/trait.UiMaterial.html "trait bevy::prelude::UiMaterial"), [MaterialNode](../../prelude/struct.MaterialNode.html "struct bevy::prelude::MaterialNode")<M>: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

[Source](https://docs.rs/bevy_ui_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_render/ui_material.rs.html#175)

#### type [QueryData](#associatedtype.QueryData) = &'static [MaterialNode](../../prelude/struct.MaterialNode.html "struct bevy::prelude::MaterialNode")<M>

[Source](https://docs.rs/bevy_ui_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_render/ui_material.rs.html#175)

#### type [QueryFilter](#associatedtype.QueryFilter) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_ui_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_render/ui_material.rs.html#175)

#### type [Out](#associatedtype.Out) = [MaterialNode](../../prelude/struct.MaterialNode.html "struct bevy::prelude::MaterialNode")<M>