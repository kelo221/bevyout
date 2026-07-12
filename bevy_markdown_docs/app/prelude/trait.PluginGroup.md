[bevy](../../index.html)::[app](../index.html)::[prelude](index.html)

# Trait PluginGroup 

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/plugin_group.rs.html#203)

```rust
pub trait PluginGroup: Sized {
    // Required method
    fn build(self) -> PluginGroupBuilder;

    // Provided methods
    fn name() -> String { ... }
    fn set<T>(self, plugin: T) -> PluginGroupBuilder
       where T: Plugin { ... }
}
```

Combines multiple [`Plugin`](../../prelude/trait.Plugin.html "trait bevy::prelude::Plugin")s into a single unit.

If you want an easier, but slightly more restrictive, method of implementing this trait, you may be interested in the [`plugin_group!`](../macro.plugin_group.html "macro bevy::app::plugin_group") macro.

## Required Methods

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/plugin_group.rs.html#205)

#### fn [build](#tymethod.build)(self) -> [PluginGroupBuilder](../struct.PluginGroupBuilder.html "struct bevy::app::PluginGroupBuilder")

Configures the [`Plugin`](../../prelude/trait.Plugin.html "trait bevy::prelude::Plugin")s that are to be added.

## Provided Methods

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/plugin_group.rs.html#207)

#### fn [name](#method.name)() -> [String](../../prelude/struct.String.html "struct bevy::prelude::String")

Configures a name for the [`PluginGroup`](../../prelude/trait.PluginGroup.html "trait bevy::prelude::PluginGroup") which is primarily used for debugging.

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/plugin_group.rs.html#211)

#### fn [set](#method.set)<T>(self, plugin: T) -> [PluginGroupBuilder](../struct.PluginGroupBuilder.html "struct bevy::app::PluginGroupBuilder")

where T: [Plugin](../../prelude/trait.Plugin.html "trait bevy::prelude::Plugin"),

Sets the value of the given [`Plugin`](../../prelude/trait.Plugin.html "trait bevy::prelude::Plugin"), if it exists

##### [Examples found in repository](#scraped-examples)[?](../../../scrape-examples-help.html)

tests/3d/no\_prepass.rs ([lines 7-10](../../../src/no_prepass/no_prepass.rs.html#7-10))

```rust
5fn main() {
6    App::new()
7        .add_plugins(DefaultPlugins.set(PbrPlugin {
8            prepass_enabled: false,
9            ..default()
10        }))
11        .run();
12}
```

Hide additional examples

examples/app/thread\_pool\_resources.rs ([lines 8-10](../../../src/thread_pool_resources/thread_pool_resources.rs.html#8-10))

```rust
6fn main() {
7    App::new()
8        .add_plugins(DefaultPlugins.set(TaskPoolPlugin {
9            task_pool_options: TaskPoolOptions::with_num_threads(4),
10        }))
11        .run();
12}
```

examples/asset/web\_asset.rs ([lines 10-12](../../../src/web_asset/web_asset.rs.html#10-12))

```rust
8fn main() {
9    App::new()
10        .add_plugins(DefaultPlugins.set(WebAssetPlugin {
11            silence_startup_warning: true,
12        }))
13        .add_systems(Startup, setup)
14        .run();
15}
```

examples/2d/tilemap\_chunk\_orientation.rs ([line 11](../../../src/tilemap_chunk_orientation/tilemap_chunk_orientation.rs.html#11))

```rust
9fn main() {
10    App::new()
11        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
12        .insert_resource(ClearColor(Color::srgb(0.5, 0.5, 0.9)))
13        .add_systems(Startup, setup)
14        .run();
15}
```

examples/2d/sprite\_sheet.rs ([line 8](../../../src/sprite_sheet/sprite_sheet.rs.html#8))

```rust
6fn main() {
7    App::new()
8        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest())) // prevents blurry sprites
9        .add_systems(Startup, setup)
10        .add_systems(Update, animate_sprite)
11        .run();
12}
```

examples/picking/sprite\_picking.rs ([line 9](../../../src/sprite_picking/sprite_picking.rs.html#9))

```rust
7fn main() {
8    App::new()
9        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
10        .add_systems(Startup, (setup, setup_atlas))
11        .add_systems(Update, (move_sprite, animate_sprite))
12        .run();
13}
```

Additional examples can be found in:  

*   [examples/app/log\_layers.rs](../../../src/log_layers/log_layers.rs.html#56-61)
*   [examples/2d/pixel\_grid\_snap.rs](../../../src/pixel_grid_snap/pixel_grid_snap.rs.html#29)
*   [examples/2d/tilemap\_chunk.rs](../../../src/tilemap_chunk/tilemap_chunk.rs.html#14)
*   [examples/shader\_advanced/texture\_binding\_array.rs](../../../src/texture_binding_array/texture_binding_array.rs.html#28)
*   [examples/window/monitor\_info.rs](../../../src/monitor_info/monitor_info.rs.html#11-15)
*   [examples/audio/decodable.rs](../../../src/decodable/decodable.rs.html#86-89)
*   [examples/app/no\_renderer.rs](../../../src/no_renderer/no_renderer.rs.html#15-22)
*   [examples/ui/layout/anchor\_layout.rs](../../../src/anchor_layout/anchor_layout.rs.html#8-14)
*   [examples/audio/spatial\_audio\_2d.rs](../../../src/spatial_audio_2d/spatial_audio_2d.rs.html#16-19)
*   [examples/ui/layout/flex\_layout.rs](../../../src/flex_layout/flex_layout.rs.html#10-16)
*   [examples/asset/asset\_settings.rs](../../../src/asset_settings/asset_settings.rs.html#12-15)
*   [examples/ui/layout/grid.rs](../../../src/grid/grid.rs.html#6-13)
*   [examples/2d/texture\_atlas.rs](../../../src/texture_atlas/texture_atlas.rs.html#14)
*   [examples/3d/fog\_volumes.rs](../../../src/fog_volumes/fog_volumes.rs.html#18-24)
*   [examples/3d/scrolling\_fog.rs](../../../src/scrolling_fog/scrolling_fog.rs.html#27-33)
*   [examples/ui/images/ui\_texture\_atlas.rs](../../../src/ui_texture_atlas/ui_texture_atlas.rs.html#7-12)
*   [examples/app/return\_after\_run.rs](../../../src/return_after_run/return_after_run.rs.html#13-19)
*   [examples/window/window\_drag\_move.rs](../../../src/window_drag_move/window_drag_move.rs.html#44-50)
*   [examples/3d/rect\_light.rs](../../../src/rect_light/rect_light.rs.html#9-15)
*   [examples/ui/text/text\_debug.rs](../../../src/text_debug/text_debug.rs.html#16-22)
*   [examples/window/scale\_factor\_override.rs](../../../src/scale_factor_override/scale_factor_override.rs.html#11-17)
*   [examples/app/log\_layers\_ecs.rs](../../../src/log_layers_ecs/log_layers_ecs.rs.html#27-34)
*   [tests/window/minimizing.rs](../../../src/minimizing/minimizing.rs.html#9-15)
*   [examples/app/logs.rs](../../../src/logs/logs.rs.html#7-12)
*   [examples/stress\_tests/many\_cameras\_lights.rs](../../../src/many_cameras_lights/many_cameras_lights.rs.html#15-22)
*   [examples/3d/depth\_of\_field.rs](../../../src/depth_of_field/depth_of_field.rs.html#55-61)
*   [examples/asset/asset\_saving.rs](../../../src/asset_saving/asset_saving.rs.html#22-27)
*   [examples/3d/anisotropy.rs](../../../src/anisotropy/anisotropy.rs.html#84-90)
*   [examples/3d/3d\_shapes.rs](../../../src/3d_shapes/3d_shapes.rs.html#31)
*   [examples/shader/shader\_prepass.rs](../../../src/shader_prepass/shader_prepass.rs.html#22-28)
*   [examples/app/settings.rs](../../../src/settings/settings.rs.html#22-30)
*   [examples/2d/sprite\_animation.rs](../../../src/sprite_animation/sprite_animation.rs.html#11)
*   [examples/window/persisting\_window\_settings.rs](../../../src/persisting_window_settings/persisting_window_settings.rs.html#20-28)
*   [examples/stress\_tests/text\_pipeline.rs](../../../src/text_pipeline/text_pipeline.rs.html#17-24)
*   [examples/gizmos/2d\_text\_gizmos.rs](../../../src/2d_text_gizmos/2d_text_gizmos.rs.html#20-27)
*   [examples/animation/animation\_masks.rs](../../../src/animation_masks/animation_masks.rs.html#97-103)
*   [examples/window/multi\_window\_text.rs](../../../src/multi_window_text/multi_window_text.rs.html#14-22)
*   [examples/3d/specular\_tint.rs](../../../src/specular_tint/specular_tint.rs.html#53-59)
*   [examples/ui/window\_fallthrough.rs](../../../src/window_fallthrough/window_fallthrough.rs.html#10-19)
*   [tests/window/resizing.rs](../../../src/resizing/resizing.rs.html#23-31)
*   [examples/shader/compute\_shader\_game\_of\_life.rs](../../../src/compute_shader_game_of_life/compute_shader_game_of_life.rs.html#37-45)
*   [examples/ui/text/text\_wrap\_debug.rs](../../../src/text_wrap_debug/text_wrap_debug.rs.html#35-38)
*   [examples/stress\_tests/many\_lights.rs](../../../src/many_lights/many_lights.rs.html#22-30)
*   [examples/gltf/custom\_gltf\_vertex\_attribute.rs](../../../src/custom_gltf_vertex_attribute/custom_gltf_vertex_attribute.rs.html#32-39)
*   [examples/3d/pccm.rs](../../../src/pccm/pccm.rs.html#56-62)
*   [examples/3d/visibility\_range.rs](../../../src/visibility_range/visibility_range.rs.html#88-94)
*   [examples/stress\_tests/many\_gizmos.rs](../../../src/many_gizmos/many_gizmos.rs.html#17-25)
*   [examples/animation/animation\_graph.rs](../../../src/animation_graph/animation_graph.rs.html#77-83)
*   [examples/asset/asset\_saving\_with\_subassets.rs](../../../src/asset_saving_with_subassets/asset_saving_with_subassets.rs.html#18-23)
*   [examples/window/transparent\_window.rs](../../../src/transparent_window/transparent_window.rs.html#13-26)
*   [examples/stress\_tests/many\_glyphs.rs](../../../src/many_glyphs/many_glyphs.rs.html#43-50)
*   [examples/stress\_tests/many\_animated\_sprite\_meshes.rs](../../../src/many_animated_sprite_meshes/many_animated_sprite_meshes.rs.html#27-34)
*   [examples/stress\_tests/many\_animated\_sprites.rs](../../../src/many_animated_sprites/many_animated_sprites.rs.html#25-32)
*   [examples/3d/contact\_shadows.rs](../../../src/contact_shadows/contact_shadows.rs.html#84-90)
*   [examples/gltf/gltf\_extension\_mesh\_2d.rs](../../../src/gltf_extension_mesh_2d/gltf_extension_mesh_2d.rs.html#38-45)
*   [examples/stress\_tests/many\_materials.rs](../../../src/many_materials/many_materials.rs.html#28-36)
*   [examples/stress\_tests/many\_sprite\_meshes.rs](../../../src/many_sprite_meshes/many_sprite_meshes.rs.html#38-45)
*   [examples/stress\_tests/many\_sprites.rs](../../../src/many_sprites/many_sprites.rs.html#36-43)
*   [examples/stress\_tests/many\_text2d.rs](../../../src/many_text2d/many_text2d.rs.html#76-83)
*   [examples/3d/ssr.rs](../../../src/ssr/ssr.rs.html#211-217)
*   [examples/picking/debug\_picking.rs](../../../src/debug_picking/debug_picking.rs.html#9-12)
*   [examples/usage/debug\_frustum\_culling.rs](../../../src/debug_frustum_culling/debug_frustum_culling.rs.html#46-52)
*   [tests/3d/test\_skinned\_mesh\_bounds.rs](../../../src/test_skinned_mesh_bounds/test_skinned_mesh_bounds.rs.html#17-23)
*   [examples/3d/occlusion\_culling.rs](../../../src/occlusion_culling/occlusion_culling.rs.html#170-176)
*   [examples/stress\_tests/many\_cubes.rs](../../../src/many_cubes/many_cubes.rs.html#128-135)
*   [examples/stress\_tests/many\_gradients.rs](../../../src/many_gradients/many_gradients.rs.html#62-70)
*   [examples/3d/pcss.rs](../../../src/pcss/pcss.rs.html#120-126)
*   [examples/3d/mixed\_lighting.rs](../../../src/mixed_lighting/mixed_lighting.rs.html#119-125)
*   [examples/app/headless.rs](../../../src/headless/headless.rs.html#28)
*   [examples/3d/clustered\_decals.rs](../../../src/clustered_decals/clustered_decals.rs.html#123-129)
*   [examples/3d/clustered\_decal\_maps.rs](../../../src/clustered_decal_maps/clustered_decal_maps.rs.html#134-136)
*   [examples/window/low\_power.rs](../../../src/low_power/low_power.rs.html#25-32)
*   [examples/3d/mirror.rs](../../../src/mirror/mirror.rs.html#113-119)
*   [examples/stress\_tests/many\_morph\_targets.rs](../../../src/many_morph_targets/many_morph_targets.rs.html#143-151)
*   [examples/showcase/desk\_toy.rs](../../../src/desk_toy/desk_toy.rs.html#21-30)
*   [examples/3d/tonemapping.rs](../../../src/tonemapping/tonemapping.rs.html#25-30)
*   [examples/stress\_tests/transform\_hierarchy.rs](../../../src/transform_hierarchy/transform_hierarchy.rs.html#191-195)
*   [examples/2d/wireframe\_2d.rs](../../../src/wireframe_2d/wireframe_2d.rs.html#23-31)
*   [examples/3d/wireframe.rs](../../../src/wireframe/wireframe.rs.html#24-32)
*   [examples/stress\_tests/many\_foxes.rs](../../../src/many_foxes/many_foxes.rs.html#50-58)
*   [examples/app/externally\_driven\_headless\_renderer.rs](../../../src/externally_driven_headless_renderer/externally_driven_headless_renderer.rs.html#57)
*   [examples/asset/processing/asset\_processing.rs](../../../src/asset_processing/asset_processing.rs.html#30-38)
*   [examples/stress\_tests/bevymark\_3d.rs](../../../src/bevymark_3d/bevymark_3d.rs.html#105-113)
*   [examples/3d/light\_textures.rs](../../../src/light_textures/light_textures.rs.html#109-115)
*   [examples/stress\_tests/bevymark.rs](../../../src/bevymark/bevymark.rs.html#136-144)
*   [examples/app/headless\_renderer.rs](../../../src/headless_renderer/headless_renderer.rs.html#87)
*   [examples/3d/light\_probe\_blending.rs](../../../src/light_probe_blending/light_probe_blending.rs.html#152-158)
*   [examples/2d/dynamic\_mip\_generation.rs](../../../src/dynamic_mip_generation/dynamic_mip_generation.rs.html#211-217)
*   [examples/window/window\_settings.rs](../../../src/window_settings/window_settings.rs.html#18-40)
*   [examples/stress\_tests/many\_buttons.rs](../../../src/many_buttons/many_buttons.rs.html#76-83)
*   [examples/testbed/ui.rs](../../../src/testbed_ui/ui.rs.html#26-35)
*   [examples/3d/irradiance\_volumes.rs](../../../src/irradiance_volumes/irradiance_volumes.rs.html#150-156)

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/lib.rs.html#287)

### impl [PluginGroup](../../prelude/trait.PluginGroup.html "trait bevy::prelude::PluginGroup") for [DefaultPickingPlugins](../../prelude/struct.DefaultPickingPlugins.html "struct bevy::prelude::DefaultPickingPlugins")

[Source](https://docs.rs/bevy_internal/0.19.0/x86_64-unknown-linux-gnu/src/bevy_internal/default_plugins.rs.html#3-114)

### impl [PluginGroup](../../prelude/trait.PluginGroup.html "trait bevy::prelude::PluginGroup") for [DefaultPlugins](../../struct.DefaultPlugins.html "struct bevy::DefaultPlugins")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/lib.rs.html#119)

### impl [PluginGroup](../../prelude/trait.PluginGroup.html "trait bevy::prelude::PluginGroup") for [FeathersPlugins](../../feathers/struct.FeathersPlugins.html "struct bevy::feathers::FeathersPlugins")

[Source](https://docs.rs/bevy_internal/0.19.0/x86_64-unknown-linux-gnu/src/bevy_internal/default_plugins.rs.html#161-190)

### impl [PluginGroup](../../prelude/trait.PluginGroup.html "trait bevy::prelude::PluginGroup") for [MinimalPlugins](../../struct.MinimalPlugins.html "struct bevy::MinimalPlugins")

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/plugin_group.rs.html#221)

### impl [PluginGroup](../../prelude/trait.PluginGroup.html "trait bevy::prelude::PluginGroup") for [PluginGroupBuilder](../struct.PluginGroupBuilder.html "struct bevy::app::PluginGroupBuilder")

[Source](https://docs.rs/bevy_solari/0.19.0/x86_64-unknown-linux-gnu/src/bevy_solari/lib.rs.html#41)

### impl [PluginGroup](../../prelude/trait.PluginGroup.html "trait bevy::prelude::PluginGroup") for [SolariPlugins](../../solari/struct.SolariPlugins.html "struct bevy::solari::SolariPlugins")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/lib.rs.html#63)

### impl [PluginGroup](../../prelude/trait.PluginGroup.html "trait bevy::prelude::PluginGroup") for [UiWidgetsPlugins](../../ui_widgets/struct.UiWidgetsPlugins.html "struct bevy::ui_widgets::UiWidgetsPlugins")