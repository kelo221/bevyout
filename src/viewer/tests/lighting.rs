use super::*;

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
