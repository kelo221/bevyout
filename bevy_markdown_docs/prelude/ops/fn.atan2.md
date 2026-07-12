[bevy](../../index.html)::[prelude](../index.html)::[ops](index.html)

# Function atan2 

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/ops.rs.html#371)

```rust
pub fn atan2(y: f32, x: f32) -> f32
```

Computes the four-quadrant arctangent of `y` and `x` in radians.

*   `x = 0`, `y = 0`: `0`
*   `x >= 0`: `arctan(y/x)` -> `[-pi/2, pi/2]`
*   `y >= 0`: `arctan(y/x) + pi` -> `(pi/2, pi]`
*   `y < 0`: `arctan(y/x) - pi` -> `(-pi, -pi/2)`

Precision is specified when the `libm` feature is enabled.

##### [Examples found in repository](#scraped-examples)[?](../../../scrape-examples-help.html)

examples/shader\_advanced/render\_depth\_to\_texture.rs ([line 424](../../../src/render_depth_to_texture/render_depth_to_texture.rs.html#424))

```rust
419    fn from_cartesian(p: Vec3) -> SphericalCoordinates {
420        let radius = p.length();
421        SphericalCoordinates {
422            radius,
423            inclination: acos(p.y / radius),
424            azimuth: atan2(p.z, p.x),
425        }
426    }
```

Hide additional examples

examples/3d/light\_probe\_blending.rs ([lines 522-525](../../../src/light_probe_blending/light_probe_blending.rs.html#522-525))

```rust
497fn handle_camera_mode_change(
498    mut commands: Commands,
499    cameras_query: Query<(Entity, &Transform), With<Camera3d>>,
500    sphere_query: Query<&Transform, (With<ReflectiveSphere>, Without<Camera3d>)>,
501    mut help_text_query: Query<&mut Text, With<HelpText>>,
502    mut windows_query: Query<&mut CursorOptions>,
503    mut app_status: ResMut<AppStatus>,
504    mut messages: MessageReader<WidgetClickEvent<CameraMode>>,
505) {
506    let Some(sphere_transform) = sphere_query.iter().next() else {
507        return;
508    };
509
510    let mut any_changes = false;
511    for message in messages.read() {
512        app_status.camera_mode = **message;
513
514        match **message {
515            CameraMode::Orbit => {
516                for (camera_entity, camera_transform) in &cameras_query {
517                    // Convert from Cartesian coordinates back to spherical
518                    // coordinates.
519                    let relative_camera_position =
520                        camera_transform.translation - sphere_transform.translation;
521                    let radius = relative_camera_position.length();
522                    let inclination = atan2(
523                        relative_camera_position.xz().length() / radius,
524                        relative_camera_position.y / radius,
525                    );
526                    let azimuth = atan2(
527                        relative_camera_position.z * relative_camera_position.xz().length_recip(),
528                        relative_camera_position.x * relative_camera_position.xz().length_recip(),
529                    );
530
531                    commands
532                        .entity(camera_entity)
533                        .remove::<FreeCamera>()
534                        .insert(OrbitCamera {
535                            radius,
536                            inclination,
537                            azimuth,
538                        });
539                }
540            }
541
542            CameraMode::Free => {
543                for (camera_entity, _) in &cameras_query {
544                    commands
545                        .entity(camera_entity)
546                        .remove::<OrbitCamera>()
547                        .insert(FreeCamera::default());
548                }
549            }
550        }
551
552        any_changes = true;
553    }
554
555    if any_changes {
556        set_help_text(&app_status, &mut help_text_query);
557
558        // Reset the cursor grab mode, because the free camera controller may
559        // have enabled it, and we don't want the cursor to disappear.
560        for mut cursor_options in &mut windows_query {
561            cursor_options.grab_mode = CursorGrabMode::None;
562            cursor_options.visible = true;
563        }
564    }
565}
```