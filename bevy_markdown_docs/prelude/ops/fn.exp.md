[bevy](../../index.html)::[prelude](../index.html)::[ops](index.html)

# Function exp 

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/ops.rs.html#251)

```rust
pub fn exp(x: f32) -> f32
```

Returns `e^(self)`, (the exponential function).

Precision is specified when the `libm` feature is enabled.

##### [Examples found in repository](#scraped-examples)[?](../../../scrape-examples-help.html)

examples/3d/transmission.rs ([line 537](../../../src/transmission/transmission.rs.html#537))

```rust
375fn example_control_system(
376    mut commands: Commands,
377    mut materials: ResMut<Assets<StandardMaterial>>,
378    controllable: Query<(&MeshMaterial3d<StandardMaterial>, &ExampleControls)>,
379    camera: Single<
380        (
381            Entity,
382            &mut ScreenSpaceTransmission,
383            &mut Transform,
384            Option<&DepthPrepass>,
385            Option<&TemporalJitter>,
386            Has<Hdr>,
387        ),
388        With<Camera3d>,
389    >,
390    mut display: Single<&mut Text, With<ExampleDisplay>>,
391    mut state: Local<ExampleState>,
392    time: Res<Time>,
393    input: Res<ButtonInput<KeyCode>>,
394) {
395    if input.pressed(KeyCode::Digit2) {
396        state.diffuse_transmission = (state.diffuse_transmission + time.delta_secs()).min(1.0);
397    } else if input.pressed(KeyCode::Digit1) {
398        state.diffuse_transmission = (state.diffuse_transmission - time.delta_secs()).max(0.0);
399    }
400
401    if input.pressed(KeyCode::KeyW) {
402        state.specular_transmission = (state.specular_transmission + time.delta_secs()).min(1.0);
403    } else if input.pressed(KeyCode::KeyQ) {
404        state.specular_transmission = (state.specular_transmission - time.delta_secs()).max(0.0);
405    }
406
407    if input.pressed(KeyCode::KeyS) {
408        state.thickness = (state.thickness + time.delta_secs()).min(5.0);
409    } else if input.pressed(KeyCode::KeyA) {
410        state.thickness = (state.thickness - time.delta_secs()).max(0.0);
411    }
412
413    if input.pressed(KeyCode::KeyX) {
414        state.ior = (state.ior + time.delta_secs()).min(3.0);
415    } else if input.pressed(KeyCode::KeyZ) {
416        state.ior = (state.ior - time.delta_secs()).max(1.0);
417    }
418
419    if input.pressed(KeyCode::KeyI) {
420        state.reflectance = (state.reflectance + time.delta_secs()).min(1.0);
421    } else if input.pressed(KeyCode::KeyU) {
422        state.reflectance = (state.reflectance - time.delta_secs()).max(0.0);
423    }
424
425    if input.pressed(KeyCode::KeyR) {
426        state.perceptual_roughness = (state.perceptual_roughness + time.delta_secs()).min(1.0);
427    } else if input.pressed(KeyCode::KeyE) {
428        state.perceptual_roughness = (state.perceptual_roughness - time.delta_secs()).max(0.0);
429    }
430
431    let randomize_colors = input.just_pressed(KeyCode::KeyC);
432
433    for (material_handle, controls) in &controllable {
434        let mut material = materials.get_mut(material_handle).unwrap();
435        if controls.specular_transmission {
436            material.specular_transmission = state.specular_transmission;
437            material.thickness = state.thickness;
438            material.ior = state.ior;
439            material.perceptual_roughness = state.perceptual_roughness;
440            material.reflectance = state.reflectance;
441        }
442
443        if controls.diffuse_transmission {
444            material.diffuse_transmission = state.diffuse_transmission;
445        }
446
447        if controls.color && randomize_colors {
448            material.base_color =
449                Color::srgba(random(), random(), random(), material.base_color.alpha());
450        }
451    }
452
453    let (
454        camera_entity,
455        mut transmission,
456        mut camera_transform,
457        depth_prepass,
458        temporal_jitter,
459        hdr,
460    ) = camera.into_inner();
461
462    if input.just_pressed(KeyCode::KeyH) {
463        if hdr {
464            commands.entity(camera_entity).remove::<Hdr>();
465        } else {
466            commands.entity(camera_entity).insert(Hdr);
467        }
468    }
469
470    #[cfg(any(feature = "webgpu", not(target_arch = "wasm32")))]
471    if input.just_pressed(KeyCode::KeyD) {
472        if depth_prepass.is_none() {
473            commands.entity(camera_entity).insert(DepthPrepass);
474        } else {
475            commands.entity(camera_entity).remove::<DepthPrepass>();
476        }
477    }
478
479    #[cfg(any(feature = "webgpu", not(target_arch = "wasm32")))]
480    if input.just_pressed(KeyCode::KeyT) {
481        if temporal_jitter.is_none() {
482            commands
483                .entity(camera_entity)
484                .insert((TemporalJitter::default(), TemporalAntiAliasing::default()));
485        } else {
486            commands
487                .entity(camera_entity)
488                .remove::<(TemporalJitter, TemporalAntiAliasing)>();
489        }
490    }
491
492    if input.just_pressed(KeyCode::KeyO) && transmission.steps > 0 {
493        transmission.steps -= 1;
494    }
495
496    if input.just_pressed(KeyCode::KeyP) && transmission.steps < 4 {
497        transmission.steps += 1;
498    }
499
500    if input.just_pressed(KeyCode::KeyJ) {
501        transmission.quality = ScreenSpaceTransmissionQuality::Low;
502    }
503
504    if input.just_pressed(KeyCode::KeyK) {
505        transmission.quality = ScreenSpaceTransmissionQuality::Medium;
506    }
507
508    if input.just_pressed(KeyCode::KeyL) {
509        transmission.quality = ScreenSpaceTransmissionQuality::High;
510    }
511
512    if input.just_pressed(KeyCode::Semicolon) {
513        transmission.quality = ScreenSpaceTransmissionQuality::Ultra;
514    }
515
516    let rotation = if input.pressed(KeyCode::ArrowRight) {
517        state.auto_camera = false;
518        time.delta_secs()
519    } else if input.pressed(KeyCode::ArrowLeft) {
520        state.auto_camera = false;
521        -time.delta_secs()
522    } else if state.auto_camera {
523        time.delta_secs() * 0.25
524    } else {
525        0.0
526    };
527
528    let distance_change =
529        if input.pressed(KeyCode::ArrowDown) && camera_transform.translation.length() < 25.0 {
530            time.delta_secs()
531        } else if input.pressed(KeyCode::ArrowUp) && camera_transform.translation.length() > 2.0 {
532            -time.delta_secs()
533        } else {
534            0.0
535        };
536
537    camera_transform.translation *= ops::exp(distance_change);
538
539    camera_transform.rotate_around(
540        Vec3::ZERO,
541        Quat::from_euler(EulerRot::XYZ, 0.0, rotation, 0.0),
542    );
543
544    display.0 = format!(
545        concat!(
546            " J / K / L / ;  Screen Space Specular Transmissive Quality: {:?}\n",
547            "         O / P  Screen Space Specular Transmissive Steps: {}\n",
548            "         1 / 2  Diffuse Transmission: {:.2}\n",
549            "         Q / W  Specular Transmission: {:.2}\n",
550            "         A / S  Thickness: {:.2}\n",
551            "         Z / X  IOR: {:.2}\n",
552            "         E / R  Perceptual Roughness: {:.2}\n",
553            "         U / I  Reflectance: {:.2}\n",
554            "    Arrow Keys  Control Camera\n",
555            "             C  Randomize Colors\n",
556            "             H  HDR + Bloom: {}\n",
557            "             D  Depth Prepass: {}\n",
558            "             T  TAA: {}\n",
559        ),
560        transmission.quality,
561        transmission.steps,
562        state.diffuse_transmission,
563        state.specular_transmission,
564        state.thickness,
565        state.ior,
566        state.perceptual_roughness,
567        state.reflectance,
568        if hdr { "ON " } else { "OFF" },
569        if cfg!(any(feature = "webgpu", not(target_arch = "wasm32"))) {
570            if depth_prepass.is_some() {
571                "ON "
572            } else {
573                "OFF"
574            }
575        } else {
576            "N/A (WebGL)"
577        },
578        if cfg!(any(feature = "webgpu", not(target_arch = "wasm32"))) {
579            if temporal_jitter.is_some() {
580                if depth_prepass.is_some() {
581                    "ON "
582                } else {
583                    "N/A (Needs Depth Prepass)"
584                }
585            } else {
586                "OFF"
587            }
588        } else {
589            "N/A (WebGL)"
590        },
591    );
592}
```