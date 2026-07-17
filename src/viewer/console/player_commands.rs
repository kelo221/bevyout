//! Player camera, movement, and scene-debug console commands.

use super::*;

pub(super) fn register(registry: &mut ConsoleRegistry) {
    for command in [
        ConsoleCommand::new(
            "tcl",
            "tcl",
            "Toggle FPS-player collision and gravity while preserving movement.",
            toggle_collision,
        )
        .mutating(),
        ConsoleCommand::new(
            "tfc",
            "tfc",
            "Toggle between the FPS player and free-fly camera.",
            toggle_fly_camera,
        )
        .aliases(&["toggleflycam"])
        .mutating(),
        ConsoleCommand::new(
            "fov",
            "fov [10..170]",
            "Get or set the horizontal camera field of view in degrees.",
            field_of_view,
        )
        .mutating(),
        ConsoleCommand::new(
            "tlights",
            "tlights",
            "Toggle all runtime scene lights.",
            toggle_lights,
        )
        .mutating(),
        ConsoleCommand::new(
            "tcg",
            "tcg",
            "Toggle collision geometry diagnostics.",
            toggle_collision_geometry,
        )
        .aliases(&["togglecollisiongeometry"])
        .mutating(),
        ConsoleCommand::new(
            "stairdebug",
            "stairdebug",
            "Toggle stair-step rejection logging.",
            toggle_stair_debug,
        )
        .mutating(),
        ConsoleCommand::new(
            "tunlit",
            "tunlit",
            "Toggle unlit material diagnostics.",
            toggle_unlit,
        )
        .mutating(),
    ] {
        registry
            .register(command)
            .expect("player console command is unique");
    }
}

pub(super) fn toggle_collision(
    world: &mut World,
    invocation: &ConsoleInvocation,
) -> Result<ConsoleCommandResult, ConsoleError> {
    no_args(invocation)?;
    if world.resource::<player::CameraModeState>().mode != player::CameraMode::Fps {
        return Err(ConsoleError::new(
            "player_unavailable",
            "tcl requires the FPS player",
        ));
    }
    let currently_no_clip = world.resource::<player::PlayerNoClip>().0;
    if currently_no_clip {
        if world.resource::<player::PhysicsDisabled>().0 {
            return Err(ConsoleError::new(
                "physics_disabled",
                "collision cannot be enabled because physics is disabled",
            ));
        }
        if !world.resource::<player::CameraModeState>().collisions_ready {
            return Err(ConsoleError::new(
                "collision_unavailable",
                "collision cannot be enabled because the scene has no usable collision geometry",
            ));
        }
    }
    let enabled = !currently_no_clip;
    world.resource_mut::<player::PlayerNoClip>().0 = enabled;
    Ok(toggle_result(
        json!({ "no_clip": enabled }),
        "Collision",
        !enabled,
    ))
}

pub(super) fn toggle_fly_camera(
    world: &mut World,
    invocation: &ConsoleInvocation,
) -> Result<ConsoleCommandResult, ConsoleError> {
    no_args(invocation)?;
    let mode = player::toggle_camera_mode_now(world).map_err(|error| match error {
        player::CameraModeError::CameraUnavailable => ConsoleError::new(
            "camera_unavailable",
            "expected exactly one active 3D camera",
        ),
        player::CameraModeError::HierarchyInvalid => ConsoleError::new(
            "camera_hierarchy_invalid",
            "camera and FPS-player hierarchy is inconsistent",
        ),
        player::CameraModeError::PlayerUnavailable => {
            ConsoleError::new("player_unavailable", "the FPS player does not exist")
        }
    })?;
    let mode = match mode {
        player::CameraMode::Free => "free",
        player::CameraMode::Fps => "fps",
    };
    Ok(toggle_result(
        json!({ "camera_mode": mode }),
        "Free camera",
        mode == "free",
    ))
}

pub(super) fn field_of_view(
    world: &mut World,
    invocation: &ConsoleInvocation,
) -> Result<ConsoleCommandResult, ConsoleError> {
    if invocation.args.len() > 1 {
        return Err(ConsoleError::new(
            "bad_arity",
            "fov accepts at most one value",
        ));
    }
    let camera = {
        let mut query = world.query_filtered::<Entity, (With<Camera3d>, With<HorizontalFov>)>();
        let mut cameras = query.iter(world);
        let Some(camera) = cameras.next() else {
            return Err(ConsoleError::new(
                "camera_unavailable",
                "the active 3D camera does not expose an FOV setting",
            ));
        };
        if cameras.next().is_some() {
            return Err(ConsoleError::new(
                "camera_unavailable",
                "expected exactly one active 3D camera",
            ));
        }
        camera
    };

    let requested = invocation
        .args
        .first()
        .map(|value| {
            value
                .parse::<f32>()
                .map_err(|_| ConsoleError::new("bad_type", "fov must be a finite number"))
        })
        .transpose()?;
    if let Some(requested) = requested {
        if !requested.is_finite()
            || !(MIN_HORIZONTAL_FOV_DEGREES..=MAX_HORIZONTAL_FOV_DEGREES).contains(&requested)
        {
            return Err(ConsoleError::new(
                "out_of_range",
                format!(
                    "fov must be between {MIN_HORIZONTAL_FOV_DEGREES} and {MAX_HORIZONTAL_FOV_DEGREES} degrees"
                ),
            ));
        }
        let aspect_ratio = match world.get::<Projection>(camera) {
            Some(Projection::Perspective(perspective)) => perspective.aspect_ratio,
            _ => {
                return Err(ConsoleError::new(
                    "camera_unavailable",
                    "the active 3D camera is not perspective",
                ));
            }
        };
        world
            .get_mut::<HorizontalFov>(camera)
            .expect("camera query required HorizontalFov")
            .0 = requested;
        let mut projection = world
            .get_mut::<Projection>(camera)
            .expect("Camera3d requires Projection");
        let Projection::Perspective(perspective) = &mut *projection else {
            unreachable!("perspective projection checked above");
        };
        perspective.fov = horizontal_to_vertical_fov(requested, aspect_ratio);
    }

    let degrees = world
        .get::<HorizontalFov>(camera)
        .expect("camera query required HorizontalFov")
        .0;
    Ok(ConsoleCommandResult::new(
        json!({
            "degrees": degrees,
            "axis": "horizontal",
        }),
        vec![if requested.is_some() {
            format!("Horizontal FOV set to {degrees} degrees.")
        } else {
            format!("Horizontal FOV is {degrees} degrees.")
        }],
    ))
}

pub(super) fn toggle_lights(
    world: &mut World,
    invocation: &ConsoleInvocation,
) -> Result<ConsoleCommandResult, ConsoleError> {
    no_args(invocation)?;
    let disabled = {
        let mut disabled = world.resource_mut::<LightsDisabled>();
        disabled.0 = !disabled.0;
        disabled.0
    };
    Ok(toggle_result(
        json!({ "lights_enabled": !disabled }),
        "Lights",
        !disabled,
    ))
}

pub(super) fn toggle_collision_geometry(
    world: &mut World,
    invocation: &ConsoleInvocation,
) -> Result<ConsoleCommandResult, ConsoleError> {
    no_args(invocation)?;
    let enabled = {
        let mut settings = world.resource_mut::<BoxdddDebugDrawSettings>();
        player::flip_collider_debug(&mut settings);
        settings.enabled
    };
    Ok(toggle_result(
        json!({ "enabled": enabled }),
        "Collision geometry",
        enabled,
    ))
}

pub(super) fn toggle_stair_debug(
    world: &mut World,
    invocation: &ConsoleInvocation,
) -> Result<ConsoleCommandResult, ConsoleError> {
    no_args(invocation)?;
    let enabled = {
        let mut settings = world.resource_mut::<player::StepDebugSettings>();
        player::flip_step_debug(&mut settings);
        player::step_debug_enabled(&settings)
    };
    Ok(toggle_result(
        json!({ "enabled": enabled }),
        "Stair debugging",
        enabled,
    ))
}

pub(super) fn toggle_unlit(
    world: &mut World,
    invocation: &ConsoleInvocation,
) -> Result<ConsoleCommandResult, ConsoleError> {
    no_args(invocation)?;
    let enabled = {
        let mut mode = world.resource_mut::<UnlitMode>();
        mode.0 = !mode.0;
        mode.0
    };
    Ok(toggle_result(
        json!({ "enabled": enabled }),
        "Unlit mode",
        enabled,
    ))
}
