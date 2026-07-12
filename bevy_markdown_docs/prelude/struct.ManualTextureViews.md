[bevy](../index.html)::[prelude](index.html)

# Struct ManualTextureViews 

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/texture/manual_texture_view.rs.html#53)

```rust
pub struct ManualTextureViews(/* private fields */);
```

Resource that stores manually managed [`ManualTextureView`](../render/texture/struct.ManualTextureView.html "struct bevy::render::texture::ManualTextureView")s for use as a [`RenderTarget`](../camera/enum.RenderTarget.html "enum bevy::camera::RenderTarget"). This type dereferences to a `HashMap<ManualTextureViewHandle, ManualTextureView>`. To add a new texture view, pick a new [`ManualTextureViewHandle`](../camera/struct.ManualTextureViewHandle.html "struct bevy::camera::ManualTextureViewHandle") and insert it into the map. Then, to render to the view, set a [`Camera`](struct.Camera.html "struct bevy::prelude::Camera")s `target` to `RenderTarget::TextureView(handle)`.

[ⓘ](# "This example is not tested")

```rust
let manual_views = world.resource_mut::<ManualTextureViews>();
let manual_view = ManualTextureView::with_default_format(texture_view, UVec2::new(1024, 1024));

// Choose an unused handle value; it's likely only you are inserting manual views.
const MANUAL_VIEW_HANDLE: ManualTextureViewHandle = ManualTextureViewHandle::new(42);
manual_views.insert(MANUAL_VIEW_HANDLE, manual_view);

// Now you can spawn a Camera that renders to the manual view:
world.spawn(Camera {
    target: RenderTarget::TextureView(MANUAL_VIEW_HANDLE),
    ..Default::default()
});
```

Bevy will then use the `ManualTextureViews` resource to find your texture view and render to it.

## Methods from [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref")<Target = [HashMap](../platform/collections/struct.HashMap.html "struct bevy::platform::collections::HashMap")<[ManualTextureViewHandle](../camera/struct.ManualTextureViewHandle.html "struct bevy::camera::ManualTextureViewHandle"), [ManualTextureView](../render/texture/struct.ManualTextureView.html "struct bevy::render::texture::ManualTextureView")\>>

[Source](https://docs.rs/bevy_platform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_platform/collections/hash_map.rs.html#402)

#### pub fn [hasher](#method.hasher)(&self) -> [&S](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

Returns a reference to the map’s [`BuildHasher`](https://doc.rust-lang.org/nightly/core/hash/trait.BuildHasher.html "trait core::hash::BuildHasher"), or `S` parameter.

Refer to [`hasher`](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/hashbrown/map/struct.HashMap.html#method.hasher "method hashbrown::map::HashMap::hasher") for further details.

[Source](https://docs.rs/bevy_platform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_platform/collections/hash_map.rs.html#421)

#### pub fn [capacity](#method.capacity)(&self) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

Returns the number of elements the map can hold without reallocating.

Refer to [`capacity`](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/hashbrown/map/struct.HashMap.html#method.capacity "method hashbrown::map::HashMap::capacity") for further details.

##### Examples

```rust
let map = HashMap::with_capacity(5);

assert!(map.capacity() >= 5);
```

[Source](https://docs.rs/bevy_platform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_platform/collections/hash_map.rs.html#449)

#### pub fn [keys](#method.keys)(&self) -> [Keys](../platform/collections/hash_map/struct.Keys.html "struct bevy::platform::collections::hash_map::Keys")<'\_, K, V> [ⓘ](#)

An iterator visiting all keys in arbitrary order. The iterator element type is `&'a K`.

Refer to [`keys`](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/hashbrown/map/struct.HashMap.html#method.keys "method hashbrown::map::HashMap::keys") for further details.

##### Examples

```rust
let mut map = HashMap::new();

map.insert("foo", 0);
map.insert("bar", 1);
map.insert("baz", 2);

for key in map.keys() {
    // foo, bar, baz
    // Note that the above order is not guaranteed
}
```

[Source](https://docs.rs/bevy_platform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_platform/collections/hash_map.rs.html#477)

#### pub fn [values](#method.values)(&self) -> [Values](../platform/collections/hash_map/struct.Values.html "struct bevy::platform::collections::hash_map::Values")<'\_, K, V> [ⓘ](#)

An iterator visiting all values in arbitrary order. The iterator element type is `&'a V`.

Refer to [`values`](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/hashbrown/map/struct.HashMap.html#method.values "method hashbrown::map::HashMap::values") for further details.

##### Examples

```rust
let mut map = HashMap::new();

map.insert("foo", 0);
map.insert("bar", 1);
map.insert("baz", 2);

for key in map.values() {
    // 0, 1, 2
    // Note that the above order is not guaranteed
}
```

##### [Examples found in repository](#scraped-examples)[?](../../scrape-examples-help.html)

tests/ecs/ambiguity\_detection.rs ([line 66](../../src/ambiguity_detection/ambiguity_detection.rs.html#66))

```rust
65    fn total(&self) -> usize {
66        self.values().sum()
67    }
```

Hide additional examples

examples/ui/scroll\_and\_overflow/scroll.rs ([line 44](../../src/scroll/scroll.rs.html#44))

```rust
27fn send_scroll_events(
28    mut mouse_wheel_reader: MessageReader<MouseWheel>,
29    hover_map: Res<HoverMap>,
30    keyboard_input: Res<ButtonInput<KeyCode>>,
31    mut commands: Commands,
32) {
33    for mouse_wheel in mouse_wheel_reader.read() {
34        let mut delta = -Vec2::new(mouse_wheel.x, mouse_wheel.y);
35
36        if mouse_wheel.unit == MouseScrollUnit::Line {
37            delta *= LINE_HEIGHT;
38        }
39
40        if keyboard_input.any_pressed([KeyCode::ControlLeft, KeyCode::ControlRight]) {
41            std::mem::swap(&mut delta.x, &mut delta.y);
42        }
43
44        for pointer_map in hover_map.values() {
45            for entity in pointer_map.keys().copied() {
46                commands.trigger(Scroll { entity, delta });
47            }
48        }
49    }
50}
```

examples/ui/text/font\_atlas\_debug.rs ([line 45](../../src/font_atlas_debug/font_atlas_debug.rs.html#45))

```rust
39fn atlas_render_system(
40    mut commands: Commands,
41    mut state: ResMut<State>,
42    font_atlas_set: Res<FontAtlasSet>,
43    images: Res<Assets<Image>>,
44) {
45    if let Some(font_atlases) = font_atlas_set.values().next() {
46        let x_offset = state.atlas_count as f32;
47        if state.atlas_count == font_atlases.len() as u32 {
48            return;
49        }
50        let font_atlas = &font_atlases[state.atlas_count as usize];
51        let image = images.get(&font_atlas.texture).unwrap();
52        state.atlas_count += 1;
53        commands.spawn((
54            ImageNode::new(font_atlas.texture.clone()),
55            Node {
56                position_type: PositionType::Absolute,
57                top: Val::ZERO,
58                left: px(image.width() as f32 * x_offset),
59                ..default()
60            },
61        ));
62    }
63}
```

[Source](https://docs.rs/bevy_platform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_platform/collections/hash_map.rs.html#505)

#### pub fn [values\_mut](#method.values_mut)(&mut self) -> [ValuesMut](../platform/collections/hash_map/struct.ValuesMut.html "struct bevy::platform::collections::hash_map::ValuesMut")<'\_, K, V> [ⓘ](#)

An iterator visiting all values mutably in arbitrary order. The iterator element type is `&'a mut V`.

Refer to [`values`](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/hashbrown/map/struct.HashMap.html#method.values "method hashbrown::map::HashMap::values") for further details.

##### Examples

```rust
let mut map = HashMap::new();

map.insert("foo", 0);
map.insert("bar", 1);
map.insert("baz", 2);

for key in map.values_mut() {
    // 0, 1, 2
    // Note that the above order is not guaranteed
}
```

[Source](https://docs.rs/bevy_platform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_platform/collections/hash_map.rs.html#533)

#### pub fn [iter](#method.iter)(&self) -> [Iter](../platform/collections/hash_map/struct.Iter.html "struct bevy::platform::collections::hash_map::Iter")<'\_, K, V> [ⓘ](#)

An iterator visiting all key-value pairs in arbitrary order. The iterator element type is `(&'a K, &'a V)`.

Refer to [`iter`](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/hashbrown/map/struct.HashMap.html#method.iter "method hashbrown::map::HashMap::iter") for further details.

##### Examples

```rust
let mut map = HashMap::new();

map.insert("foo", 0);
map.insert("bar", 1);
map.insert("baz", 2);

for (key, value) in map.iter() {
    // ("foo", 0), ("bar", 1), ("baz", 2)
    // Note that the above order is not guaranteed
}
```

##### [Examples found in repository](#scraped-examples-1)[?](../../scrape-examples-help.html)

examples/stress\_tests/many\_text2d.rs ([line 182](../../src/many_text2d/many_text2d.rs.html#182))

```rust
169fn print_counts(
170    time: Res<Time>,
171    mut timer: Local<PrintingTimer>,
172    texts: Query<&ViewVisibility, With<Text2d>>,
173    font_atlas_set: Res<FontAtlasSet>,
174    images: Res<Assets<Image>>,
175) {
176    timer.tick(time.delta());
177    if !timer.just_finished() {
178        return;
179    }
180
181    let num_atlases = font_atlas_set
182        .iter()
183        // Removed this filter for now as the keys no longer include the AssetIds
184        //        .filter(|(key, _)| key.0 == font_id)
185        .map(|(_, atlases)| atlases.len())
186        .sum::<usize>();
187
188    let visible_texts = texts.iter().filter(|visibility| visibility.get()).count();
189
190    info!(
191        "Texts: {} Visible: {} Atlases: {} Bytes: {}",
192        texts.iter().count(),
193        visible_texts,
194        num_atlases,
195        font_atlas_set.total_bytes(images.as_ref())
196    );
197}
```

Hide additional examples

examples/testbed/full\_ui.rs ([line 456](../../src/testbed_full_ui/full_ui.rs.html#456))

```rust
439pub fn update_scroll_position(
440    mut mouse_wheel_reader: MessageReader<MouseWheel>,
441    hover_map: Res<HoverMap>,
442    mut scrolled_node_query: Query<(&mut ScrollPosition, &ComputedNode), Without<Scrollbar>>,
443    keyboard_input: Res<ButtonInput<KeyCode>>,
444) {
445    for mouse_wheel in mouse_wheel_reader.read() {
446        let (mut dx, mut dy) = match mouse_wheel.unit {
447            MouseScrollUnit::Line => (mouse_wheel.x * 20., mouse_wheel.y * 20.),
448            MouseScrollUnit::Pixel => (mouse_wheel.x, mouse_wheel.y),
449        };
450
451        if keyboard_input.pressed(KeyCode::ShiftLeft) || keyboard_input.pressed(KeyCode::ShiftRight)
452        {
453            std::mem::swap(&mut dx, &mut dy);
454        }
455
456        for (_pointer, pointer_map) in hover_map.iter() {
457            for (entity, _hit) in pointer_map.iter() {
458                if let Ok((mut scroll_position, scroll_content)) =
459                    scrolled_node_query.get_mut(*entity)
460                {
461                    let visible_size = scroll_content.size();
462                    let content_size = scroll_content.content_size();
463
464                    let range = (content_size.y - visible_size.y).max(0.)
465                        * scroll_content.inverse_scale_factor;
466
467                    scroll_position.x -= dx;
468                    scroll_position.y = (scroll_position.y - dy).clamp(0., range);
469                }
470            }
471        }
472    }
473}
```

[Source](https://docs.rs/bevy_platform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_platform/collections/hash_map.rs.html#562)

#### pub fn [iter\_mut](#method.iter_mut)(&mut self) -> [IterMut](../platform/collections/hash_map/struct.IterMut.html "struct bevy::platform::collections::hash_map::IterMut")<'\_, K, V> [ⓘ](#)

An iterator visiting all key-value pairs in arbitrary order, with mutable references to the values. The iterator element type is `(&'a K, &'a mut V)`.

Refer to [`iter_mut`](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/hashbrown/map/struct.HashMap.html#method.iter_mut "method hashbrown::map::HashMap::iter_mut") for further details.

##### Examples

```rust
let mut map = HashMap::new();

map.insert("foo", 0);
map.insert("bar", 1);
map.insert("baz", 2);

for (key, value) in map.iter_mut() {
    // ("foo", 0), ("bar", 1), ("baz", 2)
    // Note that the above order is not guaranteed
}
```

##### [Examples found in repository](#scraped-examples-2)[?](../../scrape-examples-help.html)

examples/3d/tonemapping.rs ([line 383](../../src/tonemapping/tonemapping.rs.html#383))

```rust
340fn update_color_grading_settings(
341    keys: Res<ButtonInput<KeyCode>>,
342    time: Res<Time>,
343    mut per_method_settings: ResMut<PerMethodSettings>,
344    tonemapping: Single<&Tonemapping>,
345    current_scene: Res<CurrentScene>,
346    mut selected_parameter: ResMut<SelectedParameter>,
347) {
348    let color_grading = per_method_settings.settings.get_mut(*tonemapping).unwrap();
349    let mut dt = time.delta_secs() * 0.25;
350    if keys.pressed(KeyCode::ArrowLeft) {
351        dt = -dt;
352    }
353
354    if keys.just_pressed(KeyCode::ArrowDown) {
355        selected_parameter.next();
356    }
357    if keys.just_pressed(KeyCode::ArrowUp) {
358        selected_parameter.prev();
359    }
360    if keys.pressed(KeyCode::ArrowLeft) || keys.pressed(KeyCode::ArrowRight) {
361        match selected_parameter.value {
362            0 => {
363                color_grading.global.exposure += dt;
364            }
365            1 => {
366                color_grading
367                    .all_sections_mut()
368                    .for_each(|section| section.gamma += dt);
369            }
370            2 => {
371                color_grading
372                    .all_sections_mut()
373                    .for_each(|section| section.saturation += dt);
374            }
375            3 => {
376                color_grading.global.post_saturation += dt;
377            }
378            _ => {}
379        }
380    }
381
382    if keys.just_pressed(KeyCode::Space) {
383        for (_, grading) in per_method_settings.settings.iter_mut() {
384            *grading = ColorGrading::default();
385        }
386    }
387
388    if keys.just_pressed(KeyCode::Enter) && current_scene.0 == 1 {
389        for (mapper, grading) in per_method_settings.settings.iter_mut() {
390            *grading = PerMethodSettings::basic_scene_recommendation(*mapper);
391        }
392    }
393}
```

[Source](https://docs.rs/bevy_platform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_platform/collections/hash_map.rs.html#583)

#### pub fn [len](#method.len)(&self) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

Returns the number of elements in the map.

Refer to [`len`](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/hashbrown/map/struct.HashMap.html#method.len "method hashbrown::map::HashMap::len") for further details.

##### Examples

```rust
let mut map = HashMap::new();

assert_eq!(map.len(), 0);

map.insert("foo", 0);

assert_eq!(map.len(), 1);
```

##### [Examples found in repository](#scraped-examples-3)[?](../../scrape-examples-help.html)

examples/stress\_tests/many\_lights.rs ([line 182](../../src/many_lights/many_lights.rs.html#182))

```rust
166fn print_visible_light_count(
167    time: Res<Time>,
168    mut timer: Local<PrintingTimer>,
169    visible: Query<&ExtractedPointLight>,
170    global_clusterable_object_meta: Res<GlobalClusterableObjectMeta>,
171) {
172    timer.0.tick(time.delta());
173
174    if timer.0.just_finished() {
175        // Note that it's not generally a safe assumption that the number of
176        // lights equals the number of clusterable objects, since some objects
177        // other than lights are clusterable. However, in this specific example,
178        // the only clusterable objects are lights.
179        info!(
180            "Visible Lights: {}, Rendered Lights: {}",
181            visible.iter().len(),
182            global_clusterable_object_meta.entity_to_index.len()
183        );
184    }
185}
```

[Source](https://docs.rs/bevy_platform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_platform/collections/hash_map.rs.html#604)

#### pub fn [is\_empty](#method.is_empty)(&self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Returns `true` if the map contains no elements.

Refer to [`is_empty`](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/hashbrown/map/struct.HashMap.html#method.is_empty "method hashbrown::map::HashMap::is_empty") for further details.

##### Examples

```rust
let mut map = HashMap::new();

assert!(map.is_empty());

map.insert("foo", 0);

assert!(!map.is_empty());
```

##### [Examples found in repository](#scraped-examples-4)[?](../../scrape-examples-help.html)

examples/2d/mesh2d\_manual.rs ([line 389](../../src/mesh2d_manual/mesh2d_manual.rs.html#389))

```rust
379pub fn queue_colored_mesh2d(
380    transparent_draw_functions: Res<DrawFunctions<Transparent2d>>,
381    colored_mesh2d_pipeline: Res<ColoredMesh2dPipeline>,
382    mut pipelines: ResMut<SpecializedRenderPipelines<ColoredMesh2dPipeline>>,
383    pipeline_cache: Res<PipelineCache>,
384    render_meshes: Res<RenderAssets<RenderMesh>>,
385    render_mesh_instances: Res<RenderColoredMesh2dInstances>,
386    mut transparent_render_phases: ResMut<ViewSortedRenderPhases<Transparent2d>>,
387    views: Query<(&RenderVisibleEntities, &ExtractedView, &Msaa)>,
388) {
389    if render_mesh_instances.is_empty() {
390        return;
391    }
392    // Iterate each view (a camera is a view)
393    for (visible_entities, view, msaa) in &views {
394        let Some(transparent_phase) = transparent_render_phases.get_mut(&view.retained_view_entity)
395        else {
396            continue;
397        };
398
399        let draw_colored_mesh2d = transparent_draw_functions.read().id::<DrawColoredMesh2d>();
400
401        let mesh_key = Mesh2dPipelineKey::from_msaa_samples(msaa.samples())
402            | Mesh2dPipelineKey::from_target_format(view.target_format);
403
404        // Queue all entities visible to that view
405        let Some(visible_entities) = visible_entities.get::<Mesh2d>() else {
406            continue;
407        };
408        for (render_entity, visible_entity) in visible_entities.iter_visible() {
409            if let Some(mesh_instance) = render_mesh_instances.get(visible_entity) {
410                let mesh2d_handle = mesh_instance.mesh_asset_id;
411                let mesh2d_transforms = &mesh_instance.transforms;
412                // Get our specialized pipeline
413                let mut mesh2d_key = mesh_key;
414                let Some(mesh) = render_meshes.get(mesh2d_handle) else {
415                    continue;
416                };
417                mesh2d_key |= Mesh2dPipelineKey::from_primitive_topology_and_strip_index(
418                    mesh.primitive_topology(),
419                    mesh.index_format(),
420                );
421
422                let pipeline_id =
423                    pipelines.specialize(&pipeline_cache, &colored_mesh2d_pipeline, mesh2d_key);
424
425                let mesh_z = mesh2d_transforms.world_from_local.translation.z;
426                transparent_phase.add_retained(Transparent2d {
427                    entity: (*render_entity, *visible_entity),
428                    draw_function: draw_colored_mesh2d,
429                    pipeline: pipeline_id,
430                    // The 2d render items are sorted according to their z value before rendering,
431                    // in order to get correct transparency
432                    sort_key: FloatOrd(mesh_z),
433                    // This material is not batched
434                    batch_range: 0..1,
435                    extra_index: PhaseItemExtraIndex::None,
436                    extracted_index: usize::MAX,
437                    indexed: mesh.indexed(),
438                });
439            }
440        }
441    }
442}
```

[Source](https://docs.rs/bevy_platform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_platform/collections/hash_map.rs.html#632)

#### pub fn [drain](#method.drain)(&mut self) -> [Drain](../platform/collections/hash_map/struct.Drain.html "struct bevy::platform::collections::hash_map::Drain")<'\_, K, V> [ⓘ](#)

Clears the map, returning all key-value pairs as an iterator. Keeps the allocated memory for reuse.

Refer to [`drain`](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/hashbrown/map/struct.HashMap.html#method.drain "method hashbrown::map::HashMap::drain") for further details.

##### Examples

```rust
let mut map = HashMap::new();

map.insert("foo", 0);
map.insert("bar", 1);
map.insert("baz", 2);

for (key, value) in map.drain() {
    // ("foo", 0), ("bar", 1), ("baz", 2)
    // Note that the above order is not guaranteed
}

assert!(map.is_empty());
```

[Source](https://docs.rs/bevy_platform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_platform/collections/hash_map.rs.html#657-659)

#### pub fn [retain](#method.retain)<F>(&mut self, f: F)

where F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")([&K](https://doc.rust-lang.org/nightly/std/primitive.reference.html), [&mut V](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html),

Retains only the elements specified by the predicate. Keeps the allocated memory for reuse.

Refer to [`retain`](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/hashbrown/map/struct.HashMap.html#method.retain "method hashbrown::map::HashMap::retain") for further details.

##### Examples

```rust
let mut map = HashMap::new();

map.insert("foo", 0);
map.insert("bar", 1);
map.insert("baz", 2);

map.retain(|key, value| *value == 2);

assert_eq!(map.len(), 1);
```

##### [Examples found in repository](#scraped-examples-5)[?](../../scrape-examples-help.html)

examples/shader\_advanced/custom\_render\_phase.rs ([line 518](../../src/custom_render_phase/custom_render_phase.rs.html#518))

```rust
500fn extract_camera_phases(
501    mut stencil_phases: ResMut<ViewSortedRenderPhases<Stencil3d>>,
502    cameras: Extract<Query<(Entity, &Camera), With<Camera3d>>>,
503    mut live_entities: Local<HashSet<RetainedViewEntity>>,
504) {
505    live_entities.clear();
506    for (main_entity, camera) in &cameras {
507        if !camera.is_active {
508            continue;
509        }
510        // This is the main camera, so we use the first subview index (0)
511        let retained_view_entity = RetainedViewEntity::new(main_entity.into(), None, 0);
512
513        stencil_phases.prepare_for_new_frame(retained_view_entity);
514        live_entities.insert(retained_view_entity);
515    }
516
517    // Clear out all dead views.
518    stencil_phases.retain(|camera_entity, _| live_entities.contains(camera_entity));
519}
```

[Source](https://docs.rs/bevy_platform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_platform/collections/hash_map.rs.html#688-690)

#### pub fn [extract\_if](#method.extract_if)<F>(&mut self, f: F) -> [ExtractIf](../platform/collections/hash_map/struct.ExtractIf.html "struct bevy::platform::collections::hash_map::ExtractIf")<'\_, K, V, F> [ⓘ](#)

where F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")([&K](https://doc.rust-lang.org/nightly/std/primitive.reference.html), [&mut V](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html),

Drains elements which are true under the given predicate, and returns an iterator over the removed items.

Refer to [`extract_if`](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/hashbrown/map/struct.HashMap.html#method.extract_if "method hashbrown::map::HashMap::extract_if") for further details.

##### Examples

```rust
let mut map = HashMap::new();

map.insert("foo", 0);
map.insert("bar", 1);
map.insert("baz", 2);

let extracted = map
    .extract_if(|key, value| *value == 2)
    .collect::<Vec<_>>();

assert_eq!(map.len(), 2);
assert_eq!(extracted.len(), 1);
```

[Source](https://docs.rs/bevy_platform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_platform/collections/hash_map.rs.html#716)

#### pub fn [clear](#method.clear)(&mut self)

Clears the map, removing all key-value pairs. Keeps the allocated memory for reuse.

Refer to [`clear`](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/hashbrown/map/struct.HashMap.html#method.clear "method hashbrown::map::HashMap::clear") for further details.

##### Examples

```rust
let mut map = HashMap::new();

map.insert("foo", 0);
map.insert("bar", 1);
map.insert("baz", 2);

map.clear();

assert!(map.is_empty());
```

[Source](https://docs.rs/bevy_platform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_platform/collections/hash_map.rs.html#815)

#### pub fn [reserve](#method.reserve)(&mut self, additional: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html))

Reserves capacity for at least `additional` more elements to be inserted in the [`HashMap`](../platform/collections/struct.HashMap.html "struct bevy::platform::collections::HashMap"). The collection may reserve more space to avoid frequent reallocations.

Refer to [`reserve`](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/hashbrown/map/struct.HashMap.html#method.reserve "method hashbrown::map::HashMap::reserve") for further details.

##### Examples

```rust
let mut map = HashMap::with_capacity(5);

assert!(map.capacity() >= 5);

map.reserve(10);

assert!(map.capacity() - map.len() >= 10);
```

[Source](https://docs.rs/bevy_platform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_platform/collections/hash_map.rs.html#840)

#### pub fn [try\_reserve](#method.try_reserve)(&mut self, additional: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [TryReserveError](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/hashbrown/enum.TryReserveError.html "enum hashbrown::TryReserveError")\>

Tries to reserve capacity for at least `additional` more elements to be inserted in the given `HashMap<K,V>`. The collection may reserve more space to avoid frequent reallocations.

Refer to [`try_reserve`](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/hashbrown/map/struct.HashMap.html#method.try_reserve "method hashbrown::map::HashMap::try_reserve") for further details.

##### Examples

```rust
let mut map = HashMap::with_capacity(5);

assert!(map.capacity() >= 5);

map.try_reserve(10).expect("Out of Memory!");

assert!(map.capacity() - map.len() >= 10);
```

[Source](https://docs.rs/bevy_platform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_platform/collections/hash_map.rs.html#867)

#### pub fn [shrink\_to\_fit](#method.shrink_to_fit)(&mut self)

Shrinks the capacity of the map as much as possible. It will drop down as much as possible while maintaining the internal rules and possibly leaving some space in accordance with the resize policy.

Refer to [`shrink_to_fit`](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/hashbrown/map/struct.HashMap.html#method.shrink_to_fit "method hashbrown::map::HashMap::shrink_to_fit") for further details.

##### Examples

```rust
let mut map = HashMap::with_capacity(5);

map.insert("foo", 0);
map.insert("bar", 1);
map.insert("baz", 2);

assert!(map.capacity() >= 5);

map.shrink_to_fit();

assert_eq!(map.capacity(), 3);
```

[Source](https://docs.rs/bevy_platform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_platform/collections/hash_map.rs.html#877)

#### pub fn [shrink\_to](#method.shrink_to)(&mut self, min\_capacity: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html))

Shrinks the capacity of the map with a lower limit. It will drop down no lower than the supplied limit while maintaining the internal rules and possibly leaving some space in accordance with the resize policy.

Refer to [`shrink_to`](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/hashbrown/map/struct.HashMap.html#method.shrink_to "method hashbrown::map::HashMap::shrink_to") for further details.

[Source](https://docs.rs/bevy_platform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_platform/collections/hash_map.rs.html#896)

#### pub fn [entry](#method.entry)(&mut self, key: K) -> [Entry](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/hashbrown/map/enum.Entry.html "enum hashbrown::map::Entry")<'\_, K, V, S>

Gets the given key’s corresponding entry in the map for in-place manipulation.

Refer to [`entry`](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/hashbrown/map/struct.HashMap.html#method.entry "method hashbrown::map::HashMap::entry") for further details.

##### Examples

```rust
let mut map = HashMap::new();

let value = map.entry("foo").or_insert(0);
```

##### [Examples found in repository](#scraped-examples-6)[?](../../scrape-examples-help.html)

examples/ecs/observers.rs ([line 148](../../src/observers/observers.rs.html#148))

```rust
142fn on_add_mine(add: On<Add, Mine>, query: Query<&Mine>, mut index: ResMut<SpatialIndex>) {
143    let mine = query.get(add.entity).unwrap();
144    let tile = (
145        (mine.pos.x / CELL_SIZE).floor() as i32,
146        (mine.pos.y / CELL_SIZE).floor() as i32,
147    );
148    index.map.entry(tile).or_default().insert(add.entity);
149}
150
151// Remove despawned mines from our index
152fn on_remove_mine(remove: On<Remove, Mine>, query: Query<&Mine>, mut index: ResMut<SpatialIndex>) {
153    let mine = query.get(remove.entity).unwrap();
154    let tile = (
155        (mine.pos.x / CELL_SIZE).floor() as i32,
156        (mine.pos.y / CELL_SIZE).floor() as i32,
157    );
158    index.map.entry(tile).and_modify(|set| {
159        set.remove(&remove.entity);
160    });
161}
```

Hide additional examples

examples/shader\_advanced/manual\_material.rs ([line 184](../../src/manual_material/manual_material.rs.html#184))

```rust
145    fn prepare_asset(
146        source_asset: Self::SourceAsset,
147        asset_id: AssetId<Self::SourceAsset>,
148        (
149            opaque_draw_functions,
150            material_layout,
151            asset_server,
152            bind_group_allocators,
153            render_material_bindings,
154            gpu_images,
155            image_material_sampler,
156        ): &mut SystemParamItem<Self::Param>,
157    ) -> std::result::Result<Self::ErasedAsset, PrepareAssetError<Self::SourceAsset>> {
158        let material_layout = material_layout.0.clone();
159        let draw_function_id = opaque_draw_functions.read().id::<DrawMaterial>();
160        let bind_group_allocator = bind_group_allocators
161            .get_mut(&TypeId::of::<ImageMaterial>())
162            .unwrap();
163        let Some(image) = gpu_images.get(&source_asset.image) else {
164            return Err(PrepareAssetError::RetryNextUpdate(source_asset));
165        };
166        let unprepared = UnpreparedBindGroup {
167            bindings: BindingResources(vec![
168                (
169                    0,
170                    OwnedBindingResource::TextureView(
171                        TextureViewDimension::D2,
172                        image.texture_view.clone(),
173                    ),
174                ),
175                (
176                    1,
177                    OwnedBindingResource::Sampler(
178                        SamplerBindingType::NonFiltering,
179                        image_material_sampler.0.clone(),
180                    ),
181                ),
182            ]),
183        };
184        let binding = match render_material_bindings.entry(asset_id.into()) {
185            Entry::Occupied(mut occupied_entry) => {
186                bind_group_allocator.free(*occupied_entry.get());
187                let new_binding =
188                    bind_group_allocator.allocate_unprepared(unprepared, &material_layout);
189                *occupied_entry.get_mut() = new_binding;
190                new_binding
191            }
192            Entry::Vacant(vacant_entry) => *vacant_entry
193                .insert(bind_group_allocator.allocate_unprepared(unprepared, &material_layout)),
194        };
195
196        let mut properties = MaterialProperties {
197            material_layout: Some(material_layout),
198            mesh_pipeline_key_bits: ErasedMeshPipelineKey::new(MeshPipelineKey::empty()),
199            base_specialize: Some(base_specialize),
200            ..Default::default()
201        };
202        properties.add_draw_function(MainPassOpaqueDrawFunction, draw_function_id);
203        properties.add_shader(MaterialFragmentShader, asset_server.load(SHADER_ASSET_PATH));
204
205        Ok(PreparedMaterial {
206            binding,
207            properties: Arc::new(properties),
208        })
209    }
```

[Source](https://docs.rs/bevy_platform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_platform/collections/hash_map.rs.html#916-918)

#### pub fn [entry\_ref](#method.entry_ref)<'a, 'b, Q>( &'a mut self, key: [&'b Q](https://doc.rust-lang.org/nightly/std/primitive.reference.html), ) -> [EntryRef](../platform/collections/hash_map/enum.EntryRef.html "enum bevy::platform::collections::hash_map::EntryRef")<'a, 'b, K, Q, V, S>

where Q: [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash") + [Equivalent](../platform/collections/trait.Equivalent.html "trait bevy::platform::collections::Equivalent")<K> + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Gets the given key’s corresponding entry by reference in the map for in-place manipulation.

Refer to [`entry_ref`](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/hashbrown/map/struct.HashMap.html#method.entry_ref "method hashbrown::map::HashMap::entry_ref") for further details.

##### Examples

```rust
let mut map = HashMap::new();

let value = map.entry_ref("foo").or_insert(0);
```

[Source](https://docs.rs/bevy_platform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_platform/collections/hash_map.rs.html#938-940)

#### pub fn [get](#method.get)<Q>(&self, k: [&Q](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[&V](https://doc.rust-lang.org/nightly/std/primitive.reference.html)\>

where Q: [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash") + [Equivalent](../platform/collections/trait.Equivalent.html "trait bevy::platform::collections::Equivalent")<K> + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Returns a reference to the value corresponding to the key.

Refer to [`get`](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/hashbrown/map/struct.HashMap.html#method.get "method hashbrown::map::HashMap::get") for further details.

##### Examples

```rust
let mut map = HashMap::new();

map.insert("foo", 0);

assert_eq!(map.get("foo"), Some(&0));
```

##### [Examples found in repository](#scraped-examples-7)[?](../../scrape-examples-help.html)

examples/ecs/immutable\_components.rs ([line 69](../../src/immutable_components/immutable_components.rs.html#69))

```rust
68    fn get_entity(&self, name: &'static str) -> Option<Entity> {
69        self.name_to_entity.get(&Name(name)).copied()
70    }
```

Hide additional examples

examples/gltf/gltf\_extension\_animation\_graph.rs ([line 240](../../src/gltf_extension_animation_graph/gltf_extension_animation_graph.rs.html#240))

```rust
231    fn on_animations_collected(
232        &mut self,
233        _load_context: &mut LoadContext<'_>,
234        _animations: &[Handle<AnimationClip>],
235        named_animations: &HashMap<Box<str>, Handle<AnimationClip>>,
236        animation_roots: &HashSet<usize>,
237    ) {
238        self.animation_root_indices = animation_roots.clone();
239
240        if let Some(handle) = named_animations.get("Run") {
241            self.clip = Some(handle.clone());
242        }
243    }
```

examples/ecs/observers.rs ([line 229](../../src/observers/observers.rs.html#229))

```rust
221    fn get_nearby(&self, pos: Vec2) -> Vec<Entity> {
222        let tile = (
223            (pos.x / CELL_SIZE).floor() as i32,
224            (pos.y / CELL_SIZE).floor() as i32,
225        );
226        let mut nearby = Vec::new();
227        for x in -1..2 {
228            for y in -1..2 {
229                if let Some(mines) = self.map.get(&(tile.0 + x, tile.1 + y)) {
230                    nearby.extend(mines.iter());
231                }
232            }
233        }
234        nearby
235    }
```

tests/3d/test\_skinned\_mesh\_bounds.rs ([line 81](../../src/test_skinned_mesh_bounds/test_skinned_mesh_bounds.rs.html#81))

```rust
72fn spawn_scene(
73    mut commands: Commands,
74    query: Query<(Entity, &PendingScene)>,
75    assets: Res<Assets<Gltf>>,
76    mut graphs: ResMut<Assets<AnimationGraph>>,
77) {
78    for (entity, PendingScene(asset)) in query.iter() {
79        if let Some(gltf) = assets.get(asset)
80            && let Some(scene_handle) = gltf.scenes.first()
81            && let Some(animation_handle) = gltf.named_animations.get("Run")
82        {
83            let (graph, graph_node_index) = AnimationGraph::from_clip(animation_handle.clone());
84
85            commands
86                .entity(entity)
87                .remove::<PendingScene>()
88                .insert((
89                    WorldAssetRoot(scene_handle.clone()),
90                    PendingAnimation((graphs.add(graph), graph_node_index)),
91                ))
92                .observe(play_animation);
93        }
94    }
95}
```

examples/3d/tonemapping.rs ([line 319](../../src/tonemapping/tonemapping.rs.html#319))

```rust
291fn toggle_tonemapping_method(
292    keys: Res<ButtonInput<KeyCode>>,
293    mut tonemapping: Single<&mut Tonemapping>,
294    mut color_grading: Single<&mut ColorGrading>,
295    per_method_settings: Res<PerMethodSettings>,
296) {
297    if keys.just_pressed(KeyCode::Digit1) {
298        **tonemapping = Tonemapping::None;
299    } else if keys.just_pressed(KeyCode::Digit2) {
300        **tonemapping = Tonemapping::Reinhard;
301    } else if keys.just_pressed(KeyCode::Digit3) {
302        **tonemapping = Tonemapping::ReinhardLuminance;
303    } else if keys.just_pressed(KeyCode::Digit4) {
304        **tonemapping = Tonemapping::AcesFitted;
305    } else if keys.just_pressed(KeyCode::Digit5) {
306        **tonemapping = Tonemapping::AgX;
307    } else if keys.just_pressed(KeyCode::Digit6) {
308        **tonemapping = Tonemapping::SomewhatBoringDisplayTransform;
309    } else if keys.just_pressed(KeyCode::Digit7) {
310        **tonemapping = Tonemapping::TonyMcMapface;
311    } else if keys.just_pressed(KeyCode::Digit8) {
312        **tonemapping = Tonemapping::BlenderFilmic;
313    } else if keys.just_pressed(KeyCode::Digit9) {
314        **tonemapping = Tonemapping::KhronosPbrNeutral;
315    }
316
317    **color_grading = (*per_method_settings
318        .settings
319        .get::<Tonemapping>(&tonemapping)
320        .as_ref()
321        .unwrap())
322    .clone();
323}
```

examples/shader\_advanced/custom\_render\_phase.rs ([line 377](../../src/custom_render_phase/custom_render_phase.rs.html#377))

```rust
363    fn get_batch_data(
364        (mesh_instances, _render_assets, mesh_allocator): &SystemParamItem<Self::Param>,
365        (_entity, main_entity): (Entity, MainEntity),
366    ) -> Option<(
367        Self::BufferData,
368        Option<(Self::BatchSetCompareData, Self::BatchCompareData)>,
369    )> {
370        let RenderMeshInstances::CpuBuilding(ref mesh_instances) = **mesh_instances else {
371            error!(
372                "`get_batch_data` should never be called in GPU mesh uniform \
373                building mode"
374            );
375            return None;
376        };
377        let mesh_instance = mesh_instances.get(&main_entity)?;
378        let first_vertex_index =
379            match mesh_allocator.mesh_vertex_slice(&mesh_instance.mesh_asset_id()) {
380                Some(mesh_vertex_slice) => mesh_vertex_slice.range.start,
381                None => 0,
382            };
383        let mesh_uniform = {
384            let mesh_transforms = &mesh_instance.transforms;
385            let (local_from_world_transpose_a, local_from_world_transpose_b) =
386                mesh_transforms.world_from_local.inverse_transpose_3x3();
387            MeshUniform {
388                world_from_local: mesh_transforms.world_from_local.to_transpose(),
389                previous_world_from_local: mesh_transforms.previous_world_from_local.to_transpose(),
390                lightmap_uv_rect: UVec2::ZERO,
391                local_from_world_transpose_a,
392                local_from_world_transpose_b,
393                flags: mesh_transforms.flags,
394                first_vertex_index,
395                current_skin_index: u32::MAX,
396                material_and_lightmap_bind_group_slot: 0,
397                tag: 0,
398                morph_descriptor_index: u32::MAX,
399            }
400        };
401        Some((mesh_uniform, None))
402    }
403}
404
405impl GetFullBatchData for StencilPipeline {
406    type BufferInputData = MeshInputUniform;
407
408    fn get_index_and_compare_data(
409        (mesh_instances, _, _): &SystemParamItem<Self::Param>,
410        main_entity: MainEntity,
411    ) -> Option<(
412        NonMaxU32,
413        Option<(Self::BatchSetCompareData, Self::BatchCompareData)>,
414    )> {
415        // This should only be called during GPU building.
416        let RenderMeshInstances::GpuBuilding(ref mesh_instances) = **mesh_instances else {
417            error!(
418                "`get_index_and_compare_data` should never be called in CPU mesh uniform building \
419                mode"
420            );
421            return None;
422        };
423        let mesh_instance = mesh_instances.get(&main_entity)?;
424        Some((
425            NonMaxU32::new(mesh_instance.gpu_specific.current_uniform_index())?,
426            mesh_instance
427                .should_batch()
428                .then_some((mesh_instance.mesh_asset_id(), ())),
429        ))
430    }
431
432    fn get_binned_batch_data(
433        (mesh_instances, _render_assets, mesh_allocator): &SystemParamItem<Self::Param>,
434        main_entity: MainEntity,
435    ) -> Option<Self::BufferData> {
436        let RenderMeshInstances::CpuBuilding(ref mesh_instances) = **mesh_instances else {
437            error!(
438                "`get_binned_batch_data` should never be called in GPU mesh uniform building mode"
439            );
440            return None;
441        };
442        let mesh_instance = mesh_instances.get(&main_entity)?;
443        let first_vertex_index =
444            match mesh_allocator.mesh_vertex_slice(&mesh_instance.mesh_asset_id()) {
445                Some(mesh_vertex_slice) => mesh_vertex_slice.range.start,
446                None => 0,
447            };
448
449        Some(MeshUniform::new(
450            &mesh_instance.transforms,
451            first_vertex_index,
452            mesh_instance.material_bindings_index().slot,
453            None,
454            None,
455            None,
456            None,
457        ))
458    }
459
460    fn write_batch_indirect_parameters_metadata(
461        indexed: bool,
462        base_output_index: u32,
463        batch_set_index: Option<NonMaxU32>,
464        indirect_parameters_buffers: &mut UntypedPhaseIndirectParametersBuffers,
465        indirect_parameters_offset: u32,
466    ) {
467        // Note that `IndirectParameters` covers both of these structures, even
468        // though they actually have distinct layouts. See the comment above that
469        // type for more information.
470        let indirect_parameters = IndirectParametersCpuMetadata {
471            base_output_index,
472            batch_set_index: match batch_set_index {
473                None => !0,
474                Some(batch_set_index) => u32::from(batch_set_index),
475            },
476        };
477
478        if indexed {
479            indirect_parameters_buffers
480                .indexed
481                .set(indirect_parameters_offset, indirect_parameters);
482        } else {
483            indirect_parameters_buffers
484                .non_indexed
485                .set(indirect_parameters_offset, indirect_parameters);
486        }
487    }
488
489    fn get_binned_index(
490        _param: &SystemParamItem<Self::Param>,
491        _query_item: MainEntity,
492    ) -> Option<NonMaxU32> {
493        None
494    }
495}
496
497// When defining a phase, we need to extract it from the main world and add it to a resource
498// that will be used by the render world. We need to give that resource all views that will use
499// that phase
500fn extract_camera_phases(
501    mut stencil_phases: ResMut<ViewSortedRenderPhases<Stencil3d>>,
502    cameras: Extract<Query<(Entity, &Camera), With<Camera3d>>>,
503    mut live_entities: Local<HashSet<RetainedViewEntity>>,
504) {
505    live_entities.clear();
506    for (main_entity, camera) in &cameras {
507        if !camera.is_active {
508            continue;
509        }
510        // This is the main camera, so we use the first subview index (0)
511        let retained_view_entity = RetainedViewEntity::new(main_entity.into(), None, 0);
512
513        stencil_phases.prepare_for_new_frame(retained_view_entity);
514        live_entities.insert(retained_view_entity);
515    }
516
517    // Clear out all dead views.
518    stencil_phases.retain(|camera_entity, _| live_entities.contains(camera_entity));
519}
520
521/// A resource that stores meshes that couldn't be specialized yet because their
522/// materials hadn't loaded.
523///
524/// See the documentation for [`PendingQueues`] for more information.
525#[derive(Default, Deref, DerefMut, Resource)]
526struct PendingCustomMeshQueues(pub PendingQueues);
527
528// This is a very important step when writing a custom phase.
529//
530// This system determines which meshes will be added to the phase.
531fn queue_custom_meshes(
532    custom_draw_functions: Res<DrawFunctions<Stencil3d>>,
533    mut pipelines: ResMut<SpecializedMeshPipelines<StencilPipeline>>,
534    pipeline_cache: Res<PipelineCache>,
535    custom_draw_pipeline: Res<StencilPipeline>,
536    render_meshes: Res<RenderAssets<RenderMesh>>,
537    render_mesh_instances: Res<RenderMeshInstances>,
538    maybe_batched_instance_buffers: Option<
539        Res<BatchedInstanceBuffers<MeshUniform, MeshInputUniform>>,
540    >,
541    mut custom_render_phases: ResMut<ViewSortedRenderPhases<Stencil3d>>,
542    mut views: Query<(&ExtractedView, &RenderVisibleEntities)>,
543    view_key_cache: Res<ViewKeyCache>,
544    dirty_specializations: Res<DirtySpecializations>,
545    mut pending_custom_mesh_queues: ResMut<PendingCustomMeshQueues>,
546    has_marker: Query<(), With<DrawStencil>>,
547) {
548    for (view, visible_entities) in &mut views {
549        let Some(custom_phase) = custom_render_phases.get_mut(&view.retained_view_entity) else {
550            continue;
551        };
552        let draw_custom = custom_draw_functions.read().id::<DrawMesh3dStencil>();
553
554        let Some(&view_key) = view_key_cache.get(&view.retained_view_entity) else {
555            continue;
556        };
557
558        // Since our phase can work on any 3d mesh we can reuse the default mesh 3d filter
559        let Some(render_visible_mesh_entities) = visible_entities.get::<Mesh3d>() else {
560            continue;
561        };
562
563        let view_pending_custom_mesh_queues =
564            pending_custom_mesh_queues.prepare_for_new_frame(view.retained_view_entity);
565
566        // First, remove meshes that need to be respecialized, and those that were removed, from the bins.
567        for &main_entity in dirty_specializations
568            .iter_to_dequeue(view.retained_view_entity, render_visible_mesh_entities)
569        {
570            custom_phase.remove(Entity::PLACEHOLDER, main_entity);
571        }
572
573        for (render_entity, visible_entity) in dirty_specializations.iter_to_queue(
574            view.retained_view_entity,
575            render_visible_mesh_entities,
576            &view_pending_custom_mesh_queues.prev_frame,
577        ) {
578            // We only want meshes with the marker component to be queued to our phase.
579            if has_marker.get(*render_entity).is_err() {
580                continue;
581            }
582            let Some(mesh_instance) = render_mesh_instances.render_mesh_queue_data(*visible_entity)
583            else {
584                // We couldn't fetch the mesh, probably because it hasn't been
585                // loaded yet. Add the entity to the list of pending custom mesh
586                // queues and bail.
587                view_pending_custom_mesh_queues
588                    .current_frame
589                    .insert((*render_entity, *visible_entity));
590                continue;
591            };
592            let Some(mesh) = render_meshes.get(mesh_instance.mesh_asset_id()) else {
593                continue;
594            };
595
596            // Specialize the key for the current mesh entity
597            // For this example we only specialize based on the mesh topology
598            // but you could have more complex keys and that's where you'd need to create those keys
599            let mut mesh_key = view_key;
600            mesh_key |= MeshPipelineKey::from_primitive_topology_and_strip_index(
601                mesh.primitive_topology(),
602                mesh.index_format(),
603            );
604
605            let pipeline_id = pipelines.specialize(
606                &pipeline_cache,
607                &custom_draw_pipeline,
608                mesh_key,
609                &mesh.layout,
610            );
611            let pipeline_id = match pipeline_id {
612                Ok(id) => id,
613                Err(err) => {
614                    error!("{}", err);
615                    continue;
616                }
617            };
618            // At this point we have all the data we need to create a phase item and add it to our
619            // phase
620            custom_phase.add_retained(Stencil3d {
621                sorting_info: TransparentSortingInfo3d::Sorted {
622                    mesh_center: pbr::get_mesh_instance_world_from_local(
623                        *visible_entity,
624                        mesh_instance.current_uniform_index,
625                        &render_mesh_instances,
626                        maybe_batched_instance_buffers.as_deref(),
627                    )
628                    .transform_point3(
629                        render_meshes
630                            .get(mesh_instance.mesh_asset_id())
631                            .unwrap()
632                            .aabb_center,
633                    ),
634                    depth_bias: 0.0,
635                },
636                distance: FloatOrd(0.0),
637                entity: (Entity::PLACEHOLDER, *visible_entity),
638                pipeline: pipeline_id,
639                draw_function: draw_custom,
640                // Sorted phase items aren't batched
641                batch_range: 0..1,
642                extra_index: PhaseItemExtraIndex::None,
643                indexed: mesh.indexed(),
644            });
645        }
646    }
647}
648
649fn custom_draw_system(
650    world: &World,
651    view: ViewQuery<(
652        &ExtractedCamera,
653        &ExtractedView,
654        &ViewTarget,
655        Option<&MainPassResolutionOverride>,
656    )>,
657    stencil_phases: Res<ViewSortedRenderPhases<Stencil3d>>,
658    mut ctx: RenderContext,
659) {
660    let view_entity = view.entity();
661    let (camera, extracted_view, target, resolution_override) = view.into_inner();
662
663    let Some(stencil_phase) = stencil_phases.get(&extracted_view.retained_view_entity) else {
664        return;
665    };
666
667    let mut render_pass = ctx.begin_tracked_render_pass(RenderPassDescriptor {
668        label: Some("stencil pass"),
669        // For the purpose of the example, we will write directly to the view target. A real
670        // stencil pass would write to a custom texture and that texture would be used in later
671        // passes to render custom effects using it.
672        color_attachments: &[Some(target.get_color_attachment())],
673        // We don't bind any depth buffer for this pass
674        depth_stencil_attachment: None,
675        timestamp_writes: None,
676        occlusion_query_set: None,
677        multiview_mask: None,
678    });
679
680    if let Some(viewport) =
681        Viewport::from_viewport_and_override(camera.viewport.as_ref(), resolution_override)
682    {
683        render_pass.set_camera_viewport(&viewport);
684    }
685
686    if let Err(err) = stencil_phase.render(&mut render_pass, world, view_entity) {
687        error!("Error encountered while rendering the stencil phase {err:?}");
688    }
689}
```

Additional examples can be found in:  

*   [examples/shader\_advanced/custom\_shader\_instancing.rs](../../src/custom_shader_instancing/custom_shader_instancing.rs.html#160)
*   [examples/2d/mesh2d\_manual.rs](../../src/mesh2d_manual/mesh2d_manual.rs.html#409)
*   [examples/shader\_advanced/specialized\_mesh\_pipeline.rs](../../src/specialized_mesh_pipeline/specialized_mesh_pipeline.rs.html#310)

[Source](https://docs.rs/bevy_platform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_platform/collections/hash_map.rs.html#960-962)

#### pub fn [get\_key\_value](#method.get_key_value)<Q>(&self, k: [&Q](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<([&K](https://doc.rust-lang.org/nightly/std/primitive.reference.html), [&V](https://doc.rust-lang.org/nightly/std/primitive.reference.html))>

where Q: [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash") + [Equivalent](../platform/collections/trait.Equivalent.html "trait bevy::platform::collections::Equivalent")<K> + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Returns the key-value pair corresponding to the supplied key.

Refer to [`get_key_value`](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/hashbrown/map/struct.HashMap.html#method.get_key_value "method hashbrown::map::HashMap::get_key_value") for further details.

##### Examples

```rust
let mut map = HashMap::new();

map.insert("foo", 0);

assert_eq!(map.get_key_value("foo"), Some((&"foo", &0)));
```

[Source](https://docs.rs/bevy_platform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_platform/collections/hash_map.rs.html#982-984)

#### pub fn [get\_key\_value\_mut](#method.get_key_value_mut)<Q>(&mut self, k: [&Q](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<([&K](https://doc.rust-lang.org/nightly/std/primitive.reference.html), [&mut V](https://doc.rust-lang.org/nightly/std/primitive.reference.html))>

where Q: [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash") + [Equivalent](../platform/collections/trait.Equivalent.html "trait bevy::platform::collections::Equivalent")<K> + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Returns the key-value pair corresponding to the supplied key, with a mutable reference to value.

Refer to [`get_key_value_mut`](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/hashbrown/map/struct.HashMap.html#method.get_key_value_mut "method hashbrown::map::HashMap::get_key_value_mut") for further details.

##### Examples

```rust
let mut map = HashMap::new();

map.insert("foo", 0);

assert_eq!(map.get_key_value_mut("foo"), Some((&"foo", &mut 0)));
```

[Source](https://docs.rs/bevy_platform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_platform/collections/hash_map.rs.html#1004-1006)

#### pub fn [contains\_key](#method.contains_key)<Q>(&self, k: [&Q](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

where Q: [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash") + [Equivalent](../platform/collections/trait.Equivalent.html "trait bevy::platform::collections::Equivalent")<K> + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Returns `true` if the map contains a value for the specified key.

Refer to [`contains_key`](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/hashbrown/map/struct.HashMap.html#method.contains_key "method hashbrown::map::HashMap::contains_key") for further details.

##### Examples

```rust
let mut map = HashMap::new();

map.insert("foo", 0);

assert!(map.contains_key("foo"));
```

[Source](https://docs.rs/bevy_platform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_platform/collections/hash_map.rs.html#1026-1028)

#### pub fn [get\_mut](#method.get_mut)<Q>(&mut self, k: [&Q](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[&mut V](https://doc.rust-lang.org/nightly/std/primitive.reference.html)\>

where Q: [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash") + [Equivalent](../platform/collections/trait.Equivalent.html "trait bevy::platform::collections::Equivalent")<K> + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Returns a mutable reference to the value corresponding to the key.

Refer to [`get_mut`](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/hashbrown/map/struct.HashMap.html#method.get_mut "method hashbrown::map::HashMap::get_mut") for further details.

##### Examples

```rust
let mut map = HashMap::new();

map.insert("foo", 0);

assert_eq!(map.get_mut("foo"), Some(&mut 0));
```

##### [Examples found in repository](#scraped-examples-8)[?](../../scrape-examples-help.html)

examples/3d/tonemapping.rs ([line 348](../../src/tonemapping/tonemapping.rs.html#348))

```rust
340fn update_color_grading_settings(
341    keys: Res<ButtonInput<KeyCode>>,
342    time: Res<Time>,
343    mut per_method_settings: ResMut<PerMethodSettings>,
344    tonemapping: Single<&Tonemapping>,
345    current_scene: Res<CurrentScene>,
346    mut selected_parameter: ResMut<SelectedParameter>,
347) {
348    let color_grading = per_method_settings.settings.get_mut(*tonemapping).unwrap();
349    let mut dt = time.delta_secs() * 0.25;
350    if keys.pressed(KeyCode::ArrowLeft) {
351        dt = -dt;
352    }
353
354    if keys.just_pressed(KeyCode::ArrowDown) {
355        selected_parameter.next();
356    }
357    if keys.just_pressed(KeyCode::ArrowUp) {
358        selected_parameter.prev();
359    }
360    if keys.pressed(KeyCode::ArrowLeft) || keys.pressed(KeyCode::ArrowRight) {
361        match selected_parameter.value {
362            0 => {
363                color_grading.global.exposure += dt;
364            }
365            1 => {
366                color_grading
367                    .all_sections_mut()
368                    .for_each(|section| section.gamma += dt);
369            }
370            2 => {
371                color_grading
372                    .all_sections_mut()
373                    .for_each(|section| section.saturation += dt);
374            }
375            3 => {
376                color_grading.global.post_saturation += dt;
377            }
378            _ => {}
379        }
380    }
381
382    if keys.just_pressed(KeyCode::Space) {
383        for (_, grading) in per_method_settings.settings.iter_mut() {
384            *grading = ColorGrading::default();
385        }
386    }
387
388    if keys.just_pressed(KeyCode::Enter) && current_scene.0 == 1 {
389        for (mapper, grading) in per_method_settings.settings.iter_mut() {
390            *grading = PerMethodSettings::basic_scene_recommendation(*mapper);
391        }
392    }
393}
```

Hide additional examples

examples/shader\_advanced/custom\_shader\_instancing.rs ([line 155](../../src/custom_shader_instancing/custom_shader_instancing.rs.html#155))

```rust
137fn queue_custom(
138    transparent_3d_draw_functions: Res<DrawFunctions<Transparent3d>>,
139    custom_pipeline: Res<CustomPipeline>,
140    mut pipelines: ResMut<SpecializedMeshPipelines<CustomPipeline>>,
141    pipeline_cache: Res<PipelineCache>,
142    meshes: Res<RenderAssets<RenderMesh>>,
143    render_mesh_instances: Res<RenderMeshInstances>,
144    maybe_batched_instance_buffers: Option<
145        Res<BatchedInstanceBuffers<MeshUniform, MeshInputUniform>>,
146    >,
147    material_meshes: Query<(Entity, &MainEntity), With<InstanceMaterialData>>,
148    mut transparent_render_phases: ResMut<ViewSortedRenderPhases<Transparent3d>>,
149    views: Query<&ExtractedView>,
150    view_key_cache: Res<ViewKeyCache>,
151) {
152    let draw_custom = transparent_3d_draw_functions.read().id::<DrawCustom>();
153
154    for view in &views {
155        let Some(transparent_phase) = transparent_render_phases.get_mut(&view.retained_view_entity)
156        else {
157            continue;
158        };
159
160        let Some(&view_key) = view_key_cache.get(&view.retained_view_entity) else {
161            continue;
162        };
163
164        for (entity, main_entity) in &material_meshes {
165            let Some(mesh_instance) = render_mesh_instances.render_mesh_queue_data(*main_entity)
166            else {
167                continue;
168            };
169            let Some(mesh) = meshes.get(mesh_instance.mesh_asset_id()) else {
170                continue;
171            };
172            let key = view_key
173                | MeshPipelineKey::from_primitive_topology_and_strip_index(
174                    mesh.primitive_topology(),
175                    mesh.index_format(),
176                );
177            let pipeline = pipelines
178                .specialize(&pipeline_cache, &custom_pipeline, key, &mesh.layout)
179                .unwrap();
180            transparent_phase.add_retained(Transparent3d {
181                sorting_info: TransparentSortingInfo3d::Sorted {
182                    mesh_center: pbr::get_mesh_instance_world_from_local(
183                        *main_entity,
184                        mesh_instance.current_uniform_index,
185                        &render_mesh_instances,
186                        maybe_batched_instance_buffers.as_deref(),
187                    )
188                    .transform_point3(
189                        meshes
190                            .get(mesh_instance.mesh_asset_id())
191                            .unwrap()
192                            .aabb_center,
193                    ),
194                    depth_bias: 0.0,
195                },
196                entity: (entity, *main_entity),
197                pipeline,
198                draw_function: draw_custom,
199                distance: 0.0,
200                batch_range: 0..1,
201                extra_index: PhaseItemExtraIndex::None,
202                indexed: true,
203            });
204        }
205    }
206}
```

examples/2d/mesh2d\_manual.rs ([line 394](../../src/mesh2d_manual/mesh2d_manual.rs.html#394))

```rust
379pub fn queue_colored_mesh2d(
380    transparent_draw_functions: Res<DrawFunctions<Transparent2d>>,
381    colored_mesh2d_pipeline: Res<ColoredMesh2dPipeline>,
382    mut pipelines: ResMut<SpecializedRenderPipelines<ColoredMesh2dPipeline>>,
383    pipeline_cache: Res<PipelineCache>,
384    render_meshes: Res<RenderAssets<RenderMesh>>,
385    render_mesh_instances: Res<RenderColoredMesh2dInstances>,
386    mut transparent_render_phases: ResMut<ViewSortedRenderPhases<Transparent2d>>,
387    views: Query<(&RenderVisibleEntities, &ExtractedView, &Msaa)>,
388) {
389    if render_mesh_instances.is_empty() {
390        return;
391    }
392    // Iterate each view (a camera is a view)
393    for (visible_entities, view, msaa) in &views {
394        let Some(transparent_phase) = transparent_render_phases.get_mut(&view.retained_view_entity)
395        else {
396            continue;
397        };
398
399        let draw_colored_mesh2d = transparent_draw_functions.read().id::<DrawColoredMesh2d>();
400
401        let mesh_key = Mesh2dPipelineKey::from_msaa_samples(msaa.samples())
402            | Mesh2dPipelineKey::from_target_format(view.target_format);
403
404        // Queue all entities visible to that view
405        let Some(visible_entities) = visible_entities.get::<Mesh2d>() else {
406            continue;
407        };
408        for (render_entity, visible_entity) in visible_entities.iter_visible() {
409            if let Some(mesh_instance) = render_mesh_instances.get(visible_entity) {
410                let mesh2d_handle = mesh_instance.mesh_asset_id;
411                let mesh2d_transforms = &mesh_instance.transforms;
412                // Get our specialized pipeline
413                let mut mesh2d_key = mesh_key;
414                let Some(mesh) = render_meshes.get(mesh2d_handle) else {
415                    continue;
416                };
417                mesh2d_key |= Mesh2dPipelineKey::from_primitive_topology_and_strip_index(
418                    mesh.primitive_topology(),
419                    mesh.index_format(),
420                );
421
422                let pipeline_id =
423                    pipelines.specialize(&pipeline_cache, &colored_mesh2d_pipeline, mesh2d_key);
424
425                let mesh_z = mesh2d_transforms.world_from_local.translation.z;
426                transparent_phase.add_retained(Transparent2d {
427                    entity: (*render_entity, *visible_entity),
428                    draw_function: draw_colored_mesh2d,
429                    pipeline: pipeline_id,
430                    // The 2d render items are sorted according to their z value before rendering,
431                    // in order to get correct transparency
432                    sort_key: FloatOrd(mesh_z),
433                    // This material is not batched
434                    batch_range: 0..1,
435                    extra_index: PhaseItemExtraIndex::None,
436                    extracted_index: usize::MAX,
437                    indexed: mesh.indexed(),
438                });
439            }
440        }
441    }
442}
```

examples/shader\_advanced/custom\_phase\_item.rs ([line 244](../../src/custom_phase_item/custom_phase_item.rs.html#244))

```rust
227fn queue_custom_phase_item(
228    pipeline_cache: Res<PipelineCache>,
229    mut pipeline: ResMut<CustomPhasePipeline>,
230    mut opaque_render_phases: ResMut<ViewBinnedRenderPhases<Opaque3d>>,
231    opaque_draw_functions: Res<DrawFunctions<Opaque3d>>,
232    views: Query<(&ExtractedView, &RenderVisibleEntities, &Msaa)>,
233    dirty_specializations: Res<DirtySpecializations>,
234    mut pending_custom_phase_item_queues: ResMut<PendingCustomPhaseItemQueues>,
235) {
236    let draw_custom_phase_item = opaque_draw_functions
237        .read()
238        .id::<DrawCustomPhaseItemCommands>();
239
240    // Render phases are per-view, so we need to iterate over all views so that
241    // the entity appears in them. (In this example, we have only one view, but
242    // it's good practice to loop over all views anyway.)
243    for (view, view_visible_entities, msaa) in views.iter() {
244        let Some(opaque_phase) = opaque_render_phases.get_mut(&view.retained_view_entity) else {
245            continue;
246        };
247
248        // Fetch the list of visible entities in the `CustomRenderedEntity`
249        // class. If there are no such entities, then we have no entities to
250        // render, and we're done.
251        let Some(render_visible_mesh_entities) =
252            view_visible_entities.get::<CustomRenderedEntity>()
253        else {
254            continue;
255        };
256
257        let view_pending_custom_phase_item_queues =
258            pending_custom_phase_item_queues.prepare_for_new_frame(view.retained_view_entity);
259
260        // First, remove meshes that need to be respecialized, and those that
261        // were removed, from the bins.
262        for &main_entity in dirty_specializations
263            .iter_to_dequeue(view.retained_view_entity, render_visible_mesh_entities)
264        {
265            opaque_phase.remove(main_entity);
266        }
267
268        // Find all the custom rendered entities that are visible from this
269        // view.
270        for (render_entity, main_entity) in dirty_specializations.iter_to_queue(
271            view.retained_view_entity,
272            render_visible_mesh_entities,
273            &view_pending_custom_phase_item_queues.prev_frame,
274        ) {
275            // Ordinarily, the [`SpecializedRenderPipeline::Key`] would contain
276            // some per-view settings, such as whether the view is HDR, but for
277            // simplicity's sake we simply hard-code the view's characteristics,
278            // with the exception of number of MSAA samples.
279            let Ok(pipeline_id) = pipeline
280                .variants
281                .specialize(&pipeline_cache, CustomPhaseKey(*msaa))
282            else {
283                continue;
284            };
285
286            // Add the custom render item. We use the
287            // [`BinnedRenderPhaseType::NonMesh`] type to skip the special
288            // handling that Bevy has for meshes (preprocessing, indirect
289            // draws, etc.)
290            //
291            // The asset ID is arbitrary; we simply use [`AssetId::invalid`],
292            // but you can use anything you like. Note that the asset ID need
293            // not be the ID of a [`Mesh`].
294            opaque_phase.add(
295                Opaque3dBatchSetKey {
296                    draw_function: draw_custom_phase_item,
297                    pipeline: pipeline_id,
298                    material_bind_group_index: None,
299                    lightmap_slab: None,
300                    slabs: MeshSlabs::default(),
301                },
302                Opaque3dBinKey {
303                    asset_id: AssetId::<Mesh>::invalid().untyped(),
304                },
305                (*render_entity, *main_entity),
306                InputUniformIndex::default(),
307                BinnedRenderPhaseType::NonMesh,
308            );
309        }
310    }
311}
```

examples/shader\_advanced/custom\_render\_phase.rs ([line 549](../../src/custom_render_phase/custom_render_phase.rs.html#549))

```rust
531fn queue_custom_meshes(
532    custom_draw_functions: Res<DrawFunctions<Stencil3d>>,
533    mut pipelines: ResMut<SpecializedMeshPipelines<StencilPipeline>>,
534    pipeline_cache: Res<PipelineCache>,
535    custom_draw_pipeline: Res<StencilPipeline>,
536    render_meshes: Res<RenderAssets<RenderMesh>>,
537    render_mesh_instances: Res<RenderMeshInstances>,
538    maybe_batched_instance_buffers: Option<
539        Res<BatchedInstanceBuffers<MeshUniform, MeshInputUniform>>,
540    >,
541    mut custom_render_phases: ResMut<ViewSortedRenderPhases<Stencil3d>>,
542    mut views: Query<(&ExtractedView, &RenderVisibleEntities)>,
543    view_key_cache: Res<ViewKeyCache>,
544    dirty_specializations: Res<DirtySpecializations>,
545    mut pending_custom_mesh_queues: ResMut<PendingCustomMeshQueues>,
546    has_marker: Query<(), With<DrawStencil>>,
547) {
548    for (view, visible_entities) in &mut views {
549        let Some(custom_phase) = custom_render_phases.get_mut(&view.retained_view_entity) else {
550            continue;
551        };
552        let draw_custom = custom_draw_functions.read().id::<DrawMesh3dStencil>();
553
554        let Some(&view_key) = view_key_cache.get(&view.retained_view_entity) else {
555            continue;
556        };
557
558        // Since our phase can work on any 3d mesh we can reuse the default mesh 3d filter
559        let Some(render_visible_mesh_entities) = visible_entities.get::<Mesh3d>() else {
560            continue;
561        };
562
563        let view_pending_custom_mesh_queues =
564            pending_custom_mesh_queues.prepare_for_new_frame(view.retained_view_entity);
565
566        // First, remove meshes that need to be respecialized, and those that were removed, from the bins.
567        for &main_entity in dirty_specializations
568            .iter_to_dequeue(view.retained_view_entity, render_visible_mesh_entities)
569        {
570            custom_phase.remove(Entity::PLACEHOLDER, main_entity);
571        }
572
573        for (render_entity, visible_entity) in dirty_specializations.iter_to_queue(
574            view.retained_view_entity,
575            render_visible_mesh_entities,
576            &view_pending_custom_mesh_queues.prev_frame,
577        ) {
578            // We only want meshes with the marker component to be queued to our phase.
579            if has_marker.get(*render_entity).is_err() {
580                continue;
581            }
582            let Some(mesh_instance) = render_mesh_instances.render_mesh_queue_data(*visible_entity)
583            else {
584                // We couldn't fetch the mesh, probably because it hasn't been
585                // loaded yet. Add the entity to the list of pending custom mesh
586                // queues and bail.
587                view_pending_custom_mesh_queues
588                    .current_frame
589                    .insert((*render_entity, *visible_entity));
590                continue;
591            };
592            let Some(mesh) = render_meshes.get(mesh_instance.mesh_asset_id()) else {
593                continue;
594            };
595
596            // Specialize the key for the current mesh entity
597            // For this example we only specialize based on the mesh topology
598            // but you could have more complex keys and that's where you'd need to create those keys
599            let mut mesh_key = view_key;
600            mesh_key |= MeshPipelineKey::from_primitive_topology_and_strip_index(
601                mesh.primitive_topology(),
602                mesh.index_format(),
603            );
604
605            let pipeline_id = pipelines.specialize(
606                &pipeline_cache,
607                &custom_draw_pipeline,
608                mesh_key,
609                &mesh.layout,
610            );
611            let pipeline_id = match pipeline_id {
612                Ok(id) => id,
613                Err(err) => {
614                    error!("{}", err);
615                    continue;
616                }
617            };
618            // At this point we have all the data we need to create a phase item and add it to our
619            // phase
620            custom_phase.add_retained(Stencil3d {
621                sorting_info: TransparentSortingInfo3d::Sorted {
622                    mesh_center: pbr::get_mesh_instance_world_from_local(
623                        *visible_entity,
624                        mesh_instance.current_uniform_index,
625                        &render_mesh_instances,
626                        maybe_batched_instance_buffers.as_deref(),
627                    )
628                    .transform_point3(
629                        render_meshes
630                            .get(mesh_instance.mesh_asset_id())
631                            .unwrap()
632                            .aabb_center,
633                    ),
634                    depth_bias: 0.0,
635                },
636                distance: FloatOrd(0.0),
637                entity: (Entity::PLACEHOLDER, *visible_entity),
638                pipeline: pipeline_id,
639                draw_function: draw_custom,
640                // Sorted phase items aren't batched
641                batch_range: 0..1,
642                extra_index: PhaseItemExtraIndex::None,
643                indexed: mesh.indexed(),
644            });
645        }
646    }
647}
```

examples/shader\_advanced/specialized\_mesh\_pipeline.rs ([line 306](../../src/specialized_mesh_pipeline/specialized_mesh_pipeline.rs.html#306))

```rust
277fn queue_custom_mesh_pipeline(
278    pipeline_cache: Res<PipelineCache>,
279    custom_mesh_pipeline: Res<CustomMeshPipeline>,
280    (mut opaque_render_phases, opaque_draw_functions): (
281        ResMut<ViewBinnedRenderPhases<Opaque3d>>,
282        Res<DrawFunctions<Opaque3d>>,
283    ),
284    mut specialized_mesh_pipelines: ResMut<SpecializedMeshPipelines<CustomMeshPipeline>>,
285    views: Query<(&RenderVisibleEntities, &ExtractedView)>,
286    view_key_cache: Res<ViewKeyCache>,
287    (render_meshes, render_mesh_instances): (
288        Res<RenderAssets<RenderMesh>>,
289        Res<RenderMeshInstances>,
290    ),
291    mut change_tick: Local<Tick>,
292    mesh_allocator: Res<MeshAllocator>,
293    gpu_preprocessing_support: Res<GpuPreprocessingSupport>,
294    dirty_specializations: Res<DirtySpecializations>,
295    mut pending_custom_mesh_queues: ResMut<PendingCustomMeshQueues>,
296) {
297    // Get the id for our custom draw function
298    let draw_function = opaque_draw_functions
299        .read()
300        .id::<DrawSpecializedPipelineCommands>();
301
302    // Render phases are per-view, so we need to iterate over all views so that
303    // the entity appears in them. (In this example, we have only one view, but
304    // it's good practice to loop over all views anyway.)
305    for (view_visible_entities, view) in views.iter() {
306        let Some(opaque_phase) = opaque_render_phases.get_mut(&view.retained_view_entity) else {
307            continue;
308        };
309
310        let Some(&view_key) = view_key_cache.get(&view.retained_view_entity) else {
311            continue;
312        };
313
314        let Some(render_visible_mesh_entities) =
315            view_visible_entities.get::<CustomRenderedEntity>()
316        else {
317            continue;
318        };
319
320        // Initialize the pending queues.
321        let view_pending_custom_mesh_queues =
322            pending_custom_mesh_queues.prepare_for_new_frame(view.retained_view_entity);
323
324        // First, remove meshes that need to be respecialized, and those that were removed, from the bins.
325        for &main_entity in dirty_specializations
326            .iter_to_dequeue(view.retained_view_entity, render_visible_mesh_entities)
327        {
328            opaque_phase.remove(main_entity);
329        }
330
331        // Find all the custom rendered entities that are visible from this
332        // view.
333        for (render_entity, visible_entity) in dirty_specializations.iter_to_queue(
334            view.retained_view_entity,
335            render_visible_mesh_entities,
336            &view_pending_custom_mesh_queues.prev_frame,
337        ) {
338            // Get the mesh instance
339            let Some(mesh_instance) = render_mesh_instances.render_mesh_queue_data(*visible_entity)
340            else {
341                // We couldn't fetch the mesh, probably because it hasn't been
342                // loaded yet. Add the entity to the list of pending custom
343                // meshes and bail.
344                view_pending_custom_mesh_queues
345                    .current_frame
346                    .insert((*render_entity, *visible_entity));
347                continue;
348            };
349
350            // Get the mesh data
351            let Some(mesh) = render_meshes.get(mesh_instance.mesh_asset_id()) else {
352                continue;
353            };
354
355            let Some(mesh_slabs) = mesh_allocator.mesh_slabs(&mesh_instance.mesh_asset_id()) else {
356                continue;
357            };
358
359            // Specialize the key for the current mesh entity
360            // For this example we only specialize based on the mesh topology
361            // but you could have more complex keys and that's where you'd need to create those keys
362            let mut mesh_key = view_key;
363            mesh_key |= MeshPipelineKey::from_primitive_topology_and_strip_index(
364                mesh.primitive_topology(),
365                mesh.index_format(),
366            );
367
368            // Finally, we can specialize the pipeline based on the key
369            let pipeline_id = specialized_mesh_pipelines
370                .specialize(
371                    &pipeline_cache,
372                    &custom_mesh_pipeline,
373                    mesh_key,
374                    &mesh.layout,
375                )
376                // This should never happen with this example, but if your pipeline
377                // specialization can fail you need to handle the error here
378                .expect("Failed to specialize mesh pipeline");
379
380            // Bump the change tick so that Bevy is forced to rebuild the bin.
381            let next_change_tick = change_tick.get() + 1;
382            change_tick.set(next_change_tick);
383
384            // Add the mesh with our specialized pipeline
385            opaque_phase.add(
386                Opaque3dBatchSetKey {
387                    draw_function,
388                    pipeline: pipeline_id,
389                    material_bind_group_index: None,
390                    slabs: mesh_slabs,
391                    lightmap_slab: None,
392                },
393                // For this example we can use the mesh asset id as the bin key,
394                // but you can use any asset_id as a key
395                Opaque3dBinKey {
396                    asset_id: mesh_instance.mesh_asset_id().into(),
397                },
398                (*render_entity, *visible_entity),
399                mesh_instance.current_uniform_index,
400                // This example supports batching and multi draw indirect,
401                // but if your pipeline doesn't support it you can use
402                // `BinnedRenderPhaseType::UnbatchableMesh`
403                BinnedRenderPhaseType::mesh(
404                    mesh_instance.should_batch(),
405                    &gpu_preprocessing_support,
406                ),
407            );
408        }
409    }
410}
```

[Source](https://docs.rs/bevy_platform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_platform/collections/hash_map.rs.html#1052-1054)

#### pub fn [get\_disjoint\_mut](#method.get_disjoint_mut)<Q, const N: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>( &mut self, ks: \[[&Q](https://doc.rust-lang.org/nightly/std/primitive.reference.html); [N](https://doc.rust-lang.org/nightly/std/primitive.array.html)\], ) -> \[[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[&mut V](https://doc.rust-lang.org/nightly/std/primitive.reference.html)\>; [N](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]

where Q: [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash") + [Equivalent](../platform/collections/trait.Equivalent.html "trait bevy::platform::collections::Equivalent")<K> + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Attempts to get mutable references to `N` values in the map at once.

Refer to [`get_disjoint_mut`](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/hashbrown/map/struct.HashMap.html#method.get_disjoint_mut "method hashbrown::map::HashMap::get_disjoint_mut") for further details.

##### Examples

```rust
let mut map = HashMap::new();

map.insert("foo", 0);
map.insert("bar", 1);
map.insert("baz", 2);

let result = map.get_disjoint_mut(["foo", "bar"]);

assert_eq!(result, [Some(&mut 0), Some(&mut 1)]);
```

[Source](https://docs.rs/bevy_platform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_platform/collections/hash_map.rs.html#1079-1084)

#### pub fn [get\_disjoint\_key\_value\_mut](#method.get_disjoint_key_value_mut)<Q, const N: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>( &mut self, ks: \[[&Q](https://doc.rust-lang.org/nightly/std/primitive.reference.html); [N](https://doc.rust-lang.org/nightly/std/primitive.array.html)\], ) -> \[[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<([&K](https://doc.rust-lang.org/nightly/std/primitive.reference.html), [&mut V](https://doc.rust-lang.org/nightly/std/primitive.reference.html))>; [N](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]

where Q: [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash") + [Equivalent](../platform/collections/trait.Equivalent.html "trait bevy::platform::collections::Equivalent")<K> + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Attempts to get mutable references to `N` values in the map at once, with immutable references to the corresponding keys.

Refer to [`get_disjoint_key_value_mut`](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/hashbrown/map/struct.HashMap.html#method.get_disjoint_key_value_mut "method hashbrown::map::HashMap::get_disjoint_key_value_mut") for further details.

##### Examples

```rust
let mut map = HashMap::new();

map.insert("foo", 0);
map.insert("bar", 1);
map.insert("baz", 2);

let result = map.get_disjoint_key_value_mut(["foo", "bar"]);

assert_eq!(result, [Some((&"foo", &mut 0)), Some((&"bar", &mut 1))]);
```

[Source](https://docs.rs/bevy_platform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_platform/collections/hash_map.rs.html#1104)

#### pub fn [insert](#method.insert)(&mut self, k: K, v: V) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<V>

Inserts a key-value pair into the map.

Refer to [`insert`](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/hashbrown/map/struct.HashMap.html#method.insert "method hashbrown::map::HashMap::insert") for further details.

##### Examples

```rust
let mut map = HashMap::new();

map.insert("foo", 0);

assert_eq!(map.get("foo"), Some(&0));
```

##### [Examples found in repository](#scraped-examples-9)[?](../../scrape-examples-help.html)

examples/ecs/immutable\_components.rs ([line 85](../../src/immutable_components/immutable_components.rs.html#85))

```rust
77fn on_insert_name(mut world: DeferredWorld<'_>, HookContext { entity, .. }: HookContext) {
78    let Some(&name) = world.entity(entity).get::<Name>() else {
79        unreachable!("Insert hook guarantees `Name` is available on entity")
80    };
81    let Some(mut index) = world.get_resource_mut::<NameIndex>() else {
82        return;
83    };
84
85    index.name_to_entity.insert(name, entity);
86}
```

Hide additional examples

examples/3d/tonemapping.rs ([lines 603-606](../../src/tonemapping/tonemapping.rs.html#603-606))

```rust
589    fn default() -> Self {
590        let mut settings = <HashMap<_, _>>::default();
591
592        for method in [
593            Tonemapping::None,
594            Tonemapping::Reinhard,
595            Tonemapping::ReinhardLuminance,
596            Tonemapping::AcesFitted,
597            Tonemapping::AgX,
598            Tonemapping::SomewhatBoringDisplayTransform,
599            Tonemapping::TonyMcMapface,
600            Tonemapping::BlenderFilmic,
601            Tonemapping::KhronosPbrNeutral,
602        ] {
603            settings.insert(
604                method,
605                PerMethodSettings::basic_scene_recommendation(method),
606            );
607        }
608
609        Self { settings }
610    }
```

tests/ecs/ambiguity\_detection.rs ([line 107](../../src/ambiguity_detection/ambiguity_detection.rs.html#107))

```rust
91fn count_ambiguities(sub_app: &mut SubApp) -> AmbiguitiesCount {
92    let schedule_labels = sub_app
93        .world()
94        .resource::<Schedules>()
95        .iter()
96        .map(|(_, schedule)| schedule.label())
97        .collect::<Vec<_>>();
98    let mut ambiguities = <HashMap<_, _>>::default();
99    for label in schedule_labels {
100        let ambiguities_in_schedule =
101            sub_app
102                .world_mut()
103                .schedule_scope(label, |world, schedule| {
104                    schedule.initialize(world).unwrap().unwrap();
105                    schedule.graph().conflicting_systems().len()
106                });
107        ambiguities.insert(label, ambiguities_in_schedule);
108    }
109    AmbiguitiesCount(ambiguities)
110}
```

examples/shader\_advanced/manual\_material.rs ([lines 255-261](../../src/manual_material/manual_material.rs.html#255-261))

```rust
242fn extract_image_materials(
243    mut material_instances: ResMut<RenderMaterialInstances>,
244    changed_meshes_query: Extract<
245        Query<
246            (Entity, &ViewVisibility, &ImageMaterial3d),
247            Or<(Changed<ViewVisibility>, Changed<ImageMaterial3d>)>,
248        >,
249    >,
250) {
251    let last_change_tick = material_instances.current_change_tick;
252
253    for (entity, view_visibility, material) in &changed_meshes_query {
254        if view_visibility.get() {
255            material_instances.instances.insert(
256                entity.into(),
257                RenderMaterialInstance {
258                    asset_id: material.0.id().untyped(),
259                    last_change_tick,
260                },
261            );
262        } else {
263            material_instances
264                .instances
265                .remove(&MainEntity::from(entity));
266        }
267    }
268}
```

examples/2d/mesh2d\_manual.rs ([lines 363-372](../../src/mesh2d_manual/mesh2d_manual.rs.html#363-372))

```rust
332pub fn extract_colored_mesh2d(
333    mut commands: Commands,
334    mut previous_len: Local<usize>,
335    // When extracting, you must use `Extract` to mark the `SystemParam`s
336    // which should be taken from the main world.
337    query: Extract<
338        Query<
339            (
340                Entity,
341                RenderEntity,
342                &ViewVisibility,
343                &GlobalTransform,
344                &Mesh2d,
345            ),
346            With<ColoredMesh2d>,
347        >,
348    >,
349    mut render_mesh_instances: ResMut<RenderColoredMesh2dInstances>,
350) {
351    let mut values = Vec::with_capacity(*previous_len);
352    for (entity, render_entity, view_visibility, transform, handle) in &query {
353        if !view_visibility.get() {
354            continue;
355        }
356
357        let transforms = Mesh2dTransforms {
358            world_from_local: transform.affine().into(),
359            flags: MeshFlags::empty().bits(),
360        };
361
362        values.push((render_entity, ColoredMesh2d));
363        render_mesh_instances.insert(
364            entity.into(),
365            RenderMesh2dInstance {
366                mesh_asset_id: handle.0.id(),
367                transforms,
368                material_bind_group_id: Material2dBindGroupId::default(),
369                automatic_batching: false,
370                tag: 0,
371            },
372        );
373    }
374    *previous_len = values.len();
375    commands.try_insert_batch(values);
376}
```

examples/reflection/reflection\_types.rs ([line 69](../../src/reflection_types/reflection_types.rs.html#69))

```rust
67fn setup() {
68    let mut z = <HashMap<_, _>>::default();
69    z.insert("Hello".to_string(), 1.0);
70    let value: Box<dyn Reflect> = Box::new(A {
71        x: 1,
72        y: vec![1, 2],
73        z,
74    });
75
76    // There are a number of different "reflect traits", which each expose different operations on
77    // the underlying type
78    match value.reflect_ref() {
79        // `Struct` is a trait automatically implemented for structs that derive Reflect. This trait
80        // allows you to interact with fields via their string names or indices
81        ReflectRef::Struct(value) => {
82            info!(
83                "This is a 'struct' type with an 'x' value of {}",
84                value.get_field::<usize>("x").unwrap()
85            );
86        }
87        // `TupleStruct` is a trait automatically implemented for tuple structs that derive Reflect.
88        // This trait allows you to interact with fields via their indices
89        ReflectRef::TupleStruct(_) => {}
90        // `Tuple` is a special trait that can be manually implemented (instead of deriving
91        // Reflect). This exposes "tuple" operations on your type, allowing you to interact
92        // with fields via their indices. Tuple is automatically implemented for tuples of
93        // arity 12 or less.
94        ReflectRef::Tuple(_) => {}
95        // `Enum` is a trait automatically implemented for enums that derive Reflect. This trait allows you
96        // to interact with the current variant and its fields (if it has any)
97        ReflectRef::Enum(_) => {}
98        // `List` is a special trait that can be manually implemented (instead of deriving Reflect).
99        // This exposes "list" operations on your type, such as insertion. `List` is automatically
100        // implemented for relevant core types like Vec<T>.
101        ReflectRef::List(_) => {}
102        // `Array` is a special trait that can be manually implemented (instead of deriving Reflect).
103        // This exposes "array" operations on your type, such as indexing. `Array`
104        // is automatically implemented for relevant core types like [T; N].
105        ReflectRef::Array(_) => {}
106        // `Map` is a special trait that can be manually implemented (instead of deriving Reflect).
107        // This exposes "map" operations on your type, such as getting / inserting by key.
108        // Map is automatically implemented for relevant core types like HashMap<K, V>
109        ReflectRef::Map(_) => {}
110        // `Set` is a special trait that can be manually implemented (instead of deriving Reflect).
111        // This exposes "set" operations on your type, such as getting / inserting by value.
112        // Set is automatically implemented for relevant core types like HashSet<T>
113        ReflectRef::Set(_) => {}
114        // `Function` is a special trait that can be manually implemented (instead of deriving Reflect).
115        // This exposes "function" operations on your type, such as calling it with arguments.
116        // This trait is automatically implemented for types like DynamicFunction.
117        // This variant only exists if the `reflect_functions` feature is enabled.
118        #[cfg(feature = "reflect_functions")]
119        ReflectRef::Function(_) => {}
120        // `Opaque` types do not implement any of the other traits above. They are simply a Reflect
121        // implementation. Opaque is implemented for opaque types like String and Instant,
122        // but also include primitive types like i32, usize, and f32 (despite not technically being opaque).
123        ReflectRef::Opaque(_) => {}
124        #[expect(
125            clippy::allow_attributes,
126            reason = "`unreachable_patterns` is not always linted"
127        )]
128        #[allow(
129            unreachable_patterns,
130            reason = "This example cannot always detect when `bevy_reflect/functions` is enabled."
131        )]
132        _ => {}
133    }
134
135    let mut dynamic_list = DynamicList::default();
136    dynamic_list.push(3u32);
137    dynamic_list.push(4u32);
138    dynamic_list.push(5u32);
139
140    let mut value: A = value.take::<A>().unwrap();
141    value.y.apply(&dynamic_list);
142    assert_eq!(value.y, vec![3u32, 4u32, 5u32]);
143
144    // reference types defined above that are only used to demonstrate reflect
145    // derive functionality:
146    _ = || -> (A, B, C, D, E, F) { unreachable!() };
147}
```

[Source](https://docs.rs/bevy_platform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_platform/collections/hash_map.rs.html#1124)

#### pub fn [try\_insert](#method.try_insert)( &mut self, key: K, value: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[&mut V](https://doc.rust-lang.org/nightly/std/primitive.reference.html), [OccupiedError](../platform/collections/hash_map/struct.OccupiedError.html "struct bevy::platform::collections::hash_map::OccupiedError")<'\_, K, V, S>>

Tries to insert a key-value pair into the map, and returns a mutable reference to the value in the entry.

Refer to [`try_insert`](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/hashbrown/map/struct.HashMap.html#method.try_insert "method hashbrown::map::HashMap::try_insert") for further details.

##### Examples

```rust
let mut map = HashMap::new();

map.try_insert("foo", 0).unwrap();

assert!(map.try_insert("foo", 1).is_err());
```

[Source](https://docs.rs/bevy_platform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_platform/collections/hash_map.rs.html#1146-1148)

#### pub fn [remove](#method.remove)<Q>(&mut self, k: [&Q](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<V>

where Q: [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash") + [Equivalent](../platform/collections/trait.Equivalent.html "trait bevy::platform::collections::Equivalent")<K> + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Removes a key from the map, returning the value at the key if the key was previously in the map. Keeps the allocated memory for reuse.

Refer to [`remove`](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/hashbrown/map/struct.HashMap.html#method.remove "method hashbrown::map::HashMap::remove") for further details.

##### Examples

```rust
let mut map = HashMap::new();

map.insert("foo", 0);

assert_eq!(map.remove("foo"), Some(0));

assert!(map.is_empty());
```

##### [Examples found in repository](#scraped-examples-10)[?](../../scrape-examples-help.html)

examples/ecs/immutable\_components.rs ([line 100](../../src/immutable_components/immutable_components.rs.html#100))

```rust
92fn on_discard_name(mut world: DeferredWorld<'_>, HookContext { entity, .. }: HookContext) {
93    let Some(&name) = world.entity(entity).get::<Name>() else {
94        unreachable!("Discard hook guarantees `Name` is available on entity")
95    };
96    let Some(mut index) = world.get_resource_mut::<NameIndex>() else {
97        return;
98    };
99
100    index.name_to_entity.remove(&name);
101}
```

Hide additional examples

examples/shader\_advanced/manual\_material.rs ([line 265](../../src/manual_material/manual_material.rs.html#265))

```rust
242fn extract_image_materials(
243    mut material_instances: ResMut<RenderMaterialInstances>,
244    changed_meshes_query: Extract<
245        Query<
246            (Entity, &ViewVisibility, &ImageMaterial3d),
247            Or<(Changed<ViewVisibility>, Changed<ImageMaterial3d>)>,
248        >,
249    >,
250) {
251    let last_change_tick = material_instances.current_change_tick;
252
253    for (entity, view_visibility, material) in &changed_meshes_query {
254        if view_visibility.get() {
255            material_instances.instances.insert(
256                entity.into(),
257                RenderMaterialInstance {
258                    asset_id: material.0.id().untyped(),
259                    last_change_tick,
260                },
261            );
262        } else {
263            material_instances
264                .instances
265                .remove(&MainEntity::from(entity));
266        }
267    }
268}
```

[Source](https://docs.rs/bevy_platform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_platform/collections/hash_map.rs.html#1171-1173)

#### pub fn [remove\_entry](#method.remove_entry)<Q>(&mut self, k: [&Q](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[(K, V)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)\>

where Q: [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash") + [Equivalent](../platform/collections/trait.Equivalent.html "trait bevy::platform::collections::Equivalent")<K> + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Removes a key from the map, returning the stored key and value if the key was previously in the map. Keeps the allocated memory for reuse.

Refer to [`remove_entry`](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/hashbrown/map/struct.HashMap.html#method.remove_entry "method hashbrown::map::HashMap::remove_entry") for further details.

##### Examples

```rust
let mut map = HashMap::new();

map.insert("foo", 0);

assert_eq!(map.remove_entry("foo"), Some(("foo", 0)));

assert!(map.is_empty());
```

[Source](https://docs.rs/bevy_platform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_platform/collections/hash_map.rs.html#1196)

#### pub fn [allocation\_size](#method.allocation_size)(&self) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

Returns the total amount of memory allocated internally by the hash set, in bytes.

Refer to [`allocation_size`](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/hashbrown/map/struct.HashMap.html#method.allocation_size "method hashbrown::map::HashMap::allocation_size") for further details.

##### Examples

```rust
let mut map = HashMap::new();

assert_eq!(map.allocation_size(), 0);

map.insert("foo", 0u32);

assert!(map.allocation_size() >= size_of::<&'static str>() + size_of::<u32>());
```

[Source](https://docs.rs/bevy_platform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_platform/collections/hash_map.rs.html#1224)

#### pub unsafe fn [insert\_unique\_unchecked](#method.insert_unique_unchecked)( &mut self, key: K, value: V, ) -> ([&K](https://doc.rust-lang.org/nightly/std/primitive.reference.html), [&mut V](https://doc.rust-lang.org/nightly/std/primitive.reference.html))

Insert a key-value pair into the map without checking if the key already exists in the map.

Refer to [`insert_unique_unchecked`](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/hashbrown/map/struct.HashMap.html#method.insert_unique_unchecked "method hashbrown::map::HashMap::insert_unique_unchecked") for further details.

##### Safety

This operation is safe if a key does not exist in the map.

However, if a key exists in the map already, the behavior is unspecified: this operation may panic, loop forever, or any following operation with the map may panic, loop forever or return arbitrary result.

That said, this operation (and following operations) are guaranteed to not violate memory safety.

However this operation is still unsafe because the resulting `HashMap` may be passed to unsafe code which does expect the map to behave correctly, and would cause unsoundness as a result.

[Source](https://docs.rs/bevy_platform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_platform/collections/hash_map.rs.html#1250-1255)

#### pub unsafe fn [get\_disjoint\_unchecked\_mut](#method.get_disjoint_unchecked_mut)<Q, const N: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>( &mut self, keys: \[[&Q](https://doc.rust-lang.org/nightly/std/primitive.reference.html); [N](https://doc.rust-lang.org/nightly/std/primitive.array.html)\], ) -> \[[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[&mut V](https://doc.rust-lang.org/nightly/std/primitive.reference.html)\>; [N](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]

where Q: [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash") + [Equivalent](../platform/collections/trait.Equivalent.html "trait bevy::platform::collections::Equivalent")<K> + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Attempts to get mutable references to `N` values in the map at once, without validating that the values are unique.

Refer to [`get_disjoint_unchecked_mut`](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/hashbrown/map/struct.HashMap.html#method.get_disjoint_unchecked_mut "method hashbrown::map::HashMap::get_disjoint_unchecked_mut") for further details.

Returns an array of length `N` with the results of each query. `None` will be used if the key is missing.

For a safe alternative see [`get_disjoint_mut`](../platform/collections/struct.HashMap.html#method.get_disjoint_mut "method bevy::platform::collections::HashMap::get_disjoint_mut").

##### Safety

Calling this method with overlapping keys is _[undefined behavior](https://doc.rust-lang.org/reference/behavior-considered-undefined.html)_ even if the resulting references are not used.

[Source](https://docs.rs/bevy_platform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_platform/collections/hash_map.rs.html#1282-1287)

#### pub unsafe fn [get\_disjoint\_key\_value\_unchecked\_mut](#method.get_disjoint_key_value_unchecked_mut)<Q, const N: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>( &mut self, keys: \[[&Q](https://doc.rust-lang.org/nightly/std/primitive.reference.html); [N](https://doc.rust-lang.org/nightly/std/primitive.array.html)\], ) -> \[[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<([&K](https://doc.rust-lang.org/nightly/std/primitive.reference.html), [&mut V](https://doc.rust-lang.org/nightly/std/primitive.reference.html))>; [N](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]

where Q: [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash") + [Equivalent](../platform/collections/trait.Equivalent.html "trait bevy::platform::collections::Equivalent")<K> + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Attempts to get mutable references to `N` values in the map at once, with immutable references to the corresponding keys, without validating that the values are unique.

Refer to [`get_disjoint_key_value_unchecked_mut`](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/hashbrown/map/struct.HashMap.html#method.get_disjoint_key_value_unchecked_mut "method hashbrown::map::HashMap::get_disjoint_key_value_unchecked_mut") for further details.

Returns an array of length `N` with the results of each query. `None` will be returned if any of the keys are missing.

For a safe alternative see [`get_disjoint_key_value_mut`](../platform/collections/struct.HashMap.html#method.get_disjoint_key_value_mut "method bevy::platform::collections::HashMap::get_disjoint_key_value_mut").

##### Safety

Calling this method with overlapping keys is _[undefined behavior](https://doc.rust-lang.org/reference/behavior-considered-undefined.html)_ even if the resulting references are not used.

## Methods from [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref")<Target = [HashMap](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/hashbrown/map/struct.HashMap.html "struct hashbrown::map::HashMap")<K, V, S>>

[Source](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/src/hashbrown/map.rs.html#510)

#### pub fn [allocator](#method.allocator)(&self) -> [&A](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

Returns a reference to the underlying allocator.

[Source](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/src/hashbrown/map.rs.html#600)

#### pub fn [hasher](#method.hasher-1)(&self) -> [&S](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

Returns a reference to the map’s [`BuildHasher`](https://doc.rust-lang.org/std/hash/trait.BuildHasher.html).

##### Examples

```rust
use hashbrown::HashMap;
use hashbrown::DefaultHashBuilder;

let hasher = DefaultHashBuilder::default();
let map: HashMap<i32, i32> = HashMap::with_hasher(hasher);
let hasher: &DefaultHashBuilder = map.hasher();
```

[Source](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/src/hashbrown/map.rs.html#618)

#### pub fn [capacity](#method.capacity-1)(&self) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

Returns the number of elements the map can hold without reallocating.

This number is a lower bound; the `HashMap<K, V>` might be able to hold more, but is guaranteed to be able to hold at least this many.

##### Examples

```rust
use hashbrown::HashMap;
let map: HashMap<i32, i32> = HashMap::with_capacity(100);
assert_eq!(map.len(), 0);
assert!(map.capacity() >= 100);
```

[Source](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/src/hashbrown/map.rs.html#650)

#### pub fn [keys](#method.keys-1)(&self) -> [Keys](../platform/collections/hash_map/struct.Keys.html "struct bevy::platform::collections::hash_map::Keys")<'\_, K, V> [ⓘ](#)

An iterator visiting all keys in arbitrary order. The iterator element type is `&'a K`.

##### Examples

```rust
use hashbrown::HashMap;

let mut map = HashMap::new();
map.insert("a", 1);
map.insert("b", 2);
map.insert("c", 3);
assert_eq!(map.len(), 3);
let mut vec: Vec<&str> = Vec::new();

for key in map.keys() {
    println!("{}", key);
    vec.push(*key);
}

// The `Keys` iterator produces keys in arbitrary order, so the
// keys must be sorted to test them against a sorted array.
vec.sort_unstable();
assert_eq!(vec, ["a", "b", "c"]);

assert_eq!(map.len(), 3);
```

[Source](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/src/hashbrown/map.rs.html#682)

#### pub fn [values](#method.values-1)(&self) -> [Values](../platform/collections/hash_map/struct.Values.html "struct bevy::platform::collections::hash_map::Values")<'\_, K, V> [ⓘ](#)

An iterator visiting all values in arbitrary order. The iterator element type is `&'a V`.

##### Examples

```rust
use hashbrown::HashMap;

let mut map = HashMap::new();
map.insert("a", 1);
map.insert("b", 2);
map.insert("c", 3);
assert_eq!(map.len(), 3);
let mut vec: Vec<i32> = Vec::new();

for val in map.values() {
    println!("{}", val);
    vec.push(*val);
}

// The `Values` iterator produces values in arbitrary order, so the
// values must be sorted to test them against a sorted array.
vec.sort_unstable();
assert_eq!(vec, [1, 2, 3]);

assert_eq!(map.len(), 3);
```

[Source](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/src/hashbrown/map.rs.html#720)

#### pub fn [values\_mut](#method.values_mut-1)(&mut self) -> [ValuesMut](../platform/collections/hash_map/struct.ValuesMut.html "struct bevy::platform::collections::hash_map::ValuesMut")<'\_, K, V> [ⓘ](#)

An iterator visiting all values mutably in arbitrary order. The iterator element type is `&'a mut V`.

##### Examples

```rust
use hashbrown::HashMap;

let mut map = HashMap::new();

map.insert("a", 1);
map.insert("b", 2);
map.insert("c", 3);

for val in map.values_mut() {
    *val = *val + 10;
}

assert_eq!(map.len(), 3);
let mut vec: Vec<i32> = Vec::new();

for val in map.values() {
    println!("{}", val);
    vec.push(*val);
}

// The `Values` iterator produces values in arbitrary order, so the
// values must be sorted to test them against a sorted array.
vec.sort_unstable();
assert_eq!(vec, [11, 12, 13]);

assert_eq!(map.len(), 3);
```

[Source](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/src/hashbrown/map.rs.html#754)

#### pub fn [iter](#method.iter-1)(&self) -> [Iter](../platform/collections/hash_map/struct.Iter.html "struct bevy::platform::collections::hash_map::Iter")<'\_, K, V> [ⓘ](#)

An iterator visiting all key-value pairs in arbitrary order. The iterator element type is `(&'a K, &'a V)`.

##### Examples

```rust
use hashbrown::HashMap;

let mut map = HashMap::new();
map.insert("a", 1);
map.insert("b", 2);
map.insert("c", 3);
assert_eq!(map.len(), 3);
let mut vec: Vec<(&str, i32)> = Vec::new();

for (key, val) in map.iter() {
    println!("key: {} val: {}", key, val);
    vec.push((*key, *val));
}

// The `Iter` iterator produces items in arbitrary order, so the
// items must be sorted to test them against a sorted array.
vec.sort_unstable();
assert_eq!(vec, [("a", 1), ("b", 2), ("c", 3)]);

assert_eq!(map.len(), 3);
```

[Source](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/src/hashbrown/map.rs.html#799)

#### pub fn [iter\_mut](#method.iter_mut-1)(&mut self) -> [IterMut](../platform/collections/hash_map/struct.IterMut.html "struct bevy::platform::collections::hash_map::IterMut")<'\_, K, V> [ⓘ](#)

An iterator visiting all key-value pairs in arbitrary order, with mutable references to the values. The iterator element type is `(&'a K, &'a mut V)`.

##### Examples

```rust
use hashbrown::HashMap;

let mut map = HashMap::new();
map.insert("a", 1);
map.insert("b", 2);
map.insert("c", 3);

// Update all values
for (_, val) in map.iter_mut() {
    *val *= 2;
}

assert_eq!(map.len(), 3);
let mut vec: Vec<(&str, i32)> = Vec::new();

for (key, val) in &map {
    println!("key: {} val: {}", key, val);
    vec.push((*key, *val));
}

// The `Iter` iterator produces items in arbitrary order, so the
// items must be sorted to test them against a sorted array.
vec.sort_unstable();
assert_eq!(vec, [("a", 2), ("b", 4), ("c", 6)]);

assert_eq!(map.len(), 3);
```

[Source](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/src/hashbrown/map.rs.html#828)

#### pub fn [len](#method.len-1)(&self) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

Returns the number of elements in the map.

##### Examples

```rust
use hashbrown::HashMap;

let mut a = HashMap::new();
assert_eq!(a.len(), 0);
a.insert(1, "a");
assert_eq!(a.len(), 1);
```

[Source](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/src/hashbrown/map.rs.html#845)

#### pub fn [is\_empty](#method.is_empty-1)(&self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Returns `true` if the map contains no elements.

##### Examples

```rust
use hashbrown::HashMap;

let mut a = HashMap::new();
assert!(a.is_empty());
a.insert(1, "a");
assert!(!a.is_empty());
```

[Source](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/src/hashbrown/map.rs.html#888)

#### pub fn [drain](#method.drain-1)(&mut self) -> [Drain](../platform/collections/hash_map/struct.Drain.html "struct bevy::platform::collections::hash_map::Drain")<'\_, K, V, A> [ⓘ](#)

Clears the map, returning all key-value pairs as an iterator. Keeps the allocated memory for reuse.

If the returned iterator is dropped before being fully consumed, it drops the remaining key-value pairs. The returned iterator keeps a mutable borrow on the vector to optimize its implementation.

##### Examples

```rust
use hashbrown::HashMap;

let mut a = HashMap::new();
a.insert(1, "a");
a.insert(2, "b");
let capacity_before_drain = a.capacity();

for (k, v) in a.drain().take(1) {
    assert!(k == 1 || k == 2);
    assert!(v == "a" || v == "b");
}

// As we can see, the map is empty and contains no element.
assert!(a.is_empty() && a.len() == 0);
// But map capacity is equal to old one.
assert_eq!(a.capacity(), capacity_before_drain);

let mut a = HashMap::new();
a.insert(1, "a");
a.insert(2, "b");

{   // Iterator is dropped without being consumed.
    let d = a.drain();
}

// But the map is empty even if we do not use Drain iterator.
assert!(a.is_empty());
```

[Source](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/src/hashbrown/map.rs.html#917-919)

#### pub fn [retain](#method.retain-1)<F>(&mut self, f: F)

where F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")([&K](https://doc.rust-lang.org/nightly/std/primitive.reference.html), [&mut V](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html),

Retains only the elements specified by the predicate. Keeps the allocated memory for reuse.

In other words, remove all pairs `(k, v)` such that `f(&k, &mut v)` returns `false`. The elements are visited in unsorted (and unspecified) order.

##### Examples

```rust
use hashbrown::HashMap;

let mut map: HashMap<i32, i32> = (0..8).map(|x|(x, x*10)).collect();
assert_eq!(map.len(), 8);

map.retain(|&k, _| k % 2 == 0);

// We can see, that the number of elements inside map is changed.
assert_eq!(map.len(), 4);

let mut vec: Vec<(i32, i32)> = map.iter().map(|(&k, &v)| (k, v)).collect();
vec.sort_unstable();
assert_eq!(vec, [(0, 0), (2, 20), (4, 40), (6, 60)]);
```

[Source](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/src/hashbrown/map.rs.html#976-978)

#### pub fn [extract\_if](#method.extract_if-1)<F>(&mut self, f: F) -> [ExtractIf](../platform/collections/hash_map/struct.ExtractIf.html "struct bevy::platform::collections::hash_map::ExtractIf")<'\_, K, V, F, A> [ⓘ](#)

where F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")([&K](https://doc.rust-lang.org/nightly/std/primitive.reference.html), [&mut V](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html),

Drains elements which are true under the given predicate, and returns an iterator over the removed items.

In other words, move all pairs `(k, v)` such that `f(&k, &mut v)` returns `true` out into another iterator.

Note that `extract_if` lets you mutate every value in the filter closure, regardless of whether you choose to keep or remove it.

If the returned `ExtractIf` is not exhausted, e.g. because it is dropped without iterating or the iteration short-circuits, then the remaining elements will be retained. Use [`retain()`](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/hashbrown/map/struct.HashMap.html#method.retain "method hashbrown::map::HashMap::retain") with a negated predicate if you do not need the returned iterator.

Keeps the allocated memory for reuse.

##### Examples

```rust
use hashbrown::HashMap;

let mut map: HashMap<i32, i32> = (0..8).map(|x| (x, x)).collect();

let drained: HashMap<i32, i32> = map.extract_if(|k, _v| k % 2 == 0).collect();

let mut evens = drained.keys().cloned().collect::<Vec<_>>();
let mut odds = map.keys().cloned().collect::<Vec<_>>();
evens.sort();
odds.sort();

assert_eq!(evens, vec![0, 2, 4, 6]);
assert_eq!(odds, vec![1, 3, 5, 7]);

let mut map: HashMap<i32, i32> = (0..8).map(|x| (x, x)).collect();

{   // Iterator is dropped without being consumed.
    let d = map.extract_if(|k, _v| k % 2 != 0);
}

// ExtractIf was not exhausted, therefore no elements were drained.
assert_eq!(map.len(), 8);
```

[Source](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/src/hashbrown/map.rs.html#1009)

#### pub fn [clear](#method.clear-1)(&mut self)

Clears the map, removing all key-value pairs. Keeps the allocated memory for reuse.

##### Examples

```rust
use hashbrown::HashMap;

let mut a = HashMap::new();
a.insert(1, "a");
let capacity_before_clear = a.capacity();

a.clear();

// Map is empty.
assert!(a.is_empty());
// But map capacity is equal to old one.
assert_eq!(a.capacity(), capacity_before_clear);
```

[Source](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/src/hashbrown/map.rs.html#1103)

#### pub fn [reserve](#method.reserve-1)(&mut self, additional: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html))

Reserves capacity for at least `additional` more elements to be inserted in the `HashMap`. The collection may reserve more space to avoid frequent reallocations.

##### Panics

Panics if the new capacity exceeds [`isize::MAX`](https://doc.rust-lang.org/std/primitive.isize.html) bytes and [`abort`](https://doc.rust-lang.org/alloc/alloc/fn.handle_alloc_error.html) the program in case of allocation error. Use [`try_reserve`](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/hashbrown/map/struct.HashMap.html#method.try_reserve "method hashbrown::map::HashMap::try_reserve") instead if you want to handle memory allocation failure.

##### Examples

```rust
use hashbrown::HashMap;
let mut map: HashMap<&str, i32> = HashMap::new();
// Map is empty and doesn't allocate memory
assert_eq!(map.capacity(), 0);

map.reserve(10);

// And now map can hold at least 10 elements
assert!(map.capacity() >= 10);
```

[Source](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/src/hashbrown/map.rs.html#1153)

#### pub fn [try\_reserve](#method.try_reserve-1)(&mut self, additional: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [TryReserveError](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/hashbrown/enum.TryReserveError.html "enum hashbrown::TryReserveError")\>

Tries to reserve capacity for at least `additional` more elements to be inserted in the given `HashMap<K,V>`. The collection may reserve more space to avoid frequent reallocations.

##### Errors

If the capacity overflows, or the allocator reports a failure, then an error is returned.

##### Examples

```rust
use hashbrown::HashMap;

let mut map: HashMap<&str, isize> = HashMap::new();
// Map is empty and doesn't allocate memory
assert_eq!(map.capacity(), 0);

map.try_reserve(10).expect("why is the test harness OOMing on 10 bytes?");

// And now map can hold at least 10 elements
assert!(map.capacity() >= 10);
```

If the capacity overflows, or the allocator reports a failure, then an error is returned:

```rust
use hashbrown::HashMap;
use hashbrown::TryReserveError;
let mut map: HashMap<i32, i32> = HashMap::new();

match map.try_reserve(usize::MAX) {
    Err(error) => match error {
        TryReserveError::CapacityOverflow => {}
        _ => panic!("TryReserveError::AllocError ?"),
    },
    _ => panic!(),
}
```

[Source](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/src/hashbrown/map.rs.html#1175)

#### pub fn [shrink\_to\_fit](#method.shrink_to_fit-1)(&mut self)

Shrinks the capacity of the map as much as possible. It will drop down as much as possible while maintaining the internal rules and possibly leaving some space in accordance with the resize policy.

##### Examples

```rust
use hashbrown::HashMap;

let mut map: HashMap<i32, i32> = HashMap::with_capacity(100);
map.insert(1, 2);
map.insert(3, 4);
assert!(map.capacity() >= 100);
map.shrink_to_fit();
assert!(map.capacity() >= 2);
```

[Source](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/src/hashbrown/map.rs.html#1204)

#### pub fn [shrink\_to](#method.shrink_to-1)(&mut self, min\_capacity: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html))

Shrinks the capacity of the map with a lower limit. It will drop down no lower than the supplied limit while maintaining the internal rules and possibly leaving some space in accordance with the resize policy.

This function does nothing if the current capacity is smaller than the supplied minimum capacity.

##### Examples

```rust
use hashbrown::HashMap;

let mut map: HashMap<i32, i32> = HashMap::with_capacity(100);
map.insert(1, 2);
map.insert(3, 4);
assert!(map.capacity() >= 100);
map.shrink_to(10);
assert!(map.capacity() >= 10);
map.shrink_to(0);
assert!(map.capacity() >= 2);
map.shrink_to(10);
assert!(map.capacity() >= 2);
```

[Source](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/src/hashbrown/map.rs.html#1229)

#### pub fn [entry](#method.entry-1)(&mut self, key: K) -> [Entry](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/hashbrown/map/enum.Entry.html "enum hashbrown::map::Entry")<'\_, K, V, S, A>

Gets the given key’s corresponding entry in the map for in-place manipulation.

##### Examples

```rust
use hashbrown::HashMap;

let mut letters = HashMap::new();

for ch in "a short treatise on fungi".chars() {
    let counter = letters.entry(ch).or_insert(0);
    *counter += 1;
}

assert_eq!(letters[&'s'], 2);
assert_eq!(letters[&'t'], 3);
assert_eq!(letters[&'u'], 1);
assert_eq!(letters.get(&'y'), None);
```

[Source](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/src/hashbrown/map.rs.html#1264-1266)

#### pub fn [entry\_ref](#method.entry_ref-1)<'a, 'b, Q>( &'a mut self, key: [&'b Q](https://doc.rust-lang.org/nightly/std/primitive.reference.html), ) -> [EntryRef](../platform/collections/hash_map/enum.EntryRef.html "enum bevy::platform::collections::hash_map::EntryRef")<'a, 'b, K, Q, V, S, A>

where Q: [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash") + [Equivalent](../platform/collections/trait.Equivalent.html "trait bevy::platform::collections::Equivalent")<K> + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Gets the given key’s corresponding entry by reference in the map for in-place manipulation.

##### Examples

```rust
use hashbrown::HashMap;

let mut words: HashMap<String, usize> = HashMap::new();
let source = ["poneyland", "horseyland", "poneyland", "poneyland"];
for (i, &s) in source.iter().enumerate() {
    let counter = words.entry_ref(s).or_insert(0);
    *counter += 1;
}

assert_eq!(words["poneyland"], 3);
assert_eq!(words["horseyland"], 1);
```

[Source](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/src/hashbrown/map.rs.html#1304-1306)

#### pub fn [get](#method.get-1)<Q>(&self, k: [&Q](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[&V](https://doc.rust-lang.org/nightly/std/primitive.reference.html)\>

where Q: [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash") + [Equivalent](../platform/collections/trait.Equivalent.html "trait bevy::platform::collections::Equivalent")<K> + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Returns a reference to the value corresponding to the key.

The key may be any borrowed form of the map’s key type, but [`Hash`](https://doc.rust-lang.org/std/hash/trait.Hash.html) and [`Eq`](https://doc.rust-lang.org/std/cmp/trait.Eq.html) on the borrowed form _must_ match those for the key type.

##### Examples

```rust
use hashbrown::HashMap;

let mut map = HashMap::new();
map.insert(1, "a");
assert_eq!(map.get(&1), Some(&"a"));
assert_eq!(map.get(&2), None);
```

[Source](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/src/hashbrown/map.rs.html#1340-1342)

#### pub fn [get\_key\_value](#method.get_key_value-1)<Q>(&self, k: [&Q](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<([&K](https://doc.rust-lang.org/nightly/std/primitive.reference.html), [&V](https://doc.rust-lang.org/nightly/std/primitive.reference.html))>

where Q: [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash") + [Equivalent](../platform/collections/trait.Equivalent.html "trait bevy::platform::collections::Equivalent")<K> + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Returns the key-value pair corresponding to the supplied key.

The supplied key may be any borrowed form of the map’s key type, but [`Hash`](https://doc.rust-lang.org/std/hash/trait.Hash.html) and [`Eq`](https://doc.rust-lang.org/std/cmp/trait.Eq.html) on the borrowed form _must_ match those for the key type.

##### Examples

```rust
use hashbrown::HashMap;

let mut map = HashMap::new();
map.insert(1, "a");
assert_eq!(map.get_key_value(&1), Some((&1, &"a")));
assert_eq!(map.get_key_value(&2), None);
```

[Source](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/src/hashbrown/map.rs.html#1380-1382)

#### pub fn [get\_key\_value\_mut](#method.get_key_value_mut-1)<Q>(&mut self, k: [&Q](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<([&K](https://doc.rust-lang.org/nightly/std/primitive.reference.html), [&mut V](https://doc.rust-lang.org/nightly/std/primitive.reference.html))>

where Q: [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash") + [Equivalent](../platform/collections/trait.Equivalent.html "trait bevy::platform::collections::Equivalent")<K> + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Returns the key-value pair corresponding to the supplied key, with a mutable reference to value.

The supplied key may be any borrowed form of the map’s key type, but [`Hash`](https://doc.rust-lang.org/std/hash/trait.Hash.html) and [`Eq`](https://doc.rust-lang.org/std/cmp/trait.Eq.html) on the borrowed form _must_ match those for the key type.

##### Examples

```rust
use hashbrown::HashMap;

let mut map = HashMap::new();
map.insert(1, "a");
let (k, v) = map.get_key_value_mut(&1).unwrap();
assert_eq!(k, &1);
assert_eq!(v, &mut "a");
*v = "b";
assert_eq!(map.get_key_value_mut(&1), Some((&1, &mut "b")));
assert_eq!(map.get_key_value_mut(&2), None);
```

[Source](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/src/hashbrown/map.rs.html#1416-1418)

#### pub fn [contains\_key](#method.contains_key-1)<Q>(&self, k: [&Q](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

where Q: [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash") + [Equivalent](../platform/collections/trait.Equivalent.html "trait bevy::platform::collections::Equivalent")<K> + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Returns `true` if the map contains a value for the specified key.

The key may be any borrowed form of the map’s key type, but [`Hash`](https://doc.rust-lang.org/std/hash/trait.Hash.html) and [`Eq`](https://doc.rust-lang.org/std/cmp/trait.Eq.html) on the borrowed form _must_ match those for the key type.

##### Examples

```rust
use hashbrown::HashMap;

let mut map = HashMap::new();
map.insert(1, "a");
assert_eq!(map.contains_key(&1), true);
assert_eq!(map.contains_key(&2), false);
```

[Source](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/src/hashbrown/map.rs.html#1452-1454)

#### pub fn [get\_mut](#method.get_mut-1)<Q>(&mut self, k: [&Q](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[&mut V](https://doc.rust-lang.org/nightly/std/primitive.reference.html)\>

where Q: [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash") + [Equivalent](../platform/collections/trait.Equivalent.html "trait bevy::platform::collections::Equivalent")<K> + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Returns a mutable reference to the value corresponding to the key.

The key may be any borrowed form of the map’s key type, but [`Hash`](https://doc.rust-lang.org/std/hash/trait.Hash.html) and [`Eq`](https://doc.rust-lang.org/std/cmp/trait.Eq.html) on the borrowed form _must_ match those for the key type.

##### Examples

```rust
use hashbrown::HashMap;

let mut map = HashMap::new();
map.insert(1, "a");
if let Some(x) = map.get_mut(&1) {
    *x = "b";
}
assert_eq!(map[&1], "b");

assert_eq!(map.get_mut(&2), None);
```

[Source](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/src/hashbrown/map.rs.html#1533-1535)

#### pub fn [get\_disjoint\_mut](#method.get_disjoint_mut-1)<Q, const N: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>( &mut self, ks: \[[&Q](https://doc.rust-lang.org/nightly/std/primitive.reference.html); [N](https://doc.rust-lang.org/nightly/std/primitive.array.html)\], ) -> \[[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[&mut V](https://doc.rust-lang.org/nightly/std/primitive.reference.html)\>; [N](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]

where Q: [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash") + [Equivalent](../platform/collections/trait.Equivalent.html "trait bevy::platform::collections::Equivalent")<K> + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Attempts to get mutable references to `N` values in the map at once.

Returns an array of length `N` with the results of each query. For soundness, at most one mutable reference will be returned to any value. `None` will be used if the key is missing.

##### Panics

Panics if any keys are overlapping.

##### Examples

```rust
use hashbrown::HashMap;

let mut libraries = HashMap::new();
libraries.insert("Bodleian Library".to_string(), 1602);
libraries.insert("Athenæum".to_string(), 1807);
libraries.insert("Herzogin-Anna-Amalia-Bibliothek".to_string(), 1691);
libraries.insert("Library of Congress".to_string(), 1800);

// Get Athenæum and Bodleian Library
let [Some(a), Some(b)] = libraries.get_disjoint_mut([
    "Athenæum",
    "Bodleian Library",
]) else { panic!() };

// Assert values of Athenæum and Library of Congress
let got = libraries.get_disjoint_mut([
    "Athenæum",
    "Library of Congress",
]);
assert_eq!(
    got,
    [
        Some(&mut 1807),
        Some(&mut 1800),
    ],
);

// Missing keys result in None
let got = libraries.get_disjoint_mut([
    "Athenæum",
    "New York Public Library",
]);
assert_eq!(
    got,
    [
        Some(&mut 1807),
        None
    ]
);
```

[ⓘ](# "This example panics")

```rust
use hashbrown::HashMap;

let mut libraries = HashMap::new();
libraries.insert("Athenæum".to_string(), 1807);

// Duplicate keys panic!
let got = libraries.get_disjoint_mut([
    "Athenæum",
    "Athenæum",
]);
```

[Source](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/src/hashbrown/map.rs.html#1543-1545)

#### pub fn [get\_many\_mut](#method.get_many_mut)<Q, const N: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>( &mut self, ks: \[[&Q](https://doc.rust-lang.org/nightly/std/primitive.reference.html); [N](https://doc.rust-lang.org/nightly/std/primitive.array.html)\], ) -> \[[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[&mut V](https://doc.rust-lang.org/nightly/std/primitive.reference.html)\>; [N](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]

where Q: [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash") + [Equivalent](../platform/collections/trait.Equivalent.html "trait bevy::platform::collections::Equivalent")<K> + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

👎Deprecated:

use `get_disjoint_mut` instead

Attempts to get mutable references to `N` values in the map at once.

[Source](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/src/hashbrown/map.rs.html#1603-1608)

#### pub unsafe fn [get\_disjoint\_unchecked\_mut](#method.get_disjoint_unchecked_mut-1)<Q, const N: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>( &mut self, ks: \[[&Q](https://doc.rust-lang.org/nightly/std/primitive.reference.html); [N](https://doc.rust-lang.org/nightly/std/primitive.array.html)\], ) -> \[[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[&mut V](https://doc.rust-lang.org/nightly/std/primitive.reference.html)\>; [N](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]

where Q: [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash") + [Equivalent](../platform/collections/trait.Equivalent.html "trait bevy::platform::collections::Equivalent")<K> + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Attempts to get mutable references to `N` values in the map at once, without validating that the values are unique.

Returns an array of length `N` with the results of each query. `None` will be used if the key is missing.

For a safe alternative see [`get_disjoint_mut`](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/hashbrown/map/struct.HashMap.html#method.get_disjoint_mut "method hashbrown::map::HashMap::get_disjoint_mut").

##### Safety

Calling this method with overlapping keys is _[undefined behavior](https://doc.rust-lang.org/reference/behavior-considered-undefined.html)_ even if the resulting references are not used.

##### Examples

```rust
use hashbrown::HashMap;

let mut libraries = HashMap::new();
libraries.insert("Bodleian Library".to_string(), 1602);
libraries.insert("Athenæum".to_string(), 1807);
libraries.insert("Herzogin-Anna-Amalia-Bibliothek".to_string(), 1691);
libraries.insert("Library of Congress".to_string(), 1800);

// SAFETY: The keys do not overlap.
let [Some(a), Some(b)] = (unsafe { libraries.get_disjoint_unchecked_mut([
    "Athenæum",
    "Bodleian Library",
]) }) else { panic!() };

// SAFETY: The keys do not overlap.
let got = unsafe { libraries.get_disjoint_unchecked_mut([
    "Athenæum",
    "Library of Congress",
]) };
assert_eq!(
    got,
    [
        Some(&mut 1807),
        Some(&mut 1800),
    ],
);

// SAFETY: The keys do not overlap.
let got = unsafe { libraries.get_disjoint_unchecked_mut([
    "Athenæum",
    "New York Public Library",
]) };
// Missing keys result in None
assert_eq!(got, [Some(&mut 1807), None]);
```

[Source](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/src/hashbrown/map.rs.html#1617-1622)

#### pub unsafe fn [get\_many\_unchecked\_mut](#method.get_many_unchecked_mut)<Q, const N: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>( &mut self, ks: \[[&Q](https://doc.rust-lang.org/nightly/std/primitive.reference.html); [N](https://doc.rust-lang.org/nightly/std/primitive.array.html)\], ) -> \[[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[&mut V](https://doc.rust-lang.org/nightly/std/primitive.reference.html)\>; [N](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]

where Q: [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash") + [Equivalent](../platform/collections/trait.Equivalent.html "trait bevy::platform::collections::Equivalent")<K> + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

👎Deprecated:

use `get_disjoint_unchecked_mut` instead

Attempts to get mutable references to `N` values in the map at once, without validating that the values are unique.

[Source](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/src/hashbrown/map.rs.html#1681-1686)

#### pub fn [get\_disjoint\_key\_value\_mut](#method.get_disjoint_key_value_mut-1)<Q, const N: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>( &mut self, ks: \[[&Q](https://doc.rust-lang.org/nightly/std/primitive.reference.html); [N](https://doc.rust-lang.org/nightly/std/primitive.array.html)\], ) -> \[[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<([&K](https://doc.rust-lang.org/nightly/std/primitive.reference.html), [&mut V](https://doc.rust-lang.org/nightly/std/primitive.reference.html))>; [N](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]

where Q: [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash") + [Equivalent](../platform/collections/trait.Equivalent.html "trait bevy::platform::collections::Equivalent")<K> + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Attempts to get mutable references to `N` values in the map at once, with immutable references to the corresponding keys.

Returns an array of length `N` with the results of each query. For soundness, at most one mutable reference will be returned to any value. `None` will be used if the key is missing.

##### Panics

Panics if any keys are overlapping.

##### Examples

```rust
use hashbrown::HashMap;

let mut libraries = HashMap::new();
libraries.insert("Bodleian Library".to_string(), 1602);
libraries.insert("Athenæum".to_string(), 1807);
libraries.insert("Herzogin-Anna-Amalia-Bibliothek".to_string(), 1691);
libraries.insert("Library of Congress".to_string(), 1800);

let got = libraries.get_disjoint_key_value_mut([
    "Bodleian Library",
    "Herzogin-Anna-Amalia-Bibliothek",
]);
assert_eq!(
    got,
    [
        Some((&"Bodleian Library".to_string(), &mut 1602)),
        Some((&"Herzogin-Anna-Amalia-Bibliothek".to_string(), &mut 1691)),
    ],
);
// Missing keys result in None
let got = libraries.get_disjoint_key_value_mut([
    "Bodleian Library",
    "Gewandhaus",
]);
assert_eq!(got, [Some((&"Bodleian Library".to_string(), &mut 1602)), None]);
```

[ⓘ](# "This example panics")

```rust
use hashbrown::HashMap;

let mut libraries = HashMap::new();
libraries.insert("Bodleian Library".to_string(), 1602);
libraries.insert("Herzogin-Anna-Amalia-Bibliothek".to_string(), 1691);

// Duplicate keys result in panic!
let got = libraries.get_disjoint_key_value_mut([
    "Bodleian Library",
    "Herzogin-Anna-Amalia-Bibliothek",
    "Herzogin-Anna-Amalia-Bibliothek",
]);
```

[Source](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/src/hashbrown/map.rs.html#1695-1700)

#### pub fn [get\_many\_key\_value\_mut](#method.get_many_key_value_mut)<Q, const N: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>( &mut self, ks: \[[&Q](https://doc.rust-lang.org/nightly/std/primitive.reference.html); [N](https://doc.rust-lang.org/nightly/std/primitive.array.html)\], ) -> \[[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<([&K](https://doc.rust-lang.org/nightly/std/primitive.reference.html), [&mut V](https://doc.rust-lang.org/nightly/std/primitive.reference.html))>; [N](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]

where Q: [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash") + [Equivalent](../platform/collections/trait.Equivalent.html "trait bevy::platform::collections::Equivalent")<K> + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

👎Deprecated:

use `get_disjoint_key_value_mut` instead

Attempts to get mutable references to `N` values in the map at once, with immutable references to the corresponding keys.

[Source](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/src/hashbrown/map.rs.html#1755-1760)

#### pub unsafe fn [get\_disjoint\_key\_value\_unchecked\_mut](#method.get_disjoint_key_value_unchecked_mut-1)<Q, const N: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>( &mut self, ks: \[[&Q](https://doc.rust-lang.org/nightly/std/primitive.reference.html); [N](https://doc.rust-lang.org/nightly/std/primitive.array.html)\], ) -> \[[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<([&K](https://doc.rust-lang.org/nightly/std/primitive.reference.html), [&mut V](https://doc.rust-lang.org/nightly/std/primitive.reference.html))>; [N](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]

where Q: [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash") + [Equivalent](../platform/collections/trait.Equivalent.html "trait bevy::platform::collections::Equivalent")<K> + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Attempts to get mutable references to `N` values in the map at once, with immutable references to the corresponding keys, without validating that the values are unique.

Returns an array of length `N` with the results of each query. `None` will be returned if any of the keys are missing.

For a safe alternative see [`get_disjoint_key_value_mut`](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/hashbrown/map/struct.HashMap.html#method.get_disjoint_key_value_mut "method hashbrown::map::HashMap::get_disjoint_key_value_mut").

##### Safety

Calling this method with overlapping keys is _[undefined behavior](https://doc.rust-lang.org/reference/behavior-considered-undefined.html)_ even if the resulting references are not used.

##### Examples

```rust
use hashbrown::HashMap;

let mut libraries = HashMap::new();
libraries.insert("Bodleian Library".to_string(), 1602);
libraries.insert("Athenæum".to_string(), 1807);
libraries.insert("Herzogin-Anna-Amalia-Bibliothek".to_string(), 1691);
libraries.insert("Library of Congress".to_string(), 1800);

let got = libraries.get_disjoint_key_value_mut([
    "Bodleian Library",
    "Herzogin-Anna-Amalia-Bibliothek",
]);
assert_eq!(
    got,
    [
        Some((&"Bodleian Library".to_string(), &mut 1602)),
        Some((&"Herzogin-Anna-Amalia-Bibliothek".to_string(), &mut 1691)),
    ],
);
// Missing keys result in None
let got = libraries.get_disjoint_key_value_mut([
    "Bodleian Library",
    "Gewandhaus",
]);
assert_eq!(
    got,
    [
        Some((&"Bodleian Library".to_string(), &mut 1602)),
        None,
    ],
);
```

[Source](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/src/hashbrown/map.rs.html#1769-1774)

#### pub unsafe fn [get\_many\_key\_value\_unchecked\_mut](#method.get_many_key_value_unchecked_mut)<Q, const N: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>( &mut self, ks: \[[&Q](https://doc.rust-lang.org/nightly/std/primitive.reference.html); [N](https://doc.rust-lang.org/nightly/std/primitive.array.html)\], ) -> \[[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<([&K](https://doc.rust-lang.org/nightly/std/primitive.reference.html), [&mut V](https://doc.rust-lang.org/nightly/std/primitive.reference.html))>; [N](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]

where Q: [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash") + [Equivalent](../platform/collections/trait.Equivalent.html "trait bevy::platform::collections::Equivalent")<K> + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

👎Deprecated:

use `get_disjoint_key_value_unchecked_mut` instead

Attempts to get mutable references to `N` values in the map at once, with immutable references to the corresponding keys, without validating that the values are unique.

[Source](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/src/hashbrown/map.rs.html#1841)

#### pub fn [insert](#method.insert-1)(&mut self, k: K, v: V) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<V>

Inserts a key-value pair into the map.

If the map did not have this key present, [`None`](https://doc.rust-lang.org/std/option/enum.Option.html#variant.None) is returned.

If the map did have this key present, the value is updated, and the old value is returned. The key is not updated, though; this matters for types that can be `==` without being identical. See the [`std::collections`](https://doc.rust-lang.org/std/collections/index.html) [module-level documentation](https://doc.rust-lang.org/std/collections/index.html#insert-and-complex-keys) for more.

##### Examples

```rust
use hashbrown::HashMap;

let mut map = HashMap::new();
assert_eq!(map.insert(37, "a"), None);
assert_eq!(map.is_empty(), false);

map.insert(37, "b");
assert_eq!(map.insert(37, "c"), Some("b"));
assert_eq!(map[&37], "c");
```

[Source](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/src/hashbrown/map.rs.html#1928)

#### pub unsafe fn [insert\_unique\_unchecked](#method.insert_unique_unchecked-1)(&mut self, k: K, v: V) -> ([&K](https://doc.rust-lang.org/nightly/std/primitive.reference.html), [&mut V](https://doc.rust-lang.org/nightly/std/primitive.reference.html))

Insert a key-value pair into the map without checking if the key already exists in the map.

This operation is faster than regular insert, because it does not perform lookup before insertion.

This operation is useful during initial population of the map. For example, when constructing a map from another map, we know that keys are unique.

Returns a reference to the key and value just inserted.

##### Safety

This operation is safe if a key does not exist in the map.

However, if a key exists in the map already, the behavior is unspecified: this operation may panic, loop forever, or any following operation with the map may panic, loop forever or return arbitrary result.

That said, this operation (and following operations) are guaranteed to not violate memory safety.

However this operation is still unsafe because the resulting `HashMap` may be passed to unsafe code which does expect the map to behave correctly, and would cause unsoundness as a result.

##### Examples

```rust
use hashbrown::HashMap;

let mut map1 = HashMap::new();
assert_eq!(map1.insert(1, "a"), None);
assert_eq!(map1.insert(2, "b"), None);
assert_eq!(map1.insert(3, "c"), None);
assert_eq!(map1.len(), 3);

let mut map2 = HashMap::new();

for (key, value) in map1.into_iter() {
    unsafe {
        map2.insert_unique_unchecked(key, value);
    }
}

let (key, value) = unsafe { map2.insert_unique_unchecked(4, "d") };
assert_eq!(key, &4);
assert_eq!(value, &mut "d");
*value = "e";

assert_eq!(map2[&1], "a");
assert_eq!(map2[&2], "b");
assert_eq!(map2[&3], "c");
assert_eq!(map2[&4], "e");
assert_eq!(map2.len(), 4);
```

[Source](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/src/hashbrown/map.rs.html#1966-1970)

#### pub fn [try\_insert](#method.try_insert-1)( &mut self, key: K, value: V, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[&mut V](https://doc.rust-lang.org/nightly/std/primitive.reference.html), [OccupiedError](../platform/collections/hash_map/struct.OccupiedError.html "struct bevy::platform::collections::hash_map::OccupiedError")<'\_, K, V, S, A>>

Tries to insert a key-value pair into the map, and returns a mutable reference to the value in the entry.

##### Errors

If the map already had this key present, nothing is updated, and an error containing the occupied entry and the value is returned.

##### Examples

Basic usage:

```rust
use hashbrown::HashMap;
use hashbrown::hash_map::OccupiedError;

let mut map = HashMap::new();
assert_eq!(map.try_insert(37, "a").unwrap(), &"a");

match map.try_insert(37, "b") {
    Err(OccupiedError { entry, value }) => {
        assert_eq!(entry.key(), &37);
        assert_eq!(entry.get(), &"a");
        assert_eq!(value, "b");
    }
    _ => panic!()
}
```

[Source](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/src/hashbrown/map.rs.html#2005-2007)

#### pub fn [remove](#method.remove-1)<Q>(&mut self, k: [&Q](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<V>

where Q: [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash") + [Equivalent](../platform/collections/trait.Equivalent.html "trait bevy::platform::collections::Equivalent")<K> + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Removes a key from the map, returning the value at the key if the key was previously in the map. Keeps the allocated memory for reuse.

The key may be any borrowed form of the map’s key type, but [`Hash`](https://doc.rust-lang.org/std/hash/trait.Hash.html) and [`Eq`](https://doc.rust-lang.org/std/cmp/trait.Eq.html) on the borrowed form _must_ match those for the key type.

##### Examples

```rust
use hashbrown::HashMap;

let mut map = HashMap::new();
// The map is empty
assert!(map.is_empty() && map.capacity() == 0);

map.insert(1, "a");

assert_eq!(map.remove(&1), Some("a"));
assert_eq!(map.remove(&1), None);

// Now map holds none elements
assert!(map.is_empty());
```

[Source](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/src/hashbrown/map.rs.html#2044-2046)

#### pub fn [remove\_entry](#method.remove_entry-1)<Q>(&mut self, k: [&Q](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[(K, V)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)\>

where Q: [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash") + [Equivalent](../platform/collections/trait.Equivalent.html "trait bevy::platform::collections::Equivalent")<K> + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Removes a key from the map, returning the stored key and value if the key was previously in the map. Keeps the allocated memory for reuse.

The key may be any borrowed form of the map’s key type, but [`Hash`](https://doc.rust-lang.org/std/hash/trait.Hash.html) and [`Eq`](https://doc.rust-lang.org/std/cmp/trait.Eq.html) on the borrowed form _must_ match those for the key type.

##### Examples

```rust
use hashbrown::HashMap;

let mut map = HashMap::new();
// The map is empty
assert!(map.is_empty() && map.capacity() == 0);

map.insert(1, "a");

assert_eq!(map.remove_entry(&1), Some((1, "a")));
assert_eq!(map.remove(&1), None);

// Now map hold none elements
assert!(map.is_empty());
```

[Source](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/src/hashbrown/map.rs.html#2058)

#### pub fn [allocation\_size](#method.allocation_size-1)(&self) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

Returns the total amount of memory allocated internally by the hash set, in bytes.

The returned number is informational only. It is intended to be primarily used for memory profiling.

[Source](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/src/hashbrown/raw_entry.rs.html#107)

#### pub fn [raw\_entry\_mut](#method.raw_entry_mut)(&mut self) -> [RawEntryBuilderMut](../platform/collections/hash_map/struct.RawEntryBuilderMut.html "struct bevy::platform::collections::hash_map::RawEntryBuilderMut")<'\_, K, V, S, A>

Creates a raw entry builder for the `HashMap`.

Raw entries provide the lowest level of control for searching and manipulating a map. They must be manually initialized with a hash and then manually searched. After this, insertions into a vacant entry still require an owned key to be provided.

Raw entries are useful for such exotic situations as:

*   Hash memoization
*   Deferring the creation of an owned key until it is known to be required
*   Using a search key that doesn’t work with the Borrow trait
*   Using custom comparison logic without newtype wrappers

Because raw entries provide much more low-level control, it’s much easier to put the `HashMap` into an inconsistent state which, while memory-safe, will cause the map to produce seemingly random results. Higher-level and more foolproof APIs like `entry` should be preferred when possible.

In particular, the hash used to initialized the raw entry must still be consistent with the hash of the key that is ultimately stored in the entry. This is because implementations of `HashMap` may need to recompute hashes when resizing, at which point only the keys are available.

Raw entries give mutable access to the keys. This must not be used to modify how the key would compare or hash, as the map will not re-evaluate where the key should go, meaning the keys may become “lost” if their location does not reflect their state. For instance, if you change a key so that the map now contains keys which compare equal, search may start acting erratically, with two keys randomly masking each other. Implementations are free to assume this doesn’t happen (within the limits of memory-safety).

##### Examples

```rust
use core::hash::{BuildHasher, Hash};
use hashbrown::hash_map::{HashMap, RawEntryMut};

let mut map = HashMap::new();
map.extend([("a", 100), ("b", 200), ("c", 300)]);

fn compute_hash<K: Hash + ?Sized, S: BuildHasher>(hash_builder: &S, key: &K) -> u64 {
    use core::hash::Hasher;
    let mut state = hash_builder.build_hasher();
    key.hash(&mut state);
    state.finish()
}

// Existing key (insert and update)
match map.raw_entry_mut().from_key(&"a") {
    RawEntryMut::Vacant(_) => unreachable!(),
    RawEntryMut::Occupied(mut view) => {
        assert_eq!(view.get(), &100);
        let v = view.get_mut();
        let new_v = (*v) * 10;
        *v = new_v;
        assert_eq!(view.insert(1111), 1000);
    }
}

assert_eq!(map[&"a"], 1111);
assert_eq!(map.len(), 3);

// Existing key (take)
let hash = compute_hash(map.hasher(), &"c");
match map.raw_entry_mut().from_key_hashed_nocheck(hash, &"c") {
    RawEntryMut::Vacant(_) => unreachable!(),
    RawEntryMut::Occupied(view) => {
        assert_eq!(view.remove_entry(), ("c", 300));
    }
}
assert_eq!(map.raw_entry().from_key(&"c"), None);
assert_eq!(map.len(), 2);

// Nonexistent key (insert and update)
let key = "d";
let hash = compute_hash(map.hasher(), &key);
match map.raw_entry_mut().from_hash(hash, |q| *q == key) {
    RawEntryMut::Occupied(_) => unreachable!(),
    RawEntryMut::Vacant(view) => {
        let (k, value) = view.insert("d", 4000);
        assert_eq!((*k, *value), ("d", 4000));
        *value = 40000;
    }
}
assert_eq!(map[&"d"], 40000);
assert_eq!(map.len(), 3);

match map.raw_entry_mut().from_hash(hash, |q| *q == key) {
    RawEntryMut::Vacant(_) => unreachable!(),
    RawEntryMut::Occupied(view) => {
        assert_eq!(view.remove_entry(), ("d", 40000));
    }
}
assert_eq!(map.get(&"d"), None);
assert_eq!(map.len(), 2);
```

[Source](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/src/hashbrown/raw_entry.rs.html#156)

#### pub fn [raw\_entry](#method.raw_entry)(&self) -> [RawEntryBuilder](../platform/collections/hash_map/struct.RawEntryBuilder.html "struct bevy::platform::collections::hash_map::RawEntryBuilder")<'\_, K, V, S, A>

Creates a raw immutable entry builder for the `HashMap`.

Raw entries provide the lowest level of control for searching and manipulating a map. They must be manually initialized with a hash and then manually searched.

This is useful for

*   Hash memoization
*   Using a search key that doesn’t work with the Borrow trait
*   Using custom comparison logic without newtype wrappers

Unless you are in such a situation, higher-level and more foolproof APIs like `get` should be preferred.

Immutable raw entries have very limited use; you might instead want `raw_entry_mut`.

##### Examples

```rust
use core::hash::{BuildHasher, Hash};
use hashbrown::HashMap;

let mut map = HashMap::new();
map.extend([("a", 100), ("b", 200), ("c", 300)]);

fn compute_hash<K: Hash + ?Sized, S: BuildHasher>(hash_builder: &S, key: &K) -> u64 {
    use core::hash::Hasher;
    let mut state = hash_builder.build_hasher();
    key.hash(&mut state);
    state.finish()
}

for k in ["a", "b", "c", "d", "e", "f"] {
    let hash = compute_hash(map.hasher(), k);
    let v = map.get(&k).cloned();
    let kv = v.as_ref().map(|v| (&k, v));

    println!("Key: {} and value: {:?}", k, v);

    assert_eq!(map.raw_entry().from_key(&k), kv);
    assert_eq!(map.raw_entry().from_hash(hash, |q| *q == k), kv);
    assert_eq!(map.raw_entry().from_key_hashed_nocheck(hash, &k), kv);
}
```

## Trait Implementations

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/texture/manual_texture_view.rs.html#52)

### impl [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") for [ManualTextureViews](struct.ManualTextureViews.html "struct bevy::prelude::ManualTextureViews")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/texture/manual_texture_view.rs.html#52)

#### fn [clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)(&self) -> [ManualTextureViews](struct.ManualTextureViews.html "struct bevy::prelude::ManualTextureViews")

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 (const: [unstable](https://github.com/rust-lang/rust/issues/142757 "Tracking issue for const_clone")) · [Source](https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247)

#### fn [clone\_from](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/texture/manual_texture_view.rs.html#52)

### impl [Component](trait.Component.html "trait bevy::prelude::Component") for [ManualTextureViews](struct.ManualTextureViews.html "struct bevy::prelude::ManualTextureViews")

where [ManualTextureViews](struct.ManualTextureViews.html "struct bevy::prelude::ManualTextureViews"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/texture/manual_texture_view.rs.html#52)

#### const [STORAGE\_TYPE](trait.Component.html#associatedconstant.STORAGE_TYPE): [StorageType](../ecs/component/enum.StorageType.html "enum bevy::ecs::component::StorageType") = bevy\_ecs::component::StorageType::SparseSet

A constant indicating the storage type used for this component.

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/texture/manual_texture_view.rs.html#52)

#### type [Mutability](trait.Component.html#associatedtype.Mutability) = [Mutable](../ecs/component/struct.Mutable.html "struct bevy::ecs::component::Mutable")

A marker type to assist Bevy with determining if this component is mutable, or immutable. Mutable components will have [`Component<Mutability = Mutable>`](trait.Component.html "trait bevy::prelude::Component"), while immutable components will instead have [`Component<Mutability = Immutable>`](trait.Component.html "trait bevy::prelude::Component"). [Read more](trait.Component.html#associatedtype.Mutability)

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/texture/manual_texture_view.rs.html#52)

#### fn [register\_required\_components](trait.Component.html#method.register_required_components)( \_requiree: [ComponentId](../ecs/component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId"), required\_components: &mut [RequiredComponentsRegistrator](../ecs/component/struct.RequiredComponentsRegistrator.html "struct bevy::ecs::component::RequiredComponentsRegistrator")<'\_, '\_>, )

Registers required components. [Read more](trait.Component.html#method.register_required_components)

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/texture/manual_texture_view.rs.html#52)

#### fn [clone\_behavior](trait.Component.html#method.clone_behavior)() -> [ComponentCloneBehavior](../ecs/component/enum.ComponentCloneBehavior.html "enum bevy::ecs::component::ComponentCloneBehavior")

Called when registering this component, allowing to override clone function (or disable cloning altogether) for this component. [Read more](trait.Component.html#method.clone_behavior)

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/texture/manual_texture_view.rs.html#52)

#### fn [relationship\_accessor](trait.Component.html#method.relationship_accessor)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[ComponentRelationshipAccessor](../ecs/relationship/struct.ComponentRelationshipAccessor.html "struct bevy::ecs::relationship::ComponentRelationshipAccessor")<[ManualTextureViews](struct.ManualTextureViews.html "struct bevy::prelude::ManualTextureViews")\>>

Returns [`ComponentRelationshipAccessor`](../ecs/relationship/struct.ComponentRelationshipAccessor.html "struct bevy::ecs::relationship::ComponentRelationshipAccessor") required for working with relationships in dynamic contexts. [Read more](trait.Component.html#method.relationship_accessor)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/component/mod.rs.html#524)

#### fn [on\_add](trait.Component.html#method.on_add)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<for<'w> [fn](https://doc.rust-lang.org/nightly/std/primitive.fn.html)([DeferredWorld](../ecs/world/struct.DeferredWorld.html "struct bevy::ecs::world::DeferredWorld")<'w>, [HookContext](../ecs/lifecycle/struct.HookContext.html "struct bevy::ecs::lifecycle::HookContext"))>

Gets the `on_add` [`ComponentHook`](../ecs/lifecycle/type.ComponentHook.html "type bevy::ecs::lifecycle::ComponentHook") for this [`Component`](trait.Component.html "trait bevy::prelude::Component") if one is defined.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/component/mod.rs.html#529)

#### fn [on\_insert](trait.Component.html#method.on_insert)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<for<'w> [fn](https://doc.rust-lang.org/nightly/std/primitive.fn.html)([DeferredWorld](../ecs/world/struct.DeferredWorld.html "struct bevy::ecs::world::DeferredWorld")<'w>, [HookContext](../ecs/lifecycle/struct.HookContext.html "struct bevy::ecs::lifecycle::HookContext"))>

Gets the `on_insert` [`ComponentHook`](../ecs/lifecycle/type.ComponentHook.html "type bevy::ecs::lifecycle::ComponentHook") for this [`Component`](trait.Component.html "trait bevy::prelude::Component") if one is defined.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/component/mod.rs.html#534)

#### fn [on\_discard](trait.Component.html#method.on_discard)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<for<'w> [fn](https://doc.rust-lang.org/nightly/std/primitive.fn.html)([DeferredWorld](../ecs/world/struct.DeferredWorld.html "struct bevy::ecs::world::DeferredWorld")<'w>, [HookContext](../ecs/lifecycle/struct.HookContext.html "struct bevy::ecs::lifecycle::HookContext"))>

Gets the `on_discard` [`ComponentHook`](../ecs/lifecycle/type.ComponentHook.html "type bevy::ecs::lifecycle::ComponentHook") for this [`Component`](trait.Component.html "trait bevy::prelude::Component") if one is defined.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/component/mod.rs.html#539)

#### fn [on\_remove](trait.Component.html#method.on_remove)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<for<'w> [fn](https://doc.rust-lang.org/nightly/std/primitive.fn.html)([DeferredWorld](../ecs/world/struct.DeferredWorld.html "struct bevy::ecs::world::DeferredWorld")<'w>, [HookContext](../ecs/lifecycle/struct.HookContext.html "struct bevy::ecs::lifecycle::HookContext"))>

Gets the `on_remove` [`ComponentHook`](../ecs/lifecycle/type.ComponentHook.html "type bevy::ecs::lifecycle::ComponentHook") for this [`Component`](trait.Component.html "trait bevy::prelude::Component") if one is defined.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/component/mod.rs.html#544)

#### fn [on\_despawn](trait.Component.html#method.on_despawn)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<for<'w> [fn](https://doc.rust-lang.org/nightly/std/primitive.fn.html)([DeferredWorld](../ecs/world/struct.DeferredWorld.html "struct bevy::ecs::world::DeferredWorld")<'w>, [HookContext](../ecs/lifecycle/struct.HookContext.html "struct bevy::ecs::lifecycle::HookContext"))>

Gets the `on_despawn` [`ComponentHook`](../ecs/lifecycle/type.ComponentHook.html "type bevy::ecs::lifecycle::ComponentHook") for this [`Component`](trait.Component.html "trait bevy::prelude::Component") if one is defined.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/component/mod.rs.html#649)

#### fn [map\_entities](trait.Component.html#method.map_entities)<E>(\_this: &mut Self, \_mapper: [&mut E](https://doc.rust-lang.org/nightly/std/primitive.reference.html))

where E: [EntityMapper](trait.EntityMapper.html "trait bevy::prelude::EntityMapper"),

Maps the entities on this component using the given [`EntityMapper`](trait.EntityMapper.html "trait bevy::prelude::EntityMapper"). This is used to remap entities in contexts like scenes and entity cloning. When deriving [`Component`](trait.Component.html "trait bevy::prelude::Component"), this is populated by annotating fields containing entities with `#[entities]` [Read more](trait.Component.html#method.map_entities)

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/texture/manual_texture_view.rs.html#52)

### impl [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") for [ManualTextureViews](struct.ManualTextureViews.html "struct bevy::prelude::ManualTextureViews")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/texture/manual_texture_view.rs.html#52)

#### fn [default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)() -> [ManualTextureViews](struct.ManualTextureViews.html "struct bevy::prelude::ManualTextureViews")

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/texture/manual_texture_view.rs.html#55)

### impl [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref") for [ManualTextureViews](struct.ManualTextureViews.html "struct bevy::prelude::ManualTextureViews")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/texture/manual_texture_view.rs.html#56)

#### type [Target](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#associatedtype.Target) = [HashMap](../platform/collections/struct.HashMap.html "struct bevy::platform::collections::HashMap")<[ManualTextureViewHandle](../camera/struct.ManualTextureViewHandle.html "struct bevy::camera::ManualTextureViewHandle"), [ManualTextureView](../render/texture/struct.ManualTextureView.html "struct bevy::render::texture::ManualTextureView")\>

The resulting type after dereferencing.

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/texture/manual_texture_view.rs.html#58)

#### fn [deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#tymethod.deref)(&self) -> &<[ManualTextureViews](struct.ManualTextureViews.html "struct bevy::prelude::ManualTextureViews") as [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref")\>::[Target](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#associatedtype.Target "type core::ops::deref::Deref::Target")

Dereferences the value.

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/texture/manual_texture_view.rs.html#63)

### impl [DerefMut](https://doc.rust-lang.org/nightly/core/ops/deref/trait.DerefMut.html "trait core::ops::deref::DerefMut") for [ManualTextureViews](struct.ManualTextureViews.html "struct bevy::prelude::ManualTextureViews")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/texture/manual_texture_view.rs.html#64)

#### fn [deref\_mut](https://doc.rust-lang.org/nightly/core/ops/deref/trait.DerefMut.html#tymethod.deref_mut)(&mut self) -> &mut <[ManualTextureViews](struct.ManualTextureViews.html "struct bevy::prelude::ManualTextureViews") as [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref")\>::[Target](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#associatedtype.Target "type core::ops::deref::Deref::Target")

Mutably dereferences the value.

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/texture/manual_texture_view.rs.html#52)

### impl [ExtractResource](../render/extract_resource/trait.ExtractResource.html "trait bevy::render::extract_resource::ExtractResource") for [ManualTextureViews](struct.ManualTextureViews.html "struct bevy::prelude::ManualTextureViews")

where [ManualTextureViews](struct.ManualTextureViews.html "struct bevy::prelude::ManualTextureViews"): [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/texture/manual_texture_view.rs.html#52)

#### type [Source](../render/extract_resource/trait.ExtractResource.html#associatedtype.Source) = [ManualTextureViews](struct.ManualTextureViews.html "struct bevy::prelude::ManualTextureViews")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/texture/manual_texture_view.rs.html#52)

#### fn [extract\_resource](../render/extract_resource/trait.ExtractResource.html#tymethod.extract_resource)( source: &<[ManualTextureViews](struct.ManualTextureViews.html "struct bevy::prelude::ManualTextureViews") as [ExtractResource](../render/extract_resource/trait.ExtractResource.html "trait bevy::render::extract_resource::ExtractResource")\>::[Source](../render/extract_resource/trait.ExtractResource.html#associatedtype.Source "type bevy::render::extract_resource::ExtractResource::Source"), ) -> [ManualTextureViews](struct.ManualTextureViews.html "struct bevy::prelude::ManualTextureViews")

Defines how the resource is transferred into the “render world”.

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/texture/manual_texture_view.rs.html#52)

### impl [Resource](trait.Resource.html "trait bevy::prelude::Resource") for [ManualTextureViews](struct.ManualTextureViews.html "struct bevy::prelude::ManualTextureViews")

where [ManualTextureViews](struct.ManualTextureViews.html "struct bevy::prelude::ManualTextureViews"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

## Auto Trait Implementations

### impl ![RefUnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html "trait core::panic::unwind_safe::RefUnwindSafe") for [ManualTextureViews](struct.ManualTextureViews.html "struct bevy::prelude::ManualTextureViews")

### impl ![Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin") for [ManualTextureViews](struct.ManualTextureViews.html "struct bevy::prelude::ManualTextureViews")

### impl ![UnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html "trait core::panic::unwind_safe::UnwindSafe") for [ManualTextureViews](struct.ManualTextureViews.html "struct bevy::prelude::ManualTextureViews")

### impl [Freeze](https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html "trait core::marker::Freeze") for [ManualTextureViews](struct.ManualTextureViews.html "struct bevy::prelude::ManualTextureViews")

### impl [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") for [ManualTextureViews](struct.ManualTextureViews.html "struct bevy::prelude::ManualTextureViews")

### impl [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") for [ManualTextureViews](struct.ManualTextureViews.html "struct bevy::prelude::ManualTextureViews")

### impl [UnsafeUnpin](https://doc.rust-lang.org/nightly/core/marker/trait.UnsafeUnpin.html "trait core::marker::UnsafeUnpin") for [ManualTextureViews](struct.ManualTextureViews.html "struct bevy::prelude::ManualTextureViews")

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

### impl<C> [Bundle](trait.Bundle.html "trait bevy::prelude::Bundle") for C

where C: [Component](trait.Component.html "trait bevy::prelude::Component"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/bundle/impls.rs.html#17-19)

#### fn [component\_ids](trait.Bundle.html#tymethod.component_ids)( components: &mut [ComponentsRegistrator](../ecs/component/struct.ComponentsRegistrator.html "struct bevy::ecs::component::ComponentsRegistrator")<'\_>, ) -> impl [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = [ComponentId](../ecs/component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId")\> + use<C>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/bundle/impls.rs.html#23)

#### fn [get\_component\_ids](trait.Bundle.html#tymethod.get_component_ids)( components: &[Components](../ecs/component/struct.Components.html "struct bevy::ecs::component::Components"), ) -> impl [Iterator](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator")<Item = [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[ComponentId](../ecs/component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId")\>>

Return a iterator over this [`Bundle`](trait.Bundle.html "trait bevy::prelude::Bundle")’s component ids. This will be [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") if the component has not been registered.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/bundle/impls.rs.html#30)

### impl<C> [BundleFromComponents](../ecs/bundle/trait.BundleFromComponents.html "trait bevy::ecs::bundle::BundleFromComponents") for C

where C: [Component](trait.Component.html "trait bevy::prelude::Component"),

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

#### fn [into\_any](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html#tymethod.into_any)(self: [Box](struct.Box.html "struct bevy::prelude::Box")<T>) -> [Box](struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

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

#### fn [into\_any](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html#tymethod.into_any)(self: [Box](struct.Box.html "struct bevy::prelude::Box")<T>) -> [Box](struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

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

#### fn [into\_any\_send](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.DowncastSend.html#tymethod.into_any_send)(self: [Box](struct.Box.html "struct bevy::prelude::Box")<T>) -> [Box](struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send")\>

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

where C: [Component](trait.Component.html "trait bevy::prelude::Component"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/bundle/impls.rs.html#44)

#### type [Effect](../ecs/bundle/trait.DynamicBundle.html#associatedtype.Effect) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

An operation on the entity that happens _after_ inserting this bundle.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/bundle/impls.rs.html#46-49)

#### unsafe fn [get\_components](../ecs/bundle/trait.DynamicBundle.html#tymethod.get_components)( ptr: [MovingPtr](../ecs/ptr/struct.MovingPtr.html "struct bevy::ecs::ptr::MovingPtr")<'\_, C>, func: &mut impl [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")([StorageType](../ecs/component/enum.StorageType.html "enum bevy::ecs::component::StorageType"), [OwningPtr](../ecs/ptr/struct.OwningPtr.html "struct bevy::ecs::ptr::OwningPtr")<'\_>), ) -> <C as [DynamicBundle](../ecs/bundle/trait.DynamicBundle.html "trait bevy::ecs::bundle::DynamicBundle")\>::[Effect](../ecs/bundle/trait.DynamicBundle.html#associatedtype.Effect "type bevy::ecs::bundle::DynamicBundle::Effect")

Moves the components out of the bundle. [Read more](../ecs/bundle/trait.DynamicBundle.html#tymethod.get_components)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/bundle/impls.rs.html#54)

#### unsafe fn [apply\_effect](../ecs/bundle/trait.DynamicBundle.html#tymethod.apply_effect)( \_ptr: [MovingPtr](../ecs/ptr/struct.MovingPtr.html "struct bevy::ecs::ptr::MovingPtr")<'\_, [MaybeUninit](https://doc.rust-lang.org/nightly/core/mem/maybe_uninit/union.MaybeUninit.html "union core::mem::maybe_uninit::MaybeUninit")<C>>, \_entity: &mut [EntityWorldMut](struct.EntityWorldMut.html "struct bevy::prelude::EntityWorldMut")<'\_>, )

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

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#4000)

### impl<T> [FromWorld](trait.FromWorld.html "trait bevy::prelude::FromWorld") for T

where T: [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#4003)

#### fn [from\_world](trait.FromWorld.html#tymethod.from_world)(\_world: &mut [World](struct.World.html "struct bevy::prelude::World")) -> T

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

[Source](https://doc.rust-lang.org/nightly/src/core/ops/deref.rs.html#379-381)

### impl<P, T> [Receiver](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Receiver.html "trait core::ops::deref::Receiver") for P

where P: [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref")<Target = T> + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://doc.rust-lang.org/nightly/src/core/ops/deref.rs.html#383)

#### type [Target](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Receiver.html#associatedtype.Target) = T

🔬This is a nightly-only experimental API. (`arbitrary_self_types`)

The target type on which the method may be called.

[Source](https://docs.rs/typenum/1.20.1/x86_64-unknown-linux-gnu/src/typenum/type_operators.rs.html#34)

### impl<T> [Same](https://docs.rs/typenum/1.20.1/x86_64-unknown-linux-gnu/typenum/type_operators/trait.Same.html "trait typenum::type_operators::Same") for T

[Source](https://docs.rs/typenum/1.20.1/x86_64-unknown-linux-gnu/src/typenum/type_operators.rs.html#35)

#### type [Output](https://docs.rs/typenum/1.20.1/x86_64-unknown-linux-gnu/typenum/type_operators/trait.Same.html#associatedtype.Output) = T

Should always be `Self`

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/meta.rs.html#190)

### impl<T> [Settings](../asset/meta/trait.Settings.html "trait bevy::asset::meta::Settings") for T

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

### impl<T> [ToOwned](trait.ToOwned.html "trait bevy::prelude::ToOwned") for T

where T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#76)

#### type [Owned](trait.ToOwned.html#associatedtype.Owned) = T

The resulting type after obtaining ownership.

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#77)

#### fn [to\_owned](trait.ToOwned.html#tymethod.to_owned)(&self) -> T

Creates owned data from borrowed data, usually by cloning. [Read more](trait.ToOwned.html#tymethod.to_owned)

[Source](https://doc.rust-lang.org/nightly/src/alloc/borrow.rs.html#81)

#### fn [clone\_into](trait.ToOwned.html#method.clone_into)(&self, target: [&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html))

Uses borrowed data to replace owned data, usually by cloning. [Read more](trait.ToOwned.html#method.clone_into)

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

#### fn [clone\_type\_data](../reflect/trait.TypeData.html#tymethod.clone_type_data)(&self) -> [Box](struct.Box.html "struct bevy::prelude::Box")<dyn [TypeData](../reflect/trait.TypeData.html "trait bevy::reflect::TypeData")\>

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

{"Arc<dyn Any + Send + Sync>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../platform/sync/struct.Arc.html\\" title=\\"struct bevy::platform::sync::Arc\\">Arc</a>&lt;R&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;R&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a> for <a class=\\"struct\\" href=\\"../platform/sync/struct.Arc.html\\" title=\\"struct bevy::platform::sync::Arc\\">Arc</a>&lt;R&gt;<div class=\\"where\\">where\\n R: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a> + IoHandle + ?<a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\\" title=\\"trait core::marker::Sized\\">Sized</a>,\\n <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;'a R</a>: for&lt;'a&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a>,</div></div><div class=\\"where\\">impl&lt;W&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a> for <a class=\\"struct\\" href=\\"../platform/sync/struct.Arc.html\\" title=\\"struct bevy::platform::sync::Arc\\">Arc</a>&lt;W&gt;<div class=\\"where\\">where\\n W: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a> + IoHandle + ?<a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\\" title=\\"trait core::marker::Sized\\">Sized</a>,\\n <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;'a W</a>: for&lt;'a&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a>,</div></div>","Drain<'\_, K, V, A>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../platform/collections/hash\_map/struct.Drain.html\\" title=\\"struct bevy::platform::collections::hash\_map::Drain\\">Drain</a>&lt;'\_, K, V, A&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;K, V, A&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"../platform/collections/hash\_map/struct.Drain.html\\" title=\\"struct bevy::platform::collections::hash\_map::Drain\\">Drain</a>&lt;'\_, K, V, A&gt;<div class=\\"where\\">where\\n A: <a class=\\"trait\\" href=\\"https://docs.rs/allocator-api2/0.2.21/x86\_64-unknown-linux-gnu/allocator\_api2/stable/alloc/trait.Allocator.html\\" title=\\"trait allocator\_api2::stable::alloc::Allocator\\">Allocator</a>,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.tuple.html\\">(K, V)</a>;</div>","Drain<'\_, K, V>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../platform/collections/hash\_map/struct.Drain.html\\" title=\\"struct bevy::platform::collections::hash\_map::Drain\\">Drain</a>&lt;'\_, K, V, A&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;K, V, A&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"../platform/collections/hash\_map/struct.Drain.html\\" title=\\"struct bevy::platform::collections::hash\_map::Drain\\">Drain</a>&lt;'\_, K, V, A&gt;<div class=\\"where\\">where\\n A: <a class=\\"trait\\" href=\\"https://docs.rs/allocator-api2/0.2.21/x86\_64-unknown-linux-gnu/allocator\_api2/stable/alloc/trait.Allocator.html\\" title=\\"trait allocator\_api2::stable::alloc::Allocator\\">Allocator</a>,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.tuple.html\\">(K, V)</a>;</div>","Either<Self, Self>":"<h3>Notable traits for <code><a class=\\"enum\\" href=\\"https://docs.rs/either/1.16.0/x86\_64-unknown-linux-gnu/either/enum.Either.html\\" title=\\"enum either::Either\\">Either</a>&lt;L, R&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;L, R&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"enum\\" href=\\"https://docs.rs/either/1.16.0/x86\_64-unknown-linux-gnu/either/enum.Either.html\\" title=\\"enum either::Either\\">Either</a>&lt;L, R&gt;<div class=\\"where\\">where\\n L: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>,\\n R: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&lt;Item = &lt;L as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>&gt;,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &lt;L as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>;</div><div class=\\"where\\">impl&lt;L, R&gt; <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"enum\\" href=\\"https://docs.rs/either/1.16.0/x86\_64-unknown-linux-gnu/either/enum.Either.html\\" title=\\"enum either::Either\\">Either</a>&lt;L, R&gt;<div class=\\"where\\">where\\n L: <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,\\n R: <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&lt;Output = &lt;L as <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>&gt;,</div></div><div class=\\"where\\"> type <a href=\\"../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;L as <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>","ExtractIf<'\_, K, V, F, A>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../platform/collections/hash\_map/struct.ExtractIf.html\\" title=\\"struct bevy::platform::collections::hash\_map::ExtractIf\\">ExtractIf</a>&lt;'\_, K, V, F, A&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;K, V, F, A&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"../platform/collections/hash\_map/struct.ExtractIf.html\\" title=\\"struct bevy::platform::collections::hash\_map::ExtractIf\\">ExtractIf</a>&lt;'\_, K, V, F, A&gt;<div class=\\"where\\">where\\n F: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html\\" title=\\"trait core::ops::function::FnMut\\">FnMut</a>(<a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;K</a>, <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;mut V</a>) -&gt; <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.bool.html\\">bool</a>,\\n A: <a class=\\"trait\\" href=\\"https://docs.rs/allocator-api2/0.2.21/x86\_64-unknown-linux-gnu/allocator\_api2/stable/alloc/trait.Allocator.html\\" title=\\"trait allocator\_api2::stable::alloc::Allocator\\">Allocator</a>,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.tuple.html\\">(K, V)</a>;</div>","ExtractIf<'\_, K, V, F>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../platform/collections/hash\_map/struct.ExtractIf.html\\" title=\\"struct bevy::platform::collections::hash\_map::ExtractIf\\">ExtractIf</a>&lt;'\_, K, V, F, A&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;K, V, F, A&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"../platform/collections/hash\_map/struct.ExtractIf.html\\" title=\\"struct bevy::platform::collections::hash\_map::ExtractIf\\">ExtractIf</a>&lt;'\_, K, V, F, A&gt;<div class=\\"where\\">where\\n F: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html\\" title=\\"trait core::ops::function::FnMut\\">FnMut</a>(<a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;K</a>, <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;mut V</a>) -&gt; <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.bool.html\\">bool</a>,\\n A: <a class=\\"trait\\" href=\\"https://docs.rs/allocator-api2/0.2.21/x86\_64-unknown-linux-gnu/allocator\_api2/stable/alloc/trait.Allocator.html\\" title=\\"trait allocator\_api2::stable::alloc::Allocator\\">Allocator</a>,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.tuple.html\\">(K, V)</a>;</div>","Instrumented<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../log/tracing/instrument/struct.Instrumented.html\\" title=\\"struct bevy::log::tracing::instrument::Instrumented\\">Instrumented</a>&lt;T&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;T&gt; <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"../log/tracing/instrument/struct.Instrumented.html\\" title=\\"struct bevy::log::tracing::instrument::Instrumented\\">Instrumented</a>&lt;T&gt;<div class=\\"where\\">where\\n T: <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,</div></div><div class=\\"where\\"> type <a href=\\"../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;T as <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>","Iter<'\_, K, V>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../platform/collections/hash\_map/struct.Iter.html\\" title=\\"struct bevy::platform::collections::hash\_map::Iter\\">Iter</a>&lt;'a, K, V&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a, K, V&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"../platform/collections/hash\_map/struct.Iter.html\\" title=\\"struct bevy::platform::collections::hash\_map::Iter\\">Iter</a>&lt;'a, K, V&gt;</div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = (<a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;'a K</a>, <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;'a V</a>);</div>","IterMut<'\_, K, V>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../platform/collections/hash\_map/struct.IterMut.html\\" title=\\"struct bevy::platform::collections::hash\_map::IterMut\\">IterMut</a>&lt;'a, K, V&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a, K, V&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"../platform/collections/hash\_map/struct.IterMut.html\\" title=\\"struct bevy::platform::collections::hash\_map::IterMut\\">IterMut</a>&lt;'a, K, V&gt;</div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = (<a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;'a K</a>, <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;'a mut V</a>);</div>","Keys<'\_, K, V>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../platform/collections/hash\_map/struct.Keys.html\\" title=\\"struct bevy::platform::collections::hash\_map::Keys\\">Keys</a>&lt;'a, K, V&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a, K, V&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"../platform/collections/hash\_map/struct.Keys.html\\" title=\\"struct bevy::platform::collections::hash\_map::Keys\\">Keys</a>&lt;'a, K, V&gt;</div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;'a K</a>;</div>","Values<'\_, K, V>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../platform/collections/hash\_map/struct.Values.html\\" title=\\"struct bevy::platform::collections::hash\_map::Values\\">Values</a>&lt;'a, K, V&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a, K, V&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"../platform/collections/hash\_map/struct.Values.html\\" title=\\"struct bevy::platform::collections::hash\_map::Values\\">Values</a>&lt;'a, K, V&gt;</div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;'a V</a>;</div>","ValuesMut<'\_, K, V>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../platform/collections/hash\_map/struct.ValuesMut.html\\" title=\\"struct bevy::platform::collections::hash\_map::ValuesMut\\">ValuesMut</a>&lt;'a, K, V&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a, K, V&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"../platform/collections/hash\_map/struct.ValuesMut.html\\" title=\\"struct bevy::platform::collections::hash\_map::ValuesMut\\">ValuesMut</a>&lt;'a, K, V&gt;</div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;'a mut V</a>;</div>","WithDispatch<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../log/tracing/instrument/struct.WithDispatch.html\\" title=\\"struct bevy::log::tracing::instrument::WithDispatch\\">WithDispatch</a>&lt;T&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;T&gt; <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"../log/tracing/instrument/struct.WithDispatch.html\\" title=\\"struct bevy::log::tracing::instrument::WithDispatch\\">WithDispatch</a>&lt;T&gt;<div class=\\"where\\">where\\n T: <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,</div></div><div class=\\"where\\"> type <a href=\\"../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;T as <a class=\\"trait\\" href=\\"../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>"}