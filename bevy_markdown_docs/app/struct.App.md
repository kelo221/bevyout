[bevy](../index.html)::[app](index.html)

# Struct App 

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/app.rs.html#86)

```rust
pub struct App { /* private fields */ }
```

[`App`](../prelude/struct.App.html "struct bevy::prelude::App") is the primary API for writing user applications. It automates the setup of a [standard lifecycle](../prelude/struct.Main.html "struct bevy::prelude::Main") and provides interface glue for [plugins](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin").

A single [`App`](../prelude/struct.App.html "struct bevy::prelude::App") can contain multiple [`SubApp`](../prelude/struct.SubApp.html "struct bevy::prelude::SubApp") instances, but [`App`](../prelude/struct.App.html "struct bevy::prelude::App") methods only affect the “main” one. To access a particular [`SubApp`](../prelude/struct.SubApp.html "struct bevy::prelude::SubApp"), use [`get_sub_app`](../prelude/struct.App.html#method.get_sub_app "method bevy::prelude::App::get_sub_app") or [`get_sub_app_mut`](../prelude/struct.App.html#method.get_sub_app_mut "method bevy::prelude::App::get_sub_app_mut").

## Examples

Here is a simple “Hello World” Bevy app:

```rust
fn main() {
   App::new()
       .add_systems(Update, hello_world_system)
       .run();
}

fn hello_world_system() {
   println!("hello world");
}
```

## Implementations

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/app.rs.html#143)

### impl [App](../prelude/struct.App.html "struct bevy::prelude::App")

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/app.rs.html#146)

#### pub fn [new](#method.new)() -> [App](../prelude/struct.App.html "struct bevy::prelude::App")

Creates a new [`App`](../prelude/struct.App.html "struct bevy::prelude::App") with some default structure to enable core engine features. This is the preferred constructor for most use cases.

##### [Examples found in repository](#scraped-examples)[?](../../scrape-examples-help.html)

examples/app/empty.rs ([line 6](../../src/empty/empty.rs.html#6))

```rust
5fn main() {
6    App::new().run();
7}
```

Hide additional examples

examples/app/empty\_defaults.rs ([line 6](../../src/empty_defaults/empty_defaults.rs.html#6))

```rust
5fn main() {
6    App::new().add_plugins(DefaultPlugins).run();
7}
```

examples/hello\_world.rs ([line 6](../../src/hello_world/hello_world.rs.html#6))

```rust
5fn main() {
6    App::new().add_systems(Update, hello_world_system).run();
7}
```

examples/2d/mesh2d.rs ([line 6](../../src/mesh2d/mesh2d.rs.html#6))

```rust
5fn main() {
6    App::new()
7        .add_plugins(DefaultPlugins)
8        .add_systems(Startup, setup)
9        .run();
10}
```

examples/2d/mesh2d\_alpha\_mode.rs ([line 11](../../src/mesh2d_alpha_mode/mesh2d_alpha_mode.rs.html#11))

```rust
10fn main() {
11    App::new()
12        .add_plugins(DefaultPlugins)
13        .add_systems(Startup, setup)
14        .run();
15}
```

examples/2d/mesh2d\_vertex\_color\_texture.rs ([line 7](../../src/mesh2d_vertex_color_texture/mesh2d_vertex_color_texture.rs.html#7))

```rust
6fn main() {
7    App::new()
8        .add_plugins(DefaultPlugins)
9        .add_systems(Startup, setup)
10        .run();
11}
```

Additional examples can be found in:  

*   [examples/2d/sprite.rs](../../src/sprite/sprite.rs.html#6)
*   [examples/2d/sprite\_flipping.rs](../../src/sprite_flipping/sprite_flipping.rs.html#6)
*   [examples/2d/sprite\_slice.rs](../../src/sprite_slice/sprite_slice.rs.html#6)
*   [examples/2d/transparency\_2d.rs](../../src/transparency_2d/transparency_2d.rs.html#7)
*   [examples/3d/orthographic.rs](../../src/orthographic/orthographic.rs.html#6)
*   [examples/3d/texture.rs](../../src/texture/texture.rs.html#8)
*   [examples/3d/two\_passes.rs](../../src/two_passes/two_passes.rs.html#6)
*   [examples/3d/vertex\_colors.rs](../../src/vertex_colors/vertex_colors.rs.html#6)
*   [examples/animation/eased\_motion.rs](../../src/eased_motion/eased_motion.rs.html#13)
*   [examples/asset/asset\_loading.rs](../../src/asset_loading/asset_loading.rs.html#6)
*   [examples/asset/hot\_asset\_reloading.rs](../../src/hot_asset_reloading/hot_asset_reloading.rs.html#11)
*   [examples/asset/repeated\_texture.rs](../../src/repeated_texture/repeated_texture.rs.html#11)
*   [examples/audio/audio.rs](../../src/audio/audio.rs.html#7)
*   [examples/camera/custom\_projection.rs](../../src/custom_projection/custom_projection.rs.html#7)
*   [examples/reflection/reflection.rs](../../src/reflection/reflection.rs.html#18)
*   [examples/reflection/reflection\_types.rs](../../src/reflection_types/reflection_types.rs.html#12)
*   [examples/ui/images/image\_node.rs](../../src/image_node/image_node.rs.html#7)
*   [examples/ui/scroll\_and\_overflow/drag\_to\_scroll.rs](../../src/drag_to_scroll/drag_to_scroll.rs.html#13)
*   [examples/ui/scroll\_and\_overflow/overflow\_clip\_margin.rs](../../src/overflow_clip_margin/overflow_clip_margin.rs.html#6)
*   [examples/ui/styling/borders.rs](../../src/borders/borders.rs.html#6)
*   [examples/ui/styling/stacked\_gradients.rs](../../src/stacked_gradients/stacked_gradients.rs.html#10)
*   [examples/ui/text/editable\_text\_filter.rs](../../src/editable_text_filter/editable_text_filter.rs.html#10)
*   [examples/ui/text/font\_query.rs](../../src/font_query/font_query.rs.html#8)
*   [examples/ui/text/font\_variations.rs](../../src/font_variations/font_variations.rs.html#7)
*   [examples/ui/text/font\_weights.rs](../../src/font_weights/font_weights.rs.html#6)
*   [examples/ui/text/strikethrough\_and\_underline.rs](../../src/strikethrough_and_underline/strikethrough_and_underline.rs.html#9)
*   [examples/ui/ui\_drag\_and\_drop.rs](../../src/ui_drag_and_drop/ui_drag_and_drop.rs.html#6)
*   [examples/ui/ui\_target\_camera.rs](../../src/ui_target_camera/ui_target_camera.rs.html#10)
*   [examples/input/mouse\_grab.rs](../../src/mouse_grab/mouse_grab.rs.html#9)
*   [examples/input/touch\_input.rs](../../src/touch_input/touch_input.rs.html#6)
*   [examples/ui/text/generic\_font\_families.rs](../../src/generic_font_families/generic_font_families.rs.html#25)
*   [examples/3d/3d\_scene.rs](../../src/3d_scene/3d_scene.rs.html#6)
*   [examples/input/gamepad\_input.rs](../../src/gamepad_input/gamepad_input.rs.html#6)
*   [examples/input/gamepad\_rumble.rs](../../src/gamepad_rumble/gamepad_rumble.rs.html#11)
*   [examples/scene/bsn.rs](../../src/bsn/bsn.rs.html#5)
*   [examples/input/touch\_input\_events.rs](../../src/touch_input_events/touch_input_events.rs.html#6)
*   [examples/input/keyboard\_input.rs](../../src/keyboard_input/keyboard_input.rs.html#6)
*   [examples/input/keyboard\_modifiers.rs](../../src/keyboard_modifiers/keyboard_modifiers.rs.html#6)
*   [examples/ecs/startup\_system.rs](../../src/startup_system/startup_system.rs.html#6)
*   [examples/input/char\_input\_events.rs](../../src/char_input_events/char_input_events.rs.html#9)
*   [examples/app/drag\_and\_drop.rs](../../src/drag_and_drop/drag_and_drop.rs.html#6)
*   [examples/input/mouse\_input\_events.rs](../../src/mouse_input_events/mouse_input_events.rs.html#12)
*   [examples/3d/decal.rs](../../src/decal/decal.rs.html#14)
*   [examples/input/keyboard\_input\_events.rs](../../src/keyboard_input_events/keyboard_input_events.rs.html#6)
*   [examples/picking/dragdrop\_picking.rs](../../src/dragdrop_picking/dragdrop_picking.rs.html#23)
*   [examples/2d/mesh2d\_manual.rs](../../src/mesh2d_manual/mesh2d_manual.rs.html#43)
*   [examples/asset/embedded\_asset.rs](../../src/embedded_asset/embedded_asset.rs.html#16)
*   [examples/shader\_advanced/manual\_material.rs](../../src/manual_material/manual_material.rs.html#42)
*   [examples/ui/text/multiline\_text\_input.rs](../../src/multiline_text_input/multiline_text_input.rs.html#13)
*   [examples/shader\_advanced/custom\_shader\_instancing.rs](../../src/custom_shader_instancing/custom_shader_instancing.rs.html#50)
*   [examples/shader\_advanced/custom\_render\_phase.rs](../../src/custom_render_phase/custom_render_phase.rs.html#69)
*   [examples/asset/custom\_asset\_reader.rs](../../src/custom_asset_reader/custom_asset_reader.rs.html#56)
*   [examples/picking/simple\_picking.rs](../../src/simple_picking/simple_picking.rs.html#6)
*   [examples/reflection/serialization.rs](../../src/serialization/serialization.rs.html#15)
*   [examples/ecs/delayed\_commands.rs](../../src/delayed_commands/delayed_commands.rs.html#9)
*   [examples/ecs/callbacks.rs](../../src/callbacks/callbacks.rs.html#10)
*   [examples/2d/mesh2d\_repeated\_texture.rs](../../src/mesh2d_repeated_texture/mesh2d_repeated_texture.rs.html#19)
*   [examples/input/mouse\_input.rs](../../src/mouse_input/mouse_input.rs.html#9)
*   [examples/input/gamepad\_input\_events.rs](../../src/gamepad_input_events/gamepad_input_events.rs.html#12)
*   [examples/ecs/hierarchy.rs](../../src/hierarchy/hierarchy.rs.html#12)
*   [examples/ui/styling/gradients.rs](../../src/gradients/gradients.rs.html#20)
*   [examples/2d/sprite\_tile.rs](../../src/sprite_tile/sprite_tile.rs.html#6)
*   [examples/ui/images/ui\_texture\_slice\_flip\_and\_tile.rs](../../src/ui_texture_slice_flip_and_tile/ui_texture_slice_flip_and_tile.rs.html#10)
*   [examples/app/without\_winit.rs](../../src/without_winit/without_winit.rs.html#6)
*   [examples/ecs/immutable\_components.rs](../../src/immutable_components/immutable_components.rs.html#199)
*   [examples/transforms/translation.rs](../../src/translation/translation.rs.html#26)
*   [examples/3d/3d\_viewport\_to\_world.rs](../../src/3d_viewport_to_world/3d_viewport_to_world.rs.html#6)
*   [examples/transforms/3d\_rotation.rs](../../src/3d_rotation/3d_rotation.rs.html#14)
*   [examples/3d/generate\_custom\_mesh.rs](../../src/generate_custom_mesh/generate_custom_mesh.rs.html#19)
*   [examples/camera/2d\_on\_ui.rs](../../src/2d_on_ui/2d_on_ui.rs.html#6)
*   [examples/ui/images/ui\_texture\_atlas\_slice.rs](../../src/ui_texture_atlas_slice/ui_texture_atlas_slice.rs.html#11)
*   [examples/ui/images/ui\_texture\_slice.rs](../../src/ui_texture_slice/ui_texture_slice.rs.html#11)
*   [examples/ui/layout/ghost\_nodes.rs](../../src/ghost_nodes/ghost_nodes.rs.html#15)
*   [examples/3d/lines.rs](../../src/lines/lines.rs.html#16)
*   [examples/3d/parenting.rs](../../src/parenting/parenting.rs.html#7)
*   [examples/animation/easing\_functions.rs](../../src/easing_functions/easing_functions.rs.html#10)
*   [examples/gizmos/anchored\_text\_gizmos.rs](../../src/anchored_text_gizmos/anchored_text_gizmos.rs.html#10)
*   [examples/2d/move\_sprite.rs](../../src/move_sprite/move_sprite.rs.html#6)
*   [examples/ui/scroll\_and\_overflow/overflow.rs](../../src/overflow/overflow.rs.html#6)
*   [examples/shader/animate\_shader.rs](../../src/animate_shader/animate_shader.rs.html#12)
*   [examples/shader/shader\_defs.rs](../../src/shader_defs/shader_defs.rs.html#18)
*   [examples/shader/shader\_material.rs](../../src/shader_material/shader_material.rs.html#11)
*   [examples/shader/shader\_material\_glsl.rs](../../src/shader_material_glsl/shader_material_glsl.rs.html#12)
*   [examples/shader\_advanced/custom\_vertex\_attribute.rs](../../src/custom_vertex_attribute/custom_vertex_attribute.rs.html#18)
*   [examples/3d/animated\_material.rs](../../src/animated_material/animated_material.rs.html#6)
*   [examples/3d/transparency\_3d.rs](../../src/transparency_3d/transparency_3d.rs.html#8)
*   [examples/ecs/system\_param.rs](../../src/system_param/system_param.rs.html#6)
*   [examples/gizmos/3d\_text\_gizmos.rs](../../src/3d_text_gizmos/3d_text_gizmos.rs.html#7)
*   [tests/3d/no\_prepass.rs](../../src/no_prepass/no_prepass.rs.html#6)
*   [examples/ui/layout/z\_index.rs](../../src/z_index/z_index.rs.html#12)
*   [examples/ui/styling/transparency\_ui.rs](../../src/transparency_ui/transparency_ui.rs.html#7)
*   [examples/2d/bloom\_2d.rs](../../src/bloom_2d/bloom_2d.rs.html#10)
*   [examples/gltf/load\_gltf\_extras.rs](../../src/load_gltf_extras/load_gltf_extras.rs.html#9)
*   [examples/gizmos/text\_gizmos\_font.rs](../../src/text_gizmos_font/text_gizmos_font.rs.html#17)
*   [examples/shader\_advanced/specialized\_mesh\_pipeline.rs](../../src/specialized_mesh_pipeline/specialized_mesh_pipeline.rs.html#46)
*   [examples/ui/text/system\_fonts.rs](../../src/system_fonts/system_fonts.rs.html#9)
*   [examples/3d/order\_independent\_transparency.rs](../../src/order_independent_transparency/order_independent_transparency.rs.html#14)
*   [examples/gltf/query\_gltf\_primitives.rs](../../src/query_gltf_primitives/query_gltf_primitives.rs.html#9)
*   [examples/2d/rotate\_to\_cursor.rs](../../src/rotate_to_cursor/rotate_to_cursor.rs.html#7)
*   [examples/3d/pbr.rs](../../src/pbr/pbr.rs.html#7)
*   [examples/ui/text/text\_background\_colors.rs](../../src/text_background_colors/text_background_colors.rs.html#9)
*   [examples/gizmos/axes.rs](../../src/axes/axes.rs.html#11)
*   [examples/math/random\_sampling.rs](../../src/random_sampling/random_sampling.rs.html#13)
*   [examples/ui/relative\_cursor\_position.rs](../../src/relative_cursor_position/relative_cursor_position.rs.html#6)
*   [examples/app/thread\_pool\_resources.rs](../../src/thread_pool_resources/thread_pool_resources.rs.html#7)
*   [examples/ecs/parallel\_query.rs](../../src/parallel_query/parallel_query.rs.html#74)
*   [examples/3d/camera\_sub\_view.rs](../../src/camera_sub_view/camera_sub_view.rs.html#15)
*   [examples/movement/smooth\_follow.rs](../../src/smooth_follow/smooth_follow.rs.html#11)
*   [examples/transforms/scale.rs](../../src/scale/scale.rs.html#29)
*   [examples/3d/render\_to\_texture.rs](../../src/render_to_texture/render_to_texture.rs.html#9)
*   [examples/3d/rotate\_environment\_map.rs](../../src/rotate_environment_map/rotate_environment_map.rs.html#16)
*   [examples/3d/split\_screen.rs](../../src/split_screen/split_screen.rs.html#10)
*   [examples/asset/generated\_assets.rs](../../src/generated_assets/generated_assets.rs.html#6)
*   [examples/3d/blend\_modes.rs](../../src/blend_modes/blend_modes.rs.html#17)
*   [examples/app/custom\_loop.rs](../../src/custom_loop/custom_loop.rs.html#44)
*   [examples/async\_tasks/async\_compute.rs](../../src/async_compute/async_compute.rs.html#24)
*   [examples/ui/widgets/viewport\_node.rs](../../src/viewport_node/viewport_node.rs.html#14)
*   [examples/window/screenshot.rs](../../src/screenshot/screenshot.rs.html#10)
*   [examples/asset/web\_asset.rs](../../src/web_asset/web_asset.rs.html#9)
*   [examples/3d/bloom\_3d.rs](../../src/bloom_3d/bloom_3d.rs.html#15)
*   [examples/ecs/contiguous\_query.rs](../../src/contiguous_query/contiguous_query.rs.html#50)
*   [examples/shader\_advanced/custom\_post\_processing.rs](../../src/custom_post_processing/custom_post_processing.rs.html#31)
*   [examples/gltf/edit\_material\_on\_gltf.rs](../../src/edit_material_on_gltf/edit_material_on_gltf.rs.html#9)
*   [examples/ui/widgets/virtual\_keyboard.rs](../../src/virtual_keyboard/virtual_keyboard.rs.html#15)
*   [examples/ui/widgets/tab\_navigation.rs](../../src/tab_navigation/tab_navigation.rs.html#13)
*   [examples/shader/automatic\_instancing.rs](../../src/automatic_instancing/automatic_instancing.rs.html#13)
*   [examples/shader/shader\_material\_2d.rs](../../src/shader_material_2d/shader_material_2d.rs.html#15)
*   [examples/shader/shader\_material\_bindless.rs](../../src/shader_material_bindless/shader_material_bindless.rs.html#41)
*   [examples/shader/storage\_buffer.rs](../../src/storage_buffer/storage_buffer.rs.html#13)
*   [examples/audio/play\_sound\_effect.rs](../../src/play_sound_effect/play_sound_effect.rs.html#22)
*   [examples/gizmos/light\_gizmos.rs](../../src/light_gizmos/light_gizmos.rs.html#11)
*   [examples/shader/fallback\_image.rs](../../src/fallback_image/fallback_image.rs.html#16)
*   [examples/ui/text/ime\_support.rs](../../src/ime_support/ime_support.rs.html#19)
*   [examples/ui/text/text\_input.rs](../../src/text_input/text_input.rs.html#36)
*   [examples/animation/animation\_events.rs](../../src/animation_events/animation_events.rs.html#11)
*   [examples/2d/sprite\_scale.rs](../../src/sprite_scale/sprite_scale.rs.html#6)
*   [examples/shader/shader\_material\_screenspace\_texture.rs](../../src/shader_material_screenspace_texture/shader_material_screenspace_texture.rs.html#11)
*   [examples/ui/ui\_transform.rs](../../src/ui_transform/ui_transform.rs.html#9)
*   [examples/3d/auto\_exposure.rs](../../src/auto_exposure/auto_exposure.rs.html#24)
*   [examples/camera/camera\_orbit.rs](../../src/camera_orbit/camera_orbit.rs.html#37)
*   [examples/3d/mesh\_ray\_cast.rs](../../src/mesh_ray_cast/mesh_ray_cast.rs.html#12)
*   [examples/ui/widgets/vertical\_slider.rs](../../src/vertical_slider/vertical_slider.rs.html#17)
*   [examples/2d/tilemap\_chunk\_orientation.rs](../../src/tilemap_chunk_orientation/tilemap_chunk_orientation.rs.html#10)
*   [examples/audio/pitch.rs](../../src/pitch/pitch.rs.html#7)
*   [examples/time/timers.rs](../../src/timers/timers.rs.html#6)
*   [examples/ui/scroll\_and\_overflow/scroll.rs](../../src/scroll/scroll.rs.html#14)
*   [examples/ui/text/letter\_spacing.rs](../../src/letter_spacing/letter_spacing.rs.html#22)
*   [examples/2d/sprite\_sheet.rs](../../src/sprite_sheet/sprite_sheet.rs.html#7)
*   [examples/ui/ui\_material.rs](../../src/ui_material/ui_material.rs.html#12)
*   [examples/window/multiple\_windows.rs](../../src/multiple_windows/multiple_windows.rs.html#6)
*   [examples/camera/2d\_top\_down\_camera.rs](../../src/2d_top_down_camera/2d_top_down_camera.rs.html#24)
*   [examples/reflection/generic\_reflection.rs](../../src/generic_reflection/generic_reflection.rs.html#7)
*   [examples/3d/spherical\_area\_lights.rs](../../src/spherical_area_lights/spherical_area_lights.rs.html#6)
*   [examples/audio/soundtrack.rs](../../src/soundtrack/soundtrack.rs.html#7)
*   [examples/picking/sprite\_picking.rs](../../src/sprite_picking/sprite_picking.rs.html#8)
*   [examples/window/clear\_color.rs](../../src/clear_color/clear_color.rs.html#8)
*   [examples/app/log\_layers.rs](../../src/log_layers/log_layers.rs.html#55)
*   [examples/gizmos/2d\_gizmos.rs](../../src/2d_gizmos/2d_gizmos.rs.html#8)
*   [examples/gltf/update\_gltf\_scene.rs](../../src/update_gltf_scene/update_gltf_scene.rs.html#7)
*   [examples/ui/layout/size\_constraints.rs](../../src/size_constraints/size_constraints.rs.html#6)
*   [examples/2d/text2d.rs](../../src/text2d/text2d.rs.html#17)
*   [examples/gltf/load\_gltf.rs](../../src/load_gltf/load_gltf.rs.html#10)
*   [examples/2d/pixel\_grid\_snap.rs](../../src/pixel_grid_snap/pixel_grid_snap.rs.html#28)
*   [examples/audio/audio\_control.rs](../../src/audio_control/audio_control.rs.html#6)
*   [examples/2d/2d\_viewport\_to\_world.rs](../../src/2d_viewport_to_world/2d_viewport_to_world.rs.html#14)
*   [examples/ecs/component\_hooks.rs](../../src/component_hooks/component_hooks.rs.html#52)
*   [examples/ui/text/text.rs](../../src/text/text.rs.html#14)
*   [examples/ui/scroll\_and\_overflow/scrollbars.rs](../../src/scrollbars/scrollbars.rs.html#12)
*   [examples/3d/motion\_blur.rs](../../src/motion_blur/motion_blur.rs.html#12)
*   [examples/ui/render\_ui\_to\_texture.rs](../../src/render_ui_to_texture/render_ui_to_texture.rs.html#23)
*   [examples/audio/spatial\_audio\_3d.rs](../../src/spatial_audio_3d/spatial_audio_3d.rs.html#9)
*   [examples/3d/atmospheric\_fog.rs](../../src/atmospheric_fog/atmospheric_fog.rs.html#16)
*   [examples/2d/tilemap\_chunk.rs](../../src/tilemap_chunk/tilemap_chunk.rs.html#13)
*   [examples/shader/array\_texture.rs](../../src/array_texture/array_texture.rs.html#22)
*   [examples/camera/pan\_camera\_controller.rs](../../src/pan_camera_controller/pan_camera_controller.rs.html#16)
*   [examples/gizmos/3d\_gizmos.rs](../../src/3d_gizmos/3d_gizmos.rs.html#11)
*   [examples/asset/asset\_decompression.rs](../../src/asset_decompression/asset_decompression.rs.html#97)
*   [examples/dev\_tools/infinite\_grid.rs](../../src/infinite_grid/infinite_grid.rs.html#14)
*   [examples/shader\_advanced/fullscreen\_material.rs](../../src/fullscreen_material/fullscreen_material.rs.html#16)
*   [examples/scene/world\_serialization.rs](../../src/world_serialization/world_serialization.rs.html#37)
*   [examples/picking/mesh\_picking.rs](../../src/mesh_picking/mesh_picking.rs.html#27)
*   [examples/animation/animated\_transform.rs](../../src/animated_transform/animated_transform.rs.html#11)
*   [examples/animation/animated\_ui.rs](../../src/animated_ui/animated_ui.rs.html#27)
*   [examples/shader\_advanced/texture\_binding\_array.rs](../../src/texture_binding_array/texture_binding_array.rs.html#26)
*   [examples/transforms/align.rs](../../src/align/align.rs.html#13)
*   [examples/3d/ssao.rs](../../src/ssao/ssao.rs.html#14)
*   [examples/shader/shader\_material\_wesl.rs](../../src/shader_material_wesl/shader_material_wesl.rs.html#18)
*   [examples/animation/morph\_targets.rs](../../src/morph_targets/morph_targets.rs.html#11)
*   [examples/ui/text/font\_atlas\_debug.rs](../../src/font_atlas_debug/font_atlas_debug.rs.html#10)
*   [examples/window/monitor\_info.rs](../../src/monitor_info/monitor_info.rs.html#10)
*   [examples/gltf/gltf\_skinned\_mesh.rs](../../src/gltf_skinned_mesh/gltf_skinned_mesh.rs.html#9)
*   [examples/testbed/full\_ui.rs](../../src/testbed_full_ui/full_ui.rs.html#20)
*   [examples/animation/custom\_skinned\_mesh.rs](../../src/custom_skinned_mesh/custom_skinned_mesh.rs.html#20)
*   [examples/camera/free\_camera\_controller.rs](../../src/free_camera_controller/free_camera_controller.rs.html#59)
*   [examples/ui/widgets/button.rs](../../src/button/button.rs.html#11)
*   [examples/shader/extended\_material.rs](../../src/extended_material/extended_material.rs.html#16)
*   [examples/ecs/iter\_combinations.rs](../../src/iter_combinations/iter_combinations.rs.html#8)
*   [examples/3d/spotlight.rs](../../src/spotlight/spotlight.rs.html#23)
*   [examples/2d/mesh2d\_arcs.rs](../../src/mesh2d_arcs/mesh2d_arcs.rs.html#18)
*   [examples/shader/extended\_material\_bindless.rs](../../src/extended_material_bindless/extended_material_bindless.rs.html#97)
*   [examples/3d/fog.rs](../../src/fog/fog.rs.html#31)
*   [examples/shader\_advanced/compute\_mesh.rs](../../src/compute_mesh/compute_mesh.rs.html#37)
*   [examples/gizmos/transform\_gizmo.rs](../../src/transform_gizmo/transform_gizmo.rs.html#19)
*   [examples/audio/decodable.rs](../../src/decodable/decodable.rs.html#84)
*   [examples/usage/cooldown.rs](../../src/cooldown/cooldown.rs.html#9)
*   [tests/3d/test\_invalid\_skinned\_mesh.rs](../../src/test_invalid_skinned_mesh/test_invalid_skinned_mesh.rs.html#17)
*   [examples/async\_tasks/external\_source\_external\_thread.rs](../../src/external_source_external_thread/external_source_external_thread.rs.html#10)
*   [examples/ui/layout/display\_and\_visibility.rs](../../src/display_and_visibility/display_and_visibility.rs.html#13)
*   [examples/3d/clearcoat.rs](../../src/clearcoat/clearcoat.rs.html#52)
*   [examples/diagnostics/enabling\_disabling\_diagnostic.rs](../../src/enabling_disabling_diagnostic/enabling_disabling_diagnostic.rs.html#12)
*   [examples/app/no\_renderer.rs](../../src/no_renderer/no_renderer.rs.html#13)
*   [examples/ui/layout/anchor\_layout.rs](../../src/anchor_layout/anchor_layout.rs.html#7)
*   [examples/audio/spatial\_audio\_2d.rs](../../src/spatial_audio_2d/spatial_audio_2d.rs.html#15)
*   [examples/remote/server.rs](../../src/server/server.rs.html#16)
*   [examples/ecs/change\_detection.rs](../../src/change_detection/change_detection.rs.html#7)
*   [examples/ecs/entity\_disabling.rs](../../src/entity_disabling/entity_disabling.rs.html#24)
*   [examples/ecs/removal\_detection.rs](../../src/removal_detection/removal_detection.rs.html#14)
*   [examples/animation/animated\_mesh.rs](../../src/animated_mesh/animated_mesh.rs.html#13)
*   [examples/usage/context\_menu.rs](../../src/context_menu/context_menu.rs.html#29)
*   [examples/camera/first\_person\_view\_model.rs](../../src/first_person_view_model/first_person_view_model.rs.html#53)
*   [examples/ui/layout/flex\_layout.rs](../../src/flex_layout/flex_layout.rs.html#9)
*   [examples/app/plugin.rs](../../src/plugin/plugin.rs.html#11)
*   [examples/ui/images/image\_node\_resizing.rs](../../src/image_node_resizing/image_node_resizing.rs.html#18)
*   [examples/shader/gpu\_readback.rs](../../src/gpu_readback/gpu_readback.rs.html#29)
*   [examples/transforms/transform.rs](../../src/transform/transform.rs.html#25)
*   [examples/app/render\_recovery.rs](../../src/render_recovery/render_recovery.rs.html#21)
*   [examples/asset/asset\_settings.rs](../../src/asset_settings/asset_settings.rs.html#9)
*   [examples/asset/custom\_asset.rs](../../src/custom_asset/custom_asset.rs.html#93)
*   [examples/showcase/contributors.rs](../../src/contributors/contributors.rs.html#16)
*   [examples/3d/parallax\_mapping.rs](../../src/parallax_mapping/parallax_mapping.rs.html#9)
*   [examples/asset/alter\_mesh.rs](../../src/alter_mesh/alter_mesh.rs.html#9)
*   [examples/3d/skybox.rs](../../src/skybox/skybox.rs.html#39)
*   [examples/asset/alter\_sprite.rs](../../src/alter_sprite/alter_sprite.rs.html#9)
*   [examples/2d/rotation.rs](../../src/rotation/rotation.rs.html#8)
*   [examples/ui/layout/grid.rs](../../src/grid/grid.rs.html#5)
*   [examples/2d/texture\_atlas.rs](../../src/texture_atlas/texture_atlas.rs.html#13)
*   [examples/3d/deferred\_rendering.rs](../../src/deferred_rendering/deferred_rendering.rs.html#19)
*   [examples/ecs/observer\_propagation.rs](../../src/observer_propagation/observer_propagation.rs.html#9)
*   [examples/3d/color\_grading.rs](../../src/color_grading/color_grading.rs.html#101)
*   [examples/window/window\_resizing.rs](../../src/window_resizing/window_resizing.rs.html#5)
*   [examples/ui/text/multiple\_text\_inputs.rs](../../src/multiple_text_inputs/multiple_text_inputs.rs.html#19)
*   [examples/ecs/custom\_query\_param.rs](../../src/custom_query_param/custom_query_param.rs.html#22)
*   [examples/dev\_tools/schedule\_data.rs](../../src/schedule_data/schedule_data.rs.html#6)
*   [examples/3d/fog\_volumes.rs](../../src/fog_volumes/fog_volumes.rs.html#17)
*   [examples/3d/transmission.rs](../../src/transmission/transmission.rs.html#47)
*   [examples/time/time.rs](../../src/time/time.rs.html#112)
*   [examples/3d/scrolling\_fog.rs](../../src/scrolling_fog/scrolling_fog.rs.html#26)
*   [examples/ui/images/ui\_texture\_atlas.rs](../../src/ui_texture_atlas/ui_texture_atlas.rs.html#6)
*   [examples/ecs/one\_shot\_systems.rs](../../src/one_shot_systems/one_shot_systems.rs.html#17)
*   [examples/3d/post\_processing.rs](../../src/post_processing/post_processing.rs.html#52)
*   [examples/3d/meshlet.rs](../../src/meshlet/meshlet.rs.html#18)
*   [examples/3d/shadow\_biases.rs](../../src/shadow_biases/shadow_biases.rs.html#10)
*   [examples/ui/ui\_scaling.rs](../../src/ui_scaling/ui_scaling.rs.html#10)
*   [examples/async\_tasks/async\_channel\_pattern.rs](../../src/async_channel_pattern/async_channel_pattern.rs.html#23)
*   [examples/animation/color\_animation.rs](../../src/color_animation/color_animation.rs.html#22)
*   [examples/app/return\_after\_run.rs](../../src/return_after_run/return_after_run.rs.html#12)
*   [examples/window/custom\_cursor\_image.rs](../../src/custom_cursor_image/custom_cursor_image.rs.html#12)
*   [examples/window/window\_drag\_move.rs](../../src/window_drag_move/window_drag_move.rs.html#43)
*   [examples/ecs/fixed\_timestep.rs](../../src/fixed_timestep/fixed_timestep.rs.html#6)
*   [examples/3d/rect\_light.rs](../../src/rect_light/rect_light.rs.html#7)
*   [examples/3d/shadow\_caster\_receiver.rs](../../src/shadow_caster_receiver/shadow_caster_receiver.rs.html#18)
*   [examples/ui/text/text\_debug.rs](../../src/text_debug/text_debug.rs.html#14)
*   [examples/window/scale\_factor\_override.rs](../../src/scale_factor_override/scale_factor_override.rs.html#10)
*   [examples/app/log\_layers\_ecs.rs](../../src/log_layers_ecs/log_layers_ecs.rs.html#26)
*   [tests/window/minimizing.rs](../../src/minimizing/minimizing.rs.html#8)
*   [examples/app/logs.rs](../../src/logs/logs.rs.html#6)
*   [examples/2d/2d\_shapes.rs](../../src/2d_shapes/2d_shapes.rs.html#24)
*   [examples/diagnostics/custom\_diagnostic.rs](../../src/custom_diagnostic/custom_diagnostic.rs.html#11)
*   [examples/ecs/hotpatching\_systems.rs](../../src/hotpatching_systems/hotpatching_systems.rs.html#28)
*   [examples/animation/animated\_mesh\_events.rs](../../src/animated_mesh_events/animated_mesh_events.rs.html#17)
*   [examples/showcase/loading\_screen.rs](../../src/loading_screen/loading_screen.rs.html#19)
*   [examples/3d/anti\_aliasing.rs](../../src/anti_aliasing/anti_aliasing.rs.html#30)
*   [examples/stress\_tests/many\_cameras\_lights.rs](../../src/many_cameras_lights/many_cameras_lights.rs.html#14)
*   [examples/ui/widgets/feathers\_gallery.rs](../../src/feathers_gallery/feathers_gallery.rs.html#65)
*   [examples/animation/animated\_mesh\_control.rs](../../src/animated_mesh_control/animated_mesh_control.rs.html#13)
*   [examples/gltf/gltf\_extension\_animation\_graph.rs](../../src/gltf_extension_animation_graph/gltf_extension_animation_graph.rs.html#22)
*   [examples/showcase/game\_menu.rs](../../src/game_menu/game_menu.rs.html#34)
*   [examples/math/cubic\_splines.rs](../../src/cubic_splines/cubic_splines.rs.html#14)
*   [examples/3d/volumetric\_fog.rs](../../src/volumetric_fog/volumetric_fog.rs.html#43)
*   [examples/2d/cpu\_draw.rs](../../src/cpu_draw/cpu_draw.rs.html#19)
*   [examples/3d/depth\_of\_field.rs](../../src/depth_of_field/depth_of_field.rs.html#53)
*   [examples/3d/lightmaps.rs](../../src/lightmaps/lightmaps.rs.html#28)
*   [examples/asset/asset\_saving.rs](../../src/asset_saving/asset_saving.rs.html#21)
*   [examples/ui/scroll\_and\_overflow/overflow\_debug.rs](../../src/overflow_debug/overflow_debug.rs.html#11)
*   [examples/ui/widgets/feathers\_counter.rs](../../src/feathers_counter/feathers_counter.rs.html#23)
*   [examples/3d/lighting.rs](../../src/lighting/lighting.rs.html#14)
*   [examples/3d/anisotropy.rs](../../src/anisotropy/anisotropy.rs.html#82)
*   [examples/3d/3d\_shapes.rs](../../src/3d_shapes/3d_shapes.rs.html#29)
*   [examples/shader/shader\_prepass.rs](../../src/shader_prepass/shader_prepass.rs.html#20)
*   [examples/app/settings.rs](../../src/settings/settings.rs.html#21)
*   [examples/asset/extra\_source.rs](../../src/extra_asset_source/extra_source.rs.html#14)
*   [examples/ecs/state\_scoped.rs](../../src/state_scoped/state_scoped.rs.html#14)
*   [examples/ecs/generic\_system.rs](../../src/generic_system/generic_system.rs.html#34)
*   [examples/remote/app\_under\_test.rs](../../src/app_under_test/app_under_test.rs.html#22)
*   [examples/2d/sprite\_animation.rs](../../src/sprite_animation/sprite_animation.rs.html#10)
*   [examples/window/persisting\_window\_settings.rs](../../src/persisting_window_settings/persisting_window_settings.rs.html#19)
*   [examples/stress\_tests/text\_pipeline.rs](../../src/text_pipeline/text_pipeline.rs.html#15)
*   [examples/gizmos/2d\_text\_gizmos.rs](../../src/2d_text_gizmos/2d_text_gizmos.rs.html#17)
*   [examples/animation/animation\_masks.rs](../../src/animation_masks/animation_masks.rs.html#96)
*   [examples/3d/atmosphere.rs](../../src/atmosphere/atmosphere.rs.html#42)
*   [examples/ui/styling/box\_shadow.rs](../../src/box_shadow/box_shadow.rs.html#118)
*   [examples/window/multi\_window\_text.rs](../../src/multi_window_text/multi_window_text.rs.html#11)
*   [examples/app/plugin\_group.rs](../../src/plugin_group/plugin_group.rs.html#7)
*   [examples/shader\_advanced/custom\_phase\_item.rs](../../src/custom_phase_item/custom_phase_item.rs.html#165)
*   [examples/picking/custom\_hit\_data.rs](../../src/custom_hit_data/custom_hit_data.rs.html#26)
*   [examples/3d/specular\_tint.rs](../../src/specular_tint/specular_tint.rs.html#52)
*   [examples/ui/window\_fallthrough.rs](../../src/window_fallthrough/window_fallthrough.rs.html#8)
*   [tests/window/resizing.rs](../../src/resizing/resizing.rs.html#21)
*   [examples/shader/compute\_shader\_game\_of\_life.rs](../../src/compute_shader_game_of_life/compute_shader_game_of_life.rs.html#33)
*   [examples/ui/text/text\_wrap\_debug.rs](../../src/text_wrap_debug/text_wrap_debug.rs.html#34)
*   [examples/ecs/extraction.rs](../../src/extraction/extraction.rs.html#47)
*   [examples/shader\_advanced/render\_depth\_to\_texture.rs](../../src/render_depth_to_texture/render_depth_to_texture.rs.html#102)
*   [examples/stress\_tests/many\_lights.rs](../../src/many_lights/many_lights.rs.html#20)
*   [examples/showcase/breakout.rs](../../src/breakout/breakout.rs.html#54)
*   [examples/camera/2d\_screen\_shake.rs](../../src/2d_screen_shake/2d_screen_shake.rs.html#54)
*   [examples/gltf/custom\_gltf\_vertex\_attribute.rs](../../src/custom_gltf_vertex_attribute/custom_gltf_vertex_attribute.rs.html#25)
*   [examples/3d/pccm.rs](../../src/pccm/pccm.rs.html#54)
*   [examples/3d/reflection\_probes.rs](../../src/reflection_probes/reflection_probes.rs.html#72)
*   [examples/3d/visibility\_range.rs](../../src/visibility_range/visibility_range.rs.html#87)
*   [examples/stress\_tests/many\_gizmos.rs](../../src/many_gizmos/many_gizmos.rs.html#15)
*   [examples/showcase/alien\_cake\_addict.rs](../../src/alien_cake_addict/alien_cake_addict.rs.html#21)
*   [examples/ecs/fallible\_params.rs](../../src/fallible_params/fallible_params.rs.html#33)
*   [examples/animation/animation\_graph.rs](../../src/animation_graph/animation_graph.rs.html#76)
*   [examples/ui/widgets/standard\_widgets.rs](../../src/standard_widgets/standard_widgets.rs.html#28)
*   [examples/math/bounding\_2d.rs](../../src/bounding_2d/bounding_2d.rs.html#10)
*   [examples/asset/asset\_saving\_with\_subassets.rs](../../src/asset_saving_with_subassets/asset_saving_with_subassets.rs.html#17)
*   [examples/window/transparent\_window.rs](../../src/transparent_window/transparent_window.rs.html#12)
*   [examples/stress\_tests/many\_glyphs.rs](../../src/many_glyphs/many_glyphs.rs.html#41)
*   [examples/stress\_tests/many\_animated\_sprite\_meshes.rs](../../src/many_animated_sprite_meshes/many_animated_sprite_meshes.rs.html#22)
*   [examples/stress\_tests/many\_animated\_sprites.rs](../../src/many_animated_sprites/many_animated_sprites.rs.html#20)
*   [examples/3d/contact\_shadows.rs](../../src/contact_shadows/contact_shadows.rs.html#82)
*   [examples/gltf/gltf\_extension\_mesh\_2d.rs](../../src/gltf_extension_mesh_2d/gltf_extension_mesh_2d.rs.html#30)
*   [examples/stress\_tests/many\_materials.rs](../../src/many_materials/many_materials.rs.html#26)
*   [examples/stress\_tests/many\_sprite\_meshes.rs](../../src/many_sprite_meshes/many_sprite_meshes.rs.html#30)
*   [examples/stress\_tests/many\_sprites.rs](../../src/many_sprites/many_sprites.rs.html#28)
*   [examples/stress\_tests/many\_text2d.rs](../../src/many_text2d/many_text2d.rs.html#71)
*   [examples/ecs/system\_piping.rs](../../src/system_piping/system_piping.rs.html#10)
*   [examples/3d/ssr.rs](../../src/ssr/ssr.rs.html#208)
*   [examples/picking/debug\_picking.rs](../../src/debug_picking/debug_picking.rs.html#8)
*   [examples/state/custom\_transitions.rs](../../src/custom_transitions/custom_transitions.rs.html#28)
*   [examples/usage/debug\_frustum\_culling.rs](../../src/debug_frustum_culling/debug_frustum_culling.rs.html#44)
*   [examples/camera/projection\_zoom.rs](../../src/projection_zoom/projection_zoom.rs.html#22)
*   [examples/time/virtual\_time.rs](../../src/virtual_time/virtual_time.rs.html#12)
*   [examples/asset/multi\_asset\_sync.rs](../../src/multi_asset_sync/multi_asset_sync.rs.html#17)
*   [examples/3d/solari.rs](../../src/solari/solari.rs.html#43)
*   [tests/3d/test\_skinned\_mesh\_bounds.rs](../../src/test_skinned_mesh_bounds/test_skinned_mesh_bounds.rs.html#16)
*   [examples/3d/occlusion\_culling.rs](../../src/occlusion_culling/occlusion_culling.rs.html#167)
*   [examples/stress\_tests/many\_cubes.rs](../../src/many_cubes/many_cubes.rs.html#126)
*   [examples/stress\_tests/many\_gradients.rs](../../src/many_gradients/many_gradients.rs.html#58)
*   [tests/window/desktop\_request\_redraw.rs](../../src/desktop_request_redraw/desktop_request_redraw.rs.html#10)
*   [examples/3d/pcss.rs](../../src/pcss/pcss.rs.html#117)
*   [examples/3d/mixed\_lighting.rs](../../src/mixed_lighting/mixed_lighting.rs.html#118)
*   [examples/app/headless.rs](../../src/headless/headless.rs.html#27)
*   [examples/3d/clustered\_decals.rs](../../src/clustered_decals/clustered_decals.rs.html#122)
*   [examples/3d/clustered\_decal\_maps.rs](../../src/clustered_decal_maps/clustered_decal_maps.rs.html#131)
*   [examples/window/low\_power.rs](../../src/low_power/low_power.rs.html#14)
*   [examples/3d/mirror.rs](../../src/mirror/mirror.rs.html#112)
*   [examples/state/sub\_states.rs](../../src/sub_states/sub_states.rs.html#36)
*   [examples/stress\_tests/many\_morph\_targets.rs](../../src/many_morph_targets/many_morph_targets.rs.html#141)
*   [examples/showcase/desk\_toy.rs](../../src/desk_toy/desk_toy.rs.html#20)
*   [examples/math/custom\_primitives.rs](../../src/custom_primitives/custom_primitives.rs.html#131)
*   [examples/3d/tonemapping.rs](../../src/tonemapping/tonemapping.rs.html#23)
*   [examples/testbed/2d.rs](../../src/testbed_2d/2d.rs.html#25)
*   [examples/ui/widgets/standard\_widgets\_observers.rs](../../src/standard_widgets_observers/standard_widgets_observers.rs.html#21)
*   [examples/stress\_tests/transform\_hierarchy.rs](../../src/transform_hierarchy/transform_hierarchy.rs.html#188)
*   [examples/2d/wireframe\_2d.rs](../../src/wireframe_2d/wireframe_2d.rs.html#21)
*   [examples/state/states.rs](../../src/states/states.rs.html#11)
*   [examples/3d/wireframe.rs](../../src/wireframe/wireframe.rs.html#22)
*   [examples/stress\_tests/many\_foxes.rs](../../src/many_foxes/many_foxes.rs.html#48)
*   [examples/app/externally\_driven\_headless\_renderer.rs](../../src/externally_driven_headless_renderer/externally_driven_headless_renderer.rs.html#54)
*   [examples/asset/processing/asset\_processing.rs](../../src/asset_processing/asset_processing.rs.html#20)
*   [examples/dev\_tools/fps\_overlay.rs](../../src/fps_overlay/fps_overlay.rs.html#17)
*   [examples/stress\_tests/bevymark\_3d.rs](../../src/bevymark_3d/bevymark_3d.rs.html#103)
*   [examples/3d/light\_textures.rs](../../src/light_textures/light_textures.rs.html#108)
*   [examples/stress\_tests/bevymark.rs](../../src/bevymark/bevymark.rs.html#134)
*   [examples/math/render\_primitives.rs](../../src/render_primitives/render_primitives.rs.html#10)
*   [examples/ecs/message.rs](../../src/message/message.rs.html#132)
*   [examples/diagnostics/log\_diagnostics.rs](../../src/log_diagnostics/log_diagnostics.rs.html#26)
*   [examples/ui/navigation/directional\_navigation.rs](../../src/directional_navigation/directional_navigation.rs.html#36)
*   [examples/ecs/system\_closure.rs](../../src/system_closure/system_closure.rs.html#27)
*   [examples/testbed/3d.rs](../../src/testbed_3d/3d.rs.html#24)
*   [examples/app/headless\_renderer.rs](../../src/headless_renderer/headless_renderer.rs.html#78)
*   [examples/3d/light\_probe\_blending.rs](../../src/light_probe_blending/light_probe_blending.rs.html#151)
*   [examples/2d/dynamic\_mip\_generation.rs](../../src/dynamic_mip_generation/dynamic_mip_generation.rs.html#209)
*   [examples/ecs/observers.rs](../../src/observers/observers.rs.html#9)
*   [examples/window/window\_settings.rs](../../src/window_settings/window_settings.rs.html#16)
*   [tests/ecs/ambiguity\_detection.rs](../../src/ambiguity_detection/ambiguity_detection.rs.html#15)
*   [examples/stress\_tests/many\_buttons.rs](../../src/many_buttons/many_buttons.rs.html#73)
*   [examples/ecs/error\_handling.rs](../../src/error_handling/error_handling.rs.html#13)
*   [examples/ui/navigation/directional\_navigation\_overrides.rs](../../src/directional_navigation_overrides/directional_navigation_overrides.rs.html#41)
*   [examples/testbed/ui.rs](../../src/testbed_ui/ui.rs.html#25)
*   [examples/3d/irradiance\_volumes.rs](../../src/irradiance_volumes/irradiance_volumes.rs.html#149)
*   [examples/ecs/nondeterministic\_system\_order.rs](../../src/nondeterministic_system_order/nondeterministic_system_order.rs.html#21)
*   [examples/state/computed\_states.rs](../../src/computed_states/computed_states.rs.html#174)
*   [examples/ecs/custom\_schedule.rs](../../src/custom_schedule/custom_schedule.rs.html#17)
*   [examples/movement/physics\_in\_fixed\_timestep.rs](../../src/physics_in_fixed_timestep/physics_in_fixed_timestep.rs.html#101)
*   [examples/ecs/run\_conditions.rs](../../src/run_conditions/run_conditions.rs.html#11)
*   [examples/ecs/ecs\_guide.rs](../../src/ecs_guide/ecs_guide.rs.html#296)
*   [examples/ecs/system\_stepping.rs](../../src/system_stepping/system_stepping.rs.html#8)

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/app.rs.html#153)

#### pub fn [empty](#method.empty)() -> [App](../prelude/struct.App.html "struct bevy::prelude::App")

Creates a new empty [`App`](../prelude/struct.App.html "struct bevy::prelude::App") with minimal default configuration.

Use this constructor if you want to customize scheduling, exit handling, cleanup, etc.

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/app.rs.html#165)

#### pub fn [update](#method.update)(&mut self)

Runs the default schedules of all sub-apps (starting with the “main” app) once.

##### [Examples found in repository](#scraped-examples-1)[?](../../scrape-examples-help.html)

examples/app/custom\_loop.rs ([line 22](../../src/custom_loop/custom_loop.rs.html#22))

```rust
10fn my_runner(mut app: App) -> AppExit {
11    // Finalize plugin building, including running any necessary clean-up.
12    // This is normally completed by the default runner.
13    app.finish();
14    app.cleanup();
15
16    println!("Type stuff into the console");
17    for line in io::stdin().lines() {
18        {
19            let mut input = app.world_mut().resource_mut::<Input>();
20            input.0 = line.unwrap();
21        }
22        app.update();
23
24        if let Some(exit) = app.should_exit() {
25            return exit;
26        }
27    }
28
29    AppExit::Success
30}
```

Hide additional examples

examples/time/time.rs ([line 46](../../src/time/time.rs.html#46))

```rust
35fn runner(mut app: App) -> AppExit {
36    banner();
37    help();
38    let stdin = io::stdin();
39    for line in stdin.lock().lines() {
40        if let Err(err) = line {
41            println!("read err: {err:#}");
42            break;
43        }
44        match line.unwrap().as_str() {
45            "" => {
46                app.update();
47            }
48            "f" => {
49                println!("FAST: setting relative speed to 2x");
50                app.world_mut()
51                    .resource_mut::<Time<Virtual>>()
52                    .set_relative_speed(2.0);
53            }
54            "n" => {
55                println!("NORMAL: setting relative speed to 1x");
56                app.world_mut()
57                    .resource_mut::<Time<Virtual>>()
58                    .set_relative_speed(1.0);
59            }
60            "s" => {
61                println!("SLOW: setting relative speed to 0.5x");
62                app.world_mut()
63                    .resource_mut::<Time<Virtual>>()
64                    .set_relative_speed(0.5);
65            }
66            "p" => {
67                println!("PAUSE: pausing virtual clock");
68                app.world_mut().resource_mut::<Time<Virtual>>().pause();
69            }
70            "u" => {
71                println!("UNPAUSE: resuming virtual clock");
72                app.world_mut().resource_mut::<Time<Virtual>>().unpause();
73            }
74            "q" => {
75                println!("QUITTING!");
76                break;
77            }
78            _ => {
79                help();
80            }
81        }
82    }
83
84    AppExit::Success
85}
```

examples/ecs/system\_stepping.rs ([line 38](../../src/system_stepping/system_stepping.rs.html#38))

```rust
7fn main() {
8    let mut app = App::new();
9
10    app
11        // to display log messages from Stepping resource
12        .add_plugins(LogPlugin::default())
13        .add_systems(
14            Update,
15            (
16                update_system_one,
17                // establish a dependency here to simplify descriptions below
18                update_system_two.after(update_system_one),
19                update_system_three.after(update_system_two),
20                update_system_four,
21            ),
22        )
23        .add_systems(PreUpdate, pre_update_system);
24
25    // For the simplicity of this example, we directly modify the `Stepping`
26    // resource here and run the systems with `App::update()`.  Each call to
27    // `App::update()` is the equivalent of a single frame render when using
28    // `App::run()`.
29    //
30    // In a real-world situation, the `Stepping` resource would be modified by
31    // a system based on input from the user.  A full demonstration of this can
32    // be found in the breakout example.
33    println!(
34        r#"
35    Actions: call app.update()
36     Result: All systems run normally"#
37    );
38    app.update();
39
40    println!(
41        r#"
42    Actions: Add the Stepping resource then call app.update()
43     Result: All systems run normally.  Stepping has no effect unless explicitly
44             configured for a Schedule, and Stepping has been enabled."#
45    );
46    app.insert_resource(Stepping::new());
47    app.update();
48
49    println!(
50        r#"
51    Actions: Add the Update Schedule to Stepping; enable Stepping; call
52             app.update()
53     Result: Only the systems in PreUpdate run.  When Stepping is enabled,
54             systems in the configured schedules will not run unless:
55             * Stepping::step_frame() is called
56             * Stepping::continue_frame() is called
57             * System has been configured to always run"#
58    );
59    let mut stepping = app.world_mut().resource_mut::<Stepping>();
60    stepping.add_schedule(Update).enable();
61    app.update();
62
63    println!(
64        r#"
65    Actions: call Stepping.step_frame(); call app.update()
66     Result: The PreUpdate systems run, and one Update system will run.  In
67             Stepping, step means run the next system across all the schedules 
68             that have been added to the Stepping resource."#
69    );
70    let mut stepping = app.world_mut().resource_mut::<Stepping>();
71    stepping.step_frame();
72    app.update();
73
74    println!(
75        r#"
76    Actions: call app.update()
77     Result: Only the PreUpdate systems run.  The previous call to
78             Stepping::step_frame() only applies for the next call to
79             app.update()/the next frame rendered.
80    "#
81    );
82    app.update();
83
84    println!(
85        r#"
86    Actions: call Stepping::continue_frame(); call app.update()
87     Result: PreUpdate system will run, and all remaining Update systems will
88             run.  Stepping::continue_frame() tells stepping to run all systems
89             starting after the last run system until it hits the end of the
90             frame, or it encounters a system with a breakpoint set.  In this
91             case, we previously performed a step, running one system in Update.
92             This continue will cause all remaining systems in Update to run."#
93    );
94    let mut stepping = app.world_mut().resource_mut::<Stepping>();
95    stepping.continue_frame();
96    app.update();
97
98    println!(
99        r#"
100    Actions: call Stepping::step_frame() & app.update() four times in a row
101     Result: PreUpdate system runs every time we call app.update(), along with
102             one system from the Update schedule each time.  This shows what
103             execution would look like to step through an entire frame of 
104             systems."#
105    );
106    for _ in 0..4 {
107        let mut stepping = app.world_mut().resource_mut::<Stepping>();
108        stepping.step_frame();
109        app.update();
110    }
111
112    println!(
113        r#"
114    Actions: Stepping::always_run(Update, update_system_two); step through all
115             systems
116     Result: PreUpdate system and update_system_two() will run every time we
117             call app.update().  We'll also only need to step three times to
118             execute all systems in the frame.  Stepping::always_run() allows
119             us to granularly allow systems to run when stepping is enabled."#
120    );
121    let mut stepping = app.world_mut().resource_mut::<Stepping>();
122    stepping.always_run(Update, update_system_two);
123    for _ in 0..3 {
124        let mut stepping = app.world_mut().resource_mut::<Stepping>();
125        stepping.step_frame();
126        app.update();
127    }
128
129    println!(
130        r#"
131    Actions: Stepping::never_run(Update, update_system_two); continue through
132             all systems
133     Result: All systems except update_system_two() will execute.
134             Stepping::never_run() allows us to disable systems while Stepping
135             is enabled."#
136    );
137    let mut stepping = app.world_mut().resource_mut::<Stepping>();
138    stepping.never_run(Update, update_system_two);
139    stepping.continue_frame();
140    app.update();
141
142    println!(
143        r#"
144    Actions: Stepping::set_breakpoint(Update, update_system_two); continue,
145             step, continue
146     Result: During the first continue, pre_update_system() and
147             update_system_one() will run.  update_system_four() may also run
148             as it has no dependency on update_system_two() or
149             update_system_three().  Nether update_system_two() nor
150             update_system_three() will run in the first app.update() call as
151             they form a chained dependency on update_system_one() and run
152             in order of one, two, three.  Stepping stops system execution in
153             the Update schedule when it encounters the breakpoint for
154             update_system_two().
155             During the step we run update_system_two() along with the
156             pre_update_system().
157             During the final continue pre_update_system() and
158             update_system_three() run."#
159    );
160    let mut stepping = app.world_mut().resource_mut::<Stepping>();
161    stepping.set_breakpoint(Update, update_system_two);
162    stepping.continue_frame();
163    app.update();
164    let mut stepping = app.world_mut().resource_mut::<Stepping>();
165    stepping.step_frame();
166    app.update();
167    let mut stepping = app.world_mut().resource_mut::<Stepping>();
168    stepping.continue_frame();
169    app.update();
170
171    println!(
172        r#"
173    Actions: Stepping::clear_breakpoint(Update, update_system_two); continue
174             through all systems
175     Result: All systems will run"#
176    );
177    let mut stepping = app.world_mut().resource_mut::<Stepping>();
178    stepping.clear_breakpoint(Update, update_system_two);
179    stepping.continue_frame();
180    app.update();
181
182    println!(
183        r#"
184    Actions: Stepping::disable(); app.update()
185     Result: All systems will run.  With Stepping disabled, there's no need to
186             call Stepping::step_frame() or Stepping::continue_frame() to run
187             systems in the Update schedule."#
188    );
189    let mut stepping = app.world_mut().resource_mut::<Stepping>();
190    stepping.disable();
191    app.update();
192}
```

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/app.rs.html#192)

#### pub fn [run](#method.run)(&mut self) -> [AppExit](../prelude/enum.AppExit.html "enum bevy::prelude::AppExit")

Runs the [`App`](../prelude/struct.App.html "struct bevy::prelude::App") by calling its [runner](../prelude/struct.App.html#method.set_runner "method bevy::prelude::App::set_runner").

This will (re)build the [`App`](../prelude/struct.App.html "struct bevy::prelude::App") first. For general usage, see the example on the item level documentation.

##### Caveats

Calls to [`App::run()`](../prelude/struct.App.html#method.run "method bevy::prelude::App::run") will never return on iOS and Web.

Headless apps can generally expect this method to return control to the caller when it completes, but that is not the case for windowed apps. Windowed apps are typically driven by an event loop and some platforms expect the program to terminate when the event loop ends.

By default, _Bevy_ uses the `winit` crate for window creation.

##### Panics

Panics if not all plugins have been built.

##### [Examples found in repository](#scraped-examples-2)[?](../../scrape-examples-help.html)

examples/app/empty.rs ([line 6](../../src/empty/empty.rs.html#6))

```rust
5fn main() {
6    App::new().run();
7}
```

Hide additional examples

examples/app/empty\_defaults.rs ([line 6](../../src/empty_defaults/empty_defaults.rs.html#6))

```rust
5fn main() {
6    App::new().add_plugins(DefaultPlugins).run();
7}
```

examples/hello\_world.rs ([line 6](../../src/hello_world/hello_world.rs.html#6))

```rust
5fn main() {
6    App::new().add_systems(Update, hello_world_system).run();
7}
```

examples/2d/mesh2d.rs ([line 9](../../src/mesh2d/mesh2d.rs.html#9))

```rust
5fn main() {
6    App::new()
7        .add_plugins(DefaultPlugins)
8        .add_systems(Startup, setup)
9        .run();
10}
```

examples/2d/mesh2d\_alpha\_mode.rs ([line 14](../../src/mesh2d_alpha_mode/mesh2d_alpha_mode.rs.html#14))

```rust
10fn main() {
11    App::new()
12        .add_plugins(DefaultPlugins)
13        .add_systems(Startup, setup)
14        .run();
15}
```

examples/2d/mesh2d\_vertex\_color\_texture.rs ([line 10](../../src/mesh2d_vertex_color_texture/mesh2d_vertex_color_texture.rs.html#10))

```rust
6fn main() {
7    App::new()
8        .add_plugins(DefaultPlugins)
9        .add_systems(Startup, setup)
10        .run();
11}
```

Additional examples can be found in:  

*   [examples/2d/sprite.rs](../../src/sprite/sprite.rs.html#9)
*   [examples/2d/sprite\_flipping.rs](../../src/sprite_flipping/sprite_flipping.rs.html#9)
*   [examples/2d/sprite\_slice.rs](../../src/sprite_slice/sprite_slice.rs.html#9)
*   [examples/2d/transparency\_2d.rs](../../src/transparency_2d/transparency_2d.rs.html#10)
*   [examples/3d/orthographic.rs](../../src/orthographic/orthographic.rs.html#9)
*   [examples/3d/texture.rs](../../src/texture/texture.rs.html#11)
*   [examples/3d/two\_passes.rs](../../src/two_passes/two_passes.rs.html#9)
*   [examples/3d/vertex\_colors.rs](../../src/vertex_colors/vertex_colors.rs.html#9)
*   [examples/animation/eased\_motion.rs](../../src/eased_motion/eased_motion.rs.html#16)
*   [examples/asset/asset\_loading.rs](../../src/asset_loading/asset_loading.rs.html#9)
*   [examples/asset/hot\_asset\_reloading.rs](../../src/hot_asset_reloading/hot_asset_reloading.rs.html#14)
*   [examples/asset/repeated\_texture.rs](../../src/repeated_texture/repeated_texture.rs.html#14)
*   [examples/audio/audio.rs](../../src/audio/audio.rs.html#10)
*   [examples/camera/custom\_projection.rs](../../src/custom_projection/custom_projection.rs.html#10)
*   [examples/reflection/reflection.rs](../../src/reflection/reflection.rs.html#21)
*   [examples/reflection/reflection\_types.rs](../../src/reflection_types/reflection_types.rs.html#15)
*   [examples/ui/images/image\_node.rs](../../src/image_node/image_node.rs.html#10)
*   [examples/ui/scroll\_and\_overflow/drag\_to\_scroll.rs](../../src/drag_to_scroll/drag_to_scroll.rs.html#16)
*   [examples/ui/scroll\_and\_overflow/overflow\_clip\_margin.rs](../../src/overflow_clip_margin/overflow_clip_margin.rs.html#9)
*   [examples/ui/styling/borders.rs](../../src/borders/borders.rs.html#9)
*   [examples/ui/styling/stacked\_gradients.rs](../../src/stacked_gradients/stacked_gradients.rs.html#13)
*   [examples/ui/text/editable\_text\_filter.rs](../../src/editable_text_filter/editable_text_filter.rs.html#13)
*   [examples/ui/text/font\_query.rs](../../src/font_query/font_query.rs.html#11)
*   [examples/ui/text/font\_variations.rs](../../src/font_variations/font_variations.rs.html#10)
*   [examples/ui/text/font\_weights.rs](../../src/font_weights/font_weights.rs.html#9)
*   [examples/ui/text/strikethrough\_and\_underline.rs](../../src/strikethrough_and_underline/strikethrough_and_underline.rs.html#12)
*   [examples/ui/ui\_drag\_and\_drop.rs](../../src/ui_drag_and_drop/ui_drag_and_drop.rs.html#9)
*   [examples/ui/ui\_target\_camera.rs](../../src/ui_target_camera/ui_target_camera.rs.html#13)
*   [examples/input/mouse\_grab.rs](../../src/mouse_grab/mouse_grab.rs.html#12)
*   [examples/input/touch\_input.rs](../../src/touch_input/touch_input.rs.html#9)
*   [examples/ui/text/generic\_font\_families.rs](../../src/generic_font_families/generic_font_families.rs.html#28)
*   [examples/3d/3d\_scene.rs](../../src/3d_scene/3d_scene.rs.html#9)
*   [examples/input/gamepad\_input.rs](../../src/gamepad_input/gamepad_input.rs.html#9)
*   [examples/input/gamepad\_rumble.rs](../../src/gamepad_rumble/gamepad_rumble.rs.html#14)
*   [examples/scene/bsn.rs](../../src/bsn/bsn.rs.html#8)
*   [examples/input/touch\_input\_events.rs](../../src/touch_input_events/touch_input_events.rs.html#9)
*   [examples/input/keyboard\_input.rs](../../src/keyboard_input/keyboard_input.rs.html#9)
*   [examples/input/keyboard\_modifiers.rs](../../src/keyboard_modifiers/keyboard_modifiers.rs.html#9)
*   [examples/ecs/startup\_system.rs](../../src/startup_system/startup_system.rs.html#9)
*   [examples/input/char\_input\_events.rs](../../src/char_input_events/char_input_events.rs.html#12)
*   [examples/app/drag\_and\_drop.rs](../../src/drag_and_drop/drag_and_drop.rs.html#9)
*   [examples/input/mouse\_input\_events.rs](../../src/mouse_input_events/mouse_input_events.rs.html#15)
*   [examples/3d/decal.rs](../../src/decal/decal.rs.html#17)
*   [examples/input/keyboard\_input\_events.rs](../../src/keyboard_input_events/keyboard_input_events.rs.html#9)
*   [examples/picking/dragdrop\_picking.rs](../../src/dragdrop_picking/dragdrop_picking.rs.html#26)
*   [examples/2d/mesh2d\_manual.rs](../../src/mesh2d_manual/mesh2d_manual.rs.html#46)
*   [examples/asset/embedded\_asset.rs](../../src/embedded_asset/embedded_asset.rs.html#19)
*   [examples/shader\_advanced/manual\_material.rs](../../src/manual_material/manual_material.rs.html#45)
*   [examples/ui/text/multiline\_text\_input.rs](../../src/multiline_text_input/multiline_text_input.rs.html#16)
*   [examples/shader\_advanced/custom\_shader\_instancing.rs](../../src/custom_shader_instancing/custom_shader_instancing.rs.html#53)
*   [examples/shader\_advanced/custom\_render\_phase.rs](../../src/custom_render_phase/custom_render_phase.rs.html#72)
*   [examples/asset/custom\_asset\_reader.rs](../../src/custom_asset_reader/custom_asset_reader.rs.html#59)
*   [examples/picking/simple\_picking.rs](../../src/simple_picking/simple_picking.rs.html#9)
*   [examples/reflection/serialization.rs](../../src/serialization/serialization.rs.html#18)
*   [examples/ecs/delayed\_commands.rs](../../src/delayed_commands/delayed_commands.rs.html#13)
*   [examples/ecs/callbacks.rs](../../src/callbacks/callbacks.rs.html#13)
*   [examples/2d/mesh2d\_repeated\_texture.rs](../../src/mesh2d_repeated_texture/mesh2d_repeated_texture.rs.html#22)
*   [examples/input/mouse\_input.rs](../../src/mouse_input/mouse_input.rs.html#12)
*   [examples/input/gamepad\_input\_events.rs](../../src/gamepad_input_events/gamepad_input_events.rs.html#15)
*   [examples/ecs/hierarchy.rs](../../src/hierarchy/hierarchy.rs.html#16)
*   [examples/ui/styling/gradients.rs](../../src/gradients/gradients.rs.html#24)
*   [examples/2d/sprite\_tile.rs](../../src/sprite_tile/sprite_tile.rs.html#10)
*   [examples/ui/images/ui\_texture\_slice\_flip\_and\_tile.rs](../../src/ui_texture_slice_flip_and_tile/ui_texture_slice_flip_and_tile.rs.html#14)
*   [examples/app/without\_winit.rs](../../src/without_winit/without_winit.rs.html#9)
*   [examples/ecs/immutable\_components.rs](../../src/immutable_components/immutable_components.rs.html#203)
*   [examples/transforms/translation.rs](../../src/translation/translation.rs.html#30)
*   [examples/3d/3d\_viewport\_to\_world.rs](../../src/3d_viewport_to_world/3d_viewport_to_world.rs.html#10)
*   [examples/transforms/3d\_rotation.rs](../../src/3d_rotation/3d_rotation.rs.html#18)
*   [examples/3d/generate\_custom\_mesh.rs](../../src/generate_custom_mesh/generate_custom_mesh.rs.html#23)
*   [examples/camera/2d\_on\_ui.rs](../../src/2d_on_ui/2d_on_ui.rs.html#10)
*   [examples/ui/images/ui\_texture\_atlas\_slice.rs](../../src/ui_texture_atlas_slice/ui_texture_atlas_slice.rs.html#15)
*   [examples/ui/images/ui\_texture\_slice.rs](../../src/ui_texture_slice/ui_texture_slice.rs.html#15)
*   [examples/ui/layout/ghost\_nodes.rs](../../src/ghost_nodes/ghost_nodes.rs.html#19)
*   [examples/3d/lines.rs](../../src/lines/lines.rs.html#19)
*   [examples/3d/parenting.rs](../../src/parenting/parenting.rs.html#11)
*   [examples/animation/easing\_functions.rs](../../src/easing_functions/easing_functions.rs.html#14)
*   [examples/gizmos/anchored\_text\_gizmos.rs](../../src/anchored_text_gizmos/anchored_text_gizmos.rs.html#14)
*   [examples/2d/move\_sprite.rs](../../src/move_sprite/move_sprite.rs.html#10)
*   [examples/ui/scroll\_and\_overflow/overflow.rs](../../src/overflow/overflow.rs.html#10)
*   [examples/shader/animate\_shader.rs](../../src/animate_shader/animate_shader.rs.html#15)
*   [examples/shader/shader\_defs.rs](../../src/shader_defs/shader_defs.rs.html#21)
*   [examples/shader/shader\_material.rs](../../src/shader_material/shader_material.rs.html#14)
*   [examples/shader/shader\_material\_glsl.rs](../../src/shader_material_glsl/shader_material_glsl.rs.html#15)
*   [examples/shader\_advanced/custom\_vertex\_attribute.rs](../../src/custom_vertex_attribute/custom_vertex_attribute.rs.html#21)
*   [examples/3d/animated\_material.rs](../../src/animated_material/animated_material.rs.html#10)
*   [examples/3d/transparency\_3d.rs](../../src/transparency_3d/transparency_3d.rs.html#12)
*   [examples/ecs/system\_param.rs](../../src/system_param/system_param.rs.html#10)
*   [examples/gizmos/3d\_text\_gizmos.rs](../../src/3d_text_gizmos/3d_text_gizmos.rs.html#11)
*   [tests/3d/no\_prepass.rs](../../src/no_prepass/no_prepass.rs.html#11)
*   [examples/ui/layout/z\_index.rs](../../src/z_index/z_index.rs.html#16)
*   [examples/ui/styling/transparency\_ui.rs](../../src/transparency_ui/transparency_ui.rs.html#11)
*   [examples/2d/bloom\_2d.rs](../../src/bloom_2d/bloom_2d.rs.html#14)
*   [examples/gltf/load\_gltf\_extras.rs](../../src/load_gltf_extras/load_gltf_extras.rs.html#13)
*   [examples/gizmos/text\_gizmos\_font.rs](../../src/text_gizmos_font/text_gizmos_font.rs.html#21)
*   [examples/shader\_advanced/specialized\_mesh\_pipeline.rs](../../src/specialized_mesh_pipeline/specialized_mesh_pipeline.rs.html#50)
*   [examples/ui/text/system\_fonts.rs](../../src/system_fonts/system_fonts.rs.html#13)
*   [examples/3d/order\_independent\_transparency.rs](../../src/order_independent_transparency/order_independent_transparency.rs.html#18)
*   [examples/gltf/query\_gltf\_primitives.rs](../../src/query_gltf_primitives/query_gltf_primitives.rs.html#13)
*   [examples/2d/rotate\_to\_cursor.rs](../../src/rotate_to_cursor/rotate_to_cursor.rs.html#11)
*   [examples/3d/pbr.rs](../../src/pbr/pbr.rs.html#11)
*   [examples/ui/text/text\_background\_colors.rs](../../src/text_background_colors/text_background_colors.rs.html#13)
*   [examples/gizmos/axes.rs](../../src/axes/axes.rs.html#15)
*   [examples/math/random\_sampling.rs](../../src/random_sampling/random_sampling.rs.html#17)
*   [examples/ui/relative\_cursor\_position.rs](../../src/relative_cursor_position/relative_cursor_position.rs.html#10)
*   [examples/app/thread\_pool\_resources.rs](../../src/thread_pool_resources/thread_pool_resources.rs.html#11)
*   [examples/ecs/parallel\_query.rs](../../src/parallel_query/parallel_query.rs.html#78)
*   [examples/3d/camera\_sub\_view.rs](../../src/camera_sub_view/camera_sub_view.rs.html#19)
*   [examples/movement/smooth\_follow.rs](../../src/smooth_follow/smooth_follow.rs.html#15)
*   [examples/transforms/scale.rs](../../src/scale/scale.rs.html#33)
*   [examples/3d/render\_to\_texture.rs](../../src/render_to_texture/render_to_texture.rs.html#13)
*   [examples/3d/rotate\_environment\_map.rs](../../src/rotate_environment_map/rotate_environment_map.rs.html#20)
*   [examples/3d/split\_screen.rs](../../src/split_screen/split_screen.rs.html#14)
*   [examples/asset/generated\_assets.rs](../../src/generated_assets/generated_assets.rs.html#10)
*   [examples/3d/blend\_modes.rs](../../src/blend_modes/blend_modes.rs.html#23)
*   [examples/app/custom\_loop.rs](../../src/custom_loop/custom_loop.rs.html#48)
*   [examples/async\_tasks/async\_compute.rs](../../src/async_compute/async_compute.rs.html#28)
*   [examples/ui/widgets/viewport\_node.rs](../../src/viewport_node/viewport_node.rs.html#18)
*   [examples/window/screenshot.rs](../../src/screenshot/screenshot.rs.html#14)
*   [examples/asset/web\_asset.rs](../../src/web_asset/web_asset.rs.html#14)
*   [examples/3d/bloom\_3d.rs](../../src/bloom_3d/bloom_3d.rs.html#19)
*   [examples/ecs/contiguous\_query.rs](../../src/contiguous_query/contiguous_query.rs.html#54)
*   [examples/shader\_advanced/custom\_post\_processing.rs](../../src/custom_post_processing/custom_post_processing.rs.html#35)
*   [examples/gltf/edit\_material\_on\_gltf.rs](../../src/edit_material_on_gltf/edit_material_on_gltf.rs.html#13)
*   [examples/ui/widgets/virtual\_keyboard.rs](../../src/virtual_keyboard/virtual_keyboard.rs.html#19)
*   [examples/ui/widgets/tab\_navigation.rs](../../src/tab_navigation/tab_navigation.rs.html#17)
*   [examples/shader/automatic\_instancing.rs](../../src/automatic_instancing/automatic_instancing.rs.html#17)
*   [examples/shader/shader\_material\_2d.rs](../../src/shader_material_2d/shader_material_2d.rs.html#21)
*   [examples/shader/shader\_material\_bindless.rs](../../src/shader_material_bindless/shader_material_bindless.rs.html#47)
*   [examples/shader/storage\_buffer.rs](../../src/storage_buffer/storage_buffer.rs.html#17)
*   [examples/audio/play\_sound\_effect.rs](../../src/play_sound_effect/play_sound_effect.rs.html#27)
*   [examples/gizmos/light\_gizmos.rs](../../src/light_gizmos/light_gizmos.rs.html#16)
*   [examples/shader/fallback\_image.rs](../../src/fallback_image/fallback_image.rs.html#22)
*   [examples/ui/text/ime\_support.rs](../../src/ime_support/ime_support.rs.html#24)
*   [examples/ui/text/text\_input.rs](../../src/text_input/text_input.rs.html#41)
*   [examples/animation/animation\_events.rs](../../src/animation_events/animation_events.rs.html#16)
*   [examples/2d/sprite\_scale.rs](../../src/sprite_scale/sprite_scale.rs.html#10)
*   [examples/shader/shader\_material\_screenspace\_texture.rs](../../src/shader_material_screenspace_texture/shader_material_screenspace_texture.rs.html#15)
*   [examples/ui/ui\_transform.rs](../../src/ui_transform/ui_transform.rs.html#14)
*   [examples/3d/auto\_exposure.rs](../../src/auto_exposure/auto_exposure.rs.html#29)
*   [examples/camera/camera\_orbit.rs](../../src/camera_orbit/camera_orbit.rs.html#42)
*   [examples/3d/mesh\_ray\_cast.rs](../../src/mesh_ray_cast/mesh_ray_cast.rs.html#17)
*   [examples/ui/widgets/vertical\_slider.rs](../../src/vertical_slider/vertical_slider.rs.html#21)
*   [examples/2d/tilemap\_chunk\_orientation.rs](../../src/tilemap_chunk_orientation/tilemap_chunk_orientation.rs.html#14)
*   [examples/audio/pitch.rs](../../src/pitch/pitch.rs.html#12)
*   [examples/time/timers.rs](../../src/timers/timers.rs.html#11)
*   [examples/ui/scroll\_and\_overflow/scroll.rs](../../src/scroll/scroll.rs.html#21)
*   [examples/ui/text/letter\_spacing.rs](../../src/letter_spacing/letter_spacing.rs.html#27)
*   [examples/2d/sprite\_sheet.rs](../../src/sprite_sheet/sprite_sheet.rs.html#11)
*   [examples/ui/ui\_material.rs](../../src/ui_material/ui_material.rs.html#17)
*   [examples/window/multiple\_windows.rs](../../src/multiple_windows/multiple_windows.rs.html#10)
*   [examples/camera/2d\_top\_down\_camera.rs](../../src/2d_top_down_camera/2d_top_down_camera.rs.html#28)
*   [examples/reflection/generic\_reflection.rs](../../src/generic_reflection/generic_reflection.rs.html#12)
*   [examples/3d/spherical\_area\_lights.rs](../../src/spherical_area_lights/spherical_area_lights.rs.html#13)
*   [examples/audio/soundtrack.rs](../../src/soundtrack/soundtrack.rs.html#12)
*   [examples/picking/sprite\_picking.rs](../../src/sprite_picking/sprite_picking.rs.html#12)
*   [examples/window/clear\_color.rs](../../src/clear_color/clear_color.rs.html#13)
*   [examples/app/log\_layers.rs](../../src/log_layers/log_layers.rs.html#63)
*   [examples/gizmos/2d\_gizmos.rs](../../src/2d_gizmos/2d_gizmos.rs.html#13)
*   [examples/gltf/update\_gltf\_scene.rs](../../src/update_gltf_scene/update_gltf_scene.rs.html#12)
*   [examples/ui/layout/size\_constraints.rs](../../src/size_constraints/size_constraints.rs.html#11)
*   [examples/2d/text2d.rs](../../src/text2d/text2d.rs.html#24)
*   [examples/gltf/load\_gltf.rs](../../src/load_gltf/load_gltf.rs.html#15)
*   [examples/2d/pixel\_grid\_snap.rs](../../src/pixel_grid_snap/pixel_grid_snap.rs.html#32)
*   [examples/audio/audio\_control.rs](../../src/audio_control/audio_control.rs.html#13)
*   [examples/2d/2d\_viewport\_to\_world.rs](../../src/2d_viewport_to_world/2d_viewport_to_world.rs.html#19)
*   [examples/ecs/component\_hooks.rs](../../src/component_hooks/component_hooks.rs.html#58)
*   [examples/ui/text/text.rs](../../src/text/text.rs.html#18)
*   [examples/ui/scroll\_and\_overflow/scrollbars.rs](../../src/scrollbars/scrollbars.rs.html#17)
*   [examples/3d/motion\_blur.rs](../../src/motion_blur/motion_blur.rs.html#17)
*   [examples/ui/render\_ui\_to\_texture.rs](../../src/render_ui_to_texture/render_ui_to_texture.rs.html#28)
*   [examples/audio/spatial\_audio\_3d.rs](../../src/spatial_audio_3d/spatial_audio_3d.rs.html#15)
*   [examples/3d/atmospheric\_fog.rs](../../src/atmospheric_fog/atmospheric_fog.rs.html#23)
*   [examples/2d/tilemap\_chunk.rs](../../src/tilemap_chunk/tilemap_chunk.rs.html#17)
*   [examples/shader/array\_texture.rs](../../src/array_texture/array_texture.rs.html#29)
*   [examples/camera/pan\_camera\_controller.rs](../../src/pan_camera_controller/pan_camera_controller.rs.html#20)
*   [examples/gizmos/3d\_gizmos.rs](../../src/3d_gizmos/3d_gizmos.rs.html#16)
*   [examples/asset/asset\_decompression.rs](../../src/asset_decompression/asset_decompression.rs.html#103)
*   [examples/dev\_tools/infinite\_grid.rs](../../src/infinite_grid/infinite_grid.rs.html#22)
*   [examples/shader\_advanced/fullscreen\_material.rs](../../src/fullscreen_material/fullscreen_material.rs.html#23)
*   [examples/scene/world\_serialization.rs](../../src/world_serialization/world_serialization.rs.html#44)
*   [examples/picking/mesh\_picking.rs](../../src/mesh_picking/mesh_picking.rs.html#32)
*   [examples/animation/animated\_transform.rs](../../src/animated_transform/animated_transform.rs.html#19)
*   [examples/animation/animated\_ui.rs](../../src/animated_ui/animated_ui.rs.html#32)
*   [examples/shader\_advanced/texture\_binding\_array.rs](../../src/texture_binding_array/texture_binding_array.rs.html#33)
*   [examples/transforms/align.rs](../../src/align/align.rs.html#18)
*   [examples/3d/ssao.rs](../../src/ssao/ssao.rs.html#22)
*   [examples/shader/shader\_material\_wesl.rs](../../src/shader_material_wesl/shader_material_wesl.rs.html#26)
*   [examples/animation/morph\_targets.rs](../../src/morph_targets/morph_targets.rs.html#19)
*   [examples/ui/text/font\_atlas\_debug.rs](../../src/font_atlas_debug/font_atlas_debug.rs.html#16)
*   [examples/window/monitor\_info.rs](../../src/monitor_info/monitor_info.rs.html#17)
*   [examples/gltf/gltf\_skinned\_mesh.rs](../../src/gltf_skinned_mesh/gltf_skinned_mesh.rs.html#17)
*   [examples/testbed/full\_ui.rs](../../src/testbed_full_ui/full_ui.rs.html#28)
*   [examples/animation/custom\_skinned\_mesh.rs](../../src/custom_skinned_mesh/custom_skinned_mesh.rs.html#28)
*   [examples/camera/free\_camera\_controller.rs](../../src/free_camera_controller/free_camera_controller.rs.html#65)
*   [examples/ui/widgets/button.rs](../../src/button/button.rs.html#17)
*   [examples/shader/extended\_material.rs](../../src/extended_material/extended_material.rs.html#23)
*   [examples/ecs/iter\_combinations.rs](../../src/iter_combinations/iter_combinations.rs.html#14)
*   [examples/3d/spotlight.rs](../../src/spotlight/spotlight.rs.html#31)
*   [examples/2d/mesh2d\_arcs.rs](../../src/mesh2d_arcs/mesh2d_arcs.rs.html#28)
*   [examples/shader/extended\_material\_bindless.rs](../../src/extended_material_bindless/extended_material_bindless.rs.html#104)
*   [examples/3d/fog.rs](../../src/fog/fog.rs.html#39)
*   [examples/shader\_advanced/compute\_mesh.rs](../../src/compute_mesh/compute_mesh.rs.html#45)
*   [examples/gizmos/transform\_gizmo.rs](../../src/transform_gizmo/transform_gizmo.rs.html#28)
*   [examples/audio/decodable.rs](../../src/decodable/decodable.rs.html#92)
*   [examples/usage/cooldown.rs](../../src/cooldown/cooldown.rs.html#19)
*   [tests/3d/test\_invalid\_skinned\_mesh.rs](../../src/test_invalid_skinned_mesh/test_invalid_skinned_mesh.rs.html#25)
*   [examples/async\_tasks/external\_source\_external\_thread.rs](../../src/external_source_external_thread/external_source_external_thread.rs.html#17)
*   [examples/ui/layout/display\_and\_visibility.rs](../../src/display_and_visibility/display_and_visibility.rs.html#24)
*   [examples/3d/clearcoat.rs](../../src/clearcoat/clearcoat.rs.html#59)
*   [examples/diagnostics/enabling\_disabling\_diagnostic.rs](../../src/enabling_disabling_diagnostic/enabling_disabling_diagnostic.rs.html#22)
*   [examples/app/no\_renderer.rs](../../src/no_renderer/no_renderer.rs.html#24)
*   [examples/ui/layout/anchor\_layout.rs](../../src/anchor_layout/anchor_layout.rs.html#16)
*   [examples/audio/spatial\_audio\_2d.rs](../../src/spatial_audio_2d/spatial_audio_2d.rs.html#23)
*   [examples/remote/server.rs](../../src/server/server.rs.html#23)
*   [examples/ecs/change\_detection.rs](../../src/change_detection/change_detection.rs.html#19)
*   [examples/ecs/entity\_disabling.rs](../../src/entity_disabling/entity_disabling.rs.html#32)
*   [examples/ecs/removal\_detection.rs](../../src/removal_detection/removal_detection.rs.html#21)
*   [examples/animation/animated\_mesh.rs](../../src/animated_mesh/animated_mesh.rs.html#22)
*   [examples/usage/context\_menu.rs](../../src/context_menu/context_menu.rs.html#36)
*   [examples/camera/first\_person\_view\_model.rs](../../src/first_person_view_model/first_person_view_model.rs.html#65)
*   [examples/ui/layout/flex\_layout.rs](../../src/flex_layout/flex_layout.rs.html#18)
*   [examples/app/plugin.rs](../../src/plugin/plugin.rs.html#20)
*   [examples/ui/images/image\_node\_resizing.rs](../../src/image_node_resizing/image_node_resizing.rs.html#28)
*   [examples/shader/gpu\_readback.rs](../../src/gpu_readback/gpu_readback.rs.html#38)
*   [examples/transforms/transform.rs](../../src/transform/transform.rs.html#37)
*   [examples/app/render\_recovery.rs](../../src/render_recovery/render_recovery.rs.html#31)
*   [examples/asset/asset\_settings.rs](../../src/asset_settings/asset_settings.rs.html#18)
*   [examples/asset/custom\_asset.rs](../../src/custom_asset/custom_asset.rs.html#102)
*   [examples/showcase/contributors.rs](../../src/contributors/contributors.rs.html#23)
*   [examples/3d/parallax\_mapping.rs](../../src/parallax_mapping/parallax_mapping.rs.html#22)
*   [examples/asset/alter\_mesh.rs](../../src/alter_mesh/alter_mesh.rs.html#20)
*   [examples/3d/skybox.rs](../../src/skybox/skybox.rs.html#51)
*   [examples/asset/alter\_sprite.rs](../../src/alter_sprite/alter_sprite.rs.html#20)
*   [examples/2d/rotation.rs](../../src/rotation/rotation.rs.html#20)
*   [examples/ui/layout/grid.rs](../../src/grid/grid.rs.html#15)
*   [examples/2d/texture\_atlas.rs](../../src/texture_atlas/texture_atlas.rs.html#19)
*   [examples/3d/deferred\_rendering.rs](../../src/deferred_rendering/deferred_rendering.rs.html#26)
*   [examples/ecs/observer\_propagation.rs](../../src/observer_propagation/observer_propagation.rs.html#18)
*   [examples/3d/color\_grading.rs](../../src/color_grading/color_grading.rs.html#114)
*   [examples/window/window\_resizing.rs](../../src/window_resizing/window_resizing.rs.html#14)
*   [examples/ui/text/multiple\_text\_inputs.rs](../../src/multiple_text_inputs/multiple_text_inputs.rs.html#31)
*   [examples/ecs/custom\_query\_param.rs](../../src/custom_query_param/custom_query_param.rs.html#35)
*   [examples/dev\_tools/schedule\_data.rs](../../src/schedule_data/schedule_data.rs.html#13)
*   [examples/3d/fog\_volumes.rs](../../src/fog_volumes/fog_volumes.rs.html#28)
*   [examples/3d/transmission.rs](../../src/transmission/transmission.rs.html#57)
*   [examples/time/time.rs](../../src/time/time.rs.html#120)
*   [examples/3d/scrolling\_fog.rs](../../src/scrolling_fog/scrolling_fog.rs.html#37)
*   [examples/ui/images/ui\_texture\_atlas.rs](../../src/ui_texture_atlas/ui_texture_atlas.rs.html#15)
*   [examples/ecs/one\_shot\_systems.rs](../../src/one_shot_systems/one_shot_systems.rs.html#28)
*   [examples/3d/post\_processing.rs](../../src/post_processing/post_processing.rs.html#63)
*   [examples/3d/meshlet.rs](../../src/meshlet/meshlet.rs.html#30)
*   [examples/3d/shadow\_biases.rs](../../src/shadow_biases/shadow_biases.rs.html#24)
*   [examples/ui/ui\_scaling.rs](../../src/ui_scaling/ui_scaling.rs.html#22)
*   [examples/async\_tasks/async\_channel\_pattern.rs](../../src/async_channel_pattern/async_channel_pattern.rs.html#36)
*   [examples/animation/color\_animation.rs](../../src/color_animation/color_animation.rs.html#36)
*   [examples/app/return\_after\_run.rs](../../src/return_after_run/return_after_run.rs.html#21)
*   [examples/window/custom\_cursor\_image.rs](../../src/custom_cursor_image/custom_cursor_image.rs.html#28)
*   [examples/window/window\_drag\_move.rs](../../src/window_drag_move/window_drag_move.rs.html#55)
*   [examples/ecs/fixed\_timestep.rs](../../src/fixed_timestep/fixed_timestep.rs.html#14)
*   [examples/3d/rect\_light.rs](../../src/rect_light/rect_light.rs.html#20)
*   [examples/3d/shadow\_caster\_receiver.rs](../../src/shadow_caster_receiver/shadow_caster_receiver.rs.html#22)
*   [examples/ui/text/text\_debug.rs](../../src/text_debug/text_debug.rs.html#27)
*   [examples/window/scale\_factor\_override.rs](../../src/scale_factor_override/scale_factor_override.rs.html#23)
*   [examples/app/log\_layers\_ecs.rs](../../src/log_layers_ecs/log_layers_ecs.rs.html#37)
*   [tests/window/minimizing.rs](../../src/minimizing/minimizing.rs.html#18)
*   [examples/app/logs.rs](../../src/logs/logs.rs.html#17)
*   [examples/2d/2d\_shapes.rs](../../src/2d_shapes/2d_shapes.rs.html#40)
*   [examples/diagnostics/custom\_diagnostic.rs](../../src/custom_diagnostic/custom_diagnostic.rs.html#21)
*   [examples/ecs/hotpatching\_systems.rs](../../src/hotpatching_systems/hotpatching_systems.rs.html#33)
*   [examples/animation/animated\_mesh\_events.rs](../../src/animated_mesh_events/animated_mesh_events.rs.html#30)
*   [examples/showcase/loading\_screen.rs](../../src/loading_screen/loading_screen.rs.html#30)
*   [examples/3d/anti\_aliasing.rs](../../src/anti_aliasing/anti_aliasing.rs.html#44)
*   [examples/stress\_tests/many\_cameras\_lights.rs](../../src/many_cameras_lights/many_cameras_lights.rs.html#26)
*   [examples/ui/widgets/feathers\_gallery.rs](../../src/feathers_gallery/feathers_gallery.rs.html#76)
*   [examples/animation/animated\_mesh\_control.rs](../../src/animated_mesh_control/animated_mesh_control.rs.html#29)
*   [examples/gltf/gltf\_extension\_animation\_graph.rs](../../src/gltf_extension_animation_graph/gltf_extension_animation_graph.rs.html#36)
*   [examples/showcase/game\_menu.rs](../../src/game_menu/game_menu.rs.html#44)
*   [examples/math/cubic\_splines.rs](../../src/cubic_splines/cubic_splines.rs.html#32)
*   [examples/3d/volumetric\_fog.rs](../../src/volumetric_fog/volumetric_fog.rs.html#57)
*   [examples/2d/cpu\_draw.rs](../../src/cpu_draw/cpu_draw.rs.html#28)
*   [examples/3d/depth\_of\_field.rs](../../src/depth_of_field/depth_of_field.rs.html#68)
*   [examples/3d/lightmaps.rs](../../src/lightmaps/lightmaps.rs.html#39)
*   [examples/asset/asset\_saving.rs](../../src/asset_saving/asset_saving.rs.html#33)
*   [examples/ui/scroll\_and\_overflow/overflow\_debug.rs](../../src/overflow_debug/overflow_debug.rs.html#26)
*   [examples/ui/widgets/feathers\_counter.rs](../../src/feathers_counter/feathers_counter.rs.html#38)
*   [examples/3d/lighting.rs](../../src/lighting/lighting.rs.html#32)
*   [examples/3d/anisotropy.rs](../../src/anisotropy/anisotropy.rs.html#96)
*   [examples/3d/3d\_shapes.rs](../../src/3d_shapes/3d_shapes.rs.html#45)
*   [examples/shader/shader\_prepass.rs](../../src/shader_prepass/shader_prepass.rs.html#34)
*   [examples/app/settings.rs](../../src/settings/settings.rs.html#34)
*   [examples/asset/extra\_source.rs](../../src/extra_asset_source/extra_source.rs.html#27)
*   [examples/ecs/state\_scoped.rs](../../src/state_scoped/state_scoped.rs.html#26)
*   [examples/ecs/generic\_system.rs](../../src/generic_system/generic_system.rs.html#49)
*   [examples/remote/app\_under\_test.rs](../../src/app_under_test/app_under_test.rs.html#37)
*   [examples/2d/sprite\_animation.rs](../../src/sprite_animation/sprite_animation.rs.html#23)
*   [examples/window/persisting\_window\_settings.rs](../../src/persisting_window_settings/persisting_window_settings.rs.html#35)
*   [examples/stress\_tests/text\_pipeline.rs](../../src/text_pipeline/text_pipeline.rs.html#31)
*   [examples/gizmos/2d\_text\_gizmos.rs](../../src/2d_text_gizmos/2d_text_gizmos.rs.html#33)
*   [examples/animation/animation\_masks.rs](../../src/animation_masks/animation_masks.rs.html#114)
*   [examples/3d/atmosphere.rs](../../src/atmosphere/atmosphere.rs.html#58)
*   [examples/ui/styling/box\_shadow.rs](../../src/box_shadow/box_shadow.rs.html#135)
*   [examples/window/multi\_window\_text.rs](../../src/multi_window_text/multi_window_text.rs.html#24)
*   [examples/app/plugin\_group.rs](../../src/plugin_group/plugin_group.rs.html#23)
*   [examples/shader\_advanced/custom\_phase\_item.rs](../../src/custom_phase_item/custom_phase_item.rs.html#181)
*   [examples/picking/custom\_hit\_data.rs](../../src/custom_hit_data/custom_hit_data.rs.html#46)
*   [examples/3d/specular\_tint.rs](../../src/specular_tint/specular_tint.rs.html#71)
*   [examples/ui/window\_fallthrough.rs](../../src/window_fallthrough/window_fallthrough.rs.html#22)
*   [tests/window/resizing.rs](../../src/resizing/resizing.rs.html#40)
*   [examples/shader/compute\_shader\_game\_of\_life.rs](../../src/compute_shader_game_of_life/compute_shader_game_of_life.rs.html#51)
*   [examples/ui/text/text\_wrap\_debug.rs](../../src/text_wrap_debug/text_wrap_debug.rs.html#41)
*   [examples/ecs/extraction.rs](../../src/extraction/extraction.rs.html#70)
*   [examples/shader\_advanced/render\_depth\_to\_texture.rs](../../src/render_depth_to_texture/render_depth_to_texture.rs.html#124)
*   [examples/stress\_tests/many\_lights.rs](../../src/many_lights/many_lights.rs.html#38)
*   [examples/showcase/breakout.rs](../../src/breakout/breakout.rs.html#74)
*   [examples/camera/2d\_screen\_shake.rs](../../src/2d_screen_shake/2d_screen_shake.rs.html#67)
*   [examples/gltf/custom\_gltf\_vertex\_attribute.rs](../../src/custom_gltf_vertex_attribute/custom_gltf_vertex_attribute.rs.html#43)
*   [examples/3d/pccm.rs](../../src/pccm/pccm.rs.html#74)
*   [examples/3d/reflection\_probes.rs](../../src/reflection_probes/reflection_probes.rs.html#92)
*   [examples/3d/visibility\_range.rs](../../src/visibility_range/visibility_range.rs.html#108)
*   [examples/stress\_tests/many\_gizmos.rs](../../src/many_gizmos/many_gizmos.rs.html#41)
*   [examples/showcase/alien\_cake\_addict.rs](../../src/alien_cake_addict/alien_cake_addict.rs.html#47)
*   [examples/ecs/fallible\_params.rs](../../src/fallible_params/fallible_params.rs.html#43)
*   [examples/animation/animation\_graph.rs](../../src/animation_graph/animation_graph.rs.html#96)
*   [examples/ui/widgets/standard\_widgets.rs](../../src/standard_widgets/standard_widgets.rs.html#50)
*   [examples/math/bounding\_2d.rs](../../src/bounding_2d/bounding_2d.rs.html#33)
*   [examples/asset/asset\_saving\_with\_subassets.rs](../../src/asset_saving_with_subassets/asset_saving_with_subassets.rs.html#39)
*   [examples/window/transparent\_window.rs](../../src/transparent_window/transparent_window.rs.html#30)
*   [examples/stress\_tests/many\_glyphs.rs](../../src/many_glyphs/many_glyphs.rs.html#61)
*   [examples/stress\_tests/many\_animated\_sprite\_meshes.rs](../../src/many_animated_sprite_meshes/many_animated_sprite_meshes.rs.html#46)
*   [examples/stress\_tests/many\_animated\_sprites.rs](../../src/many_animated_sprites/many_animated_sprites.rs.html#44)
*   [examples/3d/contact\_shadows.rs](../../src/contact_shadows/contact_shadows.rs.html#106)
*   [examples/gltf/gltf\_extension\_mesh\_2d.rs](../../src/gltf_extension_mesh_2d/gltf_extension_mesh_2d.rs.html#53)
*   [examples/stress\_tests/many\_materials.rs](../../src/many_materials/many_materials.rs.html#44)
*   [examples/stress\_tests/many\_sprite\_meshes.rs](../../src/many_sprite_meshes/many_sprite_meshes.rs.html#53)
*   [examples/stress\_tests/many\_sprites.rs](../../src/many_sprites/many_sprites.rs.html#51)
*   [examples/stress\_tests/many\_text2d.rs](../../src/many_text2d/many_text2d.rs.html#94)
*   [examples/ecs/system\_piping.rs](../../src/system_piping/system_piping.rs.html#37)
*   [examples/3d/ssr.rs](../../src/ssr/ssr.rs.html#225)
*   [examples/picking/debug\_picking.rs](../../src/debug_picking/debug_picking.rs.html#30)
*   [examples/state/custom\_transitions.rs](../../src/custom_transitions/custom_transitions.rs.html#48)
*   [examples/usage/debug\_frustum\_culling.rs](../../src/debug_frustum_culling/debug_frustum_culling.rs.html#71)
*   [examples/camera/projection\_zoom.rs](../../src/projection_zoom/projection_zoom.rs.html#39)
*   [examples/time/virtual\_time.rs](../../src/virtual_time/virtual_time.rs.html#31)
*   [examples/asset/multi\_asset\_sync.rs](../../src/multi_asset_sync/multi_asset_sync.rs.html#42)
*   [examples/3d/solari.rs](../../src/solari/solari.rs.html#74)
*   [tests/3d/test\_skinned\_mesh\_bounds.rs](../../src/test_skinned_mesh_bounds/test_skinned_mesh_bounds.rs.html#47)
*   [examples/3d/occlusion\_culling.rs](../../src/occlusion_culling/occlusion_culling.rs.html#193)
*   [examples/stress\_tests/many\_cubes.rs](../../src/many_cubes/many_cubes.rs.html#155)
*   [examples/stress\_tests/many\_gradients.rs](../../src/many_gradients/many_gradients.rs.html#76)
*   [tests/window/desktop\_request\_redraw.rs](../../src/desktop_request_redraw/desktop_request_redraw.rs.html#33)
*   [examples/3d/pcss.rs](../../src/pcss/pcss.rs.html#146)
*   [examples/3d/mixed\_lighting.rs](../../src/mixed_lighting/mixed_lighting.rs.html#145)
*   [examples/app/headless.rs](../../src/headless/headless.rs.html#30)
*   [examples/3d/clustered\_decals.rs](../../src/clustered_decals/clustered_decals.rs.html#150)
*   [examples/3d/clustered\_decal\_maps.rs](../../src/clustered_decal_maps/clustered_decal_maps.rs.html#164)
*   [examples/window/low\_power.rs](../../src/low_power/low_power.rs.html#43)
*   [examples/3d/mirror.rs](../../src/mirror/mirror.rs.html#140)
*   [examples/state/sub\_states.rs](../../src/sub_states/sub_states.rs.html#60)
*   [examples/stress\_tests/many\_morph\_targets.rs](../../src/many_morph_targets/many_morph_targets.rs.html#166)
*   [examples/showcase/desk\_toy.rs](../../src/desk_toy/desk_toy.rs.html#51)
*   [examples/math/custom\_primitives.rs](../../src/custom_primitives/custom_primitives.rs.html#168)
*   [examples/3d/tonemapping.rs](../../src/tonemapping/tonemapping.rs.html#59)
*   [examples/testbed/2d.rs](../../src/testbed_2d/2d.rs.html#50)
*   [examples/ui/widgets/standard\_widgets\_observers.rs](../../src/standard_widgets_observers/standard_widgets_observers.rs.html#41)
*   [examples/stress\_tests/transform\_hierarchy.rs](../../src/transform_hierarchy/transform_hierarchy.rs.html#203)
*   [examples/2d/wireframe\_2d.rs](../../src/wireframe_2d/wireframe_2d.rs.html#47)
*   [examples/state/states.rs](../../src/states/states.rs.html#34)
*   [examples/3d/wireframe.rs](../../src/wireframe/wireframe.rs.html#49)
*   [examples/stress\_tests/many\_foxes.rs](../../src/many_foxes/many_foxes.rs.html#79)
*   [examples/asset/processing/asset\_processing.rs](../../src/asset_processing/asset_processing.rs.html#43)
*   [examples/dev\_tools/fps\_overlay.rs](../../src/fps_overlay/fps_overlay.rs.html#48)
*   [examples/stress\_tests/bevymark\_3d.rs](../../src/bevymark_3d/bevymark_3d.rs.html#137)
*   [examples/3d/light\_textures.rs](../../src/light_textures/light_textures.rs.html#139)
*   [examples/stress\_tests/bevymark.rs](../../src/bevymark/bevymark.rs.html#169)
*   [examples/math/render\_primitives.rs](../../src/render_primitives/render_primitives.rs.html#52)
*   [examples/ecs/message.rs](../../src/message/message.rs.html#165)
*   [examples/diagnostics/log\_diagnostics.rs](../../src/log_diagnostics/log_diagnostics.rs.html#54)
*   [examples/ui/navigation/directional\_navigation.rs](../../src/directional_navigation/directional_navigation.rs.html#67)
*   [examples/ecs/system\_closure.rs](../../src/system_closure/system_closure.rs.html#46)
*   [examples/testbed/3d.rs](../../src/testbed_3d/3d.rs.html#60)
*   [examples/app/headless\_renderer.rs](../../src/headless_renderer/headless_renderer.rs.html#111)
*   [examples/3d/light\_probe\_blending.rs](../../src/light_probe_blending/light_probe_blending.rs.html#199)
*   [examples/2d/dynamic\_mip\_generation.rs](../../src/dynamic_mip_generation/dynamic_mip_generation.rs.html#257)
*   [examples/ecs/observers.rs](../../src/observers/observers.rs.html#43)
*   [examples/window/window\_settings.rs](../../src/window_settings/window_settings.rs.html#58)
*   [examples/stress\_tests/many\_buttons.rs](../../src/many_buttons/many_buttons.rs.html#126)
*   [examples/ecs/error\_handling.rs](../../src/error_handling/error_handling.rs.html#54)
*   [examples/ui/navigation/directional\_navigation\_overrides.rs](../../src/directional_navigation_overrides/directional_navigation_overrides.rs.html#77)
*   [examples/testbed/ui.rs](../../src/testbed_ui/ui.rs.html#68)
*   [examples/3d/irradiance\_volumes.rs](../../src/irradiance_volumes/irradiance_volumes.rs.html#215)
*   [examples/ecs/nondeterministic\_system\_order.rs](../../src/nondeterministic_system_order/nondeterministic_system_order.rs.html#62)
*   [examples/state/computed\_states.rs](../../src/computed_states/computed_states.rs.html#218)
*   [examples/ecs/custom\_schedule.rs](../../src/custom_schedule/custom_schedule.rs.html#55)
*   [examples/movement/physics\_in\_fixed\_timestep.rs](../../src/physics_in_fixed_timestep/physics_in_fixed_timestep.rs.html#141)
*   [examples/ecs/run\_conditions.rs](../../src/run_conditions/run_conditions.rs.html#52)
*   [examples/stress\_tests/many\_components.rs](../../src/many_components/many_components.rs.html#169)
*   [examples/ecs/ecs\_guide.rs](../../src/ecs_guide/ecs_guide.rs.html#365)

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/app.rs.html#231)

#### pub fn [set\_runner](#method.set_runner)( &mut self, f: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([App](../prelude/struct.App.html "struct bevy::prelude::App")) -> [AppExit](../prelude/enum.AppExit.html "enum bevy::prelude::AppExit") + 'static, ) -> &mut [App](../prelude/struct.App.html "struct bevy::prelude::App")

Sets the function that will be called when the app is run.

The runner function `f` is called only once by [`App::run`](../prelude/struct.App.html#method.run "method bevy::prelude::App::run"). If the presence of a main loop in the app is desired, it is the responsibility of the runner function to provide it.

The runner function is usually not set manually, but by Bevy integrated plugins (e.g. `WinitPlugin`).

##### Examples

```rust
fn my_runner(mut app: App) -> AppExit {
    loop {
        println!("In main loop");
        app.update();
        if let Some(exit) = app.should_exit() {
            return exit;
        }
    }
}

App::new()
    .set_runner(my_runner);
```

##### [Examples found in repository](#scraped-examples-3)[?](../../scrape-examples-help.html)

examples/app/custom\_loop.rs ([line 46](../../src/custom_loop/custom_loop.rs.html#46))

```rust
43fn main() -> AppExit {
44    App::new()
45        .insert_resource(Input(String::new()))
46        .set_runner(my_runner)
47        .add_systems(Update, (print_system, exit_system))
48        .run()
49}
```

Hide additional examples

examples/time/time.rs ([line 119](../../src/time/time.rs.html#119))

```rust
111fn main() {
112    App::new()
113        .add_plugins(MinimalPlugins)
114        .insert_resource(Time::<Virtual>::from_max_delta(Duration::from_secs(5)))
115        .insert_resource(Time::<Fixed>::from_duration(Duration::from_secs(1)))
116        .add_systems(PreUpdate, print_real_time)
117        .add_systems(FixedUpdate, print_fixed_time)
118        .add_systems(Update, print_time)
119        .set_runner(runner)
120        .run();
121}
```

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/app.rs.html#240)

#### pub fn [plugins\_state](#method.plugins_state)(&mut self) -> [PluginsState](enum.PluginsState.html "enum bevy::app::PluginsState")

Returns the state of all plugins. This is usually called by the event loop, but can be useful for situations where you want to use [`App::update`](../prelude/struct.App.html#method.update "method bevy::prelude::App::update").

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/app.rs.html#268)

#### pub fn [finish](#method.finish)(&mut self)

Runs [`Plugin::finish`](../prelude/trait.Plugin.html#method.finish "method bevy::prelude::Plugin::finish") for each plugin. This is usually called by the event loop once all plugins are ready, but can be useful for situations where you want to use [`App::update`](../prelude/struct.App.html#method.update "method bevy::prelude::App::update").

##### [Examples found in repository](#scraped-examples-4)[?](../../scrape-examples-help.html)

examples/app/custom\_loop.rs ([line 13](../../src/custom_loop/custom_loop.rs.html#13))

```rust
10fn my_runner(mut app: App) -> AppExit {
11    // Finalize plugin building, including running any necessary clean-up.
12    // This is normally completed by the default runner.
13    app.finish();
14    app.cleanup();
15
16    println!("Type stuff into the console");
17    for line in io::stdin().lines() {
18        {
19            let mut input = app.world_mut().resource_mut::<Input>();
20            input.0 = line.unwrap();
21        }
22        app.update();
23
24        if let Some(exit) = app.should_exit() {
25            return exit;
26        }
27    }
28
29    AppExit::Success
30}
```

Hide additional examples

examples/app/externally\_driven\_headless\_renderer.rs ([line 67](../../src/externally_driven_headless_renderer/externally_driven_headless_renderer.rs.html#67))

```rust
39    fn new() -> Self {
40        let render_plugin = RenderPlugin {
41            // Make sure all shaders are loaded for the first frame
42            synchronous_pipeline_compilation: true,
43            ..default()
44        };
45        // We don't have any windows, but the WindowPlugin is still needed
46        // because a lot of bevy expects it to be there. Just configure it
47        // to not have any windows and not exit automatically.
48        let window_plugin = WindowPlugin {
49            primary_window: None,
50            exit_condition: ExitCondition::DontExit,
51            ..default()
52        };
53
54        let mut app = App::new();
55        app.add_plugins(
56            DefaultPlugins
57                .set(window_plugin)
58                .set(render_plugin)
59                // Disable winit because we want to own the update loop ourselves.
60                .disable::<WinitPlugin>(),
61        )
62        .add_systems(Startup, spawn_test_scene)
63        .add_systems(Update, update_camera);
64
65        // We yeet the schedule runner and never call app.run(),
66        // so we have to finish and clean up ourselves
67        app.finish();
68        app.cleanup();
69
70        // We grab the sub apps cus we dont want the runner, as we'll
71        // be pumping the update loop ourselves manually.
72        Self(std::mem::take(app.sub_apps_mut()))
73    }
```

tests/ecs/ambiguity\_detection.rs ([line 39](../../src/ambiguity_detection/ambiguity_detection.rs.html#39))

```rust
14fn main() {
15    let mut app = App::new();
16    app.add_plugins(
17        DefaultPlugins
18            .build()
19            .set(RenderPlugin {
20                // llvmpipe driver can cause segfaults when aborting the binary while pipelines are being
21                // compiled (which happens very quickly in this example since we only run for a single
22                // frame). Synchronous pipeline compilation helps prevent these segfaults as the
23                // rendering thread blocks on these pipeline compilations.
24                synchronous_pipeline_compilation: true,
25                ..Default::default()
26            })
27            // We also have to disable pipelined rendering to ensure the test doesn't end while the
28            // rendering frame is still executing in another thread.
29            .disable::<PipelinedRenderingPlugin>(),
30    );
31
32    let main_app = app.main_mut();
33    configure_ambiguity_detection(main_app);
34
35    let sub_app = app.sub_app_mut(bevy_render::RenderApp);
36    configure_ambiguity_detection(sub_app);
37
38    // Make sure all the system stuff is added.
39    app.finish();
40    app.cleanup();
41
42    let main_app_ambiguities = count_ambiguities(app.main_mut());
43    assert_eq!(
44        main_app_ambiguities.total(),
45        0,
46        "Main app has unexpected ambiguities among the following schedules: \n{main_app_ambiguities:#?}.",
47    );
48
49    let render_app = app.sub_app_mut(bevy_render::RenderApp);
50    // Initialize the MainWorld so the render world systems don't fail initialization.
51    render_app.init_resource::<bevy_render::MainWorld>();
52    let render_app_ambiguities = count_ambiguities(render_app);
53    assert_eq!(
54        render_app_ambiguities.total(),
55        0,
56        "Render app has unexpected ambiguities among the following schedules: \n{render_app_ambiguities:#?}.",
57    );
58}
```

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/app.rs.html#288)

#### pub fn [cleanup](#method.cleanup)(&mut self)

Runs [`Plugin::cleanup`](../prelude/trait.Plugin.html#method.cleanup "method bevy::prelude::Plugin::cleanup") for each plugin. This is usually called by the event loop after [`App::finish`](../prelude/struct.App.html#method.finish "method bevy::prelude::App::finish"), but can be useful for situations where you want to use [`App::update`](../prelude/struct.App.html#method.update "method bevy::prelude::App::update").

##### [Examples found in repository](#scraped-examples-5)[?](../../scrape-examples-help.html)

examples/app/custom\_loop.rs ([line 14](../../src/custom_loop/custom_loop.rs.html#14))

```rust
10fn my_runner(mut app: App) -> AppExit {
11    // Finalize plugin building, including running any necessary clean-up.
12    // This is normally completed by the default runner.
13    app.finish();
14    app.cleanup();
15
16    println!("Type stuff into the console");
17    for line in io::stdin().lines() {
18        {
19            let mut input = app.world_mut().resource_mut::<Input>();
20            input.0 = line.unwrap();
21        }
22        app.update();
23
24        if let Some(exit) = app.should_exit() {
25            return exit;
26        }
27    }
28
29    AppExit::Success
30}
```

Hide additional examples

examples/app/externally\_driven\_headless\_renderer.rs ([line 68](../../src/externally_driven_headless_renderer/externally_driven_headless_renderer.rs.html#68))

```rust
39    fn new() -> Self {
40        let render_plugin = RenderPlugin {
41            // Make sure all shaders are loaded for the first frame
42            synchronous_pipeline_compilation: true,
43            ..default()
44        };
45        // We don't have any windows, but the WindowPlugin is still needed
46        // because a lot of bevy expects it to be there. Just configure it
47        // to not have any windows and not exit automatically.
48        let window_plugin = WindowPlugin {
49            primary_window: None,
50            exit_condition: ExitCondition::DontExit,
51            ..default()
52        };
53
54        let mut app = App::new();
55        app.add_plugins(
56            DefaultPlugins
57                .set(window_plugin)
58                .set(render_plugin)
59                // Disable winit because we want to own the update loop ourselves.
60                .disable::<WinitPlugin>(),
61        )
62        .add_systems(Startup, spawn_test_scene)
63        .add_systems(Update, update_camera);
64
65        // We yeet the schedule runner and never call app.run(),
66        // so we have to finish and clean up ourselves
67        app.finish();
68        app.cleanup();
69
70        // We grab the sub apps cus we dont want the runner, as we'll
71        // be pumping the update loop ourselves manually.
72        Self(std::mem::take(app.sub_apps_mut()))
73    }
```

tests/ecs/ambiguity\_detection.rs ([line 40](../../src/ambiguity_detection/ambiguity_detection.rs.html#40))

```rust
14fn main() {
15    let mut app = App::new();
16    app.add_plugins(
17        DefaultPlugins
18            .build()
19            .set(RenderPlugin {
20                // llvmpipe driver can cause segfaults when aborting the binary while pipelines are being
21                // compiled (which happens very quickly in this example since we only run for a single
22                // frame). Synchronous pipeline compilation helps prevent these segfaults as the
23                // rendering thread blocks on these pipeline compilations.
24                synchronous_pipeline_compilation: true,
25                ..Default::default()
26            })
27            // We also have to disable pipelined rendering to ensure the test doesn't end while the
28            // rendering frame is still executing in another thread.
29            .disable::<PipelinedRenderingPlugin>(),
30    );
31
32    let main_app = app.main_mut();
33    configure_ambiguity_detection(main_app);
34
35    let sub_app = app.sub_app_mut(bevy_render::RenderApp);
36    configure_ambiguity_detection(sub_app);
37
38    // Make sure all the system stuff is added.
39    app.finish();
40    app.cleanup();
41
42    let main_app_ambiguities = count_ambiguities(app.main_mut());
43    assert_eq!(
44        main_app_ambiguities.total(),
45        0,
46        "Main app has unexpected ambiguities among the following schedules: \n{main_app_ambiguities:#?}.",
47    );
48
49    let render_app = app.sub_app_mut(bevy_render::RenderApp);
50    // Initialize the MainWorld so the render world systems don't fail initialization.
51    render_app.init_resource::<bevy_render::MainWorld>();
52    let render_app_ambiguities = count_ambiguities(render_app);
53    assert_eq!(
54        render_app_ambiguities.total(),
55        0,
56        "Render app has unexpected ambiguities among the following schedules: \n{render_app_ambiguities:#?}.",
57    );
58}
```

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/app.rs.html#328-332)

#### pub fn [add\_systems](#method.add_systems)<M>( &mut self, schedule: impl [ScheduleLabel](../ecs/schedule/trait.ScheduleLabel.html "trait bevy::ecs::schedule::ScheduleLabel"), systems: impl [IntoScheduleConfigs](../prelude/trait.IntoScheduleConfigs.html "trait bevy::prelude::IntoScheduleConfigs")<[Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [System](../prelude/trait.System.html "trait bevy::prelude::System")<Out = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), In = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)\>>, M>, ) -> &mut [App](../prelude/struct.App.html "struct bevy::prelude::App")

Adds one or more systems to the given schedule in this app’s [`Schedules`](../prelude/struct.Schedules.html "struct bevy::prelude::Schedules").

##### Examples

```rust
app.add_systems(Update, (system_a, system_b, system_c));
app.add_systems(Update, (system_a, system_b).run_if(should_run));
```

##### [Examples found in repository](#scraped-examples-6)[?](../../scrape-examples-help.html)

examples/hello\_world.rs ([line 6](../../src/hello_world/hello_world.rs.html#6))

```rust
5fn main() {
6    App::new().add_systems(Update, hello_world_system).run();
7}
```

Hide additional examples

examples/camera/free\_camera\_controller.rs ([line 72](../../src/free_camera_controller/free_camera_controller.rs.html#72))

```rust
71    fn build(&self, app: &mut App) {
72        app.add_systems(Startup, spawn_camera);
73    }
74}
75
76fn spawn_camera(mut commands: Commands) {
77    commands.spawn((
78        Camera3d::default(),
79        Transform::from_xyz(0.0, 1.0, 0.0).looking_to(Vec3::X, Vec3::Y),
80        // This component stores all camera settings and state, which is used by the FreeCameraPlugin to
81        // control it. These properties can be changed at runtime, but beware the controller system is
82        // constantly using and modifying those values unless the enabled field is false.
83        FreeCamera {
84            sensitivity: 0.2,
85            friction: 25.0,
86            walk_speed: 3.0,
87            run_speed: 9.0,
88            ..default()
89        },
90    ));
91}
92
93// Plugin that handles camera settings controls and information text
94struct CameraSettingsPlugin;
95impl Plugin for CameraSettingsPlugin {
96    fn build(&self, app: &mut App) {
97        app.add_systems(PostStartup, spawn_text)
98            .add_systems(Update, (update_camera_settings, update_text));
99    }
100}
101
102#[derive(Component)]
103struct InfoText;
104
105fn spawn_text(mut commands: Commands, free_camera_query: Query<&FreeCamera>) {
106    commands.spawn((
107        Node {
108            position_type: PositionType::Absolute,
109            top: px(-16),
110            left: px(12),
111            ..default()
112        },
113        children![Text::new(format!(
114            "{}",
115            free_camera_query.single().unwrap(),
116        ))],
117    ));
118    commands.spawn((
119        Node {
120            position_type: PositionType::Absolute,
121            bottom: px(12),
122            left: px(12),
123            ..default()
124        },
125        children![Text::new(concat![
126            "Z/X: decrease/increase sensitivity\n",
127            "C/V: decrease/increase friction\n",
128            "F/G: decrease/increase scroll factor\n",
129            "B: enable/disable controller\n",
130            "T: world/local vertical movement"
131        ]),],
132    ));
133
134    // Mutable text marked with component
135    commands.spawn((
136        Node {
137            position_type: PositionType::Absolute,
138            top: px(12),
139            right: px(12),
140            ..default()
141        },
142        children![(InfoText, Text::new(""))],
143    ));
144}
145
146fn update_camera_settings(
147    mut camera_query: Query<(&mut FreeCamera, &mut FreeCameraState)>,
148    input: Res<ButtonInput<KeyCode>>,
149) {
150    let (mut free_camera, mut free_camera_state) = camera_query.single_mut().unwrap();
151
152    if input.pressed(KeyCode::KeyZ) {
153        free_camera.sensitivity = (free_camera.sensitivity - 0.005).max(0.005);
154    }
155    if input.pressed(KeyCode::KeyX) {
156        free_camera.sensitivity += 0.005;
157    }
158    if input.pressed(KeyCode::KeyC) {
159        free_camera.friction = (free_camera.friction - 0.2).max(0.0);
160    }
161    if input.pressed(KeyCode::KeyV) {
162        free_camera.friction += 0.2;
163    }
164    if input.pressed(KeyCode::KeyF) {
165        free_camera.scroll_factor = (free_camera.scroll_factor - 0.02).max(0.02);
166    }
167    if input.pressed(KeyCode::KeyG) {
168        free_camera.scroll_factor += 0.02;
169    }
170    if input.just_pressed(KeyCode::KeyB) {
171        free_camera_state.enabled = !free_camera_state.enabled;
172    }
173    if input.just_pressed(KeyCode::KeyT) {
174        free_camera.vertical_movement_axis = match free_camera.vertical_movement_axis {
175            VerticalMovementAxis::World => VerticalMovementAxis::Local,
176            VerticalMovementAxis::Local => VerticalMovementAxis::World,
177        };
178    }
179}
180
181fn update_text(
182    mut text_query: Query<&mut Text, With<InfoText>>,
183    camera_query: Query<(&FreeCamera, &FreeCameraState)>,
184) {
185    let mut text = text_query.single_mut().unwrap();
186
187    let (free_camera, free_camera_state) = camera_query.single().unwrap();
188
189    text.0 = format!(
190        "Enabled: {},\nSensitivity: {:.03}\nFriction: {:.01}\nScroll factor: {:.02}\nWalk Speed: {:.02}\nRun Speed: {:.02}\nSpeed: {:.02}",
191        free_camera_state.enabled,
192        free_camera.sensitivity,
193        free_camera.friction,
194        free_camera.scroll_factor,
195        free_camera.walk_speed * free_camera_state.speed_multiplier,
196        free_camera.run_speed * free_camera_state.speed_multiplier,
197        free_camera_state.velocity.length(),
198    );
199}
200
201// Plugin that spawns the scene and lighting.
202struct ScenePlugin;
203impl Plugin for ScenePlugin {
204    fn build(&self, app: &mut App) {
205        app.add_systems(Startup, (spawn_lights, spawn_world));
206    }
```

examples/app/plugin\_group.rs ([line 41](../../src/plugin_group/plugin_group.rs.html#41))

```rust
40    fn build(&self, app: &mut App) {
41        app.add_systems(Update, print_hello_system);
42    }
43}
44
45fn print_hello_system() {
46    info!("hello");
47}
48
49struct PrintWorldPlugin;
50
51impl Plugin for PrintWorldPlugin {
52    fn build(&self, app: &mut App) {
53        app.add_systems(Update, print_world_system);
54    }
```

examples/2d/mesh2d.rs ([line 8](../../src/mesh2d/mesh2d.rs.html#8))

```rust
5fn main() {
6    App::new()
7        .add_plugins(DefaultPlugins)
8        .add_systems(Startup, setup)
9        .run();
10}
```

examples/2d/mesh2d\_alpha\_mode.rs ([line 13](../../src/mesh2d_alpha_mode/mesh2d_alpha_mode.rs.html#13))

```rust
10fn main() {
11    App::new()
12        .add_plugins(DefaultPlugins)
13        .add_systems(Startup, setup)
14        .run();
15}
```

examples/2d/mesh2d\_vertex\_color\_texture.rs ([line 9](../../src/mesh2d_vertex_color_texture/mesh2d_vertex_color_texture.rs.html#9))

```rust
6fn main() {
7    App::new()
8        .add_plugins(DefaultPlugins)
9        .add_systems(Startup, setup)
10        .run();
11}
```

Additional examples can be found in:  

*   [examples/2d/sprite.rs](../../src/sprite/sprite.rs.html#8)
*   [examples/2d/sprite\_flipping.rs](../../src/sprite_flipping/sprite_flipping.rs.html#8)
*   [examples/2d/sprite\_slice.rs](../../src/sprite_slice/sprite_slice.rs.html#8)
*   [examples/2d/transparency\_2d.rs](../../src/transparency_2d/transparency_2d.rs.html#9)
*   [examples/3d/orthographic.rs](../../src/orthographic/orthographic.rs.html#8)
*   [examples/3d/texture.rs](../../src/texture/texture.rs.html#10)
*   [examples/3d/two\_passes.rs](../../src/two_passes/two_passes.rs.html#8)
*   [examples/3d/vertex\_colors.rs](../../src/vertex_colors/vertex_colors.rs.html#8)
*   [examples/animation/eased\_motion.rs](../../src/eased_motion/eased_motion.rs.html#15)
*   [examples/asset/asset\_loading.rs](../../src/asset_loading/asset_loading.rs.html#8)
*   [examples/asset/hot\_asset\_reloading.rs](../../src/hot_asset_reloading/hot_asset_reloading.rs.html#13)
*   [examples/asset/repeated\_texture.rs](../../src/repeated_texture/repeated_texture.rs.html#13)
*   [examples/audio/audio.rs](../../src/audio/audio.rs.html#9)
*   [examples/camera/custom\_projection.rs](../../src/custom_projection/custom_projection.rs.html#9)
*   [examples/reflection/reflection.rs](../../src/reflection/reflection.rs.html#20)
*   [examples/reflection/reflection\_types.rs](../../src/reflection_types/reflection_types.rs.html#14)
*   [examples/ui/images/image\_node.rs](../../src/image_node/image_node.rs.html#9)
*   [examples/ui/scroll\_and\_overflow/drag\_to\_scroll.rs](../../src/drag_to_scroll/drag_to_scroll.rs.html#15)
*   [examples/ui/scroll\_and\_overflow/overflow\_clip\_margin.rs](../../src/overflow_clip_margin/overflow_clip_margin.rs.html#8)
*   [examples/ui/styling/borders.rs](../../src/borders/borders.rs.html#8)
*   [examples/ui/styling/stacked\_gradients.rs](../../src/stacked_gradients/stacked_gradients.rs.html#12)
*   [examples/ui/text/editable\_text\_filter.rs](../../src/editable_text_filter/editable_text_filter.rs.html#12)
*   [examples/ui/text/font\_query.rs](../../src/font_query/font_query.rs.html#10)
*   [examples/ui/text/font\_variations.rs](../../src/font_variations/font_variations.rs.html#9)
*   [examples/ui/text/font\_weights.rs](../../src/font_weights/font_weights.rs.html#8)
*   [examples/ui/text/strikethrough\_and\_underline.rs](../../src/strikethrough_and_underline/strikethrough_and_underline.rs.html#11)
*   [examples/ui/ui\_drag\_and\_drop.rs](../../src/ui_drag_and_drop/ui_drag_and_drop.rs.html#8)
*   [examples/ui/ui\_target\_camera.rs](../../src/ui_target_camera/ui_target_camera.rs.html#12)
*   [examples/input/mouse\_grab.rs](../../src/mouse_grab/mouse_grab.rs.html#11)
*   [examples/input/touch\_input.rs](../../src/touch_input/touch_input.rs.html#8)
*   [examples/ui/text/generic\_font\_families.rs](../../src/generic_font_families/generic_font_families.rs.html#26)
*   [examples/3d/3d\_scene.rs](../../src/3d_scene/3d_scene.rs.html#8)
*   [examples/input/gamepad\_input.rs](../../src/gamepad_input/gamepad_input.rs.html#8)
*   [examples/input/gamepad\_rumble.rs](../../src/gamepad_rumble/gamepad_rumble.rs.html#13)
*   [examples/scene/bsn.rs](../../src/bsn/bsn.rs.html#7)
*   [examples/input/touch\_input\_events.rs](../../src/touch_input_events/touch_input_events.rs.html#8)
*   [examples/input/keyboard\_input.rs](../../src/keyboard_input/keyboard_input.rs.html#8)
*   [examples/input/keyboard\_modifiers.rs](../../src/keyboard_modifiers/keyboard_modifiers.rs.html#8)
*   [examples/ecs/startup\_system.rs](../../src/startup_system/startup_system.rs.html#7)
*   [examples/input/char\_input\_events.rs](../../src/char_input_events/char_input_events.rs.html#11)
*   [examples/app/drag\_and\_drop.rs](../../src/drag_and_drop/drag_and_drop.rs.html#8)
*   [examples/input/mouse\_input\_events.rs](../../src/mouse_input_events/mouse_input_events.rs.html#14)
*   [examples/3d/decal.rs](../../src/decal/decal.rs.html#16)
*   [examples/input/keyboard\_input\_events.rs](../../src/keyboard_input_events/keyboard_input_events.rs.html#8)
*   [examples/picking/dragdrop\_picking.rs](../../src/dragdrop_picking/dragdrop_picking.rs.html#25)
*   [examples/2d/mesh2d\_manual.rs](../../src/mesh2d_manual/mesh2d_manual.rs.html#45)
*   [examples/asset/embedded\_asset.rs](../../src/embedded_asset/embedded_asset.rs.html#18)
*   [examples/shader\_advanced/manual\_material.rs](../../src/manual_material/manual_material.rs.html#44)
*   [examples/ui/text/multiline\_text\_input.rs](../../src/multiline_text_input/multiline_text_input.rs.html#15)
*   [examples/shader\_advanced/custom\_shader\_instancing.rs](../../src/custom_shader_instancing/custom_shader_instancing.rs.html#52)
*   [examples/shader\_advanced/custom\_render\_phase.rs](../../src/custom_render_phase/custom_render_phase.rs.html#71)
*   [examples/asset/custom\_asset\_reader.rs](../../src/custom_asset_reader/custom_asset_reader.rs.html#58)
*   [examples/picking/simple\_picking.rs](../../src/simple_picking/simple_picking.rs.html#8)
*   [examples/reflection/serialization.rs](../../src/serialization/serialization.rs.html#17)
*   [examples/ecs/delayed\_commands.rs](../../src/delayed_commands/delayed_commands.rs.html#11)
*   [examples/ecs/callbacks.rs](../../src/callbacks/callbacks.rs.html#11)
*   [examples/2d/mesh2d\_repeated\_texture.rs](../../src/mesh2d_repeated_texture/mesh2d_repeated_texture.rs.html#21)
*   [examples/input/mouse\_input.rs](../../src/mouse_input/mouse_input.rs.html#11)
*   [examples/input/gamepad\_input\_events.rs](../../src/gamepad_input_events/gamepad_input_events.rs.html#14)
*   [examples/ecs/hierarchy.rs](../../src/hierarchy/hierarchy.rs.html#14)
*   [examples/ui/styling/gradients.rs](../../src/gradients/gradients.rs.html#22)
*   [examples/2d/sprite\_tile.rs](../../src/sprite_tile/sprite_tile.rs.html#8)
*   [examples/ui/images/ui\_texture\_slice\_flip\_and\_tile.rs](../../src/ui_texture_slice_flip_and_tile/ui_texture_slice_flip_and_tile.rs.html#13)
*   [examples/app/without\_winit.rs](../../src/without_winit/without_winit.rs.html#8)
*   [examples/ecs/immutable\_components.rs](../../src/immutable_components/immutable_components.rs.html#200)
*   [examples/transforms/translation.rs](../../src/translation/translation.rs.html#28)
*   [examples/3d/3d\_viewport\_to\_world.rs](../../src/3d_viewport_to_world/3d_viewport_to_world.rs.html#8)
*   [examples/transforms/3d\_rotation.rs](../../src/3d_rotation/3d_rotation.rs.html#16)
*   [examples/3d/generate\_custom\_mesh.rs](../../src/generate_custom_mesh/generate_custom_mesh.rs.html#21)
*   [examples/camera/2d\_on\_ui.rs](../../src/2d_on_ui/2d_on_ui.rs.html#8)
*   [examples/ui/images/ui\_texture\_atlas\_slice.rs](../../src/ui_texture_atlas_slice/ui_texture_atlas_slice.rs.html#13)
*   [examples/ui/images/ui\_texture\_slice.rs](../../src/ui_texture_slice/ui_texture_slice.rs.html#13)
*   [examples/ui/layout/ghost\_nodes.rs](../../src/ghost_nodes/ghost_nodes.rs.html#17)
*   [examples/3d/lines.rs](../../src/lines/lines.rs.html#18)
*   [examples/3d/parenting.rs](../../src/parenting/parenting.rs.html#9)
*   [examples/animation/easing\_functions.rs](../../src/easing_functions/easing_functions.rs.html#12)
*   [examples/gizmos/anchored\_text\_gizmos.rs](../../src/anchored_text_gizmos/anchored_text_gizmos.rs.html#12)
*   [examples/2d/move\_sprite.rs](../../src/move_sprite/move_sprite.rs.html#8)
*   [examples/ui/scroll\_and\_overflow/overflow.rs](../../src/overflow/overflow.rs.html#8)
*   [examples/shader/animate\_shader.rs](../../src/animate_shader/animate_shader.rs.html#14)
*   [examples/shader/shader\_defs.rs](../../src/shader_defs/shader_defs.rs.html#20)
*   [examples/shader/shader\_material.rs](../../src/shader_material/shader_material.rs.html#13)
*   [examples/shader/shader\_material\_glsl.rs](../../src/shader_material_glsl/shader_material_glsl.rs.html#14)
*   [examples/shader\_advanced/custom\_vertex\_attribute.rs](../../src/custom_vertex_attribute/custom_vertex_attribute.rs.html#20)
*   [examples/3d/animated\_material.rs](../../src/animated_material/animated_material.rs.html#8)
*   [examples/3d/transparency\_3d.rs](../../src/transparency_3d/transparency_3d.rs.html#10)
*   [examples/ecs/system\_param.rs](../../src/system_param/system_param.rs.html#8)
*   [examples/gizmos/3d\_text\_gizmos.rs](../../src/3d_text_gizmos/3d_text_gizmos.rs.html#9)
*   [examples/ui/layout/z\_index.rs](../../src/z_index/z_index.rs.html#15)
*   [examples/ui/styling/transparency\_ui.rs](../../src/transparency_ui/transparency_ui.rs.html#10)
*   [examples/2d/bloom\_2d.rs](../../src/bloom_2d/bloom_2d.rs.html#12)
*   [examples/gltf/load\_gltf\_extras.rs](../../src/load_gltf_extras/load_gltf_extras.rs.html#11)
*   [examples/gizmos/text\_gizmos\_font.rs](../../src/text_gizmos_font/text_gizmos_font.rs.html#19)
*   [examples/shader\_advanced/specialized\_mesh\_pipeline.rs](../../src/specialized_mesh_pipeline/specialized_mesh_pipeline.rs.html#49)
*   [examples/ui/text/system\_fonts.rs](../../src/system_fonts/system_fonts.rs.html#11)
*   [examples/3d/order\_independent\_transparency.rs](../../src/order_independent_transparency/order_independent_transparency.rs.html#16)
*   [examples/gltf/query\_gltf\_primitives.rs](../../src/query_gltf_primitives/query_gltf_primitives.rs.html#11)
*   [examples/2d/rotate\_to\_cursor.rs](../../src/rotate_to_cursor/rotate_to_cursor.rs.html#9)
*   [examples/3d/pbr.rs](../../src/pbr/pbr.rs.html#9)
*   [examples/ui/text/text\_background\_colors.rs](../../src/text_background_colors/text_background_colors.rs.html#11)
*   [examples/gizmos/axes.rs](../../src/axes/axes.rs.html#13)
*   [examples/math/random\_sampling.rs](../../src/random_sampling/random_sampling.rs.html#15)
*   [examples/ui/relative\_cursor\_position.rs](../../src/relative_cursor_position/relative_cursor_position.rs.html#8)
*   [examples/ecs/parallel\_query.rs](../../src/parallel_query/parallel_query.rs.html#76)
*   [examples/3d/camera\_sub\_view.rs](../../src/camera_sub_view/camera_sub_view.rs.html#17)
*   [examples/movement/smooth\_follow.rs](../../src/smooth_follow/smooth_follow.rs.html#13)
*   [examples/transforms/scale.rs](../../src/scale/scale.rs.html#31)
*   [examples/3d/render\_to\_texture.rs](../../src/render_to_texture/render_to_texture.rs.html#11)
*   [examples/3d/rotate\_environment\_map.rs](../../src/rotate_environment_map/rotate_environment_map.rs.html#18)
*   [examples/3d/split\_screen.rs](../../src/split_screen/split_screen.rs.html#12)
*   [examples/asset/generated\_assets.rs](../../src/generated_assets/generated_assets.rs.html#8)
*   [examples/3d/blend\_modes.rs](../../src/blend_modes/blend_modes.rs.html#20)
*   [examples/app/custom\_loop.rs](../../src/custom_loop/custom_loop.rs.html#47)
*   [examples/async\_tasks/async\_compute.rs](../../src/async_compute/async_compute.rs.html#26)
*   [examples/ui/widgets/viewport\_node.rs](../../src/viewport_node/viewport_node.rs.html#16)
*   [examples/window/screenshot.rs](../../src/screenshot/screenshot.rs.html#12)
*   [examples/asset/web\_asset.rs](../../src/web_asset/web_asset.rs.html#13)
*   [examples/3d/bloom\_3d.rs](../../src/bloom_3d/bloom_3d.rs.html#17)
*   [examples/ecs/contiguous\_query.rs](../../src/contiguous_query/contiguous_query.rs.html#52)
*   [examples/shader\_advanced/custom\_post\_processing.rs](../../src/custom_post_processing/custom_post_processing.rs.html#33)
*   [examples/gltf/edit\_material\_on\_gltf.rs](../../src/edit_material_on_gltf/edit_material_on_gltf.rs.html#11)
*   [examples/ui/widgets/virtual\_keyboard.rs](../../src/virtual_keyboard/virtual_keyboard.rs.html#18)
*   [examples/ui/widgets/tab\_navigation.rs](../../src/tab_navigation/tab_navigation.rs.html#15)
*   [examples/shader/automatic\_instancing.rs](../../src/automatic_instancing/automatic_instancing.rs.html#15)
*   [examples/shader/shader\_material\_2d.rs](../../src/shader_material_2d/shader_material_2d.rs.html#20)
*   [examples/shader/shader\_material\_bindless.rs](../../src/shader_material_bindless/shader_material_bindless.rs.html#46)
*   [examples/shader/storage\_buffer.rs](../../src/storage_buffer/storage_buffer.rs.html#15)
*   [examples/audio/play\_sound\_effect.rs](../../src/play_sound_effect/play_sound_effect.rs.html#25)
*   [examples/gizmos/light\_gizmos.rs](../../src/light_gizmos/light_gizmos.rs.html#13)
*   [examples/shader/fallback\_image.rs](../../src/fallback_image/fallback_image.rs.html#21)
*   [examples/ui/text/ime\_support.rs](../../src/ime_support/ime_support.rs.html#22)
*   [examples/ui/text/text\_input.rs](../../src/text_input/text_input.rs.html#39)
*   [examples/animation/animation\_events.rs](../../src/animation_events/animation_events.rs.html#13)
*   [examples/2d/sprite\_scale.rs](../../src/sprite_scale/sprite_scale.rs.html#8)
*   [examples/shader/shader\_material\_screenspace\_texture.rs](../../src/shader_material_screenspace_texture/shader_material_screenspace_texture.rs.html#13)
*   [examples/ui/ui\_transform.rs](../../src/ui_transform/ui_transform.rs.html#11)
*   [examples/3d/auto\_exposure.rs](../../src/auto_exposure/auto_exposure.rs.html#27)
*   [examples/camera/camera\_orbit.rs](../../src/camera_orbit/camera_orbit.rs.html#40)
*   [examples/3d/mesh\_ray\_cast.rs](../../src/mesh_ray_cast/mesh_ray_cast.rs.html#14)
*   [examples/ui/widgets/vertical\_slider.rs](../../src/vertical_slider/vertical_slider.rs.html#19)
*   [examples/2d/tilemap\_chunk\_orientation.rs](../../src/tilemap_chunk_orientation/tilemap_chunk_orientation.rs.html#13)
*   [examples/audio/pitch.rs](../../src/pitch/pitch.rs.html#10)
*   [examples/time/timers.rs](../../src/timers/timers.rs.html#9)
*   [examples/ui/scroll\_and\_overflow/scroll.rs](../../src/scroll/scroll.rs.html#17)
*   [examples/ui/text/letter\_spacing.rs](../../src/letter_spacing/letter_spacing.rs.html#25)
*   [examples/2d/sprite\_sheet.rs](../../src/sprite_sheet/sprite_sheet.rs.html#9)
*   [examples/ui/ui\_material.rs](../../src/ui_material/ui_material.rs.html#15)
*   [examples/window/multiple\_windows.rs](../../src/multiple_windows/multiple_windows.rs.html#9)
*   [examples/camera/2d\_top\_down\_camera.rs](../../src/2d_top_down_camera/2d_top_down_camera.rs.html#26)
*   [examples/reflection/generic\_reflection.rs](../../src/generic_reflection/generic_reflection.rs.html#11)
*   [examples/3d/spherical\_area\_lights.rs](../../src/spherical_area_lights/spherical_area_lights.rs.html#12)
*   [examples/audio/soundtrack.rs](../../src/soundtrack/soundtrack.rs.html#9)
*   [examples/picking/sprite\_picking.rs](../../src/sprite_picking/sprite_picking.rs.html#10)
*   [examples/window/clear\_color.rs](../../src/clear_color/clear_color.rs.html#11)
*   [examples/app/log\_layers.rs](../../src/log_layers/log_layers.rs.html#62)
*   [examples/gizmos/2d\_gizmos.rs](../../src/2d_gizmos/2d_gizmos.rs.html#11)
*   [examples/gltf/update\_gltf\_scene.rs](../../src/update_gltf_scene/update_gltf_scene.rs.html#10)
*   [examples/ui/layout/size\_constraints.rs](../../src/size_constraints/size_constraints.rs.html#9)
*   [examples/2d/text2d.rs](../../src/text2d/text2d.rs.html#19)
*   [examples/gltf/load\_gltf.rs](../../src/load_gltf/load_gltf.rs.html#13)
*   [examples/2d/pixel\_grid\_snap.rs](../../src/pixel_grid_snap/pixel_grid_snap.rs.html#30)
*   [examples/audio/audio\_control.rs](../../src/audio_control/audio_control.rs.html#8)
*   [examples/2d/2d\_viewport\_to\_world.rs](../../src/2d_viewport_to_world/2d_viewport_to_world.rs.html#16)
*   [examples/ecs/component\_hooks.rs](../../src/component_hooks/component_hooks.rs.html#54)
*   [examples/ui/text/text.rs](../../src/text/text.rs.html#16)
*   [examples/ui/scroll\_and\_overflow/scrollbars.rs](../../src/scrollbars/scrollbars.rs.html#15)
*   [examples/3d/motion\_blur.rs](../../src/motion_blur/motion_blur.rs.html#15)
*   [examples/ui/render\_ui\_to\_texture.rs](../../src/render_ui_to_texture/render_ui_to_texture.rs.html#25)
*   [examples/audio/spatial\_audio\_3d.rs](../../src/spatial_audio_3d/spatial_audio_3d.rs.html#11)
*   [examples/3d/atmospheric\_fog.rs](../../src/atmospheric_fog/atmospheric_fog.rs.html#18-21)
*   [examples/2d/tilemap\_chunk.rs](../../src/tilemap_chunk/tilemap_chunk.rs.html#15)
*   [examples/shader/array\_texture.rs](../../src/array_texture/array_texture.rs.html#27)
*   [examples/camera/pan\_camera\_controller.rs](../../src/pan_camera_controller/pan_camera_controller.rs.html#19)
*   [examples/gizmos/3d\_gizmos.rs](../../src/3d_gizmos/3d_gizmos.rs.html#14)
*   [examples/asset/asset\_decompression.rs](../../src/asset_decompression/asset_decompression.rs.html#101)
*   [examples/dev\_tools/infinite\_grid.rs](../../src/infinite_grid/infinite_grid.rs.html#21)
*   [examples/shader\_advanced/fullscreen\_material.rs](../../src/fullscreen_material/fullscreen_material.rs.html#21)
*   [examples/scene/world\_serialization.rs](../../src/world_serialization/world_serialization.rs.html#39-42)
*   [examples/picking/mesh\_picking.rs](../../src/mesh_picking/mesh_picking.rs.html#30)
*   [examples/animation/animated\_transform.rs](../../src/animated_transform/animated_transform.rs.html#18)
*   [examples/animation/animated\_ui.rs](../../src/animated_ui/animated_ui.rs.html#31)
*   [examples/shader\_advanced/texture\_binding\_array.rs](../../src/texture_binding_array/texture_binding_array.rs.html#32)
*   [examples/transforms/align.rs](../../src/align/align.rs.html#15)
*   [examples/3d/ssao.rs](../../src/ssao/ssao.rs.html#20)
*   [examples/shader/shader\_material\_wesl.rs](../../src/shader_material_wesl/shader_material_wesl.rs.html#24)
*   [examples/animation/morph\_targets.rs](../../src/morph_targets/morph_targets.rs.html#17)
*   [examples/ui/text/font\_atlas\_debug.rs](../../src/font_atlas_debug/font_atlas_debug.rs.html#14)
*   [examples/window/monitor\_info.rs](../../src/monitor_info/monitor_info.rs.html#16)
*   [examples/gltf/gltf\_skinned\_mesh.rs](../../src/gltf_skinned_mesh/gltf_skinned_mesh.rs.html#15)
*   [examples/testbed/full\_ui.rs](../../src/testbed_full_ui/full_ui.rs.html#22)
*   [examples/animation/custom\_skinned\_mesh.rs](../../src/custom_skinned_mesh/custom_skinned_mesh.rs.html#26)
*   [examples/ui/widgets/button.rs](../../src/button/button.rs.html#15)
*   [examples/shader/extended\_material.rs](../../src/extended_material/extended_material.rs.html#21)
*   [examples/ecs/iter\_combinations.rs](../../src/iter_combinations/iter_combinations.rs.html#11)
*   [examples/3d/spotlight.rs](../../src/spotlight/spotlight.rs.html#29)
*   [examples/2d/mesh2d\_arcs.rs](../../src/mesh2d_arcs/mesh2d_arcs.rs.html#20)
*   [examples/shader/extended\_material\_bindless.rs](../../src/extended_material_bindless/extended_material_bindless.rs.html#102)
*   [examples/app/plugin.rs](../../src/plugin/plugin.rs.html#38)
*   [examples/3d/fog.rs](../../src/fog/fog.rs.html#34-37)
*   [examples/shader\_advanced/compute\_mesh.rs](../../src/compute_mesh/compute_mesh.rs.html#44)
*   [examples/gizmos/transform\_gizmo.rs](../../src/transform_gizmo/transform_gizmo.rs.html#26)
*   [examples/audio/decodable.rs](../../src/decodable/decodable.rs.html#91)
*   [examples/usage/cooldown.rs](../../src/cooldown/cooldown.rs.html#11)
*   [tests/3d/test\_invalid\_skinned\_mesh.rs](../../src/test_invalid_skinned_mesh/test_invalid_skinned_mesh.rs.html#23)
*   [examples/async\_tasks/external\_source\_external\_thread.rs](../../src/external_source_external_thread/external_source_external_thread.rs.html#13)
*   [examples/ui/layout/display\_and\_visibility.rs](../../src/display_and_visibility/display_and_visibility.rs.html#15)
*   [examples/3d/clearcoat.rs](../../src/clearcoat/clearcoat.rs.html#55)
*   [examples/diagnostics/enabling\_disabling\_diagnostic.rs](../../src/enabling_disabling_diagnostic/enabling_disabling_diagnostic.rs.html#18-21)
*   [examples/ui/layout/anchor\_layout.rs](../../src/anchor_layout/anchor_layout.rs.html#15)
*   [examples/audio/spatial\_audio\_2d.rs](../../src/spatial_audio_2d/spatial_audio_2d.rs.html#20)
*   [examples/remote/server.rs](../../src/server/server.rs.html#20)
*   [examples/ecs/change\_detection.rs](../../src/change_detection/change_detection.rs.html#9)
*   [examples/ecs/entity\_disabling.rs](../../src/entity_disabling/entity_disabling.rs.html#27-30)
*   [examples/ecs/removal\_detection.rs](../../src/removal_detection/removal_detection.rs.html#16)
*   [examples/animation/animated\_mesh.rs](../../src/animated_mesh/animated_mesh.rs.html#20)
*   [examples/usage/context\_menu.rs](../../src/context_menu/context_menu.rs.html#31)
*   [examples/camera/first\_person\_view\_model.rs](../../src/first_person_view_model/first_person_view_model.rs.html#55-63)
*   [examples/ui/layout/flex\_layout.rs](../../src/flex_layout/flex_layout.rs.html#17)
*   [examples/ui/images/image\_node\_resizing.rs](../../src/image_node_resizing/image_node_resizing.rs.html#25)
*   [examples/shader/gpu\_readback.rs](../../src/gpu_readback/gpu_readback.rs.html#37)
*   [examples/transforms/transform.rs](../../src/transform/transform.rs.html#27)
*   [examples/app/render\_recovery.rs](../../src/render_recovery/render_recovery.rs.html#26)
*   [examples/asset/asset\_settings.rs](../../src/asset_settings/asset_settings.rs.html#17)
*   [examples/asset/custom\_asset.rs](../../src/custom_asset/custom_asset.rs.html#100)
*   [examples/showcase/contributors.rs](../../src/contributors/contributors.rs.html#20)
*   [examples/3d/parallax\_mapping.rs](../../src/parallax_mapping/parallax_mapping.rs.html#11)
*   [examples/asset/alter\_mesh.rs](../../src/alter_mesh/alter_mesh.rs.html#11)
*   [examples/3d/skybox.rs](../../src/skybox/skybox.rs.html#42)
*   [examples/asset/alter\_sprite.rs](../../src/alter_sprite/alter_sprite.rs.html#11)
*   [examples/2d/rotation.rs](../../src/rotation/rotation.rs.html#11)
*   [examples/ui/layout/grid.rs](../../src/grid/grid.rs.html#14)
*   [examples/2d/texture\_atlas.rs](../../src/texture_atlas/texture_atlas.rs.html#16)
*   [examples/3d/deferred\_rendering.rs](../../src/deferred_rendering/deferred_rendering.rs.html#24)
*   [examples/ecs/observer\_propagation.rs](../../src/observer_propagation/observer_propagation.rs.html#11)
*   [examples/3d/color\_grading.rs](../../src/color_grading/color_grading.rs.html#104)
*   [examples/window/window\_resizing.rs](../../src/window_resizing/window_resizing.rs.html#12)
*   [examples/ui/text/multiple\_text\_inputs.rs](../../src/multiple_text_inputs/multiple_text_inputs.rs.html#22)
*   [examples/ecs/custom\_query\_param.rs](../../src/custom_query_param/custom_query_param.rs.html#23)
*   [examples/3d/fog\_volumes.rs](../../src/fog_volumes/fog_volumes.rs.html#26)
*   [examples/3d/transmission.rs](../../src/transmission/transmission.rs.html#55)
*   [examples/time/time.rs](../../src/time/time.rs.html#116)
*   [examples/3d/scrolling\_fog.rs](../../src/scrolling_fog/scrolling_fog.rs.html#35)
*   [examples/ui/images/ui\_texture\_atlas.rs](../../src/ui_texture_atlas/ui_texture_atlas.rs.html#13)
*   [examples/ecs/one\_shot\_systems.rs](../../src/one_shot_systems/one_shot_systems.rs.html#19-26)
*   [examples/3d/post\_processing.rs](../../src/post_processing/post_processing.rs.html#55)
*   [examples/3d/meshlet.rs](../../src/meshlet/meshlet.rs.html#28)
*   [examples/3d/shadow\_biases.rs](../../src/shadow_biases/shadow_biases.rs.html#13)
*   [examples/ui/ui\_scaling.rs](../../src/ui_scaling/ui_scaling.rs.html#17)
*   [examples/async\_tasks/async\_channel\_pattern.rs](../../src/async_channel_pattern/async_channel_pattern.rs.html#25-34)
*   [examples/animation/color\_animation.rs](../../src/color_animation/color_animation.rs.html#24)
*   [examples/app/return\_after\_run.rs](../../src/return_after_run/return_after_run.rs.html#20)
*   [examples/window/custom\_cursor\_image.rs](../../src/custom_cursor_image/custom_cursor_image.rs.html#14-17)
*   [examples/window/window\_drag\_move.rs](../../src/window_drag_move/window_drag_move.rs.html#53)
*   [examples/ecs/fixed\_timestep.rs](../../src/fixed_timestep/fixed_timestep.rs.html#9)
*   [examples/3d/rect\_light.rs](../../src/rect_light/rect_light.rs.html#18)
*   [examples/3d/shadow\_caster\_receiver.rs](../../src/shadow_caster_receiver/shadow_caster_receiver.rs.html#20)
*   [examples/ui/text/text\_debug.rs](../../src/text_debug/text_debug.rs.html#25)
*   [examples/window/scale\_factor\_override.rs](../../src/scale_factor_override/scale_factor_override.rs.html#18)
*   [examples/app/log\_layers\_ecs.rs](../../src/log_layers_ecs/log_layers_ecs.rs.html#35)
*   [tests/window/minimizing.rs](../../src/minimizing/minimizing.rs.html#16)
*   [examples/app/logs.rs](../../src/logs/logs.rs.html#13)
*   [examples/2d/2d\_shapes.rs](../../src/2d_shapes/2d_shapes.rs.html#30)
*   [examples/diagnostics/custom\_diagnostic.rs](../../src/custom_diagnostic/custom_diagnostic.rs.html#20)
*   [examples/ecs/hotpatching\_systems.rs](../../src/hotpatching_systems/hotpatching_systems.rs.html#31)
*   [examples/animation/animated\_mesh\_events.rs](../../src/animated_mesh_events/animated_mesh_events.rs.html#26)
*   [examples/showcase/loading\_screen.rs](../../src/loading_screen/loading_screen.rs.html#25)
*   [examples/3d/anti\_aliasing.rs](../../src/anti_aliasing/anti_aliasing.rs.html#38)
*   [examples/stress\_tests/many\_cameras\_lights.rs](../../src/many_cameras_lights/many_cameras_lights.rs.html#24)
*   [examples/ui/widgets/feathers\_gallery.rs](../../src/feathers_gallery/feathers_gallery.rs.html#74)
*   [examples/animation/animated\_mesh\_control.rs](../../src/animated_mesh_control/animated_mesh_control.rs.html#20)
*   [examples/gltf/gltf\_extension\_animation\_graph.rs](../../src/gltf_extension_animation_graph/gltf_extension_animation_graph.rs.html#30-33)
*   [examples/showcase/game\_menu.rs](../../src/game_menu/game_menu.rs.html#41)
*   [examples/math/cubic\_splines.rs](../../src/cubic_splines/cubic_splines.rs.html#16)
*   [examples/3d/volumetric\_fog.rs](../../src/volumetric_fog/volumetric_fog.rs.html#53)
*   [examples/2d/cpu\_draw.rs](../../src/cpu_draw/cpu_draw.rs.html#26)
*   [examples/3d/depth\_of\_field.rs](../../src/depth_of_field/depth_of_field.rs.html#62)
*   [examples/3d/lightmaps.rs](../../src/lightmaps/lightmaps.rs.html#37)
*   [examples/asset/asset\_saving.rs](../../src/asset_saving/asset_saving.rs.html#29-32)
*   [examples/ui/scroll\_and\_overflow/overflow\_debug.rs](../../src/overflow_debug/overflow_debug.rs.html#14)
*   [examples/ui/widgets/feathers\_counter.rs](../../src/feathers_counter/feathers_counter.rs.html#33)
*   [examples/3d/lighting.rs](../../src/lighting/lighting.rs.html#22)
*   [examples/3d/anisotropy.rs](../../src/anisotropy/anisotropy.rs.html#91)
*   [examples/3d/3d\_shapes.rs](../../src/3d_shapes/3d_shapes.rs.html#35)
*   [examples/shader/shader\_prepass.rs](../../src/shader_prepass/shader_prepass.rs.html#32)
*   [examples/app/settings.rs](../../src/settings/settings.rs.html#32)
*   [examples/asset/extra\_source.rs](../../src/extra_asset_source/extra_source.rs.html#26)
*   [examples/ecs/state\_scoped.rs](../../src/state_scoped/state_scoped.rs.html#17)
*   [examples/ecs/generic\_system.rs](../../src/generic_system/generic_system.rs.html#37)
*   [examples/remote/app\_under\_test.rs](../../src/app_under_test/app_under_test.rs.html#29)
*   [examples/2d/sprite\_animation.rs](../../src/sprite_animation/sprite_animation.rs.html#12)
*   [examples/window/persisting\_window\_settings.rs](../../src/persisting_window_settings/persisting_window_settings.rs.html#32)
*   [examples/stress\_tests/text\_pipeline.rs](../../src/text_pipeline/text_pipeline.rs.html#29)
*   [examples/gizmos/2d\_text\_gizmos.rs](../../src/2d_text_gizmos/2d_text_gizmos.rs.html#31)
*   [examples/animation/animation\_masks.rs](../../src/animation_masks/animation_masks.rs.html#104)
*   [examples/3d/atmosphere.rs](../../src/atmosphere/atmosphere.rs.html#53-56)
*   [examples/ui/styling/box\_shadow.rs](../../src/box_shadow/box_shadow.rs.html#123)
*   [examples/window/multi\_window\_text.rs](../../src/multi_window_text/multi_window_text.rs.html#23)
*   [examples/shader\_advanced/custom\_phase\_item.rs](../../src/custom_phase_item/custom_phase_item.rs.html#168)
*   [examples/picking/custom\_hit\_data.rs](../../src/custom_hit_data/custom_hit_data.rs.html#37)
*   [examples/3d/specular\_tint.rs](../../src/specular_tint/specular_tint.rs.html#67)
*   [examples/ui/window\_fallthrough.rs](../../src/window_fallthrough/window_fallthrough.rs.html#20)
*   [tests/window/resizing.rs](../../src/resizing/resizing.rs.html#38)
*   [examples/shader/compute\_shader\_game\_of\_life.rs](../../src/compute_shader_game_of_life/compute_shader_game_of_life.rs.html#49)
*   [examples/ui/text/text\_wrap\_debug.rs](../../src/text_wrap_debug/text_wrap_debug.rs.html#40)
*   [examples/ecs/extraction.rs](../../src/extraction/extraction.rs.html#57)
*   [examples/shader\_advanced/render\_depth\_to\_texture.rs](../../src/render_depth_to_texture/render_depth_to_texture.rs.html#108)
*   [examples/stress\_tests/many\_lights.rs](../../src/many_lights/many_lights.rs.html#36)
*   [examples/showcase/breakout.rs](../../src/breakout/breakout.rs.html#63)
*   [examples/camera/2d\_screen\_shake.rs](../../src/2d_screen_shake/2d_screen_shake.rs.html#56)
*   [examples/gltf/custom\_gltf\_vertex\_attribute.rs](../../src/custom_gltf_vertex_attribute/custom_gltf_vertex_attribute.rs.html#42)
*   [examples/3d/pccm.rs](../../src/pccm/pccm.rs.html#67)
*   [examples/3d/reflection\_probes.rs](../../src/reflection_probes/reflection_probes.rs.html#76)
*   [examples/3d/visibility\_range.rs](../../src/visibility_range/visibility_range.rs.html#97)
*   [examples/stress\_tests/many\_gizmos.rs](../../src/many_gizmos/many_gizmos.rs.html#34)
*   [examples/showcase/alien\_cake\_addict.rs](../../src/alien_cake_addict/alien_cake_addict.rs.html#29)
*   [examples/ecs/fallible\_params.rs](../../src/fallible_params/fallible_params.rs.html#39)
*   [examples/animation/animation\_graph.rs](../../src/animation_graph/animation_graph.rs.html#84)
*   [examples/ui/widgets/standard\_widgets.rs](../../src/standard_widgets/standard_widgets.rs.html#34)
*   [examples/math/bounding\_2d.rs](../../src/bounding_2d/bounding_2d.rs.html#13)
*   [examples/asset/asset\_saving\_with\_subassets.rs](../../src/asset_saving_with_subassets/asset_saving_with_subassets.rs.html#28-38)
*   [examples/window/transparent\_window.rs](../../src/transparent_window/transparent_window.rs.html#29)
*   [examples/stress\_tests/many\_glyphs.rs](../../src/many_glyphs/many_glyphs.rs.html#55)
*   [examples/stress\_tests/many\_animated\_sprite\_meshes.rs](../../src/many_animated_sprite_meshes/many_animated_sprite_meshes.rs.html#37)
*   [examples/stress\_tests/many\_animated\_sprites.rs](../../src/many_animated_sprites/many_animated_sprites.rs.html#35)
*   [examples/3d/contact\_shadows.rs](../../src/contact_shadows/contact_shadows.rs.html#96)
*   [examples/gltf/gltf\_extension\_mesh\_2d.rs](../../src/gltf_extension_mesh_2d/gltf_extension_mesh_2d.rs.html#52)
*   [examples/stress\_tests/many\_materials.rs](../../src/many_materials/many_materials.rs.html#42)
*   [examples/stress\_tests/many\_sprite\_meshes.rs](../../src/many_sprite_meshes/many_sprite_meshes.rs.html#48)
*   [examples/stress\_tests/many\_sprites.rs](../../src/many_sprites/many_sprites.rs.html#46)
*   [examples/stress\_tests/many\_text2d.rs](../../src/many_text2d/many_text2d.rs.html#87)
*   [examples/ecs/system\_piping.rs](../../src/system_piping/system_piping.rs.html#18-36)
*   [examples/3d/ssr.rs](../../src/ssr/ssr.rs.html#220)
*   [examples/picking/debug\_picking.rs](../../src/debug_picking/debug_picking.rs.html#14)
*   [examples/state/custom\_transitions.rs](../../src/custom_transitions/custom_transitions.rs.html#35)
*   [examples/usage/debug\_frustum\_culling.rs](../../src/debug_frustum_culling/debug_frustum_culling.rs.html#55)
*   [examples/camera/projection\_zoom.rs](../../src/projection_zoom/projection_zoom.rs.html#37)
*   [examples/time/virtual\_time.rs](../../src/virtual_time/virtual_time.rs.html#14)
*   [examples/asset/multi\_asset\_sync.rs](../../src/multi_asset_sync/multi_asset_sync.rs.html#25)
*   [examples/3d/solari.rs](../../src/solari/solari.rs.html#59)
*   [tests/3d/test\_skinned\_mesh\_bounds.rs](../../src/test_skinned_mesh_bounds/test_skinned_mesh_bounds.rs.html#42)
*   [examples/3d/occlusion\_culling.rs](../../src/occlusion_culling/occlusion_culling.rs.html#188)
*   [examples/stress\_tests/many\_cubes.rs](../../src/many_cubes/many_cubes.rs.html#140)
*   [examples/stress\_tests/many\_gradients.rs](../../src/many_gradients/many_gradients.rs.html#74)
*   [tests/window/desktop\_request\_redraw.rs](../../src/desktop_request_redraw/desktop_request_redraw.rs.html#31)
*   [examples/3d/pcss.rs](../../src/pcss/pcss.rs.html#131)
*   [examples/3d/mixed\_lighting.rs](../../src/mixed_lighting/mixed_lighting.rs.html#135)
*   [examples/app/headless.rs](../../src/headless/headless.rs.html#29)
*   [examples/3d/clustered\_decals.rs](../../src/clustered_decals/clustered_decals.rs.html#135)
*   [examples/3d/clustered\_decal\_maps.rs](../../src/clustered_decal_maps/clustered_decal_maps.rs.html#148)
*   [examples/window/low\_power.rs](../../src/low_power/low_power.rs.html#33)
*   [examples/3d/mirror.rs](../../src/mirror/mirror.rs.html#125)
*   [examples/showcase/stepping.rs](../../src/breakout/stepping.rs.html#35)
*   [examples/state/sub\_states.rs](../../src/sub_states/sub_states.rs.html#41)
*   [examples/stress\_tests/many\_morph\_targets.rs](../../src/many_morph_targets/many_morph_targets.rs.html#164)
*   [examples/showcase/desk\_toy.rs](../../src/desk_toy/desk_toy.rs.html#34)
*   [examples/math/custom\_primitives.rs](../../src/custom_primitives/custom_primitives.rs.html#140)
*   [examples/3d/tonemapping.rs](../../src/tonemapping/tonemapping.rs.html#39-47)
*   [examples/testbed/2d.rs](../../src/testbed_2d/2d.rs.html#27)
*   [examples/ui/widgets/standard\_widgets\_observers.rs](../../src/standard_widgets_observers/standard_widgets_observers.rs.html#24)
*   [examples/stress\_tests/transform\_hierarchy.rs](../../src/transform_hierarchy/transform_hierarchy.rs.html#199)
*   [examples/2d/wireframe\_2d.rs](../../src/wireframe_2d/wireframe_2d.rs.html#45)
*   [examples/state/states.rs](../../src/states/states.rs.html#14)
*   [examples/3d/wireframe.rs](../../src/wireframe/wireframe.rs.html#47)
*   [examples/stress\_tests/many\_foxes.rs](../../src/many_foxes/many_foxes.rs.html#71)
*   [examples/app/externally\_driven\_headless\_renderer.rs](../../src/externally_driven_headless_renderer/externally_driven_headless_renderer.rs.html#62)
*   [examples/asset/processing/asset\_processing.rs](../../src/asset_processing/asset_processing.rs.html#41)
*   [examples/dev\_tools/fps\_overlay.rs](../../src/fps_overlay/fps_overlay.rs.html#46)
*   [examples/stress\_tests/bevymark\_3d.rs](../../src/bevymark_3d/bevymark_3d.rs.html#123)
*   [examples/3d/light\_textures.rs](../../src/light_textures/light_textures.rs.html#119)
*   [examples/stress\_tests/bevymark.rs](../../src/bevymark/bevymark.rs.html#155)
*   [examples/math/render\_primitives.rs](../../src/render_primitives/render_primitives.rs.html#17)
*   [examples/ecs/message.rs](../../src/message/message.rs.html#144-153)
*   [examples/diagnostics/log\_diagnostics.rs](../../src/log_diagnostics/log_diagnostics.rs.html#45)
*   [examples/ui/navigation/directional\_navigation.rs](../../src/directional_navigation/directional_navigation.rs.html#51)
*   [examples/ecs/system\_closure.rs](../../src/system_closure/system_closure.rs.html#30)
*   [examples/testbed/3d.rs](../../src/testbed_3d/3d.rs.html#26)
*   [examples/app/headless\_renderer.rs](../../src/headless_renderer/headless_renderer.rs.html#110)
*   [examples/3d/light\_probe\_blending.rs](../../src/light_probe_blending/light_probe_blending.rs.html#164)
*   [examples/2d/dynamic\_mip\_generation.rs](../../src/dynamic_mip_generation/dynamic_mip_generation.rs.html#224)
*   [examples/ecs/observers.rs](../../src/observers/observers.rs.html#13)
*   [examples/window/window\_settings.rs](../../src/window_settings/window_settings.rs.html#44)
*   [examples/stress\_tests/many\_buttons.rs](../../src/many_buttons/many_buttons.rs.html#88)
*   [examples/ecs/error\_handling.rs](../../src/error_handling/error_handling.rs.html#32)
*   [examples/ui/navigation/directional\_navigation\_overrides.rs](../../src/directional_navigation_overrides/directional_navigation_overrides.rs.html#62)
*   [examples/testbed/ui.rs](../../src/testbed_ui/ui.rs.html#36)
*   [examples/3d/irradiance\_volumes.rs](../../src/irradiance_volumes/irradiance_volumes.rs.html#165)
*   [examples/ecs/nondeterministic\_system\_order.rs](../../src/nondeterministic_system_order/nondeterministic_system_order.rs.html#33-55)
*   [examples/state/computed\_states.rs](../../src/computed_states/computed_states.rs.html#185)
*   [examples/ecs/custom\_schedule.rs](../../src/custom_schedule/custom_schedule.rs.html#48)
*   [examples/movement/physics\_in\_fixed\_timestep.rs](../../src/physics_in_fixed_timestep/physics_in_fixed_timestep.rs.html#104)
*   [examples/ecs/run\_conditions.rs](../../src/run_conditions/run_conditions.rs.html#14-51)
*   [examples/ecs/ecs\_guide.rs](../../src/ecs_guide/ecs_guide.rs.html#305)
*   [examples/ecs/system\_stepping.rs](../../src/system_stepping/system_stepping.rs.html#13-22)

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/app.rs.html#360-365)

#### pub fn [remove\_systems\_in\_set](#method.remove_systems_in_set)<M>( &mut self, schedule: impl [ScheduleLabel](../ecs/schedule/trait.ScheduleLabel.html "trait bevy::ecs::schedule::ScheduleLabel"), set: impl [IntoSystemSet](../prelude/trait.IntoSystemSet.html "trait bevy::prelude::IntoSystemSet")<M>, policy: [ScheduleCleanupPolicy](../ecs/schedule/enum.ScheduleCleanupPolicy.html "enum bevy::ecs::schedule::ScheduleCleanupPolicy"), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), [ScheduleError](../ecs/schedule/enum.ScheduleError.html "enum bevy::ecs::schedule::ScheduleError")\>

Removes all systems in a [`SystemSet`](../prelude/trait.SystemSet.html "trait bevy::prelude::SystemSet"). This will cause the schedule to be rebuilt when the schedule is run again and can be slow. A [`ScheduleError`](../ecs/schedule/enum.ScheduleError.html "enum bevy::ecs::schedule::ScheduleError") is returned if the schedule needs to be [`Schedule::initialize`](../prelude/struct.Schedule.html#method.initialize "method bevy::prelude::Schedule::initialize")’d or the `set` is not found.

Note that this can remove all systems of a type if you pass the system to this function as systems implicitly create a set based on the system type.

###### Example

```rust
// add the system
app.add_systems(Update, system_a);

// remove the system
app.remove_systems_in_set(Update, system_a, ScheduleCleanupPolicy::RemoveSystemsOnly);
```

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/app.rs.html#378-384)

#### pub fn [register\_system](#method.register_system)<I, O, M>( &mut self, system: impl [IntoSystem](../prelude/trait.IntoSystem.html "trait bevy::prelude::IntoSystem")<I, O, M> + 'static, ) -> [SystemId](../ecs/system/struct.SystemId.html "struct bevy::ecs::system::SystemId")<I, O>

where I: [SystemInput](../prelude/trait.SystemInput.html "trait bevy::prelude::SystemInput") + 'static, O: 'static,

Registers a system and returns a [`SystemId`](../ecs/system/struct.SystemId.html "struct bevy::ecs::system::SystemId") so it can later be called by [`World::run_system`](../prelude/struct.World.html#method.run_system "method bevy::prelude::World::run_system").

It’s possible to register the same systems more than once, they’ll be stored separately.

This is different from adding systems to a [`Schedule`](../prelude/struct.Schedule.html "struct bevy::prelude::Schedule") with [`App::add_systems`](../prelude/struct.App.html#method.add_systems "method bevy::prelude::App::add_systems"), because the [`SystemId`](../ecs/system/struct.SystemId.html "struct bevy::ecs::system::SystemId") that is returned can be used anywhere in the [`World`](../prelude/struct.World.html "struct bevy::prelude::World") to run the associated system. This allows for running systems in a push-based fashion. Using a [`Schedule`](../prelude/struct.Schedule.html "struct bevy::prelude::Schedule") is still preferred for most cases due to its better performance and ability to run non-conflicting systems simultaneously.

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/app.rs.html#396-402)

#### pub fn [register\_tracked\_system](#method.register_tracked_system)<I, O, M>( &mut self, system: impl [IntoSystem](../prelude/trait.IntoSystem.html "trait bevy::prelude::IntoSystem")<I, O, M> + 'static, ) -> [SystemHandle](../ecs/system/enum.SystemHandle.html "enum bevy::ecs::system::SystemHandle")<I, O>

where I: [SystemInput](../prelude/trait.SystemInput.html "trait bevy::prelude::SystemInput") + 'static, O: 'static,

Registers a system and returns a tracked [`SystemHandle`](../ecs/system/enum.SystemHandle.html "enum bevy::ecs::system::SystemHandle") so it can later be called by [`World::run_system`](../prelude/struct.World.html#method.run_system "method bevy::prelude::World::run_system"). The system entity will be automatically queued for despawn when the last clone of the returned handle is dropped.

See [`World::register_tracked_system`](../prelude/struct.World.html#method.register_tracked_system "method bevy::prelude::World::register_tracked_system") for more details.

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/app.rs.html#409-413)

#### pub fn [configure\_sets](#method.configure_sets)<M>( &mut self, schedule: impl [ScheduleLabel](../ecs/schedule/trait.ScheduleLabel.html "trait bevy::ecs::schedule::ScheduleLabel"), sets: impl [IntoScheduleConfigs](../prelude/trait.IntoScheduleConfigs.html "trait bevy::prelude::IntoScheduleConfigs")<[Interned](../ecs/intern/struct.Interned.html "struct bevy::ecs::intern::Interned")<dyn [SystemSet](../prelude/trait.SystemSet.html "trait bevy::prelude::SystemSet")\>, M>, ) -> &mut [App](../prelude/struct.App.html "struct bevy::prelude::App")

Configures a collection of system sets in the provided schedule, adding any sets that do not exist.

##### [Examples found in repository](#scraped-examples-7)[?](../../scrape-examples-help.html)

examples/ecs/ecs\_guide.rs ([lines 330-339](../../src/ecs_guide/ecs_guide.rs.html#330-339))

```rust
293fn main() {
294    // Bevy apps are created using the builder pattern. We use the builder to add systems,
295    // resources, and plugins to our app
296    App::new()
297        // Resources that implement the Default or FromWorld trait can be added like this:
298        .init_resource::<GameState>()
299        // Plugins are just a grouped set of app builder calls (just like we're doing here).
300        // We could easily turn our game into a plugin, but you can check out the plugin example for
301        // that :) The plugin below runs our app's "system schedule" once every 5 seconds.
302        .add_plugins(ScheduleRunnerPlugin::run_loop(Duration::from_secs(5)))
303        // `Startup` systems run exactly once BEFORE all other systems. These are generally used for
304        // app initialization code (ex: adding entities and resources)
305        .add_systems(Startup, startup_system)
306        // `Update` systems run once every update. These are generally used for "real-time app logic"
307        .add_systems(Update, print_message_system)
308        // SYSTEM EXECUTION ORDER
309        //
310        // Each system belongs to a `Schedule`, which controls the execution strategy and broad order
311        // of the systems within each tick. The `Startup` schedule holds
312        // startup systems, which are run a single time before `Update` runs. `Update` runs once per app update,
313        // which is generally one "frame" or one "tick".
314        //
315        // By default, all systems in a `Schedule` run in parallel, except when they require mutable access to a
316        // piece of data. This is efficient, but sometimes order matters.
317        // For example, we want our "game over" system to execute after all other systems to ensure
318        // we don't accidentally run the game for an extra round.
319        //
320        // You can force an explicit ordering between systems using the `.before` or `.after` methods.
321        // Systems will not be scheduled until all of the systems that they have an "ordering dependency" on have
322        // completed.
323        // There are other schedules, such as `Last` which runs at the very end of each run.
324        .add_systems(Last, print_at_end_round)
325        // We can also create new system sets, and order them relative to other system sets.
326        // Here is what our games execution order will look like:
327        // "before_round": new_player_system, new_round_system
328        // "round": print_message_system, score_system
329        // "after_round": score_check_system, game_over_system
330        .configure_sets(
331            Update,
332            // chain() will ensure sets run in the order they are listed
333            (
334                MySystems::BeforeRound,
335                MySystems::Round,
336                MySystems::AfterRound,
337            )
338                .chain(),
339        )
340        // The add_systems function is powerful. You can define complex system configurations with ease!
341        .add_systems(
342            Update,
343            (
344                // These `BeforeRound` systems will run before `Round` systems, thanks to the chained set configuration
345                (
346                    // You can also chain systems! new_round_system will run first, followed by new_player_system
347                    (new_round_system, new_player_system).chain(),
348                    exclusive_player_system,
349                )
350                    // All of the systems in the tuple above will be added to this set
351                    .in_set(MySystems::BeforeRound),
352                // This `Round` system will run after the `BeforeRound` systems thanks to the chained set configuration
353                score_system.in_set(MySystems::Round),
354                // These `AfterRound` systems will run after the `Round` systems thanks to the chained set configuration
355                (
356                    score_check_system,
357                    // In addition to chain(), you can also use `before(system)` and `after(system)`. This also works
358                    // with sets!
359                    game_over_system.after(score_check_system),
360                )
361                    .in_set(MySystems::AfterRound),
362            ),
363        )
364        // This call to run() starts the app we just built!
365        .run();
366}
```

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/app.rs.html#435)

#### pub fn [add\_message](#method.add_message)<M>(&mut self) -> &mut [App](../prelude/struct.App.html "struct bevy::prelude::App")

where M: [Message](../prelude/trait.Message.html "trait bevy::prelude::Message"),

Initializes [`Message`](../prelude/trait.Message.html "trait bevy::prelude::Message") handling for `T` by inserting a message queue resource ([`Messages::<T>`](../prelude/struct.Messages.html "struct bevy::prelude::Messages")) and scheduling an [`message_update_system`](../ecs/message/fn.message_update_system.html "fn bevy::ecs::message::message_update_system") in [`First`](../prelude/struct.First.html "struct bevy::prelude::First").

See [`Messages`](../prelude/struct.Messages.html "struct bevy::prelude::Messages") for information on how to define messages.

##### Examples

```rust
app.add_message::<MyMessage>();
```

##### [Examples found in repository](#scraped-examples-8)[?](../../scrape-examples-help.html)

examples/audio/pitch.rs ([line 9](../../src/pitch/pitch.rs.html#9))

```rust
6fn main() {
7    App::new()
8        .add_plugins(DefaultPlugins)
9        .add_message::<PlayPitch>()
10        .add_systems(Startup, setup)
11        .add_systems(Update, (play_pitch, keyboard_input_system))
12        .run();
13}
```

Hide additional examples

examples/ui/layout/size\_constraints.rs ([line 8](../../src/size_constraints/size_constraints.rs.html#8))

```rust
5fn main() {
6    App::new()
7        .add_plugins(DefaultPlugins)
8        .add_message::<ButtonActivated>()
9        .add_systems(Startup, setup)
10        .add_systems(Update, (update_buttons, update_radio_buttons_colors))
11        .run();
12}
```

examples/ecs/component\_hooks.rs ([line 57](../../src/component_hooks/component_hooks.rs.html#57))

```rust
51fn main() {
52    App::new()
53        .add_plugins(DefaultPlugins)
54        .add_systems(Startup, setup)
55        .add_systems(Update, trigger_hooks)
56        .init_resource::<MyComponentIndex>()
57        .add_message::<MyMessage>()
58        .run();
59}
```

examples/async\_tasks/external\_source\_external\_thread.rs ([line 11](../../src/external_source_external_thread/external_source_external_thread.rs.html#11))

```rust
9fn main() {
10    App::new()
11        .add_message::<StreamMessage>()
12        .add_plugins(DefaultPlugins)
13        .add_systems(Startup, setup)
14        .add_systems(Update, (spawn_text, move_text))
15        .add_systems(FixedUpdate, read_stream)
16        .insert_resource(Time::<Fixed>::from_seconds(0.5))
17        .run();
18}
```

examples/app/log\_layers\_ecs.rs ([line 108](../../src/log_layers_ecs/log_layers_ecs.rs.html#108))

```rust
101fn custom_layer(app: &mut App) -> Option<BoxedLayer> {
102    let (sender, receiver) = mpsc::channel();
103
104    let layer = CaptureLayer { sender };
105    let resource = CapturedLogMessages(receiver);
106
107    app.insert_non_send(resource);
108    app.add_message::<LogMessage>();
109    app.add_systems(Update, transfer_log_messages);
110
111    Some(layer.boxed())
112}
```

examples/ecs/extraction.rs ([line 56](../../src/extraction/extraction.rs.html#56))

```rust
46fn main() {
47    let mut app = App::new();
48
49    // Main World
50    app.insert_resource(WorldName("Main World".into()))
51        .add_plugins((
52            DefaultPlugins,
53            // Plugin for automatically extracting A.
54            ExtractComponentPlugin::<A>::default(),
55        ))
56        .add_message::<ExtractMessage>()
57        .add_systems(Startup, setup)
58        .add_systems(Update, (set_time, trigger_extraction, display_state));
59
60    let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
61        return;
62    };
63
64    // Render World
65    render_app
66        .insert_resource(WorldName("Render World".into()))
67        .add_systems(ExtractSchedule, extract_components)
68        .add_systems(Render, display_state);
69
70    app.run();
71}
```

Additional examples can be found in:  

*   [examples/3d/pccm.rs](../../src/pccm/pccm.rs.html#66)
*   [examples/3d/contact\_shadows.rs](../../src/contact_shadows/contact_shadows.rs.html#95)
*   [examples/3d/ssr.rs](../../src/ssr/ssr.rs.html#219)
*   [examples/3d/pcss.rs](../../src/pcss/pcss.rs.html#130)
*   [examples/3d/mixed\_lighting.rs](../../src/mixed_lighting/mixed_lighting.rs.html#133)
*   [examples/3d/clustered\_decals.rs](../../src/clustered_decals/clustered_decals.rs.html#134)
*   [examples/3d/clustered\_decal\_maps.rs](../../src/clustered_decal_maps/clustered_decal_maps.rs.html#145)
*   [examples/3d/mirror.rs](../../src/mirror/mirror.rs.html#124)
*   [examples/3d/light\_textures.rs](../../src/light_textures/light_textures.rs.html#117)
*   [examples/ecs/message.rs](../../src/message/message.rs.html#136)
*   [examples/3d/light\_probe\_blending.rs](../../src/light_probe_blending/light_probe_blending.rs.html#161)
*   [examples/2d/dynamic\_mip\_generation.rs](../../src/dynamic_mip_generation/dynamic_mip_generation.rs.html#222)

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/app.rs.html#459)

#### pub fn [insert\_resource](#method.insert_resource)<R>(&mut self, resource: R) -> &mut [App](../prelude/struct.App.html "struct bevy::prelude::App")

where R: [Resource](../prelude/trait.Resource.html "trait bevy::prelude::Resource"),

Inserts the [`Resource`](../prelude/trait.Resource.html "trait bevy::prelude::Resource") into the app, overwriting any existing resource of the same type.

There is also an [`init_resource`](../prelude/struct.App.html#method.init_resource "method bevy::prelude::App::init_resource") for resources that have [`Default`](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") or [`FromWorld`](../prelude/trait.FromWorld.html "trait bevy::prelude::FromWorld") implementations.

##### Examples

```rust
#[derive(Resource)]
struct MyCounter {
    counter: usize,
}

App::new()
   .insert_resource(MyCounter { counter: 0 });
```

##### [Examples found in repository](#scraped-examples-9)[?](../../scrape-examples-help.html)

examples/ui/images/ui\_texture\_slice\_flip\_and\_tile.rs ([line 12](../../src/ui_texture_slice_flip_and_tile/ui_texture_slice_flip_and_tile.rs.html#12))

```rust
9fn main() {
10    App::new()
11        .add_plugins(DefaultPlugins)
12        .insert_resource(UiScale(2.))
13        .add_systems(Startup, setup)
14        .run();
15}
```

Hide additional examples

examples/ecs/system\_param.rs ([line 7](../../src/system_param/system_param.rs.html#7))

```rust
5fn main() {
6    App::new()
7        .insert_resource(PlayerCount(0))
8        .add_systems(Startup, spawn)
9        .add_systems(Update, count_players)
10        .run();
11}
```

examples/ui/layout/z\_index.rs ([line 13](../../src/z_index/z_index.rs.html#13))

```rust
11fn main() {
12    App::new()
13        .insert_resource(ClearColor(Color::BLACK))
14        .add_plugins(DefaultPlugins)
15        .add_systems(Startup, setup)
16        .run();
17}
```

examples/ui/styling/transparency\_ui.rs ([line 8](../../src/transparency_ui/transparency_ui.rs.html#8))

```rust
6fn main() {
7    App::new()
8        .insert_resource(ClearColor(Color::BLACK))
9        .add_plugins(DefaultPlugins)
10        .add_systems(Startup, setup)
11        .run();
12}
```

examples/app/custom\_loop.rs ([line 45](../../src/custom_loop/custom_loop.rs.html#45))

```rust
43fn main() -> AppExit {
44    App::new()
45        .insert_resource(Input(String::new()))
46        .set_runner(my_runner)
47        .add_systems(Update, (print_system, exit_system))
48        .run()
49}
```

examples/ui/widgets/virtual\_keyboard.rs ([line 17](../../src/virtual_keyboard/virtual_keyboard.rs.html#17))

```rust
14fn main() {
15    App::new()
16        .add_plugins((DefaultPlugins, FeathersPlugins))
17        .insert_resource(UiTheme(create_dark_theme()))
18        .add_systems(Startup, scene.spawn())
19        .run();
20}
```

Additional examples can be found in:  

*   [examples/3d/mesh\_ray\_cast.rs](../../src/mesh_ray_cast/mesh_ray_cast.rs.html#16)
*   [examples/2d/tilemap\_chunk\_orientation.rs](../../src/tilemap_chunk_orientation/tilemap_chunk_orientation.rs.html#12)
*   [examples/3d/spherical\_area\_lights.rs](../../src/spherical_area_lights/spherical_area_lights.rs.html#7-10)
*   [examples/window/clear\_color.rs](../../src/clear_color/clear_color.rs.html#9)
*   [examples/gltf/update\_gltf\_scene.rs](../../src/update_gltf_scene/update_gltf_scene.rs.html#8)
*   [examples/shader/shader\_material\_wesl.rs](../../src/shader_material_wesl/shader_material_wesl.rs.html#46)
*   [examples/gltf/load\_gltf.rs](../../src/load_gltf/load_gltf.rs.html#11)
*   [examples/ui/scroll\_and\_overflow/scrollbars.rs](../../src/scrollbars/scrollbars.rs.html#14)
*   [examples/animation/animated\_transform.rs](../../src/animated_transform/animated_transform.rs.html#13-17)
*   [examples/3d/ssao.rs](../../src/ssao/ssao.rs.html#15-18)
*   [examples/animation/morph\_targets.rs](../../src/morph_targets/morph_targets.rs.html#13-16)
*   [examples/ui/text/font\_atlas\_debug.rs](../../src/font_atlas_debug/font_atlas_debug.rs.html#12)
*   [examples/gltf/gltf\_skinned\_mesh.rs](../../src/gltf_skinned_mesh/gltf_skinned_mesh.rs.html#11-14)
*   [examples/animation/custom\_skinned\_mesh.rs](../../src/custom_skinned_mesh/custom_skinned_mesh.rs.html#22-25)
*   [examples/ecs/iter\_combinations.rs](../../src/iter_combinations/iter_combinations.rs.html#10)
*   [examples/3d/spotlight.rs](../../src/spotlight/spotlight.rs.html#24-27)
*   [examples/app/plugin.rs](../../src/plugin/plugin.rs.html#37)
*   [examples/3d/fog.rs](../../src/fog/fog.rs.html#32)
*   [examples/shader\_advanced/compute\_mesh.rs](../../src/compute_mesh/compute_mesh.rs.html#43)
*   [tests/3d/test\_invalid\_skinned\_mesh.rs](../../src/test_invalid_skinned_mesh/test_invalid_skinned_mesh.rs.html#19-22)
*   [examples/async\_tasks/external\_source\_external\_thread.rs](../../src/external_source_external_thread/external_source_external_thread.rs.html#16)
*   [examples/animation/animated\_mesh.rs](../../src/animated_mesh/animated_mesh.rs.html#14-18)
*   [examples/ui/images/image\_node\_resizing.rs](../../src/image_node_resizing/image_node_resizing.rs.html#21-24)
*   [examples/shader/gpu\_readback.rs](../../src/gpu_readback/gpu_readback.rs.html#36)
*   [examples/2d/rotation.rs](../../src/rotation/rotation.rs.html#10)
*   [examples/3d/deferred\_rendering.rs](../../src/deferred_rendering/deferred_rendering.rs.html#20)
*   [examples/window/window\_resizing.rs](../../src/window_resizing/window_resizing.rs.html#6-10)
*   [examples/dev\_tools/schedule\_data.rs](../../src/schedule_data/schedule_data.rs.html#10-12)
*   [examples/3d/fog\_volumes.rs](../../src/fog_volumes/fog_volumes.rs.html#25)
*   [examples/3d/transmission.rs](../../src/transmission/transmission.rs.html#49)
*   [examples/time/time.rs](../../src/time/time.rs.html#114)
*   [examples/3d/scrolling\_fog.rs](../../src/scrolling_fog/scrolling_fog.rs.html#34)
*   [examples/3d/meshlet.rs](../../src/meshlet/meshlet.rs.html#19)
*   [examples/ui/ui\_scaling.rs](../../src/ui_scaling/ui_scaling.rs.html#12-16)
*   [examples/window/window\_drag\_move.rs](../../src/window_drag_move/window_drag_move.rs.html#51)
*   [examples/ecs/fixed\_timestep.rs](../../src/fixed_timestep/fixed_timestep.rs.html#13)
*   [examples/ecs/hotpatching\_systems.rs](../../src/hotpatching_systems/hotpatching_systems.rs.html#30)
*   [examples/animation/animated\_mesh\_events.rs](../../src/animated_mesh_events/animated_mesh_events.rs.html#18-22)
*   [examples/showcase/loading\_screen.rs](../../src/loading_screen/loading_screen.rs.html#23)
*   [examples/stress\_tests/many\_cameras\_lights.rs](../../src/many_cameras_lights/many_cameras_lights.rs.html#23)
*   [examples/ui/widgets/feathers\_gallery.rs](../../src/feathers_gallery/feathers_gallery.rs.html#67)
*   [examples/animation/animated\_mesh\_control.rs](../../src/animated_mesh_control/animated_mesh_control.rs.html#14-18)
*   [examples/gltf/gltf\_extension\_animation\_graph.rs](../../src/gltf_extension_animation_graph/gltf_extension_animation_graph.rs.html#23-27)
*   [examples/showcase/game\_menu.rs](../../src/game_menu/game_menu.rs.html#37)
*   [examples/3d/volumetric\_fog.rs](../../src/volumetric_fog/volumetric_fog.rs.html#45-50)
*   [examples/2d/cpu\_draw.rs](../../src/cpu_draw/cpu_draw.rs.html#25)
*   [examples/3d/lightmaps.rs](../../src/lightmaps/lightmaps.rs.html#30)
*   [examples/ui/widgets/feathers\_counter.rs](../../src/feathers_counter/feathers_counter.rs.html#31)
*   [examples/3d/lighting.rs](../../src/lighting/lighting.rs.html#16-21)
*   [examples/ecs/state\_scoped.rs](../../src/state_scoped/state_scoped.rs.html#25)
*   [examples/remote/app\_under\_test.rs](../../src/app_under_test/app_under_test.rs.html#28)
*   [examples/stress\_tests/text\_pipeline.rs](../../src/text_pipeline/text_pipeline.rs.html#28)
*   [examples/gizmos/2d\_text\_gizmos.rs](../../src/2d_text_gizmos/2d_text_gizmos.rs.html#18)
*   [examples/animation/animation\_masks.rs](../../src/animation_masks/animation_masks.rs.html#108-112)
*   [examples/3d/atmosphere.rs](../../src/atmosphere/atmosphere.rs.html#43)
*   [examples/ui/styling/box\_shadow.rs](../../src/box_shadow/box_shadow.rs.html#120)
*   [examples/picking/custom\_hit\_data.rs](../../src/custom_hit_data/custom_hit_data.rs.html#28-31)
*   [examples/3d/specular\_tint.rs](../../src/specular_tint/specular_tint.rs.html#62-66)
*   [examples/ui/window\_fallthrough.rs](../../src/window_fallthrough/window_fallthrough.rs.html#9)
*   [tests/window/resizing.rs](../../src/resizing/resizing.rs.html#33-36)
*   [examples/shader/compute\_shader\_game\_of\_life.rs](../../src/compute_shader_game_of_life/compute_shader_game_of_life.rs.html#34)
*   [examples/ui/text/text\_wrap\_debug.rs](../../src/text_wrap_debug/text_wrap_debug.rs.html#39)
*   [examples/ecs/extraction.rs](../../src/extraction/extraction.rs.html#50)
*   [examples/stress\_tests/many\_lights.rs](../../src/many_lights/many_lights.rs.html#35)
*   [examples/showcase/breakout.rs](../../src/breakout/breakout.rs.html#61)
*   [examples/gltf/custom\_gltf\_vertex\_attribute.rs](../../src/custom_gltf_vertex_attribute/custom_gltf_vertex_attribute.rs.html#26-30)
*   [examples/3d/visibility\_range.rs](../../src/visibility_range/visibility_range.rs.html#96)
*   [examples/stress\_tests/many\_gizmos.rs](../../src/many_gizmos/many_gizmos.rs.html#29)
*   [examples/showcase/alien\_cake\_addict.rs](../../src/alien_cake_addict/alien_cake_addict.rs.html#24-27)
*   [examples/animation/animation\_graph.rs](../../src/animation_graph/animation_graph.rs.html#90)
*   [examples/ui/widgets/standard\_widgets.rs](../../src/standard_widgets/standard_widgets.rs.html#30-33)
*   [examples/window/transparent\_window.rs](../../src/transparent_window/transparent_window.rs.html#28)
*   [examples/stress\_tests/many\_glyphs.rs](../../src/many_glyphs/many_glyphs.rs.html#54)
*   [examples/stress\_tests/many\_animated\_sprite\_meshes.rs](../../src/many_animated_sprite_meshes/many_animated_sprite_meshes.rs.html#36)
*   [examples/stress\_tests/many\_animated\_sprites.rs](../../src/many_animated_sprites/many_animated_sprites.rs.html#34)
*   [examples/3d/contact\_shadows.rs](../../src/contact_shadows/contact_shadows.rs.html#94)
*   [examples/gltf/gltf\_extension\_mesh\_2d.rs](../../src/gltf_extension_mesh_2d/gltf_extension_mesh_2d.rs.html#31-35)
*   [examples/stress\_tests/many\_materials.rs](../../src/many_materials/many_materials.rs.html#40)
*   [examples/stress\_tests/many\_sprite\_meshes.rs](../../src/many_sprite_meshes/many_sprite_meshes.rs.html#31-33)
*   [examples/stress\_tests/many\_sprites.rs](../../src/many_sprites/many_sprites.rs.html#29-31)
*   [examples/stress\_tests/many\_text2d.rs](../../src/many_text2d/many_text2d.rs.html#85)
*   [examples/ecs/system\_piping.rs](../../src/system_piping/system_piping.rs.html#11)
*   [examples/3d/ssr.rs](../../src/ssr/ssr.rs.html#209)
*   [examples/picking/debug\_picking.rs](../../src/debug_picking/debug_picking.rs.html#15)
*   [examples/camera/projection\_zoom.rs](../../src/projection_zoom/projection_zoom.rs.html#24-36)
*   [examples/asset/multi\_asset\_sync.rs](../../src/multi_asset_sync/multi_asset_sync.rs.html#20-24)
*   [examples/3d/solari.rs](../../src/solari/solari.rs.html#56)
*   [tests/3d/test\_skinned\_mesh\_bounds.rs](../../src/test_skinned_mesh_bounds/test_skinned_mesh_bounds.rs.html#38-41)
*   [examples/stress\_tests/many\_cubes.rs](../../src/many_cubes/many_cubes.rs.html#139)
*   [examples/stress\_tests/many\_gradients.rs](../../src/many_gradients/many_gradients.rs.html#72)
*   [tests/window/desktop\_request\_redraw.rs](../../src/desktop_request_redraw/desktop_request_redraw.rs.html#30)
*   [examples/3d/mixed\_lighting.rs](../../src/mixed_lighting/mixed_lighting.rs.html#127-131)
*   [examples/3d/clustered\_decal\_maps.rs](../../src/clustered_decal_maps/clustered_decal_maps.rs.html#163)
*   [examples/window/low\_power.rs](../../src/low_power/low_power.rs.html#16)
*   [examples/showcase/stepping.rs](../../src/breakout/stepping.rs.html#52)
*   [examples/stress\_tests/many\_morph\_targets.rs](../../src/many_morph_targets/many_morph_targets.rs.html#155)
*   [examples/showcase/desk\_toy.rs](../../src/desk_toy/desk_toy.rs.html#31)
*   [examples/3d/tonemapping.rs](../../src/tonemapping/tonemapping.rs.html#33-35)
*   [examples/ui/widgets/standard\_widgets\_observers.rs](../../src/standard_widgets_observers/standard_widgets_observers.rs.html#23)
*   [examples/stress\_tests/transform\_hierarchy.rs](../../src/transform_hierarchy/transform_hierarchy.rs.html#189)
*   [examples/2d/wireframe\_2d.rs](../../src/wireframe_2d/wireframe_2d.rs.html#36-44)
*   [examples/3d/wireframe.rs](../../src/wireframe/wireframe.rs.html#37-46)
*   [examples/stress\_tests/many\_foxes.rs](../../src/many_foxes/many_foxes.rs.html#62)
*   [examples/stress\_tests/bevymark\_3d.rs](../../src/bevymark_3d/bevymark_3d.rs.html#117)
*   [examples/stress\_tests/bevymark.rs](../../src/bevymark/bevymark.rs.html#148)
*   [examples/ui/navigation/directional\_navigation.rs](../../src/directional_navigation/directional_navigation.rs.html#40)
*   [examples/app/headless\_renderer.rs](../../src/headless_renderer/headless_renderer.rs.html#79-83)
*   [examples/stress\_tests/many\_buttons.rs](../../src/many_buttons/many_buttons.rs.html#87)
*   [examples/3d/occlusion\_culling.rs](../../src/occlusion_culling/occlusion_culling.rs.html#205)
*   [examples/ui/navigation/directional\_navigation\_overrides.rs](../../src/directional_navigation_overrides/directional_navigation_overrides.rs.html#47)
*   [examples/3d/irradiance\_volumes.rs](../../src/irradiance_volumes/irradiance_volumes.rs.html#160-164)
*   [examples/ecs/system\_stepping.rs](../../src/system_stepping/system_stepping.rs.html#46)

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/app.rs.html#493)

#### pub fn [init\_resource](#method.init_resource)<R>(&mut self) -> &mut [App](../prelude/struct.App.html "struct bevy::prelude::App")

where R: [Resource](../prelude/trait.Resource.html "trait bevy::prelude::Resource") + [FromWorld](../prelude/trait.FromWorld.html "trait bevy::prelude::FromWorld"),

Inserts the [`Resource`](../prelude/trait.Resource.html "trait bevy::prelude::Resource"), initialized with its default value, into the app, if there is no existing instance of `R`.

`R` must implement [`FromWorld`](../prelude/trait.FromWorld.html "trait bevy::prelude::FromWorld"). If `R` implements [`Default`](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default"), [`FromWorld`](../prelude/trait.FromWorld.html "trait bevy::prelude::FromWorld") will be automatically implemented and initialize the [`Resource`](../prelude/trait.Resource.html "trait bevy::prelude::Resource") with [`Default::default`](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default "associated function core::default::Default::default").

##### Examples

```rust
#[derive(Resource)]
struct MyCounter {
    counter: usize,
}

impl Default for MyCounter {
    fn default() -> MyCounter {
        MyCounter {
            counter: 100
        }
    }
}

App::new()
    .init_resource::<MyCounter>();
```

##### [Examples found in repository](#scraped-examples-10)[?](../../scrape-examples-help.html)

examples/audio/play\_sound\_effect.rs ([line 24](../../src/play_sound_effect/play_sound_effect.rs.html#24))

```rust
21fn main() {
22    App::new()
23        .add_plugins(DefaultPlugins)
24        .init_resource::<SoundEffect>()
25        .add_systems(Startup, setup)
26        .add_systems(Update, keyboard_event)
27        .run();
28}
```

Hide additional examples

examples/camera/camera\_orbit.rs ([line 39](../../src/camera_orbit/camera_orbit.rs.html#39))

```rust
36fn main() {
37    App::new()
38        .add_plugins(DefaultPlugins)
39        .init_resource::<CameraSettings>()
40        .add_systems(Startup, (setup, instructions))
41        .add_systems(Update, orbit)
42        .run();
43}
```

examples/time/timers.rs ([line 8](../../src/timers/timers.rs.html#8))

```rust
5fn main() {
6    App::new()
7        .add_plugins(DefaultPlugins)
8        .init_resource::<Countdown>()
9        .add_systems(Startup, setup)
10        .add_systems(Update, (countdown, print_when_completed))
11        .run();
12}
```

examples/ui/text/letter\_spacing.rs ([line 24](../../src/letter_spacing/letter_spacing.rs.html#24))

```rust
21fn main() {
22    App::new()
23        .add_plugins(DefaultPlugins)
24        .init_resource::<SpacingMode>()
25        .add_systems(Startup, setup)
26        .add_systems(Update, (update_letter_spacing, toggle_mode))
27        .run();
28}
```

examples/ecs/component\_hooks.rs ([line 56](../../src/component_hooks/component_hooks.rs.html#56))

```rust
51fn main() {
52    App::new()
53        .add_plugins(DefaultPlugins)
54        .add_systems(Startup, setup)
55        .add_systems(Update, trigger_hooks)
56        .init_resource::<MyComponentIndex>()
57        .add_message::<MyMessage>()
58        .run();
59}
```

examples/ui/text/font\_atlas\_debug.rs ([line 11](../../src/font_atlas_debug/font_atlas_debug.rs.html#11))

```rust
9fn main() {
10    App::new()
11        .init_resource::<State>()
12        .insert_resource(ClearColor(Color::BLACK))
13        .add_plugins(DefaultPlugins)
14        .add_systems(Startup, setup)
15        .add_systems(Update, (text_update_system, atlas_render_system))
16        .run();
17}
```

Additional examples can be found in:  

*   [examples/ui/widgets/button.rs](../../src/button/button.rs.html#14)
*   [examples/3d/clearcoat.rs](../../src/clearcoat/clearcoat.rs.html#53)
*   [examples/asset/asset\_saving.rs](../../src/asset_saving/asset_saving.rs.html#73)
*   [examples/app/render\_recovery.rs](../../src/render_recovery/render_recovery.rs.html#28)
*   [examples/asset/custom\_asset.rs](../../src/custom_asset/custom_asset.rs.html#95)
*   [examples/showcase/contributors.rs](../../src/contributors/contributors.rs.html#18)
*   [examples/3d/color\_grading.rs](../../src/color_grading/color_grading.rs.html#103)
*   [examples/3d/post\_processing.rs](../../src/post_processing/post_processing.rs.html#53)
*   [examples/animation/animated\_mesh\_events.rs](../../src/animated_mesh_events/animated_mesh_events.rs.html#24)
*   [examples/gltf/gltf\_extension\_animation\_graph.rs](../../src/gltf_extension_animation_graph/gltf_extension_animation_graph.rs.html#29)
*   [examples/3d/volumetric\_fog.rs](../../src/volumetric_fog/volumetric_fog.rs.html#52)
*   [examples/3d/depth\_of\_field.rs](../../src/depth_of_field/depth_of_field.rs.html#54)
*   [examples/ui/scroll\_and\_overflow/overflow\_debug.rs](../../src/overflow_debug/overflow_debug.rs.html#13)
*   [examples/3d/anisotropy.rs](../../src/anisotropy/anisotropy.rs.html#83)
*   [examples/animation/animation\_masks.rs](../../src/animation_masks/animation_masks.rs.html#113)
*   [examples/picking/custom\_hit\_data.rs](../../src/custom_hit_data/custom_hit_data.rs.html#36)
*   [examples/3d/specular\_tint.rs](../../src/specular_tint/specular_tint.rs.html#60)
*   [examples/shader\_advanced/render\_depth\_to\_texture.rs](../../src/render_depth_to_texture/render_depth_to_texture.rs.html#107)
*   [examples/3d/pccm.rs](../../src/pccm/pccm.rs.html#65)
*   [examples/3d/reflection\_probes.rs](../../src/reflection_probes/reflection_probes.rs.html#74)
*   [examples/3d/visibility\_range.rs](../../src/visibility_range/visibility_range.rs.html#95)
*   [examples/showcase/alien\_cake\_addict.rs](../../src/alien_cake_addict/alien_cake_addict.rs.html#23)
*   [examples/3d/contact\_shadows.rs](../../src/contact_shadows/contact_shadows.rs.html#93)
*   [examples/stress\_tests/many\_text2d.rs](../../src/many_text2d/many_text2d.rs.html#86)
*   [examples/3d/ssr.rs](../../src/ssr/ssr.rs.html#210)
*   [examples/shader\_advanced/manual\_material.rs](../../src/manual_material/manual_material.rs.html#58)
*   [examples/3d/occlusion\_culling.rs](../../src/occlusion_culling/occlusion_culling.rs.html#187)
*   [examples/3d/pcss.rs](../../src/pcss/pcss.rs.html#118)
*   [examples/3d/mixed\_lighting.rs](../../src/mixed_lighting/mixed_lighting.rs.html#132)
*   [examples/3d/clustered\_decals.rs](../../src/clustered_decals/clustered_decals.rs.html#133)
*   [examples/3d/clustered\_decal\_maps.rs](../../src/clustered_decal_maps/clustered_decal_maps.rs.html#146)
*   [examples/3d/mirror.rs](../../src/mirror/mirror.rs.html#123)
*   [examples/3d/tonemapping.rs](../../src/tonemapping/tonemapping.rs.html#36)
*   [examples/3d/light\_textures.rs](../../src/light_textures/light_textures.rs.html#116)
*   [examples/ecs/message.rs](../../src/message/message.rs.html#139)
*   [examples/ui/navigation/directional\_navigation.rs](../../src/directional_navigation/directional_navigation.rs.html#50)
*   [examples/app/headless\_renderer.rs](../../src/headless_renderer/headless_renderer.rs.html#109)
*   [examples/3d/light\_probe\_blending.rs](../../src/light_probe_blending/light_probe_blending.rs.html#160)
*   [examples/2d/dynamic\_mip\_generation.rs](../../src/dynamic_mip_generation/dynamic_mip_generation.rs.html#220)
*   [examples/ecs/observers.rs](../../src/observers/observers.rs.html#11)
*   [examples/ui/navigation/directional\_navigation\_overrides.rs](../../src/directional_navigation_overrides/directional_navigation_overrides.rs.html#57)
*   [examples/3d/irradiance\_volumes.rs](../../src/irradiance_volumes/irradiance_volumes.rs.html#158)
*   [examples/ecs/nondeterministic\_system\_order.rs](../../src/nondeterministic_system_order/nondeterministic_system_order.rs.html#31)
*   [examples/movement/physics\_in\_fixed\_timestep.rs](../../src/physics_in_fixed_timestep/physics_in_fixed_timestep.rs.html#103)
*   [examples/ecs/run\_conditions.rs](../../src/run_conditions/run_conditions.rs.html#13)
*   [examples/ecs/ecs\_guide.rs](../../src/ecs_guide/ecs_guide.rs.html#298)

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/app.rs.html#501)

#### pub fn [insert\_non\_send\_resource](#method.insert_non_send_resource)<R>(&mut self, resource: R) -> &mut [App](../prelude/struct.App.html "struct bevy::prelude::App")

where R: 'static,

👎Deprecated since 0.19.0:

use App::insert\_non\_send

Inserts the [`!Send`](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") resource into the app, overwriting any existing data of the same type.

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/app.rs.html#524)

#### pub fn [insert\_non\_send](#method.insert_non_send)<R>(&mut self, resource: R) -> &mut [App](../prelude/struct.App.html "struct bevy::prelude::App")

where R: 'static,

Inserts the [`!Send`](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") data into the app, overwriting any existing data of the same type.

There is also an [`init_non_send`](../prelude/struct.App.html#method.init_non_send "method bevy::prelude::App::init_non_send") for [`!Send`](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") data that implement [`Default`](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default")

##### Examples

```rust
struct MyCounter {
    counter: usize,
}

App::new()
    .insert_non_send(MyCounter { counter: 0 });
```

##### [Examples found in repository](#scraped-examples-11)[?](../../scrape-examples-help.html)

examples/app/log\_layers\_ecs.rs ([line 107](../../src/log_layers_ecs/log_layers_ecs.rs.html#107))

```rust
101fn custom_layer(app: &mut App) -> Option<BoxedLayer> {
102    let (sender, receiver) = mpsc::channel();
103
104    let layer = CaptureLayer { sender };
105    let resource = CapturedLogMessages(receiver);
106
107    app.insert_non_send(resource);
108    app.add_message::<LogMessage>();
109    app.add_systems(Update, transfer_log_messages);
110
111    Some(layer.boxed())
112}
```

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/app.rs.html#531)

#### pub fn [init\_non\_send\_resource](#method.init_non_send_resource)<R>(&mut self) -> &mut [App](../prelude/struct.App.html "struct bevy::prelude::App")

where R: 'static + [FromWorld](../prelude/trait.FromWorld.html "trait bevy::prelude::FromWorld"),

👎Deprecated since 0.19.0:

use App::init\_non\_send

Inserts the [`!Send`](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") resource into the app if there is no existing instance of `R`.

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/app.rs.html#540)

#### pub fn [init\_non\_send](#method.init_non_send)<R>(&mut self) -> &mut [App](../prelude/struct.App.html "struct bevy::prelude::App")

where R: 'static + [FromWorld](../prelude/trait.FromWorld.html "trait bevy::prelude::FromWorld"),

Inserts the [`!Send`](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") data into the app if there is no existing instance of `R`.

`R` must implement [`FromWorld`](../prelude/trait.FromWorld.html "trait bevy::prelude::FromWorld"). If `R` implements [`Default`](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default"), [`FromWorld`](../prelude/trait.FromWorld.html "trait bevy::prelude::FromWorld") will be automatically implemented and initialize the [`Resource`](../prelude/trait.Resource.html "trait bevy::prelude::Resource") with [`Default::default`](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default "associated function core::default::Default::default").

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/app.rs.html#591-593)

#### pub fn [is\_plugin\_added](#method.is_plugin_added)<T>(&self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

where T: [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin"),

Returns `true` if the [`Plugin`](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") has already been added.

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/app.rs.html#617-619)

#### pub fn [get\_added\_plugins](#method.get_added_plugins)<T>(&self) -> [Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)\>

where T: [Plugin](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin"),

Returns a vector of references to all plugins of type `T` that have been added.

This can be used to read the settings of any existing plugins. This vector will be empty if no plugins of that type have been added. If multiple copies of the same plugin are added to the [`App`](../prelude/struct.App.html "struct bevy::prelude::App"), they will be listed in insertion order in this vector.

```rust
let default_sampler = app.get_added_plugins::<ImagePlugin>()[0].default_sampler;
```

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/app.rs.html#663)

#### pub fn [add\_plugins](#method.add_plugins)<M>(&mut self, plugins: impl [Plugins](trait.Plugins.html "trait bevy::app::Plugins")<M>) -> &mut [App](../prelude/struct.App.html "struct bevy::prelude::App")

Installs a [`Plugin`](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") collection.

Bevy prioritizes modularity as a core principle. **All** engine features are implemented as plugins, even the complex ones like rendering.

[`Plugin`](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin")s can be grouped into a set by using a [`PluginGroup`](../prelude/trait.PluginGroup.html "trait bevy::prelude::PluginGroup").

There are built-in [`PluginGroup`](../prelude/trait.PluginGroup.html "trait bevy::prelude::PluginGroup")s that provide core engine functionality. The [`PluginGroup`](../prelude/trait.PluginGroup.html "trait bevy::prelude::PluginGroup")s available by default are `DefaultPlugins` and `MinimalPlugins`.

To customize the plugins in the group (reorder, disable a plugin, add a new plugin before / after another plugin), call [`build()`](../prelude/trait.PluginGroup.html#tymethod.build "method bevy::prelude::PluginGroup::build") on the group, which will convert it to a [`PluginGroupBuilder`](struct.PluginGroupBuilder.html "struct bevy::app::PluginGroupBuilder").

You can also specify a group of [`Plugin`](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin")s by using a tuple over [`Plugin`](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin")s and [`PluginGroup`](../prelude/trait.PluginGroup.html "trait bevy::prelude::PluginGroup")s. See [`Plugins`](trait.Plugins.html "trait bevy::app::Plugins") for more details.

###### Examples

```rust
App::new()
    .add_plugins(MinimalPlugins);
App::new()
    .add_plugins((MinimalPlugins, LogPlugin));
```

##### Panics

Panics if one of the plugins had already been added to the application.

##### [Examples found in repository](#scraped-examples-12)[?](../../scrape-examples-help.html)

examples/app/empty\_defaults.rs ([line 6](../../src/empty_defaults/empty_defaults.rs.html#6))

```rust
5fn main() {
6    App::new().add_plugins(DefaultPlugins).run();
7}
```

Hide additional examples

examples/2d/mesh2d.rs ([line 7](../../src/mesh2d/mesh2d.rs.html#7))

```rust
5fn main() {
6    App::new()
7        .add_plugins(DefaultPlugins)
8        .add_systems(Startup, setup)
9        .run();
10}
```

examples/2d/mesh2d\_alpha\_mode.rs ([line 12](../../src/mesh2d_alpha_mode/mesh2d_alpha_mode.rs.html#12))

```rust
10fn main() {
11    App::new()
12        .add_plugins(DefaultPlugins)
13        .add_systems(Startup, setup)
14        .run();
15}
```

examples/2d/mesh2d\_vertex\_color\_texture.rs ([line 8](../../src/mesh2d_vertex_color_texture/mesh2d_vertex_color_texture.rs.html#8))

```rust
6fn main() {
7    App::new()
8        .add_plugins(DefaultPlugins)
9        .add_systems(Startup, setup)
10        .run();
11}
```

examples/2d/sprite.rs ([line 7](../../src/sprite/sprite.rs.html#7))

```rust
5fn main() {
6    App::new()
7        .add_plugins(DefaultPlugins)
8        .add_systems(Startup, setup)
9        .run();
10}
```

examples/2d/sprite\_flipping.rs ([line 7](../../src/sprite_flipping/sprite_flipping.rs.html#7))

```rust
5fn main() {
6    App::new()
7        .add_plugins(DefaultPlugins)
8        .add_systems(Startup, setup)
9        .run();
10}
```

Additional examples can be found in:  

*   [examples/2d/sprite\_slice.rs](../../src/sprite_slice/sprite_slice.rs.html#7)
*   [examples/2d/transparency\_2d.rs](../../src/transparency_2d/transparency_2d.rs.html#8)
*   [examples/3d/orthographic.rs](../../src/orthographic/orthographic.rs.html#7)
*   [examples/3d/texture.rs](../../src/texture/texture.rs.html#9)
*   [examples/3d/two\_passes.rs](../../src/two_passes/two_passes.rs.html#7)
*   [examples/3d/vertex\_colors.rs](../../src/vertex_colors/vertex_colors.rs.html#7)
*   [examples/animation/eased\_motion.rs](../../src/eased_motion/eased_motion.rs.html#14)
*   [examples/asset/asset\_loading.rs](../../src/asset_loading/asset_loading.rs.html#7)
*   [examples/asset/hot\_asset\_reloading.rs](../../src/hot_asset_reloading/hot_asset_reloading.rs.html#12)
*   [examples/asset/repeated\_texture.rs](../../src/repeated_texture/repeated_texture.rs.html#12)
*   [examples/audio/audio.rs](../../src/audio/audio.rs.html#8)
*   [examples/camera/custom\_projection.rs](../../src/custom_projection/custom_projection.rs.html#8)
*   [examples/reflection/reflection.rs](../../src/reflection/reflection.rs.html#19)
*   [examples/reflection/reflection\_types.rs](../../src/reflection_types/reflection_types.rs.html#13)
*   [examples/ui/images/image\_node.rs](../../src/image_node/image_node.rs.html#8)
*   [examples/ui/scroll\_and\_overflow/drag\_to\_scroll.rs](../../src/drag_to_scroll/drag_to_scroll.rs.html#14)
*   [examples/ui/scroll\_and\_overflow/overflow\_clip\_margin.rs](../../src/overflow_clip_margin/overflow_clip_margin.rs.html#7)
*   [examples/ui/styling/borders.rs](../../src/borders/borders.rs.html#7)
*   [examples/ui/styling/stacked\_gradients.rs](../../src/stacked_gradients/stacked_gradients.rs.html#11)
*   [examples/ui/text/editable\_text\_filter.rs](../../src/editable_text_filter/editable_text_filter.rs.html#11)
*   [examples/ui/text/font\_query.rs](../../src/font_query/font_query.rs.html#9)
*   [examples/ui/text/font\_variations.rs](../../src/font_variations/font_variations.rs.html#8)
*   [examples/ui/text/font\_weights.rs](../../src/font_weights/font_weights.rs.html#7)
*   [examples/ui/text/strikethrough\_and\_underline.rs](../../src/strikethrough_and_underline/strikethrough_and_underline.rs.html#10)
*   [examples/ui/ui\_drag\_and\_drop.rs](../../src/ui_drag_and_drop/ui_drag_and_drop.rs.html#7)
*   [examples/ui/ui\_target\_camera.rs](../../src/ui_target_camera/ui_target_camera.rs.html#11)
*   [examples/input/mouse\_grab.rs](../../src/mouse_grab/mouse_grab.rs.html#10)
*   [examples/input/touch\_input.rs](../../src/touch_input/touch_input.rs.html#7)
*   [examples/ui/text/generic\_font\_families.rs](../../src/generic_font_families/generic_font_families.rs.html#26)
*   [examples/3d/3d\_scene.rs](../../src/3d_scene/3d_scene.rs.html#7)
*   [examples/input/gamepad\_input.rs](../../src/gamepad_input/gamepad_input.rs.html#7)
*   [examples/input/gamepad\_rumble.rs](../../src/gamepad_rumble/gamepad_rumble.rs.html#12)
*   [examples/scene/bsn.rs](../../src/bsn/bsn.rs.html#6)
*   [examples/input/touch\_input\_events.rs](../../src/touch_input_events/touch_input_events.rs.html#7)
*   [examples/input/keyboard\_input.rs](../../src/keyboard_input/keyboard_input.rs.html#7)
*   [examples/input/keyboard\_modifiers.rs](../../src/keyboard_modifiers/keyboard_modifiers.rs.html#7)
*   [examples/input/char\_input\_events.rs](../../src/char_input_events/char_input_events.rs.html#10)
*   [examples/app/drag\_and\_drop.rs](../../src/drag_and_drop/drag_and_drop.rs.html#7)
*   [examples/input/mouse\_input\_events.rs](../../src/mouse_input_events/mouse_input_events.rs.html#13)
*   [examples/3d/decal.rs](../../src/decal/decal.rs.html#15)
*   [examples/input/keyboard\_input\_events.rs](../../src/keyboard_input_events/keyboard_input_events.rs.html#7)
*   [examples/picking/dragdrop\_picking.rs](../../src/dragdrop_picking/dragdrop_picking.rs.html#24)
*   [examples/2d/mesh2d\_manual.rs](../../src/mesh2d_manual/mesh2d_manual.rs.html#44)
*   [examples/asset/embedded\_asset.rs](../../src/embedded_asset/embedded_asset.rs.html#17)
*   [examples/shader\_advanced/manual\_material.rs](../../src/manual_material/manual_material.rs.html#43)
*   [examples/ui/text/multiline\_text\_input.rs](../../src/multiline_text_input/multiline_text_input.rs.html#14)
*   [examples/shader\_advanced/custom\_shader\_instancing.rs](../../src/custom_shader_instancing/custom_shader_instancing.rs.html#51)
*   [examples/shader\_advanced/custom\_render\_phase.rs](../../src/custom_render_phase/custom_render_phase.rs.html#70)
*   [examples/asset/custom\_asset\_reader.rs](../../src/custom_asset_reader/custom_asset_reader.rs.html#57)
*   [examples/picking/simple\_picking.rs](../../src/simple_picking/simple_picking.rs.html#7)
*   [examples/reflection/serialization.rs](../../src/serialization/serialization.rs.html#16)
*   [examples/ecs/delayed\_commands.rs](../../src/delayed_commands/delayed_commands.rs.html#10)
*   [examples/2d/mesh2d\_repeated\_texture.rs](../../src/mesh2d_repeated_texture/mesh2d_repeated_texture.rs.html#20)
*   [examples/input/mouse\_input.rs](../../src/mouse_input/mouse_input.rs.html#10)
*   [examples/input/gamepad\_input\_events.rs](../../src/gamepad_input_events/gamepad_input_events.rs.html#13)
*   [examples/ecs/hierarchy.rs](../../src/hierarchy/hierarchy.rs.html#13)
*   [examples/ui/styling/gradients.rs](../../src/gradients/gradients.rs.html#21)
*   [examples/2d/sprite\_tile.rs](../../src/sprite_tile/sprite_tile.rs.html#7)
*   [examples/ui/images/ui\_texture\_slice\_flip\_and\_tile.rs](../../src/ui_texture_slice_flip_and_tile/ui_texture_slice_flip_and_tile.rs.html#11)
*   [examples/app/without\_winit.rs](../../src/without_winit/without_winit.rs.html#7)
*   [examples/transforms/translation.rs](../../src/translation/translation.rs.html#27)
*   [examples/3d/3d\_viewport\_to\_world.rs](../../src/3d_viewport_to_world/3d_viewport_to_world.rs.html#7)
*   [examples/transforms/3d\_rotation.rs](../../src/3d_rotation/3d_rotation.rs.html#15)
*   [examples/3d/generate\_custom\_mesh.rs](../../src/generate_custom_mesh/generate_custom_mesh.rs.html#20)
*   [examples/camera/2d\_on\_ui.rs](../../src/2d_on_ui/2d_on_ui.rs.html#7)
*   [examples/ui/images/ui\_texture\_atlas\_slice.rs](../../src/ui_texture_atlas_slice/ui_texture_atlas_slice.rs.html#12)
*   [examples/ui/images/ui\_texture\_slice.rs](../../src/ui_texture_slice/ui_texture_slice.rs.html#12)
*   [examples/ui/layout/ghost\_nodes.rs](../../src/ghost_nodes/ghost_nodes.rs.html#16)
*   [examples/3d/lines.rs](../../src/lines/lines.rs.html#17)
*   [examples/3d/parenting.rs](../../src/parenting/parenting.rs.html#8)
*   [examples/animation/easing\_functions.rs](../../src/easing_functions/easing_functions.rs.html#11)
*   [examples/gizmos/anchored\_text\_gizmos.rs](../../src/anchored_text_gizmos/anchored_text_gizmos.rs.html#11)
*   [examples/2d/move\_sprite.rs](../../src/move_sprite/move_sprite.rs.html#7)
*   [examples/ui/scroll\_and\_overflow/overflow.rs](../../src/overflow/overflow.rs.html#7)
*   [examples/shader/animate\_shader.rs](../../src/animate_shader/animate_shader.rs.html#13)
*   [examples/shader/shader\_defs.rs](../../src/shader_defs/shader_defs.rs.html#19)
*   [examples/shader/shader\_material.rs](../../src/shader_material/shader_material.rs.html#12)
*   [examples/shader/shader\_material\_glsl.rs](../../src/shader_material_glsl/shader_material_glsl.rs.html#13)
*   [examples/shader\_advanced/custom\_vertex\_attribute.rs](../../src/custom_vertex_attribute/custom_vertex_attribute.rs.html#19)
*   [examples/3d/animated\_material.rs](../../src/animated_material/animated_material.rs.html#7)
*   [examples/3d/transparency\_3d.rs](../../src/transparency_3d/transparency_3d.rs.html#9)
*   [examples/gizmos/3d\_text\_gizmos.rs](../../src/3d_text_gizmos/3d_text_gizmos.rs.html#8)
*   [tests/3d/no\_prepass.rs](../../src/no_prepass/no_prepass.rs.html#7-10)
*   [examples/ui/layout/z\_index.rs](../../src/z_index/z_index.rs.html#14)
*   [examples/ui/styling/transparency\_ui.rs](../../src/transparency_ui/transparency_ui.rs.html#9)
*   [examples/2d/bloom\_2d.rs](../../src/bloom_2d/bloom_2d.rs.html#11)
*   [examples/gltf/load\_gltf\_extras.rs](../../src/load_gltf_extras/load_gltf_extras.rs.html#10)
*   [examples/gizmos/text\_gizmos\_font.rs](../../src/text_gizmos_font/text_gizmos_font.rs.html#18)
*   [examples/shader\_advanced/specialized\_mesh\_pipeline.rs](../../src/specialized_mesh_pipeline/specialized_mesh_pipeline.rs.html#47)
*   [examples/ui/text/system\_fonts.rs](../../src/system_fonts/system_fonts.rs.html#10)
*   [examples/3d/order\_independent\_transparency.rs](../../src/order_independent_transparency/order_independent_transparency.rs.html#15)
*   [examples/gltf/query\_gltf\_primitives.rs](../../src/query_gltf_primitives/query_gltf_primitives.rs.html#10)
*   [examples/2d/rotate\_to\_cursor.rs](../../src/rotate_to_cursor/rotate_to_cursor.rs.html#8)
*   [examples/3d/pbr.rs](../../src/pbr/pbr.rs.html#8)
*   [examples/ui/text/text\_background\_colors.rs](../../src/text_background_colors/text_background_colors.rs.html#10)
*   [examples/gizmos/axes.rs](../../src/axes/axes.rs.html#12)
*   [examples/math/random\_sampling.rs](../../src/random_sampling/random_sampling.rs.html#14)
*   [examples/ui/relative\_cursor\_position.rs](../../src/relative_cursor_position/relative_cursor_position.rs.html#7)
*   [examples/app/thread\_pool\_resources.rs](../../src/thread_pool_resources/thread_pool_resources.rs.html#8-10)
*   [examples/ecs/parallel\_query.rs](../../src/parallel_query/parallel_query.rs.html#75)
*   [examples/3d/camera\_sub\_view.rs](../../src/camera_sub_view/camera_sub_view.rs.html#16)
*   [examples/movement/smooth\_follow.rs](../../src/smooth_follow/smooth_follow.rs.html#12)
*   [examples/transforms/scale.rs](../../src/scale/scale.rs.html#30)
*   [examples/3d/render\_to\_texture.rs](../../src/render_to_texture/render_to_texture.rs.html#10)
*   [examples/3d/rotate\_environment\_map.rs](../../src/rotate_environment_map/rotate_environment_map.rs.html#17)
*   [examples/3d/split\_screen.rs](../../src/split_screen/split_screen.rs.html#11)
*   [examples/asset/generated\_assets.rs](../../src/generated_assets/generated_assets.rs.html#7)
*   [examples/3d/blend\_modes.rs](../../src/blend_modes/blend_modes.rs.html#19)
*   [examples/async\_tasks/async\_compute.rs](../../src/async_compute/async_compute.rs.html#25)
*   [examples/ui/widgets/viewport\_node.rs](../../src/viewport_node/viewport_node.rs.html#15)
*   [examples/window/screenshot.rs](../../src/screenshot/screenshot.rs.html#11)
*   [examples/asset/web\_asset.rs](../../src/web_asset/web_asset.rs.html#10-12)
*   [examples/3d/bloom\_3d.rs](../../src/bloom_3d/bloom_3d.rs.html#16)
*   [examples/ecs/contiguous\_query.rs](../../src/contiguous_query/contiguous_query.rs.html#51)
*   [examples/shader\_advanced/custom\_post\_processing.rs](../../src/custom_post_processing/custom_post_processing.rs.html#32)
*   [examples/gltf/edit\_material\_on\_gltf.rs](../../src/edit_material_on_gltf/edit_material_on_gltf.rs.html#10)
*   [examples/ui/widgets/virtual\_keyboard.rs](../../src/virtual_keyboard/virtual_keyboard.rs.html#16)
*   [examples/ui/widgets/tab\_navigation.rs](../../src/tab_navigation/tab_navigation.rs.html#14)
*   [examples/shader/automatic\_instancing.rs](../../src/automatic_instancing/automatic_instancing.rs.html#14)
*   [examples/shader/shader\_material\_2d.rs](../../src/shader_material_2d/shader_material_2d.rs.html#16-19)
*   [examples/shader/shader\_material\_bindless.rs](../../src/shader_material_bindless/shader_material_bindless.rs.html#42-45)
*   [examples/shader/storage\_buffer.rs](../../src/storage_buffer/storage_buffer.rs.html#14)
*   [examples/audio/play\_sound\_effect.rs](../../src/play_sound_effect/play_sound_effect.rs.html#23)
*   [examples/gizmos/light\_gizmos.rs](../../src/light_gizmos/light_gizmos.rs.html#12)
*   [examples/shader/fallback\_image.rs](../../src/fallback_image/fallback_image.rs.html#17-20)
*   [examples/ui/text/ime\_support.rs](../../src/ime_support/ime_support.rs.html#20)
*   [examples/ui/text/text\_input.rs](../../src/text_input/text_input.rs.html#37)
*   [examples/animation/animation\_events.rs](../../src/animation_events/animation_events.rs.html#12)
*   [examples/2d/sprite\_scale.rs](../../src/sprite_scale/sprite_scale.rs.html#7)
*   [examples/shader/shader\_material\_screenspace\_texture.rs](../../src/shader_material_screenspace_texture/shader_material_screenspace_texture.rs.html#12)
*   [examples/ui/ui\_transform.rs](../../src/ui_transform/ui_transform.rs.html#10)
*   [examples/3d/auto\_exposure.rs](../../src/auto_exposure/auto_exposure.rs.html#25)
*   [examples/camera/camera\_orbit.rs](../../src/camera_orbit/camera_orbit.rs.html#38)
*   [examples/3d/mesh\_ray\_cast.rs](../../src/mesh_ray_cast/mesh_ray_cast.rs.html#13)
*   [examples/ui/widgets/vertical\_slider.rs](../../src/vertical_slider/vertical_slider.rs.html#18)
*   [examples/2d/tilemap\_chunk\_orientation.rs](../../src/tilemap_chunk_orientation/tilemap_chunk_orientation.rs.html#11)
*   [examples/audio/pitch.rs](../../src/pitch/pitch.rs.html#8)
*   [examples/time/timers.rs](../../src/timers/timers.rs.html#7)
*   [examples/ui/scroll\_and\_overflow/scroll.rs](../../src/scroll/scroll.rs.html#16)
*   [examples/ui/text/letter\_spacing.rs](../../src/letter_spacing/letter_spacing.rs.html#23)
*   [examples/2d/sprite\_sheet.rs](../../src/sprite_sheet/sprite_sheet.rs.html#8)
*   [examples/ui/ui\_material.rs](../../src/ui_material/ui_material.rs.html#13)
*   [examples/window/multiple\_windows.rs](../../src/multiple_windows/multiple_windows.rs.html#8)
*   [examples/camera/2d\_top\_down\_camera.rs](../../src/2d_top_down_camera/2d_top_down_camera.rs.html#25)
*   [examples/reflection/generic\_reflection.rs](../../src/generic_reflection/generic_reflection.rs.html#8)
*   [examples/3d/spherical\_area\_lights.rs](../../src/spherical_area_lights/spherical_area_lights.rs.html#11)
*   [examples/audio/soundtrack.rs](../../src/soundtrack/soundtrack.rs.html#8)
*   [examples/picking/sprite\_picking.rs](../../src/sprite_picking/sprite_picking.rs.html#9)
*   [examples/window/clear\_color.rs](../../src/clear_color/clear_color.rs.html#10)
*   [examples/app/log\_layers.rs](../../src/log_layers/log_layers.rs.html#56-61)
*   [examples/gizmos/2d\_gizmos.rs](../../src/2d_gizmos/2d_gizmos.rs.html#9)
*   [examples/gltf/update\_gltf\_scene.rs](../../src/update_gltf_scene/update_gltf_scene.rs.html#9)
*   [examples/ui/layout/size\_constraints.rs](../../src/size_constraints/size_constraints.rs.html#7)
*   [examples/2d/text2d.rs](../../src/text2d/text2d.rs.html#18)
*   [examples/gltf/load\_gltf.rs](../../src/load_gltf/load_gltf.rs.html#12)
*   [examples/2d/pixel\_grid\_snap.rs](../../src/pixel_grid_snap/pixel_grid_snap.rs.html#29)
*   [examples/audio/audio\_control.rs](../../src/audio_control/audio_control.rs.html#7)
*   [examples/2d/2d\_viewport\_to\_world.rs](../../src/2d_viewport_to_world/2d_viewport_to_world.rs.html#15)
*   [examples/ecs/component\_hooks.rs](../../src/component_hooks/component_hooks.rs.html#53)
*   [examples/ui/text/text.rs](../../src/text/text.rs.html#15)
*   [examples/ui/scroll\_and\_overflow/scrollbars.rs](../../src/scrollbars/scrollbars.rs.html#13)
*   [examples/3d/motion\_blur.rs](../../src/motion_blur/motion_blur.rs.html#14)
*   [examples/ui/render\_ui\_to\_texture.rs](../../src/render_ui_to_texture/render_ui_to_texture.rs.html#24)
*   [examples/audio/spatial\_audio\_3d.rs](../../src/spatial_audio_3d/spatial_audio_3d.rs.html#10)
*   [examples/3d/atmospheric\_fog.rs](../../src/atmospheric_fog/atmospheric_fog.rs.html#17)
*   [examples/2d/tilemap\_chunk.rs](../../src/tilemap_chunk/tilemap_chunk.rs.html#14)
*   [examples/shader/array\_texture.rs](../../src/array_texture/array_texture.rs.html#23-26)
*   [examples/camera/pan\_camera\_controller.rs](../../src/pan_camera_controller/pan_camera_controller.rs.html#17)
*   [examples/gizmos/3d\_gizmos.rs](../../src/3d_gizmos/3d_gizmos.rs.html#12)
*   [examples/asset/asset\_decompression.rs](../../src/asset_decompression/asset_decompression.rs.html#98)
*   [examples/dev\_tools/infinite\_grid.rs](../../src/infinite_grid/infinite_grid.rs.html#15-20)
*   [examples/shader\_advanced/fullscreen\_material.rs](../../src/fullscreen_material/fullscreen_material.rs.html#17-20)
*   [examples/scene/world\_serialization.rs](../../src/world_serialization/world_serialization.rs.html#38)
*   [examples/picking/mesh\_picking.rs](../../src/mesh_picking/mesh_picking.rs.html#29)
*   [examples/animation/animated\_transform.rs](../../src/animated_transform/animated_transform.rs.html#12)
*   [examples/animation/animated\_ui.rs](../../src/animated_ui/animated_ui.rs.html#28)
*   [examples/shader\_advanced/texture\_binding\_array.rs](../../src/texture_binding_array/texture_binding_array.rs.html#27-31)
*   [examples/transforms/align.rs](../../src/align/align.rs.html#14)
*   [examples/3d/ssao.rs](../../src/ssao/ssao.rs.html#19)
*   [examples/shader/shader\_material\_wesl.rs](../../src/shader_material_wesl/shader_material_wesl.rs.html#19-23)
*   [examples/animation/morph\_targets.rs](../../src/morph_targets/morph_targets.rs.html#12)
*   [examples/ui/text/font\_atlas\_debug.rs](../../src/font_atlas_debug/font_atlas_debug.rs.html#13)
*   [examples/window/monitor\_info.rs](../../src/monitor_info/monitor_info.rs.html#11-15)
*   [examples/gltf/gltf\_skinned\_mesh.rs](../../src/gltf_skinned_mesh/gltf_skinned_mesh.rs.html#10)
*   [examples/testbed/full\_ui.rs](../../src/testbed_full_ui/full_ui.rs.html#21)
*   [examples/animation/custom\_skinned\_mesh.rs](../../src/custom_skinned_mesh/custom_skinned_mesh.rs.html#21)
*   [examples/camera/free\_camera\_controller.rs](../../src/free_camera_controller/free_camera_controller.rs.html#60)
*   [examples/ui/widgets/button.rs](../../src/button/button.rs.html#12)
*   [examples/shader/extended\_material.rs](../../src/extended_material/extended_material.rs.html#17)
*   [examples/ecs/iter\_combinations.rs](../../src/iter_combinations/iter_combinations.rs.html#9)
*   [examples/3d/spotlight.rs](../../src/spotlight/spotlight.rs.html#28)
*   [examples/2d/mesh2d\_arcs.rs](../../src/mesh2d_arcs/mesh2d_arcs.rs.html#19)
*   [examples/shader/extended\_material\_bindless.rs](../../src/extended_material_bindless/extended_material_bindless.rs.html#98)
*   [examples/3d/fog.rs](../../src/fog/fog.rs.html#33)
*   [examples/shader\_advanced/compute\_mesh.rs](../../src/compute_mesh/compute_mesh.rs.html#38-42)
*   [examples/gizmos/transform\_gizmo.rs](../../src/transform_gizmo/transform_gizmo.rs.html#20-25)
*   [examples/audio/decodable.rs](../../src/decodable/decodable.rs.html#86-89)
*   [examples/usage/cooldown.rs](../../src/cooldown/cooldown.rs.html#10)
*   [tests/3d/test\_invalid\_skinned\_mesh.rs](../../src/test_invalid_skinned_mesh/test_invalid_skinned_mesh.rs.html#18)
*   [examples/async\_tasks/external\_source\_external\_thread.rs](../../src/external_source_external_thread/external_source_external_thread.rs.html#12)
*   [examples/ui/layout/display\_and\_visibility.rs](../../src/display_and_visibility/display_and_visibility.rs.html#14)
*   [examples/3d/clearcoat.rs](../../src/clearcoat/clearcoat.rs.html#54)
*   [examples/diagnostics/enabling\_disabling\_diagnostic.rs](../../src/enabling_disabling_diagnostic/enabling_disabling_diagnostic.rs.html#13-17)
*   [examples/app/no\_renderer.rs](../../src/no_renderer/no_renderer.rs.html#14-23)
*   [examples/ui/layout/anchor\_layout.rs](../../src/anchor_layout/anchor_layout.rs.html#8-14)
*   [examples/audio/spatial\_audio\_2d.rs](../../src/spatial_audio_2d/spatial_audio_2d.rs.html#16-19)
*   [examples/remote/server.rs](../../src/server/server.rs.html#17)
*   [examples/ecs/change\_detection.rs](../../src/change_detection/change_detection.rs.html#8)
*   [examples/ecs/entity\_disabling.rs](../../src/entity_disabling/entity_disabling.rs.html#25)
*   [examples/ecs/removal\_detection.rs](../../src/removal_detection/removal_detection.rs.html#15)
*   [examples/animation/animated\_mesh.rs](../../src/animated_mesh/animated_mesh.rs.html#19)
*   [examples/usage/context\_menu.rs](../../src/context_menu/context_menu.rs.html#30)
*   [examples/camera/first\_person\_view\_model.rs](../../src/first_person_view_model/first_person_view_model.rs.html#54)
*   [examples/ui/layout/flex\_layout.rs](../../src/flex_layout/flex_layout.rs.html#10-16)
*   [examples/app/plugin.rs](../../src/plugin/plugin.rs.html#13-19)
*   [examples/ui/images/image\_node\_resizing.rs](../../src/image_node_resizing/image_node_resizing.rs.html#19)
*   [examples/shader/gpu\_readback.rs](../../src/gpu_readback/gpu_readback.rs.html#30-35)
*   [examples/transforms/transform.rs](../../src/transform/transform.rs.html#26)
*   [examples/app/render\_recovery.rs](../../src/render_recovery/render_recovery.rs.html#22-25)
*   [examples/asset/asset\_settings.rs](../../src/asset_settings/asset_settings.rs.html#10-16)
*   [examples/asset/custom\_asset.rs](../../src/custom_asset/custom_asset.rs.html#94)
*   [examples/showcase/contributors.rs](../../src/contributors/contributors.rs.html#17)
*   [examples/3d/parallax\_mapping.rs](../../src/parallax_mapping/parallax_mapping.rs.html#10)
*   [examples/asset/alter\_mesh.rs](../../src/alter_mesh/alter_mesh.rs.html#10)
*   [examples/3d/skybox.rs](../../src/skybox/skybox.rs.html#40)
*   [examples/asset/alter\_sprite.rs](../../src/alter_sprite/alter_sprite.rs.html#10)
*   [examples/2d/rotation.rs](../../src/rotation/rotation.rs.html#9)
*   [examples/ui/layout/grid.rs](../../src/grid/grid.rs.html#6-13)
*   [examples/2d/texture\_atlas.rs](../../src/texture_atlas/texture_atlas.rs.html#14)
*   [examples/3d/deferred\_rendering.rs](../../src/deferred_rendering/deferred_rendering.rs.html#22)
*   [examples/ecs/observer\_propagation.rs](../../src/observer_propagation/observer_propagation.rs.html#10)
*   [examples/3d/color\_grading.rs](../../src/color_grading/color_grading.rs.html#102)
*   [examples/window/window\_resizing.rs](../../src/window_resizing/window_resizing.rs.html#11)
*   [examples/ui/text/multiple\_text\_inputs.rs](../../src/multiple_text_inputs/multiple_text_inputs.rs.html#21)
*   [examples/dev\_tools/schedule\_data.rs](../../src/schedule_data/schedule_data.rs.html#7)
*   [examples/3d/fog\_volumes.rs](../../src/fog_volumes/fog_volumes.rs.html#18-24)
*   [examples/3d/transmission.rs](../../src/transmission/transmission.rs.html#48)
*   [examples/time/time.rs](../../src/time/time.rs.html#113)
*   [examples/3d/scrolling\_fog.rs](../../src/scrolling_fog/scrolling_fog.rs.html#27-33)
*   [examples/ui/images/ui\_texture\_atlas.rs](../../src/ui_texture_atlas/ui_texture_atlas.rs.html#7-12)
*   [examples/ecs/one\_shot\_systems.rs](../../src/one_shot_systems/one_shot_systems.rs.html#18)
*   [examples/3d/post\_processing.rs](../../src/post_processing/post_processing.rs.html#54)
*   [examples/3d/meshlet.rs](../../src/meshlet/meshlet.rs.html#20-27)
*   [examples/3d/shadow\_biases.rs](../../src/shadow_biases/shadow_biases.rs.html#11)
*   [examples/ui/ui\_scaling.rs](../../src/ui_scaling/ui_scaling.rs.html#11)
*   [examples/async\_tasks/async\_channel\_pattern.rs](../../src/async_channel_pattern/async_channel_pattern.rs.html#24)
*   [examples/animation/color\_animation.rs](../../src/color_animation/color_animation.rs.html#23)
*   [examples/app/return\_after\_run.rs](../../src/return_after_run/return_after_run.rs.html#13-19)
*   [examples/window/custom\_cursor\_image.rs](../../src/custom_cursor_image/custom_cursor_image.rs.html#13)
*   [examples/window/window\_drag\_move.rs](../../src/window_drag_move/window_drag_move.rs.html#44-50)
*   [examples/ecs/fixed\_timestep.rs](../../src/fixed_timestep/fixed_timestep.rs.html#7)
*   [examples/3d/rect\_light.rs](../../src/rect_light/rect_light.rs.html#8-17)
*   [examples/3d/shadow\_caster\_receiver.rs](../../src/shadow_caster_receiver/shadow_caster_receiver.rs.html#19)
*   [examples/ui/text/text\_debug.rs](../../src/text_debug/text_debug.rs.html#15-24)
*   [examples/window/scale\_factor\_override.rs](../../src/scale_factor_override/scale_factor_override.rs.html#11-17)
*   [examples/app/log\_layers\_ecs.rs](../../src/log_layers_ecs/log_layers_ecs.rs.html#27-34)
*   [tests/window/minimizing.rs](../../src/minimizing/minimizing.rs.html#9-15)
*   [examples/app/logs.rs](../../src/logs/logs.rs.html#7-12)
*   [examples/2d/2d\_shapes.rs](../../src/2d_shapes/2d_shapes.rs.html#25-29)
*   [examples/diagnostics/custom\_diagnostic.rs](../../src/custom_diagnostic/custom_diagnostic.rs.html#12-17)
*   [examples/ecs/hotpatching\_systems.rs](../../src/hotpatching_systems/hotpatching_systems.rs.html#29)
*   [examples/animation/animated\_mesh\_events.rs](../../src/animated_mesh_events/animated_mesh_events.rs.html#23)
*   [examples/showcase/loading\_screen.rs](../../src/loading_screen/loading_screen.rs.html#20)
*   [examples/3d/anti\_aliasing.rs](../../src/anti_aliasing/anti_aliasing.rs.html#37)
*   [examples/stress\_tests/many\_cameras\_lights.rs](../../src/many_cameras_lights/many_cameras_lights.rs.html#15-22)
*   [examples/ui/widgets/feathers\_gallery.rs](../../src/feathers_gallery/feathers_gallery.rs.html#66)
*   [examples/animation/animated\_mesh\_control.rs](../../src/animated_mesh_control/animated_mesh_control.rs.html#19)
*   [examples/gltf/gltf\_extension\_animation\_graph.rs](../../src/gltf_extension_animation_graph/gltf_extension_animation_graph.rs.html#28)
*   [examples/showcase/game\_menu.rs](../../src/game_menu/game_menu.rs.html#35)
*   [examples/math/cubic\_splines.rs](../../src/cubic_splines/cubic_splines.rs.html#15)
*   [examples/3d/volumetric\_fog.rs](../../src/volumetric_fog/volumetric_fog.rs.html#44)
*   [examples/2d/cpu\_draw.rs](../../src/cpu_draw/cpu_draw.rs.html#20)
*   [examples/3d/depth\_of\_field.rs](../../src/depth_of_field/depth_of_field.rs.html#55-61)
*   [examples/3d/lightmaps.rs](../../src/lightmaps/lightmaps.rs.html#29)
*   [examples/asset/asset\_saving.rs](../../src/asset_saving/asset_saving.rs.html#22-27)
*   [examples/ui/scroll\_and\_overflow/overflow\_debug.rs](../../src/overflow_debug/overflow_debug.rs.html#12)
*   [examples/ui/widgets/feathers\_counter.rs](../../src/feathers_counter/feathers_counter.rs.html#24-29)
*   [examples/3d/lighting.rs](../../src/lighting/lighting.rs.html#15)
*   [examples/3d/anisotropy.rs](../../src/anisotropy/anisotropy.rs.html#84-90)
*   [examples/3d/3d\_shapes.rs](../../src/3d_shapes/3d_shapes.rs.html#30-34)
*   [examples/shader/shader\_prepass.rs](../../src/shader_prepass/shader_prepass.rs.html#21-31)
*   [examples/app/settings.rs](../../src/settings/settings.rs.html#22-30)
*   [examples/asset/extra\_source.rs](../../src/extra_asset_source/extra_source.rs.html#25)
*   [examples/ecs/state\_scoped.rs](../../src/state_scoped/state_scoped.rs.html#15)
*   [examples/ecs/generic\_system.rs](../../src/generic_system/generic_system.rs.html#35)
*   [examples/remote/app\_under\_test.rs](../../src/app_under_test/app_under_test.rs.html#23)
*   [examples/2d/sprite\_animation.rs](../../src/sprite_animation/sprite_animation.rs.html#11)
*   [examples/window/persisting\_window\_settings.rs](../../src/persisting_window_settings/persisting_window_settings.rs.html#20-28)
*   [examples/stress\_tests/text\_pipeline.rs](../../src/text_pipeline/text_pipeline.rs.html#16-27)
*   [examples/gizmos/2d\_text\_gizmos.rs](../../src/2d_text_gizmos/2d_text_gizmos.rs.html#19-30)
*   [examples/animation/animation\_masks.rs](../../src/animation_masks/animation_masks.rs.html#97-103)
*   [examples/3d/atmosphere.rs](../../src/atmosphere/atmosphere.rs.html#47-51)
*   [examples/ui/styling/box\_shadow.rs](../../src/box_shadow/box_shadow.rs.html#119)
*   [examples/window/multi\_window\_text.rs](../../src/multi_window_text/multi_window_text.rs.html#14-22)
*   [examples/app/plugin\_group.rs](../../src/plugin_group/plugin_group.rs.html#8-13)
*   [examples/shader\_advanced/custom\_phase\_item.rs](../../src/custom_phase_item/custom_phase_item.rs.html#166)
*   [examples/picking/custom\_hit\_data.rs](../../src/custom_hit_data/custom_hit_data.rs.html#27)
*   [examples/3d/specular\_tint.rs](../../src/specular_tint/specular_tint.rs.html#53-59)
*   [examples/ui/window\_fallthrough.rs](../../src/window_fallthrough/window_fallthrough.rs.html#10-19)
*   [tests/window/resizing.rs](../../src/resizing/resizing.rs.html#22-32)
*   [examples/shader/compute\_shader\_game\_of\_life.rs](../../src/compute_shader_game_of_life/compute_shader_game_of_life.rs.html#35-48)
*   [examples/ui/text/text\_wrap\_debug.rs](../../src/text_wrap_debug/text_wrap_debug.rs.html#35-38)
*   [examples/ecs/extraction.rs](../../src/extraction/extraction.rs.html#51-55)
*   [examples/shader\_advanced/render\_depth\_to\_texture.rs](../../src/render_depth_to_texture/render_depth_to_texture.rs.html#104)
*   [examples/stress\_tests/many\_lights.rs](../../src/many_lights/many_lights.rs.html#21-34)
*   [examples/showcase/breakout.rs](../../src/breakout/breakout.rs.html#55)
*   [examples/camera/2d\_screen\_shake.rs](../../src/2d_screen_shake/2d_screen_shake.rs.html#55)
*   [examples/gltf/custom\_gltf\_vertex\_attribute.rs](../../src/custom_gltf_vertex_attribute/custom_gltf_vertex_attribute.rs.html#31-41)
*   [examples/3d/pccm.rs](../../src/pccm/pccm.rs.html#55-64)
*   [examples/3d/reflection\_probes.rs](../../src/reflection_probes/reflection_probes.rs.html#73)
*   [examples/3d/visibility\_range.rs](../../src/visibility_range/visibility_range.rs.html#88-94)
*   [examples/stress\_tests/many\_gizmos.rs](../../src/many_gizmos/many_gizmos.rs.html#16-28)
*   [examples/showcase/alien\_cake\_addict.rs](../../src/alien_cake_addict/alien_cake_addict.rs.html#22)
*   [examples/ecs/fallible\_params.rs](../../src/fallible_params/fallible_params.rs.html#38)
*   [examples/animation/animation\_graph.rs](../../src/animation_graph/animation_graph.rs.html#77-83)
*   [examples/ui/widgets/standard\_widgets.rs](../../src/standard_widgets/standard_widgets.rs.html#29)
*   [examples/math/bounding\_2d.rs](../../src/bounding_2d/bounding_2d.rs.html#11)
*   [examples/asset/asset\_saving\_with\_subassets.rs](../../src/asset_saving_with_subassets/asset_saving_with_subassets.rs.html#18-23)
*   [examples/window/transparent\_window.rs](../../src/transparent_window/transparent_window.rs.html#13-26)
*   [examples/stress\_tests/many\_glyphs.rs](../../src/many_glyphs/many_glyphs.rs.html#42-53)
*   [examples/stress\_tests/many\_animated\_sprite\_meshes.rs](../../src/many_animated_sprite_meshes/many_animated_sprite_meshes.rs.html#24-35)
*   [examples/stress\_tests/many\_animated\_sprites.rs](../../src/many_animated_sprites/many_animated_sprites.rs.html#22-33)
*   [examples/3d/contact\_shadows.rs](../../src/contact_shadows/contact_shadows.rs.html#83-92)
*   [examples/gltf/gltf\_extension\_mesh\_2d.rs](../../src/gltf_extension_mesh_2d/gltf_extension_mesh_2d.rs.html#36-51)
*   [examples/stress\_tests/many\_materials.rs](../../src/many_materials/many_materials.rs.html#27-39)
*   [examples/stress\_tests/many\_sprite\_meshes.rs](../../src/many_sprite_meshes/many_sprite_meshes.rs.html#35-46)
*   [examples/stress\_tests/many\_sprites.rs](../../src/many_sprites/many_sprites.rs.html#33-44)
*   [examples/stress\_tests/many\_text2d.rs](../../src/many_text2d/many_text2d.rs.html#73-84)
*   [examples/ecs/system\_piping.rs](../../src/system_piping/system_piping.rs.html#13-17)
*   [examples/3d/ssr.rs](../../src/ssr/ssr.rs.html#211-217)
*   [examples/picking/debug\_picking.rs](../../src/debug_picking/debug_picking.rs.html#9-12)
*   [examples/state/custom\_transitions.rs](../../src/custom_transitions/custom_transitions.rs.html#30-33)
*   [examples/usage/debug\_frustum\_culling.rs](../../src/debug_frustum_culling/debug_frustum_culling.rs.html#45-54)
*   [examples/camera/projection\_zoom.rs](../../src/projection_zoom/projection_zoom.rs.html#23)
*   [examples/time/virtual\_time.rs](../../src/virtual_time/virtual_time.rs.html#13)
*   [examples/asset/multi\_asset\_sync.rs](../../src/multi_asset_sync/multi_asset_sync.rs.html#18)
*   [examples/3d/solari.rs](../../src/solari/solari.rs.html#50-55)
*   [tests/3d/test\_skinned\_mesh\_bounds.rs](../../src/test_skinned_mesh_bounds/test_skinned_mesh_bounds.rs.html#17-23)
*   [examples/3d/occlusion\_culling.rs](../../src/occlusion_culling/occlusion_culling.rs.html#168-185)
*   [examples/stress\_tests/many\_cubes.rs](../../src/many_cubes/many_cubes.rs.html#127-138)
*   [examples/stress\_tests/many\_gradients.rs](../../src/many_gradients/many_gradients.rs.html#59-71)
*   [tests/window/desktop\_request\_redraw.rs](../../src/desktop_request_redraw/desktop_request_redraw.rs.html#11)
*   [examples/3d/pcss.rs](../../src/pcss/pcss.rs.html#119-129)
*   [examples/3d/mixed\_lighting.rs](../../src/mixed_lighting/mixed_lighting.rs.html#119-125)
*   [examples/app/headless.rs](../../src/headless/headless.rs.html#28)
*   [examples/3d/clustered\_decals.rs](../../src/clustered_decals/clustered_decals.rs.html#123-129)
*   [examples/3d/clustered\_decal\_maps.rs](../../src/clustered_decal_maps/clustered_decal_maps.rs.html#132-144)
*   [examples/window/low\_power.rs](../../src/low_power/low_power.rs.html#25-32)
*   [examples/3d/mirror.rs](../../src/mirror/mirror.rs.html#113-119)
*   [examples/state/sub\_states.rs](../../src/sub_states/sub_states.rs.html#37)
*   [examples/stress\_tests/many\_morph\_targets.rs](../../src/many_morph_targets/many_morph_targets.rs.html#142-154)
*   [examples/showcase/desk\_toy.rs](../../src/desk_toy/desk_toy.rs.html#21-30)
*   [examples/math/custom\_primitives.rs](../../src/custom_primitives/custom_primitives.rs.html#133)
*   [examples/3d/tonemapping.rs](../../src/tonemapping/tonemapping.rs.html#24-32)
*   [examples/testbed/2d.rs](../../src/testbed_2d/2d.rs.html#26)
*   [examples/ui/widgets/standard\_widgets\_observers.rs](../../src/standard_widgets_observers/standard_widgets_observers.rs.html#22)
*   [examples/stress\_tests/transform\_hierarchy.rs](../../src/transform_hierarchy/transform_hierarchy.rs.html#190-198)
*   [examples/2d/wireframe\_2d.rs](../../src/wireframe_2d/wireframe_2d.rs.html#22-34)
*   [examples/state/states.rs](../../src/states/states.rs.html#12)
*   [examples/3d/wireframe.rs](../../src/wireframe/wireframe.rs.html#23-35)
*   [examples/stress\_tests/many\_foxes.rs](../../src/many_foxes/many_foxes.rs.html#49-61)
*   [examples/app/externally\_driven\_headless\_renderer.rs](../../src/externally_driven_headless_renderer/externally_driven_headless_renderer.rs.html#55-61)
*   [examples/asset/processing/asset\_processing.rs](../../src/asset_processing/asset_processing.rs.html#29-40)
*   [examples/dev\_tools/fps\_overlay.rs](../../src/fps_overlay/fps_overlay.rs.html#18-45)
*   [examples/stress\_tests/bevymark\_3d.rs](../../src/bevymark_3d/bevymark_3d.rs.html#104-116)
*   [examples/3d/light\_textures.rs](../../src/light_textures/light_textures.rs.html#109-115)
*   [examples/stress\_tests/bevymark.rs](../../src/bevymark/bevymark.rs.html#135-147)
*   [examples/math/render\_primitives.rs](../../src/render_primitives/render_primitives.rs.html#12)
*   [examples/ecs/message.rs](../../src/message/message.rs.html#133)
*   [examples/diagnostics/log\_diagnostics.rs](../../src/log_diagnostics/log_diagnostics.rs.html#27-42)
*   [examples/ui/navigation/directional\_navigation.rs](../../src/directional_navigation/directional_navigation.rs.html#37)
*   [examples/ecs/system\_closure.rs](../../src/system_closure/system_closure.rs.html#28)
*   [examples/testbed/3d.rs](../../src/testbed_3d/3d.rs.html#25)
*   [examples/app/headless\_renderer.rs](../../src/headless_renderer/headless_renderer.rs.html#85-99)
*   [examples/3d/light\_probe\_blending.rs](../../src/light_probe_blending/light_probe_blending.rs.html#152-158)
*   [examples/2d/dynamic\_mip\_generation.rs](../../src/dynamic_mip_generation/dynamic_mip_generation.rs.html#210-219)
*   [examples/ecs/observers.rs](../../src/observers/observers.rs.html#10)
*   [examples/window/window\_settings.rs](../../src/window_settings/window_settings.rs.html#17-43)
*   [tests/ecs/ambiguity\_detection.rs](../../src/ambiguity_detection/ambiguity_detection.rs.html#16-30)
*   [examples/stress\_tests/many\_buttons.rs](../../src/many_buttons/many_buttons.rs.html#75-86)
*   [examples/ecs/error\_handling.rs](../../src/error_handling/error_handling.rs.html#24)
*   [examples/ui/navigation/directional\_navigation\_overrides.rs](../../src/directional_navigation_overrides/directional_navigation_overrides.rs.html#44)
*   [examples/testbed/ui.rs](../../src/testbed_ui/ui.rs.html#26-35)
*   [examples/3d/irradiance\_volumes.rs](../../src/irradiance_volumes/irradiance_volumes.rs.html#150-156)
*   [examples/ecs/nondeterministic\_system\_order.rs](../../src/nondeterministic_system_order/nondeterministic_system_order.rs.html#61)
*   [examples/state/computed\_states.rs](../../src/computed_states/computed_states.rs.html#175)
*   [examples/movement/physics\_in\_fixed\_timestep.rs](../../src/physics_in_fixed_timestep/physics_in_fixed_timestep.rs.html#102)
*   [examples/ecs/run\_conditions.rs](../../src/run_conditions/run_conditions.rs.html#12)
*   [examples/stress\_tests/many\_components.rs](../../src/many_components/many_components.rs.html#162)
*   [examples/ecs/ecs\_guide.rs](../../src/ecs_guide/ecs_guide.rs.html#302)
*   [examples/ecs/system\_stepping.rs](../../src/system_stepping/system_stepping.rs.html#12)

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/app.rs.html#685)

#### pub fn [register\_type](#method.register_type)<T>(&mut self) -> &mut [App](../prelude/struct.App.html "struct bevy::prelude::App")

where T: [GetTypeRegistration](../reflect/trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"),

Available on **crate feature `bevy_reflect`** only.

Registers the type `T` in the [`AppTypeRegistry`](../prelude/struct.AppTypeRegistry.html "struct bevy::prelude::AppTypeRegistry") resource, adding reflect data as specified in the [`Reflect`](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") derive:

[ⓘ](# "This example is not tested")

```rust
#[derive(Component, Serialize, Deserialize, Reflect)]
#[reflect(Component, Serialize, Deserialize)] // will register ReflectComponent, ReflectSerialize, ReflectDeserialize
```

See [`bevy_reflect::TypeRegistry::register`](../reflect/struct.TypeRegistry.html#method.register "method bevy::reflect::TypeRegistry::register") for more information.

##### [Examples found in repository](#scraped-examples-13)[?](../../scrape-examples-help.html)

examples/reflection/generic\_reflection.rs ([line 10](../../src/generic_reflection/generic_reflection.rs.html#10))

```rust
6fn main() {
7    App::new()
8        .add_plugins(DefaultPlugins)
9        // You must manually register each instance of a generic type
10        .register_type::<MyType<u32>>()
11        .add_systems(Startup, setup)
12        .run();
13}
```

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/app.rs.html#712-714)

#### pub fn [register\_type\_data](#method.register_type_data)<T, D>(&mut self) -> &mut [App](../prelude/struct.App.html "struct bevy::prelude::App")

where T: [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), D: [TypeData](../reflect/trait.TypeData.html "trait bevy::reflect::TypeData") + [FromType](../reflect/trait.FromType.html "trait bevy::reflect::FromType")<T>,

Available on **crate feature `bevy_reflect`** only.

Associates type data `D` with type `T` in the [`AppTypeRegistry`](../prelude/struct.AppTypeRegistry.html "struct bevy::prelude::AppTypeRegistry") resource.

Most of the time [`register_type`](../prelude/struct.App.html#method.register_type "method bevy::prelude::App::register_type") can be used instead to register a type you derived [`Reflect`](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for. However, in cases where you want to add a piece of type data that was not included in the list of `#[reflect(...)]` type data in the derive, or where the type is generic and cannot register e.g. `ReflectSerialize` unconditionally without knowing the specific type parameters, this method can be used to insert additional type data.

##### Example

```rust
use bevy_app::App;
use bevy_reflect::{ReflectSerialize, ReflectDeserialize};

App::new()
    .register_type::<Option<String>>()
    .register_type_data::<Option<String>, ReflectSerialize>()
    .register_type_data::<Option<String>, ReflectDeserialize>();
```

See [`bevy_reflect::TypeRegistry::register_type_data`](../reflect/struct.TypeRegistry.html#method.register_type_data "method bevy::reflect::TypeRegistry::register_type_data").

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/app.rs.html#738-742)

#### pub fn [register\_type\_conversion](#method.register_type_conversion)<T, U, F>(&mut self, function: F) -> &mut [App](../prelude/struct.App.html "struct bevy::prelude::App")

where T: [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), U: [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), F: [Fn](https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html "trait core::ops::function::Fn")(T) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<U, T> + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

Available on **crate feature `bevy_reflect`** only.

Registers a fallible conversion from type T to U with the reflection system.

The supplied closure is expected to produce a value of type U, given an instance of type T. If the conversion fails, the closure should return the input value, wrapped in an `Err` variant.

##### Example

```rust
use bevy_app::App;

App::new()
    .register_type::<i32>()
    .register_type::<String>()
    .register_type_conversion::<i32, String, _>(|n| Ok(n.to_string()));
```

See [`bevy_reflect::TypeRegistry::register_type_conversion`](../reflect/struct.TypeRegistry.html#method.register_type_conversion "method bevy::reflect::TypeRegistry::register_type_conversion").

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/app.rs.html#763-766)

#### pub fn [register\_into\_type\_conversion](#method.register_into_type_conversion)<T, U>(&mut self) -> &mut [App](../prelude/struct.App.html "struct bevy::prelude::App")

where T: [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), U: [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<T>,

Available on **crate feature `bevy_reflect`** only.

Given types T and U, where `U: From<T>`, registers that conversion with the reflection system.

##### Example

```rust
use bevy_app::App;

App::new()
    .register_type::<u8>()
    .register_type::<u32>()
    .register_into_type_conversion::<u8, u32>();
```

See [`bevy_reflect::TypeRegistry::register_into_type_conversion`](../reflect/struct.TypeRegistry.html#method.register_into_type_conversion "method bevy::reflect::TypeRegistry::register_into_type_conversion").

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/app.rs.html#835-837)

#### pub fn [register\_function](#method.register_function)<F, Marker>(&mut self, function: F) -> &mut [App](../prelude/struct.App.html "struct bevy::prelude::App")

where F: [IntoFunction](../prelude/trait.IntoFunction.html "trait bevy::prelude::IntoFunction")<'static, Marker> + 'static,

Available on **crate feature `reflect_functions`** only.

Registers the given function into the [`AppFunctionRegistry`](../prelude/struct.AppFunctionRegistry.html "struct bevy::prelude::AppFunctionRegistry") resource.

The given function will internally be stored as a [`DynamicFunction`](../reflect/func/struct.DynamicFunction.html "struct bevy::reflect::func::DynamicFunction") and mapped according to its [name](../reflect/func/struct.FunctionInfo.html#method.name "method bevy::reflect::func::FunctionInfo::name").

Because the function must have a name, anonymous functions (e.g. `|a: i32, b: i32| { a + b }`) and closures must instead be registered using [`register_function_with_name`](../prelude/struct.App.html#method.register_function_with_name "method bevy::prelude::App::register_function_with_name") or converted to a [`DynamicFunction`](../reflect/func/struct.DynamicFunction.html "struct bevy::reflect::func::DynamicFunction") and named using [`DynamicFunction::with_name`](../reflect/func/struct.DynamicFunction.html#method.with_name "method bevy::reflect::func::DynamicFunction::with_name"). Failure to do so will result in a panic.

Only types that implement [`IntoFunction`](../prelude/trait.IntoFunction.html "trait bevy::prelude::IntoFunction") may be registered via this method.

See [`FunctionRegistry::register`](../reflect/func/struct.FunctionRegistry.html#method.register "method bevy::reflect::func::FunctionRegistry::register") for more information.

##### Panics

Panics if a function has already been registered with the given name or if the function is missing a name (such as when it is an anonymous function).

##### Examples

```rust
use bevy_app::App;

fn add(a: i32, b: i32) -> i32 {
    a + b
}

App::new().register_function(add);
```

Functions cannot be registered more than once.

[ⓘ](# "This example panics")

```rust
use bevy_app::App;

fn add(a: i32, b: i32) -> i32 {
    a + b
}

App::new()
    .register_function(add)
    // Panic! A function has already been registered with the name "my_function"
    .register_function(add);
```

Anonymous functions and closures should be registered using [`register_function_with_name`](../prelude/struct.App.html#method.register_function_with_name "method bevy::prelude::App::register_function_with_name") or given a name using [`DynamicFunction::with_name`](../reflect/func/struct.DynamicFunction.html#method.with_name "method bevy::reflect::func::DynamicFunction::with_name").

[ⓘ](# "This example panics")

```rust
use bevy_app::App;

// Panic! Anonymous functions cannot be registered using `register_function`
App::new().register_function(|a: i32, b: i32| a + b);
```

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/app.rs.html#910-916)

#### pub fn [register\_function\_with\_name](#method.register_function_with_name)<F, Marker>( &mut self, name: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Cow](https://doc.rust-lang.org/nightly/alloc/borrow/enum.Cow.html "enum alloc::borrow::Cow")<'static, [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>>, function: F, ) -> &mut [App](../prelude/struct.App.html "struct bevy::prelude::App")

where F: [IntoFunction](../prelude/trait.IntoFunction.html "trait bevy::prelude::IntoFunction")<'static, Marker> + 'static,

Available on **crate feature `reflect_functions`** only.

Registers the given function or closure into the [`AppFunctionRegistry`](../prelude/struct.AppFunctionRegistry.html "struct bevy::prelude::AppFunctionRegistry") resource using the given name.

To avoid conflicts, it’s recommended to use a unique name for the function. This can be achieved by “namespacing” the function with a unique identifier, such as the name of your crate.

For example, to register a function, `add`, from a crate, `my_crate`, you could use the name, `"my_crate::add"`.

Another approach could be to use the [type name](https://doc.rust-lang.org/nightly/core/any/fn.type_name.html "fn core::any::type_name") of the function, however, it should be noted that anonymous functions do _not_ have unique type names.

For named functions (e.g. `fn add(a: i32, b: i32) -> i32 { a + b }`) where a custom name is not needed, it’s recommended to use [`register_function`](../prelude/struct.App.html#method.register_function "method bevy::prelude::App::register_function") instead as the generated name is guaranteed to be unique.

Only types that implement [`IntoFunction`](../prelude/trait.IntoFunction.html "trait bevy::prelude::IntoFunction") may be registered via this method.

See [`FunctionRegistry::register_with_name`](../reflect/func/struct.FunctionRegistry.html#method.register_with_name "method bevy::reflect::func::FunctionRegistry::register_with_name") for more information.

##### Panics

Panics if a function has already been registered with the given name.

##### Examples

```rust
use bevy_app::App;

fn mul(a: i32, b: i32) -> i32 {
    a * b
}

let div = |a: i32, b: i32| a / b;

App::new()
    // Registering an anonymous function with a unique name
    .register_function_with_name("my_crate::add", |a: i32, b: i32| {
        a + b
    })
    // Registering an existing function with its type name
    .register_function_with_name(std::any::type_name_of_val(&mul), mul)
    // Registering an existing function with a custom name
    .register_function_with_name("my_crate::mul", mul)
    // Be careful not to register anonymous functions with their type name.
    // This code works but registers the function with a non-unique name like `foo::bar::{{closure}}`
    .register_function_with_name(std::any::type_name_of_val(&div), div);
```

Names must be unique.

[ⓘ](# "This example panics")

```rust
use bevy_app::App;

fn one() {}
fn two() {}

App::new()
    .register_function_with_name("my_function", one)
    // Panic! A function has already been registered with the name "my_function"
    .register_function_with_name("my_function", two);
```

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/app.rs.html#975-977)

#### pub fn [register\_required\_components](#method.register_required_components)<T, R>(&mut self) -> &mut [App](../prelude/struct.App.html "struct bevy::prelude::App")

where T: [Component](../prelude/trait.Component.html "trait bevy::prelude::Component"), R: [Component](../prelude/trait.Component.html "trait bevy::prelude::Component") + [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default"),

Registers the given component `R` as a [required component](../prelude/trait.Component.html#required-components "trait bevy::prelude::Component") for `T`.

When `T` is added to an entity, `R` and its own required components will also be added if `R` was not already provided. The [`Default`](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") `constructor` will be used for the creation of `R`. If a custom constructor is desired, use [`App::register_required_components_with`](../prelude/struct.App.html#method.register_required_components_with "method bevy::prelude::App::register_required_components_with") instead.

For the non-panicking version, see [`App::try_register_required_components`](../prelude/struct.App.html#method.try_register_required_components "method bevy::prelude::App::try_register_required_components").

Note that requirements must currently be registered before `T` is inserted into the world for the first time. Commonly, this is done in plugins. This limitation may be fixed in the future.

##### Panics

Panics if `R` is already a directly required component for `T`, or if `T` has ever been added on an entity before the registration.

Indirect requirements through other components are allowed. In those cases, any existing requirements will only be overwritten if the new requirement is more specific.

##### Example

```rust
#[derive(Component)]
struct A;

#[derive(Component, Default, PartialEq, Eq, Debug)]
struct B(usize);

#[derive(Component, Default, PartialEq, Eq, Debug)]
struct C(u32);

// Register B as required by A and C as required by B.
app.register_required_components::<A, B>();
app.register_required_components::<B, C>();

fn setup(mut commands: Commands) {
    // This will implicitly also insert B and C with their Default constructors.
    commands.spawn(A);
}

fn validate(query: Option<Single<(&A, &B, &C)>>) {
    let (a, b, c) = query.unwrap().into_inner();
    assert_eq!(b, &B(0));
    assert_eq!(c, &C(0));
}
```

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/app.rs.html#1038-1041)

#### pub fn [register\_required\_components\_with](#method.register_required_components_with)<T, R>( &mut self, constructor: [fn](https://doc.rust-lang.org/nightly/std/primitive.fn.html)() -> R, ) -> &mut [App](../prelude/struct.App.html "struct bevy::prelude::App")

where T: [Component](../prelude/trait.Component.html "trait bevy::prelude::Component"), R: [Component](../prelude/trait.Component.html "trait bevy::prelude::Component"),

Registers the given component `R` as a [required component](../prelude/trait.Component.html#required-components "trait bevy::prelude::Component") for `T`.

When `T` is added to an entity, `R` and its own required components will also be added if `R` was not already provided. The given `constructor` will be used for the creation of `R`. If a [`Default`](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") constructor is desired, use [`App::register_required_components`](../prelude/struct.App.html#method.register_required_components "method bevy::prelude::App::register_required_components") instead.

For the non-panicking version, see [`App::try_register_required_components_with`](../prelude/struct.App.html#method.try_register_required_components_with "method bevy::prelude::App::try_register_required_components_with").

Note that requirements must currently be registered before `T` is inserted into the world for the first time. Commonly, this is done in plugins. This limitation may be fixed in the future.

##### Panics

Panics if `R` is already a directly required component for `T`, or if `T` has ever been added on an entity before the registration.

Indirect requirements through other components are allowed. In those cases, any existing requirements will only be overwritten if the new requirement is more specific.

##### Example

```rust
#[derive(Component)]
struct A;

#[derive(Component, Default, PartialEq, Eq, Debug)]
struct B(usize);

#[derive(Component, Default, PartialEq, Eq, Debug)]
struct C(u32);

// Register B and C as required by A and C as required by B.
// A requiring C directly will overwrite the indirect requirement through B.
app.register_required_components::<A, B>();
app.register_required_components_with::<B, C>(|| C(1));
app.register_required_components_with::<A, C>(|| C(2));

fn setup(mut commands: Commands) {
    // This will implicitly also insert B with its Default constructor and C
    // with the custom constructor defined by A.
    commands.spawn(A);
}

fn validate(query: Option<Single<(&A, &B, &C)>>) {
    let (a, b, c) = query.unwrap().into_inner();
    assert_eq!(b, &B(0));
    assert_eq!(c, &C(2));
}
```

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/app.rs.html#1103-1105)

#### pub fn [try\_register\_required\_components](#method.try_register_required_components)<T, R>( &mut self, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [RequiredComponentsError](../ecs/component/enum.RequiredComponentsError.html "enum bevy::ecs::component::RequiredComponentsError")\>

where T: [Component](../prelude/trait.Component.html "trait bevy::prelude::Component"), R: [Component](../prelude/trait.Component.html "trait bevy::prelude::Component") + [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default"),

Tries to register the given component `R` as a [required component](../prelude/trait.Component.html#required-components "trait bevy::prelude::Component") for `T`.

When `T` is added to an entity, `R` and its own required components will also be added if `R` was not already provided. The [`Default`](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") `constructor` will be used for the creation of `R`. If a custom constructor is desired, use [`App::register_required_components_with`](../prelude/struct.App.html#method.register_required_components_with "method bevy::prelude::App::register_required_components_with") instead.

For the panicking version, see [`App::register_required_components`](../prelude/struct.App.html#method.register_required_components "method bevy::prelude::App::register_required_components").

Note that requirements must currently be registered before `T` is inserted into the world for the first time. Commonly, this is done in plugins. This limitation may be fixed in the future.

##### Errors

Returns a [`RequiredComponentsError`](../ecs/component/enum.RequiredComponentsError.html "enum bevy::ecs::component::RequiredComponentsError") if `R` is already a directly required component for `T`, or if `T` has ever been added on an entity before the registration.

Indirect requirements through other components are allowed. In those cases, any existing requirements will only be overwritten if the new requirement is more specific.

##### Example

```rust
#[derive(Component)]
struct A;

#[derive(Component, Default, PartialEq, Eq, Debug)]
struct B(usize);

#[derive(Component, Default, PartialEq, Eq, Debug)]
struct C(u32);

// Register B as required by A and C as required by B.
app.register_required_components::<A, B>();
app.register_required_components::<B, C>();

// Duplicate registration! This will fail.
assert!(app.try_register_required_components::<A, B>().is_err());

fn setup(mut commands: Commands) {
    // This will implicitly also insert B and C with their Default constructors.
    commands.spawn(A);
}

fn validate(query: Option<Single<(&A, &B, &C)>>) {
    let (a, b, c) = query.unwrap().into_inner();
    assert_eq!(b, &B(0));
    assert_eq!(c, &C(0));
}
```

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/app.rs.html#1168-1171)

#### pub fn [try\_register\_required\_components\_with](#method.try_register_required_components_with)<T, R>( &mut self, constructor: [fn](https://doc.rust-lang.org/nightly/std/primitive.fn.html)() -> R, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [RequiredComponentsError](../ecs/component/enum.RequiredComponentsError.html "enum bevy::ecs::component::RequiredComponentsError")\>

where T: [Component](../prelude/trait.Component.html "trait bevy::prelude::Component"), R: [Component](../prelude/trait.Component.html "trait bevy::prelude::Component"),

Tries to register the given component `R` as a [required component](../prelude/trait.Component.html#required-components "trait bevy::prelude::Component") for `T`.

When `T` is added to an entity, `R` and its own required components will also be added if `R` was not already provided. The given `constructor` will be used for the creation of `R`. If a [`Default`](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") constructor is desired, use [`App::register_required_components`](../prelude/struct.App.html#method.register_required_components "method bevy::prelude::App::register_required_components") instead.

For the panicking version, see [`App::register_required_components_with`](../prelude/struct.App.html#method.register_required_components_with "method bevy::prelude::App::register_required_components_with").

Note that requirements must currently be registered before `T` is inserted into the world for the first time. Commonly, this is done in plugins. This limitation may be fixed in the future.

##### Errors

Returns a [`RequiredComponentsError`](../ecs/component/enum.RequiredComponentsError.html "enum bevy::ecs::component::RequiredComponentsError") if `R` is already a directly required component for `T`, or if `T` has ever been added on an entity before the registration.

Indirect requirements through other components are allowed. In those cases, any existing requirements will only be overwritten if the new requirement is more specific.

##### Example

```rust
#[derive(Component)]
struct A;

#[derive(Component, Default, PartialEq, Eq, Debug)]
struct B(usize);

#[derive(Component, Default, PartialEq, Eq, Debug)]
struct C(u32);

// Register B and C as required by A and C as required by B.
// A requiring C directly will overwrite the indirect requirement through B.
app.register_required_components::<A, B>();
app.register_required_components_with::<B, C>(|| C(1));
app.register_required_components_with::<A, C>(|| C(2));

// Duplicate registration! Even if the constructors were different, this would fail.
assert!(app.try_register_required_components_with::<B, C>(|| C(1)).is_err());

fn setup(mut commands: Commands) {
    // This will implicitly also insert B with its Default constructor and C
    // with the custom constructor defined by A.
    commands.spawn(A);
}

fn validate(query: Option<Single<(&A, &B, &C)>>) {
    let (a, b, c) = query.unwrap().into_inner();
    assert_eq!(b, &B(0));
    assert_eq!(c, &C(2));
}
```

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/app.rs.html#1183)

#### pub fn [register\_disabling\_component](#method.register_disabling_component)<C>(&mut self)

where C: [Component](../prelude/trait.Component.html "trait bevy::prelude::Component"),

Registers a component type as “disabling”, using [default query filters](../ecs/entity_disabling/struct.DefaultQueryFilters.html "struct bevy::ecs::entity_disabling::DefaultQueryFilters") to exclude entities with the component from queries.

##### Warning

As discussed in the [module docs](../ecs/entity_disabling/index.html "mod bevy::ecs::entity_disabling"), this can have performance implications, as well as create interoperability issues, and should be used with caution.

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/app.rs.html#1191)

#### pub fn [world](#method.world)(&self) -> &[World](../prelude/struct.World.html "struct bevy::prelude::World")

Returns a reference to the main [`SubApp`](../prelude/struct.SubApp.html "struct bevy::prelude::SubApp")’s [`World`](../prelude/struct.World.html "struct bevy::prelude::World"). This is the same as calling [`app.main().world()`](../prelude/struct.SubApp.html#method.world "method bevy::prelude::SubApp::world").

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/app.rs.html#1199)

#### pub fn [world\_mut](#method.world_mut)(&mut self) -> &mut [World](../prelude/struct.World.html "struct bevy::prelude::World")

Returns a mutable reference to the main [`SubApp`](../prelude/struct.SubApp.html "struct bevy::prelude::SubApp")’s [`World`](../prelude/struct.World.html "struct bevy::prelude::World"). This is the same as calling [`app.main_mut().world_mut()`](../prelude/struct.SubApp.html#method.world_mut "method bevy::prelude::SubApp::world_mut").

##### [Examples found in repository](#scraped-examples-14)[?](../../scrape-examples-help.html)

examples/shader/shader\_material\_wesl.rs ([line 43](../../src/shader_material_wesl/shader_material_wesl.rs.html#43))

```rust
41    fn build(&self, app: &mut App) {
42        let handle = app
43            .world_mut()
44            .resource_mut::<AssetServer>()
45            .load::<Shader>("shaders/util.wesl");
46        app.insert_resource(UtilityShader(handle));
47    }
```

Hide additional examples

examples/app/custom\_loop.rs ([line 19](../../src/custom_loop/custom_loop.rs.html#19))

```rust
10fn my_runner(mut app: App) -> AppExit {
11    // Finalize plugin building, including running any necessary clean-up.
12    // This is normally completed by the default runner.
13    app.finish();
14    app.cleanup();
15
16    println!("Type stuff into the console");
17    for line in io::stdin().lines() {
18        {
19            let mut input = app.world_mut().resource_mut::<Input>();
20            input.0 = line.unwrap();
21        }
22        app.update();
23
24        if let Some(exit) = app.should_exit() {
25            return exit;
26        }
27    }
28
29    AppExit::Success
30}
```

examples/gltf/gltf\_extension\_animation\_graph.rs ([line 141](../../src/gltf_extension_animation_graph/gltf_extension_animation_graph.rs.html#141))

```rust
130    fn build(&self, app: &mut App) {
131        #[cfg(target_family = "wasm")]
132        bevy::tasks::block_on(async {
133            app.world_mut()
134                .resource_mut::<GltfExtensionHandlers>()
135                .0
136                .write()
137                .await
138                .push(Box::new(GltfExtensionHandlerAnimation::default()))
139        });
140        #[cfg(not(target_family = "wasm"))]
141        app.world_mut()
142            .resource_mut::<GltfExtensionHandlers>()
143            .0
144            .write_blocking()
145            .push(Box::new(GltfExtensionHandlerAnimation::default()));
146    }
```

examples/gltf/gltf\_extension\_mesh\_2d.rs ([line 81](../../src/gltf_extension_mesh_2d/gltf_extension_mesh_2d.rs.html#81))

```rust
70    fn build(&self, app: &mut App) {
71        #[cfg(target_family = "wasm")]
72        bevy::tasks::block_on(async {
73            app.world_mut()
74                .resource_mut::<GltfExtensionHandlers>()
75                .0
76                .write()
77                .await
78                .push(Box::new(GltfExtensionHandlerToMesh2d))
79        });
80        #[cfg(not(target_family = "wasm"))]
81        app.world_mut()
82            .resource_mut::<GltfExtensionHandlers>()
83            .0
84            .write_blocking()
85            .push(Box::new(GltfExtensionHandlerToMesh2d));
86
87        app.add_plugins(Material2dPlugin::<CustomMaterial>::default());
88    }
```

examples/window/persisting\_window\_settings.rs ([line 50](../../src/persisting_window_settings/persisting_window_settings.rs.html#50))

```rust
49fn init_window_pos(app: &mut App) {
50    let world = app.world_mut();
51    let Some(window_settings) = world.get_resource::<WindowSettings>() else {
52        return;
53    };
54    let window_settings = window_settings.clone();
55
56    let Ok(mut window) = world.query::<&mut Window>().single_mut(world) else {
57        warn!("window not found");
58        return;
59    };
60
61    if let Some(position) = window_settings.position {
62        window.position = WindowPosition::new(position);
63    }
64
65    if let Some(size) = window_settings.size {
66        window.resolution = WindowResolution::new(size.x, size.y);
67    }
68
69    window.mode = if window_settings.fullscreen {
70        WindowMode::BorderlessFullscreen(MonitorSelection::Current)
71    } else {
72        WindowMode::Windowed
73    };
74}
```

examples/showcase/stepping.rs ([line 44](../../src/breakout/stepping.rs.html#44))

```rust
34    fn build(&self, app: &mut App) {
35        app.add_systems(Startup, build_stepping_hint);
36        if cfg!(not(feature = "bevy_debug_stepping")) {
37            return;
38        }
39
40        // create and insert our debug schedule into the main schedule order.
41        // We need an independent schedule so we have access to all other
42        // schedules through the `Stepping` resource
43        app.init_schedule(DebugSchedule);
44        let mut order = app.world_mut().resource_mut::<MainScheduleOrder>();
45        order.insert_after(Update, DebugSchedule);
46
47        // create our stepping resource
48        let mut stepping = Stepping::new();
49        for label in &self.schedule_labels {
50            stepping.add_schedule(*label);
51        }
52        app.insert_resource(stepping);
53
54        // add our startup & stepping systems
55        app.insert_resource(State {
56            ui_top: self.top,
57            ui_left: self.left,
58            systems: Vec::new(),
59        })
60        .add_systems(
61            DebugSchedule,
62            (
63                build_ui.run_if(not(initialized)),
64                handle_input,
65                update_ui.run_if(initialized),
66            )
67                .chain(),
68        );
69    }
```

Additional examples can be found in:  

*   [examples/2d/mesh2d\_manual.rs](../../src/mesh2d_manual/mesh2d_manual.rs.html#302)
*   [examples/time/time.rs](../../src/time/time.rs.html#50)
*   [examples/ecs/custom\_schedule.rs](../../src/custom_schedule/custom_schedule.rs.html#38)
*   [examples/stress\_tests/many\_components.rs](../../src/many_components/many_components.rs.html#81)
*   [examples/ecs/system\_stepping.rs](../../src/system_stepping/system_stepping.rs.html#59)

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/app.rs.html#1204)

#### pub fn [main](#method.main)(&self) -> &[SubApp](../prelude/struct.SubApp.html "struct bevy::prelude::SubApp")

Returns a reference to the main [`SubApp`](../prelude/struct.SubApp.html "struct bevy::prelude::SubApp").

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/app.rs.html#1209)

#### pub fn [main\_mut](#method.main_mut)(&mut self) -> &mut [SubApp](../prelude/struct.SubApp.html "struct bevy::prelude::SubApp")

Returns a mutable reference to the main [`SubApp`](../prelude/struct.SubApp.html "struct bevy::prelude::SubApp").

##### [Examples found in repository](#scraped-examples-15)[?](../../scrape-examples-help.html)

tests/ecs/ambiguity\_detection.rs ([line 32](../../src/ambiguity_detection/ambiguity_detection.rs.html#32))

```rust
14fn main() {
15    let mut app = App::new();
16    app.add_plugins(
17        DefaultPlugins
18            .build()
19            .set(RenderPlugin {
20                // llvmpipe driver can cause segfaults when aborting the binary while pipelines are being
21                // compiled (which happens very quickly in this example since we only run for a single
22                // frame). Synchronous pipeline compilation helps prevent these segfaults as the
23                // rendering thread blocks on these pipeline compilations.
24                synchronous_pipeline_compilation: true,
25                ..Default::default()
26            })
27            // We also have to disable pipelined rendering to ensure the test doesn't end while the
28            // rendering frame is still executing in another thread.
29            .disable::<PipelinedRenderingPlugin>(),
30    );
31
32    let main_app = app.main_mut();
33    configure_ambiguity_detection(main_app);
34
35    let sub_app = app.sub_app_mut(bevy_render::RenderApp);
36    configure_ambiguity_detection(sub_app);
37
38    // Make sure all the system stuff is added.
39    app.finish();
40    app.cleanup();
41
42    let main_app_ambiguities = count_ambiguities(app.main_mut());
43    assert_eq!(
44        main_app_ambiguities.total(),
45        0,
46        "Main app has unexpected ambiguities among the following schedules: \n{main_app_ambiguities:#?}.",
47    );
48
49    let render_app = app.sub_app_mut(bevy_render::RenderApp);
50    // Initialize the MainWorld so the render world systems don't fail initialization.
51    render_app.init_resource::<bevy_render::MainWorld>();
52    let render_app_ambiguities = count_ambiguities(render_app);
53    assert_eq!(
54        render_app_ambiguities.total(),
55        0,
56        "Render app has unexpected ambiguities among the following schedules: \n{render_app_ambiguities:#?}.",
57    );
58}
```

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/app.rs.html#1214)

#### pub fn [sub\_apps](#method.sub_apps)(&self) -> &[SubApps](struct.SubApps.html "struct bevy::app::SubApps")

Returns a reference to the [`SubApps`](struct.SubApps.html "struct bevy::app::SubApps") collection.

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/app.rs.html#1219)

#### pub fn [sub\_apps\_mut](#method.sub_apps_mut)(&mut self) -> &mut [SubApps](struct.SubApps.html "struct bevy::app::SubApps")

Returns a mutable reference to the [`SubApps`](struct.SubApps.html "struct bevy::app::SubApps") collection.

##### [Examples found in repository](#scraped-examples-16)[?](../../scrape-examples-help.html)

examples/app/externally\_driven\_headless\_renderer.rs ([line 72](../../src/externally_driven_headless_renderer/externally_driven_headless_renderer.rs.html#72))

```rust
39    fn new() -> Self {
40        let render_plugin = RenderPlugin {
41            // Make sure all shaders are loaded for the first frame
42            synchronous_pipeline_compilation: true,
43            ..default()
44        };
45        // We don't have any windows, but the WindowPlugin is still needed
46        // because a lot of bevy expects it to be there. Just configure it
47        // to not have any windows and not exit automatically.
48        let window_plugin = WindowPlugin {
49            primary_window: None,
50            exit_condition: ExitCondition::DontExit,
51            ..default()
52        };
53
54        let mut app = App::new();
55        app.add_plugins(
56            DefaultPlugins
57                .set(window_plugin)
58                .set(render_plugin)
59                // Disable winit because we want to own the update loop ourselves.
60                .disable::<WinitPlugin>(),
61        )
62        .add_systems(Startup, spawn_test_scene)
63        .add_systems(Update, update_camera);
64
65        // We yeet the schedule runner and never call app.run(),
66        // so we have to finish and clean up ourselves
67        app.finish();
68        app.cleanup();
69
70        // We grab the sub apps cus we dont want the runner, as we'll
71        // be pumping the update loop ourselves manually.
72        Self(std::mem::take(app.sub_apps_mut()))
73    }
```

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/app.rs.html#1228)

#### pub fn [sub\_app](#method.sub_app)(&self, label: impl [AppLabel](trait.AppLabel.html "trait bevy::app::AppLabel")) -> &[SubApp](../prelude/struct.SubApp.html "struct bevy::prelude::SubApp")

Returns a reference to the [`SubApp`](../prelude/struct.SubApp.html "struct bevy::prelude::SubApp") with the given label.

##### Panics

Panics if the [`SubApp`](../prelude/struct.SubApp.html "struct bevy::prelude::SubApp") doesn’t exist.

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/app.rs.html#1240)

#### pub fn [sub\_app\_mut](#method.sub_app_mut)(&mut self, label: impl [AppLabel](trait.AppLabel.html "trait bevy::app::AppLabel")) -> &mut [SubApp](../prelude/struct.SubApp.html "struct bevy::prelude::SubApp")

Returns a reference to the [`SubApp`](../prelude/struct.SubApp.html "struct bevy::prelude::SubApp") with the given label.

##### Panics

Panics if the [`SubApp`](../prelude/struct.SubApp.html "struct bevy::prelude::SubApp") doesn’t exist.

##### [Examples found in repository](#scraped-examples-17)[?](../../scrape-examples-help.html)

examples/app/render\_recovery.rs ([line 29](../../src/render_recovery/render_recovery.rs.html#29))

```rust
20fn main() {
21    let mut app = App::new();
22    app.add_plugins((
23        DefaultPlugins,
24        ExtractResourcePlugin::<RenderError>::default(),
25    ))
26    .add_systems(Startup, setup)
27    .add_systems(Update, (update_camera, input))
28    .init_resource::<RenderError>()
29    .sub_app_mut(RenderApp)
30    .add_systems(Render, cause_error);
31    app.run();
32}
```

Hide additional examples

examples/showcase/loading\_screen.rs ([line 292](../../src/loading_screen/loading_screen.rs.html#292))

```rust
284        fn build(&self, app: &mut App) {
285            app.insert_resource(PipelinesReady::default());
286
287            // In order to gain access to the pipelines status, we have to
288            // go into the `RenderApp`, grab the resource from the main App
289            // and then update the pipelines status from there.
290            // Writing between these Apps can only be done through the
291            // `ExtractSchedule`.
292            app.sub_app_mut(RenderApp)
293                .add_systems(ExtractSchedule, update_pipelines_ready);
294        }
```

examples/shader\_advanced/custom\_shader\_instancing.rs ([line 112](../../src/custom_shader_instancing/custom_shader_instancing.rs.html#112))

```rust
110    fn build(&self, app: &mut App) {
111        app.add_plugins(ExtractComponentPlugin::<InstanceMaterialData>::default());
112        app.sub_app_mut(RenderApp)
113            .add_render_command::<Transparent3d, DrawCustom>()
114            .init_resource::<SpecializedMeshPipelines<CustomPipeline>>()
115            .add_systems(
116                RenderStartup,
117                init_custom_pipeline.after(MeshPipelineSystems),
118            )
119            .add_systems(
120                Render,
121                (
122                    queue_custom.in_set(RenderSystems::QueueMeshes),
123                    prepare_instance_buffers.in_set(RenderSystems::PrepareResources),
124                ),
125            );
126    }
```

examples/shader\_advanced/custom\_phase\_item.rs ([line 171](../../src/custom_phase_item/custom_phase_item.rs.html#171))

```rust
164fn main() {
165    let mut app = App::new();
166    app.add_plugins(DefaultPlugins)
167        .add_plugins(ExtractComponentPlugin::<CustomRenderedEntity>::default())
168        .add_systems(Startup, setup);
169
170    // We make sure to add these to the render app, not the main app.
171    app.sub_app_mut(RenderApp)
172        .init_resource::<CustomPhasePipeline>()
173        .init_resource::<PendingCustomPhaseItemQueues>()
174        .add_render_command::<Opaque3d, DrawCustomPhaseItemCommands>()
175        .add_systems(
176            Render,
177            prepare_custom_phase_item_buffers.in_set(RenderSystems::Prepare),
178        )
179        .add_systems(Render, queue_custom_phase_item.in_set(RenderSystems::Queue));
180
181    app.run();
182}
```

examples/app/headless\_renderer.rs ([line 210](../../src/headless_renderer/headless_renderer.rs.html#210))

```rust
205    fn build(&self, app: &mut App) {
206        let (s, r) = crossbeam_channel::unbounded();
207
208        let render_app = app
209            .insert_resource(MainWorldReceiver(r))
210            .sub_app_mut(RenderApp);
211
212        render_app
213            .insert_resource(RenderWorldSender(s))
214            // Make ImageCopiers accessible in RenderWorld system and plugin
215            .add_systems(ExtractSchedule, image_copy_extract)
216            // Receives image data from buffer to channel
217            // so we need to run it after the render graph is done
218            .add_systems(
219                Render,
220                receive_image_from_buffer.after(RenderSystems::Render),
221            )
222            .add_systems(RenderGraph, image_copy_driver);
223    }
```

examples/shader/compute\_shader\_game\_of\_life.rs ([line 101](../../src/compute_shader_game_of_life/compute_shader_game_of_life.rs.html#101))

```rust
94    fn build(&self, app: &mut App) {
95        // Extract the game of life image resource from the main world into the render world
96        // for operation on by the compute shader and display on the sprite.
97        app.add_plugins((
98            ExtractResourcePlugin::<GameOfLifeImages>::default(),
99            ExtractResourcePlugin::<GameOfLifeUniforms>::default(),
100        ));
101        let render_app = app.sub_app_mut(RenderApp);
102        render_app
103            .init_resource::<GameOfLifeState>()
104            .add_systems(RenderStartup, init_game_of_life_pipeline)
105            .add_systems(
106                Render,
107                prepare_bind_group.in_set(RenderSystems::PrepareBindGroups),
108            )
109            .add_systems(Render, update.in_set(RenderSystems::Prepare))
110            .add_systems(RenderGraph, game_of_life.before(camera_driver));
111    }
```

Additional examples can be found in:  

*   [tests/ecs/ambiguity\_detection.rs](../../src/ambiguity_detection/ambiguity_detection.rs.html#35)

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/app.rs.html#1248)

#### pub fn [get\_sub\_app](#method.get_sub_app)(&self, label: impl [AppLabel](trait.AppLabel.html "trait bevy::app::AppLabel")) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&[SubApp](../prelude/struct.SubApp.html "struct bevy::prelude::SubApp")\>

Returns a reference to the [`SubApp`](../prelude/struct.SubApp.html "struct bevy::prelude::SubApp") with the given label, if it exists.

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/app.rs.html#1253)

#### pub fn [get\_sub\_app\_mut](#method.get_sub_app_mut)(&mut self, label: impl [AppLabel](trait.AppLabel.html "trait bevy::app::AppLabel")) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&mut [SubApp](../prelude/struct.SubApp.html "struct bevy::prelude::SubApp")\>

Returns a mutable reference to the [`SubApp`](../prelude/struct.SubApp.html "struct bevy::prelude::SubApp") with the given label, if it exists.

##### [Examples found in repository](#scraped-examples-18)[?](../../scrape-examples-help.html)

examples/shader\_advanced/texture\_binding\_array.rs ([line 45](../../src/texture_binding_array/texture_binding_array.rs.html#45))

```rust
44    fn build(&self, app: &mut App) {
45        let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
46            return;
47        };
48
49        render_app.add_systems(RenderStartup, verify_required_features);
50    }
```

Hide additional examples

examples/stress\_tests/many\_lights.rs ([line 154](../../src/many_lights/many_lights.rs.html#154))

```rust
153    fn build(&self, app: &mut App) {
154        let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
155            return;
156        };
157
158        render_app.add_systems(
159            Render,
160            print_visible_light_count.in_set(RenderSystems::Prepare),
161        );
162    }
```

examples/shader\_advanced/compute\_mesh.rs ([line 52](../../src/compute_mesh/compute_mesh.rs.html#52))

```rust
51    fn build(&self, app: &mut App) {
52        let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
53            return;
54        };
55
56        render_app
57            .init_resource::<ChunksToProcess>()
58            .add_systems(RenderStartup, init_compute_pipeline)
59            .add_systems(Render, prepare_chunks)
60            .add_systems(RenderGraph, compute_mesh.before(camera_driver));
61    }
62    fn finish(&self, app: &mut App) {
63        let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
64            return;
65        };
66        render_app
67            .world_mut()
68            .resource_mut::<MeshAllocatorSettings>()
69            // This allows using the mesh allocator slabs as
70            // storage buffers directly in the compute shader.
71            // Which means that we can write from our compute
72            // shader directly to the allocated mesh slabs.
73            .extra_buffer_usages = BufferUsages::STORAGE;
74    }
```

examples/shader/gpu\_readback.rs ([line 45](../../src/gpu_readback/gpu_readback.rs.html#45))

```rust
44    fn build(&self, app: &mut App) {
45        let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
46            return;
47        };
48        render_app
49            .add_systems(RenderStartup, init_compute_pipeline)
50            .add_systems(
51                Render,
52                prepare_bind_group
53                    .in_set(RenderSystems::PrepareBindGroups)
54                    // We don't need to recreate the bind group every frame
55                    .run_if(not(resource_exists::<GpuBufferBindGroup>)),
56            )
57            .add_systems(RenderGraph, compute);
58    }
```

examples/ecs/extraction.rs ([line 60](../../src/extraction/extraction.rs.html#60))

```rust
46fn main() {
47    let mut app = App::new();
48
49    // Main World
50    app.insert_resource(WorldName("Main World".into()))
51        .add_plugins((
52            DefaultPlugins,
53            // Plugin for automatically extracting A.
54            ExtractComponentPlugin::<A>::default(),
55        ))
56        .add_message::<ExtractMessage>()
57        .add_systems(Startup, setup)
58        .add_systems(Update, (set_time, trigger_extraction, display_state));
59
60    let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
61        return;
62    };
63
64    // Render World
65    render_app
66        .insert_resource(WorldName("Render World".into()))
67        .add_systems(ExtractSchedule, extract_components)
68        .add_systems(Render, display_state);
69
70    app.run();
71}
```

examples/shader\_advanced/render\_depth\_to\_texture.rs ([line 114](../../src/render_depth_to_texture/render_depth_to_texture.rs.html#114))

```rust
101fn main() {
102    let mut app = App::new();
103
104    app.add_plugins(DefaultPlugins)
105        .add_plugins(MaterialPlugin::<ShowDepthTextureMaterial>::default())
106        .add_plugins(ExtractResourcePlugin::<DemoDepthTexture>::default())
107        .init_resource::<DemoDepthTexture>()
108        .add_systems(Startup, setup)
109        .add_systems(Update, rotate_cube)
110        .add_systems(Update, draw_camera_gizmo)
111        .add_systems(Update, move_camera);
112
113    let render_app = app
114        .get_sub_app_mut(RenderApp)
115        .expect("Render app should be present");
116
117    render_app.add_systems(
118        Core3d,
119        copy_depth_texture_system
120            .after(Core3dSystems::Prepass)
121            .before(Core3dSystems::MainPass),
122    );
123
124    app.run();
125}
```

Additional examples can be found in:  

*   [examples/shader\_advanced/specialized\_mesh\_pipeline.rs](../../src/specialized_mesh_pipeline/specialized_mesh_pipeline.rs.html#110)
*   [examples/shader\_advanced/manual\_material.rs](../../src/manual_material/manual_material.rs.html#60)
*   [examples/shader\_advanced/custom\_post\_processing.rs](../../src/custom_post_processing/custom_post_processing.rs.html#58)
*   [examples/2d/mesh2d\_manual.rs](../../src/mesh2d_manual/mesh2d_manual.rs.html#310)
*   [examples/shader\_advanced/custom\_render\_phase.rs](../../src/custom_render_phase/custom_render_phase.rs.html#125)
*   [examples/2d/dynamic\_mip\_generation.rs](../../src/dynamic_mip_generation/dynamic_mip_generation.rs.html#248)
*   [examples/3d/occlusion\_culling.rs](../../src/occlusion_culling/occlusion_culling.rs.html#208)

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/app.rs.html#1258)

#### pub fn [insert\_sub\_app](#method.insert_sub_app)(&mut self, label: impl [AppLabel](trait.AppLabel.html "trait bevy::app::AppLabel"), sub\_app: [SubApp](../prelude/struct.SubApp.html "struct bevy::prelude::SubApp"))

Inserts a [`SubApp`](../prelude/struct.SubApp.html "struct bevy::prelude::SubApp") with the given label.

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/app.rs.html#1268)

#### pub fn [remove\_sub\_app](#method.remove_sub_app)(&mut self, label: impl [AppLabel](trait.AppLabel.html "trait bevy::app::AppLabel")) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[SubApp](../prelude/struct.SubApp.html "struct bevy::prelude::SubApp")\>

Removes the [`SubApp`](../prelude/struct.SubApp.html "struct bevy::prelude::SubApp") with the given label, if it exists.

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/app.rs.html#1273)

#### pub fn [update\_sub\_app\_by\_label](#method.update_sub_app_by_label)(&mut self, label: impl [AppLabel](trait.AppLabel.html "trait bevy::app::AppLabel"))

Extract data from the main world into the [`SubApp`](../prelude/struct.SubApp.html "struct bevy::prelude::SubApp") with the given label and perform an update if it exists.

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/app.rs.html#1279)

#### pub fn [add\_schedule](#method.add_schedule)(&mut self, schedule: [Schedule](../prelude/struct.Schedule.html "struct bevy::prelude::Schedule")) -> &mut [App](../prelude/struct.App.html "struct bevy::prelude::App")

Inserts a new `schedule` under the provided `label`, overwriting any existing schedule with the same label.

##### [Examples found in repository](#scraped-examples-19)[?](../../scrape-examples-help.html)

examples/ecs/custom\_schedule.rs ([line 27](../../src/custom_schedule/custom_schedule.rs.html#27))

```rust
16fn main() {
17    let mut app = App::new();
18
19    // Create a new [`Schedule`]. For demonstration purposes, we configure it to use a single threaded executor so that
20    // systems in this schedule are never run in parallel. However, this is not a requirement for custom schedules in
21    // general.
22    let mut custom_update_schedule = Schedule::new(SingleThreadedUpdate);
23    custom_update_schedule.set_executor(SingleThreadedExecutor::new());
24
25    // Adding the schedule to the app does not automatically run the schedule. This merely registers the schedule so
26    // that systems can look it up using the `Schedules` resource.
27    app.add_schedule(custom_update_schedule);
28
29    // Bevy `App`s have a `main_schedule_label` field that configures which schedule is run by the App's `runner`.
30    // By default, this is `Main`. The `Main` schedule is responsible for running Bevy's main schedules such as
31    // `Update`, `Startup` or `Last`.
32    //
33    // We can configure the `Main` schedule to run our custom update schedule relative to the existing ones by modifying
34    // the `MainScheduleOrder` resource.
35    //
36    // Note that we modify `MainScheduleOrder` directly in `main` and not in a startup system. The reason for this is
37    // that the `MainScheduleOrder` cannot be modified from systems that are run as part of the `Main` schedule.
38    let mut main_schedule_order = app.world_mut().resource_mut::<MainScheduleOrder>();
39    main_schedule_order.insert_after(Update, SingleThreadedUpdate);
40
41    // Adding a custom startup schedule works similarly, but needs to use `insert_startup_after`
42    // instead of `insert_after`.
43    app.add_schedule(Schedule::new(CustomStartup));
44
45    let mut main_schedule_order = app.world_mut().resource_mut::<MainScheduleOrder>();
46    main_schedule_order.insert_startup_after(PreStartup, CustomStartup);
47
48    app.add_systems(SingleThreadedUpdate, single_threaded_update_system)
49        .add_systems(CustomStartup, custom_startup_system)
50        .add_systems(PreStartup, pre_startup_system)
51        .add_systems(Startup, startup_system)
52        .add_systems(First, first_system)
53        .add_systems(Update, update_system)
54        .add_systems(Last, last_system)
55        .run();
56}
```

Hide additional examples

examples/stress\_tests/many\_components.rs ([line 161](../../src/many_components/many_components.rs.html#161))

```rust
78fn stress_test(num_entities: u32, num_components: u32, num_systems: u32) {
79    let mut rng = ChaCha8Rng::seed_from_u64(42);
80    let mut app = App::default();
81    let world = app.world_mut();
82
83    // register a bunch of components
84    let component_ids: Vec<ComponentId> = (1..=num_components)
85        .map(|i| {
86            world.register_component_with_descriptor(
87                // SAFETY:
88                // * We don't implement a drop function
89                // * u8 is Sync and Send
90                unsafe {
91                    ComponentDescriptor::new_with_layout(
92                        format!("Component{i}").to_string(),
93                        StorageType::Table,
94                        Layout::new::<u8>(),
95                        None,
96                        true, // is mutable
97                        ComponentCloneBehavior::Default,
98                        None,
99                    )
100                },
101            )
102        })
103        .collect();
104
105    // fill the schedule with systems
106    let mut schedule = Schedule::new(Update);
107    for _ in 1..=num_systems {
108        let num_access_components = rng.random_range(1..10);
109        let access_components: Vec<ComponentId> = component_ids
110            .sample(&mut rng, num_access_components)
111            .copied()
112            .collect();
113        let system = (QueryParamBuilder::new(|builder| {
114            for &access_component in &access_components {
115                if rand::random::<bool>() {
116                    builder.mut_id(access_component);
117                } else {
118                    builder.ref_id(access_component);
119                }
120            }
121        }),)
122            .build_state(world)
123            .build_any_system(base_system);
124        schedule.add_systems((move || access_components.clone()).pipe(system));
125    }
126
127    // spawn a bunch of entities
128    for _ in 1..=num_entities {
129        let num_components = rng.random_range(1..10);
130        let components: Vec<ComponentId> = component_ids
131            .sample(&mut rng, num_components)
132            .copied()
133            .collect();
134
135        let mut entity = world.spawn_empty();
136        // We use `ManuallyDrop` here as we need to avoid dropping the u8's when `values` is dropped
137        // since ownership of the values is passed to the world in `insert_by_ids`.
138        // But we do want to deallocate the memory when values is dropped.
139        let mut values: Vec<ManuallyDrop<u8>> = components
140            .iter()
141            .map(|_id| ManuallyDrop::new(rng.random_range(0..255)))
142            .collect();
143        let ptrs: Vec<OwningPtr> = values
144            .iter_mut()
145            .map(|value| {
146                // SAFETY:
147                // * We don't read/write `values` binding after this and values are `ManuallyDrop`,
148                // so we have the right to drop/move the values
149                unsafe { PtrMut::from(value).promote() }
150            })
151            .collect();
152        // SAFETY:
153        // * component_id's are from the same world
154        // * `values` was initialized above, so references are valid
155        unsafe {
156            entity.insert_by_ids(&components, ptrs.into_iter());
157        }
158    }
159
160    // overwrite Update schedule in the app
161    app.add_schedule(schedule);
162    app.add_plugins(MinimalPlugins)
163        .add_plugins(DiagnosticsPlugin)
164        .add_plugins(LogPlugin::default())
165        .add_plugins(FrameTimeDiagnosticsPlugin::default())
166        .add_plugins(LogDiagnosticsPlugin::filtered(HashSet::from_iter([
167            DiagnosticPath::new("fps"),
168        ])));
169    app.run();
170}
```

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/app.rs.html#1287)

#### pub fn [init\_schedule](#method.init_schedule)(&mut self, label: impl [ScheduleLabel](../ecs/schedule/trait.ScheduleLabel.html "trait bevy::ecs::schedule::ScheduleLabel")) -> &mut [App](../prelude/struct.App.html "struct bevy::prelude::App")

Initializes an empty `schedule` under the provided `label`, if it does not exist.

See [`add_schedule`](../prelude/struct.App.html#method.add_schedule "method bevy::prelude::App::add_schedule") to insert an existing schedule.

##### [Examples found in repository](#scraped-examples-20)[?](../../scrape-examples-help.html)

examples/showcase/stepping.rs ([line 43](../../src/breakout/stepping.rs.html#43))

```rust
34    fn build(&self, app: &mut App) {
35        app.add_systems(Startup, build_stepping_hint);
36        if cfg!(not(feature = "bevy_debug_stepping")) {
37            return;
38        }
39
40        // create and insert our debug schedule into the main schedule order.
41        // We need an independent schedule so we have access to all other
42        // schedules through the `Stepping` resource
43        app.init_schedule(DebugSchedule);
44        let mut order = app.world_mut().resource_mut::<MainScheduleOrder>();
45        order.insert_after(Update, DebugSchedule);
46
47        // create our stepping resource
48        let mut stepping = Stepping::new();
49        for label in &self.schedule_labels {
50            stepping.add_schedule(*label);
51        }
52        app.insert_resource(stepping);
53
54        // add our startup & stepping systems
55        app.insert_resource(State {
56            ui_top: self.top,
57            ui_left: self.left,
58            systems: Vec::new(),
59        })
60        .add_systems(
61            DebugSchedule,
62            (
63                build_ui.run_if(not(initialized)),
64                handle_input,
65                update_ui.run_if(initialized),
66            )
67                .chain(),
68        );
69    }
```

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/app.rs.html#1293)

#### pub fn [get\_schedule](#method.get_schedule)(&self, label: impl [ScheduleLabel](../ecs/schedule/trait.ScheduleLabel.html "trait bevy::ecs::schedule::ScheduleLabel")) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&[Schedule](../prelude/struct.Schedule.html "struct bevy::prelude::Schedule")\>

Returns a reference to the [`Schedule`](../prelude/struct.Schedule.html "struct bevy::prelude::Schedule") with the provided `label` if it exists.

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/app.rs.html#1298)

#### pub fn [get\_schedule\_mut](#method.get_schedule_mut)( &mut self, label: impl [ScheduleLabel](../ecs/schedule/trait.ScheduleLabel.html "trait bevy::ecs::schedule::ScheduleLabel"), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&mut [Schedule](../prelude/struct.Schedule.html "struct bevy::prelude::Schedule")\>

Returns a mutable reference to the [`Schedule`](../prelude/struct.Schedule.html "struct bevy::prelude::Schedule") with the provided `label` if it exists.

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/app.rs.html#1305-1309)

#### pub fn [edit\_schedule](#method.edit_schedule)( &mut self, label: impl [ScheduleLabel](../ecs/schedule/trait.ScheduleLabel.html "trait bevy::ecs::schedule::ScheduleLabel"), f: impl [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(&mut [Schedule](../prelude/struct.Schedule.html "struct bevy::prelude::Schedule")), ) -> &mut [App](../prelude/struct.App.html "struct bevy::prelude::App")

Runs function `f` with the [`Schedule`](../prelude/struct.Schedule.html "struct bevy::prelude::Schedule") associated with `label`.

**Note:** This will create the schedule if it does not already exist.

##### [Examples found in repository](#scraped-examples-21)[?](../../scrape-examples-help.html)

examples/ecs/nondeterministic\_system\_order.rs ([lines 25-30](../../src/nondeterministic_system_order/nondeterministic_system_order.rs.html#25-30))

```rust
20fn main() {
21    App::new()
22        // We can modify the reporting strategy for system execution order ambiguities on a per-schedule basis.
23        // You must do this for each schedule you want to inspect; child schedules executed within an inspected
24        // schedule do not inherit this modification.
25        .edit_schedule(Update, |schedule| {
26            schedule.set_build_settings(ScheduleBuildSettings {
27                ambiguity_detection: LogLevel::Warn,
28                ..default()
29            });
30        })
31        .init_resource::<A>()
32        .init_resource::<B>()
33        .add_systems(
34            Update,
35            (
36                // This pair of systems has an ambiguous order,
37                // as their data access conflicts, and there's no order between them.
38                reads_a,
39                writes_a,
40                // This pair of systems has conflicting data access,
41                // but it's resolved with an explicit ordering:
42                // the .after relationship here means that we will always double after adding.
43                adds_one_to_b,
44                doubles_b.after(adds_one_to_b),
45                // This system isn't ambiguous with adds_one_to_b,
46                // due to the transitive ordering created by our constraints:
47                // if A is before B is before C, then A must be before C as well.
48                reads_b.after(doubles_b),
49                // This system will conflict with all of our writing systems
50                // but we've silenced its ambiguity with adds_one_to_b.
51                // This should only be done in the case of clear false positives:
52                // leave a comment in your code justifying the decision!
53                reads_a_and_b.ambiguous_with(adds_one_to_b),
54            ),
55        )
56        // Be mindful, internal ambiguities are reported too!
57        // If there are any ambiguities due solely to DefaultPlugins,
58        // or between DefaultPlugins and any of your third party plugins,
59        // please file a bug with the repo responsible!
60        // Only *you* can prevent nondeterministic bugs due to greedy parallelism.
61        .add_plugins(DefaultPlugins)
62        .run();
63}
```

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/app.rs.html#1318-1321)

#### pub fn [configure\_schedules](#method.configure_schedules)( &mut self, schedule\_build\_settings: [ScheduleBuildSettings](../ecs/schedule/struct.ScheduleBuildSettings.html "struct bevy::ecs::schedule::ScheduleBuildSettings"), ) -> &mut [App](../prelude/struct.App.html "struct bevy::prelude::App")

Applies the provided [`ScheduleBuildSettings`](../ecs/schedule/struct.ScheduleBuildSettings.html "struct bevy::ecs::schedule::ScheduleBuildSettings") to all schedules.

This mutates all currently present schedules, but does not apply to any custom schedules that might be added in the future.

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/app.rs.html#1359)

#### pub fn [allow\_ambiguous\_component](#method.allow_ambiguous_component)<T>(&mut self) -> &mut [App](../prelude/struct.App.html "struct bevy::prelude::App")

where T: [Component](../prelude/trait.Component.html "trait bevy::prelude::Component"),

When doing [ambiguity checking](../ecs/schedule/struct.ScheduleBuildSettings.html "struct bevy::ecs::schedule::ScheduleBuildSettings") this ignores systems that are ambiguous on [`Component`](../prelude/trait.Component.html "trait bevy::prelude::Component") T.

This settings only applies to the main world. To apply this to other worlds call the [corresponding method](../prelude/struct.World.html#method.allow_ambiguous_component "method bevy::prelude::World::allow_ambiguous_component") on World

###### Example

```rust
#[derive(Component)]
struct A;

// these systems are ambiguous on A
fn system_1(_: Query<&mut A>) {}
fn system_2(_: Query<&A>) {}

let mut app = App::new();
app.configure_schedules(ScheduleBuildSettings {
  ambiguity_detection: LogLevel::Error,
  ..default()
});

app.add_systems(Update, ( system_1, system_2 ));
app.allow_ambiguous_component::<A>();

// running the app does not error.
app.update();
```

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/app.rs.html#1398)

#### pub fn [allow\_ambiguous\_resource](#method.allow_ambiguous_resource)<T>(&mut self) -> &mut [App](../prelude/struct.App.html "struct bevy::prelude::App")

where T: [Resource](../prelude/trait.Resource.html "trait bevy::prelude::Resource"),

When doing [ambiguity checking](../ecs/schedule/struct.ScheduleBuildSettings.html "struct bevy::ecs::schedule::ScheduleBuildSettings") this ignores systems that are ambiguous on [`Resource`](../prelude/trait.Resource.html "trait bevy::prelude::Resource") T.

This settings only applies to the main world. To apply this to other worlds call the [corresponding method](../prelude/struct.World.html#method.allow_ambiguous_resource "method bevy::prelude::World::allow_ambiguous_resource") on World

###### Example

```rust
#[derive(Resource)]
struct R;

// these systems are ambiguous on R
fn system_1(_: ResMut<R>) {}
fn system_2(_: Res<R>) {}

let mut app = App::new();
app.configure_schedules(ScheduleBuildSettings {
  ambiguity_detection: LogLevel::Error,
  ..default()
});
app.insert_resource(R);

app.add_systems(Update, ( system_1, system_2 ));
app.allow_ambiguous_resource::<R>();

// running the app does not error.
app.update();
```

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/app.rs.html#1410-1418)

#### pub fn [ignore\_ambiguity](#method.ignore_ambiguity)<M1, M2, S1, S2>( &mut self, schedule: impl [ScheduleLabel](../ecs/schedule/trait.ScheduleLabel.html "trait bevy::ecs::schedule::ScheduleLabel"), a: S1, b: S2, ) -> &mut [App](../prelude/struct.App.html "struct bevy::prelude::App")

where S1: [IntoSystemSet](../prelude/trait.IntoSystemSet.html "trait bevy::prelude::IntoSystemSet")<M1>, S2: [IntoSystemSet](../prelude/trait.IntoSystemSet.html "trait bevy::prelude::IntoSystemSet")<M2>,

Suppress warnings and errors that would result from systems in these sets having ambiguities (conflicting access but indeterminate order) with systems in `set`.

When possible, do this directly in the `.add_systems(Update, a.ambiguous_with(b))` call. However, sometimes two independent plugins `A` and `B` are reported as ambiguous, which you can only suppress as the consumer of both.

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/app.rs.html#1429)

#### pub fn [should\_exit](#method.should_exit)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[AppExit](../prelude/enum.AppExit.html "enum bevy::prelude::AppExit")\>

Attempts to determine if an [`AppExit`](../prelude/enum.AppExit.html "enum bevy::prelude::AppExit") was raised since the last update.

Will attempt to return the first [`Error`](../prelude/enum.AppExit.html#variant.Error "variant bevy::prelude::AppExit::Error") it encounters. This should be called after every [`update()`](../prelude/struct.App.html#method.update "method bevy::prelude::App::update") otherwise you risk dropping possible [`AppExit`](../prelude/enum.AppExit.html "enum bevy::prelude::AppExit") events.

##### [Examples found in repository](#scraped-examples-22)[?](../../scrape-examples-help.html)

examples/app/custom\_loop.rs ([line 24](../../src/custom_loop/custom_loop.rs.html#24))

```rust
10fn my_runner(mut app: App) -> AppExit {
11    // Finalize plugin building, including running any necessary clean-up.
12    // This is normally completed by the default runner.
13    app.finish();
14    app.cleanup();
15
16    println!("Type stuff into the console");
17    for line in io::stdin().lines() {
18        {
19            let mut input = app.world_mut().resource_mut::<Input>();
20            input.0 = line.unwrap();
21        }
22        app.update();
23
24        if let Some(exit) = app.should_exit() {
25            return exit;
26        }
27    }
28
29    AppExit::Success
30}
```

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/app.rs.html#1482)

#### pub fn [add\_observer](#method.add_observer)<M>(&mut self, observer: impl [IntoObserver](../ecs/observer/trait.IntoObserver.html "trait bevy::ecs::observer::IntoObserver")<M>) -> &mut [App](../prelude/struct.App.html "struct bevy::prelude::App")

Spawns an [`Observer`](../prelude/struct.Observer.html "struct bevy::prelude::Observer") entity, which will watch for and respond to the given event.

`observer` can be any system whose first parameter is [`On`](../prelude/struct.On.html "struct bevy::prelude::On").

##### Examples

```rust
app.add_observer(|event: On<Party>, friends: Query<Entity, With<Friend>>, mut commands: Commands| {
    if event.friends_allowed {
        for entity in friends.iter() {
            commands.trigger(Invite { entity } );
        }
    }
});
```

##### [Examples found in repository](#scraped-examples-23)[?](../../scrape-examples-help.html)

examples/ecs/delayed\_commands.rs ([line 12](../../src/delayed_commands/delayed_commands.rs.html#12))

```rust
8fn main() {
9    App::new()
10        .add_plugins(DefaultPlugins)
11        .add_systems(Startup, spawn)
12        .add_observer(click)
13        .run();
14}
```

Hide additional examples

examples/gltf/edit\_material\_on\_gltf.rs ([line 12](../../src/edit_material_on_gltf/edit_material_on_gltf.rs.html#12))

```rust
8fn main() {
9    App::new()
10        .add_plugins(DefaultPlugins.build().disable::<AudioPlugin>())
11        .add_systems(Startup, setup_scene)
12        .add_observer(change_material)
13        .run();
14}
```

examples/animation/animation\_events.rs ([line 15](../../src/animation_events/animation_events.rs.html#15))

```rust
10fn main() {
11    App::new()
12        .add_plugins(DefaultPlugins)
13        .add_systems(Startup, setup)
14        .add_systems(Update, animate_text_opacity)
15        .add_observer(on_set_message)
16        .run();
17}
```

examples/ui/scroll\_and\_overflow/scroll.rs ([line 19](../../src/scroll/scroll.rs.html#19))

```rust
13fn main() {
14    let mut app = App::new();
15
16    app.add_plugins(DefaultPlugins)
17        .add_systems(Startup, setup)
18        .add_systems(Update, send_scroll_events)
19        .add_observer(on_scroll_handler);
20
21    app.run();
22}
```

examples/ecs/entity\_disabling.rs ([line 26](../../src/entity_disabling/entity_disabling.rs.html#26))

```rust
23fn main() {
24    App::new()
25        .add_plugins((DefaultPlugins, MeshPickingPlugin))
26        .add_observer(disable_entities_on_click)
27        .add_systems(
28            Update,
29            (list_all_named_entities, reenable_entities_on_space),
30        )
31        .add_systems(Startup, (setup_scene, display_instructions))
32        .run();
33}
```

examples/ecs/removal\_detection.rs ([line 20](../../src/removal_detection/removal_detection.rs.html#20))

```rust
13fn main() {
14    App::new()
15        .add_plugins(DefaultPlugins)
16        .add_systems(Startup, setup)
17        // This system will remove a component after two seconds.
18        .add_systems(Update, remove_component)
19        // This observer will react to the removal of the component.
20        .add_observer(react_on_removal)
21        .run();
22}
```

Additional examples can be found in:  

*   [examples/usage/context\_menu.rs](../../src/context_menu/context_menu.rs.html#32)
*   [examples/asset/asset\_saving.rs](../../src/asset_saving/asset_saving.rs.html#70)
*   [examples/ui/images/image\_node\_resizing.rs](../../src/image_node_resizing/image_node_resizing.rs.html#27)
*   [examples/asset/asset\_saving\_with\_subassets.rs](../../src/asset_saving_with_subassets/asset_saving_with_subassets.rs.html#238)
*   [examples/ecs/observer\_propagation.rs](../../src/observer_propagation/observer_propagation.rs.html#17)
*   [examples/animation/animated\_mesh\_events.rs](../../src/animated_mesh_events/animated_mesh_events.rs.html#29)
*   [examples/gltf/gltf\_extension\_animation\_graph.rs](../../src/gltf_extension_animation_graph/gltf_extension_animation_graph.rs.html#35)
*   [examples/showcase/breakout.rs](../../src/breakout/breakout.rs.html#73)
*   [examples/ui/widgets/standard\_widgets\_observers.rs](../../src/standard_widgets_observers/standard_widgets_observers.rs.html#25)
*   [examples/ui/navigation/directional\_navigation.rs](../../src/directional_navigation/directional_navigation.rs.html#66)
*   [examples/ecs/observers.rs](../../src/observers/observers.rs.html#20-37)
*   [examples/ecs/error\_handling.rs](../../src/error_handling/error_handling.rs.html#47)
*   [examples/ui/navigation/directional\_navigation\_overrides.rs](../../src/directional_navigation_overrides/directional_navigation_overrides.rs.html#76)

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/app.rs.html#1490)

#### pub fn [get\_error\_handler](#method.get_error_handler)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[fn](https://doc.rust-lang.org/nightly/std/primitive.fn.html)([BevyError](../prelude/struct.BevyError.html "struct bevy::prelude::BevyError"), [ErrorContext](../ecs/error/enum.ErrorContext.html "enum bevy::ecs::error::ErrorContext"))>

Gets the error handler to set for new supapps.

Note that the error handler of existing subapps may differ.

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/app.rs.html#1516)

#### pub fn [set\_error\_handler](#method.set_error_handler)( &mut self, handler: [fn](https://doc.rust-lang.org/nightly/std/primitive.fn.html)([BevyError](../prelude/struct.BevyError.html "struct bevy::prelude::BevyError"), [ErrorContext](../ecs/error/enum.ErrorContext.html "enum bevy::ecs::error::ErrorContext")), ) -> &mut [App](../prelude/struct.App.html "struct bevy::prelude::App")

Set the [fallback error handler](../ecs/error/struct.FallbackErrorHandler.html "struct bevy::ecs::error::FallbackErrorHandler") for the all subapps (including the main one and future ones) that do not have one.

May only be called once and should be set by the application, not by libraries.

The handler will be called when an error is produced and not otherwise handled.

##### Panics

Panics if called multiple times.

##### Example

```rust
App::new()
    .set_error_handler(warn)
    .add_plugins(MyPlugins)
    .run();
```

##### [Examples found in repository](#scraped-examples-24)[?](../../scrape-examples-help.html)

examples/ecs/fallible\_params.rs ([line 37](../../src/fallible_params/fallible_params.rs.html#37))

```rust
26fn main() {
27    println!();
28    println!("Press 'A' to add enemy ships and 'R' to remove them.");
29    println!("Player ship will wait for enemy ships and track one if it exists,");
30    println!("but will stop tracking if there are more than one.");
31    println!();
32
33    App::new()
34        // By default, if a parameter fail to be fetched,
35        // `World::get_default_error_handler` will be used to handle the error,
36        // which by default is set to panic.
37        .set_error_handler(warn)
38        .add_plugins(DefaultPlugins)
39        .add_systems(Startup, setup)
40        .add_systems(Update, (user_input, move_targets, track_targets).chain())
41        // This system will always fail validation, because we never create an entity with both `Player` and `Enemy` components.
42        .add_systems(Update, do_nothing_fail_validation)
43        .run();
44}
```

Hide additional examples

examples/ecs/error\_handling.rs ([line 22](../../src/error_handling/error_handling.rs.html#22))

```rust
12fn main() {
13    let mut app = App::new();
14    // By default, fallible systems that return an error will respond according to the `Severity`` in the error.
15    // These will typically panic, unless `with_severity` is used to change the severity of the error.
16    //
17    // We can change this by configuring the fallback error handler, which applies to the entire app
18    // (you can also set it for specific `World`s).
19    // Here we are using one of the built-in error handlers.
20    // Bevy provides built-in handlers for `panic`, `error`, `warn`, `info`,
21    // `debug`, `trace` and `ignore`.
22    app.set_error_handler(warn);
23
24    app.add_plugins(DefaultPlugins);
25
26    #[cfg(feature = "mesh_picking")]
27    app.add_plugins(MeshPickingPlugin);
28
29    // Fallible systems can be used the same way as regular systems. The only difference is they
30    // return a `Result<(), BevyError>` instead of a `()` (unit) type. Bevy will handle both
31    // types of systems the same way, except for the error handling.
32    app.add_systems(Startup, setup);
33
34    // Commands can also return `Result`s, which are automatically handled by the global error handler
35    // if not explicitly handled by the user.
36    app.add_systems(Startup, failing_commands);
37
38    // Individual systems can also be handled by piping the output result:
39    app.add_systems(
40        PostStartup,
41        failing_system.pipe(|result: In<Result>| {
42            let _ = result.0.inspect_err(|err| info!("captured error: {err}"));
43        }),
44    );
45
46    // Fallible observers are also supported.
47    app.add_observer(fallible_observer);
48
49    // If we run the app, we'll see the following output at startup:
50    //
51    //  WARN Encountered an error in system `fallible_systems::failing_system`: Resource not initialized
52    // ERROR fallible_systems::failing_system failed: Resource not initialized
53    //  INFO captured error: Resource not initialized
54    app.run();
55}
```

## Trait Implementations

[Source](https://docs.rs/bevy_audio/0.19.0/x86_64-unknown-linux-gnu/src/bevy_audio/lib.rs.html#107)

### impl [AddAudioSource](../audio/trait.AddAudioSource.html "trait bevy::audio::AddAudioSource") for [App](../prelude/struct.App.html "struct bevy::prelude::App")

[Source](https://docs.rs/bevy_audio/0.19.0/x86_64-unknown-linux-gnu/src/bevy_audio/lib.rs.html#108-111)

#### fn [add\_audio\_source](../audio/trait.AddAudioSource.html#tymethod.add_audio_source)<T>(&mut self) -> &mut [App](../prelude/struct.App.html "struct bevy::prelude::App")

where T: [Decodable](../prelude/trait.Decodable.html "trait bevy::prelude::Decodable") + [Asset](../prelude/trait.Asset.html "trait bevy::prelude::Asset"), [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html): [FromSample](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/conv/trait.FromSample.html "trait dasp_sample::conv::FromSample")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\>,

Registers an audio source. The type must implement [`Decodable`](../prelude/trait.Decodable.html "trait bevy::prelude::Decodable"), so that it can be converted to a [`rodio::Source`](../audio/trait.Source.html "trait bevy::audio::Source") type, and [`Asset`](../prelude/trait.Asset.html "trait bevy::prelude::Asset"), so that it can be registered as an asset. To use this method on [`App`](../prelude/struct.App.html "struct bevy::prelude::App"), the [audio](../audio/struct.AudioPlugin.html "struct bevy::audio::AudioPlugin") and [asset](../prelude/struct.AssetPlugin.html "struct bevy::prelude::AssetPlugin") plugins must be added first.

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_phase/draw.rs.html#383)

### impl [AddRenderCommand](../render/render_phase/trait.AddRenderCommand.html "trait bevy::render::render_phase::AddRenderCommand") for [App](../prelude/struct.App.html "struct bevy::prelude::App")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_phase/draw.rs.html#384-388)

#### fn [add\_render\_command](../render/render_phase/trait.AddRenderCommand.html#tymethod.add_render_command)<P, C>(&mut self) -> &mut [App](../prelude/struct.App.html "struct bevy::prelude::App")

where P: [PhaseItem](../render/render_phase/trait.PhaseItem.html "trait bevy::render::render_phase::PhaseItem"), C: [RenderCommand](../render/render_phase/trait.RenderCommand.html "trait bevy::render::render_phase::RenderCommand")<P> + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static, <C as [RenderCommand](../render/render_phase/trait.RenderCommand.html "trait bevy::render::render_phase::RenderCommand")<P>>::[Param](../render/render_phase/trait.RenderCommand.html#associatedtype.Param "type bevy::render::render_phase::RenderCommand::Param"): [ReadOnlySystemParam](../ecs/system/trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam"),

Adds the [`RenderCommand`](../render/render_phase/trait.RenderCommand.html "trait bevy::render::render_phase::RenderCommand") for the specified render phase to the app.

[Source](https://docs.rs/bevy_state/0.19.0/x86_64-unknown-linux-gnu/src/bevy_state/app.rs.html#288)

### impl [AppExtStates](../prelude/trait.AppExtStates.html "trait bevy::prelude::AppExtStates") for [App](../prelude/struct.App.html "struct bevy::prelude::App")

[Source](https://docs.rs/bevy_state/0.19.0/x86_64-unknown-linux-gnu/src/bevy_state/app.rs.html#289)

#### fn [init\_state](../prelude/trait.AppExtStates.html#tymethod.init_state)<S>(&mut self) -> &mut [App](../prelude/struct.App.html "struct bevy::prelude::App")

where S: [FreelyMutableState](../state/state/trait.FreelyMutableState.html "trait bevy::state::state::FreelyMutableState") + [FromWorld](../prelude/trait.FromWorld.html "trait bevy::prelude::FromWorld"),

Initializes a [`State`](../prelude/struct.State.html "struct bevy::prelude::State") with standard starting values. [Read more](../prelude/trait.AppExtStates.html#tymethod.init_state)

[Source](https://docs.rs/bevy_state/0.19.0/x86_64-unknown-linux-gnu/src/bevy_state/app.rs.html#294)

#### fn [insert\_state](../prelude/trait.AppExtStates.html#tymethod.insert_state)<S>(&mut self, state: S) -> &mut [App](../prelude/struct.App.html "struct bevy::prelude::App")

where S: [FreelyMutableState](../state/state/trait.FreelyMutableState.html "trait bevy::state::state::FreelyMutableState"),

Inserts a specific [`State`](../prelude/struct.State.html "struct bevy::prelude::State") to the current [`App`](../prelude/struct.App.html "struct bevy::prelude::App") and overrides any [`State`](../prelude/struct.State.html "struct bevy::prelude::State") previously added of the same type. [Read more](../prelude/trait.AppExtStates.html#tymethod.insert_state)

[Source](https://docs.rs/bevy_state/0.19.0/x86_64-unknown-linux-gnu/src/bevy_state/app.rs.html#299)

#### fn [add\_computed\_state](../prelude/trait.AppExtStates.html#tymethod.add_computed_state)<S>(&mut self) -> &mut [App](../prelude/struct.App.html "struct bevy::prelude::App")

where S: [ComputedStates](../prelude/trait.ComputedStates.html "trait bevy::prelude::ComputedStates"),

Sets up a type implementing [`ComputedStates`](../prelude/trait.ComputedStates.html "trait bevy::prelude::ComputedStates"). [Read more](../prelude/trait.AppExtStates.html#tymethod.add_computed_state)

[Source](https://docs.rs/bevy_state/0.19.0/x86_64-unknown-linux-gnu/src/bevy_state/app.rs.html#304)

#### fn [add\_sub\_state](../prelude/trait.AppExtStates.html#tymethod.add_sub_state)<S>(&mut self) -> &mut [App](../prelude/struct.App.html "struct bevy::prelude::App")

where S: [SubStates](../prelude/trait.SubStates.html "trait bevy::prelude::SubStates"),

Sets up a type implementing [`SubStates`](../prelude/trait.SubStates.html "trait bevy::prelude::SubStates"). [Read more](../prelude/trait.AppExtStates.html#tymethod.add_sub_state)

[Source](https://docs.rs/bevy_state/0.19.0/x86_64-unknown-linux-gnu/src/bevy_state/app.rs.html#310-312)

#### fn [register\_type\_state](../prelude/trait.AppExtStates.html#tymethod.register_type_state)<S>(&mut self) -> &mut [App](../prelude/struct.App.html "struct bevy::prelude::App")

where S: [States](../prelude/trait.States.html "trait bevy::prelude::States") + [FromReflect](../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + [GetTypeRegistration](../reflect/trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration") + [Typed](../reflect/trait.Typed.html "trait bevy::reflect::Typed"),

Available on **crate feature `bevy_reflect`** only.

Registers the state type `T` using [`App::register_type`](../prelude/struct.App.html#method.register_type "method bevy::prelude::App::register_type"), and adds [`ReflectState`](../prelude/struct.ReflectState.html "struct bevy::prelude::ReflectState") type data to `T` in the type registry. [Read more](../prelude/trait.AppExtStates.html#tymethod.register_type_state)

[Source](https://docs.rs/bevy_state/0.19.0/x86_64-unknown-linux-gnu/src/bevy_state/app.rs.html#319-321)

#### fn [register\_type\_mutable\_state](../prelude/trait.AppExtStates.html#tymethod.register_type_mutable_state)<S>(&mut self) -> &mut [App](../prelude/struct.App.html "struct bevy::prelude::App")

where S: [FreelyMutableState](../state/state/trait.FreelyMutableState.html "trait bevy::state::state::FreelyMutableState") + [FromReflect](../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + [GetTypeRegistration](../reflect/trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration") + [Typed](../reflect/trait.Typed.html "trait bevy::reflect::Typed"),

Available on **crate feature `bevy_reflect`** only.

Registers the state type `T` using [`App::register_type`](../prelude/struct.App.html#method.register_type "method bevy::prelude::App::register_type"), and adds [`crate::reflect::ReflectState`](../prelude/struct.ReflectState.html "struct bevy::prelude::ReflectState") and [`crate::reflect::ReflectFreelyMutableState`](../prelude/struct.ReflectFreelyMutableState.html "struct bevy::prelude::ReflectFreelyMutableState") type data to `T` in the type registry. [Read more](../prelude/trait.AppExtStates.html#tymethod.register_type_mutable_state)

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/lib.rs.html#142)

### impl [AppGizmoBuilder](../prelude/trait.AppGizmoBuilder.html "trait bevy::prelude::AppGizmoBuilder") for [App](../prelude/struct.App.html "struct bevy::prelude::App")

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/lib.rs.html#143)

#### fn [init\_gizmo\_group](../prelude/trait.AppGizmoBuilder.html#tymethod.init_gizmo_group)<Config>(&mut self) -> &mut [App](../prelude/struct.App.html "struct bevy::prelude::App")

where Config: [GizmoConfigGroup](../prelude/trait.GizmoConfigGroup.html "trait bevy::prelude::GizmoConfigGroup"),

Registers [`GizmoConfigGroup`](../prelude/trait.GizmoConfigGroup.html "trait bevy::prelude::GizmoConfigGroup") in the app enabling the use of [Gizmos<Config>](../prelude/struct.Gizmos.html "struct bevy::prelude::Gizmos"). [Read more](../prelude/trait.AppGizmoBuilder.html#tymethod.init_gizmo_group)

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/lib.rs.html#185-189)

#### fn [insert\_gizmo\_config](../prelude/trait.AppGizmoBuilder.html#tymethod.insert_gizmo_config)<Config>( &mut self, group: Config, config: [GizmoConfig](../prelude/struct.GizmoConfig.html "struct bevy::prelude::GizmoConfig"), ) -> &mut [App](../prelude/struct.App.html "struct bevy::prelude::App")

where Config: [GizmoConfigGroup](../prelude/trait.GizmoConfigGroup.html "trait bevy::prelude::GizmoConfigGroup"),

Insert a [`GizmoConfig`](../prelude/struct.GizmoConfig.html "struct bevy::prelude::GizmoConfig") into a specific [`GizmoConfigGroup`](../prelude/trait.GizmoConfigGroup.html "trait bevy::prelude::GizmoConfigGroup"). [Read more](../prelude/trait.AppGizmoBuilder.html#tymethod.insert_gizmo_config)

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/lib.rs.html#592)

### impl [AssetApp](../prelude/trait.AssetApp.html "trait bevy::prelude::AssetApp") for [App](../prelude/struct.App.html "struct bevy::prelude::App")

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/lib.rs.html#593)

#### fn [register\_asset\_loader](../prelude/trait.AssetApp.html#tymethod.register_asset_loader)<L>(&mut self, loader: L) -> &mut [App](../prelude/struct.App.html "struct bevy::prelude::App")

where L: [AssetLoader](../asset/trait.AssetLoader.html "trait bevy::asset::AssetLoader"),

Registers the given `loader` in the [`App`](../prelude/struct.App.html "struct bevy::prelude::App")’s [`AssetServer`](../prelude/struct.AssetServer.html "struct bevy::prelude::AssetServer").

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/lib.rs.html#600)

#### fn [register\_asset\_processor](../prelude/trait.AssetApp.html#tymethod.register_asset_processor)<P>(&mut self, processor: P) -> &mut [App](../prelude/struct.App.html "struct bevy::prelude::App")

where P: [Process](../asset/processor/trait.Process.html "trait bevy::asset::processor::Process"),

Registers the given `processor` in the [`App`](../prelude/struct.App.html "struct bevy::prelude::App")’s [`AssetProcessor`](../asset/processor/struct.AssetProcessor.html "struct bevy::asset::processor::AssetProcessor").

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/lib.rs.html#607-611)

#### fn [register\_asset\_source](../prelude/trait.AssetApp.html#tymethod.register_asset_source)( &mut self, id: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[AssetSourceId](../asset/io/enum.AssetSourceId.html "enum bevy::asset::io::AssetSourceId")<'static>>, source: [AssetSourceBuilder](../asset/io/struct.AssetSourceBuilder.html "struct bevy::asset::io::AssetSourceBuilder"), ) -> &mut [App](../prelude/struct.App.html "struct bevy::prelude::App")

Registers the given [`AssetSourceBuilder`](../asset/io/struct.AssetSourceBuilder.html "struct bevy::asset::io::AssetSourceBuilder") with the given `id`. [Read more](../prelude/trait.AssetApp.html#tymethod.register_asset_source)

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/lib.rs.html#627)

#### fn [set\_default\_asset\_processor](../prelude/trait.AssetApp.html#tymethod.set_default_asset_processor)<P>(&mut self, extension: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)) -> &mut [App](../prelude/struct.App.html "struct bevy::prelude::App")

where P: [Process](../asset/processor/trait.Process.html "trait bevy::asset::processor::Process"),

Sets the default asset processor for the given `extension`.

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/lib.rs.html#634)

#### fn [init\_asset\_loader](../prelude/trait.AssetApp.html#tymethod.init_asset_loader)<L>(&mut self) -> &mut [App](../prelude/struct.App.html "struct bevy::prelude::App")

where L: [AssetLoader](../asset/trait.AssetLoader.html "trait bevy::asset::AssetLoader") + [FromWorld](../prelude/trait.FromWorld.html "trait bevy::prelude::FromWorld"),

Initializes the given loader in the [`App`](../prelude/struct.App.html "struct bevy::prelude::App")’s [`AssetServer`](../prelude/struct.AssetServer.html "struct bevy::prelude::AssetServer").

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/lib.rs.html#639)

#### fn [init\_asset](../prelude/trait.AssetApp.html#tymethod.init_asset)<A>(&mut self) -> &mut [App](../prelude/struct.App.html "struct bevy::prelude::App")

where A: [Asset](../prelude/trait.Asset.html "trait bevy::prelude::Asset"),

Initializes the given [`Asset`](../prelude/trait.Asset.html "trait bevy::prelude::Asset") in the [`App`](../prelude/struct.App.html "struct bevy::prelude::App") by: [Read more](../prelude/trait.AssetApp.html#tymethod.init_asset)

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/lib.rs.html#673-675)

#### fn [register\_asset\_reflect](../prelude/trait.AssetApp.html#tymethod.register_asset_reflect)<A>(&mut self) -> &mut [App](../prelude/struct.App.html "struct bevy::prelude::App")

where A: [Asset](../prelude/trait.Asset.html "trait bevy::prelude::Asset") + [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + [FromReflect](../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + [GetTypeRegistration](../reflect/trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"),

Registers the asset type `T` using `[App::register]`, and adds [`ReflectAsset`](../asset/struct.ReflectAsset.html "struct bevy::asset::ReflectAsset") type data to `T` and [`ReflectHandle`](../asset/struct.ReflectHandle.html "struct bevy::asset::ReflectHandle") type data to [`Handle<T>`](../prelude/enum.Handle.html "enum bevy::prelude::Handle") in the type registry. [Read more](../prelude/trait.AssetApp.html#tymethod.register_asset_reflect)

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/lib.rs.html#693)

#### fn [preregister\_asset\_loader](../prelude/trait.AssetApp.html#tymethod.preregister_asset_loader)<L>(&mut self, extensions: &\[&[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\]) -> &mut [App](../prelude/struct.App.html "struct bevy::prelude::App")

where L: [AssetLoader](../asset/trait.AssetLoader.html "trait bevy::asset::AssetLoader"),

Preregisters a loader for the given extensions, that will block asset loads until a real loader is registered.

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/app.rs.html#99)

### impl [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") for [App](../prelude/struct.App.html "struct bevy::prelude::App")

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/app.rs.html#100)

#### fn [fmt](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)(&self, f: &mut [Formatter](https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html "struct core::fmt::Formatter")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html "struct core::fmt::Error")\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/app.rs.html#109)

### impl [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") for [App](../prelude/struct.App.html "struct bevy::prelude::App")

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/app.rs.html#110)

#### fn [default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)() -> [App](../prelude/struct.App.html "struct bevy::prelude::App")

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/io/embedded/mod.rs.html#153)

### impl [GetAssetServer](../asset/io/embedded/trait.GetAssetServer.html "trait bevy::asset::io::embedded::GetAssetServer") for [App](../prelude/struct.App.html "struct bevy::prelude::App")

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/io/embedded/mod.rs.html#154)

#### fn [get\_asset\_server](../asset/io/embedded/trait.GetAssetServer.html#tymethod.get_asset_server)(&self) -> &[AssetServer](../prelude/struct.AssetServer.html "struct bevy::prelude::AssetServer")

[Source](https://docs.rs/bevy_diagnostic/0.19.0/x86_64-unknown-linux-gnu/src/bevy_diagnostic/diagnostic.rs.html#433)

### impl [RegisterDiagnostic](../diagnostic/trait.RegisterDiagnostic.html "trait bevy::diagnostic::RegisterDiagnostic") for [App](../prelude/struct.App.html "struct bevy::prelude::App")

[Source](https://docs.rs/bevy_diagnostic/0.19.0/x86_64-unknown-linux-gnu/src/bevy_diagnostic/diagnostic.rs.html#434)

#### fn [register\_diagnostic](../diagnostic/trait.RegisterDiagnostic.html#tymethod.register_diagnostic)(&mut self, diagnostic: [Diagnostic](../diagnostic/struct.Diagnostic.html "struct bevy::diagnostic::Diagnostic")) -> &mut [App](../prelude/struct.App.html "struct bevy::prelude::App")

Register a new [`Diagnostic`](../diagnostic/struct.Diagnostic.html "struct bevy::diagnostic::Diagnostic") with an [`App`](../prelude/struct.App.html "struct bevy::prelude::App"). [Read more](../diagnostic/trait.RegisterDiagnostic.html#tymethod.register_diagnostic)

[Source](https://docs.rs/bevy_state/0.19.0/x86_64-unknown-linux-gnu/src/bevy_state/state_scoped_events.rs.html#150)

### impl [StateScopedMessagesAppExt](../prelude/trait.StateScopedMessagesAppExt.html "trait bevy::prelude::StateScopedMessagesAppExt") for [App](../prelude/struct.App.html "struct bevy::prelude::App")

[Source](https://docs.rs/bevy_state/0.19.0/x86_64-unknown-linux-gnu/src/bevy_state/state_scoped_events.rs.html#151)

#### fn [clear\_messages\_on\_exit](../prelude/trait.StateScopedMessagesAppExt.html#tymethod.clear_messages_on_exit)<M>(&mut self, state: impl [States](../prelude/trait.States.html "trait bevy::prelude::States")) -> &mut [App](../prelude/struct.App.html "struct bevy::prelude::App")

where M: [Message](../prelude/trait.Message.html "trait bevy::prelude::Message"),

Clears a [`Message`](../prelude/trait.Message.html "trait bevy::prelude::Message") when exiting the specified `state`. [Read more](../prelude/trait.StateScopedMessagesAppExt.html#tymethod.clear_messages_on_exit)

[Source](https://docs.rs/bevy_state/0.19.0/x86_64-unknown-linux-gnu/src/bevy_state/state_scoped_events.rs.html#161)

#### fn [clear\_messages\_on\_enter](../prelude/trait.StateScopedMessagesAppExt.html#tymethod.clear_messages_on_enter)<M>(&mut self, state: impl [States](../prelude/trait.States.html "trait bevy::prelude::States")) -> &mut [App](../prelude/struct.App.html "struct bevy::prelude::App")

where M: [Message](../prelude/trait.Message.html "trait bevy::prelude::Message"),

Clears a [`Message`](../prelude/trait.Message.html "trait bevy::prelude::Message") when entering the specified `state`. [Read more](../prelude/trait.StateScopedMessagesAppExt.html#tymethod.clear_messages_on_enter)

## Auto Trait Implementations

### impl ![Freeze](https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html "trait core::marker::Freeze") for [App](../prelude/struct.App.html "struct bevy::prelude::App")

### impl ![RefUnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html "trait core::panic::unwind_safe::RefUnwindSafe") for [App](../prelude/struct.App.html "struct bevy::prelude::App")

### impl ![Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") for [App](../prelude/struct.App.html "struct bevy::prelude::App")

### impl ![Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") for [App](../prelude/struct.App.html "struct bevy::prelude::App")

### impl ![UnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html "trait core::panic::unwind_safe::UnwindSafe") for [App](../prelude/struct.App.html "struct bevy::prelude::App")

### impl [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin") for [App](../prelude/struct.App.html "struct bevy::prelude::App")

### impl [UnsafeUnpin](https://doc.rust-lang.org/nightly/core/marker/trait.UnsafeUnpin.html "trait core::marker::UnsafeUnpin") for [App](../prelude/struct.App.html "struct bevy::prelude::App")

## Blanket Implementations

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#141)

### impl<T> [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") for T

where T: 'static + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#142)

#### fn [type\_id](https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id)(&self) -> [TypeId](https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html "struct core::any::TypeId")

Gets the `TypeId` of `self`. [Read more](https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id)

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_resource/bind_group.rs.html#696-698)

### impl<T, U> [AsBindGroupShaderType](../render/render_resource/trait.AsBindGroupShaderType.html "trait bevy::render::render_resource::AsBindGroupShaderType")<U> for T

where U: [ShaderType](../render/render_resource/trait.ShaderType.html "trait bevy::render::render_resource::ShaderType"), [&'a T](https://doc.rust-lang.org/nightly/std/primitive.reference.html): for<'a> [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<U>,

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_resource/bind_group.rs.html#701)

#### fn [as\_bind\_group\_shader\_type](../render/render_resource/trait.AsBindGroupShaderType.html#tymethod.as_bind_group_shader_type)(&self, \_images: &[RenderAssets](../render/render_asset/struct.RenderAssets.html "struct bevy::render::render_asset::RenderAssets")<[GpuImage](../render/texture/struct.GpuImage.html "struct bevy::render::texture::GpuImage")\>) -> U

Return the `T` [`ShaderType`](../render/render_resource/trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") for `self`. When used in [`AsBindGroup`](../render/render_resource/trait.AsBindGroup.html "trait bevy::render::render_resource::AsBindGroup") derives, it is safe to assume that all images in `self` exist.

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#212)

### impl<T> [Borrow](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html "trait core::borrow::Borrow")<T> for T

where T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#214)

#### fn [borrow](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow)(&self) -> [&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

Immutably borrows from an owned value. [Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow)

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#221)

### impl<T> [BorrowMut](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html "trait core::borrow::BorrowMut")<T> for T

where T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#222)

#### fn [borrow\_mut](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut)(&mut self) -> [&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

Mutably borrows from an owned value. [Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut)

[Source](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/src/zerocopy/pointer/invariant.rs.html#244)

### impl<ST, DT> [CastableFrom](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/trait.CastableFrom.html "trait zerocopy::pointer::invariant::CastableFrom")<ST, [Initialized](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/enum.Initialized.html "enum zerocopy::pointer::invariant::Initialized"), [Initialized](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/enum.Initialized.html "enum zerocopy::pointer::invariant::Initialized")\> for DT

where ST: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), DT: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/src/zerocopy/pointer/invariant.rs.html#242)

### impl<ST, DT> [CastableFrom](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/trait.CastableFrom.html "trait zerocopy::pointer::invariant::CastableFrom")<ST, [Uninit](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/enum.Uninit.html "enum zerocopy::pointer::invariant::Uninit"), [Uninit](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/enum.Uninit.html "enum zerocopy::pointer::invariant::Uninit")\> for DT

where ST: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), DT: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/conv.rs.html#58)

### impl<T> [Conv](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/conv/trait.Conv.html "trait tap::conv::Conv") for T

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/conv.rs.html#49-52)

#### fn [conv](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/conv/trait.Conv.html#method.conv)<T>(self) -> T

where Self: [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<T>,

Converts `self` into `T` using `Into<T>`. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/conv/trait.Conv.html#method.conv)

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#201)

### impl<T> [Downcast](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html "trait downcast_rs::Downcast") for T

where T: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"),

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#202)

#### fn [into\_any](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html#tymethod.into_any)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<T>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

Converts `Box<dyn Trait>` (where `Trait: Downcast`) to `Box<dyn Any>`, which can then be `downcast` into `Box<dyn ConcreteType>` where `ConcreteType` implements `Trait`.

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#203)

#### fn [into\_any\_rc](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html#tymethod.into_any_rc)(self: [Rc](https://doc.rust-lang.org/nightly/alloc/rc/struct.Rc.html "struct alloc::rc::Rc")<T>) -> [Rc](https://doc.rust-lang.org/nightly/alloc/rc/struct.Rc.html "struct alloc::rc::Rc")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

Converts `Rc<Trait>` (where `Trait: Downcast`) to `Rc<Any>`, which can then be further `downcast` into `Rc<ConcreteType>` where `ConcreteType` implements `Trait`.

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#204)

#### fn [as\_any](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html#tymethod.as_any)(&self) -> &(dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

Converts `&Trait` (where `Trait: Downcast`) to `&Any`. This is needed since Rust cannot generate `&Any`’s vtable from `&Trait`’s.

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#205)

#### fn [as\_any\_mut](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html#tymethod.as_any_mut)(&mut self) -> &mut (dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

Converts `&mut Trait` (where `Trait: Downcast`) to `&Any`. This is needed since Rust cannot generate `&mut Any`’s vtable from `&mut Trait`’s.

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#189)

### impl<T> [Downcast](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html "trait downcast_rs::Downcast") for T

where T: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"),

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#190)

#### fn [into\_any](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html#tymethod.into_any)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<T>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

Convert `Box<dyn Trait>` (where `Trait: Downcast`) to `Box<dyn Any>`. `Box<dyn Any>` can then be further `downcast` into `Box<ConcreteType>` where `ConcreteType` implements `Trait`.

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#191)

#### fn [into\_any\_rc](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html#tymethod.into_any_rc)(self: [Rc](https://doc.rust-lang.org/nightly/alloc/rc/struct.Rc.html "struct alloc::rc::Rc")<T>) -> [Rc](https://doc.rust-lang.org/nightly/alloc/rc/struct.Rc.html "struct alloc::rc::Rc")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

Convert `Rc<Trait>` (where `Trait: Downcast`) to `Rc<Any>`. `Rc<Any>` can then be further `downcast` into `Rc<ConcreteType>` where `ConcreteType` implements `Trait`.

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#192)

#### fn [as\_any](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html#tymethod.as_any)(&self) -> &(dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

Convert `&Trait` (where `Trait: Downcast`) to `&Any`. This is needed since Rust cannot generate `&Any`’s vtable from `&Trait`’s.

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#193)

#### fn [as\_any\_mut](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html#tymethod.as_any_mut)(&mut self) -> &mut (dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

Convert `&mut Trait` (where `Trait: Downcast`) to `&Any`. This is needed since Rust cannot generate `&mut Any`’s vtable from `&mut Trait`’s.

[Source](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/src/dasp_sample/conv.rs.html#699)

### impl<S, T> [Duplex](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/conv/trait.Duplex.html "trait dasp_sample::conv::Duplex")<S> for T

where T: [FromSample](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/conv/trait.FromSample.html "trait dasp_sample::conv::FromSample")<S> + [ToSample](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/conv/trait.ToSample.html "trait dasp_sample::conv::ToSample")<S>,

[Source](https://docs.rs/yoke/0.8.3/x86_64-unknown-linux-gnu/src/yoke/erased.rs.html#22)

### impl<T> [ErasedDestructor](https://docs.rs/yoke/0.8.3/x86_64-unknown-linux-gnu/yoke/erased/trait.ErasedDestructor.html "trait yoke::erased::ErasedDestructor") for T

where T: 'static,

[Source](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/src/wyz/fmt.rs.html#114)

### impl<T> [FmtForward](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/trait.FmtForward.html "trait wyz::fmt::FmtForward") for T

[Source](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/src/wyz/fmt.rs.html#41-42)

#### fn [fmt\_binary](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/trait.FmtForward.html#method.fmt_binary)(self) -> [FmtBinary](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/struct.FmtBinary.html "struct wyz::fmt::FmtBinary")<Self>

where Self: [Binary](https://doc.rust-lang.org/nightly/core/fmt/trait.Binary.html "trait core::fmt::Binary"),

Causes `self` to use its `Binary` implementation when `Debug`\-formatted.

[Source](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/src/wyz/fmt.rs.html#49-50)

#### fn [fmt\_display](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/trait.FmtForward.html#method.fmt_display)(self) -> [FmtDisplay](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/struct.FmtDisplay.html "struct wyz::fmt::FmtDisplay")<Self>

where Self: [Display](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html "trait core::fmt::Display"),

Causes `self` to use its `Display` implementation when `Debug`\-formatted.

[Source](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/src/wyz/fmt.rs.html#57-58)

#### fn [fmt\_lower\_exp](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/trait.FmtForward.html#method.fmt_lower_exp)(self) -> [FmtLowerExp](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/struct.FmtLowerExp.html "struct wyz::fmt::FmtLowerExp")<Self>

where Self: [LowerExp](https://doc.rust-lang.org/nightly/core/fmt/trait.LowerExp.html "trait core::fmt::LowerExp"),

Causes `self` to use its `LowerExp` implementation when `Debug`\-formatted.

[Source](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/src/wyz/fmt.rs.html#65-66)

#### fn [fmt\_lower\_hex](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/trait.FmtForward.html#method.fmt_lower_hex)(self) -> [FmtLowerHex](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/struct.FmtLowerHex.html "struct wyz::fmt::FmtLowerHex")<Self>

where Self: [LowerHex](https://doc.rust-lang.org/nightly/core/fmt/trait.LowerHex.html "trait core::fmt::LowerHex"),

Causes `self` to use its `LowerHex` implementation when `Debug`\-formatted.

[Source](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/src/wyz/fmt.rs.html#72-73)

#### fn [fmt\_octal](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/trait.FmtForward.html#method.fmt_octal)(self) -> [FmtOctal](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/struct.FmtOctal.html "struct wyz::fmt::FmtOctal")<Self>

where Self: [Octal](https://doc.rust-lang.org/nightly/core/fmt/trait.Octal.html "trait core::fmt::Octal"),

Causes `self` to use its `Octal` implementation when `Debug`\-formatted.

[Source](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/src/wyz/fmt.rs.html#80-81)

#### fn [fmt\_pointer](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/trait.FmtForward.html#method.fmt_pointer)(self) -> [FmtPointer](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/struct.FmtPointer.html "struct wyz::fmt::FmtPointer")<Self>

where Self: [Pointer](https://doc.rust-lang.org/nightly/core/fmt/trait.Pointer.html "trait core::fmt::Pointer"),

Causes `self` to use its `Pointer` implementation when `Debug`\-formatted.

[Source](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/src/wyz/fmt.rs.html#88-89)

#### fn [fmt\_upper\_exp](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/trait.FmtForward.html#method.fmt_upper_exp)(self) -> [FmtUpperExp](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/struct.FmtUpperExp.html "struct wyz::fmt::FmtUpperExp")<Self>

where Self: [UpperExp](https://doc.rust-lang.org/nightly/core/fmt/trait.UpperExp.html "trait core::fmt::UpperExp"),

Causes `self` to use its `UpperExp` implementation when `Debug`\-formatted.

[Source](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/src/wyz/fmt.rs.html#96-97)

#### fn [fmt\_upper\_hex](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/trait.FmtForward.html#method.fmt_upper_hex)(self) -> [FmtUpperHex](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/struct.FmtUpperHex.html "struct wyz::fmt::FmtUpperHex")<Self>

where Self: [UpperHex](https://doc.rust-lang.org/nightly/core/fmt/trait.UpperHex.html "trait core::fmt::UpperHex"),

Causes `self` to use its `UpperHex` implementation when `Debug`\-formatted.

[Source](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/src/wyz/fmt.rs.html#108-109)

#### fn [fmt\_list](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/trait.FmtForward.html#method.fmt_list)(self) -> [FmtList](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/struct.FmtList.html "struct wyz::fmt::FmtList")<Self>

where &'a Self: for<'a> [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator"),

Formats each item in a sequence. [Read more](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/trait.FmtForward.html#method.fmt_list)

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#787)

### impl<T> [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<T> for T

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#790)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(t: T) -> T

Returns the argument unchanged.

[Source](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/src/dasp_sample/conv.rs.html#574)

### impl<S> [FromSample](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/conv/trait.FromSample.html "trait dasp_sample::conv::FromSample")<S> for S

[Source](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/src/dasp_sample/conv.rs.html#576)

#### fn [from\_sample\_](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/conv/trait.FromSample.html#tymethod.from_sample_)(s: S) -> S

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#4000)

### impl<T> [FromWorld](../prelude/trait.FromWorld.html "trait bevy::prelude::FromWorld") for T

where T: [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#4003)

#### fn [from\_world](../prelude/trait.FromWorld.html#tymethod.from_world)(\_world: &mut [World](../prelude/struct.World.html "struct bevy::prelude::World")) -> T

Creates `Self` using [`default()`](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default "associated function core::default::Default::default").

[Source](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/src/typewit/type_witness_traits.rs.html#106-109)

### impl<T, W> [HasTypeWitness](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/typewit/type_witness_traits/trait.HasTypeWitness.html "trait typewit::type_witness_traits::HasTypeWitness")<W> for T

where W: [MakeTypeWitness](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/typewit/type_witness_traits/trait.MakeTypeWitness.html "trait typewit::type_witness_traits::MakeTypeWitness")<Arg = T>, T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/src/typewit/type_witness_traits.rs.html#111)

#### const [WITNESS](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/typewit/type_witness_traits/trait.HasTypeWitness.html#associatedconstant.WITNESS): W = W::MAKE

A constant of the type witness

[Source](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/src/typewit/type_identity.rs.html#77)

### impl<T> [Identity](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/typewit/type_identity/trait.Identity.html "trait typewit::type_identity::Identity") for T

where T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/src/typewit/type_identity.rs.html#80)

#### const [TYPE\_EQ](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/typewit/type_identity/trait.Identity.html#associatedconstant.TYPE_EQ): [TypeEq](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/typewit/type_eq/type_eq_/struct.TypeEq.html "struct typewit::type_eq::type_eq_::TypeEq")<T, <T as [Identity](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/typewit/type_identity/trait.Identity.html "trait typewit::type_identity::Identity")\>::[Type](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/typewit/type_identity/trait.Identity.html#associatedtype.Type "type typewit::type_identity::Identity::Type")\> = TypeEq::NEW

Proof that `Self` is the same type as `Self::Type`, provides methods for casting between `Self` and `Self::Type`.

[Source](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/src/typewit/type_identity.rs.html#78)

#### type [Type](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/typewit/type_identity/trait.Identity.html#associatedtype.Type) = T

The same type as `Self`, used to emulate type equality bounds (`T == U`) with associated type equality constraints (`T: Identity<Type = U>`).

[Source](https://docs.rs/dioxus-signals/0.7.9/x86_64-unknown-linux-gnu/src/dioxus_signals/global/mod.rs.html#19)

### impl<T> [InitializeFromFunction](https://docs.rs/dioxus-signals/0.7.9/x86_64-unknown-linux-gnu/dioxus_signals/global/trait.InitializeFromFunction.html "trait dioxus_signals::global::InitializeFromFunction")<T> for T

[Source](https://docs.rs/dioxus-signals/0.7.9/x86_64-unknown-linux-gnu/src/dioxus_signals/global/mod.rs.html#20)

#### fn [initialize\_from\_function](https://docs.rs/dioxus-signals/0.7.9/x86_64-unknown-linux-gnu/dioxus_signals/global/trait.InitializeFromFunction.html#tymethod.initialize_from_function)(f: [fn](https://doc.rust-lang.org/nightly/std/primitive.fn.html)() -> T) -> T

Create an instance of this type from an initialization function

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#325)

### impl<T> [Instrument](../log/tracing/trait.Instrument.html "trait bevy::log::tracing::Instrument") for T

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#86)

#### fn [instrument](../log/tracing/trait.Instrument.html#method.instrument)(self, span: [Span](../log/tracing/struct.Span.html "struct bevy::log::tracing::Span")) -> [Instrumented](../log/tracing/instrument/struct.Instrumented.html "struct bevy::log::tracing::instrument::Instrumented")<Self> [ⓘ](#)

Instruments this type with the provided [`Span`](../log/tracing/struct.Span.html "struct bevy::log::tracing::Span"), returning an `Instrumented` wrapper. [Read more](../log/tracing/trait.Instrument.html#method.instrument)

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#128)

#### fn [in\_current\_span](../log/tracing/trait.Instrument.html#method.in_current_span)(self) -> [Instrumented](../log/tracing/instrument/struct.Instrumented.html "struct bevy::log::tracing::instrument::Instrumented")<Self> [ⓘ](#)

Instruments this type with the [current](../log/tracing/struct.Span.html#method.current "associated function bevy::log::tracing::Span::current") [`Span`](../log/tracing/struct.Span.html "struct bevy::log::tracing::Span"), returning an `Instrumented` wrapper. [Read more](../log/tracing/trait.Instrument.html#method.in_current_span)

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#769-771)

### impl<T, U> [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<U> for T

where U: [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#779)

#### fn [into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html#tymethod.into)(self) -> U

Calls `U::from(self)`.

That is, this conversion is whatever the implementation of `[From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<T> for U` chooses to do.

[Source](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/src/either/into_either.rs.html#64)

### impl<T> [IntoEither](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/into_either/trait.IntoEither.html "trait either::into_either::IntoEither") for T

[Source](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/src/either/into_either.rs.html#29)

#### fn [into\_either](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/into_either/trait.IntoEither.html#method.into_either)(self, into\_left: [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)) -> [Either](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/enum.Either.html "enum either::Either")<Self, Self> [ⓘ](#)

Converts `self` into a [`Left`](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Left "variant either::Either::Left") variant of [`Either<Self, Self>`](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/enum.Either.html "enum either::Either") if `into_left` is `true`. Converts `self` into a [`Right`](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Right "variant either::Either::Right") variant of [`Either<Self, Self>`](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/enum.Either.html "enum either::Either") otherwise. [Read more](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/into_either/trait.IntoEither.html#method.into_either)

[Source](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/src/either/into_either.rs.html#55-57)

#### fn [into\_either\_with](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/into_either/trait.IntoEither.html#method.into_either_with)<F>(self, into\_left: F) -> [Either](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/enum.Either.html "enum either::Either")<Self, Self> [ⓘ](#)

where F: [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(&Self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html),

Converts `self` into a [`Left`](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Left "variant either::Either::Left") variant of [`Either<Self, Self>`](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/enum.Either.html "enum either::Either") if `into_left(&self)` returns `true`. Converts `self` into a [`Right`](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Right "variant either::Either::Right") variant of [`Either<Self, Self>`](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/enum.Either.html "enum either::Either") otherwise. [Read more](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/into_either/trait.IntoEither.html#method.into_either_with)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/function_system.rs.html#596)

### impl<T> [IntoResult](../ecs/system/trait.IntoResult.html "trait bevy::ecs::system::IntoResult")<T> for T

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/function_system.rs.html#597)

#### fn [into\_result](../ecs/system/trait.IntoResult.html#tymethod.into_result)(self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, [RunSystemError](../ecs/system/enum.RunSystemError.html "enum bevy::ecs::system::RunSystemError")\>

Converts this type into the system output type.

[Source](https://docs.rs/symphonia-core/0.5.5/x86_64-unknown-linux-gnu/src/symphonia_core/conv.rs.html#636)

### impl<F, T> [IntoSample](https://docs.rs/symphonia-core/0.5.5/x86_64-unknown-linux-gnu/symphonia_core/conv/trait.IntoSample.html "trait symphonia_core::conv::IntoSample")<T> for F

where T: [FromSample](https://docs.rs/symphonia-core/0.5.5/x86_64-unknown-linux-gnu/symphonia_core/conv/trait.FromSample.html "trait symphonia_core::conv::FromSample")<F>,

[Source](https://docs.rs/symphonia-core/0.5.5/x86_64-unknown-linux-gnu/src/symphonia_core/conv.rs.html#638)

#### fn [into\_sample](https://docs.rs/symphonia-core/0.5.5/x86_64-unknown-linux-gnu/symphonia_core/conv/trait.IntoSample.html#tymethod.into_sample)(self) -> T

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/is.rs.html#26)

### impl<A> [Is](../reflect/trait.Is.html "trait bevy::reflect::Is") for A

where A: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/is.rs.html#28)

#### fn [is](../reflect/trait.Is.html#tymethod.is)<T>() -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

where T: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"),

Checks if the current type “is” another type, using a [`TypeId`](https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html "struct core::any::TypeId") equality comparison. This is most useful in the context of generic logic. [Read more](../reflect/trait.Is.html#tymethod.is)

[Source](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/src/zvariant/optional.rs.html#31-33)

### impl<T> [NoneValue](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/zvariant/optional/trait.NoneValue.html "trait zvariant::optional::NoneValue") for T

where T: [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default"),

[Source](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/src/zvariant/optional.rs.html#35)

#### type [NoneType](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/zvariant/optional/trait.NoneValue.html#associatedtype.NoneType) = T

[Source](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/src/zvariant/optional.rs.html#37)

#### fn [null\_value](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/zvariant/optional/trait.NoneValue.html#tymethod.null_value)() -> T

The none-equivalent value.

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/pipe.rs.html#234)

### impl<T> [Pipe](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html "trait tap::pipe::Pipe") for T

where T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/pipe.rs.html#73-76)

#### fn [pipe](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe)<R>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(Self) -> R) -> R

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Pipes by value. This is generally the method you want to use. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/pipe.rs.html#97-99)

#### fn [pipe\_ref](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_ref)<'a, R>(&'a self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(&'a Self) -> R) -> R

where R: 'a,

Borrows `self` and passes that borrow into the pipe function. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_ref)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/pipe.rs.html#122-127)

#### fn [pipe\_ref\_mut](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_ref_mut)<'a, R>(&'a mut self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(&'a mut Self) -> R) -> R

where R: 'a,

Mutably borrows `self` and passes that borrow into the pipe function. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_ref_mut)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/pipe.rs.html#145-149)

#### fn [pipe\_borrow](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_borrow)<'a, B, R>(&'a self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&'a B](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> R) -> R

where Self: [Borrow](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html "trait core::borrow::Borrow")<B>, B: 'a + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), R: 'a,

Borrows `self`, then passes `self.borrow()` into the pipe function. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_borrow)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/pipe.rs.html#169-176)

#### fn [pipe\_borrow\_mut](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_borrow_mut)<'a, B, R>( &'a mut self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&'a mut B](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> R, ) -> R

where Self: [BorrowMut](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html "trait core::borrow::BorrowMut")<B>, B: 'a + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), R: 'a,

Mutably borrows `self`, then passes `self.borrow_mut()` into the pipe function. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_borrow_mut)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/pipe.rs.html#183-187)

#### fn [pipe\_as\_ref](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_as_ref)<'a, U, R>(&'a self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&'a U](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> R) -> R

where Self: [AsRef](https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html "trait core::convert::AsRef")<U>, U: 'a + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), R: 'a,

Borrows `self`, then passes `self.as_ref()` into the pipe function.

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/pipe.rs.html#195-202)

#### fn [pipe\_as\_mut](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_as_mut)<'a, U, R>(&'a mut self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&'a mut U](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> R) -> R

where Self: [AsMut](https://doc.rust-lang.org/nightly/core/convert/trait.AsMut.html "trait core::convert::AsMut")<U>, U: 'a + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), R: 'a,

Mutably borrows `self`, then passes `self.as_mut()` into the pipe function.

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/pipe.rs.html#209-213)

#### fn [pipe\_deref](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_deref)<'a, T, R>(&'a self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&'a T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> R) -> R

where Self: [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref")<Target = T>, T: 'a + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), R: 'a,

Borrows `self`, then passes `self.deref()` into the pipe function.

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/pipe.rs.html#221-228)

#### fn [pipe\_deref\_mut](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_deref_mut)<'a, T, R>( &'a mut self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&'a mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> R, ) -> R

where Self: [DerefMut](https://doc.rust-lang.org/nightly/core/ops/deref/trait.DerefMut.html "trait core::ops::deref::DerefMut")<Target = T> + [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref"), T: 'a + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), R: 'a,

Mutably borrows `self`, then passes `self.deref_mut()` into the pipe function.

[Source](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/src/zerocopy/pointer/invariant.rs.html#263)

### impl<T> [Read](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/trait.Read.html "trait zerocopy::pointer::invariant::Read")<[Exclusive](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/enum.Exclusive.html "enum zerocopy::pointer::invariant::Exclusive"), [BecauseExclusive](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/enum.BecauseExclusive.html "enum zerocopy::pointer::invariant::BecauseExclusive")\> for T

where T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#347)

### impl<R, P> [ReadPrimitive](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadPrimitive.html "trait lebe::io::ReadPrimitive")<R> for P

where R: [Read](https://doc.rust-lang.org/nightly/std/io/trait.Read.html "trait std::io::Read") + [ReadEndian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html "trait lebe::io::ReadEndian")<P>, P: [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default"),

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#377)

#### fn [read\_from\_little\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadPrimitive.html#method.read_from_little_endian)(read: [&mut R](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Self, [Error](../tasks/futures_lite/io/struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Read this value from the supplied reader. Same as `ReadEndian::read_from_little_endian()`.

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#382)

#### fn [read\_from\_big\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadPrimitive.html#method.read_from_big_endian)(read: [&mut R](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Self, [Error](../tasks/futures_lite/io/struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Read this value from the supplied reader. Same as `ReadEndian::read_from_big_endian()`.

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#387)

#### fn [read\_from\_native\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadPrimitive.html#method.read_from_native_endian)(read: [&mut R](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Self, [Error](../tasks/futures_lite/io/struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Read this value from the supplied reader. Same as `ReadEndian::read_from_native_endian()`.

[Source](https://docs.rs/typenum/1.20.1/x86_64-unknown-linux-gnu/src/typenum/type_operators.rs.html#34)

### impl<T> [Same](https://docs.rs/typenum/1.20.1/x86_64-unknown-linux-gnu/typenum/type_operators/trait.Same.html "trait typenum::type_operators::Same") for T

[Source](https://docs.rs/typenum/1.20.1/x86_64-unknown-linux-gnu/src/typenum/type_operators.rs.html#35)

#### type [Output](https://docs.rs/typenum/1.20.1/x86_64-unknown-linux-gnu/typenum/type_operators/trait.Same.html#associatedtype.Output) = T

Should always be `Self`

[Source](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/src/dioxus_core/events.rs.html#328)

### impl<Ret> [SpawnIfAsync](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/dioxus_core/events/trait.SpawnIfAsync.html "trait dioxus_core::events::SpawnIfAsync")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), Ret> for Ret

[Source](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/src/dioxus_core/events.rs.html#329)

#### fn [spawn](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/dioxus_core/events/trait.SpawnIfAsync.html#tymethod.spawn)(self) -> Ret

Spawn the value into the dioxus runtime if it is an async block

[Source](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/src/dioxus_core/properties.rs.html#199-201)

### impl<T, O> [SuperFrom](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/dioxus_core/properties/trait.SuperFrom.html "trait dioxus_core::properties::SuperFrom")<T> for O

where O: [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<T>,

[Source](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/src/dioxus_core/properties.rs.html#203)

#### fn [super\_from](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/dioxus_core/properties/trait.SuperFrom.html#tymethod.super_from)(input: T) -> O

Convert from a type to another type.

[Source](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/src/dioxus_core/properties.rs.html#183-185)

### impl<T, O, M> [SuperInto](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/dioxus_core/properties/trait.SuperInto.html "trait dioxus_core::properties::SuperInto")<O, M> for T

where O: [SuperFrom](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/dioxus_core/properties/trait.SuperFrom.html "trait dioxus_core::properties::SuperFrom")<T, M>,

[Source](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/src/dioxus_core/properties.rs.html#187)

#### fn [super\_into](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/dioxus_core/properties/trait.SuperInto.html#tymethod.super_into)(self) -> O

Convert from a type to another type.

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#329)

### impl<T> [Tap](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html "trait tap::tap::Tap") for T

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#78)

#### fn [tap](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap)(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(&Self)) -> Self

Immutable access to a value. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#116)

#### fn [tap\_mut](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_mut)(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(&mut Self)) -> Self

Mutable access to a value. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_mut)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#129-132)

#### fn [tap\_borrow](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_borrow)<B>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&B](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -> Self

where Self: [Borrow](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html "trait core::borrow::Borrow")<B>, B: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Immutable access to the `Borrow<B>` of a value. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_borrow)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#146-149)

#### fn [tap\_borrow\_mut](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_borrow_mut)<B>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&mut B](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -> Self

where Self: [BorrowMut](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html "trait core::borrow::BorrowMut")<B>, B: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Mutable access to the `BorrowMut<B>` of a value. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_borrow_mut)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#163-166)

#### fn [tap\_ref](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_ref)<R>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&R](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -> Self

where Self: [AsRef](https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html "trait core::convert::AsRef")<R>, R: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Immutable access to the `AsRef<R>` view of a value. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_ref)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#180-183)

#### fn [tap\_ref\_mut](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_ref_mut)<R>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&mut R](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -> Self

where Self: [AsMut](https://doc.rust-lang.org/nightly/core/convert/trait.AsMut.html "trait core::convert::AsMut")<R>, R: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Mutable access to the `AsMut<R>` view of a value. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_ref_mut)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#197-200)

#### fn [tap\_deref](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_deref)<T>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -> Self

where Self: [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref")<Target = T>, T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Immutable access to the `Deref::Target` of a value. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_deref)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#214-217)

#### fn [tap\_deref\_mut](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_deref_mut)<T>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -> Self

where Self: [DerefMut](https://doc.rust-lang.org/nightly/core/ops/deref/trait.DerefMut.html "trait core::ops::deref::DerefMut")<Target = T> + [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref"), T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Mutable access to the `Deref::Target` of a value. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_deref_mut)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#227)

#### fn [tap\_dbg](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_dbg)(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(&Self)) -> Self

Calls `.tap()` only in debug builds, and is erased in release builds.

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#237)

#### fn [tap\_mut\_dbg](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_mut_dbg)(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(&mut Self)) -> Self

Calls `.tap_mut()` only in debug builds, and is erased in release builds.

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#247-250)

#### fn [tap\_borrow\_dbg](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_borrow_dbg)<B>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&B](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -> Self

where Self: [Borrow](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html "trait core::borrow::Borrow")<B>, B: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Calls `.tap_borrow()` only in debug builds, and is erased in release builds.

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#261-264)

#### fn [tap\_borrow\_mut\_dbg](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_borrow_mut_dbg)<B>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&mut B](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -> Self

where Self: [BorrowMut](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html "trait core::borrow::BorrowMut")<B>, B: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Calls `.tap_borrow_mut()` only in debug builds, and is erased in release builds.

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#275-278)

#### fn [tap\_ref\_dbg](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_ref_dbg)<R>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&R](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -> Self

where Self: [AsRef](https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html "trait core::convert::AsRef")<R>, R: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Calls `.tap_ref()` only in debug builds, and is erased in release builds.

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#289-292)

#### fn [tap\_ref\_mut\_dbg](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_ref_mut_dbg)<R>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&mut R](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -> Self

where Self: [AsMut](https://doc.rust-lang.org/nightly/core/convert/trait.AsMut.html "trait core::convert::AsMut")<R>, R: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Calls `.tap_ref_mut()` only in debug builds, and is erased in release builds.

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#303-306)

#### fn [tap\_deref\_dbg](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_deref_dbg)<T>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -> Self

where Self: [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref")<Target = T>, T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Calls `.tap_deref()` only in debug builds, and is erased in release builds.

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#317-320)

#### fn [tap\_deref\_mut\_dbg](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_deref_mut_dbg)<T>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -> Self

where Self: [DerefMut](https://doc.rust-lang.org/nightly/core/ops/deref/trait.DerefMut.html "trait core::ops::deref::DerefMut")<Target = T> + [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref"), T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Calls `.tap_deref_mut()` only in debug builds, and is erased in release builds.

[Source](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/src/dasp_sample/conv.rs.html#687-689)

### impl<T, U> [ToSample](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/conv/trait.ToSample.html "trait dasp_sample::conv::ToSample")<U> for T

where U: [FromSample](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/conv/trait.FromSample.html "trait dasp_sample::conv::FromSample")<T>,

[Source](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/src/dasp_sample/conv.rs.html#692)

#### fn [to\_sample\_](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/conv/trait.ToSample.html#tymethod.to_sample_)(self) -> U

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/conv.rs.html#87)

### impl<T> [TryConv](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/conv/trait.TryConv.html "trait tap::conv::TryConv") for T

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/conv.rs.html#78-81)

#### fn [try\_conv](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/conv/trait.TryConv.html#method.try_conv)<T>(self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, Self::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#associatedtype.Error "type core::convert::TryInto::Error")\>

where Self: [TryInto](https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html "trait core::convert::TryInto")<T>,

Attempts to convert `self` into `T` using `TryInto<T>`. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/conv/trait.TryConv.html#method.try_conv)

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#829-831)

### impl<T, U> [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<U> for T

where U: [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#833)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error) = [Infallible](https://doc.rust-lang.org/nightly/core/convert/enum.Infallible.html "enum core::convert::Infallible")

The type returned in the event of a conversion error.

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#836)

#### fn [try\_from](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from)(value: U) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, <T as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<U>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")\>

Performs the conversion.

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#813-815)

### impl<T, U> [TryInto](https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html "trait core::convert::TryInto")<U> for T

where U: [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#817)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#associatedtype.Error) = <U as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<T>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")

The type returned in the event of a conversion error.

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#820)

#### fn [try\_into](https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#tymethod.try_into)(self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<U, <U as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<T>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")\>

Performs the conversion.

[Source](https://docs.rs/ppv-lite86/0.2.21/x86_64-unknown-linux-gnu/src/ppv_lite86/types.rs.html#221-223)

### impl<V, T> [VZip](https://docs.rs/ppv-lite86/0.2.21/x86_64-unknown-linux-gnu/ppv_lite86/types/trait.VZip.html "trait ppv_lite86::types::VZip")<V> for T

where V: [MultiLane](https://docs.rs/ppv-lite86/0.2.21/x86_64-unknown-linux-gnu/ppv_lite86/types/trait.MultiLane.html "trait ppv_lite86::types::MultiLane")<T>,

[Source](https://docs.rs/ppv-lite86/0.2.21/x86_64-unknown-linux-gnu/src/ppv_lite86/types.rs.html#226)

#### fn [vzip](https://docs.rs/ppv-lite86/0.2.21/x86_64-unknown-linux-gnu/ppv_lite86/types/trait.VZip.html#tymethod.vzip)(self) -> V

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#393)

### impl<T> [WithSubscriber](../log/tracing/instrument/trait.WithSubscriber.html "trait bevy::log::tracing::instrument::WithSubscriber") for T

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#176-178)

#### fn [with\_subscriber](../log/tracing/instrument/trait.WithSubscriber.html#method.with_subscriber)<S>(self, subscriber: S) -> [WithDispatch](../log/tracing/instrument/struct.WithDispatch.html "struct bevy::log::tracing::instrument::WithDispatch")<Self> [ⓘ](#)

where S: [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Dispatch](../log/tracing/struct.Dispatch.html "struct bevy::log::tracing::Dispatch")\>,

Attaches the provided [`Subscriber`](../log/tracing/trait.Subscriber.html "trait bevy::log::tracing::Subscriber") to this type, returning a [`WithDispatch`](../log/tracing/instrument/struct.WithDispatch.html "struct bevy::log::tracing::instrument::WithDispatch") wrapper. [Read more](../log/tracing/instrument/trait.WithSubscriber.html#method.with_subscriber)

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#228)

#### fn [with\_current\_subscriber](../log/tracing/instrument/trait.WithSubscriber.html#method.with_current_subscriber)(self) -> [WithDispatch](../log/tracing/instrument/struct.WithDispatch.html "struct bevy::log::tracing::instrument::WithDispatch")<Self> [ⓘ](#)

Attaches the current [default](../log/tracing/dispatcher/index.html#setting-the-default-subscriber "mod bevy::log::tracing::dispatcher") [`Subscriber`](../log/tracing/trait.Subscriber.html "trait bevy::log::tracing::Subscriber") to this type, returning a [`WithDispatch`](../log/tracing/instrument/struct.WithDispatch.html "struct bevy::log::tracing::instrument::WithDispatch") wrapper. [Read more](../log/tracing/instrument/trait.WithSubscriber.html#method.with_current_subscriber)

{"Either<Self, Self>":"<h3>Notable traits for <code><a class=\\"enum\\" href=\\"https://docs.rs/either/1.16.0/x86\_64-unknown-linux-gnu/either/enum.Either.html\\" title=\\"enum either::Either\\">Either</a>&lt;L, R&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;L, R&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"enum\\" href=\\"https://docs.rs/either/1.16.0/x86\_64-unknown-linux-gnu/either/enum.Either.html\\" title=\\"enum either::Either\\">Either</a>&lt;L, R&gt;<div class=\\"where\\">where\\n L: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>,\\n R: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&lt;Item = &lt;L as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>&gt;,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &lt;L as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>;</div><div class=\\"where\\">impl&lt;L, R&gt; <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"enum\\" href=\\"https://docs.rs/either/1.16.0/x86\_64-unknown-linux-gnu/either/enum.Either.html\\" title=\\"enum either::Either\\">Either</a>&lt;L, R&gt;<div class=\\"where\\">where\\n L: <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,\\n R: <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&lt;Output = &lt;L as <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>&gt;,</div></div><div class=\\"where\\"> type <a href=\\"../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;L as <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>","Instrumented<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../log/tracing/instrument/struct.Instrumented.html\\" title=\\"struct bevy::log::tracing::instrument::Instrumented\\">Instrumented</a>&lt;T&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;T&gt; <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"../log/tracing/instrument/struct.Instrumented.html\\" title=\\"struct bevy::log::tracing::instrument::Instrumented\\">Instrumented</a>&lt;T&gt;<div class=\\"where\\">where\\n T: <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,</div></div><div class=\\"where\\"> type <a href=\\"../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;T as <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>","WithDispatch<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../log/tracing/instrument/struct.WithDispatch.html\\" title=\\"struct bevy::log::tracing::instrument::WithDispatch\\">WithDispatch</a>&lt;T&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;T&gt; <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"../log/tracing/instrument/struct.WithDispatch.html\\" title=\\"struct bevy::log::tracing::instrument::WithDispatch\\">WithDispatch</a>&lt;T&gt;<div class=\\"where\\">where\\n T: <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,</div></div><div class=\\"where\\"> type <a href=\\"../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;T as <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>"}