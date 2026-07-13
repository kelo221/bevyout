//! Player camera and interpolation systems.

use super::*;

pub(crate) fn capture_player_render_history(
    mut players: Query<(&Transform, &mut PlayerRenderHistory), With<FpsPlayer>>,
) {
    let Ok((transform, mut history)) = players.single_mut() else {
        return;
    };
    history.previous_position = transform.translation;
}

pub(crate) fn interpolate_fps_camera(
    time: Res<Time>,
    fixed_time: Res<Time<Fixed>>,
    state: Res<CameraModeState>,
    mut players: Query<(&Transform, &KccState, &mut PlayerRenderHistory), With<FpsPlayer>>,
    mut cameras: Query<(&Transform, &mut GlobalTransform), With<Camera3d>>,
) {
    if state.mode != CameraMode::Fps {
        return;
    }
    let Some(player_entity) = state.player else {
        return;
    };
    let Ok((player_transform, kcc, mut history)) = players.get_mut(player_entity) else {
        return;
    };
    let Ok((camera_transform, mut camera_global)) = cameras.single_mut() else {
        return;
    };

    let mut interpolated_player_position = interpolate_render_position(
        history.previous_position,
        player_transform.translation,
        fixed_time.overstep_fraction(),
    );
    interpolated_player_position.y = smooth_grounded_camera_y(
        &mut history,
        interpolated_player_position.y,
        kcc.grounded,
        time.delta_secs(),
    );

    // Rebuild the authoritative camera pose from local transforms instead of
    // reading the previous render override. This prevents offsets accumulating
    // when Bevy skips propagation for an unchanged hierarchy.
    let authoritative_camera =
        GlobalTransform::from(*player_transform).mul_transform(*camera_transform);
    let (scale, rotation, translation) = authoritative_camera.to_scale_rotation_translation();
    let render_translation =
        translation + (interpolated_player_position - player_transform.translation);
    *camera_global = GlobalTransform::from(Transform {
        translation: render_translation,
        rotation,
        scale,
    });
}

pub(crate) fn smooth_grounded_camera_y(
    history: &mut PlayerRenderHistory,
    target_y: f32,
    grounded: bool,
    delta_seconds: f32,
) -> f32 {
    let discontinuity = (target_y - history.last_target_y).abs() > STEP_SWEEP_DISTANCE;
    let reset =
        !history.vertical_initialized || !grounded || !history.was_grounded || discontinuity;

    if reset {
        history.smoothed_y = target_y;
        history.vertical_initialized = true;
    } else {
        let exponent = -CAMERA_VERTICAL_SETTLE_LOG_FACTOR * delta_seconds.max(0.0)
            / CAMERA_VERTICAL_SETTLE_SECONDS;
        let alpha = 1.0 - exponent.exp();
        history.smoothed_y += (target_y - history.smoothed_y) * alpha;
    }

    history.last_target_y = target_y;
    history.was_grounded = grounded;
    history.smoothed_y
}

pub(crate) fn interpolate_render_position(previous: Vec3, current: Vec3, alpha: f32) -> Vec3 {
    previous.lerp(current, alpha.clamp(0.0, 1.0))
}

pub(crate) fn fps_mouse_look(
    mut mouse: MessageReader<MouseMotion>,
    cursor_options: Single<&CursorOptions>,
    state: Res<CameraModeState>,
    mut players: Query<(&mut FpsPlayer, &mut Transform), Without<ChildOf>>,
    mut cameras: Query<FpsCameraQuery<'_>, (With<Camera3d>, With<ChildOf>)>,
) {
    let delta = mouse
        .read()
        .fold(Vec2::ZERO, |sum, event| sum + event.delta);
    if state.mode != CameraMode::Fps
        || !matches!(cursor_options.grab_mode, CursorGrabMode::Locked)
        || delta == Vec2::ZERO
    {
        return;
    }

    let Ok((mut player, mut player_transform)) = players.single_mut() else {
        return;
    };
    let Ok((mut camera_transform, mut fly_camera)) = cameras.single_mut() else {
        return;
    };
    player.yaw -= delta.x * MOUSE_SENSITIVITY;
    player.pitch = (player.pitch - delta.y * MOUSE_SENSITIVITY).clamp(-1.5, 1.5);
    player_transform.rotation = Quat::from_rotation_y(player.yaw);
    camera_transform.rotation = Quat::from_rotation_x(player.pitch);
    fly_camera.yaw = player.yaw;
    fly_camera.pitch = player.pitch;
}

pub(crate) fn camera_angles(rotation: Quat) -> (f32, f32) {
    let (yaw, pitch, _) = rotation.to_euler(EulerRot::YXZ);
    (yaw, pitch.clamp(-1.5, 1.5))
}

pub(crate) fn tab_pressed(keys: &ButtonInput<KeyCode>) -> bool {
    keys.just_pressed(KeyCode::Tab)
}
