[bevy](../index.html)::[asset](index.html)

# Struct AssetServer 

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/server/mod.rs.html#66)

```rust
pub struct AssetServer { /* private fields */ }
```

Loads and tracks the state of [`Asset`](../prelude/trait.Asset.html "trait bevy::prelude::Asset") values from a configured [`AssetReader`](io/trait.AssetReader.html "trait bevy::asset::io::AssetReader"). This can be used to kick off new asset loads and retrieve their current load states.

The general process to load an asset is:

1.  Initialize a new [`Asset`](../prelude/trait.Asset.html "trait bevy::prelude::Asset") type with the [`AssetServer`](../prelude/struct.AssetServer.html "struct bevy::prelude::AssetServer") via [`AssetApp::init_asset`](../prelude/trait.AssetApp.html#tymethod.init_asset "method bevy::prelude::AssetApp::init_asset"), which will internally call [`AssetServer::register_asset`](../prelude/struct.AssetServer.html#method.register_asset "method bevy::prelude::AssetServer::register_asset") and set up related ECS [`Assets`](../prelude/struct.Assets.html "struct bevy::prelude::Assets") storage and systems.
2.  Register one or more [`AssetLoader`](trait.AssetLoader.html "trait bevy::asset::AssetLoader")s for that asset with [`AssetApp::init_asset_loader`](../prelude/trait.AssetApp.html#tymethod.init_asset_loader "method bevy::prelude::AssetApp::init_asset_loader")
3.  Add the asset to your asset folder (defaults to `assets`).
4.  Call [`AssetServer::load`](../prelude/struct.AssetServer.html#method.load "method bevy::prelude::AssetServer::load") with a path to your asset.

[`AssetServer`](../prelude/struct.AssetServer.html "struct bevy::prelude::AssetServer") can be cloned. It is backed by an [`Arc`](../platform/sync/struct.Arc.html "struct bevy::platform::sync::Arc") so clones will share state. Clones can be freely used in parallel.

## Implementations

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/server/mod.rs.html#91)

### impl [AssetServer](../prelude/struct.AssetServer.html "struct bevy::prelude::AssetServer")

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/server/mod.rs.html#93)

#### pub const [STARTED\_LOAD\_COUNT](#associatedconstant.STARTED_LOAD_COUNT): [DiagnosticPath](../diagnostic/struct.DiagnosticPath.html "struct bevy::diagnostic::DiagnosticPath")

The number of loads that have been started by the server.

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/server/mod.rs.html#97-102)

#### pub fn [new](#method.new)( sources: [Arc](../platform/sync/struct.Arc.html "struct bevy::platform::sync::Arc")<[AssetSources](io/struct.AssetSources.html "struct bevy::asset::io::AssetSources")\>, mode: [AssetServerMode](enum.AssetServerMode.html "enum bevy::asset::AssetServerMode"), watching\_for\_changes: [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html), unapproved\_path\_mode: [UnapprovedPathMode](enum.UnapprovedPathMode.html "enum bevy::asset::UnapprovedPathMode"), ) -> [AssetServer](../prelude/struct.AssetServer.html "struct bevy::prelude::AssetServer")

Create a new instance of [`AssetServer`](../prelude/struct.AssetServer.html "struct bevy::prelude::AssetServer"). If `watch_for_changes` is true, the [`AssetReader`](io/trait.AssetReader.html "trait bevy::asset::io::AssetReader") storage will watch for changes to asset sources and hot-reload them.

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/server/mod.rs.html#115-121)

#### pub fn [new\_with\_meta\_check](#method.new_with_meta_check)( sources: [Arc](../platform/sync/struct.Arc.html "struct bevy::platform::sync::Arc")<[AssetSources](io/struct.AssetSources.html "struct bevy::asset::io::AssetSources")\>, mode: [AssetServerMode](enum.AssetServerMode.html "enum bevy::asset::AssetServerMode"), meta\_check: [AssetMetaCheck](enum.AssetMetaCheck.html "enum bevy::asset::AssetMetaCheck"), watching\_for\_changes: [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html), unapproved\_path\_mode: [UnapprovedPathMode](enum.UnapprovedPathMode.html "enum bevy::asset::UnapprovedPathMode"), ) -> [AssetServer](../prelude/struct.AssetServer.html "struct bevy::prelude::AssetServer")

Create a new instance of [`AssetServer`](../prelude/struct.AssetServer.html "struct bevy::prelude::AssetServer"). If `watch_for_changes` is true, the [`AssetReader`](io/trait.AssetReader.html "trait bevy::asset::io::AssetReader") storage will watch for changes to asset sources and hot-reload them.

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/server/mod.rs.html#186-189)

#### pub fn [get\_source](#method.get_source)<'a>( &self, source: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[AssetSourceId](io/enum.AssetSourceId.html "enum bevy::asset::io::AssetSourceId")<'a>>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<&[AssetSource](io/struct.AssetSource.html "struct bevy::asset::io::AssetSource"), [MissingAssetSourceError](io/struct.MissingAssetSourceError.html "struct bevy::asset::io::MissingAssetSourceError")\>

Retrieves the [`AssetSource`](io/struct.AssetSource.html "struct bevy::asset::io::AssetSource") for the given `source`.

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/server/mod.rs.html#194)

#### pub fn [watching\_for\_changes](#method.watching_for_changes)(&self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Returns true if the [`AssetServer`](../prelude/struct.AssetServer.html "struct bevy::prelude::AssetServer") watches for changes.

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/server/mod.rs.html#199)

#### pub fn [register\_loader](#method.register_loader)<L>(&self, loader: L)

where L: [AssetLoader](trait.AssetLoader.html "trait bevy::asset::AssetLoader"),

Registers a new [`AssetLoader`](trait.AssetLoader.html "trait bevy::asset::AssetLoader"). [`AssetLoader`](trait.AssetLoader.html "trait bevy::asset::AssetLoader")s must be registered before they can be used.

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/server/mod.rs.html#204)

#### pub fn [register\_asset](#method.register_asset)<A>(&self, assets: &[Assets](../prelude/struct.Assets.html "struct bevy::prelude::Assets")<A>)

where A: [Asset](../prelude/trait.Asset.html "trait bevy::prelude::Asset"),

Registers a new [`Asset`](../prelude/trait.Asset.html "trait bevy::prelude::Asset") type. [`Asset`](../prelude/trait.Asset.html "trait bevy::prelude::Asset") types must be registered before assets of that type can be loaded.

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/server/mod.rs.html#244-247)

#### pub async fn [get\_asset\_loader\_with\_extension](#method.get_asset_loader_with_extension)( &self, extension: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Arc](../platform/sync/struct.Arc.html "struct bevy::platform::sync::Arc")<dyn [ErasedAssetLoader](trait.ErasedAssetLoader.html "trait bevy::asset::ErasedAssetLoader")\>, [MissingAssetLoaderForExtensionError](struct.MissingAssetLoaderForExtensionError.html "struct bevy::asset::MissingAssetLoaderForExtensionError")\>

Returns the registered [`AssetLoader`](trait.AssetLoader.html "trait bevy::asset::AssetLoader") associated with the given extension, if it exists.

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/server/mod.rs.html#260-263)

#### pub async fn [get\_asset\_loader\_with\_type\_name](#method.get_asset_loader_with_type_name)( &self, type\_name: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Arc](../platform/sync/struct.Arc.html "struct bevy::platform::sync::Arc")<dyn [ErasedAssetLoader](trait.ErasedAssetLoader.html "trait bevy::asset::ErasedAssetLoader")\>, [MissingAssetLoaderForTypeNameError](struct.MissingAssetLoaderForTypeNameError.html "struct bevy::asset::MissingAssetLoaderForTypeNameError")\>

Returns the registered [`AssetLoader`](trait.AssetLoader.html "trait bevy::asset::AssetLoader") associated with the given type name, if it exists.

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/server/mod.rs.html#276-279)

#### pub async fn [get\_path\_asset\_loader](#method.get_path_asset_loader)<'a>( &self, path: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[AssetPath](struct.AssetPath.html "struct bevy::asset::AssetPath")<'a>>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Arc](../platform/sync/struct.Arc.html "struct bevy::platform::sync::Arc")<dyn [ErasedAssetLoader](trait.ErasedAssetLoader.html "trait bevy::asset::ErasedAssetLoader")\>, [MissingAssetLoaderForExtensionError](struct.MissingAssetLoaderForExtensionError.html "struct bevy::asset::MissingAssetLoaderForExtensionError")\>

Retrieves the default [`AssetLoader`](trait.AssetLoader.html "trait bevy::asset::AssetLoader") for the given path, if one can be found.

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/server/mod.rs.html#302-305)

#### pub async fn [get\_asset\_loader\_with\_asset\_type\_id](#method.get_asset_loader_with_asset_type_id)( &self, type\_id: [TypeId](https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html "struct core::any::TypeId"), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Arc](../platform/sync/struct.Arc.html "struct bevy::platform::sync::Arc")<dyn [ErasedAssetLoader](trait.ErasedAssetLoader.html "trait bevy::asset::ErasedAssetLoader")\>, [MissingAssetLoaderForTypeIdError](struct.MissingAssetLoaderForTypeIdError.html "struct bevy::asset::MissingAssetLoaderForTypeIdError")\>

Retrieves the default [`AssetLoader`](trait.AssetLoader.html "trait bevy::asset::AssetLoader") for the given [`Asset`](../prelude/trait.Asset.html "trait bevy::prelude::Asset") [`TypeId`](https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html "struct core::any::TypeId"), if one can be found.

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/server/mod.rs.html#313-315)

#### pub async fn [get\_asset\_loader\_with\_asset\_type](#method.get_asset_loader_with_asset_type)<A>( &self, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Arc](../platform/sync/struct.Arc.html "struct bevy::platform::sync::Arc")<dyn [ErasedAssetLoader](trait.ErasedAssetLoader.html "trait bevy::asset::ErasedAssetLoader")\>, [MissingAssetLoaderForTypeIdError](struct.MissingAssetLoaderForTypeIdError.html "struct bevy::asset::MissingAssetLoaderForTypeIdError")\>

where A: [Asset](../prelude/trait.Asset.html "trait bevy::prelude::Asset"),

Retrieves the default [`AssetLoader`](trait.AssetLoader.html "trait bevy::asset::AssetLoader") for the given [`Asset`](../prelude/trait.Asset.html "trait bevy::prelude::Asset") type, if one can be found.

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/server/mod.rs.html#364)

#### pub fn [load](#method.load)<'a, A>(&self, path: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[AssetPath](struct.AssetPath.html "struct bevy::asset::AssetPath")<'a>>) -> [Handle](../prelude/enum.Handle.html "enum bevy::prelude::Handle")<A>

where A: [Asset](../prelude/trait.Asset.html "trait bevy::prelude::Asset"),

Begins loading an [`Asset`](../prelude/trait.Asset.html "trait bevy::prelude::Asset") of type `A` stored at `path`. This will not block on the asset load. Instead, it returns a “strong” [`Handle`](../prelude/enum.Handle.html "enum bevy::prelude::Handle"). When the [`Asset`](../prelude/trait.Asset.html "trait bevy::prelude::Asset") is loaded (and enters [`LoadState::Loaded`](enum.LoadState.html#variant.Loaded "variant bevy::asset::LoadState::Loaded")), it will be added to the associated [`Assets`](../prelude/struct.Assets.html "struct bevy::prelude::Assets") resource.

Note that if the asset at this path is already loaded, this function will return the existing handle, and will not waste work spawning a new load task.

In case the file path contains a hashtag (`#`), the `path` must be specified using [`Path`](https://doc.rust-lang.org/nightly/std/path/struct.Path.html "struct std::path::Path") or [`AssetPath`](struct.AssetPath.html "struct bevy::asset::AssetPath") because otherwise the hashtag would be interpreted as separator between the file path and the label. For example:

```rust
// `#path` is a label.
asset_server.load("some/file#path");

// `#path` is part of the file name.
asset_server.load(Path::new("some/file#path"));
```

Furthermore, if you need to load a file with a hashtag in its name _and_ a label, you can manually construct an [`AssetPath`](struct.AssetPath.html "struct bevy::asset::AssetPath").

```rust
asset_server.load(AssetPath::from_path(Path::new("some/file#path")).with_label("subasset"));
```

You can check the asset’s load state by reading [`AssetEvent`](../prelude/enum.AssetEvent.html "enum bevy::prelude::AssetEvent") events, calling [`AssetServer::load_state`](../prelude/struct.AssetServer.html#method.load_state "method bevy::prelude::AssetServer::load_state"), or checking the [`Assets`](../prelude/struct.Assets.html "struct bevy::prelude::Assets") storage to see if the [`Asset`](../prelude/trait.Asset.html "trait bevy::prelude::Asset") exists yet.

The asset load will fail and an error will be printed to the logs if the asset stored at `path` is not of type `A`.

##### [Examples found in repository](#scraped-examples)[?](../../scrape-examples-help.html)

examples/asset/asset\_saving\_with\_subassets.rs ([line 91](../../src/asset_saving_with_subassets/asset_saving_with_subassets.rs.html#91))

```rust
90fn start_load(mut commands: Commands, asset_server: Res<AssetServer>) {
91    commands.spawn(PendingLoad(asset_server.load(ASSET_PATH)));
92}
```

Hide additional examples

examples/state/states.rs ([line 122](../../src/states/states.rs.html#122))

```rust
121fn setup_game(mut commands: Commands, asset_server: Res<AssetServer>) {
122    commands.spawn(Sprite::from_image(asset_server.load("branding/icon.png")));
123}
```

examples/state/sub\_states.rs ([line 195](../../src/sub_states/sub_states.rs.html#195))

```rust
194    pub fn setup_game(mut commands: Commands, asset_server: Res<AssetServer>) {
195        commands.spawn(Sprite::from_image(asset_server.load("branding/icon.png")));
196    }
```

examples/audio/audio.rs ([line 15](../../src/audio/audio.rs.html#15))

```rust
13fn setup(asset_server: Res<AssetServer>, mut commands: Commands) {
14    commands.spawn(AudioPlayer::new(
15        asset_server.load("sounds/Windless Slopes.ogg"),
16    ));
17}
```

examples/asset/custom\_asset\_reader.rs ([line 64](../../src/custom_asset_reader/custom_asset_reader.rs.html#64))

```rust
62fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
63    commands.spawn(Camera2d);
64    commands.spawn(Sprite::from_image(asset_server.load("branding/icon.png")));
65}
```

examples/state/custom\_transitions.rs ([line 226](../../src/custom_transitions/custom_transitions.rs.html#226))

```rust
225fn setup_game(mut commands: Commands, asset_server: Res<AssetServer>) {
226    commands.spawn(Sprite::from_image(asset_server.load("branding/icon.png")));
227    info!("Setup game");
228}
```

Additional examples can be found in:  

*   [examples/window/transparent\_window.rs](../../src/transparent_window/transparent_window.rs.html#35)
*   [examples/2d/sprite.rs](../../src/sprite/sprite.rs.html#16)
*   [examples/3d/specular\_tint.rs](../../src/specular_tint/specular_tint.rs.html#35)
*   [examples/audio/play\_sound\_effect.rs](../../src/play_sound_effect/play_sound_effect.rs.html#16)
*   [examples/3d/light\_probe\_blending.rs](../../src/light_probe_blending/light_probe_blending.rs.html#262-264)
*   [examples/state/computed\_states.rs](../../src/computed_states/computed_states.rs.html#412)
*   [examples/shader/shader\_material\_wesl.rs](../../src/shader_material_wesl/shader_material_wesl.rs.html#45)
*   [examples/asset/asset\_decompression.rs](../../src/asset_decompression/asset_decompression.rs.html#110)
*   [examples/3d/mirror.rs](../../src/mirror/mirror.rs.html#285)
*   [examples/ecs/removal\_detection.rs](../../src/removal_detection/removal_detection.rs.html#32)
*   [examples/3d/reflection\_probes.rs](../../src/reflection_probes/reflection_probes.rs.html#115)
*   [examples/2d/move\_sprite.rs](../../src/move_sprite/move_sprite.rs.html#23)
*   [examples/2d/rotate\_to\_cursor.rs](../../src/rotate_to_cursor/rotate_to_cursor.rs.html#29)
*   [examples/camera/pan\_camera\_controller.rs](../../src/pan_camera_controller/pan_camera_controller.rs.html#40)
*   [examples/shader\_advanced/custom\_shader\_instancing.rs](../../src/custom_shader_instancing/custom_shader_instancing.rs.html#244)
*   [examples/shader\_advanced/custom\_render\_phase.rs](../../src/custom_render_phase/custom_render_phase.rs.html#176)
*   [tests/3d/test\_skinned\_mesh\_bounds.rs](../../src/test_skinned_mesh_bounds/test_skinned_mesh_bounds.rs.html#65)
*   [examples/gltf/gltf\_extension\_mesh\_2d.rs](../../src/gltf_extension_mesh_2d/gltf_extension_mesh_2d.rs.html#60)
*   [examples/3d/clearcoat.rs](../../src/clearcoat/clearcoat.rs.html#157)
*   [examples/asset/web\_asset.rs](../../src/web_asset/web_asset.rs.html#21)
*   [examples/shader\_advanced/specialized\_mesh\_pipeline.rs](../../src/specialized_mesh_pipeline/specialized_mesh_pipeline.rs.html#176)
*   [examples/2d/sprite\_flipping.rs](../../src/sprite_flipping/sprite_flipping.rs.html#16)
*   [examples/gltf/gltf\_extension\_animation\_graph.rs](../../src/gltf_extension_animation_graph/gltf_extension_animation_graph.rs.html#57)
*   [examples/scene/world\_serialization.rs](../../src/world_serialization/world_serialization.rs.html#126)
*   [examples/gltf/gltf\_skinned\_mesh.rs](../../src/gltf_skinned_mesh/gltf_skinned_mesh.rs.html#28-30)
*   [examples/3d/pccm.rs](../../src/pccm/pccm.rs.html#85)
*   [examples/asset/custom\_asset.rs](../../src/custom_asset/custom_asset.rs.html#115)
*   [examples/2d/dynamic\_mip\_generation.rs](../../src/dynamic_mip_generation/dynamic_mip_generation.rs.html#296)
*   [examples/asset/processing/asset\_processing.rs](../../src/asset_processing/asset_processing.rs.html#241)
*   [examples/2d/sprite\_tile.rs](../../src/sprite_tile/sprite_tile.rs.html#31)
*   [examples/gltf/query\_gltf\_primitives.rs](../../src/query_gltf_primitives/query_gltf_primitives.rs.html#65-67)
*   [examples/3d/lightmaps.rs](../../src/lightmaps/lightmaps.rs.html#43-45)
*   [examples/shader/shader\_material\_2d.rs](../../src/shader_material_2d/shader_material_2d.rs.html#39)
*   [examples/ui/scroll\_and\_overflow/overflow\_debug.rs](../../src/overflow_debug/overflow_debug.rs.html#143)
*   [examples/ecs/fallible\_params.rs](../../src/fallible_params/fallible_params.rs.html#68)
*   [examples/3d/color\_grading.rs](../../src/color_grading/color_grading.rs.html#126)
*   [examples/3d/ssr.rs](../../src/ssr/ssr.rs.html#269)
*   [examples/3d/light\_textures.rs](../../src/light_textures/light_textures.rs.html#211)
*   [examples/audio/soundtrack.rs](../../src/soundtrack/soundtrack.rs.html#56)
*   [examples/3d/occlusion\_culling.rs](../../src/occlusion_culling/occlusion_culling.rs.html#322)
*   [examples/gltf/custom\_gltf\_vertex\_attribute.rs](../../src/custom_gltf_vertex_attribute/custom_gltf_vertex_attribute.rs.html#52-58)
*   [examples/2d/pixel\_grid\_snap.rs](../../src/pixel_grid_snap/pixel_grid_snap.rs.html#54)
*   [examples/shader/shader\_material.rs](../../src/shader_material/shader_material.rs.html#29)
*   [examples/shader/shader\_material\_glsl.rs](../../src/shader_material_glsl/shader_material_glsl.rs.html#30)
*   [examples/3d/mixed\_lighting.rs](../../src/mixed_lighting/mixed_lighting.rs.html#169-172)
*   [examples/asset/extra\_source.rs](../../src/extra_asset_source/extra_source.rs.html#45)
*   [examples/shader\_advanced/texture\_binding\_array.rs](../../src/texture_binding_array/texture_binding_array.rs.html#67)
*   [examples/2d/sprite\_slice.rs](../../src/sprite_slice/sprite_slice.rs.html#109)
*   [examples/ui/window\_fallthrough.rs](../../src/window_fallthrough/window_fallthrough.rs.html#33)
*   [examples/asset/hot\_asset\_reloading.rs](../../src/hot_asset_reloading/hot_asset_reloading.rs.html#20)
*   [examples/ecs/parallel\_query.rs](../../src/parallel_query/parallel_query.rs.html#12)
*   [examples/audio/audio\_control.rs](../../src/audio_control/audio_control.rs.html#18)
*   [examples/2d/transparency\_2d.rs](../../src/transparency_2d/transparency_2d.rs.html#16)
*   [examples/window/window\_settings.rs](../../src/window_settings/window_settings.rs.html#171)
*   [examples/3d/clustered\_decals.rs](../../src/clustered_decals/clustered_decals.rs.html#219)
*   [examples/shader/gpu\_readback.rs](../../src/gpu_readback/gpu_readback.rs.html#186)
*   [examples/animation/morph\_targets.rs](../../src/morph_targets/morph_targets.rs.html#34)
*   [examples/3d/rotate\_environment\_map.rs](../../src/rotate_environment_map/rotate_environment_map.rs.html#116)
*   [examples/ui/widgets/standard\_widgets.rs](../../src/standard_widgets/standard_widgets.rs.html#212)
*   [examples/ui/widgets/standard\_widgets\_observers.rs](../../src/standard_widgets_observers/standard_widgets_observers.rs.html#143)
*   [examples/3d/clustered\_decal\_maps.rs](../../src/clustered_decal_maps/clustered_decal_maps.rs.html#52)
*   [examples/gltf/load\_gltf\_extras.rs](../../src/load_gltf_extras/load_gltf_extras.rs.html#31-33)
*   [examples/asset/embedded\_asset.rs](../../src/embedded_asset/embedded_asset.rs.html#57)
*   [examples/shader\_advanced/compute\_mesh.rs](../../src/compute_mesh/compute_mesh.rs.html#233)
*   [examples/2d/sprite\_sheet.rs](../../src/sprite_sheet/sprite_sheet.rs.html#47)
*   [examples/3d/tonemapping.rs](../../src/tonemapping/tonemapping.rs.html#81)
*   [examples/showcase/loading\_screen.rs](../../src/loading_screen/loading_screen.rs.html#145)
*   [examples/3d/order\_independent\_transparency.rs](../../src/order_independent_transparency/order_independent_transparency.rs.html#328)
*   [examples/3d/post\_processing.rs](../../src/post_processing/post_processing.rs.html#93)
*   [examples/showcase/game\_menu.rs](../../src/game_menu/game_menu.rs.html#75)
*   [examples/shader/shader\_material\_screenspace\_texture.rs](../../src/shader_material_screenspace_texture/shader_material_screenspace_texture.rs.html#37-39)
*   [examples/3d/depth\_of\_field.rs](../../src/depth_of_field/depth_of_field.rs.html#87-89)
*   [examples/ui/images/image\_node.rs](../../src/image_node/image_node.rs.html#30)
*   [examples/testbed/3d.rs](../../src/testbed_3d/3d.rs.html#268)
*   [examples/shader/shader\_material\_bindless.rs](../../src/shader_material_bindless/shader_material_bindless.rs.html#62)
*   [examples/gltf/edit\_material\_on\_gltf.rs](../../src/edit_material_on_gltf/edit_material_on_gltf.rs.html#34)
*   [examples/animation/animation\_masks.rs](../../src/animation_masks/animation_masks.rs.html#144)
*   [examples/ui/text/font\_atlas\_debug.rs](../../src/font_atlas_debug/font_atlas_debug.rs.html#85)
*   [examples/3d/anisotropy.rs](../../src/anisotropy/anisotropy.rs.html#110)
*   [examples/3d/animated\_material.rs](../../src/animated_material/animated_material.rs.html#23)
*   [examples/3d/pcss.rs](../../src/pcss/pcss.rs.html#180)
*   [examples/gltf/load\_gltf.rs](../../src/load_gltf/load_gltf.rs.html#23)
*   [examples/ui/widgets/button.rs](../../src/button/button.rs.html#104)
*   [examples/animation/animated\_mesh.rs](../../src/animated_mesh/animated_mesh.rs.html#42)
*   [examples/shader/extended\_material\_bindless.rs](../../src/extended_material_bindless/extended_material_bindless.rs.html#130)
*   [examples/shader/compute\_shader\_game\_of\_life.rs](../../src/compute_shader_game_of_life/compute_shader_game_of_life.rs.html#190)
*   [examples/gltf/update\_gltf\_scene.rs](../../src/update_gltf_scene/update_gltf_scene.rs.html#30)
*   [examples/stress\_tests/text\_pipeline.rs](../../src/text_pipeline/text_pipeline.rs.html#44)
*   [examples/3d/atmospheric\_fog.rs](../../src/atmospheric_fog/atmospheric_fog.rs.html#69-71)
*   [examples/3d/skybox.rs](../../src/skybox/skybox.rs.html#71)
*   [examples/3d/fog\_volumes.rs](../../src/fog_volumes/fog_volumes.rs.html#37)
*   [examples/window/custom\_cursor\_image.rs](../../src/custom_cursor_image/custom_cursor_image.rs.html#47)
*   [examples/2d/mesh2d\_vertex\_color\_texture.rs](../../src/mesh2d_vertex_color_texture/mesh2d_vertex_color_texture.rs.html#20)
*   [examples/ecs/hierarchy.rs](../../src/hierarchy/hierarchy.rs.html#21)
*   [examples/3d/generate\_custom\_mesh.rs](../../src/generate_custom_mesh/generate_custom_mesh.rs.html#33)
*   [examples/stress\_tests/many\_sprites.rs](../../src/many_sprites/many_sprites.rs.html#65)
*   [examples/stress\_tests/many\_sprite\_meshes.rs](../../src/many_sprite_meshes/many_sprite_meshes.rs.html#67)
*   [examples/audio/spatial\_audio\_2d.rs](../../src/spatial_audio_2d/spatial_audio_2d.rs.html#41)
*   [examples/ui/relative\_cursor\_position.rs](../../src/relative_cursor_position/relative_cursor_position.rs.html#52)
*   [examples/ui/ui\_material.rs](../../src/ui_material/ui_material.rs.html#50)
*   [examples/animation/animated\_mesh\_control.rs](../../src/animated_mesh_control/animated_mesh_control.rs.html#49)
*   [examples/ui/text/font\_weights.rs](../../src/font_weights/font_weights.rs.html#13)
*   [examples/testbed/2d.rs](../../src/testbed_2d/2d.rs.html#215)
*   [examples/showcase/contributors.rs](../../src/contributors/contributors.rs.html#91)
*   [examples/remote/app\_under\_test.rs](../../src/app_under_test/app_under_test.rs.html#104)
*   [examples/window/multiple\_windows.rs](../../src/multiple_windows/multiple_windows.rs.html#16)
*   [examples/ui/ui\_scaling.rs](../../src/ui_scaling/ui_scaling.rs.html#69)
*   [examples/usage/cooldown.rs](../../src/cooldown/cooldown.rs.html#28)
*   [examples/stress\_tests/many\_morph\_targets.rs](../../src/many_morph_targets/many_morph_targets.rs.html#231)
*   [examples/ui/images/ui\_texture\_atlas.rs](../../src/ui_texture_atlas/ui_texture_atlas.rs.html#29)
*   [examples/stress\_tests/many\_animated\_sprites.rs](../../src/many_animated_sprites/many_animated_sprites.rs.html#62)
*   [examples/2d/bloom\_2d.rs](../../src/bloom_2d/bloom_2d.rs.html#36)
*   [examples/stress\_tests/many\_animated\_sprite\_meshes.rs](../../src/many_animated_sprite_meshes/many_animated_sprite_meshes.rs.html#64)
*   [examples/ui/text/font\_variations.rs](../../src/font_variations/font_variations.rs.html#14)
*   [examples/animation/animated\_ui.rs](../../src/animated_ui/animated_ui.rs.html#165)
*   [examples/ui/layout/ghost\_nodes.rs](../../src/ghost_nodes/ghost_nodes.rs.html#26)
*   [examples/3d/atmosphere.rs](../../src/atmosphere/atmosphere.rs.html#133)
*   [examples/animation/animated\_mesh\_events.rs](../../src/animated_mesh_events/animated_mesh_events.rs.html#90)
*   [examples/audio/spatial\_audio\_3d.rs](../../src/spatial_audio_3d/spatial_audio_3d.rs.html#33)
*   [examples/stress\_tests/many\_buttons.rs](../../src/many_buttons/many_buttons.rs.html#155)
*   [examples/shader/automatic\_instancing.rs](../../src/automatic_instancing/automatic_instancing.rs.html#33)
*   [examples/asset/alter\_sprite.rs](../../src/alter_sprite/alter_sprite.rs.html#87)
*   [examples/ui/text/text\_input.rs](../../src/text_input/text_input.rs.html#65)
*   [examples/camera/projection\_zoom.rs](../../src/projection_zoom/projection_zoom.rs.html#81)
*   [examples/2d/rotation.rs](../../src/rotation/rotation.rs.html#53)
*   [examples/ui/images/ui\_texture\_slice.rs](../../src/ui_texture_slice/ui_texture_slice.rs.html#45)
*   [examples/camera/2d\_on\_ui.rs](../../src/2d_on_ui/2d_on_ui.rs.html#58)
*   [examples/3d/visibility\_range.rs](../../src/visibility_range/visibility_range.rs.html#130)
*   [examples/2d/sprite\_animation.rs](../../src/sprite_animation/sprite_animation.rs.html#104)
*   [examples/3d/volumetric\_fog.rs](../../src/volumetric_fog/volumetric_fog.rs.html#63-65)
*   [examples/animation/animation\_graph.rs](../../src/animation_graph/animation_graph.rs.html#161)
*   [examples/3d/texture.rs](../../src/texture/texture.rs.html#22)
*   [examples/3d/motion\_blur.rs](../../src/motion_blur/motion_blur.rs.html#114)
*   [examples/shader\_advanced/custom\_phase\_item.rs](../../src/custom_phase_item/custom_phase_item.rs.html#324)
*   [examples/time/virtual\_time.rs](../../src/virtual_time/virtual_time.rs.html#52)
*   [examples/shader\_advanced/custom\_post\_processing.rs](../../src/custom_post_processing/custom_post_processing.rs.html#201)
*   [examples/ui/layout/size\_constraints.rs](../../src/size_constraints/size_constraints.rs.html#47)
*   [examples/3d/anti\_aliasing.rs](../../src/anti_aliasing/anti_aliasing.rs.html#440-442)
*   [examples/testbed/ui.rs](../../src/testbed_ui/ui.rs.html#181)
*   [examples/transforms/align.rs](../../src/align/align.rs.html#86)
*   [examples/ui/styling/transparency\_ui.rs](../../src/transparency_ui/transparency_ui.rs.html#17)
*   [examples/ui/layout/anchor\_layout.rs](../../src/anchor_layout/anchor_layout.rs.html#20)
*   [examples/3d/decal.rs](../../src/decal/decal.rs.html#33)
*   [examples/ui/images/ui\_texture\_atlas\_slice.rs](../../src/ui_texture_atlas_slice/ui_texture_atlas_slice.rs.html#52)
*   [examples/picking/sprite\_picking.rs](../../src/sprite_picking/sprite_picking.rs.html#72)
*   [examples/shader\_advanced/manual\_material.rs](../../src/manual_material/manual_material.rs.html#203)
*   [examples/showcase/stepping.rs](../../src/breakout/stepping.rs.html#123)
*   [examples/2d/mesh2d\_alpha\_mode.rs](../../src/mesh2d_alpha_mode/mesh2d_alpha_mode.rs.html#25)
*   [examples/2d/mesh2d\_arcs.rs](../../src/mesh2d_arcs/mesh2d_arcs.rs.html#40)
*   [examples/ui/text/text\_wrap\_debug.rs](../../src/text_wrap_debug/text_wrap_debug.rs.html#48)
*   [examples/showcase/alien\_cake\_addict.rs](../../src/alien_cake_addict/alien_cake_addict.rs.html#136)
*   [examples/asset/repeated\_texture.rs](../../src/repeated_texture/repeated_texture.rs.html#24)
*   [examples/asset/asset\_settings.rs](../../src/asset_settings/asset_settings.rs.html#28)
*   [examples/2d/mesh2d\_repeated\_texture.rs](../../src/mesh2d_repeated_texture/mesh2d_repeated_texture.rs.html#34)
*   [examples/asset/alter\_mesh.rs](../../src/alter_mesh/alter_mesh.rs.html#92-98)
*   [examples/3d/meshlet.rs](../../src/meshlet/meshlet.rs.html#45)
*   [examples/3d/pbr.rs](../../src/pbr/pbr.rs.html#119)
*   [examples/3d/auto\_exposure.rs](../../src/auto_exposure/auto_exposure.rs.html#39)
*   [examples/asset/asset\_loading.rs](../../src/asset_loading/asset_loading.rs.html#23-29)
*   [examples/ui/scroll\_and\_overflow/overflow.rs](../../src/overflow/overflow.rs.html#18)
*   [examples/ui/scroll\_and\_overflow/overflow\_clip\_margin.rs](../../src/overflow_clip_margin/overflow_clip_margin.rs.html#15)
*   [examples/ui/layout/flex\_layout.rs](../../src/flex_layout/flex_layout.rs.html#22)
*   [examples/stress\_tests/bevymark\_3d.rs](../../src/bevymark_3d/bevymark_3d.rs.html#197)
*   [examples/shader/shader\_prepass.rs](../../src/shader_prepass/shader_prepass.rs.html#84)
*   [examples/ui/widgets/vertical\_slider.rs](../../src/vertical_slider/vertical_slider.rs.html#67)
*   [examples/stress\_tests/bevymark.rs](../../src/bevymark/bevymark.rs.html#234)
*   [examples/ui/layout/display\_and\_visibility.rs](../../src/display_and_visibility/display_and_visibility.rs.html#78)
*   [examples/3d/split\_screen.rs](../../src/split_screen/split_screen.rs.html#31)
*   [examples/showcase/breakout.rs](../../src/breakout/breakout.rs.html#182)
*   [examples/ui/scroll\_and\_overflow/scroll.rs](../../src/scroll/scroll.rs.html#115)
*   [examples/3d/solari.rs](../../src/solari/solari.rs.html#88-91)
*   [examples/ui/images/image\_node\_resizing.rs](../../src/image_node_resizing/image_node_resizing.rs.html#63)
*   [examples/ui/text/strikethrough\_and\_underline.rs](../../src/strikethrough_and_underline/strikethrough_and_underline.rs.html#22)
*   [examples/showcase/desk\_toy.rs](../../src/desk_toy/desk_toy.rs.html#109)
*   [examples/3d/parallax\_mapping.rs](../../src/parallax_mapping/parallax_mapping.rs.html#256)
*   [examples/2d/texture\_atlas.rs](../../src/texture_atlas/texture_atlas.rs.html#120)
*   [examples/2d/sprite\_scale.rs](../../src/sprite_scale/sprite_scale.rs.html#18)
*   [examples/stress\_tests/many\_foxes.rs](../../src/many_foxes/many_foxes.rs.html#124)
*   [examples/ui/text/multiple\_text\_inputs.rs](../../src/multiple_text_inputs/multiple_text_inputs.rs.html#47)
*   [examples/3d/deferred\_rendering.rs](../../src/deferred_rendering/deferred_rendering.rs.html#49)
*   [examples/ui/styling/box\_shadow.rs](../../src/box_shadow/box_shadow.rs.html#277)
*   [examples/3d/contact\_shadows.rs](../../src/contact_shadows/contact_shadows.rs.html#120)
*   [examples/ui/text/text.rs](../../src/text/text.rs.html#39)
*   [examples/2d/text2d.rs](../../src/text2d/text2d.rs.html#37)
*   [examples/animation/custom\_skinned\_mesh.rs](../../src/custom_skinned_mesh/custom_skinned_mesh.rs.html#179)
*   [examples/3d/blend\_modes.rs](../../src/blend_modes/blend_modes.rs.html#164)
*   [examples/3d/lighting.rs](../../src/lighting/lighting.rs.html#90)
*   [examples/ui/text/letter\_spacing.rs](../../src/letter_spacing/letter_spacing.rs.html#33)
*   [examples/ui/text/text\_debug.rs](../../src/text_debug/text_debug.rs.html#34)
*   [examples/ui/layout/grid.rs](../../src/grid/grid.rs.html#19)
*   [examples/3d/transmission.rs](../../src/transmission/transmission.rs.html#318)
*   [examples/ui/ui\_transform.rs](../../src/ui_transform/ui_transform.rs.html#221)
*   [examples/ui/text/font\_query.rs](../../src/font_query/font_query.rs.html#15)
*   [examples/ui/text/multiline\_text\_input.rs](../../src/multiline_text_input/multiline_text_input.rs.html#74)
*   [examples/testbed/full\_ui.rs](../../src/testbed_full_ui/full_ui.rs.html#74)

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/server/mod.rs.html#371)

#### pub fn [load\_builder](#method.load_builder)(&self) -> [LoadBuilder](struct.LoadBuilder.html "struct bevy::asset::LoadBuilder")<'\_>

Returns a [`LoadBuilder`](struct.LoadBuilder.html "struct bevy::asset::LoadBuilder") that can be used to start more complex loads. See [`LoadBuilder`](struct.LoadBuilder.html "struct bevy::asset::LoadBuilder") for details.

##### [Examples found in repository](#scraped-examples-1)[?](../../scrape-examples-help.html)

examples/3d/clustered\_decal\_maps.rs ([line 54](../../src/clustered_decal_maps/clustered_decal_maps.rs.html#54))

```rust
48    fn from_world(world: &mut World) -> Self {
49        // Load all the decal textures.
50        let asset_server = world.resource::<AssetServer>();
51        AppTextures {
52            decal_base_color_texture: asset_server.load("branding/bevy_bird_dark.png"),
53            decal_normal_map_texture: asset_server
54                .load_builder()
55                .with_settings(|settings: &mut ImageLoaderSettings| settings.is_srgb = false)
56                .load(get_web_asset_url("BevyLogo-Normal.png")),
57            decal_metallic_roughness_map_texture: asset_server
58                .load_builder()
59                .with_settings(|settings: &mut ImageLoaderSettings| settings.is_srgb = false)
60                .load(get_web_asset_url("BevyLogo-MetallicRoughness.png")),
61            decal_emissive_texture: asset_server.load(get_web_asset_url("BevyLogo-Emissive.png")),
62        }
63    }
64}
65
66/// A component that we place on our decals to track them for animation
67/// purposes.
68#[derive(Component)]
69struct ExampleDecal {
70    /// The width and height of the square decal in meters.
71    size: f32,
72    /// What state the decal is in (animating in, idling, or animating out).
73    state: ExampleDecalState,
74}
75
76/// The animation state of a decal.
77///
78/// When each [`Timer`] goes off, the decal advances to the next state.
79enum ExampleDecalState {
80    /// The decal has just been spawned and is animating in.
81    AnimatingIn(Timer),
82    /// The decal has animated in and is waiting to animate out.
83    Idling(Timer),
84    /// The decal is animating out.
85    ///
86    /// When this timer expires, the decal is despawned.
87    AnimatingOut(Timer),
88}
89
90/// All settings that the user can change.
91///
92/// This app only has one: whether newly-spawned decals are emissive.
93#[derive(Clone, Copy, PartialEq)]
94enum AppSetting {
95    /// True if newly-spawned decals have an emissive channel (i.e. they glow),
96    /// or false otherwise.
97    EmissiveDecals(bool),
98}
99
100/// The current values of the settings that the user can change.
101///
102/// This app only has one: whether newly-spawned decals are emissive.
103#[derive(Default, Resource)]
104struct AppStatus {
105    /// True if newly-spawned decals have an emissive channel (i.e. they glow),
106    /// or false otherwise.
107    emissive_decals: bool,
108}
109
110/// Half of the width and height of the plane onto which the decals are
111/// projected.
112const PLANE_HALF_SIZE: f32 = 2.0;
113/// The minimum width and height that a decal may have.
114///
115/// The actual size is determined randomly, using this value as a lower bound.
116const DECAL_MIN_SIZE: f32 = 0.5;
117/// The maximum width and height that a decal may have.
118///
119/// The actual size is determined randomly, using this value as an upper bound.
120const DECAL_MAX_SIZE: f32 = 1.5;
121
122/// How long it takes the decal to grow to its full size when animating in.
123const DECAL_ANIMATE_IN_DURATION: Duration = Duration::from_millis(300);
124/// How long a decal stays in the idle state before starting to animate out.
125const DECAL_IDLE_DURATION: Duration = Duration::from_secs(10);
126/// How long it takes the decal to shrink down to nothing when animating out.
127const DECAL_ANIMATE_OUT_DURATION: Duration = Duration::from_millis(300);
128
129/// The demo entry point.
130fn main() {
131    App::new()
132        .add_plugins(
133            DefaultPlugins
134                .set(WebAssetPlugin {
135                    silence_startup_warning: true,
136                })
137                .set(WindowPlugin {
138                    primary_window: Some(Window {
139                        title: "Bevy Clustered Decal Maps Example".into(),
140                        ..default()
141                    }),
142                    ..default()
143                }),
144        )
145        .add_message::<WidgetClickEvent<AppSetting>>()
146        .init_resource::<AppStatus>()
147        .init_resource::<AppTextures>()
148        .add_systems(Startup, setup)
149        .add_systems(Update, draw_gizmos)
150        .add_systems(Update, spawn_decal)
151        .add_systems(Update, animate_decals)
152        .add_systems(
153            Update,
154            (
155                widgets::handle_ui_interactions::<AppSetting>,
156                update_radio_buttons,
157            ),
158        )
159        .add_systems(
160            Update,
161            handle_emission_type_change.after(widgets::handle_ui_interactions::<AppSetting>),
162        )
163        .insert_resource(SeededRng(ChaCha8Rng::seed_from_u64(19878367467712)))
164        .run();
165}
166
167#[derive(Resource)]
168struct SeededRng(ChaCha8Rng);
169
170/// Spawns all the objects in the scene.
171fn setup(
172    mut commands: Commands,
173    asset_server: Res<AssetServer>,
174    mut meshes: ResMut<Assets<Mesh>>,
175    mut materials: ResMut<Assets<StandardMaterial>>,
176) {
177    spawn_plane_mesh(&mut commands, &asset_server, &mut meshes, &mut materials);
178    spawn_light(&mut commands);
179    spawn_camera(&mut commands);
180    spawn_buttons(&mut commands);
181}
182
183/// Spawns the plane onto which the decals are projected.
184fn spawn_plane_mesh(
185    commands: &mut Commands,
186    asset_server: &AssetServer,
187    meshes: &mut Assets<Mesh>,
188    materials: &mut Assets<StandardMaterial>,
189) {
190    // Create a plane onto which we project decals.
191    //
192    // As the plane has a normal map, we must generate tangents for the
193    // vertices.
194    let plane_mesh = meshes.add(
195        Plane3d {
196            normal: Dir3::NEG_Z,
197            half_size: Vec2::splat(PLANE_HALF_SIZE),
198        }
199        .mesh()
200        .build()
201        .with_duplicated_vertices()
202        .with_computed_flat_normals()
203        .with_generated_tangents()
204        .unwrap(),
205    );
206
207    // Give the plane some texture.
208    //
209    // Note that, as this is a normal map, we must disable sRGB when loading.
210    let normal_map_texture = asset_server
211        .load_builder()
212        .with_settings(|settings: &mut ImageLoaderSettings| settings.is_srgb = false)
213        .load("textures/ScratchedGold-Normal.png");
214
215    // Actually spawn the plane.
216    commands.spawn((
217        Mesh3d(plane_mesh),
218        MeshMaterial3d(materials.add(StandardMaterial {
219            base_color: Color::from(CRIMSON),
220            normal_map_texture: Some(normal_map_texture),
221            ..StandardMaterial::default()
222        })),
223        Transform::IDENTITY,
224    ));
225}
```

Hide additional examples

examples/3d/rotate\_environment\_map.rs ([line 76](../../src/rotate_environment_map/rotate_environment_map.rs.html#76))

```rust
62fn spawn_sphere(
63    commands: &mut Commands,
64    materials: &mut Assets<StandardMaterial>,
65    asset_server: &AssetServer,
66    sphere_mesh: &Handle<Mesh>,
67) {
68    commands.spawn((
69        Mesh3d(sphere_mesh.clone()),
70        MeshMaterial3d(
71            materials.add(StandardMaterial {
72                clearcoat: 1.0,
73                clearcoat_perceptual_roughness: 0.3,
74                clearcoat_normal_texture: Some(
75                    asset_server
76                        .load_builder()
77                        .with_settings(|settings: &mut ImageLoaderSettings| {
78                            settings.is_srgb = false;
79                        })
80                        .load("textures/ScratchedGold-Normal.png"),
81                ),
82                metallic: 0.9,
83                perceptual_roughness: 0.1,
84                base_color: GOLD.into(),
85                ..default()
86            }),
87        ),
88        Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3::splat(1.25)),
89    ));
90}
```

examples/3d/clearcoat.rs ([line 109](../../src/clearcoat/clearcoat.rs.html#109))

```rust
94fn spawn_car_paint_sphere(
95    commands: &mut Commands,
96    materials: &mut Assets<StandardMaterial>,
97    asset_server: &AssetServer,
98    sphere: &Handle<Mesh>,
99) {
100    commands
101        .spawn((
102            Mesh3d(sphere.clone()),
103            MeshMaterial3d(
104                materials.add(StandardMaterial {
105                    clearcoat: 1.0,
106                    clearcoat_perceptual_roughness: 0.1,
107                    normal_map_texture: Some(
108                        asset_server
109                            .load_builder()
110                            .with_settings(|settings: &mut ImageLoaderSettings| {
111                                settings.is_srgb = false;
112                            })
113                            .load("textures/BlueNoise-Normal.png"),
114                    ),
115                    metallic: 0.9,
116                    perceptual_roughness: 0.5,
117                    base_color: BLUE.into(),
118                    ..default()
119                }),
120            ),
121            Transform::from_xyz(-1.0, 1.0, 0.0).with_scale(Vec3::splat(SPHERE_SCALE)),
122        ))
123        .insert(ExampleSphere);
124}
125
126/// Spawn a semitransparent object with a clearcoat layer.
127fn spawn_coated_glass_bubble_sphere(
128    commands: &mut Commands,
129    materials: &mut Assets<StandardMaterial>,
130    sphere: &Handle<Mesh>,
131) {
132    commands
133        .spawn((
134            Mesh3d(sphere.clone()),
135            MeshMaterial3d(materials.add(StandardMaterial {
136                clearcoat: 1.0,
137                clearcoat_perceptual_roughness: 0.1,
138                metallic: 0.5,
139                perceptual_roughness: 0.1,
140                base_color: Color::srgba(0.9, 0.9, 0.9, 0.3),
141                alpha_mode: AlphaMode::Blend,
142                ..default()
143            })),
144            Transform::from_xyz(-1.0, -1.0, 0.0).with_scale(Vec3::splat(SPHERE_SCALE)),
145        ))
146        .insert(ExampleSphere);
147}
148
149/// Spawns an object with both a clearcoat normal map (a scratched varnish) and
150/// a main layer normal map (the golf ball pattern).
151///
152/// This object is in glTF format, using the `KHR_materials_clearcoat`
153/// extension.
154fn spawn_golf_ball(commands: &mut Commands, asset_server: &AssetServer) {
155    commands.spawn((
156        WorldAssetRoot(
157            asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/GolfBall/GolfBall.glb")),
158        ),
159        Transform::from_xyz(1.0, 1.0, 0.0).with_scale(Vec3::splat(SPHERE_SCALE)),
160        ExampleSphere,
161    ));
162}
163
164/// Spawns an object with only a clearcoat normal map (a scratch pattern) and no
165/// main layer normal map.
166fn spawn_scratched_gold_ball(
167    commands: &mut Commands,
168    materials: &mut Assets<StandardMaterial>,
169    asset_server: &AssetServer,
170    sphere: &Handle<Mesh>,
171) {
172    commands
173        .spawn((
174            Mesh3d(sphere.clone()),
175            MeshMaterial3d(
176                materials.add(StandardMaterial {
177                    clearcoat: 1.0,
178                    clearcoat_perceptual_roughness: 0.3,
179                    clearcoat_normal_texture: Some(
180                        asset_server
181                            .load_builder()
182                            .with_settings(|settings: &mut ImageLoaderSettings| {
183                                settings.is_srgb = false;
184                            })
185                            .load("textures/ScratchedGold-Normal.png"),
186                    ),
187                    metallic: 0.9,
188                    perceptual_roughness: 0.1,
189                    base_color: GOLD.into(),
190                    ..default()
191                }),
192            ),
193            Transform::from_xyz(1.0, -1.0, 0.0).with_scale(Vec3::splat(SPHERE_SCALE)),
194        ))
195        .insert(ExampleSphere);
196}
```

examples/asset/multi\_asset\_sync.rs ([line 147](../../src/multi_asset_sync/multi_asset_sync.rs.html#147))

```rust
144fn setup_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
145    let (barrier, guard) = AssetBarrier::new();
146    commands.insert_resource(OneHundredThings(std::array::from_fn(|i| {
147        let builder = asset_server.load_builder().with_guard(guard.clone());
148        match i % 5 {
149            0 => builder.load("models/GolfBall/GolfBall.glb"),
150            1 => builder.load("models/AlienCake/alien.glb"),
151            2 => builder.load("models/AlienCake/cakeBirthday.glb"),
152            3 => builder.load("models/FlightHelmet/FlightHelmet.gltf"),
153            4 => builder.load("models/torus/torus.gltf"),
154            _ => unreachable!(),
155        }
156    })));
157    let future = barrier.wait_async();
158    commands.insert_resource(barrier);
159
160    let loading_state = Arc::new(AtomicBool::new(false));
161    commands.insert_resource(AsyncLoadingState(loading_state.clone()));
162
163    // await the `AssetBarrierFuture`.
164    AsyncComputeTaskPool::get()
165        .spawn(async move {
166            future.await;
167            // Notify via `AsyncLoadingState`
168            loading_state.store(true, Ordering::Release);
169        })
170        .detach();
171}
```

examples/shader/array\_texture.rs ([line 40](../../src/array_texture/array_texture.rs.html#40))

```rust
32fn setup(
33    mut commands: Commands,
34    mut meshes: ResMut<Assets<Mesh>>,
35    mut materials: ResMut<Assets<ArrayTextureMaterial>>,
36    asset_server: Res<AssetServer>,
37) {
38    // Load the texture.
39    let array_texture = asset_server
40        .load_builder()
41        .with_settings(|settings: &mut ImageLoaderSettings| {
42            settings.array_layout = Some(ImageArrayLayout::RowCount {
43                rows: TEXTURE_COUNT,
44            });
45        })
46        .load("textures/array_texture.png");
47
48    // light
49    commands.spawn((
50        DirectionalLight::default(),
51        Transform::from_xyz(3.0, 2.0, 1.0).looking_at(Vec3::ZERO, Vec3::Y),
52    ));
53
54    // camera
55    commands.spawn((
56        Camera3d::default(),
57        Transform::from_xyz(5.0, 5.0, 5.0).looking_at(Vec3::new(1.5, 0.0, 0.0), Vec3::Y),
58    ));
59
60    // Spawn some cubes using the array texture.
61    let mesh_handle = meshes.add(Cuboid::default());
62    let material_handle = materials.add(ArrayTextureMaterial { array_texture });
63    for x in -5..=5 {
64        commands.spawn((
65            Mesh3d(mesh_handle.clone()),
66            MeshMaterial3d(material_handle.clone()),
67            // Pass a different mesh tag to allow selecting different layers of
68            // the array texture in the shader.
69            MeshTag(x as u32 % TEXTURE_COUNT),
70            Transform::from_xyz(x as f32 + 0.5, 0.0, 0.0),
71        ));
72    }
73}
```

examples/2d/tilemap\_chunk.rs ([line 49](../../src/tilemap_chunk/tilemap_chunk.rs.html#49))

```rust
26fn setup(mut commands: Commands, assets: Res<AssetServer>) {
27    // We're seeding the PRNG here to make this example deterministic for testing purposes.
28    // This isn't strictly required in practical use unless you need your app to be deterministic.
29    let mut rng = ChaCha8Rng::seed_from_u64(42);
30
31    let chunk_size = UVec2::splat(64);
32    let tile_display_size = UVec2::splat(8);
33    let tile_data: Vec<Option<TileData>> = (0..chunk_size.element_product())
34        .map(|_| rng.random_range(0..5))
35        .map(|i| {
36            if i == 0 {
37                None
38            } else {
39                Some(TileData::from_tileset_index(i - 1))
40            }
41        })
42        .collect();
43
44    commands.spawn((
45        TilemapChunk {
46            chunk_size,
47            tile_display_size,
48            tileset: assets
49                .load_builder()
50                .with_settings(|settings: &mut ImageLoaderSettings| {
51                    // The tileset texture is expected to be an array of tile textures, so we tell the
52                    // `ImageLoader` that our texture is composed of 4 stacked tile images.
53                    settings.array_layout = Some(ImageArrayLayout::RowCount { rows: 4 });
54                })
55                .load("textures/array_texture.png"),
56            ..default()
57        },
58        TilemapChunkTileData(tile_data),
59        UpdateTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
60    ));
61
62    commands.spawn(Camera2d);
63
64    commands.insert_resource(SeededRng(rng));
65}
```

Additional examples can be found in:  

*   [examples/testbed/3d.rs](../../src/testbed_3d/3d.rs.html#467)
*   [examples/3d/deferred\_rendering.rs](../../src/deferred_rendering/deferred_rendering.rs.html#222)
*   [examples/asset/alter\_sprite.rs](../../src/alter_sprite/alter_sprite.rs.html#54)
*   [examples/3d/atmosphere.rs](../../src/atmosphere/atmosphere.rs.html#277)
*   [examples/3d/ssr.rs](../../src/ssr/ssr.rs.html#381)
*   [examples/2d/tilemap\_chunk\_orientation.rs](../../src/tilemap_chunk_orientation/tilemap_chunk_orientation.rs.html#65)
*   [examples/ui/images/ui\_texture\_slice\_flip\_and\_tile.rs](../../src/ui_texture_slice_flip_and_tile/ui_texture_slice_flip_and_tile.rs.html#19)
*   [examples/3d/scrolling\_fog.rs](../../src/scrolling_fog/scrolling_fog.rs.html#93)
*   [examples/asset/repeated\_texture.rs](../../src/repeated_texture/repeated_texture.rs.html#43)
*   [examples/asset/asset\_saving.rs](../../src/asset_saving/asset_saving.rs.html#108)
*   [examples/asset/asset\_settings.rs](../../src/asset_settings/asset_settings.rs.html#66)
*   [examples/2d/mesh2d\_repeated\_texture.rs](../../src/mesh2d_repeated_texture/mesh2d_repeated_texture.rs.html#36)
*   [examples/asset/alter\_mesh.rs](../../src/alter_mesh/alter_mesh.rs.html#59)
*   [examples/3d/parallax\_mapping.rs](../../src/parallax_mapping/parallax_mapping.rs.html#210)
*   [examples/3d/solari.rs](../../src/solari/solari.rs.html#247)

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/server/mod.rs.html#383)

#### pub fn [load\_override](#method.load_override)<'a, A>(&self, path: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[AssetPath](struct.AssetPath.html "struct bevy::asset::AssetPath")<'a>>) -> [Handle](../prelude/enum.Handle.html "enum bevy::prelude::Handle")<A>

where A: [Asset](../prelude/trait.Asset.html "trait bevy::prelude::Asset"),

👎Deprecated:

Use `asset_server.load_builder().override_unapproved().load(path)` instead

Same as [`load`](../prelude/struct.AssetServer.html#method.load "method bevy::prelude::AssetServer::load"), but you can load assets from unapproved paths if [`AssetPlugin::unapproved_path_mode`](../prelude/struct.AssetPlugin.html#structfield.unapproved_path_mode "field bevy::prelude::AssetPlugin::unapproved_path_mode") is [`Deny`](enum.UnapprovedPathMode.html#variant.Deny "variant bevy::asset::UnapprovedPathMode::Deny").

See [`UnapprovedPathMode`](enum.UnapprovedPathMode.html "enum bevy::asset::UnapprovedPathMode") and [`AssetPath::is_unapproved`](struct.AssetPath.html#method.is_unapproved "method bevy::asset::AssetPath::is_unapproved")

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/server/mod.rs.html#390-394)

#### pub fn [load\_erased](#method.load_erased)<'a>( &self, type\_id: [TypeId](https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html "struct core::any::TypeId"), path: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[AssetPath](struct.AssetPath.html "struct bevy::asset::AssetPath")<'a>>, ) -> [UntypedHandle](../prelude/enum.UntypedHandle.html "enum bevy::prelude::UntypedHandle")

👎Deprecated:

Use `asset_server.load_builder().load_erased(type_id, path)` instead

Same as [`load`](../prelude/struct.AssetServer.html#method.load "method bevy::prelude::AssetServer::load"), but the type of the asset to load is specified by the runtime `type_id`.

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/server/mod.rs.html#415-419)

#### pub fn [load\_acquire](#method.load_acquire)<'a, A, G>( &self, path: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[AssetPath](struct.AssetPath.html "struct bevy::asset::AssetPath")<'a>>, guard: G, ) -> [Handle](../prelude/enum.Handle.html "enum bevy::prelude::Handle")<A>

where A: [Asset](../prelude/trait.Asset.html "trait bevy::prelude::Asset"), G: [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

👎Deprecated:

Use `asset_server.load_builder().with_guard(guard).load(path)` instead

Begins loading an [`Asset`](../prelude/trait.Asset.html "trait bevy::prelude::Asset") of type `A` stored at `path` while holding a guard item. The guard item is dropped when either the asset is loaded or loading has failed.

This function returns a “strong” [`Handle`](../prelude/enum.Handle.html "enum bevy::prelude::Handle"). When the [`Asset`](../prelude/trait.Asset.html "trait bevy::prelude::Asset") is loaded (and enters [`LoadState::Loaded`](enum.LoadState.html#variant.Loaded "variant bevy::asset::LoadState::Loaded")), it will be added to the associated [`Assets`](../prelude/struct.Assets.html "struct bevy::prelude::Assets") resource.

The guard item should notify the caller in its [`Drop`](https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html "trait core::ops::drop::Drop") implementation. See example `multi_asset_sync`. Synchronously this can be a [`Arc<AtomicU32>`](../platform/sync/struct.Arc.html "struct bevy::platform::sync::Arc") that decrements its counter, asynchronously this can be a `Barrier`. This function only guarantees the asset referenced by the [`Handle`](../prelude/enum.Handle.html "enum bevy::prelude::Handle") is loaded. If your asset is separated into multiple files, sub-assets referenced by the main asset might still be loading, depend on the implementation of the [`AssetLoader`](trait.AssetLoader.html "trait bevy::asset::AssetLoader").

Additionally, you can check the asset’s load state by reading [`AssetEvent`](../prelude/enum.AssetEvent.html "enum bevy::prelude::AssetEvent") events, calling [`AssetServer::load_state`](../prelude/struct.AssetServer.html#method.load_state "method bevy::prelude::AssetServer::load_state"), or checking the [`Assets`](../prelude/struct.Assets.html "struct bevy::prelude::Assets") storage to see if the [`Asset`](../prelude/trait.Asset.html "trait bevy::prelude::Asset") exists yet.

The asset load will fail and an error will be printed to the logs if the asset stored at `path` is not of type `A`.

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/server/mod.rs.html#431-435)

#### pub fn [load\_acquire\_override](#method.load_acquire_override)<'a, A, G>( &self, path: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[AssetPath](struct.AssetPath.html "struct bevy::asset::AssetPath")<'a>>, guard: G, ) -> [Handle](../prelude/enum.Handle.html "enum bevy::prelude::Handle")<A>

where A: [Asset](../prelude/trait.Asset.html "trait bevy::prelude::Asset"), G: [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

👎Deprecated:

Use `asset_server.load_builder().with_guard(guard).override_unapproved().load(path)` instead

Same as [`load`](../prelude/struct.AssetServer.html#method.load_acquire "method bevy::prelude::AssetServer::load_acquire"), but you can load assets from unapproved paths if [`AssetPlugin::unapproved_path_mode`](../prelude/struct.AssetPlugin.html#structfield.unapproved_path_mode "field bevy::prelude::AssetPlugin::unapproved_path_mode") is [`Deny`](enum.UnapprovedPathMode.html#variant.Deny "variant bevy::asset::UnapprovedPathMode::Deny").

See [`UnapprovedPathMode`](enum.UnapprovedPathMode.html "enum bevy::asset::UnapprovedPathMode") and [`AssetPath::is_unapproved`](struct.AssetPath.html#method.is_unapproved "method bevy::asset::AssetPath::is_unapproved")

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/server/mod.rs.html#449-453)

#### pub fn [load\_with\_settings](#method.load_with_settings)<'a, A, S>( &self, path: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[AssetPath](struct.AssetPath.html "struct bevy::asset::AssetPath")<'a>>, settings: impl [Fn](https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html "trait core::ops::function::Fn")([&mut S](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static, ) -> [Handle](../prelude/enum.Handle.html "enum bevy::prelude::Handle")<A>

where A: [Asset](../prelude/trait.Asset.html "trait bevy::prelude::Asset"), S: [Settings](meta/trait.Settings.html "trait bevy::asset::meta::Settings"),

👎Deprecated:

Use `asset_server.load_builder().with_settings(settings).load(path)` instead

Begins loading an [`Asset`](../prelude/trait.Asset.html "trait bevy::prelude::Asset") of type `A` stored at `path`. The given `settings` function will override the asset’s [`AssetLoader`](trait.AssetLoader.html "trait bevy::asset::AssetLoader") settings. The type `S` _must_ match the configured [`AssetLoader::Settings`](trait.AssetLoader.html#associatedtype.Settings "associated type bevy::asset::AssetLoader::Settings") or `settings` changes will be ignored and an error will be printed to the log.

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/server/mod.rs.html#467-471)

#### pub fn [load\_with\_settings\_override](#method.load_with_settings_override)<'a, A, S>( &self, path: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[AssetPath](struct.AssetPath.html "struct bevy::asset::AssetPath")<'a>>, settings: impl [Fn](https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html "trait core::ops::function::Fn")([&mut S](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static, ) -> [Handle](../prelude/enum.Handle.html "enum bevy::prelude::Handle")<A>

where A: [Asset](../prelude/trait.Asset.html "trait bevy::prelude::Asset"), S: [Settings](meta/trait.Settings.html "trait bevy::asset::meta::Settings"),

👎Deprecated:

Use `asset_server.load_builder().with_settings(settings).override_unapproved().load(path)` instead

Same as [`load`](../prelude/struct.AssetServer.html#method.load_with_settings "method bevy::prelude::AssetServer::load_with_settings"), but you can load assets from unapproved paths if [`AssetPlugin::unapproved_path_mode`](../prelude/struct.AssetPlugin.html#structfield.unapproved_path_mode "field bevy::prelude::AssetPlugin::unapproved_path_mode") is [`Deny`](enum.UnapprovedPathMode.html#variant.Deny "variant bevy::asset::UnapprovedPathMode::Deny").

See [`UnapprovedPathMode`](enum.UnapprovedPathMode.html "enum bevy::asset::UnapprovedPathMode") and [`AssetPath::is_unapproved`](struct.AssetPath.html#method.is_unapproved "method bevy::asset::AssetPath::is_unapproved")

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/server/mod.rs.html#491-496)

#### pub fn [load\_acquire\_with\_settings](#method.load_acquire_with_settings)<'a, A, S, G>( &self, path: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[AssetPath](struct.AssetPath.html "struct bevy::asset::AssetPath")<'a>>, settings: impl [Fn](https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html "trait core::ops::function::Fn")([&mut S](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static, guard: G, ) -> [Handle](../prelude/enum.Handle.html "enum bevy::prelude::Handle")<A>

where A: [Asset](../prelude/trait.Asset.html "trait bevy::prelude::Asset"), S: [Settings](meta/trait.Settings.html "trait bevy::asset::meta::Settings"), G: [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

👎Deprecated:

Use `asset_server.load_builder().with_guard(guard).with_settings(settings).load(path)` instead

Begins loading an [`Asset`](../prelude/trait.Asset.html "trait bevy::prelude::Asset") of type `A` stored at `path` while holding a guard item. The guard item is dropped when either the asset is loaded or loading has failed.

This function only guarantees the asset referenced by the [`Handle`](../prelude/enum.Handle.html "enum bevy::prelude::Handle") is loaded. If your asset is separated into multiple files, sub-assets referenced by the main asset might still be loading, depend on the implementation of the [`AssetLoader`](trait.AssetLoader.html "trait bevy::asset::AssetLoader").

The given `settings` function will override the asset’s [`AssetLoader`](trait.AssetLoader.html "trait bevy::asset::AssetLoader") settings. The type `S` _must_ match the configured [`AssetLoader::Settings`](trait.AssetLoader.html#associatedtype.Settings "associated type bevy::asset::AssetLoader::Settings") or `settings` changes will be ignored and an error will be printed to the log.

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/server/mod.rs.html#511-521)

#### pub fn [load\_acquire\_with\_settings\_override](#method.load_acquire_with_settings_override)<'a, A, S, G>( &self, path: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[AssetPath](struct.AssetPath.html "struct bevy::asset::AssetPath")<'a>>, settings: impl [Fn](https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html "trait core::ops::function::Fn")([&mut S](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static, guard: G, ) -> [Handle](../prelude/enum.Handle.html "enum bevy::prelude::Handle")<A>

where A: [Asset](../prelude/trait.Asset.html "trait bevy::prelude::Asset"), S: [Settings](meta/trait.Settings.html "trait bevy::asset::meta::Settings"), G: [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

👎Deprecated:

Use `asset_server.load_builder().with_guard(guard).with_settings(settings).override_unapproved().load(path)` instead

Same as [`load`](../prelude/struct.AssetServer.html#method.load_acquire_with_settings "method bevy::prelude::AssetServer::load_acquire_with_settings"), but you can load assets from unapproved paths if [`AssetPlugin::unapproved_path_mode`](../prelude/struct.AssetPlugin.html#structfield.unapproved_path_mode "field bevy::prelude::AssetPlugin::unapproved_path_mode") is [`Deny`](enum.UnapprovedPathMode.html#variant.Deny "variant bevy::asset::UnapprovedPathMode::Deny").

See [`UnapprovedPathMode`](enum.UnapprovedPathMode.html "enum bevy::asset::UnapprovedPathMode") and [`AssetPath::is_unapproved`](struct.AssetPath.html#method.is_unapproved "method bevy::asset::AssetPath::is_unapproved")

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/server/mod.rs.html#615-618)

#### pub async fn [load\_untyped\_async](#method.load_untyped_async)<'a>( &self, path: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[AssetPath](struct.AssetPath.html "struct bevy::asset::AssetPath")<'a>>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[UntypedHandle](../prelude/enum.UntypedHandle.html "enum bevy::prelude::UntypedHandle"), [AssetLoadError](enum.AssetLoadError.html "enum bevy::asset::AssetLoadError")\>

👎Deprecated:

Use `asset_server.load_builder().load_untyped_async(path)` instead

Asynchronously load an asset that you do not know the type of statically. If you _do_ know the type of the asset, you should use [`AssetServer::load`](../prelude/struct.AssetServer.html#method.load "method bevy::prelude::AssetServer::load"). If you don’t know the type of the asset, but you can’t use an async method, consider using [`AssetServer::load_untyped`](../prelude/struct.AssetServer.html#method.load_untyped "method bevy::prelude::AssetServer::load_untyped").

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/server/mod.rs.html#729)

#### pub fn [load\_untyped](#method.load_untyped)<'a>( &self, path: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[AssetPath](struct.AssetPath.html "struct bevy::asset::AssetPath")<'a>>, ) -> [Handle](../prelude/enum.Handle.html "enum bevy::prelude::Handle")<[LoadedUntypedAsset](struct.LoadedUntypedAsset.html "struct bevy::asset::LoadedUntypedAsset")\>

👎Deprecated:

Use `asset_server.load_builder().load_untyped(path)` instead

Load an asset without knowing its type. The method returns a handle to a [`LoadedUntypedAsset`](struct.LoadedUntypedAsset.html "struct bevy::asset::LoadedUntypedAsset").

Once the [`LoadedUntypedAsset`](struct.LoadedUntypedAsset.html "struct bevy::asset::LoadedUntypedAsset") is loaded, an untyped handle for the requested path can be retrieved from it.

```rust
use bevy_asset::{Assets, Handle, LoadedUntypedAsset};
use bevy_ecs::system::Res;
use bevy_ecs::resource::Resource;

#[derive(Resource)]
struct LoadingUntypedHandle(Handle<LoadedUntypedAsset>);

fn resolve_loaded_untyped_handle(loading_handle: Res<LoadingUntypedHandle>, loaded_untyped_assets: Res<Assets<LoadedUntypedAsset>>) {
    if let Some(loaded_untyped_asset) = loaded_untyped_assets.get(&loading_handle.0) {
        let handle = loaded_untyped_asset.handle.clone();
        // continue working with `handle` which points to the asset at the originally requested path
    }
}
```

This indirection enables a non blocking load of an untyped asset, since I/O is required to figure out the asset type before a handle can be created.

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/server/mod.rs.html#952)

#### pub fn [reload](#method.reload)<'a>(&self, path: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[AssetPath](struct.AssetPath.html "struct bevy::asset::AssetPath")<'a>>)

Kicks off a reload of the asset stored at the given path. This will only reload the asset if it currently loaded.

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/server/mod.rs.html#1008)

#### pub fn [add](#method.add)<A>(&self, asset: A) -> [Handle](../prelude/enum.Handle.html "enum bevy::prelude::Handle")<A>

where A: [Asset](../prelude/trait.Asset.html "trait bevy::prelude::Asset"),

Queues a new asset to be tracked by the [`AssetServer`](../prelude/struct.AssetServer.html "struct bevy::prelude::AssetServer") and returns a [`Handle`](../prelude/enum.Handle.html "enum bevy::prelude::Handle") to it. This can be used to track dependencies of assets created at runtime.

After the asset has been fully loaded by the [`AssetServer`](../prelude/struct.AssetServer.html "struct bevy::prelude::AssetServer"), it will show up in the relevant [`Assets`](../prelude/struct.Assets.html "struct bevy::prelude::Assets") storage.

##### [Examples found in repository](#scraped-examples-2)[?](../../scrape-examples-help.html)

examples/asset/asset\_decompression.rs ([line 131](../../src/asset_decompression/asset_decompression.rs.html#131))

```rust
115fn decompress<T: Component + From<Handle<A>>, A: Asset>(
116    mut commands: Commands,
117    asset_server: Res<AssetServer>,
118    mut compressed_assets: ResMut<Assets<GzAsset>>,
119    query: Query<(Entity, &Compressed<A>)>,
120) {
121    for (entity, Compressed { compressed, .. }) in query.iter() {
122        let Some(GzAsset { uncompressed }) = compressed_assets.remove(compressed) else {
123            continue;
124        };
125
126        let uncompressed = uncompressed.take::<A>().unwrap();
127
128        commands
129            .entity(entity)
130            .remove::<Compressed<A>>()
131            .insert(T::from(asset_server.add(uncompressed)));
132    }
133}
```

Hide additional examples

examples/3d/anisotropy.rs ([lines 118-122](../../src/anisotropy/anisotropy.rs.html#118-122))

```rust
100fn setup(mut commands: Commands, asset_server: Res<AssetServer>, app_status: Res<AppStatus>) {
101    commands.spawn((
102        Camera3d::default(),
103        Transform::from_translation(CAMERA_INITIAL_POSITION).looking_at(Vec3::ZERO, Vec3::Y),
104    ));
105
106    spawn_directional_light(&mut commands);
107
108    commands.spawn((
109        WorldAssetRoot(
110            asset_server.load("models/AnisotropyBarnLamp/AnisotropyBarnLamp.gltf#Scene0"),
111        ),
112        Transform::from_xyz(0.0, 0.07, -0.13),
113        Scene::BarnLamp,
114    ));
115
116    commands.spawn((
117        Mesh3d(
118            asset_server.add(
119                Mesh::from(Sphere::new(0.1))
120                    .with_generated_tangents()
121                    .unwrap(),
122            ),
123        ),
124        MeshMaterial3d(asset_server.add(StandardMaterial {
125            base_color: palettes::tailwind::GRAY_300.into(),
126            anisotropy_rotation: 0.5,
127            anisotropy_strength: 1.,
128            ..default()
129        })),
130        Scene::Sphere,
131        Visibility::Hidden,
132    ));
133
134    spawn_text(&mut commands, &app_status);
135}
```

examples/3d/contact\_shadows.rs ([line 232](../../src/contact_shadows/contact_shadows.rs.html#232))

```rust
109fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
110    commands.spawn((
111        Camera3d::default(),
112        Transform::from_xyz(-0.8, 0.6, -0.8).looking_at(Vec3::new(0.0, 0.35, 0.0), Vec3::Y),
113        ContactShadows::default(),
114        TemporalAntiAliasing::default(), // Contact shadows and AO benefit from TAA
115        // Everything past this point is extra to look pretty.
116        Bloom::default(),
117        Hdr,
118        Skybox {
119            brightness: 1000.0,
120            image: Some(asset_server.load("environment_maps/pisa_diffuse_rgb9e5_zstd.ktx2")),
121            ..default()
122        },
123        EnvironmentMapLight {
124            diffuse_map: asset_server.load("environment_maps/pisa_diffuse_rgb9e5_zstd.ktx2"),
125            specular_map: asset_server.load("environment_maps/pisa_specular_rgb9e5_zstd.ktx2"),
126            intensity: 1000.0,
127            ..default()
128        },
129        ScreenSpaceAmbientOcclusion::default(),
130        Msaa::Off,
131        Tonemapping::AcesFitted,
132        MotionBlur {
133            shutter_angle: 2.0, // This is really just for fun when spinning the model
134            ..default()
135        },
136    ));
137
138    let directional_light = commands
139        .spawn((
140            DirectionalLight {
141                shadow_maps_enabled: true,
142                contact_shadows_enabled: true,
143                ..default()
144            },
145            Visibility::Hidden,
146        ))
147        .id();
148
149    let point_light = commands
150        .spawn((
151            PointLight {
152                intensity: light_consts::lumens::VERY_LARGE_CINEMA_LIGHT * 0.4,
153                shadow_maps_enabled: true,
154                contact_shadows_enabled: true,
155                ..default()
156            },
157            Visibility::Visible,
158        ))
159        .id();
160
161    let spot_light = commands
162        .spawn((
163            SpotLight {
164                intensity: light_consts::lumens::VERY_LARGE_CINEMA_LIGHT * 0.4,
165                shadow_maps_enabled: true,
166                contact_shadows_enabled: true,
167                ..default()
168            },
169            Visibility::Hidden,
170        ))
171        .id();
172
173    commands
174        .spawn((
175            Transform::from_xyz(-0.8, 1.5, 1.2).looking_at(Vec3::ZERO, Vec3::Y),
176            Visibility::default(),
177            LightContainer,
178        ))
179        .add_child(directional_light)
180        .add_child(point_light)
181        .add_child(spot_light);
182
183    commands
184        .spawn((
185            WorldAssetRoot(asset_server.load(
186                GltfAssetLabel::Scene(0).from_asset("models/FlightHelmet/FlightHelmet.gltf"),
187            )),
188            Transform::from_rotation(Quat::from_rotation_y(std::f32::consts::PI)),
189        ))
190        .observe(
191            |event: On<Pointer<Drag>>,
192             mut query: Query<&mut Transform, With<WorldAssetRoot>>,
193             mut commands: Commands,
194             mut window: Query<Entity, With<PrimaryWindow>>| {
195                for mut transform in query.iter_mut() {
196                    transform.rotate_y(event.delta.x * 0.01);
197                }
198                commands
199                    .entity(window.single_mut().unwrap())
200                    .insert(CursorIcon::System(SystemCursorIcon::Grabbing));
201            },
202        )
203        .observe(
204            |_: On<Pointer<Over>>,
205             mut commands: Commands,
206             mut window: Query<Entity, With<PrimaryWindow>>| {
207                commands
208                    .entity(window.single_mut().unwrap())
209                    .insert(CursorIcon::System(SystemCursorIcon::Grab));
210            },
211        )
212        .observe(
213            |_: On<Pointer<Out>>,
214             mut commands: Commands,
215             mut window: Query<Entity, With<PrimaryWindow>>| {
216                commands
217                    .entity(window.single_mut().unwrap())
218                    .insert(CursorIcon::System(SystemCursorIcon::Default));
219            },
220        )
221        .observe(
222            |_: On<Pointer<DragEnd>>,
223             mut commands: Commands,
224             mut window: Query<Entity, With<PrimaryWindow>>| {
225                commands
226                    .entity(window.single_mut().unwrap())
227                    .insert(CursorIcon::System(SystemCursorIcon::Default));
228            },
229        );
230
231    commands.spawn((
232        Mesh3d(asset_server.add(Circle::default().mesh().into())),
233        MeshMaterial3d(asset_server.add(StandardMaterial {
234            base_color: Color::srgb(0.06, 0.06, 0.06),
235            ..default()
236        })),
237        Transform::from_rotation(Quat::from_axis_angle(Vec3::X, -std::f32::consts::FRAC_PI_2)),
238        GroundPlane,
239    ));
240
241    spawn_buttons(&mut commands);
242
243    commands.spawn((
244        Node {
245            position_type: PositionType::Absolute,
246            top: px(12.0),
247            left: px(0.0),
248            right: px(0.0),
249            justify_content: JustifyContent::Center,
250            ..default()
251        },
252        children![(
253            Text::new("Drag model to spin"),
254            TextFont {
255                font_size: FontSize::Px(18.0),
256                ..default()
257            },
258        )],
259    ));
260}
```

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/server/mod.rs.html#1054-1057)

#### pub fn [add\_async](#method.add_async)<A, E>( &self, future: impl [Future](../tasks/futures_lite/trait.Future.html "trait bevy::tasks::futures_lite::Future")<Output = [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<A, E>> + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + 'static, ) -> [Handle](../prelude/enum.Handle.html "enum bevy::prelude::Handle")<A>

where A: [Asset](../prelude/trait.Asset.html "trait bevy::prelude::Asset"), E: [Error](https://doc.rust-lang.org/nightly/core/error/trait.Error.html "trait core::error::Error") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

Queues a new asset to be tracked by the [`AssetServer`](../prelude/struct.AssetServer.html "struct bevy::prelude::AssetServer") and returns a [`Handle`](../prelude/enum.Handle.html "enum bevy::prelude::Handle") to it. This can be used to track dependencies of assets created at runtime.

After the asset has been fully loaded, it will show up in the relevant [`Assets`](../prelude/struct.Assets.html "struct bevy::prelude::Assets") storage.

##### [Examples found in repository](#scraped-examples-3)[?](../../scrape-examples-help.html)

examples/asset/generated\_assets.rs ([line 35](../../src/generated_assets/generated_assets.rs.html#35))

```rust
13fn setup(
14    mut commands: Commands,
15    asset_server: Res<AssetServer>,
16    mut materials: ResMut<Assets<StandardMaterial>>,
17    meshes: Res<Assets<Mesh>>,
18) {
19    commands.spawn((Camera3d::default(), Transform::from_xyz(0.0, 0.0, 5.0)));
20
21    commands.spawn((
22        DirectionalLight::default(),
23        Transform::default().looking_to(Dir3::new(Vec3::new(-1.0, -1.0, -1.0)).unwrap(), Dir3::Y),
24    ));
25
26    // The simplest way to generate an asset is to add it directly to the `Assets`.
27    let material_handle = materials.add(StandardMaterial::default());
28
29    commands.spawn((
30        Transform::from_xyz(-2.0, 0.0, 0.0),
31        MeshMaterial3d(material_handle.clone()),
32        // Alternatively, `add_async` creates a task that runs your async function. Once it
33        // completes, the asset is added to the `Assets`. This is "deferred" meaning that the asset
34        // may take a frame to be added after the task completes.
35        Mesh3d(asset_server.add_async(generate_mesh_async())),
36    ));
37
38    // The last way to generate assets is to reserve a handle, and then use `Assets::insert` to
39    // populate the asset later. In this example, the `generate_mesh_system` system runs to populate
40    // the mesh.
41    let mesh_handle = meshes.reserve_handle();
42    commands.insert_resource(HandleToGenerate(mesh_handle.clone()));
43    commands.spawn((
44        Transform::from_xyz(2.0, 0.0, 0.0)
45            .with_rotation(Quat::from_rotation_x(50.0f32.to_radians())),
46        Mesh3d(mesh_handle),
47        MeshMaterial3d(material_handle),
48    ));
49}
```

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/server/mod.rs.html#1115)

#### pub fn [load\_folder](#method.load_folder)<'a>( &self, path: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[AssetPath](struct.AssetPath.html "struct bevy::asset::AssetPath")<'a>>, ) -> [Handle](../prelude/enum.Handle.html "enum bevy::prelude::Handle")<[LoadedFolder](struct.LoadedFolder.html "struct bevy::asset::LoadedFolder")\>

Loads all assets from the specified folder recursively. The [`LoadedFolder`](struct.LoadedFolder.html "struct bevy::asset::LoadedFolder") asset (when it loads) will contain handles to all assets in the folder. You can wait for all assets to load by checking the [`LoadedFolder`](struct.LoadedFolder.html "struct bevy::asset::LoadedFolder")’s [`RecursiveDependencyLoadState`](enum.RecursiveDependencyLoadState.html "enum bevy::asset::RecursiveDependencyLoadState").

Loading the same folder multiple times will return the same handle. If the `file_watcher` feature is enabled, [`LoadedFolder`](struct.LoadedFolder.html "struct bevy::asset::LoadedFolder") handles will reload when a file in the folder is removed, added or moved. This includes files in subdirectories and moving, adding, or removing complete subdirectories.

##### [Examples found in repository](#scraped-examples-4)[?](../../scrape-examples-help.html)

examples/2d/texture\_atlas.rs ([line 34](../../src/texture_atlas/texture_atlas.rs.html#34))

```rust
32fn load_textures(mut commands: Commands, asset_server: Res<AssetServer>) {
33    // Load multiple, individual sprites from a folder
34    commands.insert_resource(RpgSpriteFolder(asset_server.load_folder("textures/rpg")));
35}
```

Hide additional examples

examples/asset/asset\_loading.rs ([line 55](../../src/asset_loading/asset_loading.rs.html#55))

```rust
12fn setup(
13    mut commands: Commands,
14    asset_server: Res<AssetServer>,
15    meshes: Res<Assets<Mesh>>,
16    mut materials: ResMut<Assets<StandardMaterial>>,
17) {
18    // By default AssetServer will load assets from inside the "assets" folder.
19    // For example, the next line will load GltfAssetLabel::Primitive{mesh:0,primitive:0}.from_asset("ROOT/assets/models/cube/cube.gltf"),
20    // where "ROOT" is the directory of the Application.
21    //
22    // This can be overridden by setting [`AssetPlugin.file_path`].
23    let cube_handle = asset_server.load(
24        GltfAssetLabel::Primitive {
25            mesh: 0,
26            primitive: 0,
27        }
28        .from_asset("models/cube/cube.gltf"),
29    );
30    let sphere_handle = asset_server.load(
31        GltfAssetLabel::Primitive {
32            mesh: 0,
33            primitive: 0,
34        }
35        .from_asset("models/sphere/sphere.gltf"),
36    );
37
38    // All assets end up in their Assets<T> collection once they are done loading:
39    if let Some(sphere) = meshes.get(&sphere_handle) {
40        // You might notice that this doesn't run! This is because assets load in parallel without
41        // blocking. When an asset has loaded, it will appear in relevant Assets<T>
42        // collection.
43        info!("{:?}", sphere.primitive_topology());
44    } else {
45        info!("sphere hasn't loaded yet");
46    }
47
48    // You can load all assets in a folder like this. They will be loaded in parallel without
49    // blocking. The LoadedFolder asset holds handles to each asset in the folder. These are all
50    // dependencies of the LoadedFolder asset, meaning you can wait for the LoadedFolder asset to
51    // fire AssetEvent::LoadedWithDependencies if you want to wait for all assets in the folder
52    // to load.
53    // If you want to keep the assets in the folder alive, make sure you store the returned handle
54    // somewhere.
55    let _loaded_folder: Handle<LoadedFolder> = asset_server.load_folder("models/torus");
56
57    // If you want a handle to a specific asset in a loaded folder, the easiest way to get one is to call load.
58    // It will _not_ be loaded a second time.
59    // The LoadedFolder asset will ultimately also hold handles to the assets, but waiting for it to load
60    // and finding the right handle is more work!
61    let torus_handle = asset_server.load(
62        GltfAssetLabel::Primitive {
63            mesh: 0,
64            primitive: 0,
65        }
66        .from_asset("models/torus/torus.gltf"),
67    );
68
69    // You can also add assets directly to their Assets<T> storage:
70    let material_handle = materials.add(StandardMaterial {
71        base_color: Color::srgb(0.8, 0.7, 0.6),
72        ..default()
73    });
74
75    // torus
76    commands.spawn((
77        Mesh3d(torus_handle),
78        MeshMaterial3d(material_handle.clone()),
79        Transform::from_xyz(-3.0, 0.0, 0.0),
80    ));
81    // cube
82    commands.spawn((
83        Mesh3d(cube_handle),
84        MeshMaterial3d(material_handle.clone()),
85        Transform::from_xyz(0.0, 0.0, 0.0),
86    ));
87    // sphere
88    commands.spawn((
89        Mesh3d(sphere_handle),
90        MeshMaterial3d(material_handle),
91        Transform::from_xyz(3.0, 0.0, 0.0),
92    ));
93    // light
94    commands.spawn((PointLight::default(), Transform::from_xyz(4.0, 5.0, 4.0)));
95    // camera
96    commands.spawn((
97        Camera3d::default(),
98        Transform::from_xyz(0.0, 3.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
99    ));
100}
```

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/server/mod.rs.html#1221-1224)

#### pub fn [get\_load\_states](#method.get_load_states)( &self, id: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[UntypedAssetId](enum.UntypedAssetId.html "enum bevy::asset::UntypedAssetId")\>, ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<([LoadState](enum.LoadState.html "enum bevy::asset::LoadState"), [DependencyLoadState](enum.DependencyLoadState.html "enum bevy::asset::DependencyLoadState"), [RecursiveDependencyLoadState](enum.RecursiveDependencyLoadState.html "enum bevy::asset::RecursiveDependencyLoadState"))>

Retrieves all loads states for the given asset id.

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/server/mod.rs.html#1243)

#### pub fn [get\_load\_state](#method.get_load_state)(&self, id: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[UntypedAssetId](enum.UntypedAssetId.html "enum bevy::asset::UntypedAssetId")\>) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[LoadState](enum.LoadState.html "enum bevy::asset::LoadState")\>

Retrieves the main [`LoadState`](enum.LoadState.html "enum bevy::asset::LoadState") of a given asset `id`.

Note that this is “just” the root asset load state. To get the load state of its dependencies or recursive dependencies, see [`AssetServer::get_dependency_load_state`](../prelude/struct.AssetServer.html#method.get_dependency_load_state "method bevy::prelude::AssetServer::get_dependency_load_state") and [`AssetServer::get_recursive_dependency_load_state`](../prelude/struct.AssetServer.html#method.get_recursive_dependency_load_state "method bevy::prelude::AssetServer::get_recursive_dependency_load_state") respectively.

##### [Examples found in repository](#scraped-examples-5)[?](../../scrape-examples-help.html)

examples/scene/world\_serialization.rs ([line 239](../../src/world_serialization/world_serialization.rs.html#239))

```rust
237fn panic_on_fail(world_roots: Query<&DynamicWorldRoot>, asset_server: Res<AssetServer>) {
238    for world_root in &world_roots {
239        if let Some(LoadState::Failed(err)) = asset_server.get_load_state(&world_root.0) {
240            panic!("Failed to load world. {err}");
241        }
242    }
243}
```

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/server/mod.rs.html#1256-1259)

#### pub fn [get\_dependency\_load\_state](#method.get_dependency_load_state)( &self, id: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[UntypedAssetId](enum.UntypedAssetId.html "enum bevy::asset::UntypedAssetId")\>, ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[DependencyLoadState](enum.DependencyLoadState.html "enum bevy::asset::DependencyLoadState")\>

Retrieves the [`DependencyLoadState`](enum.DependencyLoadState.html "enum bevy::asset::DependencyLoadState") of a given asset `id`’s dependencies.

Note that this is only the load state of direct dependencies of the root asset. To get the load state of the root asset itself or its recursive dependencies, see [`AssetServer::get_load_state`](../prelude/struct.AssetServer.html#method.get_load_state "method bevy::prelude::AssetServer::get_load_state") and [`AssetServer::get_recursive_dependency_load_state`](../prelude/struct.AssetServer.html#method.get_recursive_dependency_load_state "method bevy::prelude::AssetServer::get_recursive_dependency_load_state") respectively.

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/server/mod.rs.html#1274-1277)

#### pub fn [get\_recursive\_dependency\_load\_state](#method.get_recursive_dependency_load_state)( &self, id: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[UntypedAssetId](enum.UntypedAssetId.html "enum bevy::asset::UntypedAssetId")\>, ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[RecursiveDependencyLoadState](enum.RecursiveDependencyLoadState.html "enum bevy::asset::RecursiveDependencyLoadState")\>

Retrieves the main [`RecursiveDependencyLoadState`](enum.RecursiveDependencyLoadState.html "enum bevy::asset::RecursiveDependencyLoadState") of a given asset `id`’s recursive dependencies.

Note that this is only the load state of recursive dependencies of the root asset. To get the load state of the root asset itself or its direct dependencies only, see [`AssetServer::get_load_state`](../prelude/struct.AssetServer.html#method.get_load_state "method bevy::prelude::AssetServer::get_load_state") and [`AssetServer::get_dependency_load_state`](../prelude/struct.AssetServer.html#method.get_dependency_load_state "method bevy::prelude::AssetServer::get_dependency_load_state") respectively.

##### [Examples found in repository](#scraped-examples-6)[?](../../scrape-examples-help.html)

examples/showcase/loading\_screen.rs ([line 210](../../src/loading_screen/loading_screen.rs.html#210))

```rust
197fn update_loading_data(
198    mut loading_data: ResMut<LoadingData>,
199    mut loading_state: ResMut<LoadingState>,
200    asset_server: Res<AssetServer>,
201    pipelines_ready: Res<PipelinesReady>,
202) {
203    if !loading_data.loading_assets.is_empty() || !pipelines_ready.0 {
204        // If we are still loading assets / pipelines are not fully compiled,
205        // we reset the confirmation frame count.
206        loading_data.confirmation_frames_count = 0;
207
208        loading_data.loading_assets.retain(|asset| {
209            asset_server
210                .get_recursive_dependency_load_state(asset)
211                .is_none_or(|state| !state.is_loaded())
212        });
213
214        // If there are no more assets being monitored, and pipelines
215        // are compiled, then start counting confirmation frames.
216        // Once enough confirmations have passed, everything will be
217        // considered to be fully loaded.
218    } else {
219        loading_data.confirmation_frames_count += 1;
220        if loading_data.confirmation_frames_count == loading_data.confirmation_frames_target {
221            *loading_state = LoadingState::LevelReady;
222        }
223    }
224}
```

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/server/mod.rs.html#1291)

#### pub fn [load\_state](#method.load_state)(&self, id: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[UntypedAssetId](enum.UntypedAssetId.html "enum bevy::asset::UntypedAssetId")\>) -> [LoadState](enum.LoadState.html "enum bevy::asset::LoadState")

Retrieves the main [`LoadState`](enum.LoadState.html "enum bevy::asset::LoadState") of a given asset `id`.

This is the same as [`AssetServer::get_load_state`](../prelude/struct.AssetServer.html#method.get_load_state "method bevy::prelude::AssetServer::get_load_state") except the result is unwrapped. If the result is None, [`LoadState::NotLoaded`](enum.LoadState.html#variant.NotLoaded "variant bevy::asset::LoadState::NotLoaded") is returned.

##### [Examples found in repository](#scraped-examples-7)[?](../../scrape-examples-help.html)

examples/3d/pbr.rs ([line 134](../../src/pbr/pbr.rs.html#134))

```rust
127fn environment_map_load_finish(
128    mut commands: Commands,
129    asset_server: Res<AssetServer>,
130    environment_map: Single<&EnvironmentMapLight>,
131    label_entity: Option<Single<Entity, With<EnvironmentMapLabel>>>,
132) {
133    if asset_server
134        .load_state(&environment_map.diffuse_map)
135        .is_loaded()
136        && asset_server
137            .load_state(&environment_map.specular_map)
138            .is_loaded()
139    {
140        // Do not attempt to remove `label_entity` if it has already been removed.
141        if let Some(label_entity) = label_entity {
142            commands.entity(*label_entity).despawn();
143        }
144    }
145}
```

Hide additional examples

examples/3d/skybox.rs ([line 154](../../src/skybox/skybox.rs.html#154))

```rust
148fn asset_loaded(
149    asset_server: Res<AssetServer>,
150    mut images: ResMut<Assets<Image>>,
151    mut cubemap: ResMut<Cubemap>,
152    mut skyboxes: Query<&mut Skybox>,
153) {
154    if !cubemap.is_loaded && asset_server.load_state(&cubemap.image_handle).is_loaded() {
155        info!("Swapping to {}...", CUBEMAPS[cubemap.index].0);
156        let mut image = images.get_mut(&cubemap.image_handle).unwrap();
157        // NOTE: PNGs do not have any metadata that could indicate they contain a cubemap texture,
158        // so they appear as one texture. The following code reconfigures the texture as necessary.
159        if image.texture_descriptor.array_layer_count() == 1 {
160            let layers = image.height() / image.width();
161            image
162                .reinterpret_stacked_2d_as_array(layers)
163                .expect("asset should be 2d texture and height will always be evenly divisible with the given layers");
164            image.texture_view_descriptor = Some(TextureViewDescriptor {
165                dimension: Some(TextureViewDimension::Cube),
166                ..default()
167            });
168        }
169
170        for mut skybox in &mut skyboxes {
171            skybox.image = Some(cubemap.image_handle.clone());
172        }
173
174        cubemap.is_loaded = true;
175    }
176}
```

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/server/mod.rs.html#1299)

#### pub fn [dependency\_load\_state](#method.dependency_load_state)( &self, id: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[UntypedAssetId](enum.UntypedAssetId.html "enum bevy::asset::UntypedAssetId")\>, ) -> [DependencyLoadState](enum.DependencyLoadState.html "enum bevy::asset::DependencyLoadState")

Retrieves the [`DependencyLoadState`](enum.DependencyLoadState.html "enum bevy::asset::DependencyLoadState") of a given asset `id`.

This is the same as [`AssetServer::get_dependency_load_state`](../prelude/struct.AssetServer.html#method.get_dependency_load_state "method bevy::prelude::AssetServer::get_dependency_load_state") except the result is unwrapped. If the result is None, [`DependencyLoadState::NotLoaded`](enum.DependencyLoadState.html#variant.NotLoaded "variant bevy::asset::DependencyLoadState::NotLoaded") is returned.

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/server/mod.rs.html#1308-1311)

#### pub fn [recursive\_dependency\_load\_state](#method.recursive_dependency_load_state)( &self, id: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[UntypedAssetId](enum.UntypedAssetId.html "enum bevy::asset::UntypedAssetId")\>, ) -> [RecursiveDependencyLoadState](enum.RecursiveDependencyLoadState.html "enum bevy::asset::RecursiveDependencyLoadState")

Retrieves the [`RecursiveDependencyLoadState`](enum.RecursiveDependencyLoadState.html "enum bevy::asset::RecursiveDependencyLoadState") of a given asset `id`.

This is the same as [`AssetServer::get_recursive_dependency_load_state`](../prelude/struct.AssetServer.html#method.get_recursive_dependency_load_state "method bevy::prelude::AssetServer::get_recursive_dependency_load_state") except the result is unwrapped. If the result is None, [`RecursiveDependencyLoadState::NotLoaded`](enum.RecursiveDependencyLoadState.html#variant.NotLoaded "variant bevy::asset::RecursiveDependencyLoadState::NotLoaded") is returned.

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/server/mod.rs.html#1317)

#### pub fn [is\_loaded](#method.is_loaded)(&self, id: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[UntypedAssetId](enum.UntypedAssetId.html "enum bevy::asset::UntypedAssetId")\>) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Convenience method that returns true if the asset has been loaded.

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/server/mod.rs.html#1322)

#### pub fn [is\_loaded\_with\_direct\_dependencies](#method.is_loaded_with_direct_dependencies)( &self, id: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[UntypedAssetId](enum.UntypedAssetId.html "enum bevy::asset::UntypedAssetId")\>, ) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Convenience method that returns true if the asset and all of its direct dependencies have been loaded.

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/server/mod.rs.html#1331)

#### pub fn [is\_loaded\_with\_dependencies](#method.is_loaded_with_dependencies)(&self, id: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[UntypedAssetId](enum.UntypedAssetId.html "enum bevy::asset::UntypedAssetId")\>) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Convenience method that returns true if the asset, all of its dependencies, and all of its recursive dependencies have been loaded.

##### [Examples found in repository](#scraped-examples-8)[?](../../scrape-examples-help.html)

examples/animation/animated\_mesh\_control.rs ([line 104](../../src/animated_mesh_control/animated_mesh_control.rs.html#104))

```rust
97fn spawn_fox_asset_when_ready(
98    mut commands: Commands,
99    fox_handle: Res<Fox>,
100    asset_server: Res<AssetServer>,
101    gltfs: Res<Assets<Gltf>>,
102    mut graphs: ResMut<Assets<AnimationGraph>>,
103) {
104    if !asset_server.is_loaded_with_dependencies(&fox_handle.0) {
105        // fox is not loaded yet
106        return;
107    }
108
109    let fox = gltfs
110        .get(&fox_handle.0)
111        .expect("a loaded asset should exist in the glTF assets collection");
112
113    // Build the animation graph
114    let (graph, node_indices) = AnimationGraph::from_clips([
115        fox.named_animations["Run"].clone(),
116        fox.named_animations["Walk"].clone(),
117        fox.named_animations["Survey"].clone(),
118    ]);
119
120    // Keep our animation graph in a Resource so that it can be inserted onto
121    // the correct entity once the scene actually loads.
122    let graph_handle = graphs.add(graph);
123    commands.insert_resource(Animations {
124        animations: node_indices,
125        graph_handle,
126    });
127
128    // Fox
129    commands
130        .spawn(WorldAssetRoot(
131            fox.default_scene
132                .clone()
133                .expect("a default scene exists in this file"),
134        ))
135        .observe(setup_scene);
136}
```

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/server/mod.rs.html#1345)

#### pub fn [are\_dependencies\_loaded](#method.are_dependencies_loaded)( &self, value: &impl [VisitAssetDependencies](trait.VisitAssetDependencies.html "trait bevy::asset::VisitAssetDependencies"), ) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Returns true if all of `value`s dependencies (included recursive dependencies) are loaded.

This allows querying for whether all the handles in a resource or component are loaded.

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/server/mod.rs.html#1370)

#### pub fn [are\_direct\_dependencies\_loaded](#method.are_direct_dependencies_loaded)( &self, value: &impl [VisitAssetDependencies](trait.VisitAssetDependencies.html "trait bevy::asset::VisitAssetDependencies"), ) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Returns true if all of `value`s dependencies (excluding recursive dependencies) are loaded.

This allows querying for whether all the handles in a resource or component are loaded.

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/server/mod.rs.html#1394)

#### pub fn [get\_handle](#method.get_handle)<'a, A>( &self, path: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[AssetPath](struct.AssetPath.html "struct bevy::asset::AssetPath")<'a>>, ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Handle](../prelude/enum.Handle.html "enum bevy::prelude::Handle")<A>>

where A: [Asset](../prelude/trait.Asset.html "trait bevy::prelude::Asset"),

Returns an active handle for the given path, if the asset at the given path has already started loading, or is still “alive”.

##### [Examples found in repository](#scraped-examples-9)[?](../../scrape-examples-help.html)

examples/2d/texture\_atlas.rs ([line 144](../../src/texture_atlas/texture_atlas.rs.html#144))

```rust
50fn setup(
51    mut commands: Commands,
52    rpg_sprite_handles: Res<RpgSpriteFolder>,
53    asset_server: Res<AssetServer>,
54    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
55    loaded_folders: Res<Assets<LoadedFolder>>,
56    mut textures: ResMut<Assets<Image>>,
57) {
58    let loaded_folder = loaded_folders.get(&rpg_sprite_handles.0).unwrap();
59
60    // Create texture atlases with different padding and sampling
61
62    let (texture_atlas_linear, linear_sources, linear_texture) = create_texture_atlas(
63        loaded_folder,
64        None,
65        Some(ImageSampler::linear()),
66        &mut textures,
67    );
68    let atlas_linear_handle = texture_atlases.add(texture_atlas_linear);
69
70    let (texture_atlas_nearest, nearest_sources, nearest_texture) = create_texture_atlas(
71        loaded_folder,
72        None,
73        Some(ImageSampler::nearest()),
74        &mut textures,
75    );
76    let atlas_nearest_handle = texture_atlases.add(texture_atlas_nearest);
77
78    let (texture_atlas_linear_padded, linear_padded_sources, linear_padded_texture) =
79        create_texture_atlas(
80            loaded_folder,
81            Some(UVec2::new(6, 6)),
82            Some(ImageSampler::linear()),
83            &mut textures,
84        );
85    let atlas_linear_padded_handle = texture_atlases.add(texture_atlas_linear_padded.clone());
86
87    let (texture_atlas_nearest_padded, nearest_padded_sources, nearest_padded_texture) =
88        create_texture_atlas(
89            loaded_folder,
90            Some(UVec2::new(6, 6)),
91            Some(ImageSampler::nearest()),
92            &mut textures,
93        );
94    let atlas_nearest_padded_handle = texture_atlases.add(texture_atlas_nearest_padded);
95
96    commands.spawn(Camera2d);
97
98    // Padded textures are to the right, unpadded to the left
99
100    // Draw unpadded texture atlas
101    commands.spawn((
102        Sprite::from_image(linear_texture.clone()),
103        Transform {
104            translation: Vec3::new(-250.0, -160.0, 0.0),
105            scale: Vec3::splat(0.5),
106            ..default()
107        },
108    ));
109
110    // Draw padded texture atlas
111    commands.spawn((
112        Sprite::from_image(linear_padded_texture.clone()),
113        Transform {
114            translation: Vec3::new(250.0, -160.0, 0.0),
115            scale: Vec3::splat(0.5),
116            ..default()
117        },
118    ));
119
120    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
121
122    // Padding label text style
123    let text_style: TextFont = TextFont {
124        font: font.clone().into(),
125        font_size: FontSize::Px(42.0),
126        ..default()
127    };
128
129    // Labels to indicate padding
130
131    // No padding
132    create_label(
133        &mut commands,
134        (-250.0, 250.0, 0.0),
135        "No padding",
136        text_style.clone(),
137    );
138
139    // Padding
140    create_label(&mut commands, (250.0, 250.0, 0.0), "Padding", text_style);
141
142    // Get handle to a sprite to render
143    let vendor_handle: Handle<Image> = asset_server
144        .get_handle("textures/rpg/chars/vendor/generic-rpg-vendor.png")
145        .unwrap();
146
147    // Configuration array to render sprites through iteration
148    let configurations: [(
149        &str,
150        Handle<TextureAtlasLayout>,
151        TextureAtlasSources,
152        Handle<Image>,
153        f32,
154    ); 4] = [
155        (
156            "Linear",
157            atlas_linear_handle,
158            linear_sources,
159            linear_texture,
160            -350.0,
161        ),
162        (
163            "Nearest",
164            atlas_nearest_handle,
165            nearest_sources,
166            nearest_texture,
167            -150.0,
168        ),
169        (
170            "Linear",
171            atlas_linear_padded_handle,
172            linear_padded_sources,
173            linear_padded_texture,
174            150.0,
175        ),
176        (
177            "Nearest",
178            atlas_nearest_padded_handle,
179            nearest_padded_sources,
180            nearest_padded_texture,
181            350.0,
182        ),
183    ];
184
185    // Label text style
186    let sampling_label_style = TextFont {
187        font: font.into(),
188        font_size: FontSize::Px(25.0),
189        ..default()
190    };
191
192    let base_y = 80.0; // y position of the sprites
193
194    for (sampling, atlas_handle, atlas_sources, atlas_texture, x) in configurations {
195        // Render a sprite from the texture_atlas
196        create_sprite_from_atlas(
197            &mut commands,
198            (x, base_y, 0.0),
199            atlas_texture,
200            atlas_sources,
201            atlas_handle,
202            &vendor_handle,
203        );
204
205        // Render a label to indicate the sampling setting
206        create_label(
207            &mut commands,
208            (x, base_y + 110.0, 0.0), // Offset to y position of the sprite
209            sampling,
210            sampling_label_style.clone(),
211        );
212    }
213}
```

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/server/mod.rs.html#1406)

#### pub fn [get\_id\_handle](#method.get_id_handle)<A>(&self, id: [AssetId](../prelude/enum.AssetId.html "enum bevy::prelude::AssetId")<A>) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Handle](../prelude/enum.Handle.html "enum bevy::prelude::Handle")<A>>

where A: [Asset](../prelude/trait.Asset.html "trait bevy::prelude::Asset"),

Get a `Handle` from an `AssetId`.

This only returns `Some` if `id` is derived from a `Handle` that was loaded through an `AssetServer`, otherwise it returns `None`.

Consider using [`Assets::get_strong_handle`](../prelude/struct.Assets.html#method.get_strong_handle "method bevy::prelude::Assets::get_strong_handle") in the case the `Handle` comes from [`Assets::add`](../prelude/struct.Assets.html#method.add "method bevy::prelude::Assets::add").

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/server/mod.rs.html#1413)

#### pub fn [get\_id\_handle\_untyped](#method.get_id_handle_untyped)(&self, id: [UntypedAssetId](enum.UntypedAssetId.html "enum bevy::asset::UntypedAssetId")) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[UntypedHandle](../prelude/enum.UntypedHandle.html "enum bevy::prelude::UntypedHandle")\>

Get an `UntypedHandle` from an `UntypedAssetId`. See [`AssetServer::get_id_handle`](../prelude/struct.AssetServer.html#method.get_id_handle "method bevy::prelude::AssetServer::get_id_handle") for details.

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/server/mod.rs.html#1423)

#### pub fn [is\_managed](#method.is_managed)(&self, id: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[UntypedAssetId](enum.UntypedAssetId.html "enum bevy::asset::UntypedAssetId")\>) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Returns `true` if the given `id` corresponds to an asset that is managed by this [`AssetServer`](../prelude/struct.AssetServer.html "struct bevy::prelude::AssetServer"). Otherwise, returns `false`.

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/server/mod.rs.html#1437)

#### pub fn [get\_path\_id](#method.get_path_id)<'a>( &self, path: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[AssetPath](struct.AssetPath.html "struct bevy::asset::AssetPath")<'a>>, ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[UntypedAssetId](enum.UntypedAssetId.html "enum bevy::asset::UntypedAssetId")\>

Returns an active untyped asset id for the given path, if the asset at the given path has already started loading, or is still “alive”. Returns the first ID in the event of multiple assets being registered against a single path.

##### See also

[`get_path_ids`](../prelude/struct.AssetServer.html#method.get_path_ids "method bevy::prelude::AssetServer::get_path_ids") for all handles.

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/server/mod.rs.html#1447)

#### pub fn [get\_path\_ids](#method.get_path_ids)<'a>( &self, path: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[AssetPath](struct.AssetPath.html "struct bevy::asset::AssetPath")<'a>>, ) -> [Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[UntypedAssetId](enum.UntypedAssetId.html "enum bevy::asset::UntypedAssetId")\>

Returns all active untyped asset IDs for the given path, if the assets at the given path have already started loading, or are still “alive”. Multiple IDs will be returned in the event that a single path is used by multiple [`AssetLoader`](trait.AssetLoader.html "trait bevy::asset::AssetLoader")’s.

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/server/mod.rs.html#1461)

#### pub fn [get\_handle\_untyped](#method.get_handle_untyped)<'a>( &self, path: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[AssetPath](struct.AssetPath.html "struct bevy::asset::AssetPath")<'a>>, ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[UntypedHandle](../prelude/enum.UntypedHandle.html "enum bevy::prelude::UntypedHandle")\>

Returns an active untyped handle for the given path, if the asset at the given path has already started loading, or is still “alive”. Returns the first handle in the event of multiple assets being registered against a single path.

##### See also

[`get_handles_untyped`](../prelude/struct.AssetServer.html#method.get_handles_untyped "method bevy::prelude::AssetServer::get_handles_untyped") for all handles.

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/server/mod.rs.html#1469)

#### pub fn [get\_handles\_untyped](#method.get_handles_untyped)<'a>( &self, path: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[AssetPath](struct.AssetPath.html "struct bevy::asset::AssetPath")<'a>>, ) -> [Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[UntypedHandle](../prelude/enum.UntypedHandle.html "enum bevy::prelude::UntypedHandle")\>

Returns all active untyped handles for the given path, if the assets at the given path have already started loading, or are still “alive”. Multiple handles will be returned in the event that a single path is used by multiple [`AssetLoader`](trait.AssetLoader.html "trait bevy::asset::AssetLoader")’s.

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/server/mod.rs.html#1476-1480)

#### pub fn [get\_path\_and\_type\_id\_handle](#method.get_path_and_type_id_handle)( &self, path: &[AssetPath](struct.AssetPath.html "struct bevy::asset::AssetPath")<'\_>, type\_id: [TypeId](https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html "struct core::any::TypeId"), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[UntypedHandle](../prelude/enum.UntypedHandle.html "enum bevy::prelude::UntypedHandle")\>

Returns an active untyped handle for the given path and [`TypeId`](https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html "struct core::any::TypeId"), if the asset at the given path has already started loading, or is still “alive”.

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/server/mod.rs.html#1487)

#### pub fn [get\_path](#method.get_path)(&self, id: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[UntypedAssetId](enum.UntypedAssetId.html "enum bevy::asset::UntypedAssetId")\>) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[AssetPath](struct.AssetPath.html "struct bevy::asset::AssetPath")<'\_>>

Returns the path for the given `id`, if it has one.

##### [Examples found in repository](#scraped-examples-10)[?](../../scrape-examples-help.html)

examples/animation/morph\_targets.rs ([line 87](../../src/morph_targets/morph_targets.rs.html#87))

```rust
80fn name_morphs(
81    asset_server: Res<AssetServer>,
82    mut events: MessageReader<AssetEvent<Mesh>>,
83    meshes: Res<Assets<Mesh>>,
84) {
85    for event in events.read() {
86        if let AssetEvent::<Mesh>::Added { id } = event
87            && let Some(path) = asset_server.get_path(*id)
88            && let Some(mesh) = meshes.get(*id)
89            && let Some(names) = mesh.morph_target_names()
90        {
91            info!("Morph target names for {path:?}:");
92
93            for name in names {
94                info!("  {name}");
95            }
96        }
97    }
98}
```

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/server/mod.rs.html#1498)

#### pub fn [mode](#method.mode)(&self) -> [AssetServerMode](enum.AssetServerMode.html "enum bevy::asset::AssetServerMode")

Returns the [`AssetServerMode`](enum.AssetServerMode.html "enum bevy::asset::AssetServerMode") this server is currently in.

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/server/mod.rs.html#1506)

#### pub fn [preregister\_loader](#method.preregister_loader)<L>(&self, extensions: &\[&[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\])

where L: [AssetLoader](trait.AssetLoader.html "trait bevy::asset::AssetLoader"),

Pre-register a loader that will later be added.

Assets loaded with matching extensions will be blocked until the real loader is added.

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/server/mod.rs.html#1695-1700)

#### pub async fn [wait\_for\_asset](#method.wait_for_asset)<A>( &self, handle: &[Handle](../prelude/enum.Handle.html "enum bevy::prelude::Handle")<A>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [WaitForAssetError](enum.WaitForAssetError.html "enum bevy::asset::WaitForAssetError")\>

where A: [Asset](../prelude/trait.Asset.html "trait bevy::prelude::Asset"),

Returns a future that will suspend until the specified asset and its dependencies finish loading.

##### Errors

This will return an error if the asset or any of its dependencies fail to load, or if the asset has not been queued up to be loaded.

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/server/mod.rs.html#1711-1716)

#### pub async fn [wait\_for\_asset\_untyped](#method.wait_for_asset_untyped)( &self, handle: &[UntypedHandle](../prelude/enum.UntypedHandle.html "enum bevy::prelude::UntypedHandle"), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [WaitForAssetError](enum.WaitForAssetError.html "enum bevy::asset::WaitForAssetError")\>

Returns a future that will suspend until the specified asset and its dependencies finish loading.

##### Errors

This will return an error if the asset or any of its dependencies fail to load, or if the asset has not been queued up to be loaded.

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/server/mod.rs.html#1739-1742)

#### pub async fn [wait\_for\_asset\_id](#method.wait_for_asset_id)( &self, id: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[UntypedAssetId](enum.UntypedAssetId.html "enum bevy::asset::UntypedAssetId")\>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [WaitForAssetError](enum.WaitForAssetError.html "enum bevy::asset::WaitForAssetError")\>

Returns a future that will suspend until the specified asset and its dependencies finish loading.

Note that since an asset ID does not count as a reference to the asset, the future returned from this method will _not_ keep the asset alive. This may lead to the asset unexpectedly being dropped while you are waiting for it to finish loading.

When calling this method, make sure a strong handle is stored elsewhere to prevent the asset from being dropped. If you have access to an asset’s strong [`Handle`](../prelude/enum.Handle.html "enum bevy::prelude::Handle"), you should prefer to call [`AssetServer::wait_for_asset`](../prelude/struct.AssetServer.html#method.wait_for_asset "method bevy::prelude::AssetServer::wait_for_asset") or [`wait_for_asset_untyped`](../prelude/struct.AssetServer.html#method.wait_for_asset_untyped "method bevy::prelude::AssetServer::wait_for_asset_untyped") to ensure the asset finishes loading.

##### Errors

This will return an error if the asset or any of its dependencies fail to load, or if the asset has not been queued up to be loaded.

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/server/mod.rs.html#1827-1830)

#### pub async fn [write\_default\_loader\_meta\_file\_for\_path](#method.write_default_loader_meta_file_for_path)( &self, path: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[AssetPath](struct.AssetPath.html "struct bevy::asset::AssetPath")<'\_>>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [WriteDefaultMetaError](enum.WriteDefaultMetaError.html "enum bevy::asset::WriteDefaultMetaError")\>

Writes the default loader meta file for the provided `path`.

This function only generates meta files that simply load the path directly. To generate a meta file that will use the default asset processor for the path, see [`AssetProcessor::write_default_meta_file_for_path`](processor/struct.AssetProcessor.html#method.write_default_meta_file_for_path "method bevy::asset::processor::AssetProcessor::write_default_meta_file_for_path").

Note if there is already a meta file for `path`, this function returns `Err(WriteDefaultMetaError::MetaAlreadyExists)`.

## Trait Implementations

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/server/mod.rs.html#65)

### impl [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") for [AssetServer](../prelude/struct.AssetServer.html "struct bevy::prelude::AssetServer")

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/server/mod.rs.html#65)

#### fn [clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)(&self) -> [AssetServer](../prelude/struct.AssetServer.html "struct bevy::prelude::AssetServer")

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/142757 "Tracking issue for const_clone")) · [Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247)

#### fn [clone\_from](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/server/mod.rs.html#65)

### impl [Component](../prelude/trait.Component.html "trait bevy::prelude::Component") for [AssetServer](../prelude/struct.AssetServer.html "struct bevy::prelude::AssetServer")

where [AssetServer](../prelude/struct.AssetServer.html "struct bevy::prelude::AssetServer"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/server/mod.rs.html#65)

#### const [STORAGE\_TYPE](../prelude/trait.Component.html#associatedconstant.STORAGE_TYPE): [StorageType](../ecs/component/enum.StorageType.html "enum bevy::ecs::component::StorageType") = bevy\_ecs::component::StorageType::SparseSet

A constant indicating the storage type used for this component.

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/server/mod.rs.html#65)

#### type [Mutability](../prelude/trait.Component.html#associatedtype.Mutability) = [Mutable](../ecs/component/struct.Mutable.html "struct bevy::ecs::component::Mutable")

A marker type to assist Bevy with determining if this component is mutable, or immutable. Mutable components will have [`Component<Mutability = Mutable>`](../prelude/trait.Component.html "trait bevy::prelude::Component"), while immutable components will instead have [`Component<Mutability = Immutable>`](../prelude/trait.Component.html "trait bevy::prelude::Component"). [Read more](../prelude/trait.Component.html#associatedtype.Mutability)

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/server/mod.rs.html#65)

#### fn [register\_required\_components](../prelude/trait.Component.html#method.register_required_components)( \_requiree: [ComponentId](../ecs/component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId"), required\_components: &mut [RequiredComponentsRegistrator](../ecs/component/struct.RequiredComponentsRegistrator.html "struct bevy::ecs::component::RequiredComponentsRegistrator")<'\_, '\_>, )

Registers required components. [Read more](../prelude/trait.Component.html#method.register_required_components)

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/server/mod.rs.html#65)

#### fn [clone\_behavior](../prelude/trait.Component.html#method.clone_behavior)() -> [ComponentCloneBehavior](../ecs/component/enum.ComponentCloneBehavior.html "enum bevy::ecs::component::ComponentCloneBehavior")

Called when registering this component, allowing to override clone function (or disable cloning altogether) for this component. [Read more](../prelude/trait.Component.html#method.clone_behavior)

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/server/mod.rs.html#65)

#### fn [relationship\_accessor](../prelude/trait.Component.html#method.relationship_accessor)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[ComponentRelationshipAccessor](../ecs/relationship/struct.ComponentRelationshipAccessor.html "struct bevy::ecs::relationship::ComponentRelationshipAccessor")<[AssetServer](../prelude/struct.AssetServer.html "struct bevy::prelude::AssetServer")\>>

Returns [`ComponentRelationshipAccessor`](../ecs/relationship/struct.ComponentRelationshipAccessor.html "struct bevy::ecs::relationship::ComponentRelationshipAccessor") required for working with relationships in dynamic contexts. [Read more](../prelude/trait.Component.html#method.relationship_accessor)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/component/mod.rs.html#524)

#### fn [on\_add](../prelude/trait.Component.html#method.on_add)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<for<'w> [fn](https://doc.rust-lang.org/nightly/std/primitive.fn.html)([DeferredWorld](../ecs/world/struct.DeferredWorld.html "struct bevy::ecs::world::DeferredWorld")<'w>, [HookContext](../ecs/lifecycle/struct.HookContext.html "struct bevy::ecs::lifecycle::HookContext"))>

Gets the `on_add` [`ComponentHook`](../ecs/lifecycle/type.ComponentHook.html "type bevy::ecs::lifecycle::ComponentHook") for this [`Component`](../prelude/trait.Component.html "trait bevy::prelude::Component") if one is defined.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/component/mod.rs.html#529)

#### fn [on\_insert](../prelude/trait.Component.html#method.on_insert)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<for<'w> [fn](https://doc.rust-lang.org/nightly/std/primitive.fn.html)([DeferredWorld](../ecs/world/struct.DeferredWorld.html "struct bevy::ecs::world::DeferredWorld")<'w>, [HookContext](../ecs/lifecycle/struct.HookContext.html "struct bevy::ecs::lifecycle::HookContext"))>

Gets the `on_insert` [`ComponentHook`](../ecs/lifecycle/type.ComponentHook.html "type bevy::ecs::lifecycle::ComponentHook") for this [`Component`](../prelude/trait.Component.html "trait bevy::prelude::Component") if one is defined.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/component/mod.rs.html#534)

#### fn [on\_discard](../prelude/trait.Component.html#method.on_discard)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<for<'w> [fn](https://doc.rust-lang.org/nightly/std/primitive.fn.html)([DeferredWorld](../ecs/world/struct.DeferredWorld.html "struct bevy::ecs::world::DeferredWorld")<'w>, [HookContext](../ecs/lifecycle/struct.HookContext.html "struct bevy::ecs::lifecycle::HookContext"))>

Gets the `on_discard` [`ComponentHook`](../ecs/lifecycle/type.ComponentHook.html "type bevy::ecs::lifecycle::ComponentHook") for this [`Component`](../prelude/trait.Component.html "trait bevy::prelude::Component") if one is defined.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/component/mod.rs.html#539)

#### fn [on\_remove](../prelude/trait.Component.html#method.on_remove)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<for<'w> [fn](https://doc.rust-lang.org/nightly/std/primitive.fn.html)([DeferredWorld](../ecs/world/struct.DeferredWorld.html "struct bevy::ecs::world::DeferredWorld")<'w>, [HookContext](../ecs/lifecycle/struct.HookContext.html "struct bevy::ecs::lifecycle::HookContext"))>

Gets the `on_remove` [`ComponentHook`](../ecs/lifecycle/type.ComponentHook.html "type bevy::ecs::lifecycle::ComponentHook") for this [`Component`](../prelude/trait.Component.html "trait bevy::prelude::Component") if one is defined.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/component/mod.rs.html#544)

#### fn [on\_despawn](../prelude/trait.Component.html#method.on_despawn)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<for<'w> [fn](https://doc.rust-lang.org/nightly/std/primitive.fn.html)([DeferredWorld](../ecs/world/struct.DeferredWorld.html "struct bevy::ecs::world::DeferredWorld")<'w>, [HookContext](../ecs/lifecycle/struct.HookContext.html "struct bevy::ecs::lifecycle::HookContext"))>

Gets the `on_despawn` [`ComponentHook`](../ecs/lifecycle/type.ComponentHook.html "type bevy::ecs::lifecycle::ComponentHook") for this [`Component`](../prelude/trait.Component.html "trait bevy::prelude::Component") if one is defined.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/component/mod.rs.html#649)

#### fn [map\_entities](../prelude/trait.Component.html#method.map_entities)<E>(\_this: &mut Self, \_mapper: [&mut E](https://doc.rust-lang.org/nightly/std/primitive.reference.html))

where E: [EntityMapper](../prelude/trait.EntityMapper.html "trait bevy::prelude::EntityMapper"),

Maps the entities on this component using the given [`EntityMapper`](../prelude/trait.EntityMapper.html "trait bevy::prelude::EntityMapper"). This is used to remap entities in contexts like scenes and entity cloning. When deriving [`Component`](../prelude/trait.Component.html "trait bevy::prelude::Component"), this is populated by annotating fields containing entities with `#[entities]` [Read more](../prelude/trait.Component.html#method.map_entities)

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/server/mod.rs.html#2485)

### impl [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") for [AssetServer](../prelude/struct.AssetServer.html "struct bevy::prelude::AssetServer")

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/server/mod.rs.html#2486)

#### fn [fmt](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)(&self, f: &mut [Formatter](https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html "struct core::fmt::Formatter")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html "struct core::fmt::Error")\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/io/embedded/mod.rs.html#165)

### impl [GetAssetServer](io/embedded/trait.GetAssetServer.html "trait bevy::asset::io::embedded::GetAssetServer") for [AssetServer](../prelude/struct.AssetServer.html "struct bevy::prelude::AssetServer")

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/io/embedded/mod.rs.html#166)

#### fn [get\_asset\_server](io/embedded/trait.GetAssetServer.html#tymethod.get_asset_server)(&self) -> &[AssetServer](../prelude/struct.AssetServer.html "struct bevy::prelude::AssetServer")

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/reflect.rs.html#416)

### impl [LoadFromPath](trait.LoadFromPath.html "trait bevy::asset::LoadFromPath") for [AssetServer](../prelude/struct.AssetServer.html "struct bevy::prelude::AssetServer")

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/reflect.rs.html#417-421)

#### fn [load\_from\_path\_erased](trait.LoadFromPath.html#tymethod.load_from_path_erased)( &mut self, type\_id: [TypeId](https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html "struct core::any::TypeId"), path: [AssetPath](struct.AssetPath.html "struct bevy::asset::AssetPath")<'static>, ) -> [UntypedHandle](../prelude/enum.UntypedHandle.html "enum bevy::prelude::UntypedHandle")

Initiates the load for the given expected type ID, and the path. [Read more](trait.LoadFromPath.html#tymethod.load_from_path_erased)

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/reflect.rs.html#426)

### impl [LoadFromPath](trait.LoadFromPath.html "trait bevy::asset::LoadFromPath") for &[AssetServer](../prelude/struct.AssetServer.html "struct bevy::prelude::AssetServer")

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/reflect.rs.html#427-431)

#### fn [load\_from\_path\_erased](trait.LoadFromPath.html#tymethod.load_from_path_erased)( &mut self, type\_id: [TypeId](https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html "struct core::any::TypeId"), path: [AssetPath](struct.AssetPath.html "struct bevy::asset::AssetPath")<'static>, ) -> [UntypedHandle](../prelude/enum.UntypedHandle.html "enum bevy::prelude::UntypedHandle")

Initiates the load for the given expected type ID, and the path. [Read more](trait.LoadFromPath.html#tymethod.load_from_path_erased)

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/server/mod.rs.html#65)

### impl [Resource](../prelude/trait.Resource.html "trait bevy::prelude::Resource") for [AssetServer](../prelude/struct.AssetServer.html "struct bevy::prelude::AssetServer")

where [AssetServer](../prelude/struct.AssetServer.html "struct bevy::prelude::AssetServer"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

## Auto Trait Implementations

### impl ![RefUnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html "trait core::panic::unwind_safe::RefUnwindSafe") for [AssetServer](../prelude/struct.AssetServer.html "struct bevy::prelude::AssetServer")

### impl ![UnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html "trait core::panic::unwind_safe::UnwindSafe") for [AssetServer](../prelude/struct.AssetServer.html "struct bevy::prelude::AssetServer")

### impl [Freeze](https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html "trait core::marker::Freeze") for [AssetServer](../prelude/struct.AssetServer.html "struct bevy::prelude::AssetServer")

### impl [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") for [AssetServer](../prelude/struct.AssetServer.html "struct bevy::prelude::AssetServer")

### impl [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") for [AssetServer](../prelude/struct.AssetServer.html "struct bevy::prelude::AssetServer")

### impl [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin") for [AssetServer](../prelude/struct.AssetServer.html "struct bevy::prelude::AssetServer")

### impl [UnsafeUnpin](https://doc.rust-lang.org/nightly/core/marker/trait.UnsafeUnpin.html "trait core::marker::UnsafeUnpin") for [AssetServer](../prelude/struct.AssetServer.html "struct bevy::prelude::AssetServer")

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

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/bundle/impls.rs.html#16)

### impl<C> [Bundle](../prelude/trait.Bundle.html "trait bevy::prelude::Bundle") for C

where C: [Component](../prelude/trait.Component.html "trait bevy::prelude::Component"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/bundle/impls.rs.html#17-19)

#### fn [component\_ids](../prelude/trait.Bundle.html#tymethod.component_ids)( components: &mut [ComponentsRegistrator](../ecs/component/struct.ComponentsRegistrator.html "struct bevy::ecs::component::ComponentsRegistrator")<'\_>, ) -> impl [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = [ComponentId](../ecs/component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId")\> + use<C>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/bundle/impls.rs.html#23)

#### fn [get\_component\_ids](../prelude/trait.Bundle.html#tymethod.get_component_ids)( components: &[Components](../ecs/component/struct.Components.html "struct bevy::ecs::component::Components"), ) -> impl [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[ComponentId](../ecs/component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId")\>>

Return a iterator over this [`Bundle`](../prelude/trait.Bundle.html "trait bevy::prelude::Bundle")’s component ids. This will be [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") if the component has not been registered.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/bundle/impls.rs.html#30)

### impl<C> [BundleFromComponents](../ecs/bundle/trait.BundleFromComponents.html "trait bevy::ecs::bundle::BundleFromComponents") for C

where C: [Component](../prelude/trait.Component.html "trait bevy::prelude::Component"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/bundle/impls.rs.html#31-35)

#### unsafe fn [from\_components](../ecs/bundle/trait.BundleFromComponents.html#tymethod.from_components)<T, F>(ctx: [&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html), func: [&mut F](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> C

where F: for<'a> [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")([&'a mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [OwningPtr](../ecs/ptr/struct.OwningPtr.html "struct bevy::ecs::ptr::OwningPtr")<'a>, C: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/src/zerocopy/pointer/invariant.rs.html#244)

### impl<ST, DT> [CastableFrom](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/trait.CastableFrom.html "trait zerocopy::pointer::invariant::CastableFrom")<ST, [Initialized](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/enum.Initialized.html "enum zerocopy::pointer::invariant::Initialized"), [Initialized](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/enum.Initialized.html "enum zerocopy::pointer::invariant::Initialized")\> for DT

where ST: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), DT: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/src/zerocopy/pointer/invariant.rs.html#242)

### impl<ST, DT> [CastableFrom](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/trait.CastableFrom.html "trait zerocopy::pointer::invariant::CastableFrom")<ST, [Uninit](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/enum.Uninit.html "enum zerocopy::pointer::invariant::Uninit"), [Uninit](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/enum.Uninit.html "enum zerocopy::pointer::invariant::Uninit")\> for DT

where ST: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), DT: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#648)

### impl<T> [CloneToUninit](https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html "trait core::clone::CloneToUninit") for T

where T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

[Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#650)

#### unsafe fn [clone\_to\_uninit](https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html#tymethod.clone_to_uninit)(&self, dest: [\*mut](https://doc.rust-lang.org/nightly/std/primitive.pointer.html) [u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html))

🔬This is a nightly-only experimental API. (`clone_to_uninit`)

Performs copy-assignment from `self` to `dest`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.CloneToUninit.html#tymethod.clone_to_uninit)

[Source](https://docs.rs/bevy_tasks/0.19.0/x86_64-unknown-linux-gnu/src/bevy_tasks/lib.rs.html#50)

### impl<T> [ConditionalSend](../tasks/trait.ConditionalSend.html "trait bevy::tasks::ConditionalSend") for T

where T: [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send"),

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

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#215)

### impl<T> [DowncastSend](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.DowncastSend.html "trait downcast_rs::DowncastSend") for T

where T: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send"),

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#216)

#### fn [into\_any\_send](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.DowncastSend.html#tymethod.into_any_send)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<T>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send")\>

Converts `Box<Trait>` (where `Trait: DowncastSend`) to `Box<dyn Any + Send>`, which can then be `downcast` into `Box<ConcreteType>` where `ConcreteType` implements `Trait`.

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#203)

### impl<T> [DowncastSync](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.DowncastSync.html "trait downcast_rs::DowncastSync") for T

where T: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#204)

#### fn [into\_any\_arc](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.DowncastSync.html#tymethod.into_any_arc)(self: [Arc](../platform/sync/struct.Arc.html "struct bevy::platform::sync::Arc")<T>) -> [Arc](../platform/sync/struct.Arc.html "struct bevy::platform::sync::Arc")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync")\> [ⓘ](#)

Convert `Arc<Trait>` (where `Trait: Downcast`) to `Arc<Any>`. `Arc<Any>` can then be further `downcast` into `Arc<ConcreteType>` where `ConcreteType` implements `Trait`.

[Source](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/src/dasp_sample/conv.rs.html#699)

### impl<S, T> [Duplex](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/conv/trait.Duplex.html "trait dasp_sample::conv::Duplex")<S> for T

where T: [FromSample](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/conv/trait.FromSample.html "trait dasp_sample::conv::FromSample")<S> + [ToSample](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/conv/trait.ToSample.html "trait dasp_sample::conv::ToSample")<S>,

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/bundle/impls.rs.html#43)

### impl<C> [DynamicBundle](../ecs/bundle/trait.DynamicBundle.html "trait bevy::ecs::bundle::DynamicBundle") for C

where C: [Component](../prelude/trait.Component.html "trait bevy::prelude::Component"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/bundle/impls.rs.html#44)

#### type [Effect](../ecs/bundle/trait.DynamicBundle.html#associatedtype.Effect) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

An operation on the entity that happens _after_ inserting this bundle.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/bundle/impls.rs.html#46-49)

#### unsafe fn [get\_components](../ecs/bundle/trait.DynamicBundle.html#tymethod.get_components)( ptr: [MovingPtr](../ecs/ptr/struct.MovingPtr.html "struct bevy::ecs::ptr::MovingPtr")<'\_, C>, func: &mut impl [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")([StorageType](../ecs/component/enum.StorageType.html "enum bevy::ecs::component::StorageType"), [OwningPtr](../ecs/ptr/struct.OwningPtr.html "struct bevy::ecs::ptr::OwningPtr")<'\_>), ) -> <C as [DynamicBundle](../ecs/bundle/trait.DynamicBundle.html "trait bevy::ecs::bundle::DynamicBundle")\>::[Effect](../ecs/bundle/trait.DynamicBundle.html#associatedtype.Effect "type bevy::ecs::bundle::DynamicBundle::Effect")

Moves the components out of the bundle. [Read more](../ecs/bundle/trait.DynamicBundle.html#tymethod.get_components)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/bundle/impls.rs.html#54)

#### unsafe fn [apply\_effect](../ecs/bundle/trait.DynamicBundle.html#tymethod.apply_effect)( \_ptr: [MovingPtr](../ecs/ptr/struct.MovingPtr.html "struct bevy::ecs::ptr::MovingPtr")<'\_, [MaybeUninit](https://doc.rust-lang.org/nightly/core/mem/maybe_uninit/union.MaybeUninit.html "union core::mem::maybe_uninit::MaybeUninit")<C>>, \_entity: &mut [EntityWorldMut](../prelude/struct.EntityWorldMut.html "struct bevy::prelude::EntityWorldMut")<'\_>, )

Applies the after-effects of spawning this bundle. [Read more](../ecs/bundle/trait.DynamicBundle.html#tymethod.apply_effect)

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

[Source](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/src/typewit/type_witness_traits.rs.html#106-109)

### impl<T, W> [HasTypeWitness](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/typewit/type_witness_traits/trait.HasTypeWitness.html "trait typewit::type_witness_traits::HasTypeWitness")<W> for T

where W: [MakeTypeWitness](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/typewit/type_witness_traits/trait.MakeTypeWitness.html "trait typewit::type_witness_traits::MakeTypeWitness")<Arg = T>, T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/src/typewit/type_witness_traits.rs.html#111)

#### const [WITNESS](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/typewit/type_witness_traits/trait.HasTypeWitness.html#associatedconstant.WITNESS): W = W::MAKE

A constant of the type witness

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/backend.rs.html#80)

### impl<T> [HitDataExtra](../picking/backend/trait.HitDataExtra.html "trait bevy::picking::backend::HitDataExtra") for T

where T: [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") + [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static,

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

[Source](https://docs.rs/typenum/1.20.1/x86_64-unknown-linux-gnu/src/typenum/type_operators.rs.html#34)

### impl<T> [Same](https://docs.rs/typenum/1.20.1/x86_64-unknown-linux-gnu/typenum/type_operators/trait.Same.html "trait typenum::type_operators::Same") for T

[Source](https://docs.rs/typenum/1.20.1/x86_64-unknown-linux-gnu/src/typenum/type_operators.rs.html#35)

#### type [Output](https://docs.rs/typenum/1.20.1/x86_64-unknown-linux-gnu/typenum/type_operators/trait.Same.html#associatedtype.Output) = T

Should always be `Self`

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/meta.rs.html#190)

### impl<T> [Settings](meta/trait.Settings.html "trait bevy::asset::meta::Settings") for T

where T: 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

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

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#72-74)

### impl<T> [ToOwned](../prelude/trait.ToOwned.html "trait bevy::prelude::ToOwned") for T

where T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#76)

#### type [Owned](../prelude/trait.ToOwned.html#associatedtype.Owned) = T

The resulting type after obtaining ownership.

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#77)

#### fn [to\_owned](../prelude/trait.ToOwned.html#tymethod.to_owned)(&self) -> T

Creates owned data from borrowed data, usually by cloning. [Read more](../prelude/trait.ToOwned.html#tymethod.to_owned)

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#81)

#### fn [clone\_into](../prelude/trait.ToOwned.html#method.clone_into)(&self, target: [&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html))

Uses borrowed data to replace owned data, usually by cloning. [Read more](../prelude/trait.ToOwned.html#method.clone_into)

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

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/type_registry.rs.html#811-813)

### impl<T> [TypeData](../reflect/trait.TypeData.html "trait bevy::reflect::TypeData") for T

where T: 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/type_registry.rs.html#815)

#### fn [clone\_type\_data](../reflect/trait.TypeData.html#tymethod.clone_type_data)(&self) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [TypeData](../reflect/trait.TypeData.html "trait bevy::reflect::TypeData")\>

Creates a type-erased clone of this value.

[Source](https://docs.rs/ppv-lite86/0.2.21/x86_64-unknown-linux-gnu/src/ppv_lite86/types.rs.html#221-223)

### impl<V, T> [VZip](https://docs.rs/ppv-lite86/0.2.21/x86_64-unknown-linux-gnu/ppv_lite86/types/trait.VZip.html "trait ppv_lite86::types::VZip")<V> for T

where V: [MultiLane](https://docs.rs/ppv-lite86/0.2.21/x86_64-unknown-linux-gnu/ppv_lite86/types/trait.MultiLane.html "trait ppv_lite86::types::MultiLane")<T>,

[Source](https://docs.rs/ppv-lite86/0.2.21/x86_64-unknown-linux-gnu/src/ppv_lite86/types.rs.html#226)

#### fn [vzip](https://docs.rs/ppv-lite86/0.2.21/x86_64-unknown-linux-gnu/ppv_lite86/types/trait.VZip.html#tymethod.vzip)(self) -> V

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/send_sync.rs.html#18)

### impl<T> [WasmNotSend](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/wgpu_types/send_sync/trait.WasmNotSend.html "trait wgpu_types::send_sync::WasmNotSend") for T

where T: [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send"),

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/send_sync.rs.html#2)

### impl<T> [WasmNotSendSync](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/wgpu_types/send_sync/trait.WasmNotSendSync.html "trait wgpu_types::send_sync::WasmNotSendSync") for T

where T: [WasmNotSend](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/wgpu_types/send_sync/trait.WasmNotSend.html "trait wgpu_types::send_sync::WasmNotSend") + [WasmNotSync](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/wgpu_types/send_sync/trait.WasmNotSync.html "trait wgpu_types::send_sync::WasmNotSync"),

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/send_sync.rs.html#51)

### impl<T> [WasmNotSync](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/wgpu_types/send_sync/trait.WasmNotSync.html "trait wgpu_types::send_sync::WasmNotSync") for T

where T: [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#393)

### impl<T> [WithSubscriber](../log/tracing/instrument/trait.WithSubscriber.html "trait bevy::log::tracing::instrument::WithSubscriber") for T

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#176-178)

#### fn [with\_subscriber](../log/tracing/instrument/trait.WithSubscriber.html#method.with_subscriber)<S>(self, subscriber: S) -> [WithDispatch](../log/tracing/instrument/struct.WithDispatch.html "struct bevy::log::tracing::instrument::WithDispatch")<Self> [ⓘ](#)

where S: [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Dispatch](../log/tracing/struct.Dispatch.html "struct bevy::log::tracing::Dispatch")\>,

Attaches the provided [`Subscriber`](../log/tracing/trait.Subscriber.html "trait bevy::log::tracing::Subscriber") to this type, returning a [`WithDispatch`](../log/tracing/instrument/struct.WithDispatch.html "struct bevy::log::tracing::instrument::WithDispatch") wrapper. [Read more](../log/tracing/instrument/trait.WithSubscriber.html#method.with_subscriber)

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#228)

#### fn [with\_current\_subscriber](../log/tracing/instrument/trait.WithSubscriber.html#method.with_current_subscriber)(self) -> [WithDispatch](../log/tracing/instrument/struct.WithDispatch.html "struct bevy::log::tracing::instrument::WithDispatch")<Self> [ⓘ](#)

Attaches the current [default](../log/tracing/dispatcher/index.html#setting-the-default-subscriber "mod bevy::log::tracing::dispatcher") [`Subscriber`](../log/tracing/trait.Subscriber.html "trait bevy::log::tracing::Subscriber") to this type, returning a [`WithDispatch`](../log/tracing/instrument/struct.WithDispatch.html "struct bevy::log::tracing::instrument::WithDispatch") wrapper. [Read more](../log/tracing/instrument/trait.WithSubscriber.html#method.with_current_subscriber)

{"Arc<dyn Any + Send + Sync>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../platform/sync/struct.Arc.html\\" title=\\"struct bevy::platform::sync::Arc\\">Arc</a>&lt;R&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;R&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a> for <a class=\\"struct\\" href=\\"../platform/sync/struct.Arc.html\\" title=\\"struct bevy::platform::sync::Arc\\">Arc</a>&lt;R&gt;<div class=\\"where\\">where\\n R: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a> + IoHandle + ?<a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\\" title=\\"trait core::marker::Sized\\">Sized</a>,\\n <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;'a R</a>: for&lt;'a&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a>,</div></div><div class=\\"where\\">impl&lt;W&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a> for <a class=\\"struct\\" href=\\"../platform/sync/struct.Arc.html\\" title=\\"struct bevy::platform::sync::Arc\\">Arc</a>&lt;W&gt;<div class=\\"where\\">where\\n W: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a> + IoHandle + ?<a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\\" title=\\"trait core::marker::Sized\\">Sized</a>,\\n <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;'a W</a>: for&lt;'a&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a>,</div></div>","Either<Self, Self>":"<h3>Notable traits for <code><a class=\\"enum\\" href=\\"https://docs.rs/either/1.16.0/x86\_64-unknown-linux-gnu/either/enum.Either.html\\" title=\\"enum either::Either\\">Either</a>&lt;L, R&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;L, R&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"enum\\" href=\\"https://docs.rs/either/1.16.0/x86\_64-unknown-linux-gnu/either/enum.Either.html\\" title=\\"enum either::Either\\">Either</a>&lt;L, R&gt;<div class=\\"where\\">where\\n L: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>,\\n R: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&lt;Item = &lt;L as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>&gt;,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &lt;L as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>;</div><div class=\\"where\\">impl&lt;L, R&gt; <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"enum\\" href=\\"https://docs.rs/either/1.16.0/x86\_64-unknown-linux-gnu/either/enum.Either.html\\" title=\\"enum either::Either\\">Either</a>&lt;L, R&gt;<div class=\\"where\\">where\\n L: <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,\\n R: <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&lt;Output = &lt;L as <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>&gt;,</div></div><div class=\\"where\\"> type <a href=\\"../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;L as <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>","Instrumented<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../log/tracing/instrument/struct.Instrumented.html\\" title=\\"struct bevy::log::tracing::instrument::Instrumented\\">Instrumented</a>&lt;T&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;T&gt; <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"../log/tracing/instrument/struct.Instrumented.html\\" title=\\"struct bevy::log::tracing::instrument::Instrumented\\">Instrumented</a>&lt;T&gt;<div class=\\"where\\">where\\n T: <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,</div></div><div class=\\"where\\"> type <a href=\\"../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;T as <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>","WithDispatch<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../log/tracing/instrument/struct.WithDispatch.html\\" title=\\"struct bevy::log::tracing::instrument::WithDispatch\\">WithDispatch</a>&lt;T&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;T&gt; <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"../log/tracing/instrument/struct.WithDispatch.html\\" title=\\"struct bevy::log::tracing::instrument::WithDispatch\\">WithDispatch</a>&lt;T&gt;<div class=\\"where\\">where\\n T: <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,</div></div><div class=\\"where\\"> type <a href=\\"../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;T as <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>"}