[bevy](../../index.html)::[prelude](../index.html)::[ops](index.html)

# Function powf 

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/ops.rs.html#243)

```rust
pub fn powf(x: f32, y: f32) -> f32
```

Raises a number to a floating point power.

Precision is specified when the `libm` feature is enabled.

##### [Examples found in repository](#scraped-examples)[?](../../../scrape-examples-help.html)

examples/math/custom\_primitives.rs ([line 421](../../../src/custom_primitives/custom_primitives.rs.html#421))

```rust
420    fn perimeter(&self) -> f32 {
421        self.radius * (2.5 * PI + ops::powf(2f32, 1.5) + 2.0)
422    }
423
424    fn area(&self) -> f32 {
425        let circle_area = PI * self.radius * self.radius;
426        let triangle_area = self.radius * self.radius * (1.0 + 2f32.sqrt()) / 2.0;
427        let cutout = triangle_area - circle_area * 3.0 / 16.0;
428
429        2.0 * circle_area + 4.0 * cutout
430    }
431}
432
433// The `Bounded2d` or `Bounded3d` traits are used to compute the Axis Aligned Bounding Boxes or bounding circles / spheres for primitives.
434impl Bounded2d for Heart {
435    fn aabb_2d(&self, isometry: impl Into<Isometry2d>) -> Aabb2d {
436        let isometry = isometry.into();
437
438        // The center of the circle at the center of the right wing of the heart
439        let circle_center = isometry.rotation * Vec2::new(self.radius, 0.0);
440        // The maximum X and Y positions of the two circles of the wings of the heart.
441        let max_circle = circle_center.abs() + Vec2::splat(self.radius);
442        // Since the two circles of the heart are mirrored around the origin, the minimum position is the negative of the maximum.
443        let min_circle = -max_circle;
444
445        // The position of the tip at the bottom of the heart
446        let tip_position = isometry.rotation * Vec2::new(0.0, -self.radius * (1. + SQRT_2));
447
448        Aabb2d {
449            min: isometry.translation + min_circle.min(tip_position),
450            max: isometry.translation + max_circle.max(tip_position),
451        }
452    }
453
454    fn bounding_circle(&self, isometry: impl Into<Isometry2d>) -> BoundingCircle {
455        let isometry = isometry.into();
456
457        // The bounding circle of the heart is not at its origin. This `offset` is the offset between the center of the bounding circle and its translation.
458        let offset = self.radius / ops::powf(2f32, 1.5);
459        // The center of the bounding circle
460        let center = isometry * Vec2::new(0.0, -offset);
461        // The radius of the bounding circle
462        let radius = self.radius * (1.0 + 2f32.sqrt()) - offset;
463
464        BoundingCircle::new(center, radius)
465    }
```

Hide additional examples

examples/ui/ui\_scaling.rs ([line 139](../../../src/ui_scaling/ui_scaling.rs.html#139))

```rust
135fn ease_in_expo(x: f32) -> f32 {
136    if x == 0. {
137        0.
138    } else {
139        ops::powf(2.0f32, 5. * x - 5.)
140    }
141}
```

examples/audio/pitch.rs ([line 47](../../../src/pitch/pitch.rs.html#47))

```rust
41fn keyboard_input_system(
42    keyboard_input: Res<ButtonInput<KeyCode>>,
43    mut frequency: ResMut<PitchFrequency>,
44    mut play_pitch_writer: MessageWriter<PlayPitch>,
45) {
46    if keyboard_input.just_pressed(KeyCode::ArrowUp) {
47        frequency.0 *= ops::powf(2.0f32, 1.0 / 12.0);
48    }
49    if keyboard_input.just_pressed(KeyCode::ArrowDown) {
50        frequency.0 /= ops::powf(2.0f32, 1.0 / 12.0);
51    }
52    if keyboard_input.just_pressed(KeyCode::Space) {
53        play_pitch_writer.write(PlayPitch);
54    }
55}
```

examples/showcase/desk\_toy.rs ([line 369](../../../src/desk_toy/desk_toy.rs.html#369))

```rust
360fn move_pupils(time: Res<Time>, mut q_pupils: Query<(&mut Pupil, &mut Transform)>) {
361    for (mut pupil, mut transform) in &mut q_pupils {
362        // The wiggle radius is how much the pupil can move within the eye
363        let wiggle_radius = pupil.eye_radius - pupil.pupil_radius;
364        // Store the Z component
365        let z = transform.translation.z;
366        // Truncate the Z component to make the calculations be on [`Vec2`]
367        let mut translation = transform.translation.truncate();
368        // Decay the pupil velocity
369        pupil.velocity *= ops::powf(0.04f32, time.delta_secs());
370        // Move the pupil
371        translation += pupil.velocity * time.delta_secs();
372        // If the pupil hit the outside border of the eye, limit the translation to be within the wiggle radius and invert the velocity.
373        // This is not physically accurate but it's good enough for the googly eyes effect.
374        if translation.length() > wiggle_radius {
375            translation = translation.normalize() * wiggle_radius;
376            // Invert and decrease the velocity of the pupil when it bounces
377            pupil.velocity *= -0.75;
378        }
379        // Update the entity transform with the new translation after reading the Z component
380        transform.translation = translation.extend(z);
381    }
382}
```

examples/2d/2d\_viewport\_to\_world.rs ([line 71](../../../src/2d_viewport_to_world/2d_viewport_to_world.rs.html#71))

```rust
42fn controls(
43    camera_query: Single<(&mut Camera, &mut Transform, &mut Projection)>,
44    window: Single<&Window>,
45    input: Res<ButtonInput<KeyCode>>,
46    time: Res<Time<Fixed>>,
47) {
48    let (mut camera, mut transform, mut projection) = camera_query.into_inner();
49
50    let fspeed = 600.0 * time.delta_secs();
51    let uspeed = fspeed as u32;
52    let window_size = window.resolution.physical_size();
53
54    // Camera movement controls
55    if input.pressed(KeyCode::ArrowUp) {
56        transform.translation.y += fspeed;
57    }
58    if input.pressed(KeyCode::ArrowDown) {
59        transform.translation.y -= fspeed;
60    }
61    if input.pressed(KeyCode::ArrowLeft) {
62        transform.translation.x -= fspeed;
63    }
64    if input.pressed(KeyCode::ArrowRight) {
65        transform.translation.x += fspeed;
66    }
67
68    // Camera zoom controls
69    if let Projection::Orthographic(projection2d) = &mut *projection {
70        if input.pressed(KeyCode::Comma) {
71            projection2d.scale *= powf(4.0f32, time.delta_secs());
72        }
73
74        if input.pressed(KeyCode::Period) {
75            projection2d.scale *= powf(0.25f32, time.delta_secs());
76        }
77    }
78
79    if let Some(viewport) = camera.viewport.as_mut() {
80        // Reset viewport size on window resize
81        if viewport.physical_size.x > window_size.x || viewport.physical_size.y > window_size.y {
82            viewport.physical_size = (window_size.as_vec2() * 0.75).as_uvec2();
83        }
84
85        // Viewport movement controls
86        if input.pressed(KeyCode::KeyW) {
87            viewport.physical_position.y = viewport.physical_position.y.saturating_sub(uspeed);
88        }
89        if input.pressed(KeyCode::KeyS) {
90            viewport.physical_position.y += uspeed;
91        }
92        if input.pressed(KeyCode::KeyA) {
93            viewport.physical_position.x = viewport.physical_position.x.saturating_sub(uspeed);
94        }
95        if input.pressed(KeyCode::KeyD) {
96            viewport.physical_position.x += uspeed;
97        }
98
99        // Bound viewport position so it doesn't go off-screen
100        viewport.physical_position = viewport
101            .physical_position
102            .min(window_size - viewport.physical_size);
103
104        // Viewport size controls
105        if input.pressed(KeyCode::KeyI) {
106            viewport.physical_size.y = viewport.physical_size.y.saturating_sub(uspeed);
107        }
108        if input.pressed(KeyCode::KeyK) {
109            viewport.physical_size.y += uspeed;
110        }
111        if input.pressed(KeyCode::KeyJ) {
112            viewport.physical_size.x = viewport.physical_size.x.saturating_sub(uspeed);
113        }
114        if input.pressed(KeyCode::KeyL) {
115            viewport.physical_size.x += uspeed;
116        }
117
118        // Bound viewport size so it doesn't go off-screen
119        viewport.physical_size = viewport
120            .physical_size
121            .min(window_size - viewport.physical_position)
122            .max(UVec2::new(20, 20));
123    }
124}
```

examples/camera/2d\_screen\_shake.rs ([line 109](../../../src/2d_screen_shake/2d_screen_shake.rs.html#109))

```rust
72fn shake_camera(
73    camera_shake: Single<(&mut CameraShakeState, &CameraShakeConfig, &mut Transform)>,
74    time: Res<Time>,
75) {
76    let (mut camera_shake, config, mut transform) = camera_shake.into_inner();
77
78    // Before we even start thinking about the shake, we save the original transform so it's not lost.
79    // At the start of the next frame, we will restore the camera's transform to this original transform.
80    camera_shake.original_transform = *transform;
81
82    // To generate the transform offset, we use a noise function. Noise is like a random number generator, but cooler.
83    // Let's start with a visual intuition: <https://assets-global.website-files.com/64b6d182aee713bd0401f4b9/64b95974ec292aabac45fc8e_image.png>
84    // The image on the left is made from pure randomness, the image on the right is made from a kind of noise called Perlin noise.
85    // Notice how the noise has much more "structure" than the randomness? How it looks like it has peaks and valleys?
86    // This property makes noise very desirable for a variety of visual effects. In our case, what we want is that the
87    // camera does not wildly teleport around the world, but instead *moves* through the world frame by frame.
88    // We can use 1D Perlin noise for this, which takes one input and outputs a value between -1.0 and 1.0. If we increase the input by a little bit,
89    // like by the time since the last frame, we get a different output that is still "close" to the previous one.
90
91    // This is the input to the noise function. Just using the elapsed time is pretty good input,
92    // since it means that noise generations that are close in time will be close in output.
93    // We simply multiply it by a constant to be able to "speed up" or "slow down" the noise.
94    let t = time.elapsed_secs() * config.noise_speed;
95
96    // Now we generate three noise values. One for the rotation, one for the x-offset, and one for the y-offset.
97    // But if we generated those three noise values with the same input, we would get the same output three times!
98    // To avoid this, we simply add a random offset to each input.
99    // You can think of this as the seed value you would give a random number generator.
100    let rotation_noise = perlin_noise::generate(t + 0.0);
101    let x_noise = perlin_noise::generate(t + 100.0);
102    let y_noise = perlin_noise::generate(t + 200.0);
103
104    // Games often deal with linear increments. For example, if an enemy deals 10 damage and attacks you 2 times, you will take 20 damage.
105    // But that's not how impact feels! Human senses are much more attuned to exponential changes.
106    // So, we make sure that the `shake` value we use is an exponential function of the trauma.
107    // But doesn't this make the value explode? Fortunately not: since `trauma` is between 0.0 and 1.0, exponentiating it will actually make it smaller!
108    // See <https://www.wolframalpha.com/input?i=plot+x+and+x%5E2+and+x%5E3+for+x+in+%5B0%2C+1%5D> for a graph.
109    let shake = powf(camera_shake.trauma, config.exponent);
110
111    // Now, to get the final offset, we multiply this noise value by the shake value and the maximum value.
112    // The noise value is in [-1, 1], so by multiplying it with a maximum value, we get a value in [-max_value, +max_value].
113    // Multiply this by the shake value to get the exponential effect, and we're done!
114    let roll_offset = rotation_noise * shake * config.max_angle;
115    let x_offset = x_noise * shake * config.max_translation;
116    let y_offset = y_noise * shake * config.max_translation;
117
118    // Finally, we apply the offset to the camera's transform. Since we already stored the original transform,
119    // and this system runs right at the end of the frame, we can't accidentally break any game logic by changing the transform.
120    transform.translation.x += x_offset;
121    transform.translation.y += y_offset;
122    transform.rotate_z(roll_offset);
123
124    // Some bookkeeping at the end: trauma should decay over time.
125    camera_shake.trauma -= config.trauma_decay_per_second * time.delta_secs();
126    camera_shake.trauma = camera_shake.trauma.clamp(0.0, 1.0);
127}
```