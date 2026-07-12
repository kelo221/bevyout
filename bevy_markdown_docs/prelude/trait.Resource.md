[bevy](../index.html)::[prelude](index.html)

# Trait Resource 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/resource.rs.html#87)

```rust
pub trait Resource: Component { }
```

A type that can be inserted into a [`World`](struct.World.html "struct bevy::prelude::World") as a singleton.

You can access resource data in systems using the [`Res`](struct.Res.html "struct bevy::prelude::Res") and [`ResMut`](struct.ResMut.html "struct bevy::prelude::ResMut") system parameters

Only one resource of each type can be stored in a [`World`](struct.World.html "struct bevy::prelude::World") at any given time.

## Examples

```rust
#[derive(Resource)]
struct MyResource { value: u32 }

world.insert_resource(MyResource { value: 42 });

fn read_resource_system(resource: Res<MyResource>) {
    assert_eq!(resource.value, 42);
}

fn write_resource_system(mut resource: ResMut<MyResource>) {
    assert_eq!(resource.value, 42);
    resource.value = 0;
    assert_eq!(resource.value, 0);
}
```

## `!Sync` Resources

A `!Sync` type cannot implement `Resource`. However, it is possible to wrap a `Send` but not `Sync` type in [`SyncCell`](../platform/cell/struct.SyncCell.html "struct bevy::platform::cell::SyncCell") or the currently unstable [`Exclusive`](https://doc.rust-lang.org/nightly/std/sync/struct.Exclusive.html) to make it `Sync`. This forces only having mutable access (`&mut T` only, never `&T`), but makes it safe to reference across multiple threads.

This will fail to compile since `RefCell` is `!Sync`.

[ⓘ](# "This example deliberately fails to compile")

```rust
#[derive(Resource)]
struct NotSync {
   counter: RefCell<usize>,
}
```

This will compile since the `RefCell` is wrapped with `SyncCell`.

```rust
use bevy_platform::cell::SyncCell;

#[derive(Resource)]
struct ActuallySync {
   counter: SyncCell<RefCell<usize>>,
}
```

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_a11y/0.19.0/x86_64-unknown-linux-gnu/src/bevy_a11y/lib.rs.html#107)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [AccessibilityRequested](../a11y/struct.AccessibilityRequested.html "struct bevy::a11y::AccessibilityRequested")

where [AccessibilityRequested](../a11y/struct.AccessibilityRequested.html "struct bevy::a11y::AccessibilityRequested"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/mouse.rs.html#207)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [AccumulatedMouseMotion](../input/mouse/struct.AccumulatedMouseMotion.html "struct bevy::input::mouse::AccumulatedMouseMotion")

where [AccumulatedMouseMotion](../input/mouse/struct.AccumulatedMouseMotion.html "struct bevy::input::mouse::AccumulatedMouseMotion"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/mouse.rs.html#228)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [AccumulatedMouseScroll](../input/mouse/struct.AccumulatedMouseScroll.html "struct bevy::input::mouse::AccumulatedMouseScroll")

where [AccumulatedMouseScroll](../input/mouse/struct.AccumulatedMouseScroll.html "struct bevy::input::mouse::AccumulatedMouseScroll"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/renderer/raw_vulkan_init.rs.html#137)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [AdditionalVulkanFeatures](../render/renderer/raw_vulkan_init/struct.AdditionalVulkanFeatures.html "struct bevy::render::renderer::raw_vulkan_init::AdditionalVulkanFeatures")

where [AdditionalVulkanFeatures](../render/renderer/raw_vulkan_init/struct.AdditionalVulkanFeatures.html "struct bevy::render::renderer::raw_vulkan_init::AdditionalVulkanFeatures"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/reflect/mod.rs.html#71)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [AppFunctionRegistry](struct.AppFunctionRegistry.html "struct bevy::prelude::AppFunctionRegistry")

where [AppFunctionRegistry](struct.AppFunctionRegistry.html "struct bevy::prelude::AppFunctionRegistry"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/reflect/mod.rs.html#35)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [AppTypeRegistry](struct.AppTypeRegistry.html "struct bevy::prelude::AppTypeRegistry")

where [AppTypeRegistry](struct.AppTypeRegistry.html "struct bevy::prelude::AppTypeRegistry"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/lib.rs.html#177)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [AreaLightLuts](../pbr/struct.AreaLightLuts.html "struct bevy::pbr::AreaLightLuts")

where [AreaLightLuts](../pbr/struct.AreaLightLuts.html "struct bevy::pbr::AreaLightLuts"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/processor/mod.rs.html#97)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [AssetProcessor](../asset/processor/struct.AssetProcessor.html "struct bevy::asset::processor::AssetProcessor")

where [AssetProcessor](../asset/processor/struct.AssetProcessor.html "struct bevy::asset::processor::AssetProcessor"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/server/mod.rs.html#65)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [AssetServer](struct.AssetServer.html "struct bevy::prelude::AssetServer")

where [AssetServer](struct.AssetServer.html "struct bevy::prelude::AssetServer"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/io/source.rs.html#328)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [AssetSourceBuilders](../asset/io/struct.AssetSourceBuilders.html "struct bevy::asset::io::AssetSourceBuilders")

where [AssetSourceBuilders](../asset/io/struct.AssetSourceBuilders.html "struct bevy::asset::io::AssetSourceBuilders"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/atmosphere/resources.rs.html#806)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [AtmosphereBuffer](../pbr/resources/struct.AtmosphereBuffer.html "struct bevy::pbr::resources::AtmosphereBuffer")

where [AtmosphereBuffer](../pbr/resources/struct.AtmosphereBuffer.html "struct bevy::pbr::resources::AtmosphereBuffer"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/atmosphere/resources.rs.html#231)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [AtmosphereSampler](../pbr/resources/struct.AtmosphereSampler.html "struct bevy::pbr::resources::AtmosphereSampler")

where [AtmosphereSampler](../pbr/resources/struct.AtmosphereSampler.html "struct bevy::pbr::resources::AtmosphereSampler"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/atmosphere/resources.rs.html#500)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [AtmosphereTransforms](../pbr/resources/struct.AtmosphereTransforms.html "struct bevy::pbr::resources::AtmosphereTransforms")

where [AtmosphereTransforms](../pbr/resources/struct.AtmosphereTransforms.html "struct bevy::pbr::resources::AtmosphereTransforms"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_input_focus/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input_focus/directional_navigation.rs.html#87)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [AutoNavigationConfig](../input_focus/directional_navigation/struct.AutoNavigationConfig.html "struct bevy::input_focus::directional_navigation::AutoNavigationConfig")

where [AutoNavigationConfig](../input_focus/directional_navigation/struct.AutoNavigationConfig.html "struct bevy::input_focus::directional_navigation::AutoNavigationConfig"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/gpu_preprocess.rs.html#353)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [BinUnpackingBindGroups](../pbr/struct.BinUnpackingBindGroups.html "struct bevy::pbr::BinUnpackingBindGroups")

where [BinUnpackingBindGroups](../pbr/struct.BinUnpackingBindGroups.html "struct bevy::pbr::BinUnpackingBindGroups"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/batching/gpu_preprocessing.rs.html#1127)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [BinUnpackingBuffers](../render/batching/gpu_preprocessing/struct.BinUnpackingBuffers.html "struct bevy::render::batching::gpu_preprocessing::BinUnpackingBuffers")

where [BinUnpackingBuffers](../render/batching/gpu_preprocessing/struct.BinUnpackingBuffers.html "struct bevy::render::batching::gpu_preprocessing::BinUnpackingBuffers"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/blit/mod.rs.html#35)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [BlitPipeline](../core_pipeline/blit/struct.BlitPipeline.html "struct bevy::core_pipeline::blit::BlitPipeline")

where [BlitPipeline](../core_pipeline/blit/struct.BlitPipeline.html "struct bevy::core_pipeline::blit::BlitPipeline"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/lib.rs.html#164)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [Bluenoise](../pbr/struct.Bluenoise.html "struct bevy::pbr::Bluenoise")

where [Bluenoise](../pbr/struct.Bluenoise.html "struct bevy::pbr::Bluenoise"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_ui_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_render/box_shadow.rs.html#88)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [BoxShadowMeta](../ui_render/box_shadow/struct.BoxShadowMeta.html "struct bevy::ui_render::box_shadow::BoxShadowMeta")

where [BoxShadowMeta](../ui_render/box_shadow/struct.BoxShadowMeta.html "struct bevy::ui_render::box_shadow::BoxShadowMeta"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_ui_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_render/box_shadow.rs.html#105)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [BoxShadowPipeline](../ui_render/box_shadow/struct.BoxShadowPipeline.html "struct bevy::ui_render::box_shadow::BoxShadowPipeline")

where [BoxShadowPipeline](../ui_render/box_shadow/struct.BoxShadowPipeline.html "struct bevy::ui_render::box_shadow::BoxShadowPipeline"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_remote/0.19.0/x86_64-unknown-linux-gnu/src/bevy_remote/builtin_methods.rs.html#1557)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [BrpEventObservers](../remote/builtin_methods/struct.BrpEventObservers.html "struct bevy::remote::builtin_methods::BrpEventObservers")

where [BrpEventObservers](../remote/builtin_methods/struct.BrpEventObservers.html "struct bevy::remote::builtin_methods::BrpEventObservers"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_remote/0.19.0/x86_64-unknown-linux-gnu/src/bevy_remote/lib.rs.html#1467)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [BrpReceiver](../remote/struct.BrpReceiver.html "struct bevy::remote::BrpReceiver")

where [BrpReceiver](../remote/struct.BrpReceiver.html "struct bevy::remote::BrpReceiver"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_remote/0.19.0/x86_64-unknown-linux-gnu/src/bevy_remote/lib.rs.html#1460)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [BrpSender](../remote/struct.BrpSender.html "struct bevy::remote::BrpSender")

where [BrpSender](../remote/struct.BrpSender.html "struct bevy::remote::BrpSender"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/gpu_preprocess.rs.html#321)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [BuildIndirectParametersBindGroups](../pbr/struct.BuildIndirectParametersBindGroups.html "struct bevy::pbr::BuildIndirectParametersBindGroups")

where [BuildIndirectParametersBindGroups](../pbr/struct.BuildIndirectParametersBindGroups.html "struct bevy::pbr::BuildIndirectParametersBindGroups"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/camera.rs.html#57)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [CameraMainPassTextureFormats](../render/camera/struct.CameraMainPassTextureFormats.html "struct bevy::render::camera::CameraMainPassTextureFormats")

where [CameraMainPassTextureFormats](../render/camera/struct.CameraMainPassTextureFormats.html "struct bevy::render::camera::CameraMainPassTextureFormats"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/view/window/screenshot.rs.html#121)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [CapturedScreenshots](../render/view/window/screenshot/struct.CapturedScreenshots.html "struct bevy::render::view::window::screenshot::CapturedScreenshots")

where [CapturedScreenshots](../render/view/window/screenshot/struct.CapturedScreenshots.html "struct bevy::render::view::window::screenshot::CapturedScreenshots"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_anti_alias/0.19.0/x86_64-unknown-linux-gnu/src/bevy_anti_alias/contrast_adaptive_sharpening/mod.rs.html#135)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [CasPipeline](../anti_alias/contrast_adaptive_sharpening/struct.CasPipeline.html "struct bevy::anti_alias::contrast_adaptive_sharpening::CasPipeline")

where [CasPipeline](../anti_alias/contrast_adaptive_sharpening/struct.CasPipeline.html "struct bevy::anti_alias::contrast_adaptive_sharpening::CasPipeline"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_dev_tools/0.19.0/x86_64-unknown-linux-gnu/src/bevy_dev_tools/ci_testing/config.rs.html#10)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [CiTestingConfig](../dev_tools/ci_testing/struct.CiTestingConfig.html "struct bevy::dev_tools::ci_testing::CiTestingConfig")

where [CiTestingConfig](../dev_tools/ci_testing/struct.CiTestingConfig.html "struct bevy::dev_tools::ci_testing::CiTestingConfig"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/clear_color.rs.html#53)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [ClearColor](struct.ClearColor.html "struct bevy::prelude::ClearColor")

where [ClearColor](struct.ClearColor.html "struct bevy::prelude::ClearColor"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_clipboard/0.19.0/x86_64-unknown-linux-gnu/src/bevy_clipboard/lib.rs.html#190)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [Clipboard](struct.Clipboard.html "struct bevy::prelude::Clipboard")

where [Clipboard](struct.Clipboard.html "struct bevy::prelude::Clipboard"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/image.rs.html#2439)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [CompressedImageFormatSupport](../image/struct.CompressedImageFormatSupport.html "struct bevy::image::CompressedImageFormatSupport")

where [CompressedImageFormatSupport](../image/struct.CompressedImageFormatSupport.html "struct bevy::image::CompressedImageFormatSupport"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/contact_shadows.rs.html#97)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [ContactShadowsBuffer](../pbr/struct.ContactShadowsBuffer.html "struct bevy::pbr::ContactShadowsBuffer")

where [ContactShadowsBuffer](../pbr/struct.ContactShadowsBuffer.html "struct bevy::pbr::ContactShadowsBuffer"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/renderer/render_context.rs.html#204)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [CurrentView](../render/renderer/struct.CurrentView.html "struct bevy::render::renderer::CurrentView")

where [CurrentView](../render/renderer/struct.CurrentView.html "struct bevy::render::renderer::CurrentView"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_dev_tools/0.19.0/x86_64-unknown-linux-gnu/src/bevy_dev_tools/picking_debug.rs.html#21)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [DebugPickingMode](../dev_tools/picking_debug/enum.DebugPickingMode.html "enum bevy::dev_tools::picking_debug::DebugPickingMode")

where [DebugPickingMode](../dev_tools/picking_debug/enum.DebugPickingMode.html "enum bevy::dev_tools::picking_debug::DebugPickingMode"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/decal/clustered.rs.html#151)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [DecalsBuffer](../pbr/decal/clustered/struct.DecalsBuffer.html "struct bevy::pbr::decal::clustered::DecalsBuffer")

where [DecalsBuffer](../pbr/decal/clustered/struct.DecalsBuffer.html "struct bevy::pbr::decal::clustered::DecalsBuffer"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/cursor.rs.html#23)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [DefaultCursor](../feathers/cursor/struct.DefaultCursor.html "struct bevy::feathers::cursor::DefaultCursor")

where [DefaultCursor](../feathers/cursor/struct.DefaultCursor.html "struct bevy::feathers::cursor::DefaultCursor"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_gltf/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gltf/lib.rs.html#173)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [DefaultGltfImageSampler](../gltf/struct.DefaultGltfImageSampler.html "struct bevy::gltf::DefaultGltfImageSampler")

where [DefaultGltfImageSampler](../gltf/struct.DefaultGltfImageSampler.html "struct bevy::gltf::DefaultGltfImageSampler"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_resource/texture.rs.html#175)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [DefaultImageSampler](../render/render_resource/struct.DefaultImageSampler.html "struct bevy::render::render_resource::DefaultImageSampler")

where [DefaultImageSampler](../render/render_resource/struct.DefaultImageSampler.html "struct bevy::render::render_resource::DefaultImageSampler"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_resource/texture.rs.html#167)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [DefaultImageSamplerDescriptor](../render/render_resource/struct.DefaultImageSamplerDescriptor.html "struct bevy::render::render_resource::DefaultImageSamplerDescriptor")

where [DefaultImageSamplerDescriptor](../render/render_resource/struct.DefaultImageSamplerDescriptor.html "struct bevy::render::render_resource::DefaultImageSamplerDescriptor"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/material.rs.html#1358)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [DefaultOpaqueRendererMethod](../pbr/struct.DefaultOpaqueRendererMethod.html "struct bevy::pbr::DefaultOpaqueRendererMethod")

where [DefaultOpaqueRendererMethod](../pbr/struct.DefaultOpaqueRendererMethod.html "struct bevy::pbr::DefaultOpaqueRendererMethod"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity_disabling.rs.html#169)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [DefaultQueryFilters](../ecs/entity_disabling/struct.DefaultQueryFilters.html "struct bevy::ecs::entity_disabling::DefaultQueryFilters")

where [DefaultQueryFilters](../ecs/entity_disabling/struct.DefaultQueryFilters.html "struct bevy::ecs::entity_disabling::DefaultQueryFilters"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_audio/0.19.0/x86_64-unknown-linux-gnu/src/bevy_audio/audio.rs.html#232)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [DefaultSpatialScale](../audio/struct.DefaultSpatialScale.html "struct bevy::audio::DefaultSpatialScale")

where [DefaultSpatialScale](../audio/struct.DefaultSpatialScale.html "struct bevy::audio::DefaultSpatialScale"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/deferred/mod.rs.html#188)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [DeferredLightingLayout](../pbr/deferred/struct.DeferredLightingLayout.html "struct bevy::pbr::deferred::DeferredLightingLayout")

where [DeferredLightingLayout](../pbr/deferred/struct.DeferredLightingLayout.html "struct bevy::pbr::deferred::DeferredLightingLayout"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_post_process/0.19.0/x86_64-unknown-linux-gnu/src/bevy_post_process/dof/mod.rs.html#251)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [DepthOfFieldGlobalBindGroup](../post_process/dof/struct.DepthOfFieldGlobalBindGroup.html "struct bevy::post_process::dof::DepthOfFieldGlobalBindGroup")

where [DepthOfFieldGlobalBindGroup](../post_process/dof/struct.DepthOfFieldGlobalBindGroup.html "struct bevy::post_process::dof::DepthOfFieldGlobalBindGroup"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_post_process/0.19.0/x86_64-unknown-linux-gnu/src/bevy_post_process/dof/mod.rs.html#241)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [DepthOfFieldGlobalBindGroupLayout](../post_process/dof/struct.DepthOfFieldGlobalBindGroupLayout.html "struct bevy::post_process::dof::DepthOfFieldGlobalBindGroupLayout")

where [DepthOfFieldGlobalBindGroupLayout](../post_process/dof/struct.DepthOfFieldGlobalBindGroupLayout.html "struct bevy::post_process::dof::DepthOfFieldGlobalBindGroupLayout"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/mip_generation/experimental/depth.rs.html#485)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [DepthPyramidDummyTexture](../core_pipeline/mip_generation/experimental/depth/struct.DepthPyramidDummyTexture.html "struct bevy::core_pipeline::mip_generation::experimental::depth::DepthPyramidDummyTexture")

where [DepthPyramidDummyTexture](../core_pipeline/mip_generation/experimental/depth/struct.DepthPyramidDummyTexture.html "struct bevy::core_pipeline::mip_generation::experimental::depth::DepthPyramidDummyTexture"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/lib.rs.html#184)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [DfgLut](../pbr/struct.DfgLut.html "struct bevy::pbr::DfgLut")

where [DfgLut](../pbr/struct.DfgLut.html "struct bevy::pbr::DfgLut"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_dev_tools/0.19.0/x86_64-unknown-linux-gnu/src/bevy_dev_tools/diagnostics_overlay.rs.html#161)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [DiagnosticsOverlayStyle](../dev_tools/diagnostics_overlay/struct.DiagnosticsOverlayStyle.html "struct bevy::dev_tools::diagnostics_overlay::DiagnosticsOverlayStyle")

where [DiagnosticsOverlayStyle](../dev_tools/diagnostics_overlay/struct.DiagnosticsOverlayStyle.html "struct bevy::dev_tools::diagnostics_overlay::DiagnosticsOverlayStyle"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/diagnostic/internal.rs.html#42)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [DiagnosticsRecorder](../render/diagnostic/struct.DiagnosticsRecorder.html "struct bevy::render::diagnostic::DiagnosticsRecorder")

where [DiagnosticsRecorder](../render/diagnostic/struct.DiagnosticsRecorder.html "struct bevy::render::diagnostic::DiagnosticsRecorder"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_diagnostic/0.19.0/x86_64-unknown-linux-gnu/src/bevy_diagnostic/diagnostic.rs.html#304)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [DiagnosticsStore](../diagnostic/struct.DiagnosticsStore.html "struct bevy::diagnostic::DiagnosticsStore")

where [DiagnosticsStore](../diagnostic/struct.DiagnosticsStore.html "struct bevy::diagnostic::DiagnosticsStore"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/directional_light.rs.html#191)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [DirectionalLightShadowMap](../light/struct.DirectionalLightShadowMap.html "struct bevy::light::DirectionalLightShadowMap")

where [DirectionalLightShadowMap](../light/struct.DirectionalLightShadowMap.html "struct bevy::light::DirectionalLightShadowMap"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_input_focus/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input_focus/directional_navigation.rs.html#248)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [DirectionalNavigationMap](../input_focus/directional_navigation/struct.DirectionalNavigationMap.html "struct bevy::input_focus::directional_navigation::DirectionalNavigationMap")

where [DirectionalNavigationMap](../input_focus/directional_navigation/struct.DirectionalNavigationMap.html "struct bevy::input_focus::directional_navigation::DirectionalNavigationMap"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/camera.rs.html#826)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [DirtySpecializations](../render/camera/struct.DirtySpecializations.html "struct bevy::render::camera::DirtySpecializations")

where [DirtySpecializations](../render/camera/struct.DirtySpecializations.html "struct bevy::render::camera::DirtySpecializations"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/camera.rs.html#1033)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [DirtyWireframeSpecializations](../render/camera/struct.DirtyWireframeSpecializations.html "struct bevy::render::camera::DirtyWireframeSpecializations")

where [DirtyWireframeSpecializations](../render/camera/struct.DirtyWireframeSpecializations.html "struct bevy::render::camera::DirtyWireframeSpecializations"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_winit/0.19.0/x86_64-unknown-linux-gnu/src/bevy_winit/lib.rs.html#214)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [DisplayHandleWrapper](../winit/struct.DisplayHandleWrapper.html "struct bevy::winit::DisplayHandleWrapper")

where [DisplayHandleWrapper](../winit/struct.DisplayHandleWrapper.html "struct bevy::winit::DisplayHandleWrapper"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/mip_generation/experimental/depth.rs.html#256)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [DownsampleDepthPipeline](../core_pipeline/mip_generation/experimental/depth/struct.DownsampleDepthPipeline.html "struct bevy::core_pipeline::mip_generation::experimental::depth::DownsampleDepthPipeline")

where [DownsampleDepthPipeline](../core_pipeline/mip_generation/experimental/depth/struct.DownsampleDepthPipeline.html "struct bevy::core_pipeline::mip_generation::experimental::depth::DownsampleDepthPipeline"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/mip_generation/experimental/depth.rs.html#285)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [DownsampleDepthPipelines](../core_pipeline/mip_generation/experimental/depth/struct.DownsampleDepthPipelines.html "struct bevy::core_pipeline::mip_generation::experimental::depth::DownsampleDepthPipelines")

where [DownsampleDepthPipelines](../core_pipeline/mip_generation/experimental/depth/struct.DownsampleDepthPipelines.html "struct bevy::core_pipeline::mip_generation::experimental::depth::DownsampleDepthPipelines"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/mip_generation/mod.rs.html#61)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [DownsampleShaders](../core_pipeline/mip_generation/struct.DownsampleShaders.html "struct bevy::core_pipeline::mip_generation::DownsampleShaders")

where [DownsampleShaders](../core_pipeline/mip_generation/struct.DownsampleShaders.html "struct bevy::core_pipeline::mip_generation::DownsampleShaders"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/light_probe/generate.rs.html#95)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [DownsamplingConfig](../pbr/generate/struct.DownsamplingConfig.html "struct bevy::pbr::generate::DownsamplingConfig")

where [DownsamplingConfig](../pbr/generate/struct.DownsamplingConfig.html "struct bevy::pbr::generate::DownsamplingConfig"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/io/embedded/mod.rs.html#31)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [EmbeddedAssetRegistry](../asset/io/embedded/struct.EmbeddedAssetRegistry.html "struct bevy::asset::io::embedded::EmbeddedAssetRegistry")

where [EmbeddedAssetRegistry](../asset/io/embedded/struct.EmbeddedAssetRegistry.html "struct bevy::asset::io::embedded::EmbeddedAssetRegistry"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_winit/0.19.0/x86_64-unknown-linux-gnu/src/bevy_winit/lib.rs.html#205)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [EventLoopProxyWrapper](../winit/struct.EventLoopProxyWrapper.html "struct bevy::winit::EventLoopProxyWrapper")

where [EventLoopProxyWrapper](../winit/struct.EventLoopProxyWrapper.html "struct bevy::winit::EventLoopProxyWrapper"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_ui_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_render/box_shadow.rs.html#201)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [ExtractedBoxShadows](../ui_render/box_shadow/struct.ExtractedBoxShadows.html "struct bevy::ui_render::box_shadow::ExtractedBoxShadows")

where [ExtractedBoxShadows](../ui_render/box_shadow/struct.ExtractedBoxShadows.html "struct bevy::ui_render::box_shadow::ExtractedBoxShadows"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/render/mod.rs.html#338)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [ExtractedSlices](../sprite_render/struct.ExtractedSlices.html "struct bevy::sprite_render::ExtractedSlices")

where [ExtractedSlices](../sprite_render/struct.ExtractedSlices.html "struct bevy::sprite_render::ExtractedSlices"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/render/mod.rs.html#333)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [ExtractedSprites](../sprite_render/struct.ExtractedSprites.html "struct bevy::sprite_render::ExtractedSprites")

where [ExtractedSprites](../sprite_render/struct.ExtractedSprites.html "struct bevy::sprite_render::ExtractedSprites"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_ui_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_render/lib.rs.html#376)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [ExtractedUiNodes](../ui_render/struct.ExtractedUiNodes.html "struct bevy::ui_render::ExtractedUiNodes")

where [ExtractedUiNodes](../ui_render/struct.ExtractedUiNodes.html "struct bevy::ui_render::ExtractedUiNodes"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_ui_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_render/ui_texture_slice_pipeline.rs.html#210)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [ExtractedUiTextureSlices](../ui_render/ui_texture_slice_pipeline/struct.ExtractedUiTextureSlices.html "struct bevy::ui_render::ui_texture_slice_pipeline::ExtractedUiTextureSlices")

where [ExtractedUiTextureSlices](../ui_render/ui_texture_slice_pipeline/struct.ExtractedUiTextureSlices.html "struct bevy::ui_render::ui_texture_slice_pipeline::ExtractedUiTextureSlices"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/view/window/mod.rs.html#105)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [ExtractedWindows](../render/view/struct.ExtractedWindows.html "struct bevy::render::view::ExtractedWindows")

where [ExtractedWindows](../render/view/struct.ExtractedWindows.html "struct bevy::render::view::ExtractedWindows"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/material_bind_groups.rs.html#217)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [FallbackBindlessResources](../pbr/struct.FallbackBindlessResources.html "struct bevy::pbr::FallbackBindlessResources")

where [FallbackBindlessResources](../pbr/struct.FallbackBindlessResources.html "struct bevy::pbr::FallbackBindlessResources"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/error/handler.rs.html#114)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [FallbackErrorHandler](../ecs/error/struct.FallbackErrorHandler.html "struct bevy::ecs::error::FallbackErrorHandler")

where [FallbackErrorHandler](../ecs/error/struct.FallbackErrorHandler.html "struct bevy::ecs::error::FallbackErrorHandler"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/texture/fallback_image.rs.html#22)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [FallbackImage](../render/texture/struct.FallbackImage.html "struct bevy::render::texture::FallbackImage")

where [FallbackImage](../render/texture/struct.FallbackImage.html "struct bevy::render::texture::FallbackImage"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/texture/fallback_image.rs.html#64)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [FallbackImageCubemap](../render/texture/struct.FallbackImageCubemap.html "struct bevy::render::texture::FallbackImageCubemap")

where [FallbackImageCubemap](../render/texture/struct.FallbackImageCubemap.html "struct bevy::render::texture::FallbackImageCubemap"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/texture/fallback_image.rs.html#247)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [FallbackImageFormatMsaaCache](../render/texture/struct.FallbackImageFormatMsaaCache.html "struct bevy::render::texture::FallbackImageFormatMsaaCache")

where [FallbackImageFormatMsaaCache](../render/texture/struct.FallbackImageFormatMsaaCache.html "struct bevy::render::texture::FallbackImageFormatMsaaCache"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/texture/fallback_image.rs.html#58)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [FallbackImageZero](../render/texture/struct.FallbackImageZero.html "struct bevy::render::texture::FallbackImageZero")

where [FallbackImageZero](../render/texture/struct.FallbackImageZero.html "struct bevy::render::texture::FallbackImageZero"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/main_schedule.rs.html#348)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [FixedMainScheduleOrder](../app/struct.FixedMainScheduleOrder.html "struct bevy::app::FixedMainScheduleOrder")

where [FixedMainScheduleOrder](../app/struct.FixedMainScheduleOrder.html "struct bevy::app::FixedMainScheduleOrder"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/fog.rs.html#44)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [FogMeta](../pbr/struct.FogMeta.html "struct bevy::pbr::FogMeta")

where [FogMeta](../pbr/struct.FogMeta.html "struct bevy::pbr::FogMeta"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/font_atlas_set.rs.html#28)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [FontAtlasSet](../text/struct.FontAtlasSet.html "struct bevy::text::FontAtlasSet")

where [FontAtlasSet](../text/struct.FontAtlasSet.html "struct bevy::text::FontAtlasSet"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/parley_context.rs.html#36)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [FontCx](../text/struct.FontCx.html "struct bevy::text::FontCx")

where [FontCx](../text/struct.FontCx.html "struct bevy::text::FontCx"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_dev_tools/0.19.0/x86_64-unknown-linux-gnu/src/bevy_dev_tools/fps_overlay.rs.html#108)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [FpsOverlayConfig](../dev_tools/fps_overlay/struct.FpsOverlayConfig.html "struct bevy::dev_tools::fps_overlay::FpsOverlayConfig")

where [FpsOverlayConfig](../dev_tools/fps_overlay/struct.FpsOverlayConfig.html "struct bevy::dev_tools::fps_overlay::FpsOverlayConfig"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_diagnostic/0.19.0/x86_64-unknown-linux-gnu/src/bevy_diagnostic/frame_count.rs.html#20)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [FrameCount](../diagnostic/struct.FrameCount.html "struct bevy::diagnostic::FrameCount")

where [FrameCount](../diagnostic/struct.FrameCount.html "struct bevy::diagnostic::FrameCount"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/fullscreen_vertex_shader/mod.rs.html#7)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [FullscreenShader](../core_pipeline/struct.FullscreenShader.html "struct bevy::core_pipeline::FullscreenShader")

where [FullscreenShader](../core_pipeline/struct.FullscreenShader.html "struct bevy::core_pipeline::FullscreenShader"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_anti_alias/0.19.0/x86_64-unknown-linux-gnu/src/bevy_anti_alias/fxaa/mod.rs.html#113)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [FxaaPipeline](../anti_alias/fxaa/struct.FxaaPipeline.html "struct bevy::anti_alias::fxaa::FxaaPipeline")

where [FxaaPipeline](../anti_alias/fxaa/struct.FxaaPipeline.html "struct bevy::anti_alias::fxaa::FxaaPipeline"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/light_probe/generate.rs.html#69)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [GeneratorBindGroupLayouts](../pbr/generate/struct.GeneratorBindGroupLayouts.html "struct bevy::pbr::generate::GeneratorBindGroupLayouts")

where [GeneratorBindGroupLayouts](../pbr/generate/struct.GeneratorBindGroupLayouts.html "struct bevy::pbr::generate::GeneratorBindGroupLayouts"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/light_probe/generate.rs.html#85)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [GeneratorPipelines](../pbr/generate/struct.GeneratorPipelines.html "struct bevy::pbr::generate::GeneratorPipelines")

where [GeneratorPipelines](../pbr/generate/struct.GeneratorPipelines.html "struct bevy::pbr::generate::GeneratorPipelines"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/light_probe/generate.rs.html#79)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [GeneratorSamplers](../pbr/generate/struct.GeneratorSamplers.html "struct bevy::pbr::generate::GeneratorSamplers")

where [GeneratorSamplers](../pbr/generate/struct.GeneratorSamplers.html "struct bevy::pbr::generate::GeneratorSamplers"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/config.rs.html#97)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [GizmoConfigStore](struct.GizmoConfigStore.html "struct bevy::prelude::GizmoConfigStore")

where [GizmoConfigStore](struct.GizmoConfigStore.html "struct bevy::prelude::GizmoConfigStore"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/lib.rs.html#205)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [GizmoHandles](../gizmos/struct.GizmoHandles.html "struct bevy::gizmos::GizmoHandles")

where [GizmoHandles](../gizmos/struct.GizmoHandles.html "struct bevy::gizmos::GizmoHandles"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/ambient_light.rs.html#60)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [GlobalAmbientLight](struct.GlobalAmbientLight.html "struct bevy::prelude::GlobalAmbientLight")

where [GlobalAmbientLight](struct.GlobalAmbientLight.html "struct bevy::prelude::GlobalAmbientLight"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/cluster/mod.rs.html#41)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [GlobalClusterSettings](../light/cluster/struct.GlobalClusterSettings.html "struct bevy::light::cluster::GlobalClusterSettings")

where [GlobalClusterSettings](../light/cluster/struct.GlobalClusterSettings.html "struct bevy::light::cluster::GlobalClusterSettings"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/cluster/mod.rs.html#135)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [GlobalClusterableObjectMeta](../pbr/struct.GlobalClusterableObjectMeta.html "struct bevy::pbr::GlobalClusterableObjectMeta")

where [GlobalClusterableObjectMeta](../pbr/struct.GlobalClusterableObjectMeta.html "struct bevy::pbr::GlobalClusterableObjectMeta"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_dev_tools/0.19.0/x86_64-unknown-linux-gnu/src/bevy_dev_tools/render_debug.rs.html#296)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [GlobalRenderDebugOverlay](../dev_tools/render_debug/struct.GlobalRenderDebugOverlay.html "struct bevy::dev_tools::render_debug::GlobalRenderDebugOverlay")

where [GlobalRenderDebugOverlay](../dev_tools/render_debug/struct.GlobalRenderDebugOverlay.html "struct bevy::dev_tools::render_debug::GlobalRenderDebugOverlay"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_ui_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_render/debug_overlay.rs.html#107)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [GlobalUiDebugOptions](struct.GlobalUiDebugOptions.html "struct bevy::prelude::GlobalUiDebugOptions")

where [GlobalUiDebugOptions](struct.GlobalUiDebugOptions.html "struct bevy::prelude::GlobalUiDebugOptions"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_audio/0.19.0/x86_64-unknown-linux-gnu/src/bevy_audio/volume.rs.html#8)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [GlobalVolume](struct.GlobalVolume.html "struct bevy::prelude::GlobalVolume")

where [GlobalVolume](struct.GlobalVolume.html "struct bevy::prelude::GlobalVolume"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/globals.rs.html#59)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [GlobalsBuffer](../render/globals/struct.GlobalsBuffer.html "struct bevy::render::globals::GlobalsBuffer")

where [GlobalsBuffer](../render/globals/struct.GlobalsBuffer.html "struct bevy::render::globals::GlobalsBuffer"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/globals.rs.html#42)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [GlobalsUniform](../render/globals/struct.GlobalsUniform.html "struct bevy::render::globals::GlobalsUniform")

where [GlobalsUniform](../render/globals/struct.GlobalsUniform.html "struct bevy::render::globals::GlobalsUniform"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_gltf/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gltf/loader/extensions/mod.rs.html#33)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [GltfExtensionHandlers](../gltf/extensions/struct.GltfExtensionHandlers.html "struct bevy::gltf::extensions::GltfExtensionHandlers")

where [GltfExtensionHandlers](../gltf/extensions/struct.GltfExtensionHandlers.html "struct bevy::gltf::extensions::GltfExtensionHandlers"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/batching/gpu_preprocessing.rs.html#103)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [GpuPreprocessingSupport](../render/batching/gpu_preprocessing/struct.GpuPreprocessingSupport.html "struct bevy::render::batching::gpu_preprocessing::GpuPreprocessingSupport")

where [GpuPreprocessingSupport](../render/batching/gpu_preprocessing/struct.GpuPreprocessingSupport.html "struct bevy::render::batching::gpu_preprocessing::GpuPreprocessingSupport"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_remote/0.19.0/x86_64-unknown-linux-gnu/src/bevy_remote/http.rs.html#67)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [Headers](../remote/http/struct.Headers.html "struct bevy::remote::http::Headers")

where [Headers](../remote/http/struct.Headers.html "struct bevy::remote::http::Headers"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_remote/0.19.0/x86_64-unknown-linux-gnu/src/bevy_remote/http.rs.html#219)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [HostAddress](../remote/http/struct.HostAddress.html "struct bevy::remote::http::HostAddress")

where [HostAddress](../remote/http/struct.HostAddress.html "struct bevy::remote::http::HostAddress"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_remote/0.19.0/x86_64-unknown-linux-gnu/src/bevy_remote/http.rs.html#226)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [HostPort](../remote/http/struct.HostPort.html "struct bevy::remote::http::HostPort")

where [HostPort](../remote/http/struct.HostPort.html "struct bevy::remote::http::HostPort"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/lib.rs.html#154)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [HotPatchChanges](../ecs/struct.HotPatchChanges.html "struct bevy::ecs::HotPatchChanges")

where [HotPatchChanges](../ecs/struct.HotPatchChanges.html "struct bevy::ecs::HotPatchChanges"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/hover.rs.html#59)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [HoverMap](../picking/hover/struct.HoverMap.html "struct bevy::picking::hover::HoverMap")

where [HoverMap](../picking/hover/struct.HoverMap.html "struct bevy::picking::hover::HoverMap"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/render/mod.rs.html#494)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [ImageBindGroups](../sprite_render/struct.ImageBindGroups.html "struct bevy::sprite_render::ImageBindGroups")

where [ImageBindGroups](../sprite_render/struct.ImageBindGroups.html "struct bevy::sprite_render::ImageBindGroups"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_ui_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_render/lib.rs.html#1570)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [ImageNodeBindGroups](../ui_render/struct.ImageNodeBindGroups.html "struct bevy::ui_render::ImageNodeBindGroups")

where [ImageNodeBindGroups](../ui_render/struct.ImageNodeBindGroups.html "struct bevy::ui_render::ImageNodeBindGroups"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/batching/gpu_preprocessing.rs.html#907)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [IndirectParametersBuffers](../render/batching/gpu_preprocessing/struct.IndirectParametersBuffers.html "struct bevy::render::batching::gpu_preprocessing::IndirectParametersBuffers")

where [IndirectParametersBuffers](../render/batching/gpu_preprocessing/struct.IndirectParametersBuffers.html "struct bevy::render::batching::gpu_preprocessing::IndirectParametersBuffers"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/batching/gpu_preprocessing.rs.html#918)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [IndirectParametersBuffersSettings](../render/batching/gpu_preprocessing/struct.IndirectParametersBuffersSettings.html "struct bevy::render::batching::gpu_preprocessing::IndirectParametersBuffersSettings")

where [IndirectParametersBuffersSettings](../render/batching/gpu_preprocessing/struct.IndirectParametersBuffersSettings.html "struct bevy::render::batching::gpu_preprocessing::IndirectParametersBuffersSettings"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_input_focus/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input_focus/lib.rs.html#97)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [InputFocus](../input_focus/struct.InputFocus.html "struct bevy::input_focus::InputFocus")

where [InputFocus](../input_focus/struct.InputFocus.html "struct bevy::input_focus::InputFocus"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_input_focus/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input_focus/lib.rs.html#170)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [InputFocusVisible](../input_focus/struct.InputFocusVisible.html "struct bevy::input_focus::InputFocusVisible")

where [InputFocusVisible](../input_focus/struct.InputFocusVisible.html "struct bevy::input_focus::InputFocusVisible"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/parley_context.rs.html#175)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [LayoutCx](../text/struct.LayoutCx.html "struct bevy::text::LayoutCx")

where [LayoutCx](../text/struct.LayoutCx.html "struct bevy::text::LayoutCx"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/light.rs.html#2196)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [LightKeyCache](../pbr/struct.LightKeyCache.html "struct bevy::pbr::LightKeyCache")

where [LightKeyCache](../pbr/struct.LightKeyCache.html "struct bevy::pbr::LightKeyCache"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/light.rs.html#961)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [LightMeta](../pbr/struct.LightMeta.html "struct bevy::pbr::LightMeta")

where [LightMeta](../pbr/struct.LightMeta.html "struct bevy::pbr::LightMeta"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/light_probe/mod.rs.html#162)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [LightProbesBuffer](../pbr/struct.LightProbesBuffer.html "struct bevy::pbr::LightProbesBuffer")

where [LightProbesBuffer](../pbr/struct.LightProbesBuffer.html "struct bevy::pbr::LightProbesBuffer"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_gizmos_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos_render/lib.rs.html#622)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [LineGizmoEntities](../gizmos_render/struct.LineGizmoEntities.html "struct bevy::gizmos_render::LineGizmoEntities")

where [LineGizmoEntities](../gizmos_render/struct.LineGizmoEntities.html "struct bevy::gizmos_render::LineGizmoEntities"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_diagnostic/0.19.0/x86_64-unknown-linux-gnu/src/bevy_diagnostic/log_diagnostics_plugin.rs.html#30)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [LogDiagnosticsState](../diagnostic/struct.LogDiagnosticsState.html "struct bevy::diagnostic::LogDiagnosticsState")

where [LogDiagnosticsState](../diagnostic/struct.LogDiagnosticsState.html "struct bevy::diagnostic::LogDiagnosticsState"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/main_schedule.rs.html#213)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [MainScheduleOrder](../app/struct.MainScheduleOrder.html "struct bevy::app::MainScheduleOrder")

where [MainScheduleOrder](../app/struct.MainScheduleOrder.html "struct bevy::app::MainScheduleOrder"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/executor/multi_threaded.rs.html#844)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [MainThreadExecutor](../ecs/schedule/struct.MainThreadExecutor.html "struct bevy::ecs::schedule::MainThreadExecutor")

where [MainThreadExecutor](../ecs/schedule/struct.MainThreadExecutor.html "struct bevy::ecs::schedule::MainThreadExecutor"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/extract_plugin.rs.html#105)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [MainWorld](../render/struct.MainWorld.html "struct bevy::render::MainWorld")

where [MainWorld](../render/struct.MainWorld.html "struct bevy::render::MainWorld"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_a11y/0.19.0/x86_64-unknown-linux-gnu/src/bevy_a11y/lib.rs.html#152)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [ManageAccessibilityUpdates](../a11y/struct.ManageAccessibilityUpdates.html "struct bevy::a11y::ManageAccessibilityUpdates")

where [ManageAccessibilityUpdates](../a11y/struct.ManageAccessibilityUpdates.html "struct bevy::a11y::ManageAccessibilityUpdates"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/texture/manual_texture_view.rs.html#52)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [ManualTextureViews](struct.ManualTextureViews.html "struct bevy::prelude::ManualTextureViews")

where [ManualTextureViews](struct.ManualTextureViews.html "struct bevy::prelude::ManualTextureViews"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/material_bind_groups.rs.html#36)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [MaterialBindGroupAllocators](../pbr/struct.MaterialBindGroupAllocators.html "struct bevy::pbr::MaterialBindGroupAllocators")

where [MaterialBindGroupAllocators](../pbr/struct.MaterialBindGroupAllocators.html "struct bevy::pbr::MaterialBindGroupAllocators"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/material.rs.html#442)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [MaterialPipeline](../pbr/struct.MaterialPipeline.html "struct bevy::pbr::MaterialPipeline")

where [MaterialPipeline](../pbr/struct.MaterialPipeline.html "struct bevy::pbr::MaterialPipeline"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/mesh2d/mesh.rs.html#746)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [Mesh2dBindGroup](../sprite_render/struct.Mesh2dBindGroup.html "struct bevy::sprite_render::Mesh2dBindGroup")

where [Mesh2dBindGroup](../sprite_render/struct.Mesh2dBindGroup.html "struct bevy::sprite_render::Mesh2dBindGroup"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/mesh2d/mesh.rs.html#311)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [Mesh2dPipeline](../sprite_render/struct.Mesh2dPipeline.html "struct bevy::sprite_render::Mesh2dPipeline")

where [Mesh2dPipeline](../sprite_render/struct.Mesh2dPipeline.html "struct bevy::sprite_render::Mesh2dPipeline"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/mesh/allocator.rs.html#44)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [MeshAllocator](../render/mesh/allocator/struct.MeshAllocator.html "struct bevy::render::mesh::allocator::MeshAllocator")

where [MeshAllocator](../render/mesh/allocator/struct.MeshAllocator.html "struct bevy::render::mesh::allocator::MeshAllocator"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/mesh/allocator.rs.html#64)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [MeshAllocatorSettings](../render/mesh/allocator/struct.MeshAllocatorSettings.html "struct bevy::render::mesh::allocator::MeshAllocatorSettings")

where [MeshAllocatorSettings](../render/mesh/allocator/struct.MeshAllocatorSettings.html "struct bevy::render::mesh::allocator::MeshAllocatorSettings"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#3726)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [MeshBindGroups](../pbr/enum.MeshBindGroups.html "enum bevy::pbr::MeshBindGroups")

where [MeshBindGroups](../pbr/enum.MeshBindGroups.html "enum bevy::pbr::MeshBindGroups"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#649)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [MeshCullingDataBuffer](../pbr/struct.MeshCullingDataBuffer.html "struct bevy::pbr::MeshCullingDataBuffer")

where [MeshCullingDataBuffer](../pbr/struct.MeshCullingDataBuffer.html "struct bevy::pbr::MeshCullingDataBuffer"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/mesh_picking/mod.rs.html#38)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [MeshPickingSettings](struct.MeshPickingSettings.html "struct bevy::prelude::MeshPickingSettings")

where [MeshPickingSettings](struct.MeshPickingSettings.html "struct bevy::prelude::MeshPickingSettings"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#2665)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [MeshPipeline](../pbr/struct.MeshPipeline.html "struct bevy::pbr::MeshPipeline")

where [MeshPipeline](../pbr/struct.MeshPipeline.html "struct bevy::pbr::MeshPipeline"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh_view_bindings.rs.html#533)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [MeshPipelineViewLayouts](../pbr/struct.MeshPipelineViewLayouts.html "struct bevy::pbr::MeshPipelineViewLayouts")

where [MeshPipelineViewLayouts](../pbr/struct.MeshPipelineViewLayouts.html "struct bevy::pbr::MeshPipelineViewLayouts"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/vertex.rs.html#954)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [MeshVertexBufferLayouts](../mesh/struct.MeshVertexBufferLayouts.html "struct bevy::mesh::MeshVertexBufferLayouts")

where [MeshVertexBufferLayouts](../mesh/struct.MeshVertexBufferLayouts.html "struct bevy::mesh::MeshVertexBufferLayouts"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#1091)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [MeshesToReextractNextFrame](../pbr/struct.MeshesToReextractNextFrame.html "struct bevy::pbr::MeshesToReextractNextFrame")

where [MeshesToReextractNextFrame](../pbr/struct.MeshesToReextractNextFrame.html "struct bevy::pbr::MeshesToReextractNextFrame"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/message/message_registry.rs.html#22)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [MessageRegistry](../ecs/message/struct.MessageRegistry.html "struct bevy::ecs::message::MessageRegistry")

where [MessageRegistry](../ecs/message/struct.MessageRegistry.html "struct bevy::ecs::message::MessageRegistry"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/mip_generation/mod.rs.html#143)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [MipGenerationJobs](../core_pipeline/mip_generation/struct.MipGenerationJobs.html "struct bevy::core_pipeline::mip_generation::MipGenerationJobs")

where [MipGenerationJobs](../core_pipeline/mip_generation/struct.MipGenerationJobs.html "struct bevy::core_pipeline::mip_generation::MipGenerationJobs"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/mip_generation/mod.rs.html#191)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [MipGenerationPipelines](../core_pipeline/mip_generation/struct.MipGenerationPipelines.html "struct bevy::core_pipeline::mip_generation::MipGenerationPipelines")

where [MipGenerationPipelines](../core_pipeline/mip_generation/struct.MipGenerationPipelines.html "struct bevy::core_pipeline::mip_generation::MipGenerationPipelines"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/morph.rs.html#37)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [MorphIndices](../pbr/enum.MorphIndices.html "enum bevy::pbr::MorphIndices")

where [MorphIndices](../pbr/enum.MorphIndices.html "enum bevy::pbr::MorphIndices"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/morph.rs.html#103)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [MorphUniforms](../pbr/struct.MorphUniforms.html "struct bevy::pbr::MorphUniforms")

where [MorphUniforms](../pbr/struct.MorphUniforms.html "struct bevy::pbr::MorphUniforms"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_post_process/0.19.0/x86_64-unknown-linux-gnu/src/bevy_post_process/motion_blur/pipeline.rs.html#28)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [MotionBlurPipeline](../post_process/motion_blur/pipeline/struct.MotionBlurPipeline.html "struct bevy::post_process::motion_blur::pipeline::MotionBlurPipeline")

where [MotionBlurPipeline](../post_process/motion_blur/pipeline/struct.MotionBlurPipeline.html "struct bevy::post_process::motion_blur::pipeline::MotionBlurPipeline"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/oit/mod.rs.html#151)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [OitBuffers](../core_pipeline/oit/struct.OitBuffers.html "struct bevy::core_pipeline::oit::OitBuffers")

where [OitBuffers](../core_pipeline/oit/struct.OitBuffers.html "struct bevy::core_pipeline::oit::OitBuffers"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/oit/resolve/mod.rs.html#98)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [OitResolveBindGroup](../core_pipeline/oit/resolve/struct.OitResolveBindGroup.html "struct bevy::core_pipeline::oit::resolve::OitResolveBindGroup")

where [OitResolveBindGroup](../core_pipeline/oit/resolve/struct.OitResolveBindGroup.html "struct bevy::core_pipeline::oit::resolve::OitResolveBindGroup"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/oit/resolve/mod.rs.html#102)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [OitResolvePipeline](../core_pipeline/oit/resolve/struct.OitResolvePipeline.html "struct bevy::core_pipeline::oit::resolve::OitResolvePipeline")

where [OitResolvePipeline](../core_pipeline/oit/resolve/struct.OitResolvePipeline.html "struct bevy::core_pipeline::oit::resolve::OitResolvePipeline"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/cursor.rs.html#47)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [OverrideCursor](../feathers/cursor/struct.OverrideCursor.html "struct bevy::feathers::cursor::OverrideCursor")

where [OverrideCursor](../feathers/cursor/struct.OverrideCursor.html "struct bevy::feathers::cursor::OverrideCursor"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/renderer/render_context.rs.html#28)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [PendingCommandBuffers](../render/renderer/struct.PendingCommandBuffers.html "struct bevy::render::renderer::PendingCommandBuffers")

where [PendingCommandBuffers](../render/renderer/struct.PendingCommandBuffers.html "struct bevy::render::renderer::PendingCommandBuffers"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/mesh2d/material.rs.html#726)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [PendingMeshMaterial2dQueues](../sprite_render/struct.PendingMeshMaterial2dQueues.html "struct bevy::sprite_render::PendingMeshMaterial2dQueues")

where [PendingMeshMaterial2dQueues](../sprite_render/struct.PendingMeshMaterial2dQueues.html "struct bevy::sprite_render::PendingMeshMaterial2dQueues"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/material.rs.html#893)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [PendingMeshMaterialQueues](../pbr/struct.PendingMeshMaterialQueues.html "struct bevy::pbr::PendingMeshMaterialQueues")

where [PendingMeshMaterialQueues](../pbr/struct.PendingMeshMaterialQueues.html "struct bevy::pbr::PendingMeshMaterialQueues"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/prepass/mod.rs.html#848)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [PendingPrepassMeshMaterialQueues](../pbr/struct.PendingPrepassMeshMaterialQueues.html "struct bevy::pbr::PendingPrepassMeshMaterialQueues")

where [PendingPrepassMeshMaterialQueues](../pbr/struct.PendingPrepassMeshMaterialQueues.html "struct bevy::pbr::PendingPrepassMeshMaterialQueues"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/light.rs.html#2264)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [PendingShadowQueues](../pbr/struct.PendingShadowQueues.html "struct bevy::pbr::PendingShadowQueues")

where [PendingShadowQueues](../pbr/struct.PendingShadowQueues.html "struct bevy::pbr::PendingShadowQueues"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/mesh2d/wireframe2d.rs.html#736)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [PendingWireframe2dQueues](../sprite_render/struct.PendingWireframe2dQueues.html "struct bevy::sprite_render::PendingWireframe2dQueues")

where [PendingWireframe2dQueues](../sprite_render/struct.PendingWireframe2dQueues.html "struct bevy::sprite_render::PendingWireframe2dQueues"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/wireframe.rs.html#1350)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [PendingWireframeQueues](../pbr/wireframe/struct.PendingWireframeQueues.html "struct bevy::pbr::wireframe::PendingWireframeQueues")

where [PendingWireframeQueues](../pbr/wireframe/struct.PendingWireframeQueues.html "struct bevy::pbr::wireframe::PendingWireframeQueues"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/lib.rs.html#296)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [PickingSettings](../picking/struct.PickingSettings.html "struct bevy::picking::PickingSettings")

where [PickingSettings](../picking/struct.PickingSettings.html "struct bevy::picking::PickingSettings"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_resource/pipeline_cache.rs.html#202)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [PipelineCache](../render/render_resource/struct.PipelineCache.html "struct bevy::render::render_resource::PipelineCache")

where [PipelineCache](../render/render_resource/struct.PipelineCache.html "struct bevy::render::render_resource::PipelineCache"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/point_light.rs.html#177)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [PointLightShadowMap](../light/struct.PointLightShadowMap.html "struct bevy::light::PointLightShadowMap")

where [PointLightShadowMap](../light/struct.PointLightShadowMap.html "struct bevy::light::PointLightShadowMap"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/input.rs.html#42)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [PointerInputSettings](../picking/input/struct.PointerInputSettings.html "struct bevy::picking::input::PointerInputSettings")

where [PointerInputSettings](../picking/input/struct.PointerInputSettings.html "struct bevy::picking::input::PointerInputSettings"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/pointer.rs.html#93)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [PointerMap](../picking/pointer/struct.PointerMap.html "struct bevy::picking::pointer::PointerMap")

where [PointerMap](../picking/pointer/struct.PointerMap.html "struct bevy::picking::pointer::PointerMap"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/events.rs.html#549)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [PointerState](struct.PointerState.html "struct bevy::prelude::PointerState")

where [PointerState](struct.PointerState.html "struct bevy::prelude::PointerState"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_post_process/0.19.0/x86_64-unknown-linux-gnu/src/bevy_post_process/effect_stack/mod.rs.html#79)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [PostProcessingPipeline](../post_process/effect_stack/struct.PostProcessingPipeline.html "struct bevy::post_process::effect_stack::PostProcessingPipeline")

where [PostProcessingPipeline](../post_process/effect_stack/struct.PostProcessingPipeline.html "struct bevy::post_process::effect_stack::PostProcessingPipeline"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_post_process/0.19.0/x86_64-unknown-linux-gnu/src/bevy_post_process/effect_stack/mod.rs.html#108)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [PostProcessingUniformBuffers](../post_process/effect_stack/struct.PostProcessingUniformBuffers.html "struct bevy::post_process::effect_stack::PostProcessingUniformBuffers")

where [PostProcessingUniformBuffers](../post_process/effect_stack/struct.PostProcessingUniformBuffers.html "struct bevy::post_process::effect_stack::PostProcessingUniformBuffers"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/prepass/mod.rs.html#246)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [PrepassPipeline](../pbr/struct.PrepassPipeline.html "struct bevy::pbr::PrepassPipeline")

where [PrepassPipeline](../pbr/struct.PrepassPipeline.html "struct bevy::pbr::PrepassPipeline"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/prepass/mod.rs.html#711)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [PrepassViewBindGroup](../pbr/struct.PrepassViewBindGroup.html "struct bevy::pbr::PrepassViewBindGroup")

where [PrepassViewBindGroup](../pbr/struct.PrepassViewBindGroup.html "struct bevy::pbr::PrepassViewBindGroup"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/gpu_preprocess.rs.html#96)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [PreprocessPipelines](../pbr/struct.PreprocessPipelines.html "struct bevy::pbr::PreprocessPipelines")

where [PreprocessPipelines](../pbr/struct.PreprocessPipelines.html "struct bevy::pbr::PreprocessPipelines"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/hover.rs.html#63)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [PreviousHoverMap](../picking/hover/struct.PreviousHoverMap.html "struct bevy::picking::hover::PreviousHoverMap")

where [PreviousHoverMap](../picking/hover/struct.PreviousHoverMap.html "struct bevy::picking::hover::PreviousHoverMap"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/prepass/mod.rs.html#110)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [PreviousViewUniforms](../core_pipeline/prepass/struct.PreviousViewUniforms.html "struct bevy::core_pipeline::prepass::PreviousViewUniforms")

where [PreviousViewUniforms](../core_pipeline/prepass/struct.PreviousViewUniforms.html "struct bevy::core_pipeline::prepass::PreviousViewUniforms"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/spawn.rs.html#670)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [QueuedScenes](../scene/struct.QueuedScenes.html "struct bevy::scene::QueuedScenes")

where [QueuedScenes](../scene/struct.QueuedScenes.html "struct bevy::scene::QueuedScenes"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/renderer/raw_vulkan_init.rs.html#11)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [RawVulkanInitSettings](../render/renderer/raw_vulkan_init/struct.RawVulkanInitSettings.html "struct bevy::render::renderer::raw_vulkan_init::RawVulkanInitSettings")

where [RawVulkanInitSettings](../render/renderer/raw_vulkan_init/struct.RawVulkanInitSettings.html "struct bevy::render::renderer::raw_vulkan_init::RawVulkanInitSettings"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/backend.rs.html#282)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [RayMap](../picking/backend/prelude/struct.RayMap.html "struct bevy::picking::backend::prelude::RayMap")

where [RayMap](../picking/backend/prelude/struct.RayMap.html "struct bevy::picking::backend::prelude::RayMap"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_solari/0.19.0/x86_64-unknown-linux-gnu/src/bevy_solari/scene/binder.rs.html#30)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [RaytracingSceneBindings](../solari/scene/struct.RaytracingSceneBindings.html "struct bevy::solari::scene::RaytracingSceneBindings")

where [RaytracingSceneBindings](../solari/scene/struct.RaytracingSceneBindings.html "struct bevy::solari::scene::RaytracingSceneBindings"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_registry.rs.html#118)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [RegisteredSystemDespawner](../ecs/system/struct.RegisteredSystemDespawner.html "struct bevy::ecs::system::RegisteredSystemDespawner")

where [RegisteredSystemDespawner](../ecs/system/struct.RegisteredSystemDespawner.html "struct bevy::ecs::system::RegisteredSystemDespawner"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#572)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [RemSize](../text/struct.RemSize.html "struct bevy::text::RemSize")

where [RemSize](../text/struct.RemSize.html "struct bevy::text::RemSize"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_remote/0.19.0/x86_64-unknown-linux-gnu/src/bevy_remote/lib.rs.html#964)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [RemoteMethods](../remote/struct.RemoteMethods.html "struct bevy::remote::RemoteMethods")

where [RemoteMethods](../remote/struct.RemoteMethods.html "struct bevy::remote::RemoteMethods"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_remote/0.19.0/x86_64-unknown-linux-gnu/src/bevy_remote/lib.rs.html#996)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [RemoteWatchingRequests](../remote/struct.RemoteWatchingRequests.html "struct bevy::remote::RemoteWatchingRequests")

where [RemoteWatchingRequests](../remote/struct.RemoteWatchingRequests.html "struct bevy::remote::RemoteWatchingRequests"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/renderer/mod.rs.html#129)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [RenderAdapter](../render/renderer/struct.RenderAdapter.html "struct bevy::render::renderer::RenderAdapter")

where [RenderAdapter](../render/renderer/struct.RenderAdapter.html "struct bevy::render::renderer::RenderAdapter"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/renderer/mod.rs.html#138)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [RenderAdapterInfo](../render/renderer/struct.RenderAdapterInfo.html "struct bevy::render::renderer::RenderAdapterInfo")

where [RenderAdapterInfo](../render/renderer/struct.RenderAdapterInfo.html "struct bevy::render::renderer::RenderAdapterInfo"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/pipelined_rendering.rs.html#21)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [RenderAppChannels](../render/pipelined_rendering/struct.RenderAppChannels.html "struct bevy::render::pipelined_rendering::RenderAppChannels")

where [RenderAppChannels](../render/pipelined_rendering/struct.RenderAppChannels.html "struct bevy::render::pipelined_rendering::RenderAppChannels"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_asset.rs.html#495)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [RenderAssetBytesPerFrame](../render/render_asset/struct.RenderAssetBytesPerFrame.html "struct bevy::render::render_asset::RenderAssetBytesPerFrame")

where [RenderAssetBytesPerFrame](../render/render_asset/struct.RenderAssetBytesPerFrame.html "struct bevy::render::render_asset::RenderAssetBytesPerFrame"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_asset.rs.html#518)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [RenderAssetBytesPerFrameLimiter](../render/render_asset/struct.RenderAssetBytesPerFrameLimiter.html "struct bevy::render::render_asset::RenderAssetBytesPerFrameLimiter")

where [RenderAssetBytesPerFrameLimiter](../render/render_asset/struct.RenderAssetBytesPerFrameLimiter.html "struct bevy::render::render_asset::RenderAssetBytesPerFrameLimiter"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/decal/clustered.rs.html#67)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [RenderClusteredDecals](../pbr/decal/clustered/struct.RenderClusteredDecals.html "struct bevy::pbr::decal::clustered::RenderClusteredDecals")

where [RenderClusteredDecals](../pbr/decal/clustered/struct.RenderClusteredDecals.html "struct bevy::pbr::decal::clustered::RenderClusteredDecals"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/renderer/render_device.rs.html#14)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [RenderDevice](../render/renderer/struct.RenderDevice.html "struct bevy::render::renderer::RenderDevice")

where [RenderDevice](../render/renderer/struct.RenderDevice.html "struct bevy::render::renderer::RenderDevice"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/error_handler.rs.html#46)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [RenderErrorHandler](../render/error_handler/struct.RenderErrorHandler.html "struct bevy::render::error_handler::RenderErrorHandler")

where [RenderErrorHandler](../render/error_handler/struct.RenderErrorHandler.html "struct bevy::render::error_handler::RenderErrorHandler"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#1675)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [RenderGpuCulledEntities](../pbr/struct.RenderGpuCulledEntities.html "struct bevy::pbr::RenderGpuCulledEntities")

where [RenderGpuCulledEntities](../pbr/struct.RenderGpuCulledEntities.html "struct bevy::pbr::RenderGpuCulledEntities"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/renderer/mod.rs.html#134)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [RenderInstance](../render/renderer/struct.RenderInstance.html "struct bevy::render::renderer::RenderInstance")

where [RenderInstance](../render/renderer/struct.RenderInstance.html "struct bevy::render::renderer::RenderInstance"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/lightmap/mod.rs.html#141)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [RenderLightmaps](../pbr/struct.RenderLightmaps.html "struct bevy::pbr::RenderLightmaps")

where [RenderLightmaps](../pbr/struct.RenderLightmaps.html "struct bevy::pbr::RenderLightmaps"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/mesh2d/material.rs.html#1064)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [RenderMaterial2dBindGroupIds](../sprite_render/struct.RenderMaterial2dBindGroupIds.html "struct bevy::sprite_render::RenderMaterial2dBindGroupIds")

where [RenderMaterial2dBindGroupIds](../sprite_render/struct.RenderMaterial2dBindGroupIds.html "struct bevy::sprite_render::RenderMaterial2dBindGroupIds"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/mesh2d/material.rs.html#1067)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [RenderMaterial2dIds](../sprite_render/struct.RenderMaterial2dIds.html "struct bevy::sprite_render::RenderMaterial2dIds")

where [RenderMaterial2dIds](../sprite_render/struct.RenderMaterial2dIds.html "struct bevy::sprite_render::RenderMaterial2dIds"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/material.rs.html#1438)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [RenderMaterialBindings](../pbr/struct.RenderMaterialBindings.html "struct bevy::pbr::RenderMaterialBindings")

where [RenderMaterialBindings](../pbr/struct.RenderMaterialBindings.html "struct bevy::pbr::RenderMaterialBindings"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/material.rs.html#570)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [RenderMaterialInstances](../pbr/struct.RenderMaterialInstances.html "struct bevy::pbr::RenderMaterialInstances")

where [RenderMaterialInstances](../pbr/struct.RenderMaterialInstances.html "struct bevy::pbr::RenderMaterialInstances"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/mesh2d/mesh.rs.html#262)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [RenderMesh2dInstances](../sprite_render/struct.RenderMesh2dInstances.html "struct bevy::sprite_render::RenderMesh2dInstances")

where [RenderMesh2dInstances](../sprite_render/struct.RenderMesh2dInstances.html "struct bevy::sprite_render::RenderMesh2dInstances"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#1084)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [RenderMeshInstanceGpuQueues](../pbr/struct.RenderMeshInstanceGpuQueues.html "struct bevy::pbr::RenderMeshInstanceGpuQueues")

where [RenderMeshInstanceGpuQueues](../pbr/struct.RenderMeshInstanceGpuQueues.html "struct bevy::pbr::RenderMeshInstanceGpuQueues"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#1175)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [RenderMeshInstances](../pbr/enum.RenderMeshInstances.html "enum bevy::pbr::RenderMeshInstances")

where [RenderMeshInstances](../pbr/enum.RenderMeshInstances.html "enum bevy::pbr::RenderMeshInstances"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/mesh/morph.rs.html#118)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [RenderMorphTargetAllocator](../render/mesh/morph/enum.RenderMorphTargetAllocator.html "enum bevy::render::mesh::morph::RenderMorphTargetAllocator")

where [RenderMorphTargetAllocator](../render/mesh/morph/enum.RenderMorphTargetAllocator.html "enum bevy::render::mesh::morph::RenderMorphTargetAllocator"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/renderer/mod.rs.html#124)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [RenderQueue](../render/renderer/struct.RenderQueue.html "struct bevy::render::renderer::RenderQueue")

where [RenderQueue](../render/renderer/struct.RenderQueue.html "struct bevy::render::renderer::RenderQueue"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/lib.rs.html#250)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [RenderScheduleOrder](../render/struct.RenderScheduleOrder.html "struct bevy::render::RenderScheduleOrder")

where [RenderScheduleOrder](../render/struct.RenderScheduleOrder.html "struct bevy::render::RenderScheduleOrder"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/view/mod.rs.html#607)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [RenderShadowLodOrigin](../render/view/struct.RenderShadowLodOrigin.html "struct bevy::render::view::RenderShadowLodOrigin")

where [RenderShadowLodOrigin](../render/view/struct.RenderShadowLodOrigin.html "struct bevy::render::view::RenderShadowLodOrigin"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/view/visibility/range.rs.html#62)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [RenderVisibilityRanges](../render/view/struct.RenderVisibilityRanges.html "struct bevy::render::view::RenderVisibilityRanges")

where [RenderVisibilityRanges](../render/view/struct.RenderVisibilityRanges.html "struct bevy::render::view::RenderVisibilityRanges"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/wireframe.rs.html#966)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for bevy::pbr::wireframe::[RenderWireframeInstances](../pbr/wireframe/struct.RenderWireframeInstances.html "struct bevy::pbr::wireframe::RenderWireframeInstances")

where [RenderWireframeInstances](../pbr/wireframe/struct.RenderWireframeInstances.html "struct bevy::pbr::wireframe::RenderWireframeInstances"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/mesh2d/wireframe2d.rs.html#474)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for bevy::sprite\_render::[RenderWireframeInstances](../sprite_render/struct.RenderWireframeInstances.html "struct bevy::sprite_render::RenderWireframeInstances")

where [RenderWireframeInstances](../sprite_render/struct.RenderWireframeInstances.html "struct bevy::sprite_render::RenderWireframeInstances"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/parley_context.rs.html#179)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [ScaleCx](../text/struct.ScaleCx.html "struct bevy::text::ScaleCx")

where [ScaleCx](../text/struct.ScaleCx.html "struct bevy::text::ScaleCx"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/medium.rs.html#218)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [ScatteringMediumSampler](../pbr/struct.ScatteringMediumSampler.html "struct bevy::pbr::ScatteringMediumSampler")

where [ScatteringMediumSampler](../pbr/struct.ScatteringMediumSampler.html "struct bevy::pbr::ScatteringMediumSampler"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/schedule.rs.html#45)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [Schedules](struct.Schedules.html "struct bevy::prelude::Schedules")

where [Schedules](struct.Schedules.html "struct bevy::prelude::Schedules"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_remote/0.19.0/x86_64-unknown-linux-gnu/src/bevy_remote/schemas/mod.rs.html#18)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [SchemaTypesMetadata](../remote/schemas/struct.SchemaTypesMetadata.html "struct bevy::remote::schemas::SchemaTypesMetadata")

where [SchemaTypesMetadata](../remote/schemas/struct.SchemaTypesMetadata.html "struct bevy::remote::schemas::SchemaTypesMetadata"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/ssr/mod.rs.html#178)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [ScreenSpaceReflectionsBuffer](../pbr/struct.ScreenSpaceReflectionsBuffer.html "struct bevy::pbr::ScreenSpaceReflectionsBuffer")

where [ScreenSpaceReflectionsBuffer](../pbr/struct.ScreenSpaceReflectionsBuffer.html "struct bevy::pbr::ScreenSpaceReflectionsBuffer"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/ssr/mod.rs.html#165)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [ScreenSpaceReflectionsPipeline](../pbr/struct.ScreenSpaceReflectionsPipeline.html "struct bevy::pbr::ScreenSpaceReflectionsPipeline")

where [ScreenSpaceReflectionsPipeline](../pbr/struct.ScreenSpaceReflectionsPipeline.html "struct bevy::pbr::ScreenSpaceReflectionsPipeline"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/view/window/screenshot.rs.html#445)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [ScreenshotToScreenPipeline](../render/view/window/screenshot/struct.ScreenshotToScreenPipeline.html "struct bevy::render::view::window::screenshot::ScreenshotToScreenPipeline")

where [ScreenshotToScreenPipeline](../render/view/window/screenshot/struct.ScreenshotToScreenPipeline.html "struct bevy::render::view::window::screenshot::ScreenshotToScreenPipeline"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_dev_tools/0.19.0/x86_64-unknown-linux-gnu/src/bevy_dev_tools/schedule_data/plugin.rs.html#80)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [SerializeSchedulesFilePath](../dev_tools/schedule_data/plugin/struct.SerializeSchedulesFilePath.html "struct bevy::dev_tools::schedule_data::plugin::SerializeSchedulesFilePath")

where [SerializeSchedulesFilePath](../dev_tools/schedule_data/plugin/struct.SerializeSchedulesFilePath.html "struct bevy::dev_tools::schedule_data::plugin::SerializeSchedulesFilePath"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/light.rs.html#234)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [ShadowSamplers](../pbr/struct.ShadowSamplers.html "struct bevy::pbr::ShadowSamplers")

where [ShadowSamplers](../pbr/struct.ShadowSamplers.html "struct bevy::pbr::ShadowSamplers"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/skin.rs.html#75)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [SkinUniforms](../pbr/struct.SkinUniforms.html "struct bevy::pbr::SkinUniforms")

where [SkinUniforms](../pbr/struct.SkinUniforms.html "struct bevy::pbr::SkinUniforms"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_anti_alias/0.19.0/x86_64-unknown-linux-gnu/src/bevy_anti_alias/smaa/mod.rs.html#227)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [SmaaInfoUniformBuffer](../anti_alias/smaa/struct.SmaaInfoUniformBuffer.html "struct bevy::anti_alias::smaa::SmaaInfoUniformBuffer")

where [SmaaInfoUniformBuffer](../anti_alias/smaa/struct.SmaaInfoUniformBuffer.html "struct bevy::anti_alias::smaa::SmaaInfoUniformBuffer"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_anti_alias/0.19.0/x86_64-unknown-linux-gnu/src/bevy_anti_alias/smaa/mod.rs.html#136)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [SmaaPipelines](../anti_alias/smaa/struct.SmaaPipelines.html "struct bevy::anti_alias::smaa::SmaaPipelines")

where [SmaaPipelines](../anti_alias/smaa/struct.SmaaPipelines.html "struct bevy::anti_alias::smaa::SmaaPipelines"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_anti_alias/0.19.0/x86_64-unknown-linux-gnu/src/bevy_anti_alias/smaa/mod.rs.html#276)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [SmaaSpecializedRenderPipelines](../anti_alias/smaa/struct.SmaaSpecializedRenderPipelines.html "struct bevy::anti_alias::smaa::SmaaSpecializedRenderPipelines")

where [SmaaSpecializedRenderPipelines](../anti_alias/smaa/struct.SmaaSpecializedRenderPipelines.html "struct bevy::anti_alias::smaa::SmaaSpecializedRenderPipelines"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/camera.rs.html#715)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [SortedCameras](../render/camera/struct.SortedCameras.html "struct bevy::render::camera::SortedCameras")

where [SortedCameras](../render/camera/struct.SortedCameras.html "struct bevy::render::camera::SortedCameras"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_resource/sparse_buffer_vec.rs.html#126)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [SparseBufferUpdateBindGroups](../render/render_resource/struct.SparseBufferUpdateBindGroups.html "struct bevy::render::render_resource::SparseBufferUpdateBindGroups")

where [SparseBufferUpdateBindGroups](../render/render_resource/struct.SparseBufferUpdateBindGroups.html "struct bevy::render::render_resource::SparseBufferUpdateBindGroups"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_resource/sparse_buffer_vec.rs.html#145)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [SparseBufferUpdateJobs](../render/render_resource/struct.SparseBufferUpdateJobs.html "struct bevy::render::render_resource::SparseBufferUpdateJobs")

where [SparseBufferUpdateJobs](../render/render_resource/struct.SparseBufferUpdateJobs.html "struct bevy::render::render_resource::SparseBufferUpdateJobs"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_resource/sparse_buffer_vec.rs.html#113)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [SparseBufferUpdatePipelines](../render/render_resource/struct.SparseBufferUpdatePipelines.html "struct bevy::render::render_resource::SparseBufferUpdatePipelines")

where [SparseBufferUpdatePipelines](../render/render_resource/struct.SparseBufferUpdatePipelines.html "struct bevy::render::render_resource::SparseBufferUpdatePipelines"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/material.rs.html#817)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [SpecializedMaterialPipelineCache](../pbr/struct.SpecializedMaterialPipelineCache.html "struct bevy::pbr::SpecializedMaterialPipelineCache")

where [SpecializedMaterialPipelineCache](../pbr/struct.SpecializedMaterialPipelineCache.html "struct bevy::pbr::SpecializedMaterialPipelineCache"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/prepass/mod.rs.html#777)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [SpecializedPrepassMaterialPipelineCache](../pbr/struct.SpecializedPrepassMaterialPipelineCache.html "struct bevy::pbr::SpecializedPrepassMaterialPipelineCache")

where [SpecializedPrepassMaterialPipelineCache](../pbr/struct.SpecializedPrepassMaterialPipelineCache.html "struct bevy::pbr::SpecializedPrepassMaterialPipelineCache"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/light.rs.html#2199)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [SpecializedShadowMaterialPipelineCache](../pbr/struct.SpecializedShadowMaterialPipelineCache.html "struct bevy::pbr::SpecializedShadowMaterialPipelineCache")

where [SpecializedShadowMaterialPipelineCache](../pbr/struct.SpecializedShadowMaterialPipelineCache.html "struct bevy::pbr::SpecializedShadowMaterialPipelineCache"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/wireframe.rs.html#980)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for bevy::pbr::wireframe::[SpecializedWireframePipelineCache](../pbr/wireframe/struct.SpecializedWireframePipelineCache.html "struct bevy::pbr::wireframe::SpecializedWireframePipelineCache")

where [SpecializedWireframePipelineCache](../pbr/wireframe/struct.SpecializedWireframePipelineCache.html "struct bevy::pbr::wireframe::SpecializedWireframePipelineCache"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/mesh2d/wireframe2d.rs.html#489)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for bevy::sprite\_render::[SpecializedWireframePipelineCache](../sprite_render/struct.SpecializedWireframePipelineCache.html "struct bevy::sprite_render::SpecializedWireframePipelineCache")

where [SpecializedWireframePipelineCache](../sprite_render/struct.SpecializedWireframePipelineCache.html "struct bevy::sprite_render::SpecializedWireframePipelineCache"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/render/mod.rs.html#343)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [SpriteAssetEvents](../sprite_render/struct.SpriteAssetEvents.html "struct bevy::sprite_render::SpriteAssetEvents")

where [SpriteAssetEvents](../sprite_render/struct.SpriteAssetEvents.html "struct bevy::sprite_render::SpriteAssetEvents"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/render/mod.rs.html#485)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [SpriteBatches](../sprite_render/struct.SpriteBatches.html "struct bevy::sprite_render::SpriteBatches")

where [SpriteBatches](../sprite_render/struct.SpriteBatches.html "struct bevy::sprite_render::SpriteBatches"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/render/mod.rs.html#465)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [SpriteMeta](../sprite_render/struct.SpriteMeta.html "struct bevy::sprite_render::SpriteMeta")

where [SpriteMeta](../sprite_render/struct.SpriteMeta.html "struct bevy::sprite_render::SpriteMeta"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_sprite/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite/picking_backend.rs.html#51)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [SpritePickingSettings](struct.SpritePickingSettings.html "struct bevy::prelude::SpritePickingSettings")

where [SpritePickingSettings](struct.SpritePickingSettings.html "struct bevy::prelude::SpritePickingSettings"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/render/mod.rs.html#54)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [SpritePipeline](../sprite_render/struct.SpritePipeline.html "struct bevy::sprite_render::SpritePipeline")

where [SpritePipeline](../sprite_render/struct.SpritePipeline.html "struct bevy::sprite_render::SpritePipeline"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/systems.rs.html#87)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [StaticTransformOptimizations](enum.StaticTransformOptimizations.html "enum bevy::prelude::StaticTransformOptimizations")

where [StaticTransformOptimizations](enum.StaticTransformOptimizations.html "enum bevy::prelude::StaticTransformOptimizations"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/stepping.rs.html#94)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [Stepping](../ecs/schedule/struct.Stepping.html "struct bevy::ecs::schedule::Stepping")

where [Stepping](../ecs/schedule/struct.Stepping.html "struct bevy::ecs::schedule::Stepping"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_diagnostic/0.19.0/x86_64-unknown-linux-gnu/src/bevy_diagnostic/system_information_diagnostics_plugin.rs.html#47)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [SystemInfo](../diagnostic/struct.SystemInfo.html "struct bevy::diagnostic::SystemInfo")

where [SystemInfo](../diagnostic/struct.SystemInfo.html "struct bevy::diagnostic::SystemInfo"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_anti_alias/0.19.0/x86_64-unknown-linux-gnu/src/bevy_anti_alias/taa/mod.rs.html#219)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [TaaPipeline](../anti_alias/taa/struct.TaaPipeline.html "struct bevy::anti_alias::taa::TaaPipeline")

where [TaaPipeline](../anti_alias/taa/struct.TaaPipeline.html "struct bevy::anti_alias::taa::TaaPipeline"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text_access.rs.html#19)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [TextIterScratch](../text/struct.TextIterScratch.html "struct bevy::text::TextIterScratch")

where [TextIterScratch](../text/struct.TextIterScratch.html "struct bevy::text::TextIterScratch"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/pipeline.rs.html#43)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [TextPipeline](../text/struct.TextPipeline.html "struct bevy::text::TextPipeline")

where [TextPipeline](../text/struct.TextPipeline.html "struct bevy::text::TextPipeline"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/texture/texture_cache.rs.html#30)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [TextureCache](../render/texture/struct.TextureCache.html "struct bevy::render::texture::TextureCache")

where [TextureCache](../render/texture/struct.TextureCache.html "struct bevy::render::texture::TextureCache"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/graph.rs.html#288)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [ThreadedAnimationGraphs](struct.ThreadedAnimationGraphs.html "struct bevy::prelude::ThreadedAnimationGraphs")

where [ThreadedAnimationGraphs](struct.ThreadedAnimationGraphs.html "struct bevy::prelude::ThreadedAnimationGraphs"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/tilemap_chunk/mod.rs.html#46)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [TilemapChunkMeshCache](../sprite_render/struct.TilemapChunkMeshCache.html "struct bevy::sprite_render::TilemapChunkMeshCache")

where [TilemapChunkMeshCache](../sprite_render/struct.TilemapChunkMeshCache.html "struct bevy::sprite_render::TilemapChunkMeshCache"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/lib.rs.html#127)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [TimeReceiver](../time/struct.TimeReceiver.html "struct bevy::time::TimeReceiver")

where [TimeReceiver](../time/struct.TimeReceiver.html "struct bevy::time::TimeReceiver"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/lib.rs.html#132)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [TimeSender](../time/struct.TimeSender.html "struct bevy::time::TimeSender")

where [TimeSender](../time/struct.TimeSender.html "struct bevy::time::TimeSender"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/lib.rs.html#106)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [TimeUpdateStrategy](../time/enum.TimeUpdateStrategy.html "enum bevy::time::TimeUpdateStrategy")

where [TimeUpdateStrategy](../time/enum.TimeUpdateStrategy.html "enum bevy::time::TimeUpdateStrategy"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/tonemapping/mod.rs.html#35)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [TonemappingLuts](../core_pipeline/tonemapping/struct.TonemappingLuts.html "struct bevy::core_pipeline::tonemapping::TonemappingLuts")

where [TonemappingLuts](../core_pipeline/tonemapping/struct.TonemappingLuts.html "struct bevy::core_pipeline::tonemapping::TonemappingLuts"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/tonemapping/mod.rs.html#105)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [TonemappingPipeline](../core_pipeline/tonemapping/struct.TonemappingPipeline.html "struct bevy::core_pipeline::tonemapping::TonemappingPipeline")

where [TonemappingPipeline](../core_pipeline/tonemapping/struct.TonemappingPipeline.html "struct bevy::core_pipeline::tonemapping::TonemappingPipeline"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/touch.rs.html#248)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [Touches](struct.Touches.html "struct bevy::prelude::Touches")

where [Touches](struct.Touches.html "struct bevy::prelude::Touches"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/transform_gizmo.rs.html#136)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [TransformGizmoSettings](struct.TransformGizmoSettings.html "struct bevy::prelude::TransformGizmoSettings")

where [TransformGizmoSettings](struct.TransformGizmoSettings.html "struct bevy::prelude::TransformGizmoSettings"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/transform_gizmo.rs.html#179)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [TransformGizmoState](struct.TransformGizmoState.html "struct bevy::prelude::TransformGizmoState")

where [TransformGizmoState](struct.TransformGizmoState.html "struct bevy::prelude::TransformGizmoState"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_ui_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_render/lib.rs.html#1459)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [UiMeta](../ui_render/struct.UiMeta.html "struct bevy::ui_render::UiMeta")

where [UiMeta](../ui_render/struct.UiMeta.html "struct bevy::ui_render::UiMeta"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/picking_backend.rs.html#45)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [UiPickingSettings](struct.UiPickingSettings.html "struct bevy::prelude::UiPickingSettings")

where [UiPickingSettings](struct.UiPickingSettings.html "struct bevy::prelude::UiPickingSettings"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_ui_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_render/pipeline.rs.html#14)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [UiPipeline](../ui_render/struct.UiPipeline.html "struct bevy::ui_render::UiPipeline")

where [UiPipeline](../ui_render/struct.UiPipeline.html "struct bevy::ui_render::UiPipeline"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/lib.rs.html#124)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [UiScale](struct.UiScale.html "struct bevy::prelude::UiScale")

where [UiScale](struct.UiScale.html "struct bevy::prelude::UiScale"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/stack.rs.html#25)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [UiStack](../ui/struct.UiStack.html "struct bevy::ui::UiStack")

where [UiStack](../ui/struct.UiStack.html "struct bevy::ui::UiStack"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/layout/ui_surface.rs.html#59)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [UiSurface](../ui/ui_surface/struct.UiSurface.html "struct bevy::ui::ui_surface::UiSurface")

where [UiSurface](../ui/ui_surface/struct.UiSurface.html "struct bevy::ui::ui_surface::UiSurface"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_ui_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_render/ui_texture_slice_pipeline.rs.html#100)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [UiTextureSliceImageBindGroups](../ui_render/ui_texture_slice_pipeline/struct.UiTextureSliceImageBindGroups.html "struct bevy::ui_render::ui_texture_slice_pipeline::UiTextureSliceImageBindGroups")

where [UiTextureSliceImageBindGroups](../ui_render/ui_texture_slice_pipeline/struct.UiTextureSliceImageBindGroups.html "struct bevy::ui_render::ui_texture_slice_pipeline::UiTextureSliceImageBindGroups"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_ui_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_render/ui_texture_slice_pipeline.rs.html#83)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [UiTextureSliceMeta](../ui_render/ui_texture_slice_pipeline/struct.UiTextureSliceMeta.html "struct bevy::ui_render::ui_texture_slice_pipeline::UiTextureSliceMeta")

where [UiTextureSliceMeta](../ui_render/ui_texture_slice_pipeline/struct.UiTextureSliceMeta.html "struct bevy::ui_render::ui_texture_slice_pipeline::UiTextureSliceMeta"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_ui_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_render/ui_texture_slice_pipeline.rs.html#105)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [UiTextureSlicePipeline](../ui_render/ui_texture_slice_pipeline/struct.UiTextureSlicePipeline.html "struct bevy::ui_render::ui_texture_slice_pipeline::UiTextureSlicePipeline")

where [UiTextureSlicePipeline](../ui_render/ui_texture_slice_pipeline/struct.UiTextureSlicePipeline.html "struct bevy::ui_render::ui_texture_slice_pipeline::UiTextureSlicePipeline"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/theme.rs.html#59)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [UiTheme](../feathers/theme/struct.UiTheme.html "struct bevy::feathers::theme::UiTheme")

where [UiTheme](../feathers/theme/struct.UiTheme.html "struct bevy::feathers::theme::UiTheme"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#355)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for bevy::pbr::[ViewKeyCache](../pbr/struct.ViewKeyCache.html "struct bevy::pbr::ViewKeyCache")

where [ViewKeyCache](../pbr/struct.ViewKeyCache.html "struct bevy::pbr::ViewKeyCache"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/mesh2d/mesh.rs.html#123)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for bevy::sprite\_render::[ViewKeyCache](../sprite_render/struct.ViewKeyCache.html "struct bevy::sprite_render::ViewKeyCache")

where [ViewKeyCache](../sprite_render/struct.ViewKeyCache.html "struct bevy::sprite_render::ViewKeyCache"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/prepass/mod.rs.html#793)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [ViewKeyPrepassCache](../pbr/struct.ViewKeyPrepassCache.html "struct bevy::pbr::ViewKeyPrepassCache")

where [ViewKeyPrepassCache](../pbr/struct.ViewKeyPrepassCache.html "struct bevy::pbr::ViewKeyPrepassCache"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/view/mod.rs.html#712)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [ViewTargetAttachments](../render/view/struct.ViewTargetAttachments.html "struct bevy::render::view::ViewTargetAttachments")

where [ViewTargetAttachments](../render/view/struct.ViewTargetAttachments.html "struct bevy::render::view::ViewTargetAttachments"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/view/mod.rs.html#671)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [ViewUniforms](../render/view/struct.ViewUniforms.html "struct bevy::render::view::ViewUniforms")

where [ViewUniforms](../render/view/struct.ViewUniforms.html "struct bevy::render::view::ViewUniforms"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/visibility/range.rs.html#187)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [VisibleEntityRanges](../camera/visibility/struct.VisibleEntityRanges.html "struct bevy::camera::visibility::VisibleEntityRanges")

where [VisibleEntityRanges](../camera/visibility/struct.VisibleEntityRanges.html "struct bevy::camera::visibility::VisibleEntityRanges"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/spawn.rs.html#678)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [WaitingScenes](../scene/struct.WaitingScenes.html "struct bevy::scene::WaitingScenes")

where [WaitingScenes](../scene/struct.WaitingScenes.html "struct bevy::scene::WaitingScenes"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/view/window/mod.rs.html#209)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [WindowSurfaces](../render/view/struct.WindowSurfaces.html "struct bevy::render::view::WindowSurfaces")

where [WindowSurfaces](../render/view/struct.WindowSurfaces.html "struct bevy::render::view::WindowSurfaces"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_winit/0.19.0/x86_64-unknown-linux-gnu/src/bevy_winit/accessibility.rs.html#41)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [WinitActionRequestHandlers](../winit/accessibility/struct.WinitActionRequestHandlers.html "struct bevy::winit::accessibility::WinitActionRequestHandlers")

where [WinitActionRequestHandlers](../winit/accessibility/struct.WinitActionRequestHandlers.html "struct bevy::winit::accessibility::WinitActionRequestHandlers"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_winit/0.19.0/x86_64-unknown-linux-gnu/src/bevy_winit/winit_monitors.rs.html#14)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [WinitMonitors](../winit/struct.WinitMonitors.html "struct bevy::winit::WinitMonitors")

where [WinitMonitors](../winit/struct.WinitMonitors.html "struct bevy::winit::WinitMonitors"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_winit/0.19.0/x86_64-unknown-linux-gnu/src/bevy_winit/winit_config.rs.html#5)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [WinitSettings](../winit/struct.WinitSettings.html "struct bevy::winit::WinitSettings")

where [WinitSettings](../winit/struct.WinitSettings.html "struct bevy::winit::WinitSettings"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/mesh2d/wireframe2d.rs.html#422)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [Wireframe2dConfig](../sprite_render/struct.Wireframe2dConfig.html "struct bevy::sprite_render::Wireframe2dConfig")

where [Wireframe2dConfig](../sprite_render/struct.Wireframe2dConfig.html "struct bevy::sprite_render::Wireframe2dConfig"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/mesh2d/wireframe2d.rs.html#318)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [Wireframe2dPipeline](../sprite_render/struct.Wireframe2dPipeline.html "struct bevy::sprite_render::Wireframe2dPipeline")

where [Wireframe2dPipeline](../sprite_render/struct.Wireframe2dPipeline.html "struct bevy::sprite_render::Wireframe2dPipeline"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/wireframe.rs.html#695)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [Wireframe3dPipeline](../pbr/wireframe/struct.Wireframe3dPipeline.html "struct bevy::pbr::wireframe::Wireframe3dPipeline")

where [Wireframe3dPipeline](../pbr/wireframe/struct.Wireframe3dPipeline.html "struct bevy::pbr::wireframe::Wireframe3dPipeline"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/wireframe.rs.html#883)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [WireframeConfig](../pbr/wireframe/struct.WireframeConfig.html "struct bevy::pbr::wireframe::WireframeConfig")

where [WireframeConfig](../pbr/wireframe/struct.WireframeConfig.html "struct bevy::pbr::wireframe::WireframeConfig"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/wireframe.rs.html#972)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for bevy::pbr::wireframe::[WireframeEntitiesNeedingSpecialization](../pbr/wireframe/struct.WireframeEntitiesNeedingSpecialization.html "struct bevy::pbr::wireframe::WireframeEntitiesNeedingSpecialization")

where [WireframeEntitiesNeedingSpecialization](../pbr/wireframe/struct.WireframeEntitiesNeedingSpecialization.html "struct bevy::pbr::wireframe::WireframeEntitiesNeedingSpecialization"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/mesh2d/wireframe2d.rs.html#480)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for bevy::sprite\_render::[WireframeEntitiesNeedingSpecialization](../sprite_render/struct.WireframeEntitiesNeedingSpecialization.html "struct bevy::sprite_render::WireframeEntitiesNeedingSpecialization")

where [WireframeEntitiesNeedingSpecialization](../sprite_render/struct.WireframeEntitiesNeedingSpecialization.html "struct bevy::sprite_render::WireframeEntitiesNeedingSpecialization"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/wireframe.rs.html#407)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [WireframeWideBindGroups](../pbr/wireframe/struct.WireframeWideBindGroups.html "struct bevy::pbr::wireframe::WireframeWideBindGroups")

where [WireframeWideBindGroups](../pbr/wireframe/struct.WireframeWideBindGroups.html "struct bevy::pbr::wireframe::WireframeWideBindGroups"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_world_serialization/0.19.0/x86_64-unknown-linux-gnu/src/bevy_world_serialization/world_asset_spawner.rs.html#82)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [WorldInstanceSpawner](struct.WorldInstanceSpawner.html "struct bevy::prelude::WorldInstanceSpawner")

where [WorldInstanceSpawner](struct.WorldInstanceSpawner.html "struct bevy::prelude::WorldInstanceSpawner"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/assets.rs.html#287)

### impl<A> [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [Assets](struct.Assets.html "struct bevy::prelude::Assets")<A>

where A: [Asset](trait.Asset.html "trait bevy::prelude::Asset"), [Assets](struct.Assets.html "struct bevy::prelude::Assets")<A>: [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/erased_render_asset.rs.html#164)

### impl<A> [Resource](trait.Resource.html "trait bevy::prelude::Resource") for bevy::render::erased\_render\_asset::[ExtractedAssets](../render/erased_render_asset/struct.ExtractedAssets.html "struct bevy::render::erased_render_asset::ExtractedAssets")<A>

where A: [ErasedRenderAsset](../render/erased_render_asset/trait.ErasedRenderAsset.html "trait bevy::render::erased_render_asset::ErasedRenderAsset"), [ExtractedAssets](../render/erased_render_asset/struct.ExtractedAssets.html "struct bevy::render::erased_render_asset::ExtractedAssets")<A>: [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_asset.rs.html#174)

### impl<A> [Resource](trait.Resource.html "trait bevy::prelude::Resource") for bevy::render::render\_asset::[ExtractedAssets](../render/render_asset/struct.ExtractedAssets.html "struct bevy::render::render_asset::ExtractedAssets")<A>

where A: [RenderAsset](../render/render_asset/trait.RenderAsset.html "trait bevy::render::render_asset::RenderAsset"), [ExtractedAssets](../render/render_asset/struct.ExtractedAssets.html "struct bevy::render::render_asset::ExtractedAssets")<A>: [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/erased_render_asset.rs.html#357)

### impl<A> [Resource](trait.Resource.html "trait bevy::prelude::Resource") for bevy::render::erased\_render\_asset::[PrepareNextFrameAssets](../render/erased_render_asset/struct.PrepareNextFrameAssets.html "struct bevy::render::erased_render_asset::PrepareNextFrameAssets")<A>

where A: [ErasedRenderAsset](../render/erased_render_asset/trait.ErasedRenderAsset.html "trait bevy::render::erased_render_asset::ErasedRenderAsset"), [PrepareNextFrameAssets](../render/erased_render_asset/struct.PrepareNextFrameAssets.html "struct bevy::render::erased_render_asset::PrepareNextFrameAssets")<A>: [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_asset.rs.html#365)

### impl<A> [Resource](trait.Resource.html "trait bevy::prelude::Resource") for bevy::render::render\_asset::[PrepareNextFrameAssets](../render/render_asset/struct.PrepareNextFrameAssets.html "struct bevy::render::render_asset::PrepareNextFrameAssets")<A>

where A: [RenderAsset](../render/render_asset/trait.RenderAsset.html "trait bevy::render::render_asset::RenderAsset"), [PrepareNextFrameAssets](../render/render_asset/struct.PrepareNextFrameAssets.html "struct bevy::render::render_asset::PrepareNextFrameAssets")<A>: [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_asset.rs.html#206)

### impl<A> [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [RenderAssets](../render/render_asset/struct.RenderAssets.html "struct bevy::render::render_asset::RenderAssets")<A>

where A: [RenderAsset](../render/render_asset/trait.RenderAsset.html "trait bevy::render::render_asset::RenderAsset"), [RenderAssets](../render/render_asset/struct.RenderAssets.html "struct bevy::render::render_asset::RenderAssets")<A>: [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/batching/gpu_preprocessing.rs.html#166)

### impl<BD, BDI> [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [BatchedInstanceBuffers](../render/batching/gpu_preprocessing/struct.BatchedInstanceBuffers.html "struct bevy::render::batching::gpu_preprocessing::BatchedInstanceBuffers")<BD, BDI>

where BD: [GpuArrayBufferable](../render/render_resource/trait.GpuArrayBufferable.html "trait bevy::render::render_resource::GpuArrayBufferable") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + 'static, BDI: [AtomicPod](../render/render_resource/trait.AtomicPod.html "trait bevy::render::render_resource::AtomicPod"), [BatchedInstanceBuffers](../render/batching/gpu_preprocessing/struct.BatchedInstanceBuffers.html "struct bevy::render::batching::gpu_preprocessing::BatchedInstanceBuffers")<BD, BDI>: [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/batching/no_gpu_preprocessing.rs.html#28)

### impl<BD> [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [BatchedInstanceBuffer](../render/batching/no_gpu_preprocessing/struct.BatchedInstanceBuffer.html "struct bevy::render::batching::no_gpu_preprocessing::BatchedInstanceBuffer")<BD>

where BD: [GpuArrayBufferable](../render/render_resource/trait.GpuArrayBufferable.html "trait bevy::render::render_resource::GpuArrayBufferable") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + 'static, [BatchedInstanceBuffer](../render/batching/no_gpu_preprocessing/struct.BatchedInstanceBuffer.html "struct bevy::render::batching::no_gpu_preprocessing::BatchedInstanceBuffer")<BD>: [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_phase/mod.rs.html#91)

### impl<BPI> [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [ViewBinnedRenderPhases](../render/render_phase/struct.ViewBinnedRenderPhases.html "struct bevy::render::render_phase::ViewBinnedRenderPhases")<BPI>

where BPI: [BinnedPhaseItem](../render/render_phase/trait.BinnedPhaseItem.html "trait bevy::render::render_phase::BinnedPhaseItem"), [ViewBinnedRenderPhases](../render/render_phase/struct.ViewBinnedRenderPhases.html "struct bevy::render::render_phase::ViewBinnedRenderPhases")<BPI>: [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/uniform.rs.html#55)

### impl<C> [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [ComponentUniforms](../render/extract_component/struct.ComponentUniforms.html "struct bevy::render::extract_component::ComponentUniforms")<C>

where C: [Component](trait.Component.html "trait bevy::prelude::Component") + [ShaderType](../render/render_resource/trait.ShaderType.html "trait bevy::render::render_resource::ShaderType"), [ComponentUniforms](../render/extract_component/struct.ComponentUniforms.html "struct bevy::render::extract_component::ComponentUniforms")<C>: [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/gizmos.rs.html#32)

### impl<Config, Clear> [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [GizmoStorage](../gizmos/gizmos/struct.GizmoStorage.html "struct bevy::gizmos::gizmos::GizmoStorage")<Config, Clear>

where [GizmoStorage](../gizmos/gizmos/struct.GizmoStorage.html "struct bevy::gizmos::gizmos::GizmoStorage")<Config, Clear>: [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/extract_instances.rs.html#56)

### impl<EI> [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [ExtractedInstances](../render/extract_instances/struct.ExtractedInstances.html "struct bevy::render::extract_instances::ExtractedInstances")<EI>

where EI: [ExtractInstance](../render/extract_instances/trait.ExtractInstance.html "trait bevy::render::extract_instances::ExtractInstance"), [ExtractedInstances](../render/extract_instances/struct.ExtractedInstances.html "struct bevy::render::extract_instances::ExtractedInstances")<EI>: [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/erased_render_asset.rs.html#196)

### impl<ERA> [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [ErasedRenderAssets](../render/erased_render_asset/struct.ErasedRenderAssets.html "struct bevy::render::erased_render_asset::ErasedRenderAssets")<ERA>

where [ErasedRenderAssets](../render/erased_render_asset/struct.ErasedRenderAssets.html "struct bevy::render::erased_render_asset::ErasedRenderAssets")<ERA>: [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/material.rs.html#794)

### impl<M> [Resource](trait.Resource.html "trait bevy::prelude::Resource") for bevy::pbr::[EntitiesNeedingSpecialization](../pbr/struct.EntitiesNeedingSpecialization.html "struct bevy::pbr::EntitiesNeedingSpecialization")<M>

where [EntitiesNeedingSpecialization](../pbr/struct.EntitiesNeedingSpecialization.html "struct bevy::pbr::EntitiesNeedingSpecialization")<M>: [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/mesh2d/material.rs.html#619)

### impl<M> [Resource](trait.Resource.html "trait bevy::prelude::Resource") for bevy::sprite\_render::[EntitiesNeedingSpecialization](../sprite_render/struct.EntitiesNeedingSpecialization.html "struct bevy::sprite_render::EntitiesNeedingSpecialization")<M>

where [EntitiesNeedingSpecialization](../sprite_render/struct.EntitiesNeedingSpecialization.html "struct bevy::sprite_render::EntitiesNeedingSpecialization")<M>: [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_ui_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_render/ui_material_pipeline.rs.html#308)

### impl<M> [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [ExtractedUiMaterialNodes](../ui_render/struct.ExtractedUiMaterialNodes.html "struct bevy::ui_render::ExtractedUiMaterialNodes")<M>

where M: [UiMaterial](trait.UiMaterial.html "trait bevy::prelude::UiMaterial"), [ExtractedUiMaterialNodes](../ui_render/struct.ExtractedUiMaterialNodes.html "struct bevy::ui_render::ExtractedUiMaterialNodes")<M>: [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/mesh2d/material.rs.html#398)

### impl<M> [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [Material2dPipeline](../sprite_render/struct.Material2dPipeline.html "struct bevy::sprite_render::Material2dPipeline")<M>

where M: [Material2d](../sprite_render/trait.Material2d.html "trait bevy::sprite_render::Material2d"), [Material2dPipeline](../sprite_render/struct.Material2dPipeline.html "struct bevy::sprite_render::Material2dPipeline")<M>: [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/message/messages.rs.html#93)

### impl<M> [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [Messages](struct.Messages.html "struct bevy::prelude::Messages")<M>

where M: [Message](trait.Message.html "trait bevy::prelude::Message"), [Messages](struct.Messages.html "struct bevy::prelude::Messages")<M>: [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/mesh2d/material.rs.html#331)

### impl<M> [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [RenderMaterial2dInstances](../sprite_render/struct.RenderMaterial2dInstances.html "struct bevy::sprite_render::RenderMaterial2dInstances")<M>

where M: [Material2d](../sprite_render/trait.Material2d.html "trait bevy::sprite_render::Material2d"), [RenderMaterial2dInstances](../sprite_render/struct.RenderMaterial2dInstances.html "struct bevy::sprite_render::RenderMaterial2dInstances")<M>: [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/mesh2d/material.rs.html#639)

### impl<M> [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [SpecializedMaterial2dPipelineCache](../sprite_render/struct.SpecializedMaterial2dPipelineCache.html "struct bevy::sprite_render::SpecializedMaterial2dPipelineCache")<M>

where [SpecializedMaterial2dPipelineCache](../sprite_render/struct.SpecializedMaterial2dPipelineCache.html "struct bevy::sprite_render::SpecializedMaterial2dPipelineCache")<M>: [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_ui_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_render/ui_material_pipeline.rs.html#77)

### impl<M> [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [UiMaterialMeta](../ui_render/struct.UiMaterialMeta.html "struct bevy::ui_render::UiMaterialMeta")<M>

where M: [UiMaterial](trait.UiMaterial.html "trait bevy::prelude::UiMaterial"), [UiMaterialMeta](../ui_render/struct.UiMaterialMeta.html "struct bevy::ui_render::UiMaterialMeta")<M>: [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_ui_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_render/ui_material_pipeline.rs.html#114)

### impl<M> [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [UiMaterialPipeline](../ui_render/struct.UiMaterialPipeline.html "struct bevy::ui_render::UiMaterialPipeline")<M>

where M: [UiMaterial](trait.UiMaterial.html "trait bevy::prelude::UiMaterial"), [UiMaterialPipeline](../ui_render/struct.UiMaterialPipeline.html "struct bevy::ui_render::UiMaterialPipeline")<M>: [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_phase/draw.rs.html#113)

### impl<P> [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [DrawFunctions](../render/render_phase/struct.DrawFunctions.html "struct bevy::render::render_phase::DrawFunctions")<P>

where P: [PhaseItem](../render/render_phase/trait.PhaseItem.html "trait bevy::render::render_phase::PhaseItem"), [DrawFunctions](../render/render_phase/struct.DrawFunctions.html "struct bevy::render::render_phase::DrawFunctions")<P>: [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/batching/gpu_preprocessing.rs.html#216)

### impl<PI, BD> [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [PhaseBatchedInstanceBuffers](../render/batching/gpu_preprocessing/struct.PhaseBatchedInstanceBuffers.html "struct bevy::render::batching::gpu_preprocessing::PhaseBatchedInstanceBuffers")<PI, BD>

where PI: [PhaseItem](../render/render_phase/trait.PhaseItem.html "trait bevy::render::render_phase::PhaseItem"), BD: [GpuArrayBufferable](../render/render_resource/trait.GpuArrayBufferable.html "trait bevy::render::render_resource::GpuArrayBufferable") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + 'static, [PhaseBatchedInstanceBuffers](../render/batching/gpu_preprocessing/struct.PhaseBatchedInstanceBuffers.html "struct bevy::render::batching::gpu_preprocessing::PhaseBatchedInstanceBuffers")<PI, BD>: [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/batching/gpu_preprocessing.rs.html#980)

### impl<PI> [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [PhaseIndirectParametersBuffers](../render/batching/gpu_preprocessing/struct.PhaseIndirectParametersBuffers.html "struct bevy::render::batching::gpu_preprocessing::PhaseIndirectParametersBuffers")<PI>

where PI: [PhaseItem](../render/render_phase/trait.PhaseItem.html "trait bevy::render::render_phase::PhaseItem"), [PhaseIndirectParametersBuffers](../render/batching/gpu_preprocessing/struct.PhaseIndirectParametersBuffers.html "struct bevy::render::batching::gpu_preprocessing::PhaseIndirectParametersBuffers")<PI>: [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_registry.rs.html#426)

### impl<S> [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [CachedSystemId](../ecs/system/struct.CachedSystemId.html "struct bevy::ecs::system::CachedSystemId")<S>

where [CachedSystemId](../ecs/system/struct.CachedSystemId.html "struct bevy::ecs::system::CachedSystemId")<S>: [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_state/0.19.0/x86_64-unknown-linux-gnu/src/bevy_state/state/resources.rs.html#175)

### impl<S> [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [NextState](enum.NextState.html "enum bevy::prelude::NextState")<S>

where S: [FreelyMutableState](../state/state/trait.FreelyMutableState.html "trait bevy::state::state::FreelyMutableState"), [NextState](enum.NextState.html "enum bevy::prelude::NextState")<S>: [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_state/0.19.0/x86_64-unknown-linux-gnu/src/bevy_state/state/resources.rs.html#128)

### impl<S> [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [PreviousState](struct.PreviousState.html "struct bevy::prelude::PreviousState")<S>

where S: [States](trait.States.html "trait bevy::prelude::States"), [PreviousState](struct.PreviousState.html "struct bevy::prelude::PreviousState")<S>: [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_resource/pipeline_specializer.rs.html#105)

### impl<S> [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [SpecializedComputePipelines](../render/render_resource/struct.SpecializedComputePipelines.html "struct bevy::render::render_resource::SpecializedComputePipelines")<S>

where S: [SpecializedComputePipeline](../render/render_resource/trait.SpecializedComputePipeline.html "trait bevy::render::render_resource::SpecializedComputePipeline"), [SpecializedComputePipelines](../render/render_resource/struct.SpecializedComputePipelines.html "struct bevy::render::render_resource::SpecializedComputePipelines")<S>: [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_resource/pipeline_specializer.rs.html#152)

### impl<S> [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [SpecializedMeshPipelines](../render/render_resource/struct.SpecializedMeshPipelines.html "struct bevy::render::render_resource::SpecializedMeshPipelines")<S>

where S: [SpecializedMeshPipeline](../render/render_resource/trait.SpecializedMeshPipeline.html "trait bevy::render::render_resource::SpecializedMeshPipeline"), [SpecializedMeshPipelines](../render/render_resource/struct.SpecializedMeshPipelines.html "struct bevy::render::render_resource::SpecializedMeshPipelines")<S>: [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_resource/pipeline_specializer.rs.html#50)

### impl<S> [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [SpecializedRenderPipelines](../render/render_resource/struct.SpecializedRenderPipelines.html "struct bevy::render::render_resource::SpecializedRenderPipelines")<S>

where S: [SpecializedRenderPipeline](../render/render_resource/trait.SpecializedRenderPipeline.html "trait bevy::render::render_resource::SpecializedRenderPipeline"), [SpecializedRenderPipelines](../render/render_resource/struct.SpecializedRenderPipelines.html "struct bevy::render::render_resource::SpecializedRenderPipelines")<S>: [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_state/0.19.0/x86_64-unknown-linux-gnu/src/bevy_state/state/resources.rs.html#52)

### impl<S> [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [State](struct.State.html "struct bevy::prelude::State")<S>

where S: [States](trait.States.html "trait bevy::prelude::States"), [State](struct.State.html "struct bevy::prelude::State")<S>: [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_phase/mod.rs.html#1567)

### impl<SPI> [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [ViewSortedRenderPhases](../render/render_phase/struct.ViewSortedRenderPhases.html "struct bevy::render::render_phase::ViewSortedRenderPhases")<SPI>

where SPI: [SortedPhaseItem](../render/render_phase/trait.SortedPhaseItem.html "trait bevy::render::render_phase::SortedPhaseItem"), [ViewSortedRenderPhases](../render/render_phase/struct.ViewSortedRenderPhases.html "struct bevy::render::render_phase::ViewSortedRenderPhases")<SPI>: [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/axis.rs.html#15)

### impl<T> [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [Axis](struct.Axis.html "struct bevy::prelude::Axis")<T>

where [Axis](struct.Axis.html "struct bevy::prelude::Axis")<T>: [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/button_input.rs.html#123)

### impl<T> [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [ButtonInput](struct.ButtonInput.html "struct bevy::prelude::ButtonInput")<T>

where T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static, [ButtonInput](struct.ButtonInput.html "struct bevy::prelude::ButtonInput")<T>: [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/fullscreen_material.rs.html#100)

### impl<T> [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [FullscreenMaterialPipeline](../core_pipeline/fullscreen_material/struct.FullscreenMaterialPipeline.html "struct bevy::core_pipeline::fullscreen_material::FullscreenMaterialPipeline")<T>

where T: [FullscreenMaterial](../core_pipeline/fullscreen_material/trait.FullscreenMaterial.html "trait bevy::core_pipeline::fullscreen_material::FullscreenMaterial"), [FullscreenMaterialPipeline](../core_pipeline/fullscreen_material/struct.FullscreenMaterialPipeline.html "struct bevy::core_pipeline::fullscreen_material::FullscreenMaterialPipeline")<T>: [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_resource/gpu_array_buffer.rs.html#37)

### impl<T> [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [GpuArrayBuffer](../render/render_resource/enum.GpuArrayBuffer.html "enum bevy::render::render_resource::GpuArrayBuffer")<T>

where T: [GpuArrayBufferable](../render/render_resource/trait.GpuArrayBufferable.html "trait bevy::render::render_resource::GpuArrayBufferable"), [GpuArrayBuffer](../render/render_resource/enum.GpuArrayBuffer.html "enum bevy::render::render_resource::GpuArrayBuffer")<T>: [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/time.rs.html#190)

### impl<T> [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [Time](struct.Time.html "struct bevy::prelude::Time")<T>

where T: [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default"), [Time](struct.Time.html "struct bevy::prelude::Time")<T>: [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,