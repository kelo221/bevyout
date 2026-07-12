[bevy](../index.html)::[math](index.html)

# Function uvec2 

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/u32/uvec2.rs.html#17)

```rust
pub const fn uvec2(x: u32, y: u32) -> UVec2
```

Creates a 2-dimensional vector.

##### [Examples found in repository](#scraped-examples)[?](../../scrape-examples-help.html)

examples/2d/dynamic\_mip\_generation.rs ([line 754](../../src/dynamic_mip_generation/dynamic_mip_generation.rs.html#754))

```rust
753    fn image_size_u32(&self) -> UVec2 {
754        uvec2(self.image_width as u32, self.image_height as u32)
755    }
```

Hide additional examples

examples/3d/mirror.rs ([line 241](../../src/mirror/mirror.rs.html#241))

```rust
235fn create_mirror_texture_resource(
236    commands: &mut Commands,
237    windows_query: &Query<&Window>,
238    images: &mut Assets<Image>,
239) -> Handle<Image> {
240    let window = windows_query.iter().next().expect("No window found");
241    let window_size = uvec2(window.physical_width(), window.physical_height());
242    let image = create_mirror_texture_image(images, window_size);
243    commands.insert_resource(MirrorImage(image.clone()));
244    image
245}
246
247/// Spawns the camera that renders the mirror world.
248fn spawn_mirror_camera(
249    commands: &mut Commands,
250    camera_transform: &Transform,
251    camera_projection: &PerspectiveProjection,
252    mirror_transform: &Transform,
253    mirror_render_target: Handle<Image>,
254) {
255    let (mirror_camera_transform, mirror_camera_projection) =
256        calculate_mirror_camera_transform_and_projection(
257            camera_transform,
258            camera_projection,
259            mirror_transform,
260        );
261
262    commands.spawn((
263        Camera3d::default(),
264        Camera {
265            order: -1,
266            // Reflecting the model across the mirror will flip the winding of
267            // all the polygons. Therefore, in order to properly backface cull,
268            // we need to turn on `invert_culling`.
269            invert_culling: true,
270            ..default()
271        },
272        RenderTarget::Image(mirror_render_target.clone().into()),
273        mirror_camera_transform,
274        Projection::Perspective(mirror_camera_projection),
275        MirrorCamera,
276    ));
277}
278
279/// Spawns the animated fox.
280///
281/// Note that this doesn't play the animation; that's handled in
282/// [`play_fox_animation`].
283fn spawn_fox(commands: &mut Commands, asset_server: &AssetServer) {
284    commands.spawn((
285        WorldAssetRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset(FOX_ASSET_PATH))),
286        Transform::from_xyz(-50.0, 0.0, -100.0),
287    ));
288}
289
290/// Spawns the mirror plane mesh and returns its transform.
291fn spawn_mirror(
292    commands: &mut Commands,
293    meshes: &mut Assets<Mesh>,
294    screen_space_texture_materials: &mut Assets<
295        ExtendedMaterial<StandardMaterial, ScreenSpaceTextureExtension>,
296    >,
297    mirror_render_target: Handle<Image>,
298) -> Transform {
299    let mirror_transform = Transform::from_scale(vec3(300.0, 1.0, 150.0))
300        .with_rotation(Quat::from_rotation_x(MIRROR_ROTATION_ANGLE))
301        .with_translation(MIRROR_POSITION);
302
303    commands.spawn((
304        Mesh3d(meshes.add(Plane3d::default().mesh().size(1.0, 1.0))),
305        MeshMaterial3d(screen_space_texture_materials.add(ExtendedMaterial {
306            base: StandardMaterial {
307                base_color: Color::BLACK,
308                emissive: Color::WHITE.into(),
309                emissive_texture: Some(mirror_render_target),
310                perceptual_roughness: 0.0,
311                metallic: 1.0,
312                ..default()
313            },
314            extension: ScreenSpaceTextureExtension { dummy: 0.0 },
315        })),
316        mirror_transform,
317        Mirror,
318    ));
319
320    mirror_transform
321}
322
323/// Spawns the buttons at the bottom of the screen.
324fn spawn_buttons(commands: &mut Commands) {
325    // Spawn the radio buttons that allow the user to select an object to
326    // control.
327    commands.spawn((
328        widgets::main_ui_node(),
329        children![widgets::option_buttons(
330            "Drag Action",
331            &[
332                (DragAction::MoveCamera, "Move Camera"),
333                (DragAction::MoveFox, "Move Fox"),
334            ],
335        )],
336    ));
337}
338
339/// Given the transform and projection of the main camera, returns an
340/// appropriate transform and projection for the mirror camera.
341fn calculate_mirror_camera_transform_and_projection(
342    main_camera_transform: &Transform,
343    main_camera_projection: &PerspectiveProjection,
344    mirror_transform: &Transform,
345) -> (Transform, PerspectiveProjection) {
346    // Calculate the reflection matrix (a.k.a. Householder matrix) that will
347    // reflect the scene across the mirror plane.
348    //
349    // Note that you must calculate this in *matrix* form and only *afterward*
350    // convert to a `Transform` instead of composing `Transform`s. This is
351    // because the reflection matrix has non-uniform scale, and composing
352    // transforms can't always handle composition of matrices with non-uniform
353    // scales.
354    let mirror_camera_transform = Transform::from_matrix(
355        Mat4::from_mat3a(reflection_matrix(Vec3::NEG_Z)) * main_camera_transform.to_matrix(),
356    );
357
358    // Compute the distance from the camera to the mirror plane. This will be
359    // used to calculate the distance to the near clip plane for the mirror
360    // world.
361    let distance_from_camera_to_mirror = InfinitePlane3d::new(mirror_transform.rotation * Vec3::Y)
362        .signed_distance(
363            Isometry3d::IDENTITY,
364            mirror_transform.translation - main_camera_transform.translation,
365        );
366
367    // Compute the normal of the mirror plane in view space.
368    let view_from_world = main_camera_transform.compute_affine().matrix3.inverse();
369    let mirror_projection_plane_normal =
370        (view_from_world * (mirror_transform.rotation * Vec3::NEG_Y)).normalize();
371
372    // Compute the final projection. It should match the main camera projection,
373    // except that `near` and `near_normal` should be set to the updated near
374    // plane and near normal plane as above.
375    let mirror_camera_projection = PerspectiveProjection {
376        near_clip_plane: mirror_projection_plane_normal.extend(distance_from_camera_to_mirror),
377        ..*main_camera_projection
378    };
379
380    (mirror_camera_transform, mirror_camera_projection)
381}
382
383/// A system that resizes the render target image when the user resizes the window.
384///
385/// Since the image that stores the rendered mirror world has the same physical
386/// size as the window, we need to reallocate it and reattach it to the mirror
387/// material whenever the window size changes.
388fn handle_window_resize_messages(
389    windows_query: Query<&Window>,
390    mut mirror_cameras_query: Query<&mut RenderTarget, With<MirrorCamera>>,
391    mut images: ResMut<Assets<Image>>,
392    mut mirror_image: ResMut<MirrorImage>,
393    mut screen_space_texture_materials: ResMut<
394        Assets<ExtendedMaterial<StandardMaterial, ScreenSpaceTextureExtension>>,
395    >,
396    mut resize_messages: MessageReader<WindowResized>,
397) {
398    // We run at most once, regardless of the number of window resize messages
399    // there were this frame.
400    let Some(resize_message) = resize_messages.read().next() else {
401        return;
402    };
403    let Ok(window) = windows_query.get(resize_message.window) else {
404        return;
405    };
406
407    let window_size = uvec2(window.physical_width(), window.physical_height());
408    let image = create_mirror_texture_image(&mut images, window_size);
409    images.remove(mirror_image.0.id());
410
411    mirror_image.0 = image.clone();
412
413    for mut target in mirror_cameras_query.iter_mut() {
414        *target = image.clone().into();
415    }
416
417    for (_, material) in screen_space_texture_materials.iter_mut() {
418        material.base.emissive_texture = Some(image.clone());
419    }
420}
```