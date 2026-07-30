use std::time::Duration;

use bevy::animation::{AnimatedBy, AnimationTargetId, animated_field};
use bevy::ecs::system::RunSystemOnce;
use bevy::math::curve::EaseFunction;
use bevy::time::TimeUpdateStrategy;

use super::*;

#[test]
fn form_ids_accept_plain_and_prefixed_hex() {
    assert_eq!(parse_form_id("00041606").unwrap(), 0x0004_1606);
    assert_eq!(parse_form_id("0x00041606").unwrap(), 0x0004_1606);
    assert!(parse_form_id("raider").is_err());
}

#[test]
fn animation_node_names_bridge_niftools_side_suffixes() {
    assert_eq!(animation_node_name_key("Bip01 Calf.L"), "bip01 l calf");
    assert_eq!(animation_node_name_key("Bip01 L Calf"), "bip01 l calf");
    assert_eq!(
        animation_node_name_key("Bip01 Finger1.R"),
        "bip01 r finger1"
    );
    assert_eq!(animation_node_name_key("Weapon"), "weapon");
}

#[test]
fn aabb_min_y_uses_all_transformed_corners() {
    let global = GlobalTransform::from(Transform::from_rotation(Quat::from_rotation_z(
        std::f32::consts::FRAC_PI_2,
    )));
    let aabb = Aabb {
        center: Vec3A::ZERO,
        half_extents: Vec3A::new(2.0, 1.0, 3.0),
    };
    assert!((aabb_min_y(&global, &aabb) + 2.0).abs() < 1e-5);
}

#[test]
fn partial_aim_clips_resolve_to_their_full_body_pair() {
    let clip = PreparedActorAnimationClip {
        name: "1hmaimdown".into(),
        animated_target_count: 6,
        ..default()
    };
    assert_eq!(layer_base_clip_name(&clip).as_deref(), Some("1hmaim"));
    let clip = PreparedActorAnimationClip {
        name: "1hmattackspinup".into(),
        animated_target_count: 7,
        ..default()
    };
    assert_eq!(
        layer_base_clip_name(&clip).as_deref(),
        Some("1hmattackspin")
    );
}

#[test]
fn retarget_global_transform_preserves_target_rest_pose() {
    let source_rest = Mat4::from_scale_rotation_translation(
        Vec3::splat(0.5),
        Quat::from_rotation_y(0.25),
        Vec3::new(0.0, 1.0, 2.0),
    );
    let target_rest = Mat4::from_scale_rotation_translation(
        Vec3::splat(2.0),
        Quat::from_rotation_x(-0.4),
        Vec3::new(3.0, -1.0, 0.5),
    );
    let desired = retarget_global_transform(target_rest, source_rest, source_rest);
    let (scale, rotation, translation) = desired.to_scale_rotation_translation();
    assert!(scale.abs_diff_eq(Vec3::splat(2.0), 1e-5));
    assert!(rotation.abs_diff_eq(Quat::from_rotation_x(-0.4), 1e-5));
    assert!(translation.abs_diff_eq(Vec3::new(3.0, -1.0, 0.5), 1e-5));
}

#[test]
fn retarget_global_transform_applies_source_delta_in_target_space() {
    let source_rest = Mat4::from_translation(Vec3::new(0.0, 1.0, 0.0));
    let source_current = Mat4::from_translation(Vec3::new(0.0, 2.0, 0.0));
    let target_rest = Mat4::from_translation(Vec3::new(4.0, 0.0, 0.0));
    let desired = retarget_global_transform(target_rest, source_rest, source_current);
    let (_, _, translation) = desired.to_scale_rotation_translation();
    assert!(translation.abs_diff_eq(Vec3::new(4.0, 1.0, 0.0), 1e-5));
}

#[test]
fn bridge_controls_queue_deterministic_actions() {
    let mut world = World::new();
    world.init_resource::<AnimationZooRuntime>();
    queue_agent_control(&mut world, "next").unwrap();
    assert_eq!(
        world.resource::<AnimationZooRuntime>().pending_controls,
        [ZooControlAction::Next]
    );
    queue_agent_control(&mut world, "select:12").unwrap();
    assert_eq!(
        world.resource::<AnimationZooRuntime>().pending_controls,
        [ZooControlAction::Next, ZooControlAction::Select(12)]
    );
    assert!(queue_agent_control(&mut world, "dance").is_err());
}

#[test]
fn pose_restoration_resets_actor_root_and_bones() {
    let mut world = World::new();
    let root = world.spawn(Transform::from_xyz(1.0, 2.0, 3.0)).id();
    let bone = world.spawn(Transform::from_rotation(Quat::IDENTITY)).id();
    let cache = HashMap::from([
        (root, *world.get::<Transform>(root).unwrap()),
        (bone, *world.get::<Transform>(bone).unwrap()),
    ]);
    world.get_mut::<Transform>(root).unwrap().translation = Vec3::splat(9.0);
    world.get_mut::<Transform>(bone).unwrap().rotation = Quat::from_rotation_x(1.0);
    world
        .run_system_once(move |mut transforms: Query<&mut Transform>| {
            restore_bind_pose(&cache, &mut transforms);
        })
        .unwrap();
    assert_eq!(
        world.get::<Transform>(root).unwrap().translation,
        Vec3::new(1.0, 2.0, 3.0)
    );
    assert_eq!(
        world.get::<Transform>(bone).unwrap().rotation,
        Quat::IDENTITY
    );
}

#[test]
fn external_clip_targets_the_prepared_actor_hierarchy() {
    let mut app = App::new();
    app.add_plugins((
        MinimalPlugins,
        AssetPlugin::default(),
        TransformPlugin,
        AnimationPlugin,
    ));
    app.insert_resource(TimeUpdateStrategy::ManualDuration(Duration::from_millis(
        100,
    )));
    let target_name = Name::new("Bip01 Spine");
    let target_id = AnimationTargetId::from_name(&target_name);
    let mut clip = AnimationClip::default();
    let curve = EasingCurve::new(Vec3::ZERO, Vec3::Y, EaseFunction::Linear)
        .reparametrize_linear(interval(0.0, 1.0).unwrap())
        .unwrap();
    clip.add_curve_to_target(
        target_id,
        AnimatableCurve::new(animated_field!(Transform::translation), curve),
    );
    let clip_handle = app
        .world_mut()
        .resource_mut::<Assets<AnimationClip>>()
        .add(clip);
    let (graph, node) = AnimationGraph::from_clip(clip_handle);
    let graph = app
        .world_mut()
        .resource_mut::<Assets<AnimationGraph>>()
        .add(graph);
    let mut player = AnimationPlayer::default();
    player.play(node);
    let actor = app
        .world_mut()
        .spawn((player, AnimationGraphHandle(graph), Transform::default()))
        .id();
    let bone = app
        .world_mut()
        .spawn((
            target_name,
            target_id,
            AnimatedBy(actor),
            Transform::default(),
        ))
        .id();
    app.world_mut().entity_mut(actor).add_child(bone);
    app.finish();
    for _ in 0..5 {
        app.update();
    }
    assert!(app.world().get::<Transform>(bone).unwrap().translation.y > 0.0);
}
