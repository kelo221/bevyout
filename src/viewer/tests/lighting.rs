use super::*;
use bevy::prelude::Ref;

#[test]
fn status_reports_prepared_memory_without_a_runtime_cache() {
    let mut world = World::new();
    world.insert_resource(PreparedPointShadowRuntime {
        resolution: 256,
        layers: 11,
        ..default()
    });
    world.insert_resource(PointLightShadowSamples(1));

    let status = shadow_cache_status(&mut world);
    assert_eq!(status["realtime_shadows_enabled"], false);
    assert_eq!(status["estimated_memory_bytes"], 17_301_504_u64);
    assert_eq!(status["runtime_shadow_passes"], 0);
}

#[test]
fn disabled_realtime_shadows_turn_off_every_candidate() {
    let mut app = App::new();
    app.insert_resource(RealtimeShadowSettings::default())
        .insert_resource(RealtimeShadowLight::default())
        .add_systems(Update, apply_realtime_shadow_light);
    app.world_mut()
        .spawn((Camera3d::default(), GlobalTransform::default()));
    let light = app
        .world_mut()
        .spawn((
            RealtimeShadowCandidate {
                reference_form_id: 1,
            },
            PointLight {
                shadow_maps_enabled: true,
                ..default()
            },
            GlobalTransform::default(),
        ))
        .id();

    app.update();

    assert!(
        app.world()
            .get::<PointLight>(light)
            .is_some_and(|light| { !light.shadow_maps_enabled })
    );
    assert_eq!(app.world().resource::<RealtimeShadowLight>().0, None);
}

#[test]
fn strongest_camera_candidate_uses_local_contribution_not_spawn_order() {
    let far = Entity::from_bits(1);
    let near = Entity::from_bits(2);
    let outside = Entity::from_bits(3);
    assert_eq!(
        strongest_camera_candidate(
            Vec3::ZERO,
            [
                (far, 10, Vec3::new(0.0, 0.0, 3.0), 4.0, 8.0),
                (near, 20, Vec3::new(0.0, 0.0, 1.0), 4.0, 8.0),
                (outside, 5, Vec3::new(0.0, 0.0, 5.0), 4.0, 100.0),
            ],
        ),
        Some(near)
    );
}

#[test]
fn strongest_camera_candidate_tie_breaks_by_reference_form_id() {
    let high_form = Entity::from_bits(7);
    let low_form = Entity::from_bits(8);
    assert_eq!(
        strongest_camera_candidate(
            Vec3::ZERO,
            [
                (high_form, 20, Vec3::Z, 4.0, 8.0),
                (low_form, 10, Vec3::Z, 4.0, 8.0),
            ],
        ),
        Some(low_form)
    );
}

#[test]
fn strongest_camera_candidate_falls_back_to_nearest_light_sphere() {
    let near = Entity::from_bits(9);
    let far = Entity::from_bits(10);
    assert_eq!(
        strongest_camera_candidate(
            Vec3::ZERO,
            [
                (far, 10, Vec3::new(0.0, 0.0, 12.0), 2.0, 100.0),
                (near, 20, Vec3::new(0.0, 0.0, 5.0), 3.0, 1.0),
            ],
        ),
        Some(near)
    );
}

#[test]
fn non_finite_candidate_intensities_are_ignored() {
    let entity = Entity::from_bits(7);
    assert_eq!(
        strongest_camera_candidate(Vec3::ZERO, [(entity, 1, Vec3::ZERO, 4.0, f32::NAN)],),
        None
    );
}

// ---------------------------------------------------------------------
// Issue #267 (PERF wave 1): the disabled realtime-shadow path -- the
// default configuration, since the pass is opt-in -- must perform zero
// mutable writes on a steady frame, and transition frames must perform
// exactly the needed writes.
// ---------------------------------------------------------------------

/// Records `PointLight`/selection change-detection hits observed *after*
/// `apply_realtime_shadow_light` each frame. A frame with zero hits is
/// provably write-free: Bevy change detection fires only on deref-mut.
#[derive(Resource, Default)]
struct ShadowWriteProbe {
    light_writes: usize,
    selection_writes: usize,
}

fn probe_shadow_writes(
    lights: Query<Ref<PointLight>, With<RealtimeShadowCandidate>>,
    selected: Res<RealtimeShadowLight>,
    mut probe: ResMut<ShadowWriteProbe>,
) {
    for light in &lights {
        if light.is_changed() {
            probe.light_writes += 1;
        }
    }
    if selected.is_changed() {
        probe.selection_writes += 1;
    }
}

fn realtime_shadow_test_app(enabled: bool) -> App {
    let mut app = App::new();
    app.insert_resource(RealtimeShadowSettings { enabled })
        .init_resource::<RealtimeShadowLight>()
        .init_resource::<ShadowWriteProbe>()
        .add_systems(
            Update,
            (apply_realtime_shadow_light, probe_shadow_writes).chain(),
        );
    app
}

fn spawn_shadow_camera(app: &mut App, position: Vec3) -> Entity {
    app.world_mut()
        .spawn((
            Camera3d::default(),
            GlobalTransform::from_translation(position),
        ))
        .id()
}

/// Spawns a candidate exactly the way the prepared-scene spawn site does:
/// `shadow_maps_enabled: false`; only the shadow system ever flips the flag.
fn spawn_shadow_candidate(app: &mut App, form_id: u32, position: Vec3, intensity: f32) -> Entity {
    app.world_mut()
        .spawn((
            RealtimeShadowCandidate {
                reference_form_id: form_id,
            },
            PointLight {
                intensity,
                range: 4.0,
                shadow_maps_enabled: false,
                ..default()
            },
            GlobalTransform::from_translation(position),
        ))
        .id()
}

/// Clears the probe, runs one frame, and returns that frame's observed
/// `(PointLight, selection)` change-detection hits.
fn write_probe_frame(app: &mut App) -> (usize, usize) {
    {
        let mut probe = app.world_mut().resource_mut::<ShadowWriteProbe>();
        probe.light_writes = 0;
        probe.selection_writes = 0;
    }
    app.update();
    let probe = app.world().resource::<ShadowWriteProbe>();
    (probe.light_writes, probe.selection_writes)
}

fn set_realtime_shadows_enabled(app: &mut App, enabled: bool) {
    app.world_mut()
        .resource_mut::<RealtimeShadowSettings>()
        .enabled = enabled;
}

#[test]
fn disabled_steady_frame_writes_nothing() {
    let mut app = realtime_shadow_test_app(false);
    spawn_shadow_camera(&mut app, Vec3::ZERO);
    spawn_shadow_candidate(&mut app, 1, Vec3::ZERO, 8.0);
    // Absorb creation ticks; the settings-insertion frame is outside the
    // steady-state guarantee.
    app.update();
    app.update();

    for frame in 0..3 {
        assert_eq!(
            write_probe_frame(&mut app),
            (0, 0),
            "disabled steady frame {frame} fired change detection"
        );
    }
    assert_eq!(app.world().resource::<RealtimeShadowLight>().0, None);
}

#[test]
fn disabling_after_enable_pays_one_conditional_cleanup() {
    let mut app = realtime_shadow_test_app(true);
    spawn_shadow_camera(&mut app, Vec3::ZERO);
    let light_a = spawn_shadow_candidate(&mut app, 1, Vec3::ZERO, 8.0);
    spawn_shadow_candidate(&mut app, 2, Vec3::new(50.0, 0.0, 0.0), 8.0);

    // Settle the enabled selection; a steady enabled frame is write-free too.
    app.update();
    app.update();
    assert_eq!(
        app.world().resource::<RealtimeShadowLight>().0,
        Some(light_a)
    );
    assert!(
        app.world()
            .get::<PointLight>(light_a)
            .is_some_and(|light| light.shadow_maps_enabled)
    );
    assert_eq!(write_probe_frame(&mut app), (0, 0));

    // The disable toggle writes the previously enabled light and the
    // selection record exactly once -- the untouched candidate must keep
    // its change tick.
    set_realtime_shadows_enabled(&mut app, false);
    assert_eq!(
        write_probe_frame(&mut app),
        (1, 1),
        "disable transition must write only the enabled light and the selection"
    );
    assert_eq!(app.world().resource::<RealtimeShadowLight>().0, None);
    assert!(
        app.world()
            .get::<PointLight>(light_a)
            .is_some_and(|light| !light.shadow_maps_enabled)
    );

    for frame in 0..3 {
        assert_eq!(
            write_probe_frame(&mut app),
            (0, 0),
            "steady disabled frame {frame} after the toggle must be write-free"
        );
    }
}

#[test]
fn enabled_selection_follows_the_camera() {
    let mut app = realtime_shadow_test_app(true);
    let camera = spawn_shadow_camera(&mut app, Vec3::ZERO);
    let light_a = spawn_shadow_candidate(&mut app, 1, Vec3::ZERO, 8.0);
    let light_b = spawn_shadow_candidate(&mut app, 2, Vec3::new(10.0, 0.0, 0.0), 8.0);

    app.update();
    app.update();
    assert_eq!(
        app.world().resource::<RealtimeShadowLight>().0,
        Some(light_a)
    );
    assert!(
        app.world()
            .get::<PointLight>(light_a)
            .is_some_and(|light| light.shadow_maps_enabled)
    );
    assert!(
        app.world()
            .get::<PointLight>(light_b)
            .is_some_and(|light| !light.shadow_maps_enabled)
    );

    // The camera moves into light B's volume. The per-frame candidate rescan
    // is deliberate -- this flip is the behavior that requires it while the
    // opt-in is engaged -- and it must still write only the two affected
    // lights exactly once.
    *app.world_mut()
        .get_mut::<GlobalTransform>(camera)
        .expect("camera transform") = GlobalTransform::from_translation(Vec3::new(10.0, 0.0, 0.0));
    assert_eq!(
        write_probe_frame(&mut app),
        (2, 1),
        "a selection flip must write only the two affected lights"
    );
    assert_eq!(
        app.world().resource::<RealtimeShadowLight>().0,
        Some(light_b)
    );
    assert!(
        app.world()
            .get::<PointLight>(light_a)
            .is_some_and(|light| !light.shadow_maps_enabled)
    );
    assert!(
        app.world()
            .get::<PointLight>(light_b)
            .is_some_and(|light| light.shadow_maps_enabled)
    );

    // Once the selection has followed, engaged frames are quiet again.
    assert_eq!(write_probe_frame(&mut app), (0, 0));
}

#[test]
fn enabled_selection_follows_intensity_changes() {
    let mut app = realtime_shadow_test_app(true);
    spawn_shadow_camera(&mut app, Vec3::ZERO);
    // Both volumes contain the camera with symmetric geometry, so intensity
    // decides dominance; equal strength falls back to the lower FormID.
    let light_a = spawn_shadow_candidate(&mut app, 1, Vec3::new(0.0, 0.0, 1.0), 8.0);
    let light_b = spawn_shadow_candidate(&mut app, 2, Vec3::new(0.0, 0.0, -1.0), 8.0);

    app.update();
    app.update();
    assert_eq!(
        app.world().resource::<RealtimeShadowLight>().0,
        Some(light_a)
    );

    app.world_mut()
        .get_mut::<PointLight>(light_b)
        .expect("light B")
        .intensity = 64.0;
    app.update();
    assert_eq!(
        app.world().resource::<RealtimeShadowLight>().0,
        Some(light_b),
        "an authored intensity change must move the realtime pass"
    );
    assert!(
        app.world()
            .get::<PointLight>(light_b)
            .is_some_and(|light| light.shadow_maps_enabled)
    );
    assert!(
        app.world()
            .get::<PointLight>(light_a)
            .is_some_and(|light| !light.shadow_maps_enabled)
    );
}
