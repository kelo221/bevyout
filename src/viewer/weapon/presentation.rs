use std::f32::consts::PI;

use bevy::gltf::{Gltf, GltfAssetLabel};
use bevy::prelude::*;

use bevyout_core::weapon::WeaponAction;

use super::PlayerWeaponRuntime;
use crate::app_state::GameplayModal;
use crate::viewer::WorldAssetRoot;
use crate::viewer::player::{CameraMode, CameraModeState};

const IDLE_TRANSLATION: Vec3 = Vec3::new(0.28, -0.23, -0.58);
// Native NIF conversion already bakes the asset's -90° X basis correction;
// Apply the remaining +90° Y first-person orientation here. The -90° Z roll
// is composed after that yaw so it rotates around the now-forward barrel axis
// and keeps the left-handed weapon basis upright in the camera view.
const IDLE_ROTATION: Vec3 = Vec3::new(0.0, 0.5 * PI, -0.5 * PI);
const MUZZLE_TRANSLATION: Vec3 = Vec3::new(0.20, -0.15, -0.82);
const MUZZLE_LIGHT_INTENSITY: f32 = 9_000.0;

#[derive(Component)]
pub(super) struct WeaponViewmodelRoot;

/// The root GLTF handle is kept alongside the spawned scene handle so the
/// weapon animation adapter can discover named clips after the scene's
/// `AnimationPlayer` entities have been instantiated.
#[derive(Component)]
pub(super) struct WeaponViewmodelSource(pub(super) Handle<Gltf>);

#[derive(Component)]
pub(super) struct WeaponMuzzleLight;

type ViewmodelTransformQuery<'w, 's> = Query<
    'w,
    's,
    (
        &'static Transform,
        &'static mut GlobalTransform,
        Option<&'static ChildOf>,
        Option<&'static WeaponMuzzleLight>,
    ),
    Without<Camera3d>,
>;

pub(super) fn sync_viewmodel(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mode: Res<CameraModeState>,
    modal: Res<State<GameplayModal>>,
    cameras: Query<(Entity, &Camera), With<Camera3d>>,
    mut runtime: ResMut<PlayerWeaponRuntime>,
) {
    let visible = mode.mode == CameraMode::Fps && *modal.get() == GameplayModal::None;
    let desired_asset = visible
        .then(|| {
            runtime
                .equipped
                .as_ref()
                .and_then(|weapon| weapon.viewmodel_asset_path.clone())
        })
        .flatten();
    let active_camera = cameras
        .iter()
        .find_map(|(entity, camera)| camera.is_active.then_some(entity));

    if desired_asset.is_none() || active_camera.is_none() {
        despawn_presentation(&mut commands, &mut runtime);
        return;
    }
    if should_retain_viewmodel(
        desired_asset.as_deref(),
        runtime.spawned_viewmodel_asset_path.as_deref(),
        runtime.viewmodel_entity.is_some(),
        runtime.muzzle_light_entity.is_some(),
    ) {
        return;
    }
    if runtime.viewmodel_entity.is_some()
        || runtime.muzzle_light_entity.is_some()
        || runtime.spawned_viewmodel_asset_path.is_some()
    {
        despawn_presentation(&mut commands, &mut runtime);
    }
    let camera = active_camera.expect("checked above");
    let asset_path = desired_asset.expect("checked above");
    let gltf = asset_server.load::<Gltf>(asset_path.clone());
    let viewmodel = commands
        .spawn((
            WeaponViewmodelRoot,
            WeaponViewmodelSource(gltf),
            WorldAssetRoot(
                asset_server.load(GltfAssetLabel::Scene(0).from_asset(asset_path.clone())),
            ),
            idle_transform(),
            ChildOf(camera),
        ))
        .id();
    let light = commands
        .spawn((
            WeaponMuzzleLight,
            PointLight {
                color: Color::srgb(1.0, 0.62, 0.25),
                intensity: MUZZLE_LIGHT_INTENSITY,
                range: 4.0,
                shadow_maps_enabled: false,
                ..default()
            },
            Transform::from_translation(MUZZLE_TRANSLATION),
            Visibility::Hidden,
            ChildOf(camera),
        ))
        .id();
    runtime.viewmodel_entity = Some(viewmodel);
    runtime.spawned_viewmodel_asset_path = Some(asset_path.clone());
    runtime.muzzle_light_entity = Some(light);
    info!(
        "weapon viewmodel {:08x} asset={asset_path}",
        runtime
            .equipped
            .as_ref()
            .map_or(0, |weapon| weapon.base_form_id)
    );
}

pub(super) fn animate_viewmodel(
    time: Res<Time<Real>>,
    mut runtime: ResMut<PlayerWeaponRuntime>,
    mut roots: Query<&mut Transform, With<WeaponViewmodelRoot>>,
    mut lights: Query<&mut Visibility, With<WeaponMuzzleLight>>,
) {
    let (action, progress) = runtime
        .state
        .as_ref()
        .map_or((WeaponAction::Idle, 0.0), |state| {
            (state.action(), state.action_progress())
        });
    if let Some(root) = runtime.viewmodel_entity
        && let Ok(mut transform) = roots.get_mut(root)
    {
        *transform = action_transform(action, progress);
    }
    if let Some(light) = runtime.muzzle_light_entity
        && let Ok(mut visibility) = lights.get_mut(light)
    {
        *visibility = if runtime.muzzle_flash_remaining > 0.0 {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
    runtime.muzzle_flash_remaining = (runtime.muzzle_flash_remaining - time.delta_secs()).max(0.0);
    if let Some(state) = runtime.state.as_mut() {
        state.advance(time.delta_secs());
    }
}

/// Reapply the interpolated camera pose to the viewmodel hierarchy.
///
/// The player camera writes its render-only interpolated pose after Bevy's
/// normal transform propagation. Since the viewmodel is a camera child, its
/// scene hierarchy would otherwise keep the pre-interpolation globals for one
/// frame, which is visible as stutter while moving. Propagating this small
/// first-person hierarchy here keeps it in lockstep with the rendered camera
/// without changing authoritative gameplay transforms.
pub(super) fn interpolate_viewmodel_globals(
    cameras: Query<&GlobalTransform, With<Camera3d>>,
    children: Query<&Children>,
    mut transforms: ViewmodelTransformQuery<'_, '_>,
    roots: Query<(Entity, &ChildOf), With<WeaponViewmodelRoot>>,
) {
    for (root, parent) in &roots {
        let Ok(camera_global) = cameras.get(parent.0) else {
            continue;
        };
        let mut pending = vec![(root, *camera_global)];
        while let Some((entity, parent_global)) = pending.pop() {
            let world = {
                let Ok((local, mut global, _, _)) = transforms.get_mut(entity) else {
                    continue;
                };
                let world = compose_global_transform(parent_global, *local);
                *global = world;
                world
            };
            if let Ok(child_entities) = children.get(entity) {
                pending.extend(child_entities.iter().map(|child| (child, world)));
            }
        }
    }

    for (local, mut global, parent, muzzle_light) in &mut transforms {
        let (Some(parent), Some(_)) = (parent, muzzle_light) else {
            continue;
        };
        let Ok(camera_global) = cameras.get(parent.0) else {
            continue;
        };
        *global = compose_global_transform(*camera_global, *local);
    }
}

fn despawn_presentation(commands: &mut Commands, runtime: &mut PlayerWeaponRuntime) {
    for entity in [
        runtime.viewmodel_entity.take(),
        runtime.muzzle_light_entity.take(),
    ]
    .into_iter()
    .flatten()
    {
        commands.entity(entity).despawn();
    }
    runtime.spawned_viewmodel_asset_path = None;
}

fn should_retain_viewmodel(
    desired_asset: Option<&str>,
    spawned_asset: Option<&str>,
    has_viewmodel: bool,
    has_muzzle_light: bool,
) -> bool {
    desired_asset.is_some() && desired_asset == spawned_asset && has_viewmodel && has_muzzle_light
}

fn compose_global_transform(parent: GlobalTransform, local: Transform) -> GlobalTransform {
    parent.mul_transform(local)
}

fn idle_transform() -> Transform {
    let rotation = Quat::from_rotation_z(IDLE_ROTATION.z)
        * Quat::from_rotation_y(IDLE_ROTATION.y)
        * Quat::from_rotation_x(IDLE_ROTATION.x);
    Transform::from_translation(IDLE_TRANSLATION).with_rotation(rotation)
}

fn action_transform(action: WeaponAction, progress: f32) -> Transform {
    let mut transform = idle_transform();
    let arc = (progress.clamp(0.0, 1.0) * PI).sin();
    match action {
        WeaponAction::Idle => {}
        WeaponAction::Firing => {
            transform.translation += Vec3::new(0.0, -0.01 * arc, 0.08 * arc);
            transform.rotate_local_x(-0.16 * arc);
        }
        WeaponAction::Reloading => {
            // Keep the reload readable without moving the large first-person
            // mesh outside the camera. These are intentionally camera-local
            // offsets; the authored weapon basis already supplies its yaw and
            // roll, so a large local rotation would swing it off-screen.
            transform.translation += Vec3::new(0.03 * arc, -0.08 * arc, 0.02 * arc);
            transform.rotate_local_z(0.12 * arc);
            transform.rotate_local_x(-0.08 * arc);
        }
    }
    transform
}

#[cfg(test)]
#[path = "tests/presentation.rs"]
mod tests;
