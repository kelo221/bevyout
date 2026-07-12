[bevy](../../index.html)::[prelude](../index.html)::[ops](index.html)

# Function acos 

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/ops.rs.html#349)

```rust
pub fn acos(x: f32) -> f32
```

Computes the arccosine of a number. Return value is in radians in Hyperbolic tangent function.

Precision is specified when the `libm` feature is enabled. the range \[0, pi\] or NaN if the number is outside the range \[-1, 1\].

Precision is specified when the `libm` feature is enabled.

##### [Examples found in repository](#scraped-examples)[?](../../../scrape-examples-help.html)

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

Hide additional examples

examples/shader\_advanced/render\_depth\_to\_texture.rs ([line 423](../../../src/render_depth_to_texture/render_depth_to_texture.rs.html#423))

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

examples/stress\_tests/many\_lights.rs ([line 123](../../../src/many_lights/many_lights.rs.html#123))

```rust
120fn fibonacci_spiral_on_sphere(golden_ratio: f64, i: usize, n: usize) -> DVec2 {
121    DVec2::new(
122        PI * 2. * (i as f64 / golden_ratio),
123        ops::acos((1.0 - 2.0 * (i as f64 + EPSILON) / (n as f64 - 1.0 + 2.0 * EPSILON)) as f32)
124            as f64,
125    )
126}
```

examples/3d/clustered\_decals.rs ([line 406](../../../src/clustered_decals/clustered_decals.rs.html#406))

```rust
386fn process_move_input(
387    mut selections: Query<(&mut Transform, &Selection)>,
388    mouse_buttons: Res<ButtonInput<MouseButton>>,
389    mouse_motion: Res<AccumulatedMouseMotion>,
390    app_status: Res<AppStatus>,
391) {
392    // Only process drags when movement is selected.
393    if !mouse_buttons.pressed(MouseButton::Left) || app_status.drag_mode != DragMode::Move {
394        return;
395    }
396
397    for (mut transform, selection) in &mut selections {
398        if app_status.selection != *selection {
399            continue;
400        }
401
402        let position = transform.translation;
403
404        // Convert to spherical coordinates.
405        let radius = position.length();
406        let mut theta = acos(position.y / radius);
407        let mut phi = position.z.signum() * acos(position.x * position.xz().length_recip());
408
409        // Camera movement is the inverse of object movement.
410        let (phi_factor, theta_factor) = match *selection {
411            Selection::Camera => (1.0, -1.0),
412            Selection::DecalA | Selection::DecalB => (-1.0, 1.0),
413        };
414
415        // Adjust the spherical coordinates. Clamp the inclination to (0, π).
416        phi += phi_factor * mouse_motion.delta.x * MOVE_SPEED;
417        theta = f32::clamp(
418            theta + theta_factor * mouse_motion.delta.y * MOVE_SPEED,
419            0.001,
420            PI - 0.001,
421        );
422
423        // Convert spherical coordinates back to Cartesian coordinates.
424        transform.translation =
425            radius * vec3(sin(theta) * cos(phi), cos(theta), sin(theta) * sin(phi));
426
427        // Look at the center, but preserve the previous roll angle.
428        let roll = transform.rotation.to_euler(EulerRot::YXZ).2;
429        transform.look_at(Vec3::ZERO, Vec3::Y);
430        let (yaw, pitch, _) = transform.rotation.to_euler(EulerRot::YXZ);
431        transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, roll);
432    }
433}
```

examples/3d/light\_textures.rs ([line 488](../../../src/light_textures/light_textures.rs.html#488))

```rust
461fn process_move_input(
462    mut selections: Query<(&mut Transform, &Selection)>,
463    mouse_buttons: Res<ButtonInput<MouseButton>>,
464    mouse_motion: Res<AccumulatedMouseMotion>,
465    app_status: Res<AppStatus>,
466) {
467    // Only process drags when movement is selected.
468    if !mouse_buttons.pressed(MouseButton::Left) || app_status.drag_mode != DragMode::Move {
469        return;
470    }
471
472    for (mut transform, selection) in &mut selections {
473        if app_status.selection != *selection {
474            continue;
475        }
476
477        // use simple movement for the point light
478        if *selection == Selection::PointLight {
479            transform.translation +=
480                (mouse_motion.delta * Vec2::new(1.0, -1.0) * MOVE_SPEED).extend(0.0);
481            return;
482        }
483
484        let position = transform.translation;
485
486        // Convert to spherical coordinates.
487        let radius = position.length();
488        let mut theta = acos(position.y / radius);
489        let mut phi = position.z.signum() * acos(position.x * position.xz().length_recip());
490
491        // Camera movement is the inverse of object movement.
492        let (phi_factor, theta_factor) = match *selection {
493            Selection::Camera => (1.0, -1.0),
494            _ => (-1.0, 1.0),
495        };
496
497        // Adjust the spherical coordinates. Clamp the inclination to (0, π).
498        phi += phi_factor * mouse_motion.delta.x * MOVE_SPEED;
499        theta = f32::clamp(
500            theta + theta_factor * mouse_motion.delta.y * MOVE_SPEED,
501            0.001,
502            PI - 0.001,
503        );
504
505        // Convert spherical coordinates back to Cartesian coordinates.
506        transform.translation =
507            radius * vec3(sin(theta) * cos(phi), cos(theta), sin(theta) * sin(phi));
508
509        // Look at the center, but preserve the previous roll angle.
510        let roll = transform.rotation.to_euler(EulerRot::YXZ).2;
511        transform.look_at(Vec3::ZERO, Vec3::Y);
512        let (yaw, pitch, _) = transform.rotation.to_euler(EulerRot::YXZ);
513        transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, roll);
514    }
515}
```

examples/2d/rotation.rs ([line 237](../../../src/rotation/rotation.rs.html#237))

```rust
195fn rotate_to_player_system(
196    time: Res<Time>,
197    mut query: Query<(&RotateToPlayer, &mut Transform), Without<Player>>,
198    player_transform: Single<&Transform, With<Player>>,
199) {
200    // Get the player translation in 2D
201    let player_translation = player_transform.translation.xy();
202
203    for (config, mut enemy_transform) in &mut query {
204        // Get the enemy ship forward vector in 2D (already unit length)
205        let enemy_forward = (enemy_transform.rotation * Vec3::Y).xy();
206
207        // Get the vector from the enemy ship to the player ship in 2D and normalize it.
208        let to_player = (player_translation - enemy_transform.translation.xy()).normalize();
209
210        // Get the dot product between the enemy forward vector and the direction to the player.
211        let forward_dot_player = enemy_forward.dot(to_player);
212
213        // If the dot product is approximately 1.0 then the enemy is already facing the player and
214        // we can early out.
215        if (forward_dot_player - 1.0).abs() < f32::EPSILON {
216            continue;
217        }
218
219        // Get the right vector of the enemy ship in 2D (already unit length)
220        let enemy_right = (enemy_transform.rotation * Vec3::X).xy();
221
222        // Get the dot product of the enemy right vector and the direction to the player ship.
223        // If the dot product is negative them we need to rotate counter clockwise, if it is
224        // positive we need to rotate clockwise. Note that `copysign` will still return 1.0 if the
225        // dot product is 0.0 (because the player is directly behind the enemy, so perpendicular
226        // with the right vector).
227        let right_dot_player = enemy_right.dot(to_player);
228
229        // Determine the sign of rotation from the right dot player. We need to negate the sign
230        // here as the 2D bevy co-ordinate system rotates around +Z, which is pointing out of the
231        // screen. Due to the right hand rule, positive rotation around +Z is counter clockwise and
232        // negative is clockwise.
233        let rotation_sign = -f32::copysign(1.0, right_dot_player);
234
235        // Limit rotation so we don't overshoot the target. We need to convert our dot product to
236        // an angle here so we can get an angle of rotation to clamp against.
237        let max_angle = ops::acos(forward_dot_player.clamp(-1.0, 1.0)); // Clamp acos for safety
238
239        // Calculate angle of rotation with limit
240        let rotation_angle =
241            rotation_sign * (config.rotation_speed * time.delta_secs()).min(max_angle);
242
243        // Rotate the enemy to face the player
244        enemy_transform.rotate_z(rotation_angle);
245    }
246}
```