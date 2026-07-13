//! FPS movement and kinematic controller systems.

use super::*;

#[allow(clippy::too_many_arguments)]
pub(crate) fn apply_player_controls(
    keys: Res<ButtonInput<KeyCode>>,
    state: Res<CameraModeState>,
    physics_disabled: Res<PhysicsDisabled>,
    time: Res<Time<Fixed>>,
    mut context: NonSendMut<BoxdddPhysicsContext>,
    mut players: Query<(
        &FpsPlayer,
        &mut Transform,
        &mut KccState,
        &mut LocomotionState,
    )>,
) {
    let Ok((player, mut transform, mut kcc, mut locomotion)) = players.single_mut() else {
        return;
    };
    let jump_pressed = keys.pressed(KeyCode::Space);
    let jump_started = jump_pressed && !locomotion.jump_was_pressed();
    locomotion.set_jump_pressed(jump_pressed);
    if physics_disabled.0 || state.mode != CameraMode::Fps {
        kcc.velocity = Vec3::ZERO;
        kcc.grounded = false;
        return;
    }
    let Some(world) = context.world_mut() else {
        return;
    };

    let dt = time.delta_secs();
    let mover = boxddd::Capsule::new(
        [0.0, -(CAPSULE_HEIGHT * 0.5 - CAPSULE_RADIUS), 0.0],
        [0.0, CAPSULE_HEIGHT * 0.5 - CAPSULE_RADIUS, 0.0],
        CAPSULE_RADIUS,
    );
    let filter = boxddd::QueryFilter::new()
        .category_bits(PLAYER_QUERY)
        .mask_bits(WORLD_STATIC | WORLD_DYNAMIC);
    let origin = to_box_vec3(transform.translation);
    let initial_planes = world
        .collide_mover(origin, &mover, filter)
        .unwrap_or_default();
    let mut grounded = has_walkable_plane(&initial_planes);
    kcc.grounded = grounded;

    let yaw = Quat::from_rotation_y(player.yaw);
    let mut input = Vec3::ZERO;
    if keys.pressed(KeyCode::KeyW) {
        input -= Vec3::Z;
    }
    if keys.pressed(KeyCode::KeyS) {
        input += Vec3::Z;
    }
    if keys.pressed(KeyCode::KeyD) {
        input += Vec3::X;
    }
    if keys.pressed(KeyCode::KeyA) {
        input -= Vec3::X;
    }
    let world_input = yaw * input;
    let ground_target = air_control_motion(world_input, false) * PLAYER_SPEED;
    if jump_started && grounded {
        let (height, direction) = jump_profile(world_input);
        kcc.velocity.y = (2.0 * GRAVITY * height).sqrt();
        if let Some(direction) = direction {
            let airtime = 2.0 * kcc.velocity.y / GRAVITY;
            let inherited_speed = Vec3::new(kcc.velocity.x, 0.0, kcc.velocity.z)
                .dot(direction)
                .max(0.0);
            let distance_speed = DIRECTIONAL_JUMP_HORIZONTAL_DISTANCE / airtime;
            let launch_speed = inherited_speed
                .max(ground_target.length())
                .max(distance_speed);
            kcc.velocity.x = direction.x * launch_speed;
            kcc.velocity.z = direction.z * launch_speed;
        } else {
            kcc.velocity.x = ground_target.x;
            kcc.velocity.z = ground_target.z;
        }
        locomotion.mark_jump_started();
        grounded = false;
    }

    if grounded {
        kcc.velocity.x = ground_target.x;
        kcc.velocity.z = ground_target.z;
        if kcc.velocity.y < 0.0 {
            kcc.velocity.y = 0.0;
        }
    } else {
        // Air control changes the horizontal velocity gradually. It no longer
        // treats the reduced OpenMW air-control factor as a lower terminal
        // speed, which used to make a forward jump visibly lose momentum.
        let air_velocity = apply_air_control(
            Vec3::new(kcc.velocity.x, 0.0, kcc.velocity.z),
            world_input,
            dt,
        );
        kcc.velocity.x = air_velocity.x;
        kcc.velocity.z = air_velocity.z;
        kcc.velocity.y -= GRAVITY * dt;
    }

    let desired_delta = to_box_vec3(kcc.velocity * dt);
    let (mut position, planes) = move_mover(world, origin, &mover, desired_delta, filter);
    grounded = has_walkable_plane(&planes);
    if !grounded && kcc.velocity.y <= 0.0 {
        let snap = to_box_vec3(Vec3::new(0.0, -GROUND_SNAP_DISTANCE, 0.0));
        let fraction = world
            .cast_mover(position, &mover, snap, filter)
            .unwrap_or(1.0)
            .clamp(0.0, 1.0);
        if fraction < 1.0 {
            position = add_box_vec3(position, scale_box_vec3(snap, fraction));
            grounded = true;
        }
    }
    if grounded && kcc.velocity.y < 0.0 {
        kcc.velocity.y = 0.0;
    }
    kcc.grounded = grounded;
    transform.translation = from_box_vec3(position);
}

pub(crate) fn move_mover(
    world: &mut boxddd::World,
    mut position: boxddd::Vec3,
    mover: &boxddd::Capsule,
    mut remaining: boxddd::Vec3,
    filter: boxddd::QueryFilter,
) -> (boxddd::Vec3, Vec<boxddd::MoverPlane>) {
    for _ in 0..MAX_SLIDE_PASSES {
        if box_vec_length_squared(remaining) <= f32::EPSILON {
            break;
        }
        let fraction = world
            .cast_mover(position, mover, remaining, filter)
            .unwrap_or(1.0)
            .clamp(0.0, 1.0);
        position = add_box_vec3(position, scale_box_vec3(remaining, fraction));
        remaining = scale_box_vec3(remaining, 1.0 - fraction);
        let planes = world
            .collide_mover(position, mover, filter)
            .unwrap_or_default();
        if planes.is_empty() {
            break;
        }
        if planes
            .iter()
            .any(|plane| plane.plane.normal.y < WALKABLE_SLOPE_COS)
            && (remaining.x * remaining.x + remaining.z * remaining.z) > f32::EPSILON
            && let Some(stepped) = try_step_up(world, position, mover, remaining, filter)
        {
            position = stepped;
            break;
        }
        let mut solver_planes = planes
            .iter()
            .filter_map(|plane| CollisionPlane::new(plane.plane, STEP_HEIGHT, true).ok())
            .collect::<Vec<_>>();
        if let Ok(correction) = boxddd::solve_planes(boxddd::Vec3::ZERO, &mut solver_planes) {
            position = add_box_vec3(position, correction.delta);
        }
        remaining = boxddd::clip_vector(remaining, &solver_planes).unwrap_or(boxddd::Vec3::ZERO);
        if fraction >= 1.0 && box_vec_length_squared(remaining) <= f32::EPSILON {
            break;
        }
    }
    let planes = world
        .collide_mover(position, mover, filter)
        .unwrap_or_default();
    (position, planes)
}

pub(crate) fn try_step_up(
    world: &mut boxddd::World,
    position: boxddd::Vec3,
    mover: &boxddd::Capsule,
    remaining: boxddd::Vec3,
    filter: boxddd::QueryFilter,
) -> Option<boxddd::Vec3> {
    let up = boxddd::Vec3::new(0.0, STEP_HEIGHT, 0.0);
    if world.cast_mover(position, mover, up, filter).ok()? < 1.0 {
        return None;
    }
    let elevated = add_box_vec3(position, up);
    let horizontal = boxddd::Vec3::new(remaining.x, 0.0, remaining.z);
    let horizontal_fraction = world.cast_mover(elevated, mover, horizontal, filter).ok()?;
    let elevated = add_box_vec3(elevated, scale_box_vec3(horizontal, horizontal_fraction));
    let down = boxddd::Vec3::new(0.0, -STEP_HEIGHT, 0.0);
    let down_fraction = world.cast_mover(elevated, mover, down, filter).ok()?;
    if down_fraction >= 1.0 {
        return None;
    }
    let stepped = add_box_vec3(elevated, scale_box_vec3(down, down_fraction));
    let planes = world.collide_mover(stepped, mover, filter).ok()?;
    has_walkable_plane(&planes).then_some(stepped)
}
