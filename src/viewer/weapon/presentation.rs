use std::f32::consts::PI;

use bevy::gltf::GltfAssetLabel;
use bevy::prelude::*;

use bevyout_core::weapon::WeaponAction;

use super::PlayerWeaponRuntime;
use crate::app_state::GameplayModal;
use crate::viewer::WorldAssetRoot;
use crate::viewer::player::{CameraMode, CameraModeState};

const IDLE_TRANSLATION: Vec3 = Vec3::new(0.28, -0.23, -0.58);
const IDLE_ROTATION: Vec3 = Vec3::new(0.0, PI, 0.0);
const MUZZLE_TRANSLATION: Vec3 = Vec3::new(0.20, -0.15, -0.82);

#[derive(Component)]
pub(super) struct WeaponViewmodelRoot;

#[derive(Component)]
pub(super) struct WeaponMuzzleLight;

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
    if runtime.viewmodel_entity.is_some() {
        return;
    }
    let camera = active_camera.expect("checked above");
    let asset_path = desired_asset.expect("checked above");
    let viewmodel = commands
        .spawn((
            WeaponViewmodelRoot,
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
                intensity: 45_000.0,
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
}

fn idle_transform() -> Transform {
    Transform::from_translation(IDLE_TRANSLATION).with_rotation(Quat::from_euler(
        EulerRot::XYZ,
        IDLE_ROTATION.x,
        IDLE_ROTATION.y,
        IDLE_ROTATION.z,
    ))
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
            transform.translation += Vec3::new(0.05 * arc, -0.28 * arc, 0.08 * arc);
            transform.rotate_local_z(0.95 * arc);
            transform.rotate_local_x(-0.25 * arc);
        }
    }
    transform
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn recoil_and_reload_are_distinct_camera_local_poses() {
        let idle = action_transform(WeaponAction::Idle, 0.5);
        let recoil = action_transform(WeaponAction::Firing, 0.5);
        let reload = action_transform(WeaponAction::Reloading, 0.5);
        assert_ne!(recoil.translation, idle.translation);
        assert_ne!(reload.translation, recoil.translation);
        assert!(reload.translation.y < idle.translation.y);
    }
}
