//! Viewer-only "cinema" debug camera (issue #209).
//!
//! A developer camera controller in the same additive, isolated posture as
//! `tfc`/`tna`/the agent bridge: it *reads* entity transforms and drives
//! *only* the shared `Camera3d` `Transform`. It never writes any
//! gameplay/actor/physics state. Every system is gated on [`CinemaState`]
//! being active, so the feature is completely inert unless a `cam` console
//! command engages it, and `cam release` cleanly restores the normal camera.
//!
//! The geometry lives in the pure `math` submodule (std/glam only) and is
//! unit-tested directly; the systems here are thin consumers that feed a
//! target's world pose in and turn the returned pose into a `Transform`.

use bevy::prelude::*;
use bevy::transform::TransformSystems;

use crate::app_state::AppState;
use crate::console::RefRegistry;

use super::player::CameraMode;

mod math;

pub(crate) use math::CameraPose;
use math::{follow_pose, orbit_pose, path_tangent, sample_path};

/// What the cinema camera is currently doing. `Inactive` means the whole
/// feature is dormant and the drive system is a no-op.
#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) enum CinemaMode {
    #[default]
    Inactive,
    /// Sit behind/above a moving target and keep it framed.
    Follow { target: u32, dist: f32, height: f32 },
    /// Aim at a target reference without moving the camera.
    LookAtTarget { target: u32 },
    /// Aim at a fixed world point without moving the camera.
    LookAtPoint { point: Vec3 },
    /// Orbit a target at a fixed radius.
    Orbit {
        target: u32,
        radius: f32,
        deg_per_sec: f32,
        angle: f32,
    },
    /// Dolly along waypoints over a duration, optionally trailing a target.
    Path(CinemaPath),
}

/// State for a `cam path` dolly move.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct CinemaPath {
    pub(crate) waypoints: Vec<Vec3>,
    pub(crate) seconds: f32,
    pub(crate) elapsed: f32,
    /// When set, the camera looks at this reference instead of along its
    /// direction of travel.
    pub(crate) follow_target: Option<u32>,
    /// Latches once the move has run its full duration, so `cinema path done`
    /// is logged exactly once.
    pub(crate) done: bool,
}

/// The single source of truth for the cinema camera. Default is fully
/// inactive, so the plugin adds no behaviour until a command engages it.
#[derive(Resource, Default)]
pub(crate) struct CinemaState {
    pub(crate) mode: CinemaMode,
    /// Camera mode to restore on `cam release`. `Some` only when cinema
    /// itself forced the camera out of FPS mode; `None` when the user was
    /// already free-flying (`tfc`) and there is nothing to hand back.
    pub(crate) restore_mode: Option<CameraMode>,
}

impl CinemaState {
    pub(crate) fn is_active(&self) -> bool {
        !matches!(self.mode, CinemaMode::Inactive)
    }

    pub(crate) fn mode_label(&self) -> &'static str {
        match self.mode {
            CinemaMode::Inactive => "inactive",
            CinemaMode::Follow { .. } => "follow",
            CinemaMode::LookAtTarget { .. } | CinemaMode::LookAtPoint { .. } => "lookat",
            CinemaMode::Orbit { .. } => "orbit",
            CinemaMode::Path(_) => "path",
        }
    }

    /// The reference this mode tracks, if any, for `cam status`/logs.
    pub(crate) fn target_form_id(&self) -> Option<u32> {
        match &self.mode {
            CinemaMode::Follow { target, .. }
            | CinemaMode::LookAtTarget { target }
            | CinemaMode::Orbit { target, .. } => Some(*target),
            CinemaMode::Path(path) => path.follow_target,
            CinemaMode::Inactive | CinemaMode::LookAtPoint { .. } => None,
        }
    }
}

pub(crate) struct CinemaPlugin;

impl Plugin for CinemaPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CinemaState>().add_systems(
            PostUpdate,
            drive_cinema_camera
                .before(TransformSystems::Propagate)
                .run_if(in_state(AppState::InGame)),
        );
    }
}

/// Per-frame camera drive. Runs in `PostUpdate` before transform propagation
/// so the camera's `GlobalTransform` is derived from the pose we write this
/// frame. Reads only target transforms and writes only the camera
/// `Transform`; a completely dormant no-op while `CinemaState` is inactive.
fn drive_cinema_camera(
    time: Res<Time>,
    refs: Res<RefRegistry>,
    mut state: ResMut<CinemaState>,
    targets: Query<&GlobalTransform>,
    mut cameras: Query<(&mut Transform, Option<&ChildOf>), With<Camera3d>>,
) {
    if !state.is_active() {
        return;
    }
    let Ok((mut camera, parent)) = cameras.single_mut() else {
        return;
    };
    // In FPS mode the camera is parented to the player and its pose is owned
    // by the interpolation system; cinema only drives the free (unparented)
    // camera, so it yields rather than fight that.
    if parent.is_some() {
        return;
    }

    // World pose (position, forward) of a tracked reference, resolved fresh
    // each frame so a despawn/cell swap simply stops updating.
    let pose_of = |form_id: u32| -> Option<(Vec3, Vec3)> {
        let entity = refs.entity_of(form_id)?;
        let global = targets.get(entity).ok()?;
        let (_, rotation, translation) = global.to_scale_rotation_translation();
        Some((translation, rotation * Vec3::NEG_Z))
    };

    let dt = time.delta_secs();
    match &mut state.mode {
        CinemaMode::Inactive => {}
        CinemaMode::Follow {
            target,
            dist,
            height,
        } => {
            if let Some((position, forward)) = pose_of(*target) {
                apply_pose(&mut camera, follow_pose(position, forward, *dist, *height));
            }
        }
        CinemaMode::LookAtTarget { target } => {
            if let Some((position, _)) = pose_of(*target)
                && position != camera.translation
            {
                camera.look_at(position, Vec3::Y);
            }
        }
        CinemaMode::LookAtPoint { point } => {
            let point = *point;
            if point != camera.translation {
                camera.look_at(point, Vec3::Y);
            }
        }
        CinemaMode::Orbit {
            target,
            radius,
            deg_per_sec,
            angle,
        } => {
            if let Some((center, _)) = pose_of(*target) {
                *angle += deg_per_sec.to_radians() * dt;
                let height = *radius * 0.3;
                apply_pose(&mut camera, orbit_pose(center, *radius, *angle, height));
            }
        }
        CinemaMode::Path(path) => {
            path.elapsed += dt;
            let position = sample_path(&path.waypoints, path.elapsed, path.seconds);
            camera.translation = position;
            let focus = path
                .follow_target
                .and_then(|target| pose_of(target).map(|(target_position, _)| target_position))
                .or_else(|| {
                    path_tangent(&path.waypoints, path.elapsed, path.seconds)
                        .map(|tangent| position + tangent)
                });
            if let Some(focus) = focus
                && focus != position
            {
                camera.look_at(focus, Vec3::Y);
            }
            if !path.done && path.elapsed >= path.seconds {
                path.done = true;
                info!("cinema path done");
            }
        }
    }
}

fn apply_pose(camera: &mut Transform, pose: CameraPose) {
    *camera = Transform::from_translation(pose.eye).looking_at(pose.focus, Vec3::Y);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_state_is_inactive() {
        let state = CinemaState::default();
        assert!(!state.is_active());
        assert_eq!(state.mode_label(), "inactive");
        assert_eq!(state.target_form_id(), None);
        assert!(state.restore_mode.is_none());
    }

    #[test]
    fn active_modes_report_label_and_target() {
        let follow = CinemaState {
            mode: CinemaMode::Follow {
                target: 0x0005_cf10,
                dist: 4.0,
                height: 2.0,
            },
            restore_mode: None,
        };
        assert!(follow.is_active());
        assert_eq!(follow.mode_label(), "follow");
        assert_eq!(follow.target_form_id(), Some(0x0005_cf10));

        let point = CinemaState {
            mode: CinemaMode::LookAtPoint {
                point: Vec3::splat(1.0),
            },
            restore_mode: None,
        };
        assert_eq!(point.mode_label(), "lookat");
        assert_eq!(point.target_form_id(), None);
    }
}
