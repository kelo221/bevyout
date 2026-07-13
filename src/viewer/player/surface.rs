//! Surface probing, landing, and footstep systems.

use super::*;

pub(crate) fn emit_landing_events(
    state: Res<CameraModeState>,
    context: NonSend<BoxdddPhysicsContext>,
    collision_world: Res<PreparedCollisionWorld>,
    mut landings: MessageWriter<PlayLanding>,
    mut players: Query<
        (
            Entity,
            &Transform,
            &KccState,
            &mut LocomotionState,
            &mut FootstepState,
        ),
        With<FpsPlayer>,
    >,
) {
    let Ok((entity, transform, kcc, mut locomotion, mut footstep)) = players.single_mut() else {
        return;
    };
    if state.mode != CameraMode::Fps {
        locomotion.reset(transform.translation);
        footstep.reset_tracking();
        return;
    }

    let airborne = !kcc.grounded;
    let Some(impact) = locomotion.update(transform.translation, kcc.velocity.y, airborne) else {
        return;
    };
    let surface = footstep_surface_or_default(probe_surface(
        entity,
        transform.translation,
        &context,
        &collision_world,
    ));
    landings.write(PlayLanding {
        surface: surface.into(),
        variant: impact.variant,
        hard: impact.hard,
    });
    footstep.reset_at(transform.translation);
}

pub(crate) fn emit_footsteps(
    state: Res<CameraModeState>,
    context: NonSend<BoxdddPhysicsContext>,
    collision_world: Res<PreparedCollisionWorld>,
    mut footsteps: MessageWriter<PlayFootstep>,
    mut players: Query<(Entity, &Transform, &KccState, &mut FootstepState), With<FpsPlayer>>,
) {
    let Ok((entity, transform, kcc, mut footstep)) = players.single_mut() else {
        return;
    };
    if state.mode != CameraMode::Fps {
        footstep.reset_tracking();
        return;
    }
    let position = transform.translation;
    footstep.record_motion(position, kcc.grounded && kcc.velocity.y.abs() <= 2.5);
    if footstep.distance < FOOTSTEP_DISTANCE {
        return;
    }
    let surface =
        footstep_surface_or_default(probe_surface(entity, position, &context, &collision_world));
    while let Some((right, variant)) = footstep.take_step() {
        footsteps.write(PlayFootstep {
            surface: surface.into(),
            right,
            variant,
        });
    }
}

pub(crate) fn probe_surface(
    _entity: Entity,
    position: Vec3,
    context: &BoxdddPhysicsContext,
    collision_world: &PreparedCollisionWorld,
) -> Option<&'static str> {
    let world = context.world()?;
    let origin = to_box_vec3(position - Vec3::Y * (CAPSULE_HEIGHT * 0.5 - 0.06));
    let translation = to_box_vec3(Vec3::new(0.0, -0.24, 0.0));
    let filter = boxddd::QueryFilter::new()
        .category_bits(PLAYER_QUERY)
        .mask_bits(WORLD_STATIC | WORLD_DYNAMIC);
    let hits = world.cast_ray(origin, translation, filter).ok()?;
    let hit = hits
        .iter()
        .min_by(|left, right| left.fraction.total_cmp(&right.fraction))?;
    let material = collision_world
        .surfaces
        .get(&hit.shape_id)
        .and_then(|surface| surface.material)
        .or_else(|| (hit.user_material_id != 0).then_some(hit.user_material_id as u32));
    Some(surface_family(material))
}

pub(crate) fn footstep_surface_or_default(surface: Option<&'static str>) -> &'static str {
    surface.unwrap_or(DEFAULT_FOOTSTEP_SURFACE)
}

pub(crate) fn has_walkable_plane(planes: &[boxddd::MoverPlane]) -> bool {
    planes
        .iter()
        .any(|plane| plane.plane.normal.y >= WALKABLE_SLOPE_COS)
}

pub(crate) fn to_box_vec3(value: Vec3) -> boxddd::Vec3 {
    boxddd::Vec3::new(value.x, value.y, value.z)
}

pub(crate) fn from_box_vec3(value: boxddd::Vec3) -> Vec3 {
    Vec3::new(value.x, value.y, value.z)
}

pub(crate) fn to_box_quat(value: Quat) -> boxddd::Quat {
    boxddd::Quat::new(boxddd::Vec3::new(value.x, value.y, value.z), value.w)
}

pub(crate) fn from_box_quat(value: boxddd::Quat) -> Quat {
    Quat::from_xyzw(value.v.x, value.v.y, value.v.z, value.s).normalize()
}

pub(crate) fn add_box_vec3(left: boxddd::Vec3, right: boxddd::Vec3) -> boxddd::Vec3 {
    boxddd::Vec3::new(left.x + right.x, left.y + right.y, left.z + right.z)
}

pub(crate) fn scale_box_vec3(value: boxddd::Vec3, scalar: f32) -> boxddd::Vec3 {
    boxddd::Vec3::new(value.x * scalar, value.y * scalar, value.z * scalar)
}

pub(crate) fn box_vec_length_squared(value: boxddd::Vec3) -> f32 {
    value.x * value.x + value.y * value.y + value.z * value.z
}

pub(crate) fn approach(current: f32, target: f32, max_change: f32) -> f32 {
    let delta = target - current;
    if delta.abs() <= max_change {
        target
    } else {
        current + delta.signum() * max_change
    }
}

pub(crate) fn apply_air_control(current: Vec3, input: Vec3, dt: f32) -> Vec3 {
    let air_direction = air_control_motion(input, true).normalize_or_zero();
    let air_target = air_direction * PLAYER_SPEED;
    let max_change = PLAYER_SPEED * AIR_CONTROL_FACTOR * 8.0 * dt;
    Vec3::new(
        approach(current.x, air_target.x, max_change),
        0.0,
        approach(current.z, air_target.z, max_change),
    )
}

pub(crate) fn surface_family(material: Option<u32>) -> &'static str {
    let Some(material) = material else {
        return "concrete";
    };
    match material % 32 {
        2 => "dirt",
        4 => "grass",
        5 | 11 | 13 | 14 | 15 | 20 | 21 | 23 | 24 | 25 | 26 | 27 | 28 | 29 | 30 | 31 => {
            "metal_solid"
        }
        8 => "water",
        9 | 12 => "wood",
        16 => "metal_hollow",
        17 => "metal_sheet",
        18 => "gravel",
        19 => "concrete_broken",
        _ => "concrete",
    }
}
