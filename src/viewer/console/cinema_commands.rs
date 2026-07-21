//! Cinema debug-camera console commands (issue #209).
//!
//! One `cam` command dispatches to the follow/lookat/orbit/path/release/status
//! sub-commands. Each engages the viewer-only [`CinemaState`]; the per-frame
//! drive system in `viewer::cinema` reads target transforms and moves only the
//! camera. Stable, machine-parseable log lines (`cinema follow <formid>`,
//! `cinema orbit <formid>`, `cinema path start`/`done`, `cinema release`) let
//! the agent bridge and the #210 filmstrip tooling script against this.

use super::super::cinema::{CinemaMode, CinemaPath, CinemaState};
use super::super::player::{CameraMode, CameraModeState, set_camera_mode};
use super::*;
use crate::console::RefRegistry;

const DEFAULT_FOLLOW_DIST: f32 = 4.0;
const DEFAULT_FOLLOW_HEIGHT: f32 = 2.0;
const DEFAULT_ORBIT_DEG_PER_SEC: f32 = 30.0;
const DEFAULT_PATH_SECONDS: f32 = 6.0;

pub(super) struct CinemaCommandProvider;

impl ConsoleCommandProvider for CinemaCommandProvider {
    fn register_commands(&self, registry: &mut ConsoleRegistry) -> Result<(), ConsoleError> {
        registry.register(
            ConsoleCommand::new(
                "cam",
                "cam <follow FormID [dist] [height] | lookat FormID|x y z | orbit FormID radius [deg_per_sec] | path x y z [more...] [seconds] | path follow FormID | release | status>",
                "Cinema debug camera: follow, orbit, aim, or dolly the free camera to watch actors and routes. Viewer-only -- reads entity transforms and drives only the camera. `cam release` restores the normal camera.",
                cam,
            )
            .mutating(),
        )
    }
}

fn finite(value: &str, label: &str) -> Result<f32, ConsoleError> {
    let parsed = value
        .parse::<f32>()
        .map_err(|_| ConsoleError::new("bad_type", format!("{label} must be a finite number")))?;
    if parsed.is_finite() {
        Ok(parsed)
    } else {
        Err(ConsoleError::new(
            "bad_type",
            format!("{label} must be a finite number"),
        ))
    }
}

/// Resolves a selector to a live reference and returns its FormID, so the
/// cinema mode can re-resolve it every frame. Confirms the reference exists
/// now, matching the other viewer commands' resolution.
fn resolve_target_form_id(world: &mut World, selector: &str) -> Result<u32, ConsoleError> {
    let entity = resolve_reference(world, selector)?;
    world
        .resource::<RefRegistry>()
        .identity(entity)
        .and_then(|identity| identity.form_id)
        .ok_or_else(|| {
            ConsoleError::new("no_form_id", "reference has no FormID for cinema to track")
        })
}

/// Forces the free camera and records the mode to restore on release. Only the
/// first engage records a restore target; if the user was already free-flying
/// (`tfc`) there is nothing to hand back.
fn engage(world: &mut World) {
    if world.resource::<CinemaState>().restore_mode.is_some() {
        return;
    }
    if world.resource::<CameraModeState>().mode == CameraMode::Fps {
        let _ = set_camera_mode(world, CameraMode::Free);
        world.resource_mut::<CinemaState>().restore_mode = Some(CameraMode::Fps);
    }
}

fn set_mode(world: &mut World, mode: CinemaMode) {
    world.resource_mut::<CinemaState>().mode = mode;
}

pub(super) fn cam(
    world: &mut World,
    invocation: &ConsoleInvocation,
) -> Result<ConsoleCommandResult, ConsoleError> {
    let Some(sub) = invocation.args.first() else {
        return Err(ConsoleError::new(
            "bad_arity",
            "cam requires a sub-command: follow, lookat, orbit, path, release, or status",
        ));
    };
    let rest = &invocation.args[1..];
    match sub.to_ascii_lowercase().as_str() {
        "follow" => cam_follow(world, rest),
        "lookat" => cam_lookat(world, rest),
        "orbit" => cam_orbit(world, rest),
        "path" => cam_path(world, rest),
        "release" => cam_release(world),
        "status" => cam_status(world),
        other => Err(ConsoleError::new(
            "bad_subcommand",
            format!("unknown cam sub-command '{other}'"),
        )),
    }
}

fn cam_follow(world: &mut World, args: &[String]) -> Result<ConsoleCommandResult, ConsoleError> {
    let (selector, dist, height) = match args {
        [selector] => (selector, DEFAULT_FOLLOW_DIST, DEFAULT_FOLLOW_HEIGHT),
        [selector, dist] => (selector, finite(dist, "dist")?, DEFAULT_FOLLOW_HEIGHT),
        [selector, dist, height] => (selector, finite(dist, "dist")?, finite(height, "height")?),
        _ => {
            return Err(ConsoleError::new(
                "bad_arity",
                "cam follow <FormID> [dist] [height]",
            ));
        }
    };
    let target = resolve_target_form_id(world, selector)?;
    engage(world);
    set_mode(
        world,
        CinemaMode::Follow {
            target,
            dist,
            height,
        },
    );
    info!("cinema follow {target:08x}");
    Ok(ConsoleCommandResult::new(
        json!({
            "mode": "follow",
            "target_form_id": target,
            "dist": dist,
            "height": height,
        }),
        vec![format!(
            "cinema follow {target:08x} dist={dist:.2} height={height:.2}"
        )],
    ))
}

fn cam_lookat(world: &mut World, args: &[String]) -> Result<ConsoleCommandResult, ConsoleError> {
    match args {
        [selector] => {
            let target = resolve_target_form_id(world, selector)?;
            engage(world);
            set_mode(world, CinemaMode::LookAtTarget { target });
            info!("cinema lookat {target:08x}");
            Ok(ConsoleCommandResult::new(
                json!({ "mode": "lookat", "target_form_id": target }),
                vec![format!("cinema lookat {target:08x}")],
            ))
        }
        [x, y, z] => {
            let point = Vec3::new(finite(x, "x")?, finite(y, "y")?, finite(z, "z")?);
            engage(world);
            set_mode(world, CinemaMode::LookAtPoint { point });
            info!(
                "cinema lookat point {:.3} {:.3} {:.3}",
                point.x, point.y, point.z
            );
            Ok(ConsoleCommandResult::new(
                json!({ "mode": "lookat", "point": [point.x, point.y, point.z] }),
                vec![format!(
                    "cinema lookat point {:.3} {:.3} {:.3}",
                    point.x, point.y, point.z
                )],
            ))
        }
        _ => Err(ConsoleError::new(
            "bad_arity",
            "cam lookat <FormID> | <x> <y> <z>",
        )),
    }
}

fn cam_orbit(world: &mut World, args: &[String]) -> Result<ConsoleCommandResult, ConsoleError> {
    let (selector, radius, deg_per_sec) = match args {
        [selector, radius] => (
            selector,
            finite(radius, "radius")?,
            DEFAULT_ORBIT_DEG_PER_SEC,
        ),
        [selector, radius, speed] => (
            selector,
            finite(radius, "radius")?,
            finite(speed, "deg_per_sec")?,
        ),
        _ => {
            return Err(ConsoleError::new(
                "bad_arity",
                "cam orbit <FormID> <radius> [deg_per_sec]",
            ));
        }
    };
    if radius <= 0.0 {
        return Err(ConsoleError::new(
            "bad_value",
            "orbit radius must be greater than zero",
        ));
    }
    let target = resolve_target_form_id(world, selector)?;
    engage(world);
    set_mode(
        world,
        CinemaMode::Orbit {
            target,
            radius,
            deg_per_sec,
            angle: 0.0,
        },
    );
    info!("cinema orbit {target:08x}");
    Ok(ConsoleCommandResult::new(
        json!({
            "mode": "orbit",
            "target_form_id": target,
            "radius": radius,
            "deg_per_sec": deg_per_sec,
        }),
        vec![format!(
            "cinema orbit {target:08x} radius={radius:.2} deg_per_sec={deg_per_sec:.2}"
        )],
    ))
}

fn cam_path(world: &mut World, args: &[String]) -> Result<ConsoleCommandResult, ConsoleError> {
    // `cam path follow <FormID> [seconds]` trails a moving reference along the
    // straight line from the camera's current position to it. Every other form
    // is a list of explicit waypoints with an optional trailing duration.
    if args
        .first()
        .is_some_and(|first| first.eq_ignore_ascii_case("follow"))
    {
        let (selector, seconds) = match &args[1..] {
            [selector] => (selector, DEFAULT_PATH_SECONDS),
            [selector, seconds] => (selector, finite(seconds, "seconds")?),
            _ => {
                return Err(ConsoleError::new(
                    "bad_arity",
                    "cam path follow <FormID> [seconds]",
                ));
            }
        };
        let target = resolve_target_form_id(world, selector)?;
        let start = camera_position(world);
        let target_position = target_position(world, target).unwrap_or(start);
        engage(world);
        set_mode(
            world,
            CinemaMode::Path(CinemaPath {
                waypoints: vec![start, target_position],
                seconds,
                elapsed: 0.0,
                follow_target: Some(target),
                done: false,
            }),
        );
        info!("cinema path start");
        return Ok(ConsoleCommandResult::new(
            json!({
                "mode": "path",
                "follow_target_form_id": target,
                "seconds": seconds,
                "waypoints": 2,
            }),
            vec![format!(
                "cinema path start follow {target:08x} seconds={seconds:.2}"
            )],
        ));
    }

    // Group the numeric arguments into XYZ waypoints; a lone trailing value is
    // the duration.
    let seconds = if args.len() % 3 == 1 {
        finite(args.last().expect("len checked"), "seconds")?
    } else {
        DEFAULT_PATH_SECONDS
    };
    let coordinate_args = if args.len() % 3 == 1 {
        &args[..args.len() - 1]
    } else {
        args
    };
    if coordinate_args.len() < 6 || coordinate_args.len() % 3 != 0 {
        return Err(ConsoleError::new(
            "bad_arity",
            "cam path needs at least two <x> <y> <z> waypoints and an optional trailing [seconds]",
        ));
    }
    let mut waypoints = Vec::with_capacity(coordinate_args.len() / 3);
    for point in coordinate_args.chunks_exact(3) {
        waypoints.push(Vec3::new(
            finite(&point[0], "x")?,
            finite(&point[1], "y")?,
            finite(&point[2], "z")?,
        ));
    }
    let count = waypoints.len();
    engage(world);
    set_mode(
        world,
        CinemaMode::Path(CinemaPath {
            waypoints,
            seconds,
            elapsed: 0.0,
            follow_target: None,
            done: false,
        }),
    );
    info!("cinema path start");
    Ok(ConsoleCommandResult::new(
        json!({
            "mode": "path",
            "waypoints": count,
            "seconds": seconds,
        }),
        vec![format!(
            "cinema path start waypoints={count} seconds={seconds:.2}"
        )],
    ))
}

fn cam_release(world: &mut World) -> Result<ConsoleCommandResult, ConsoleError> {
    let restore_mode = {
        let mut state = world.resource_mut::<CinemaState>();
        state.mode = CinemaMode::Inactive;
        state.restore_mode.take()
    };
    // Keep the free camera pointed where cinema left it: sync the fly-camera
    // yaw/pitch to the current pose so free-fly resumes without snapping.
    sync_fly_camera_from_transform(world);
    let restored = if restore_mode == Some(CameraMode::Fps)
        && world.resource::<CameraModeState>().mode == CameraMode::Free
    {
        let _ = set_camera_mode(world, CameraMode::Fps);
        "fps"
    } else {
        "free"
    };
    info!("cinema release");
    Ok(ConsoleCommandResult::new(
        json!({ "mode": "inactive", "restored_camera": restored }),
        vec![format!("cinema release (camera: {restored})")],
    ))
}

fn cam_status(world: &mut World) -> Result<ConsoleCommandResult, ConsoleError> {
    let state = world.resource::<CinemaState>();
    let mode = state.mode_label();
    let active = state.is_active();
    let target = state.target_form_id();
    let details = match &state.mode {
        CinemaMode::Follow { dist, height, .. } => json!({ "dist": dist, "height": height }),
        CinemaMode::Orbit {
            radius,
            deg_per_sec,
            angle,
            ..
        } => json!({ "radius": radius, "deg_per_sec": deg_per_sec, "angle_rad": angle }),
        CinemaMode::LookAtPoint { point } => json!({ "point": [point.x, point.y, point.z] }),
        CinemaMode::Path(path) => json!({
            "waypoints": path.waypoints.len(),
            "seconds": path.seconds,
            "elapsed": path.elapsed,
            "done": path.done,
            "follow_target_form_id": path.follow_target,
        }),
        CinemaMode::Inactive | CinemaMode::LookAtTarget { .. } => Value::Null,
    };
    let target_hex = target.map_or_else(|| "none".to_owned(), |form_id| format!("{form_id:08x}"));
    let summary = format!("cinema status mode={mode} active={active} target={target_hex}");
    info!("{summary}");
    Ok(ConsoleCommandResult::new(
        json!({
            "mode": mode,
            "active": active,
            "target_form_id": target,
            "details": details,
        }),
        vec![summary],
    ))
}

fn camera_position(world: &mut World) -> Vec3 {
    world
        .query_filtered::<&GlobalTransform, With<Camera3d>>()
        .iter(world)
        .next()
        .map_or(Vec3::ZERO, |global| global.translation())
}

fn target_position(world: &mut World, form_id: u32) -> Option<Vec3> {
    let entity = world.resource::<RefRegistry>().entity_of(form_id)?;
    world
        .get::<GlobalTransform>(entity)
        .map(GlobalTransform::translation)
}

fn sync_fly_camera_from_transform(world: &mut World) {
    let rotation = world
        .query_filtered::<&Transform, With<Camera3d>>()
        .iter(world)
        .next()
        .map(|transform| transform.rotation);
    let Some(rotation) = rotation else {
        return;
    };
    let (yaw, pitch, _) = rotation.to_euler(EulerRot::YXZ);
    let mut cameras = world.query_filtered::<&mut super::super::FlyCamera, With<Camera3d>>();
    if let Some(mut fly) = cameras.iter_mut(world).next() {
        fly.yaw = yaw;
        fly.pitch = pitch.clamp(-1.5, 1.5);
    }
}
