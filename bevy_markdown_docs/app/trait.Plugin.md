[bevy](../index.html)::[app](index.html)

# Trait Plugin 

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/plugin.rs.html#57)

```rust
pub trait Plugin:
    Downcast
    + Any
    + Send
    + Sync {
    // Required method
    fn build(&self, app: &mut App);

    // Provided methods
    fn ready(&self, _app: &App) -> bool { ... }
    fn finish(&self, _app: &mut App) { ... }
    fn cleanup(&self, _app: &mut App) { ... }
    fn name(&self) -> &str { ... }
    fn is_unique(&self) -> bool { ... }
}
```

A collection of Bevy app logic and configuration.

Plugins configure an [`App`](../prelude/struct.App.html "struct bevy::prelude::App"). When an [`App`](../prelude/struct.App.html "struct bevy::prelude::App") registers a plugin, the plugin’s [`Plugin::build`](../prelude/trait.Plugin.html#tymethod.build "method bevy::prelude::Plugin::build") function is run. By default, a plugin can only be added once to an [`App`](../prelude/struct.App.html "struct bevy::prelude::App").

If the plugin may need to be added twice or more, the function [`is_unique()`](../prelude/trait.Plugin.html#method.is_unique "method bevy::prelude::Plugin::is_unique") should be overridden to return `false`. Plugins are considered duplicate if they have the same [`name()`](../prelude/trait.Plugin.html#method.name "method bevy::prelude::Plugin::name"). The default `name()` implementation returns the type name, which means generic plugins with different type parameters will not be considered duplicates.

### Lifecycle of a plugin

When adding a plugin to an [`App`](../prelude/struct.App.html "struct bevy::prelude::App"):

*   the app calls [`Plugin::build`](../prelude/trait.Plugin.html#tymethod.build "method bevy::prelude::Plugin::build") immediately, and register the plugin
*   once the app started, it will wait for all registered [`Plugin::ready`](../prelude/trait.Plugin.html#method.ready "method bevy::prelude::Plugin::ready") to return `true`
*   it will then call all registered [`Plugin::finish`](../prelude/trait.Plugin.html#method.finish "method bevy::prelude::Plugin::finish")
*   and call all registered [`Plugin::cleanup`](../prelude/trait.Plugin.html#method.cleanup "method bevy::prelude::Plugin::cleanup")

### Defining a plugin.

Most plugins are simply functions that add configuration to an [`App`](../prelude/struct.App.html "struct bevy::prelude::App").

```rust
App::new().add_plugins(my_plugin).run();

// This function implements `Plugin`, along with every other `fn(&mut App)`.
pub fn my_plugin(app: &mut App) {
    app.add_systems(Update, hello_world);
}
```

For more advanced use cases, the `Plugin` trait can be implemented manually for a type.

```rust
pub struct AccessibilityPlugin {
    pub flicker_damping: bool,
    // ...
}

impl Plugin for AccessibilityPlugin {
    fn build(&self, app: &mut App) {
        if self.flicker_damping {
            app.add_systems(PostUpdate, damp_flickering);
        }
    }
}
```

## Required Methods

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/plugin.rs.html#59)

#### fn [build](#tymethod.build)(&self, app: &mut [App](../prelude/struct.App.html "struct bevy::prelude::App"))

Configures the [`App`](../prelude/struct.App.html "struct bevy::prelude::App") to which this plugin is added.

## Provided Methods

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/plugin.rs.html#64)

#### fn [ready](#method.ready)(&self, \_app: &[App](../prelude/struct.App.html "struct bevy::prelude::App")) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Has the plugin finished its setup? This can be useful for plugins that need something asynchronous to happen before they can finish their setup, like the initialization of a renderer. Once the plugin is ready, [`finish`](../prelude/trait.Plugin.html#method.finish "method bevy::prelude::Plugin::finish") should be called.

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/plugin.rs.html#70)

#### fn [finish](#method.finish)(&self, \_app: &mut [App](../prelude/struct.App.html "struct bevy::prelude::App"))

Finish adding this plugin to the [`App`](../prelude/struct.App.html "struct bevy::prelude::App"), once all plugins registered are ready. This can be useful for plugins that depends on another plugin asynchronous setup, like the renderer.

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/plugin.rs.html#77)

#### fn [cleanup](#method.cleanup)(&self, \_app: &mut [App](../prelude/struct.App.html "struct bevy::prelude::App"))

Runs after all plugins are built and finished, but before the app schedule is executed. This can be useful if you have some resource that other plugins need during their build step, but after build you want to remove it and send it to another thread.

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/plugin.rs.html#83)

#### fn [name](#method.name)(&self) -> &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

Configures a name for the [`Plugin`](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") which is primarily used for checking plugin uniqueness and debugging.

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/plugin.rs.html#89)

#### fn [is\_unique](#method.is_unique)(&self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

If the plugin can be meaningfully instantiated several times in an [`App`](../prelude/struct.App.html "struct bevy::prelude::App"), override this method to return `false`.

## Implementations

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/plugin.rs.html#94)

### impl dyn [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin")

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/plugin.rs.html#94)

#### pub fn [is](#method.is)<\_\_T>(&self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

where \_\_T: [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin"),

Returns true if the trait object wraps an object of type `__T`.

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/plugin.rs.html#94)

#### pub fn [downcast](#method.downcast)<\_\_T>(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin")\>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<\_\_T>, [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin")\>>

where \_\_T: [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin"),

Returns a boxed object from a boxed trait object if the underlying object is of type `__T`. Returns the original boxed trait if it isn’t.

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/plugin.rs.html#94)

#### pub fn [downcast\_rc](#method.downcast_rc)<\_\_T>(self: [Rc](https://doc.rust-lang.org/nightly/alloc/rc/struct.Rc.html "struct alloc::rc::Rc")<dyn [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin")\>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Rc](https://doc.rust-lang.org/nightly/alloc/rc/struct.Rc.html "struct alloc::rc::Rc")<\_\_T>, [Rc](https://doc.rust-lang.org/nightly/alloc/rc/struct.Rc.html "struct alloc::rc::Rc")<dyn [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin")\>>

where \_\_T: [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin"),

Returns an `Rc`\-ed object from an `Rc`\-ed trait object if the underlying object is of type `__T`. Returns the original `Rc`\-ed trait if it isn’t.

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/plugin.rs.html#94)

#### pub fn [downcast\_ref](#method.downcast_ref)<\_\_T>(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[&\_\_T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)\>

where \_\_T: [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin"),

Returns a reference to the object within the trait object if it is of type `__T`, or `None` if it isn’t.

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/plugin.rs.html#94)

#### pub fn [downcast\_mut](#method.downcast_mut)<\_\_T>(&mut self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[&mut \_\_T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)\>

where \_\_T: [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin"),

Returns a mutable reference to the object within the trait object if it is of type `__T`, or `None` if it isn’t.

## Dyn Compatibility

This trait **is** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/aabb.rs.html#27)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [AabbGizmoPlugin](../gizmos/aabb/struct.AabbGizmoPlugin.html "struct bevy::gizmos::aabb::AabbGizmoPlugin")

[Source](https://docs.rs/bevy_winit/0.19.0/x86_64-unknown-linux-gnu/src/bevy_winit/accessibility.rs.html#311)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [AccessKitPlugin](../winit/accessibility/struct.AccessKitPlugin.html "struct bevy::winit::accessibility::AccessKitPlugin")

[Source](https://docs.rs/bevy_a11y/0.19.0/x86_64-unknown-linux-gnu/src/bevy_a11y/lib.rs.html#277)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [AccessibilityPlugin](../a11y/struct.AccessibilityPlugin.html "struct bevy::a11y::AccessibilityPlugin")

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/lib.rs.html#1280)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [AnimationPlugin](../prelude/struct.AnimationPlugin.html "struct bevy::prelude::AnimationPlugin")

[Source](https://docs.rs/bevy_anti_alias/0.19.0/x86_64-unknown-linux-gnu/src/bevy_anti_alias/lib.rs.html#25)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [AntiAliasPlugin](../anti_alias/struct.AntiAliasPlugin.html "struct bevy::anti_alias::AntiAliasPlugin")

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/lib.rs.html#350)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [AssetPlugin](../prelude/struct.AssetPlugin.html "struct bevy::prelude::AssetPlugin")

[Source](https://docs.rs/bevy_audio/0.19.0/x86_64-unknown-linux-gnu/src/bevy_audio/lib.rs.html#81)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [AudioPlugin](../audio/struct.AudioPlugin.html "struct bevy::audio::AudioPlugin")

[Source](https://docs.rs/bevy_post_process/0.19.0/x86_64-unknown-linux-gnu/src/bevy_post_process/auto_exposure/mod.rs.html#47)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [AutoExposurePlugin](../post_process/auto_exposure/struct.AutoExposurePlugin.html "struct bevy::post_process::auto_exposure::AutoExposurePlugin")

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/prepass/background_motion_vectors.rs.html#95)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [BackgroundMotionVectorsPlugin](../core_pipeline/prepass/struct.BackgroundMotionVectorsPlugin.html "struct bevy::core_pipeline::prepass::BackgroundMotionVectorsPlugin")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/batching/gpu_preprocessing.rs.html#61)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [BatchingPlugin](../render/batching/gpu_preprocessing/struct.BatchingPlugin.html "struct bevy::render::batching::gpu_preprocessing::BatchingPlugin")

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/blit/mod.rs.html#20)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [BlitPlugin](../core_pipeline/blit/struct.BlitPlugin.html "struct bevy::core_pipeline::blit::BlitPlugin")

[Source](https://docs.rs/bevy_post_process/0.19.0/x86_64-unknown-linux-gnu/src/bevy_post_process/bloom/mod.rs.html#46)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [BloomPlugin](../post_process/bloom/struct.BloomPlugin.html "struct bevy::post_process::bloom::BloomPlugin")

[Source](https://docs.rs/bevy_ui_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_render/box_shadow.rs.html#43)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [BoxShadowPlugin](../ui_render/box_shadow/struct.BoxShadowPlugin.html "struct bevy::ui_render::box_shadow::BoxShadowPlugin")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/button.rs.html#329)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for bevy::feathers::controls::[ButtonPlugin](../feathers/controls/struct.ButtonPlugin.html "struct bevy::feathers::controls::ButtonPlugin")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/button.rs.html#142)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for bevy::ui\_widgets::[ButtonPlugin](../ui_widgets/struct.ButtonPlugin.html "struct bevy::ui_widgets::ButtonPlugin")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/lib.rs.html#20)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for bevy::camera::[CameraPlugin](../camera/struct.CameraPlugin.html "struct bevy::camera::CameraPlugin")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/camera.rs.html#63)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for bevy::render::camera::[CameraPlugin](../render/camera/struct.CameraPlugin.html "struct bevy::render::camera::CameraPlugin")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/projection.rs.html#19)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [CameraProjectionPlugin](../camera/struct.CameraProjectionPlugin.html "struct bevy::camera::CameraProjectionPlugin")

[Source](https://docs.rs/bevy_anti_alias/0.19.0/x86_64-unknown-linux-gnu/src/bevy_anti_alias/contrast_adaptive_sharpening/mod.rs.html#105)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [CasPlugin](../anti_alias/contrast_adaptive_sharpening/struct.CasPlugin.html "struct bevy::anti_alias::contrast_adaptive_sharpening::CasPlugin")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/checkbox.rs.html#458)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for bevy::feathers::controls::[CheckboxPlugin](../feathers/controls/struct.CheckboxPlugin.html "struct bevy::feathers::controls::CheckboxPlugin")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/checkbox.rs.html#258)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for bevy::ui\_widgets::[CheckboxPlugin](../ui_widgets/struct.CheckboxPlugin.html "struct bevy::ui_widgets::CheckboxPlugin")

[Source](https://docs.rs/bevy_dev_tools/0.19.0/x86_64-unknown-linux-gnu/src/bevy_dev_tools/ci_testing/mod.rs.html#31)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [CiTestingPlugin](../dev_tools/ci_testing/struct.CiTestingPlugin.html "struct bevy::dev_tools::ci_testing::CiTestingPlugin")

[Source](https://docs.rs/bevy_clipboard/0.19.0/x86_64-unknown-linux-gnu/src/bevy_clipboard/lib.rs.html#57)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [ClipboardPlugin](../prelude/struct.ClipboardPlugin.html "struct bevy::prelude::ClipboardPlugin")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/decal/clustered.rs.html#160)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [ClusteredDecalPlugin](../pbr/struct.ClusteredDecalPlugin.html "struct bevy::pbr::ClusteredDecalPlugin")

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/mesh2d/color_material.rs.html#14)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [ColorMaterialPlugin](../sprite_render/struct.ColorMaterialPlugin.html "struct bevy::sprite_render::ColorMaterialPlugin")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/color_plane.rs.html#461)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [ColorPlanePlugin](../feathers/controls/struct.ColorPlanePlugin.html "struct bevy::feathers::controls::ColorPlanePlugin")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/color_slider.rs.html#495)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [ColorSliderPlugin](../feathers/controls/struct.ColorSliderPlugin.html "struct bevy::feathers::controls::ColorSliderPlugin")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/color_swatch.rs.html#122)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [ColorSwatchPlugin](../feathers/controls/struct.ColorSwatchPlugin.html "struct bevy::feathers::controls::ColorSwatchPlugin")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/contact_shadows.rs.html#100)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [ContactShadowsPlugin](../prelude/struct.ContactShadowsPlugin.html "struct bevy::prelude::ContactShadowsPlugin")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/mod.rs.html#42)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [ControlsPlugin](../feathers/controls/struct.ControlsPlugin.html "struct bevy::feathers::controls::ControlsPlugin")

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/deferred/copy_lighting_id.rs.html#25)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [CopyDeferredLightingIdPlugin](../core_pipeline/deferred/copy_lighting_id/struct.CopyDeferredLightingIdPlugin.html "struct bevy::core_pipeline::deferred::copy_lighting_id::CopyDeferredLightingIdPlugin")

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/core_2d/mod.rs.html#51)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [Core2dPlugin](../core_pipeline/core_2d/struct.Core2dPlugin.html "struct bevy::core_pipeline::core_2d::Core2dPlugin")

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/core_3d/mod.rs.html#102)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [Core3dPlugin](../core_pipeline/core_3d/struct.Core3dPlugin.html "struct bevy::core_pipeline::core_3d::Core3dPlugin")

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/lib.rs.html#47)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [CorePipelinePlugin](../core_pipeline/struct.CorePipelinePlugin.html "struct bevy::core_pipeline::CorePipelinePlugin")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/cursor.rs.html#124)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [CursorIconPlugin](../feathers/cursor/struct.CursorIconPlugin.html "struct bevy::feathers::cursor::CursorIconPlugin")

[Source](https://docs.rs/bevy_dev_tools/0.19.0/x86_64-unknown-linux-gnu/src/bevy_dev_tools/picking_debug.rs.html#83)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [DebugPickingPlugin](../dev_tools/picking_debug/struct.DebugPickingPlugin.html "struct bevy::dev_tools::picking_debug::DebugPickingPlugin")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/deferred/mod.rs.html#92)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [DeferredPbrLightingPlugin](../pbr/deferred/struct.DeferredPbrLightingPlugin.html "struct bevy::pbr::deferred::DeferredPbrLightingPlugin")

[Source](https://docs.rs/bevy_post_process/0.19.0/x86_64-unknown-linux-gnu/src/bevy_post_process/dof/mod.rs.html#195)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [DepthOfFieldPlugin](../post_process/dof/struct.DepthOfFieldPlugin.html "struct bevy::post_process::dof::DepthOfFieldPlugin")

[Source](https://docs.rs/bevy_dev_tools/0.19.0/x86_64-unknown-linux-gnu/src/bevy_dev_tools/diagnostics_overlay.rs.html#250)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [DiagnosticsOverlayPlugin](../dev_tools/diagnostics_overlay/struct.DiagnosticsOverlayPlugin.html "struct bevy::dev_tools::diagnostics_overlay::DiagnosticsOverlayPlugin")

[Source](https://docs.rs/bevy_diagnostic/0.19.0/x86_64-unknown-linux-gnu/src/bevy_diagnostic/lib.rs.html#41)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [DiagnosticsPlugin](../diagnostic/struct.DiagnosticsPlugin.html "struct bevy::diagnostic::DiagnosticsPlugin")

[Source](https://docs.rs/bevy_input_focus/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input_focus/directional_navigation.rs.html#76)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [DirectionalNavigationPlugin](../input_focus/directional_navigation/struct.DirectionalNavigationPlugin.html "struct bevy::input_focus::directional_navigation::DirectionalNavigationPlugin")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/disclosure_toggle.rs.html#159)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [DisclosureTogglePlugin](../feathers/controls/struct.DisclosureTogglePlugin.html "struct bevy::feathers::controls::DisclosureTogglePlugin")

[Source](https://docs.rs/bevy_dev_tools/0.19.0/x86_64-unknown-linux-gnu/src/bevy_dev_tools/easy_screenshot.rs.html#369)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [EasyCameraMovementPlugin](../dev_tools/struct.EasyCameraMovementPlugin.html "struct bevy::dev_tools::EasyCameraMovementPlugin")

[Source](https://docs.rs/bevy_dev_tools/0.19.0/x86_64-unknown-linux-gnu/src/bevy_dev_tools/easy_screenshot.rs.html#49)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [EasyScreenshotPlugin](../dev_tools/struct.EasyScreenshotPlugin.html "struct bevy::dev_tools::EasyScreenshotPlugin")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/text_input.rs.html#488)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [EditableTextInputPlugin](../ui_widgets/struct.EditableTextInputPlugin.html "struct bevy::ui_widgets::EditableTextInputPlugin")

[Source](https://docs.rs/bevy_post_process/0.19.0/x86_64-unknown-linux-gnu/src/bevy_post_process/effect_stack/mod.rs.html#125)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [EffectStackPlugin](../post_process/effect_stack/struct.EffectStackPlugin.html "struct bevy::post_process::effect_stack::EffectStackPlugin")

[Source](https://docs.rs/bevy_diagnostic/0.19.0/x86_64-unknown-linux-gnu/src/bevy_diagnostic/entity_count_diagnostics_plugin.rs.html#31)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [EntityCountDiagnosticsPlugin](../diagnostic/struct.EntityCountDiagnosticsPlugin.html "struct bevy::diagnostic::EntityCountDiagnosticsPlugin")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/light_probe/generate.rs.html#103)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [EnvironmentMapGenerationPlugin](../pbr/generate/struct.EnvironmentMapGenerationPlugin.html "struct bevy::pbr::generate::EnvironmentMapGenerationPlugin")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/extract_plugin.rs.html#31)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [ExtractPlugin](../render/extract_plugin/struct.ExtractPlugin.html "struct bevy::render::extract_plugin::ExtractPlugin")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/lib.rs.html#65)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [FeathersCorePlugin](../feathers/struct.FeathersCorePlugin.html "struct bevy::feathers::FeathersCorePlugin")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/focus.rs.html#93)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [FocusOutlinesPlugin](../feathers/focus/struct.FocusOutlinesPlugin.html "struct bevy::feathers::focus::FocusOutlinesPlugin")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/fog.rs.html#127)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [FogPlugin](../pbr/struct.FogPlugin.html "struct bevy::pbr::FogPlugin")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/decal/forward.rs.html#28)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [ForwardDecalPlugin](../pbr/decal/struct.ForwardDecalPlugin.html "struct bevy::pbr::decal::ForwardDecalPlugin")

[Source](https://docs.rs/bevy_dev_tools/0.19.0/x86_64-unknown-linux-gnu/src/bevy_dev_tools/fps_overlay.rs.html#68)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [FpsOverlayPlugin](../dev_tools/fps_overlay/struct.FpsOverlayPlugin.html "struct bevy::dev_tools::fps_overlay::FpsOverlayPlugin")

[Source](https://docs.rs/bevy_diagnostic/0.19.0/x86_64-unknown-linux-gnu/src/bevy_diagnostic/frame_count.rs.html#27)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [FrameCountPlugin](../diagnostic/struct.FrameCountPlugin.html "struct bevy::diagnostic::FrameCountPlugin")

[Source](https://docs.rs/bevy_diagnostic/0.19.0/x86_64-unknown-linux-gnu/src/bevy_diagnostic/frame_time_diagnostics_plugin.rs.html#38)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [FrameTimeDiagnosticsPlugin](../diagnostic/struct.FrameTimeDiagnosticsPlugin.html "struct bevy::diagnostic::FrameTimeDiagnosticsPlugin")

[Source](https://docs.rs/bevy_dev_tools/0.19.0/x86_64-unknown-linux-gnu/src/bevy_dev_tools/frame_time_graph/mod.rs.html#27)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [FrameTimeGraphPlugin](../dev_tools/frame_time_graph/struct.FrameTimeGraphPlugin.html "struct bevy::dev_tools::frame_time_graph::FrameTimeGraphPlugin")

[Source](https://docs.rs/bevy_camera_controller/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera_controller/free_camera.rs.html#43)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [FreeCameraPlugin](../camera_controller/free_camera/struct.FreeCameraPlugin.html "struct bevy::camera_controller::free_camera::FreeCameraPlugin")

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/frustum.rs.html#60)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [FrustumGizmoPlugin](../gizmos/frustum/struct.FrustumGizmoPlugin.html "struct bevy::gizmos::frustum::FrustumGizmoPlugin")

[Source](https://docs.rs/bevy_anti_alias/0.19.0/x86_64-unknown-linux-gnu/src/bevy_anti_alias/fxaa/mod.rs.html#86)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [FxaaPlugin](../anti_alias/fxaa/struct.FxaaPlugin.html "struct bevy::anti_alias::fxaa::FxaaPlugin")

[Source](https://docs.rs/bevy_gilrs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gilrs/lib.rs.html#90)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [GilrsPlugin](../prelude/struct.GilrsPlugin.html "struct bevy::prelude::GilrsPlugin")

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/lib.rs.html#107)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [GizmoPlugin](../gizmos/struct.GizmoPlugin.html "struct bevy::gizmos::GizmoPlugin")

[Source](https://docs.rs/bevy_gizmos_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos_render/lib.rs.html#87)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [GizmoRenderPlugin](../gizmos_render/struct.GizmoRenderPlugin.html "struct bevy::gizmos_render::GizmoRenderPlugin")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/globals.rs.html#16)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [GlobalsPlugin](../render/globals/struct.GlobalsPlugin.html "struct bevy::render::globals::GlobalsPlugin")

[Source](https://docs.rs/bevy_gltf/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gltf/lib.rs.html#269)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [GltfPlugin](../gltf/struct.GltfPlugin.html "struct bevy::gltf::GltfPlugin")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/gpu_preprocess.rs.html#394)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [GpuMeshPreprocessPlugin](../pbr/struct.GpuMeshPreprocessPlugin.html "struct bevy::pbr::GpuMeshPreprocessPlugin")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/gpu_readback.rs.html#50)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [GpuReadbackPlugin](../render/gpu_readback/struct.GpuReadbackPlugin.html "struct bevy::render::gpu_readback::GpuReadbackPlugin")

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/hotpatch.rs.html#24)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [HotPatchPlugin](hotpatch/struct.HotPatchPlugin.html "struct bevy::app::hotpatch::HotPatchPlugin")

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/image.rs.html#207)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [ImagePlugin](../prelude/struct.ImagePlugin.html "struct bevy::prelude::ImagePlugin")

[Source](https://docs.rs/bevy_dev_tools/0.19.0/x86_64-unknown-linux-gnu/src/bevy_dev_tools/infinite_grid.rs.html#52)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [InfiniteGridPlugin](../dev_tools/infinite_grid/struct.InfiniteGridPlugin.html "struct bevy::dev_tools::infinite_grid::InfiniteGridPlugin")

[Source](https://docs.rs/bevy_input_focus/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input_focus/lib.rs.html#289)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [InputDispatchPlugin](../input_focus/struct.InputDispatchPlugin.html "struct bevy::input_focus::InputDispatchPlugin")

[Source](https://docs.rs/bevy_input_focus/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input_focus/lib.rs.html#270)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [InputFocusPlugin](../input_focus/struct.InputFocusPlugin.html "struct bevy::input_focus::InputFocusPlugin")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/lib.rs.html#108)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [InputPlugin](../input/struct.InputPlugin.html "struct bevy::input::InputPlugin")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/lib.rs.html#419)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [InteractionPlugin](../prelude/struct.InteractionPlugin.html "struct bevy::prelude::InteractionPlugin")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/gizmos.rs.html#135)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [LightGizmoPlugin](../light/gizmos/struct.LightGizmoPlugin.html "struct bevy::light::gizmos::LightGizmoPlugin")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/lib.rs.html#165)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [LightPlugin](../light/struct.LightPlugin.html "struct bevy::light::LightPlugin")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/light_probe/mod.rs.html#373)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [LightProbePlugin](../pbr/struct.LightProbePlugin.html "struct bevy::pbr::LightProbePlugin")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/lightmap/mod.rs.html#186)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [LightmapPlugin](../pbr/struct.LightmapPlugin.html "struct bevy::pbr::LightmapPlugin")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/list.rs.html#305)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [ListBoxPlugin](../ui_widgets/struct.ListBoxPlugin.html "struct bevy::ui_widgets::ListBoxPlugin")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/listview.rs.html#325)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [ListViewPlugin](../feathers/controls/struct.ListViewPlugin.html "struct bevy::feathers::controls::ListViewPlugin")

[Source](https://docs.rs/bevy_diagnostic/0.19.0/x86_64-unknown-linux-gnu/src/bevy_diagnostic/log_diagnostics_plugin.rs.html#100)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [LogDiagnosticsPlugin](../diagnostic/struct.LogDiagnosticsPlugin.html "struct bevy::diagnostic::LogDiagnosticsPlugin")

[Source](https://docs.rs/bevy_log/0.19.0/x86_64-unknown-linux-gnu/src/bevy_log/lib.rs.html#295)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [LogPlugin](../log/struct.LogPlugin.html "struct bevy::log::LogPlugin")

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/main_schedule.rs.html#311)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [MainSchedulePlugin](struct.MainSchedulePlugin.html "struct bevy::app::MainSchedulePlugin")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/material.rs.html#289)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [MaterialsPlugin](../pbr/struct.MaterialsPlugin.html "struct bevy::pbr::MaterialsPlugin")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/menu.rs.html#460)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for bevy::feathers::controls::[MenuPlugin](../feathers/controls/struct.MenuPlugin.html "struct bevy::feathers::controls::MenuPlugin")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/menu.rs.html#472)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for bevy::ui\_widgets::[MenuPlugin](../ui_widgets/struct.MenuPlugin.html "struct bevy::ui_widgets::MenuPlugin")

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/mesh2d/mesh.rs.html#64)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [Mesh2dRenderPlugin](../sprite_render/struct.Mesh2dRenderPlugin.html "struct bevy::sprite_render::Mesh2dRenderPlugin")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/diagnostic/mesh_allocator_diagnostic_plugin.rs.html#36)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [MeshAllocatorDiagnosticPlugin](../render/diagnostic/struct.MeshAllocatorDiagnosticPlugin.html "struct bevy::render::diagnostic::MeshAllocatorDiagnosticPlugin")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/mesh/allocator.rs.html#191)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [MeshAllocatorPlugin](../render/mesh/allocator/struct.MeshAllocatorPlugin.html "struct bevy::render::mesh::allocator::MeshAllocatorPlugin")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/mesh_picking/mod.rs.html#69)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [MeshPickingPlugin](../prelude/struct.MeshPickingPlugin.html "struct bevy::prelude::MeshPickingPlugin")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/lib.rs.html#62)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [MeshPlugin](../mesh/struct.MeshPlugin.html "struct bevy::mesh::MeshPlugin")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/mesh/mod.rs.html#35)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [MeshRenderAssetPlugin](../render/mesh/struct.MeshRenderAssetPlugin.html "struct bevy::render::mesh::MeshRenderAssetPlugin")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#147)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [MeshRenderPlugin](../pbr/struct.MeshRenderPlugin.html "struct bevy::pbr::MeshRenderPlugin")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/meshlet/mod.rs.html#130)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [MeshletPlugin](../pbr/experimental/meshlet/struct.MeshletPlugin.html "struct bevy::pbr::experimental::meshlet::MeshletPlugin")

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/mip_generation/mod.rs.html#254)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [MipGenerationPlugin](../core_pipeline/mip_generation/struct.MipGenerationPlugin.html "struct bevy::core_pipeline::mip_generation::MipGenerationPlugin")

[Source](https://docs.rs/bevy_post_process/0.19.0/x86_64-unknown-linux-gnu/src/bevy_post_process/motion_blur/mod.rs.html#150)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [MotionBlurPlugin](../post_process/motion_blur/struct.MotionBlurPlugin.html "struct bevy::post_process::motion_blur::MotionBlurPlugin")

[Source](https://docs.rs/bevy_post_process/0.19.0/x86_64-unknown-linux-gnu/src/bevy_post_process/msaa_writeback.rs.html#23)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [MsaaWritebackPlugin](../post_process/msaa_writeback/struct.MsaaWritebackPlugin.html "struct bevy::post_process::msaa_writeback::MsaaWritebackPlugin")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/occlusion_culling/mod.rs.html#19)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [OcclusionCullingPlugin](../render/occlusion_culling/struct.OcclusionCullingPlugin.html "struct bevy::render::occlusion_culling::OcclusionCullingPlugin")

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/oit/resolve/mod.rs.html#39)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [OitResolvePlugin](../core_pipeline/oit/resolve/struct.OitResolvePlugin.html "struct bevy::core_pipeline::oit::resolve::OitResolvePlugin")

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/oit/mod.rs.html#84)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [OrderIndependentTransparencyPlugin](../core_pipeline/oit/struct.OrderIndependentTransparencyPlugin.html "struct bevy::core_pipeline::oit::OrderIndependentTransparencyPlugin")

[Source](https://docs.rs/bevy_camera_controller/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera_controller/pan_camera.rs.html#28)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [PanCameraPlugin](../camera_controller/pan_camera/struct.PanCameraPlugin.html "struct bevy::camera_controller::pan_camera::PanCameraPlugin")

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/panic_handler.rs.html#40)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [PanicHandlerPlugin](struct.PanicHandlerPlugin.html "struct bevy::app::PanicHandlerPlugin")

[Source](https://docs.rs/bevy_solari/0.19.0/x86_64-unknown-linux-gnu/src/bevy_solari/pathtracer/mod.rs.html#29)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [PathtracingPlugin](../solari/pathtracer/struct.PathtracingPlugin.html "struct bevy::solari::pathtracer::PathtracingPlugin")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/lib.rs.html#189)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [PbrPlugin](../pbr/struct.PbrPlugin.html "struct bevy::pbr::PbrPlugin")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/lib.rs.html#368)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [PickingPlugin](../prelude/struct.PickingPlugin.html "struct bevy::prelude::PickingPlugin")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/pipelined_rendering.rs.html#111)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [PipelinedRenderingPlugin](../render/pipelined_rendering/struct.PipelinedRenderingPlugin.html "struct bevy::render::pipelined_rendering::PipelinedRenderingPlugin")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/input.rs.html#95)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [PointerInputPlugin](../prelude/struct.PointerInputPlugin.html "struct bevy::prelude::PointerInputPlugin")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/popover.rs.html#311)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [PopoverPlugin](../ui_widgets/popover/struct.PopoverPlugin.html "struct bevy::ui_widgets::popover::PopoverPlugin")

[Source](https://docs.rs/bevy_post_process/0.19.0/x86_64-unknown-linux-gnu/src/bevy_post_process/lib.rs.html#27)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [PostProcessPlugin](../post_process/struct.PostProcessPlugin.html "struct bevy::post_process::PostProcessPlugin")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/prepass/mod.rs.html#76)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [PrepassPipelinePlugin](../pbr/struct.PrepassPipelinePlugin.html "struct bevy::pbr::PrepassPipelinePlugin")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/prepass/mod.rs.html#120)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [PrepassPlugin](../pbr/struct.PrepassPlugin.html "struct bevy::pbr::PrepassPlugin")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/radio.rs.html#328)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [RadioGroupPlugin](../ui_widgets/struct.RadioGroupPlugin.html "struct bevy::ui_widgets::RadioGroupPlugin")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/radio.rs.html#413)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [RadioPlugin](../feathers/controls/struct.RadioPlugin.html "struct bevy::feathers::controls::RadioPlugin")

[Source](https://docs.rs/bevy_solari/0.19.0/x86_64-unknown-linux-gnu/src/bevy_solari/scene/mod.rs.html#32)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [RaytracingScenePlugin](../solari/scene/struct.RaytracingScenePlugin.html "struct bevy::solari::scene::RaytracingScenePlugin")

[Source](https://docs.rs/bevy_remote/0.19.0/x86_64-unknown-linux-gnu/src/bevy_remote/http.rs.html#135)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [RemoteHttpPlugin](../remote/http/struct.RemoteHttpPlugin.html "struct bevy::remote::http::RemoteHttpPlugin")

[Source](https://docs.rs/bevy_remote/0.19.0/x86_64-unknown-linux-gnu/src/bevy_remote/lib.rs.html#805)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [RemotePlugin](../remote/struct.RemotePlugin.html "struct bevy::remote::RemotePlugin")

[Source](https://docs.rs/bevy_dev_tools/0.19.0/x86_64-unknown-linux-gnu/src/bevy_dev_tools/render_debug.rs.html#52)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [RenderDebugOverlayPlugin](../dev_tools/render_debug/struct.RenderDebugOverlayPlugin.html "struct bevy::dev_tools::render_debug::RenderDebugOverlayPlugin")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/diagnostic/mod.rs.html#66)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [RenderDiagnosticsPlugin](../render/diagnostic/struct.RenderDiagnosticsPlugin.html "struct bevy::render::diagnostic::RenderDiagnosticsPlugin")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/lib.rs.html#348)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [RenderPlugin](../render/struct.RenderPlugin.html "struct bevy::render::RenderPlugin")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/view/visibility/range.rs.html#45)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [RenderVisibilityRangePlugin](../render/view/struct.RenderVisibilityRangePlugin.html "struct bevy::render::view::RenderVisibilityRangePlugin")

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/lib.rs.html#1070)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [ScenePlugin](../scene/struct.ScenePlugin.html "struct bevy::scene::ScenePlugin")

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/schedule_runner.rs.html#73)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [ScheduleRunnerPlugin](struct.ScheduleRunnerPlugin.html "struct bevy::app::ScheduleRunnerPlugin")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/ssao/mod.rs.html#45)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [ScreenSpaceAmbientOcclusionPlugin](../prelude/struct.ScreenSpaceAmbientOcclusionPlugin.html "struct bevy::prelude::ScreenSpaceAmbientOcclusionPlugin")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/ssr/mod.rs.html#193)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [ScreenSpaceReflectionsPlugin](../pbr/struct.ScreenSpaceReflectionsPlugin.html "struct bevy::pbr::ScreenSpaceReflectionsPlugin")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/transmission/mod.rs.html#30)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [ScreenSpaceTransmissionPlugin](../pbr/struct.ScreenSpaceTransmissionPlugin.html "struct bevy::pbr::ScreenSpaceTransmissionPlugin")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/view/window/screenshot.rs.html#408)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [ScreenshotPlugin](../render/view/window/screenshot/struct.ScreenshotPlugin.html "struct bevy::render::view::window::screenshot::ScreenshotPlugin")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/scrollarea.rs.html#126)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [ScrollAreaPlugin](../ui_widgets/struct.ScrollAreaPlugin.html "struct bevy::ui_widgets::ScrollAreaPlugin")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/scrollbar.rs.html#94)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for bevy::feathers::controls::[ScrollbarPlugin](../feathers/controls/struct.ScrollbarPlugin.html "struct bevy::feathers::controls::ScrollbarPlugin")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/scrollbar.rs.html#463)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for bevy::ui\_widgets::[ScrollbarPlugin](../ui_widgets/struct.ScrollbarPlugin.html "struct bevy::ui_widgets::ScrollbarPlugin")

[Source](https://docs.rs/bevy_dev_tools/0.19.0/x86_64-unknown-linux-gnu/src/bevy_dev_tools/schedule_data/plugin.rs.html#58)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [SerializeSchedulesPlugin](../dev_tools/schedule_data/plugin/struct.SerializeSchedulesPlugin.html "struct bevy::dev_tools::schedule_data::plugin::SerializeSchedulesPlugin")

[Source](https://docs.rs/bevy-settings/0.19.0/x86_64-unknown-linux-gnu/src/bevy_settings/lib.rs.html#96)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [SettingsPlugin](../settings/struct.SettingsPlugin.html "struct bevy::settings::SettingsPlugin")

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/skinned_mesh_bounds.rs.html#32)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [SkinnedMeshBoundsGizmoPlugin](../gizmos/skinned_mesh_bounds/struct.SkinnedMeshBoundsGizmoPlugin.html "struct bevy::gizmos::skinned_mesh_bounds::SkinnedMeshBoundsGizmoPlugin")

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/skybox/mod.rs.html#36)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [SkyboxPlugin](../core_pipeline/skybox/struct.SkyboxPlugin.html "struct bevy::core_pipeline::skybox::SkyboxPlugin")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/slider.rs.html#372)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for bevy::feathers::controls::[SliderPlugin](../feathers/controls/struct.SliderPlugin.html "struct bevy::feathers::controls::SliderPlugin")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/slider.rs.html#726)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for bevy::ui\_widgets::[SliderPlugin](../ui_widgets/struct.SliderPlugin.html "struct bevy::ui_widgets::SliderPlugin")

[Source](https://docs.rs/bevy_anti_alias/0.19.0/x86_64-unknown-linux-gnu/src/bevy_anti_alias/smaa/mod.rs.html#290)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [SmaaPlugin](../anti_alias/smaa/struct.SmaaPlugin.html "struct bevy::anti_alias::smaa::SmaaPlugin")

[Source](https://docs.rs/bevy_solari/0.19.0/x86_64-unknown-linux-gnu/src/bevy_solari/realtime/mod.rs.html#35)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [SolariLightingPlugin](../solari/realtime/struct.SolariLightingPlugin.html "struct bevy::solari::realtime::SolariLightingPlugin")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_resource/sparse_buffer_vec.rs.html#47)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [SparseBufferPlugin](../render/render_resource/struct.SparseBufferPlugin.html "struct bevy::render::render_resource::SparseBufferPlugin")

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/sprite_mesh/sprite_material.rs.html#25)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [SpriteMaterialPlugin](../sprite_render/struct.SpriteMaterialPlugin.html "struct bevy::sprite_render::SpriteMaterialPlugin")

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/sprite_mesh/mod.rs.html#25)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [SpriteMeshPlugin](../sprite_render/struct.SpriteMeshPlugin.html "struct bevy::sprite_render::SpriteMeshPlugin")

[Source](https://docs.rs/bevy_sprite/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite/picking_backend.rs.html#80)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [SpritePickingPlugin](../prelude/struct.SpritePickingPlugin.html "struct bevy::prelude::SpritePickingPlugin")

[Source](https://docs.rs/bevy_sprite/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite/lib.rs.html#77)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [SpritePlugin](../sprite/struct.SpritePlugin.html "struct bevy::sprite::SpritePlugin")

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/lib.rs.html#63)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [SpriteRenderPlugin](../sprite_render/struct.SpriteRenderPlugin.html "struct bevy::sprite_render::SpriteRenderPlugin")

[Source](https://docs.rs/bevy_state/0.19.0/x86_64-unknown-linux-gnu/src/bevy_state/app.rs.html#332)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [StatesPlugin](../state/app/struct.StatesPlugin.html "struct bevy::state::app::StatesPlugin")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/storage.rs.html#18)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [StoragePlugin](../render/storage/struct.StoragePlugin.html "struct bevy::render::storage::StoragePlugin")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/sync_world.rs.html#92)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [SyncWorldPlugin](../render/sync_world/struct.SyncWorldPlugin.html "struct bevy::render::sync_world::SyncWorldPlugin")

[Source](https://docs.rs/bevy_diagnostic/0.19.0/x86_64-unknown-linux-gnu/src/bevy_diagnostic/system_information_diagnostics_plugin.rs.html#24)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [SystemInformationDiagnosticsPlugin](../diagnostic/struct.SystemInformationDiagnosticsPlugin.html "struct bevy::diagnostic::SystemInformationDiagnosticsPlugin")

[Source](https://docs.rs/bevy_input_focus/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input_focus/tab_navigation.rs.html#382)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [TabNavigationPlugin](../input_focus/tab_navigation/struct.TabNavigationPlugin.html "struct bevy::input_focus::tab_navigation::TabNavigationPlugin")

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/task_pool_plugin.rs.html#32)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [TaskPoolPlugin](../prelude/struct.TaskPoolPlugin.html "struct bevy::prelude::TaskPoolPlugin")

[Source](https://docs.rs/bevy_anti_alias/0.19.0/x86_64-unknown-linux-gnu/src/bevy_anti_alias/taa/mod.rs.html#49)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [TemporalAntiAliasPlugin](../anti_alias/taa/struct.TemporalAntiAliasPlugin.html "struct bevy::anti_alias::taa::TemporalAntiAliasPlugin")

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/terminal_ctrl_c_handler.rs.html#66)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [TerminalCtrlCHandlerPlugin](struct.TerminalCtrlCHandlerPlugin.html "struct bevy::app::TerminalCtrlCHandlerPlugin")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/text_input.rs.html#208)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [TextInputPlugin](../feathers/controls/struct.TextInputPlugin.html "struct bevy::feathers::controls::TextInputPlugin")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/lib.rs.html#111)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [TextPlugin](../text/struct.TextPlugin.html "struct bevy::text::TextPlugin")

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/texture_atlas.rs.html#18)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [TextureAtlasPlugin](../image/struct.TextureAtlasPlugin.html "struct bevy::image::TextureAtlasPlugin")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/texture/mod.rs.html#28)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [TexturePlugin](../render/texture/struct.TexturePlugin.html "struct bevy::render::texture::TexturePlugin")

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/tilemap_chunk/tilemap_chunk_material.rs.html#15)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [TilemapChunkMaterialPlugin](../sprite_render/struct.TilemapChunkMaterialPlugin.html "struct bevy::sprite_render::TilemapChunkMaterialPlugin")

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/tilemap_chunk/mod.rs.html#38)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [TilemapChunkPlugin](../sprite_render/struct.TilemapChunkPlugin.html "struct bevy::sprite_render::TilemapChunkPlugin")

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/lib.rs.html#66)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [TimePlugin](../time/struct.TimePlugin.html "struct bevy::time::TimePlugin")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/toggle_switch.rs.html#427)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [ToggleSwitchPlugin](../feathers/controls/struct.ToggleSwitchPlugin.html "struct bevy::feathers::controls::ToggleSwitchPlugin")

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/tonemapping/mod.rs.html#44)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [TonemappingPlugin](../core_pipeline/tonemapping/struct.TonemappingPlugin.html "struct bevy::core_pipeline::tonemapping::TonemappingPlugin")

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/transform_gizmo.rs.html#228)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [TransformGizmoPlugin](../prelude/struct.TransformGizmoPlugin.html "struct bevy::prelude::TransformGizmoPlugin")

[Source](https://docs.rs/bevy_gizmos_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos_render/transform_gizmo_render.rs.html#82)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [TransformGizmoRenderPlugin](../gizmos_render/transform_gizmo_render/struct.TransformGizmoRenderPlugin.html "struct bevy::gizmos_render::transform_gizmo_render::TransformGizmoRenderPlugin")

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/plugins.rs.html#22)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [TransformPlugin](../prelude/struct.TransformPlugin.html "struct bevy::prelude::TransformPlugin")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/picking_backend.rs.html#77)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [UiPickingPlugin](../prelude/struct.UiPickingPlugin.html "struct bevy::prelude::UiPickingPlugin")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/lib.rs.html#142)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [UiPlugin](../ui/struct.UiPlugin.html "struct bevy::ui::UiPlugin")

[Source](https://docs.rs/bevy_ui_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_render/lib.rs.html#199)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [UiRenderPlugin](../ui_render/struct.UiRenderPlugin.html "struct bevy::ui_render::UiRenderPlugin")

[Source](https://docs.rs/bevy_ui_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_render/ui_texture_slice_pipeline.rs.html#38)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [UiTextureSlicerPlugin](../ui_render/ui_texture_slice_pipeline/struct.UiTextureSlicerPlugin.html "struct bevy::ui_render::ui_texture_slice_pipeline::UiTextureSlicerPlugin")

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/upscaling/mod.rs.html#16)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [UpscalingPlugin](../core_pipeline/upscaling/struct.UpscalingPlugin.html "struct bevy::core_pipeline::upscaling::UpscalingPlugin")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/view/mod.rs.html#169)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [ViewPlugin](../render/view/struct.ViewPlugin.html "struct bevy::render::view::ViewPlugin")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/visibility/mod.rs.html#495)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [VisibilityPlugin](../camera/visibility/struct.VisibilityPlugin.html "struct bevy::camera::visibility::VisibilityPlugin")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/visibility/range.rs.html#31)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [VisibilityRangePlugin](../camera/visibility/struct.VisibilityRangePlugin.html "struct bevy::camera::visibility::VisibilityRangePlugin")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/volumetric_fog/mod.rs.html#65)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [VolumetricFogPlugin](../pbr/struct.VolumetricFogPlugin.html "struct bevy::pbr::VolumetricFogPlugin")

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/io/web.rs.html#62)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [WebAssetPlugin](../asset/io/web/struct.WebAssetPlugin.html "struct bevy::asset::io::web::WebAssetPlugin")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/lib.rs.html#104)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [WindowPlugin](../prelude/struct.WindowPlugin.html "struct bevy::prelude::WindowPlugin")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/view/window/mod.rs.html#30)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [WindowRenderPlugin](../render/view/struct.WindowRenderPlugin.html "struct bevy::render::view::WindowRenderPlugin")

[Source](https://docs.rs/bevy_winit/0.19.0/x86_64-unknown-linux-gnu/src/bevy_winit/lib.rs.html#85)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [WinitPlugin](../winit/struct.WinitPlugin.html "struct bevy::winit::WinitPlugin")

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/mesh2d/wireframe2d.rs.html#80)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [Wireframe2dPlugin](../sprite_render/struct.Wireframe2dPlugin.html "struct bevy::sprite_render::Wireframe2dPlugin")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/wireframe.rs.html#87)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [WireframePlugin](../pbr/wireframe/struct.WireframePlugin.html "struct bevy::pbr::wireframe::WireframePlugin")

[Source](https://docs.rs/bevy_world_serialization/0.19.0/x86_64-unknown-linux-gnu/src/bevy_world_serialization/lib.rs.html#58)

### impl [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [WorldSerializationPlugin](../world_serialization/struct.WorldSerializationPlugin.html "struct bevy::world_serialization::WorldSerializationPlugin")

Available on **crate feature `serialize`** only.

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/erased_render_asset.rs.html#116-117)

### impl<A, AFTER> [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [ErasedRenderAssetPlugin](../render/erased_render_asset/struct.ErasedRenderAssetPlugin.html "struct bevy::render::erased_render_asset::ErasedRenderAssetPlugin")<A, AFTER>

where A: [ErasedRenderAsset](../render/erased_render_asset/trait.ErasedRenderAsset.html "trait bevy::render::erased_render_asset::ErasedRenderAsset"), AFTER: [ErasedRenderAssetDependency](../render/erased_render_asset/trait.ErasedRenderAssetDependency.html "trait bevy::render::erased_render_asset::ErasedRenderAssetDependency") + 'static,

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_asset.rs.html#132-133)

### impl<A, AFTER> [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [RenderAssetPlugin](../render/render_asset/struct.RenderAssetPlugin.html "struct bevy::render::render_asset::RenderAssetPlugin")<A, AFTER>

where A: [RenderAsset](../render/render_asset/trait.RenderAsset.html "trait bevy::render::render_asset::RenderAsset"), AFTER: [RenderAssetDependency](../render/render_asset/trait.RenderAssetDependency.html "trait bevy::render::render_asset::RenderAssetDependency") + 'static,

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/diagnostic/erased_render_asset_diagnostic_plugin.rs.html#35)

### impl<A> [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [ErasedRenderAssetDiagnosticPlugin](../render/diagnostic/struct.ErasedRenderAssetDiagnosticPlugin.html "struct bevy::render::diagnostic::ErasedRenderAssetDiagnosticPlugin")<A>

where A: [ErasedRenderAsset](../render/erased_render_asset/trait.ErasedRenderAsset.html "trait bevy::render::erased_render_asset::ErasedRenderAsset"),

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/diagnostic/render_asset_diagnostic_plugin.rs.html#31)

### impl<A> [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [RenderAssetDiagnosticPlugin](../render/diagnostic/struct.RenderAssetDiagnosticPlugin.html "struct bevy::render::diagnostic::RenderAssetDiagnosticPlugin")<A>

where A: [RenderAsset](../render/render_asset/trait.RenderAsset.html "trait bevy::render::render_asset::RenderAsset"),

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_phase/mod.rs.html#1511-1514)

### impl<BPI, GFBD> [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [BinnedRenderPhasePlugin](../render/render_phase/struct.BinnedRenderPhasePlugin.html "struct bevy::render::render_phase::BinnedRenderPhasePlugin")<BPI, GFBD>

where BPI: [BinnedPhaseItem](../render/render_phase/trait.BinnedPhaseItem.html "trait bevy::render::render_phase::BinnedPhaseItem"), GFBD: [GetFullBatchData](../render/batching/trait.GetFullBatchData.html "trait bevy::render::batching::GetFullBatchData") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + 'static,

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/propagate.rs.html#137-138)

### impl<C, F, R> [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [HierarchyPropagatePlugin](struct.HierarchyPropagatePlugin.html "struct bevy::app::HierarchyPropagatePlugin")<C, F, R>

where C: [Component](../prelude/trait.Component.html "trait bevy::prelude::Component") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq"), F: [QueryFilter](../ecs/query/trait.QueryFilter.html "trait bevy::ecs::query::QueryFilter") + 'static, R: [Relationship](../ecs/relationship/trait.Relationship.html "trait bevy::ecs::relationship::Relationship"),

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/extract_component.rs.html#83)

### impl<C, F> [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [ExtractComponentPlugin](../render/extract_component/struct.ExtractComponentPlugin.html "struct bevy::render::extract_component::ExtractComponentPlugin")<C, F>

where C: [ExtractComponent](../render/extract_component/trait.ExtractComponent.html "trait bevy::render::extract_component::ExtractComponent")<F>, F: 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/sync_component.rs.html#48)

### impl<C, F> [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [SyncComponentPlugin](../render/sync_component/struct.SyncComponentPlugin.html "struct bevy::render::sync_component::SyncComponentPlugin")<C, F>

where C: [SyncComponent](../render/sync_component/trait.SyncComponent.html "trait bevy::render::sync_component::SyncComponent")<F>, F: [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/gpu_component_array_buffer.rs.html#18)

### impl<C> [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [GpuComponentArrayBufferPlugin](../render/gpu_component_array_buffer/struct.GpuComponentArrayBufferPlugin.html "struct bevy::render::gpu_component_array_buffer::GpuComponentArrayBufferPlugin")<C>

where C: [Component](../prelude/trait.Component.html "trait bevy::prelude::Component") + [GpuArrayBufferable](../render/render_resource/trait.GpuArrayBufferable.html "trait bevy::render::render_resource::GpuArrayBufferable"),

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/uniform.rs.html#41)

### impl<C> [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [UniformComponentPlugin](../render/extract_component/struct.UniformComponentPlugin.html "struct bevy::render::extract_component::UniformComponentPlugin")<C>

where C: [Component](../prelude/trait.Component.html "trait bevy::prelude::Component") + [ShaderType](../render/render_resource/trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") + [WriteInto](../render/render_resource/encase/internal/trait.WriteInto.html "trait bevy::render::render_resource::encase::internal::WriteInto") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/hierarchy.rs.html#45)

### impl<C> [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [ValidateParentHasComponentPlugin](struct.ValidateParentHasComponentPlugin.html "struct bevy::app::ValidateParentHasComponentPlugin")<C>

where C: [Component](../prelude/trait.Component.html "trait bevy::prelude::Component"),

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/extract_instances.rs.html#93-95)

### impl<EI> [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [ExtractInstancesPlugin](../render/extract_instances/struct.ExtractInstancesPlugin.html "struct bevy::render::extract_instances::ExtractInstancesPlugin")<EI>

where EI: [ExtractInstance](../render/extract_instances/trait.ExtractInstance.html "trait bevy::render::extract_instances::ExtractInstance"),

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/mesh2d/material.rs.html#276-278)

### impl<M> [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [Material2dPlugin](../sprite_render/struct.Material2dPlugin.html "struct bevy::sprite_render::Material2dPlugin")<M>

where M: [Material2d](../sprite_render/trait.Material2d.html "trait bevy::sprite_render::Material2d"), <M as [AsBindGroup](../render/render_resource/trait.AsBindGroup.html "trait bevy::render::render_resource::AsBindGroup")\>::[Data](../render/render_resource/trait.AsBindGroup.html#associatedtype.Data "type bevy::render::render_resource::AsBindGroup::Data"): [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq") + [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/diagnostic.rs.html#52)

### impl<M> [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [MaterialAllocatorDiagnosticPlugin](../pbr/diagnostic/struct.MaterialAllocatorDiagnosticPlugin.html "struct bevy::pbr::diagnostic::MaterialAllocatorDiagnosticPlugin")<M>

where M: [Material](../prelude/trait.Material.html "trait bevy::prelude::Material"),

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/material.rs.html#371-373)

### impl<M> [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [MaterialPlugin](../prelude/struct.MaterialPlugin.html "struct bevy::prelude::MaterialPlugin")<M>

where M: [Material](../prelude/trait.Material.html "trait bevy::prelude::Material"), <M as [AsBindGroup](../render/render_resource/trait.AsBindGroup.html "trait bevy::render::render_resource::AsBindGroup")\>::[Data](../render/render_resource/trait.AsBindGroup.html#associatedtype.Data "type bevy::render::render_resource::AsBindGroup::Data"): [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq") + [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

[Source](https://docs.rs/bevy_ui_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_render/ui_material_pipeline.rs.html#42-44)

### impl<M> [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [UiMaterialPlugin](../prelude/struct.UiMaterialPlugin.html "struct bevy::prelude::UiMaterialPlugin")<M>

where M: [UiMaterial](../prelude/trait.UiMaterial.html "trait bevy::prelude::UiMaterial"), <M as [AsBindGroup](../render/render_resource/trait.AsBindGroup.html "trait bevy::render::render_resource::AsBindGroup")\>::[Data](../render/render_resource/trait.AsBindGroup.html#associatedtype.Data "type bevy::render::render_resource::AsBindGroup::Data"): [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq") + [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/extract_resource.rs.html#41-42)

### impl<R, F> [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [ExtractResourcePlugin](../render/extract_resource/struct.ExtractResourcePlugin.html "struct bevy::render::extract_resource::ExtractResourcePlugin")<R, F>

where R: [ExtractResource](../render/extract_resource/trait.ExtractResource.html "trait bevy::render::extract_resource::ExtractResource")<F, Mutability = [Mutable](../ecs/component/struct.Mutable.html "struct bevy::ecs::component::Mutable")\>, F: 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_phase/mod.rs.html#1632-1635)

### impl<SPI, GFBD> [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [SortedRenderPhasePlugin](../render/render_phase/struct.SortedRenderPhasePlugin.html "struct bevy::render::render_phase::SortedRenderPhasePlugin")<SPI, GFBD>

where SPI: [SortedPhaseItem](../render/render_phase/trait.SortedPhaseItem.html "trait bevy::render::render_phase::SortedPhaseItem") + [CachedRenderPipelinePhaseItem](../render/render_phase/trait.CachedRenderPipelinePhaseItem.html "trait bevy::render::render_phase::CachedRenderPipelinePhaseItem"), GFBD: [GetFullBatchData](../render/batching/trait.GetFullBatchData.html "trait bevy::render::batching::GetFullBatchData") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + 'static,

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/fullscreen_material.rs.html#50)

### impl<T> [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for [FullscreenMaterialPlugin](../core_pipeline/fullscreen_material/struct.FullscreenMaterialPlugin.html "struct bevy::core_pipeline::fullscreen_material::FullscreenMaterialPlugin")<T>

where T: [FullscreenMaterial](../core_pipeline/fullscreen_material/trait.FullscreenMaterial.html "trait bevy::core_pipeline::fullscreen_material::FullscreenMaterial"),

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/plugin.rs.html#96)

### impl<T> [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") for T

where T: [Fn](https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html "trait core::ops::function::Fn")(&mut [App](../prelude/struct.App.html "struct bevy::prelude::App")) + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,