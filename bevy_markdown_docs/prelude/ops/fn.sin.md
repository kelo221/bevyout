[bevy](../../index.html)::[prelude](../index.html)::[ops](index.html)

# Function sin 

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/ops.rs.html#310)

```rust
pub fn sin(x: f32) -> f32
```

Computes the sine of a number (in radians).

Precision is specified when the `libm` feature is enabled.

##### [Examples found in repository](#scraped-examples)[?](../../../scrape-examples-help.html)

examples/time/virtual\_time.rs ([line 153](../../../src/virtual_time/virtual_time.rs.html#153))

```rust
152fn get_sprite_translation_x(elapsed: f32) -> f32 {
153    ops::sin(elapsed) * 500.
154}
```

Hide additional examples

examples/stress\_tests/transform\_hierarchy.rs ([line 268](../../../src/transform_hierarchy/transform_hierarchy.rs.html#268))

```rust
266fn set_translation(translation: &mut Vec3, a: f32) {
267    translation.x = ops::cos(a) * 32.0;
268    translation.y = ops::sin(a) * 32.0;
269}
```

examples/gizmos/axes.rs ([line 203](../../../src/axes/axes.rs.html#203))

```rust
201fn build_direction(height: f32, theta: f32) -> Vec3 {
202    let z = height;
203    let m = ops::sin(ops::acos(z));
204    let x = ops::cos(theta) * m;
205    let y = ops::sin(theta) * m;
206
207    Vec3::new(x, y, z)
208}
```

examples/ui/scroll\_and\_overflow/overflow\_debug.rs ([line 52](../../../src/overflow_debug/overflow_debug.rs.html#52))

```rust
51    fn update(&self, t: f32, transform: &mut UiTransform) {
52        transform.translation.x = percent(ops::sin(t * TAU - FRAC_PI_2) * 50.);
53        transform.translation.y = percent(-ops::cos(t * TAU - FRAC_PI_2) * 50.);
54    }
```

examples/shader/shader\_prepass.rs ([line 175](../../../src/shader_prepass/shader_prepass.rs.html#175))

```rust
173fn rotate(mut q: Query<&mut Transform, With<Rotates>>, time: Res<Time>) {
174    for mut t in q.iter_mut() {
175        let rot = (ops::sin(time.elapsed_secs()) * 0.5 + 0.5) * std::f32::consts::PI * 2.0;
176        t.rotation = Quat::from_rotation_z(rot);
177    }
178}
```

examples/3d/transparency\_3d.rs ([line 111](../../../src/transparency_3d/transparency_3d.rs.html#111))

```rust
110pub fn fade_transparency(time: Res<Time>, mut materials: ResMut<Assets<StandardMaterial>>) {
111    let alpha = (ops::sin(time.elapsed_secs()) / 2.0) + 0.5;
112    for (_, material) in materials.iter_mut() {
113        material.base_color.set_alpha(alpha);
114    }
115}
```

Additional examples can be found in:  

*   [examples/stress\_tests/text\_pipeline.rs](../../../src/text_pipeline/text_pipeline.rs.html#82)
*   [examples/3d/bloom\_3d.rs](../../../src/bloom_3d/bloom_3d.rs.html#226)
*   [examples/audio/decodable.rs](../../../src/decodable/decodable.rs.html#51)
*   [examples/audio/audio\_control.rs](../../../src/audio_control/audio_control.rs.html#69)
*   [examples/state/custom\_transitions.rs](../../../src/custom_transitions/custom_transitions.rs.html#199)
*   [examples/state/states.rs](../../../src/states/states.rs.html#155)
*   [examples/state/sub\_states.rs](../../../src/sub_states/sub_states.rs.html#120)
*   [examples/shader/automatic\_instancing.rs](../../../src/automatic_instancing/automatic_instancing.rs.html#80)
*   [examples/2d/text2d.rs](../../../src/text2d/text2d.rs.html#181)
*   [examples/state/computed\_states.rs](../../../src/computed_states/computed_states.rs.html#515)
*   [examples/3d/anisotropy.rs](../../../src/anisotropy/anisotropy.rs.html#188)
*   [examples/picking/sprite\_picking.rs](../../../src/sprite_picking/sprite_picking.rs.html#22)
*   [examples/showcase/alien\_cake\_addict.rs](../../../src/alien_cake_addict/alien_cake_addict.rs.html#372)
*   [examples/async\_tasks/async\_channel\_pattern.rs](../../../src/async_channel_pattern/async_channel_pattern.rs.html#164)
*   [examples/3d/motion\_blur.rs](../../../src/motion_blur/motion_blur.rs.html#290)
*   [examples/3d/clearcoat.rs](../../../src/clearcoat/clearcoat.rs.html#250)
*   [examples/math/bounding\_2d.rs](../../../src/bounding_2d/bounding_2d.rs.html#286)
*   [examples/ui/text/text.rs](../../../src/text/text.rs.html#182)
*   [examples/2d/tilemap\_chunk.rs](../../../src/tilemap_chunk/tilemap_chunk.rs.html#102)
*   [examples/animation/color\_animation.rs](../../../src/color_animation/color_animation.rs.html#93)
*   [examples/shader\_advanced/fullscreen\_material.rs](../../../src/fullscreen_material/fullscreen_material.rs.html#53)
*   [examples/3d/spotlight.rs](../../../src/spotlight/spotlight.rs.html#141)
*   [examples/math/render\_primitives.rs](../../../src/render_primitives/render_primitives.rs.html#658)
*   [examples/shader\_advanced/custom\_post\_processing.rs](../../../src/custom_post_processing/custom_post_processing.rs.html#291)
*   [examples/gltf/update\_gltf\_scene.rs](../../../src/update_gltf_scene/update_gltf_scene.rs.html#68)
*   [examples/audio/spatial\_audio\_2d.rs](../../../src/spatial_audio_2d/spatial_audio_2d.rs.html#101)
*   [examples/stress\_tests/many\_gizmos.rs](../../../src/many_gizmos/many_gizmos.rs.html#71)
*   [examples/shader/storage\_buffer.rs](../../../src/storage_buffer/storage_buffer.rs.html#82)
*   [examples/audio/spatial\_audio\_3d.rs](../../../src/spatial_audio_3d/spatial_audio_3d.rs.html#106)
*   [examples/3d/mesh\_ray\_cast.rs](../../../src/mesh_ray_cast/mesh_ray_cast.rs.html#32)
*   [tests/3d/test\_skinned\_mesh\_bounds.rs](../../../src/test_skinned_mesh_bounds/test_skinned_mesh_bounds.rs.html#269)
*   [examples/gltf/gltf\_skinned\_mesh.rs](../../../src/gltf_skinned_mesh/gltf_skinned_mesh.rs.html#69)
*   [examples/gltf/query\_gltf\_primitives.rs](../../../src/query_gltf_primitives/query_gltf_primitives.rs.html#44)
*   [examples/stress\_tests/many\_gradients.rs](../../../src/many_gradients/many_gradients.rs.html#150)
*   [examples/2d/cpu\_draw.rs](../../../src/cpu_draw/cpu_draw.rs.html#119)
*   [examples/animation/custom\_skinned\_mesh.rs](../../../src/custom_skinned_mesh/custom_skinned_mesh.rs.html#201)
*   [examples/3d/light\_probe\_blending.rs](../../../src/light_probe_blending/light_probe_blending.rs.html#439)
*   [examples/3d/clustered\_decals.rs](../../../src/clustered_decals/clustered_decals.rs.html#425)
*   [examples/3d/light\_textures.rs](../../../src/light_textures/light_textures.rs.html#507)
*   [examples/stress\_tests/many\_cameras\_lights.rs](../../../src/many_cameras_lights/many_cameras_lights.rs.html#64)
*   [examples/gizmos/2d\_gizmos.rs](../../../src/2d_gizmos/2d_gizmos.rs.html#46)
*   [examples/2d/mesh2d\_manual.rs](../../../src/mesh2d_manual/mesh2d_manual.rs.html#85)
*   [examples/gizmos/3d\_gizmos.rs](../../../src/3d_gizmos/3d_gizmos.rs.html#153)
*   [examples/3d/ssao.rs](../../../src/ssao/ssao.rs.html#105)
*   [examples/3d/fog.rs](../../../src/fog/fog.rs.html#149)