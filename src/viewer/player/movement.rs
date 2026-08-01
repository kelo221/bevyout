//! FPS movement and kinematic controller systems.

use super::*;

use crate::viewer::world::exterior::{ExteriorWaterState, SwimmingState};

const STEP_DEBUG_LOG_INTERVAL: f64 = 0.2;
const STEP_SUPPORT_FORWARD_OFFSET: f32 = CAPSULE_RADIUS;
const STEP_SUPPORT_MID_FORWARD_OFFSET: f32 = CAPSULE_RADIUS * 0.5;
const STEP_SUPPORT_FAR_FORWARD_OFFSET: f32 = CAPSULE_RADIUS * 2.0;
const STEP_SUPPORT_MAX_FORWARD_OFFSET: f32 = CAPSULE_RADIUS * 3.0;
const STEP_SUPPORT_LATERAL_OFFSET: f32 = CAPSULE_RADIUS * 0.5;

pub(crate) fn player_collision_filter() -> boxddd::QueryFilter {
    boxddd::QueryFilter::new()
        .category_bits(PLAYER_QUERY)
        .mask_bits(WORLD_STATIC | WORLD_DYNAMIC)
}

pub(crate) fn stair_support_filter() -> boxddd::QueryFilter {
    boxddd::QueryFilter::new()
        .category_bits(PLAYER_QUERY)
        .mask_bits(STEP_SUPPORT)
}

macro_rules! step_debug {
    ($enabled:expr, $($arg:tt)*) => {
        if $enabled {
            info!(target: "bevyout::stair_debug", $($arg)*);
        }
    };
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn apply_player_controls(
    keys: Res<ButtonInput<KeyCode>>,
    state: Res<CameraModeState>,
    physics_disabled: Res<PhysicsDisabled>,
    no_clip: Res<PlayerNoClip>,
    cell_physics: Res<CellPhysicsReadiness>,
    mut step_debug: ResMut<StepDebugSettings>,
    water: Res<ExteriorWaterState>,
    mut swimming: ResMut<SwimmingState>,
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
    if !cell_physics.static_collision_ready() {
        kcc.velocity = Vec3::ZERO;
        kcc.grounded = false;
        locomotion.set_jump_pressed(false);
        return;
    }
    let jump_pressed = keys.pressed(KeyCode::Space);
    let jump_started = jump_pressed && !locomotion.jump_was_pressed();
    locomotion.set_jump_pressed(jump_pressed);
    if state.mode != CameraMode::Fps {
        kcc.velocity = Vec3::ZERO;
        kcc.grounded = false;
        return;
    }
    if no_clip.0 {
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
        let mut world_input = yaw * input;
        if keys.pressed(KeyCode::Space) {
            world_input += Vec3::Y;
        }
        if keys.pressed(KeyCode::ControlLeft) || keys.pressed(KeyCode::ControlRight) {
            world_input -= Vec3::Y;
        }
        if world_input != Vec3::ZERO {
            transform.translation += world_input.normalize() * PLAYER_SPEED * time.delta_secs();
        }
        kcc.velocity = Vec3::ZERO;
        kcc.grounded = false;
        locomotion.set_jump_pressed(false);
        return;
    }
    if physics_disabled.0 {
        kcc.velocity = Vec3::ZERO;
        kcc.grounded = false;
        return;
    }
    let Some(world) = context.world_mut() else {
        return;
    };

    let dt = time.delta_secs();
    let submerged = water.contact.is_some_and(|contact| contact.submerged);
    swimming.submerged = submerged;
    if submerged {
        swimming.breath_seconds = (swimming.breath_seconds - dt).max(0.0);
    } else {
        swimming.breath_seconds =
            (swimming.breath_seconds + dt * 1.5).min(swimming.max_breath_seconds);
    }
    let mover = boxddd::Capsule::new(
        [0.0, -(CAPSULE_HEIGHT * 0.5 - CAPSULE_RADIUS), 0.0],
        [0.0, CAPSULE_HEIGHT * 0.5 - CAPSULE_RADIUS, 0.0],
        CAPSULE_RADIUS,
    );
    let collision_filter = player_collision_filter();
    let support_filter = stair_support_filter();
    let origin = to_box_vec3(transform.translation);
    let was_grounded = kcc.grounded;
    let initial_planes = world
        .collide_mover(origin, &mover, collision_filter)
        .unwrap_or_default();
    // Preserve support for one solve while crossing a tread edge. The final
    // downward sweep below decides whether that support still exists.
    let mut grounded = was_grounded || has_walkable_plane(&initial_planes);

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
    let mut jumped_this_tick = false;
    if submerged {
        // Swimming is still driven by the same BoxDDD capsule. Only the
        // movement policy changes: horizontal input is damped, gravity is
        // replaced by a bounded buoyancy/sink term, and jump/crouch become
        // surface-relative vertical controls.
        kcc.velocity.x = ground_target.x * 0.65;
        kcc.velocity.z = ground_target.z * 0.65;
        let vertical_input = if jump_pressed {
            PLAYER_SPEED * 0.45
        } else if keys.pressed(KeyCode::ControlLeft) || keys.pressed(KeyCode::ControlRight) {
            -PLAYER_SPEED * 0.45
        } else {
            water
                .contact
                .map(|contact| (contact.surface_height - transform.translation.y) * 0.75)
                .unwrap_or(0.0)
        };
        kcc.velocity.y = vertical_input.clamp(-PLAYER_SPEED * 0.45, PLAYER_SPEED * 0.45);
        grounded = false;
    } else if jump_started && grounded {
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
        jumped_this_tick = true;
    }

    if submerged {
        // The submerged branch already chose the complete velocity.
    } else if grounded {
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
    let intentional_horizontal_motion =
        ground_target.x * ground_target.x + ground_target.z * ground_target.z > f32::EPSILON;
    let keep_grounded = grounded && !jumped_this_tick;
    let debug_now = step_debug.enabled && time.elapsed_secs_f64() >= step_debug.next_log_at;
    let (mut position, planes, stepped_up, step_attempted) = move_mover(
        world,
        origin,
        &mover,
        desired_delta,
        collision_filter,
        support_filter,
        keep_grounded && intentional_horizontal_motion,
        debug_now,
    );
    if debug_now && step_attempted {
        step_debug.next_log_at = time.elapsed_secs_f64() + STEP_DEBUG_LOG_INTERVAL;
    }
    grounded = stepped_up || has_walkable_plane(&planes);
    if !grounded && keep_grounded && kcc.velocity.y <= 0.0 {
        if intentional_horizontal_motion
            && let Some(supported) =
                try_forward_step_support(world, position, desired_delta, support_filter)
        {
            position = supported;
            grounded = true;
        } else if let Some(snapped) = try_step_down(
            world,
            position,
            &mover,
            desired_delta,
            collision_filter,
            support_filter,
        ) {
            position = snapped;
            grounded = true;
        }
    }
    if grounded && kcc.velocity.y < 0.0 {
        kcc.velocity.y = 0.0;
    }
    kcc.grounded = grounded;
    transform.translation = from_box_vec3(position);
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn move_mover(
    world: &mut boxddd::World,
    mut position: boxddd::Vec3,
    mover: &boxddd::Capsule,
    mut remaining: boxddd::Vec3,
    collision_filter: boxddd::QueryFilter,
    support_filter: boxddd::QueryFilter,
    allow_step_up: bool,
    step_debug_enabled: bool,
) -> (boxddd::Vec3, Vec<boxddd::MoverPlane>, bool, bool) {
    let mut stepped_up = false;
    let mut step_attempted = false;
    for _ in 0..MAX_SLIDE_PASSES {
        if box_vec_length_squared(remaining) <= f32::EPSILON {
            break;
        }
        let fraction = world
            .cast_mover(position, mover, remaining, collision_filter)
            .unwrap_or(1.0)
            .clamp(0.0, 1.0);
        position = add_box_vec3(position, scale_box_vec3(remaining, fraction));
        remaining = scale_box_vec3(remaining, 1.0 - fraction);
        let planes = world
            .collide_mover(position, mover, collision_filter)
            .unwrap_or_default();
        if planes.is_empty() {
            break;
        }
        let has_blocking_plane = planes
            .iter()
            .any(|plane| plane.plane.normal.y < WALKABLE_SLOPE_COS);
        let has_horizontal_remainder =
            (remaining.x * remaining.x + remaining.z * remaining.z) > f32::EPSILON;
        if allow_step_up && has_blocking_plane && has_horizontal_remainder {
            step_attempted = true;
            let normals = step_debug_enabled.then(|| {
                planes
                    .iter()
                    .map(|plane| {
                        let normal = plane.plane.normal;
                        [normal.x, normal.y, normal.z]
                    })
                    .collect::<Vec<_>>()
            });
            step_debug!(
                step_debug_enabled,
                "[stair-debug] attempt position=({:.4},{:.4},{:.4}) remaining=({:.4},{:.4},{:.4}) contacts={:?}",
                position.x,
                position.y,
                position.z,
                remaining.x,
                remaining.y,
                remaining.z,
                normals.unwrap_or_default()
            );
            if let Some(stepped) = try_step_up(
                world,
                position,
                mover,
                remaining,
                collision_filter,
                support_filter,
                step_debug_enabled,
            ) {
                position = stepped;
                stepped_up = true;
                break;
            }
            step_debug!(
                step_debug_enabled,
                "[stair-debug] rejected final: candidate failed; using ordinary plane sliding"
            );
        } else if allow_step_up
            && step_debug_enabled
            && has_blocking_plane
            && !has_horizontal_remainder
        {
            step_debug!(
                true,
                "[stair-debug] not attempted position=({:.4},{:.4},{:.4}): no horizontal displacement remains",
                position.x,
                position.y,
                position.z
            );
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
        .collide_mover(position, mover, collision_filter)
        .unwrap_or_default();
    (position, planes, stepped_up, step_attempted)
}

pub(crate) fn try_step_up(
    world: &mut boxddd::World,
    position: boxddd::Vec3,
    mover: &boxddd::Capsule,
    remaining: boxddd::Vec3,
    collision_filter: boxddd::QueryFilter,
    support_filter: boxddd::QueryFilter,
    step_debug_enabled: bool,
) -> Option<boxddd::Vec3> {
    let up = boxddd::Vec3::new(0.0, STEP_SWEEP_DISTANCE, 0.0);
    let up_fraction = match world.cast_mover(position, mover, up, collision_filter) {
        Ok(fraction) => fraction.clamp(0.0, 1.0),
        Err(error) => {
            step_debug!(
                step_debug_enabled,
                "[stair-debug] rejected upward sweep: BoxDDD error={error:?}"
            );
            return None;
        }
    };
    let available_up = STEP_SWEEP_DISTANCE * up_fraction;
    let up_distance = if up_fraction < 1.0 {
        available_up - STEP_CLEARANCE
    } else {
        STEP_SWEEP_DISTANCE
    };
    step_debug!(
        step_debug_enabled,
        "[stair-debug] upward sweep fraction={up_fraction:.4} available={available_up:.4} usable={up_distance:.4} required_max={STEP_SWEEP_DISTANCE:.4}"
    );
    if up_distance <= f32::EPSILON {
        step_debug!(
            step_debug_enabled,
            "[stair-debug] rejected upward clearance: usable={up_distance:.4} after clearance={STEP_CLEARANCE:.4}"
        );
        return None;
    }
    let elevated = add_box_vec3(position, boxddd::Vec3::new(0.0, up_distance, 0.0));
    let horizontal = boxddd::Vec3::new(remaining.x, 0.0, remaining.z);
    if box_vec_length_squared(horizontal) <= f32::EPSILON {
        step_debug!(
            step_debug_enabled,
            "[stair-debug] rejected horizontal sweep: requested displacement is zero"
        );
        return None;
    }
    let horizontal_fraction = match world.cast_mover(elevated, mover, horizontal, collision_filter)
    {
        Ok(fraction) => fraction.clamp(0.0, 1.0),
        Err(error) => {
            step_debug!(
                step_debug_enabled,
                "[stair-debug] rejected horizontal sweep: BoxDDD error={error:?}"
            );
            return None;
        }
    };
    let elevated = add_box_vec3(elevated, scale_box_vec3(horizontal, horizontal_fraction));
    let horizontal_progress =
        boxddd::Vec3::new(elevated.x - position.x, 0.0, elevated.z - position.z);
    step_debug!(
        step_debug_enabled,
        "[stair-debug] horizontal sweep fraction={horizontal_fraction:.4} progress=({:.4},{:.4}) requested=({:.4},{:.4})",
        horizontal_progress.x,
        horizontal_progress.z,
        horizontal.x,
        horizontal.z
    );
    if box_vec_length_squared(horizontal_progress) <= f32::EPSILON {
        step_debug!(
            step_debug_enabled,
            "[stair-debug] rejected horizontal progress: elevated sweep could not advance"
        );
        return None;
    }
    let down = boxddd::Vec3::new(0.0, -up_distance, 0.0);
    let down_fraction = match world.cast_mover(elevated, mover, down, support_filter) {
        Ok(fraction) => fraction.clamp(0.0, 1.0),
        Err(error) => {
            step_debug!(
                step_debug_enabled,
                "[stair-debug] direct downward sweep failed: BoxDDD error={error:?}; trying nosing fallback"
            );
            1.0
        }
    };
    let collision_down_fraction = match world.cast_mover(elevated, mover, down, collision_filter) {
        Ok(fraction) => fraction.clamp(0.0, 1.0),
        Err(error) => {
            step_debug!(
                step_debug_enabled,
                "[stair-debug] rejected direct candidate clearance check: BoxDDD error={error:?}"
            );
            return None;
        }
    };
    let excluded_surface_precedes_support =
        collision_down_fraction + STEP_VALIDATION_EPSILON < down_fraction;
    if excluded_surface_precedes_support {
        step_debug!(
            step_debug_enabled,
            "[stair-debug] direct candidate found only excluded non-level support before eligible level geometry general_fraction={collision_down_fraction:.4} support_fraction={down_fraction:.4}; trying nosing fallback"
        );
    }
    if down_fraction < 1.0 && !excluded_surface_precedes_support {
        let stepped = add_box_vec3(elevated, scale_box_vec3(down, down_fraction));
        let rise = stepped.y - position.y;
        if rise > f32::EPSILON && rise <= STEP_HEIGHT + STEP_VALIDATION_EPSILON {
            match world.collide_mover(stepped, mover, support_filter) {
                Ok(planes) if has_walkable_plane(&planes) => {
                    step_debug!(
                        step_debug_enabled,
                        "[stair-debug] accepted direct candidate rise={rise:.4} down_fraction={down_fraction:.4}"
                    );
                    return Some(stepped);
                }
                Ok(planes) => {
                    step_debug!(
                        step_debug_enabled,
                        "[stair-debug] direct candidate not walkable rise={rise:.4} contacts={} down_fraction={down_fraction:.4}; trying nosing fallback",
                        planes.len()
                    );
                }
                Err(error) => {
                    step_debug!(
                        step_debug_enabled,
                        "[stair-debug] direct candidate contact query failed rise={rise:.4} error={error:?}; trying nosing fallback"
                    );
                }
            }
        } else {
            step_debug!(
                step_debug_enabled,
                "[stair-debug] direct candidate rise out of range rise={rise:.4} accepted=(0,{:.4}]; trying nosing fallback",
                STEP_HEIGHT + STEP_VALIDATION_EPSILON
            );
        }
    } else if !excluded_surface_precedes_support {
        step_debug!(
            step_debug_enabled,
            "[stair-debug] direct downward sweep found no eligible level support within {up_distance:.4}; trying nosing fallback"
        );
    }

    // A rounded capsule can touch the stair nosing with a steep normal before
    // its center reaches the tread. Probe one capsule radius ahead, then keep
    // the real horizontal displacement while adopting only the validated tread
    // height. Subsequent frames use `try_forward_step_support` until the center
    // is physically over the tread.
    let horizontal_length = box_vec_length_squared(horizontal).sqrt();
    let direction = scale_box_vec3(horizontal, 1.0 / horizontal_length);
    let feet_y = position.y - CAPSULE_HEIGHT * 0.5;
    let current_probe_origin = boxddd::Vec3::new(position.x, feet_y + STEP_CLEARANCE, position.z);
    let current_probe_origins = step_support_probe_origins(current_probe_origin, direction);
    let current_probe_down = boxddd::Vec3::new(0.0, -STEP_CLEARANCE * 2.0, 0.0);
    let (current_support, current_hits, current_errors) = probe_walkable_step_support(
        world,
        &current_probe_origins,
        current_probe_down,
        feet_y - STEP_CLEARANCE - STEP_VALIDATION_EPSILON,
        feet_y + STEP_CLEARANCE + STEP_VALIDATION_EPSILON,
        support_filter,
    );
    let current_support_is_excluded = current_support.is_none()
        && step_debug_enabled
        && probe_walkable_step_support(
            world,
            &current_probe_origins,
            current_probe_down,
            feet_y - STEP_CLEARANCE - STEP_VALIDATION_EPSILON,
            feet_y + STEP_CLEARANCE + STEP_VALIDATION_EPSILON,
            collision_filter,
        )
        .0
        .is_some();
    let Some((current_ground, current_forward, current_lateral)) = current_support else {
        let reason = if current_support_is_excluded {
            "only excluded non-level collider"
        } else {
            "no eligible level support"
        };
        step_debug!(
            step_debug_enabled,
            "[stair-debug] rejected fallback current support: {reason} samples={} eligible_hits={current_hits} errors={current_errors} center=({:.4},{:.4},{:.4})",
            current_probe_origins.len(),
            current_probe_origin.x,
            current_probe_origin.y,
            current_probe_origin.z
        );
        return None;
    };
    step_debug!(
        step_debug_enabled && (current_forward > 0.0 || current_lateral != 0.0),
        "[stair-debug] current support recovered by footprint forward={current_forward:.4} lateral={current_lateral:.4} point=({:.4},{:.4},{:.4})",
        current_ground.x,
        current_ground.y,
        current_ground.z
    );
    let probe_origins = forward_step_probe_origins(elevated, direction, current_ground.y);
    let probe_origin = probe_origins[0].0;
    let probe_down = boxddd::Vec3::new(0.0, -(STEP_SWEEP_DISTANCE + STEP_CLEARANCE), 0.0);
    let min_support_y = current_ground.y + f32::EPSILON;
    let max_support_y = current_ground.y + STEP_HEIGHT + STEP_VALIDATION_EPSILON;
    let (support, total_hits, ray_errors) = probe_walkable_step_support(
        world,
        &probe_origins,
        probe_down,
        min_support_y,
        max_support_y,
        support_filter,
    );
    let ahead_support_is_excluded = support.is_none()
        && step_debug_enabled
        && probe_walkable_step_support(
            world,
            &probe_origins,
            probe_down,
            min_support_y,
            max_support_y,
            collision_filter,
        )
        .0
        .is_some();
    let Some((hit_point, forward_offset, lateral_offset)) = support else {
        let reason = if ahead_support_is_excluded {
            "only excluded non-level collider"
        } else {
            "no eligible level support"
        };
        step_debug!(
            step_debug_enabled,
            "[stair-debug] rejected fallback ahead support: {reason} samples={} eligible_hits={total_hits} errors={ray_errors} center=({:.4},{:.4},{:.4})",
            probe_origins.len(),
            probe_origin.x,
            probe_origin.y,
            probe_origin.z
        );
        return None;
    };
    let rise = hit_point.y - current_ground.y;
    if rise <= f32::EPSILON || rise > STEP_HEIGHT + STEP_VALIDATION_EPSILON {
        step_debug!(
            step_debug_enabled,
            "[stair-debug] rejected fallback rise: measured={rise:.4} accepted=(0,{:.4}] current_y={:.4} ahead_y={:.4}",
            STEP_HEIGHT + STEP_VALIDATION_EPSILON,
            current_ground.y,
            hit_point.y
        );
        return None;
    }
    if rise + STEP_CLEARANCE > up_distance + STEP_VALIDATION_EPSILON {
        step_debug!(
            step_debug_enabled,
            "[stair-debug] rejected fallback overhead: rise={rise:.4} clearance={STEP_CLEARANCE:.4} usable_up={up_distance:.4}"
        );
        return None;
    }
    let stepped = boxddd::Vec3::new(elevated.x, position.y + rise, elevated.z);
    let descent = boxddd::Vec3::new(0.0, stepped.y - elevated.y, 0.0);
    let descent_fraction = match world.cast_mover(elevated, mover, descent, collision_filter) {
        Ok(fraction) => fraction.clamp(0.0, 1.0),
        Err(error) => {
            step_debug!(
                step_debug_enabled,
                "[stair-debug] rejected fallback placement sweep: BoxDDD error={error:?}"
            );
            return None;
        }
    };
    if descent_fraction < 1.0 {
        step_debug!(
            step_debug_enabled,
            "[stair-debug] rejected fallback placement: sweep fraction={descent_fraction:.4} rise={rise:.4}"
        );
        return None;
    }
    step_debug!(
        step_debug_enabled,
        "[stair-debug] accepted nosing fallback rise={rise:.4} usable_up={up_distance:.4} forward={forward_offset:.4} lateral={lateral_offset:.4} ahead=({:.4},{:.4},{:.4})",
        hit_point.x,
        hit_point.y,
        hit_point.z
    );
    Some(stepped)
}

pub(crate) fn forward_step_probe_origins(
    elevated: boxddd::Vec3,
    direction: boxddd::Vec3,
    current_ground_y: f32,
) -> [(boxddd::Vec3, f32, f32); 15] {
    let center = boxddd::Vec3::new(
        elevated.x + direction.x * (CAPSULE_RADIUS + STEP_CLEARANCE),
        current_ground_y + STEP_SWEEP_DISTANCE,
        elevated.z + direction.z * (CAPSULE_RADIUS + STEP_CLEARANCE),
    );
    let near = step_support_probe_row(center, direction, 0.0);
    let mid = step_support_probe_row(center, direction, STEP_SUPPORT_MID_FORWARD_OFFSET);
    let forward = step_support_probe_row(center, direction, STEP_SUPPORT_FORWARD_OFFSET);
    let far = step_support_probe_row(center, direction, STEP_SUPPORT_FAR_FORWARD_OFFSET);
    let max = step_support_probe_row(center, direction, STEP_SUPPORT_MAX_FORWARD_OFFSET);
    [
        near[0], near[1], near[2], mid[0], mid[1], mid[2], forward[0], forward[1], forward[2],
        far[0], far[1], far[2], max[0], max[1], max[2],
    ]
}

pub(crate) fn step_support_probe_origins(
    center: boxddd::Vec3,
    direction: boxddd::Vec3,
) -> [(boxddd::Vec3, f32, f32); 9] {
    let near = step_support_probe_row(center, direction, 0.0);
    let mid = step_support_probe_row(center, direction, STEP_SUPPORT_MID_FORWARD_OFFSET);
    let forward = step_support_probe_row(center, direction, STEP_SUPPORT_FORWARD_OFFSET);
    [
        near[0], near[1], near[2], mid[0], mid[1], mid[2], forward[0], forward[1], forward[2],
    ]
}

fn step_support_probe_row(
    center: boxddd::Vec3,
    direction: boxddd::Vec3,
    forward_offset: f32,
) -> [(boxddd::Vec3, f32, f32); 3] {
    let lateral = boxddd::Vec3::new(-direction.z, 0.0, direction.x);
    let forward_center = add_box_vec3(center, scale_box_vec3(direction, forward_offset));
    [
        (forward_center, forward_offset, 0.0),
        (
            add_box_vec3(
                forward_center,
                scale_box_vec3(lateral, STEP_SUPPORT_LATERAL_OFFSET),
            ),
            forward_offset,
            STEP_SUPPORT_LATERAL_OFFSET,
        ),
        (
            add_box_vec3(
                forward_center,
                scale_box_vec3(lateral, -STEP_SUPPORT_LATERAL_OFFSET),
            ),
            forward_offset,
            -STEP_SUPPORT_LATERAL_OFFSET,
        ),
    ]
}

pub(crate) fn probe_walkable_step_support(
    world: &mut boxddd::World,
    origins: &[(boxddd::Vec3, f32, f32)],
    down: boxddd::Vec3,
    min_y: f32,
    max_y: f32,
    filter: boxddd::QueryFilter,
) -> (Option<(boxddd::Vec3, f32, f32)>, usize, usize) {
    let mut best = None;
    let mut total_hits = 0;
    let mut ray_errors = 0;

    for &(origin, forward_offset, lateral_offset) in origins {
        let hits = match world.cast_ray(origin, down, filter) {
            Ok(hits) => hits,
            Err(_) => {
                ray_errors += 1;
                continue;
            }
        };
        total_hits += hits.len();
        for hit in hits.iter().filter(|hit| {
            hit.normal.y >= WALKABLE_SLOPE_COS && (min_y..=max_y).contains(&hit.point.y)
        }) {
            let replace = match &best {
                Some((_, best_fraction, _, _)) => hit.fraction < *best_fraction,
                None => true,
            };
            if replace {
                best = Some((
                    boxddd::Vec3::new(hit.point.x, hit.point.y, hit.point.z),
                    hit.fraction,
                    forward_offset,
                    lateral_offset,
                ));
            }
        }
    }

    (
        best.map(|(point, _, forward_offset, lateral_offset)| {
            (point, forward_offset, lateral_offset)
        }),
        total_hits,
        ray_errors,
    )
}

pub(crate) fn try_step_down(
    world: &mut boxddd::World,
    position: boxddd::Vec3,
    mover: &boxddd::Capsule,
    horizontal_motion: boxddd::Vec3,
    collision_filter: boxddd::QueryFilter,
    support_filter: boxddd::QueryFilter,
) -> Option<boxddd::Vec3> {
    let down = boxddd::Vec3::new(0.0, -STEP_SWEEP_DISTANCE, 0.0);
    let fraction = world
        .cast_mover(position, mover, down, support_filter)
        .ok()?
        .clamp(0.0, 1.0);
    if fraction >= 1.0 {
        return None;
    }
    let collision_fraction = world
        .cast_mover(position, mover, down, collision_filter)
        .ok()?
        .clamp(0.0, 1.0);
    if collision_fraction + STEP_VALIDATION_EPSILON < fraction {
        return None;
    }
    let snapped = add_box_vec3(position, scale_box_vec3(down, fraction));
    let drop = position.y - snapped.y;
    if !(-STEP_VALIDATION_EPSILON..=STEP_HEIGHT + STEP_VALIDATION_EPSILON).contains(&drop) {
        return None;
    }
    let planes = world.collide_mover(snapped, mover, support_filter).ok()?;
    if has_walkable_plane(&planes) {
        return Some(snapped);
    }

    // While rolling off a tread, the capsule can contact the upper nosing with
    // a steep normal before its flat-foot position reaches the lower tread.
    // Keep that intermediate contact grounded only when a walkable surface is
    // directly below and still within the configured step-down distance.
    let feet_y = position.y - CAPSULE_HEIGHT * 0.5;
    let horizontal = boxddd::Vec3::new(horizontal_motion.x, 0.0, horizontal_motion.z);
    let horizontal_length = box_vec_length_squared(horizontal).sqrt();
    let direction = if horizontal_length > f32::EPSILON {
        scale_box_vec3(horizontal, 1.0 / horizontal_length)
    } else {
        boxddd::Vec3::new(1.0, 0.0, 0.0)
    };
    let probe_origin = boxddd::Vec3::new(position.x, feet_y + STEP_CLEARANCE, position.z);
    let probe_origins = step_support_probe_origins(probe_origin, direction);
    let probe_down = boxddd::Vec3::new(0.0, -(STEP_SWEEP_DISTANCE + STEP_CLEARANCE), 0.0);
    let (support, _, _) = probe_walkable_step_support(
        world,
        &probe_origins,
        probe_down,
        feet_y - STEP_HEIGHT - STEP_VALIDATION_EPSILON,
        feet_y + STEP_VALIDATION_EPSILON,
        support_filter,
    );
    let (hit_point, _, _) = support?;
    let support_drop = feet_y - hit_point.y;
    (-STEP_VALIDATION_EPSILON..=STEP_HEIGHT + STEP_VALIDATION_EPSILON)
        .contains(&support_drop)
        .then_some(snapped)
}

pub(crate) fn try_forward_step_support(
    world: &mut boxddd::World,
    position: boxddd::Vec3,
    horizontal_motion: boxddd::Vec3,
    support_filter: boxddd::QueryFilter,
) -> Option<boxddd::Vec3> {
    let horizontal = boxddd::Vec3::new(horizontal_motion.x, 0.0, horizontal_motion.z);
    let horizontal_length = box_vec_length_squared(horizontal).sqrt();
    if horizontal_length <= f32::EPSILON {
        return None;
    }
    let direction = scale_box_vec3(horizontal, 1.0 / horizontal_length);
    let feet_y = position.y - CAPSULE_HEIGHT * 0.5;
    let probe_origin = boxddd::Vec3::new(
        position.x + direction.x * (CAPSULE_RADIUS + STEP_CLEARANCE),
        feet_y + STEP_CLEARANCE,
        position.z + direction.z * (CAPSULE_RADIUS + STEP_CLEARANCE),
    );
    let probe_origins = step_support_probe_origins(probe_origin, direction);
    let probe_down = boxddd::Vec3::new(0.0, -STEP_CLEARANCE * 2.0, 0.0);
    let (support, _, _) = probe_walkable_step_support(
        world,
        &probe_origins,
        probe_down,
        feet_y - STEP_CLEARANCE - STEP_VALIDATION_EPSILON,
        feet_y + STEP_CLEARANCE + STEP_VALIDATION_EPSILON,
        support_filter,
    );
    let (hit_point, _, _) = support?;
    let supported_y = hit_point.y + CAPSULE_HEIGHT * 0.5;
    if (supported_y - position.y).abs() > STEP_CLEARANCE + f32::EPSILON {
        return None;
    }
    Some(boxddd::Vec3::new(position.x, supported_y, position.z))
}
