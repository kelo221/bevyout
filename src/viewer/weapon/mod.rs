//! First-person player-weapon runtime adapter.
//!
//! `bevyout-core::weapon` owns deterministic action and damage policy. This
//! module only adapts equipment, input, prepared assets, audio, and Bevy
//! presentation to that policy.

mod animation;
mod hitscan;
mod presentation;

use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use bevy::window::{CursorGrabMode, CursorOptions, PrimaryWindow};
use bevyout_core::item_transaction::{HolderId, ItemInstanceId};
use bevyout_core::weapon::{
    FireDecision, ReloadDecision, WeaponAction, WeaponDefinition, WeaponState,
};

use crate::app_state::{AppState, GameplayModal};
use crate::vsa::{PreparedItemCatalog, PreparedItemDefinition, PreparedItemStats};

use super::interaction::{CanonicalItemLedger, PlayerEquipment, PlayerInventory};

const DEFAULT_HITSCAN_RANGE_METERS: f32 = 100.0;

#[derive(Clone, Debug)]
pub(crate) struct EquippedWeapon {
    pub(crate) base_form_id: u32,
    pub(crate) label: String,
    pub(crate) damage: f32,
    pub(crate) range_meters: f32,
    pub(crate) ammo_form_id: Option<u32>,
    pub(crate) magazine_capacity: u16,
    pub(crate) viewmodel_asset_path: Option<String>,
    pub(crate) fire_sound_3d_form_id: Option<u32>,
    pub(crate) fire_sound_2d_form_id: Option<u32>,
}

impl EquippedWeapon {
    fn from_item(item: &PreparedItemDefinition) -> Option<Self> {
        let PreparedItemStats::Weapon {
            damage,
            clip_size,
            ammo_form_id,
            first_person_asset_path,
            fire_sound_3d_form_id,
            fire_sound_2d_form_id,
            ..
        } = &item.stats
        else {
            return None;
        };
        Some(Self {
            base_form_id: item.base_form_id,
            label: item
                .display_name
                .clone()
                .or_else(|| item.editor_id.clone())
                .unwrap_or_else(|| format!("{:08X}", item.base_form_id)),
            damage: f32::from(damage.unwrap_or_default()),
            range_meters: DEFAULT_HITSCAN_RANGE_METERS,
            ammo_form_id: *ammo_form_id,
            magazine_capacity: u16::from(clip_size.unwrap_or_default()),
            viewmodel_asset_path: first_person_asset_path
                .clone()
                .or_else(|| item.world_asset_path.clone()),
            fire_sound_3d_form_id: *fire_sound_3d_form_id,
            fire_sound_2d_form_id: *fire_sound_2d_form_id,
        })
    }

    fn definition(&self) -> WeaponDefinition {
        WeaponDefinition::new(self.damage, self.range_meters)
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) enum FireStatus {
    #[default]
    Never,
    NoWeapon,
    BlockedFiring,
    BlockedReloading,
    BlockedEmpty,
    Miss,
    WorldHit,
    ActorHit,
    ActorKilled,
    ActorStateUnavailable,
}

impl FireStatus {
    pub(crate) const fn label(self) -> &'static str {
        match self {
            Self::Never => "never",
            Self::NoWeapon => "no_weapon",
            Self::BlockedFiring => "blocked_firing",
            Self::BlockedReloading => "blocked_reloading",
            Self::BlockedEmpty => "blocked_empty",
            Self::Miss => "miss",
            Self::WorldHit => "world_hit",
            Self::ActorHit => "actor_hit",
            Self::ActorKilled => "actor_killed",
            Self::ActorStateUnavailable => "actor_state_unavailable",
        }
    }
}

#[derive(Clone, Debug, Default)]
pub(crate) struct FireReport {
    pub(crate) status: FireStatus,
    pub(crate) shot_index: Option<u64>,
    pub(crate) target_reference_form_id: Option<u32>,
    pub(crate) hit_distance: Option<f32>,
    pub(crate) applied_damage: Option<f32>,
    pub(crate) remaining_health: Option<f32>,
}

#[derive(Resource, Default)]
pub(crate) struct PlayerWeaponRuntime {
    pub(crate) equipped: Option<EquippedWeapon>,
    pub(crate) equipped_instance_id: Option<ItemInstanceId>,
    pub(crate) state: Option<WeaponState>,
    pub(crate) last_fire: FireReport,
    pub(crate) last_fire_sound_form_id: Option<u32>,
    pub(crate) last_reload_sound_form_id: Option<u32>,
    pub(crate) last_muzzle_flash_seconds: Option<f32>,
    pub(crate) last_reload: Option<ReloadDecision>,
    pub(crate) viewmodel_entity: Option<Entity>,
    pub(crate) spawned_viewmodel_asset_path: Option<String>,
    pub(crate) muzzle_light_entity: Option<Entity>,
    pub(crate) muzzle_flash_remaining: f32,
}

impl PlayerWeaponRuntime {
    pub(crate) fn action(&self) -> WeaponAction {
        self.state
            .as_ref()
            .map_or(WeaponAction::Idle, WeaponState::action)
    }

    pub(crate) fn shots_fired(&self) -> u64 {
        self.state.as_ref().map_or(0, WeaponState::shots_fired)
    }
}

#[derive(Message, Clone, Copy, Debug, Default)]
pub(crate) struct FireWeaponRequested;

#[derive(Message, Clone, Copy, Debug, Default)]
pub(crate) struct ReloadWeaponRequested;

#[derive(Message, Clone, Debug)]
struct AcceptedWeaponShot {
    shot_index: u64,
    weapon: EquippedWeapon,
}

#[derive(SystemParam)]
struct WeaponActionResources<'w> {
    inventory: ResMut<'w, PlayerInventory>,
    canonical: ResMut<'w, CanonicalItemLedger>,
    runtime: ResMut<'w, PlayerWeaponRuntime>,
}

pub(crate) struct WeaponPlugin;

impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerWeaponRuntime>()
            .init_resource::<animation::PendingWeaponAnimationDiscovery>()
            .add_message::<FireWeaponRequested>()
            .add_message::<ReloadWeaponRequested>()
            .add_message::<AcceptedWeaponShot>()
            .add_systems(
                Update,
                collect_weapon_input
                    .in_set(super::plugins::ViewerSet::Input)
                    .run_if(in_state(AppState::InGame))
                    .run_if(in_state(GameplayModal::None)),
            )
            .add_systems(
                Update,
                (
                    sync_equipped_weapon,
                    process_action_requests,
                    hitscan::resolve_accepted_shots,
                )
                    .chain()
                    .in_set(super::plugins::ViewerSet::Interaction)
                    .run_if(in_state(AppState::InGame))
                    .run_if(in_state(GameplayModal::None)),
            )
            .add_systems(
                Update,
                (
                    presentation::sync_viewmodel,
                    animation::discover_animation_players,
                    animation::resolve_pending_animation_discovery,
                    presentation::animate_viewmodel,
                    animation::drive_viewmodel_animations,
                )
                    .chain()
                    .in_set(super::plugins::ViewerSet::WorldSync)
                    .run_if(in_state(AppState::InGame)),
            );
        app.add_systems(
            PostUpdate,
            presentation::interpolate_viewmodel_globals
                .after(super::player::interpolate_fps_camera)
                .run_if(in_state(AppState::InGame)),
        );
    }
}

fn collect_weapon_input(
    keys: Res<ButtonInput<KeyCode>>,
    buttons: Res<ButtonInput<MouseButton>>,
    windows: Query<&CursorOptions, With<PrimaryWindow>>,
    mut fire: MessageWriter<FireWeaponRequested>,
    mut reload: MessageWriter<ReloadWeaponRequested>,
) {
    let captured = windows
        .single()
        .is_ok_and(|options| options.grab_mode == CursorGrabMode::Locked);
    if captured && buttons.just_pressed(MouseButton::Left) {
        fire.write_default();
    }
    if keys.just_pressed(KeyCode::KeyR) {
        reload.write_default();
    }
}

fn sync_equipped_weapon(
    equipment: Res<PlayerEquipment>,
    inventory: Res<PlayerInventory>,
    catalog: Res<PreparedItemCatalog>,
    mut canonical: ResMut<CanonicalItemLedger>,
    mut runtime: ResMut<PlayerWeaponRuntime>,
) {
    let desired_form_id = equipment.equipped_weapon().map(|key| key.base_form_id);
    if canonical.sync_player(&inventory.legacy_snapshot()).is_err() {
        return;
    }
    let desired_instance_id = desired_form_id.and_then(|form_id| {
        let holder = canonical.ledger.holders().get(&HolderId::Player)?;
        canonical
            .ledger
            .bindings()
            .get(&HolderId::Player)
            .and_then(|binding| binding.equipped)
            .filter(|item_id| {
                holder
                    .find(*item_id)
                    .is_some_and(|item| item.base_form_id == form_id)
            })
            .or_else(|| {
                holder
                    .items
                    .iter()
                    .find(|item| item.base_form_id == form_id)
                    .map(|item| item.id)
            })
    });
    if let Some(item_id) = desired_instance_id
        && canonical
            .ledger
            .bindings()
            .get(&HolderId::Player)
            .and_then(|binding| binding.equipped)
            != Some(item_id)
    {
        let _ = canonical.ledger.unequip(HolderId::Player);
        let _ = canonical.ledger.equip(HolderId::Player, item_id);
    }
    if runtime.equipped.as_ref().map(|weapon| weapon.base_form_id) == desired_form_id
        && runtime.equipped_instance_id == desired_instance_id
    {
        return;
    }
    runtime.equipped_instance_id = desired_instance_id;
    runtime.equipped = desired_form_id.and_then(|form_id| {
        catalog
            .items
            .iter()
            .find(|item| item.base_form_id == form_id)
            .and_then(EquippedWeapon::from_item)
    });
    runtime.state = runtime
        .equipped
        .as_ref()
        .map(|weapon| WeaponState::new(weapon.definition()));
    runtime.last_fire = FireReport::default();
    runtime.last_fire_sound_form_id = None;
    runtime.last_reload_sound_form_id = None;
    runtime.last_muzzle_flash_seconds = None;
    runtime.last_reload = None;
    runtime.muzzle_flash_remaining = 0.0;
}

fn process_action_requests(
    mut fire_requests: MessageReader<FireWeaponRequested>,
    mut reload_requests: MessageReader<ReloadWeaponRequested>,
    mut accepted_shots: MessageWriter<AcceptedWeaponShot>,
    mut sounds: MessageWriter<super::audio::PlaySound>,
    cameras: Query<&GlobalTransform, With<Camera3d>>,
    manifest: Option<Res<crate::viewer::LoadedSceneManifest>>,
    mut action: WeaponActionResources,
) {
    let inventory = &mut action.inventory;
    let canonical = &mut action.canonical;
    let runtime = &mut action.runtime;
    for _ in reload_requests.read() {
        let can_start = runtime.action() == WeaponAction::Idle;
        let reload_committed = if can_start {
            runtime
                .equipped
                .as_ref()
                .zip(runtime.equipped_instance_id)
                .and_then(|(weapon, weapon_id)| {
                    let ammo_form_id = weapon.ammo_form_id?;
                    let id = canonical.ledger.next_transaction_id();
                    canonical
                        .ledger
                        .reload_weapon_with_id(
                            id,
                            HolderId::Player,
                            weapon_id,
                            ammo_form_id,
                            weapon.magazine_capacity,
                        )
                        .ok()
                })
        } else {
            None
        };
        let decision = reload_committed
            .as_ref()
            .and_then(|_| runtime.state.as_mut().map(WeaponState::request_reload));
        if let Some(receipt) = reload_committed {
            runtime.equipped_instance_id = Some(receipt.weapon_id);
        }
        canonical.write_player_projection(inventory);
        let reload_sound_form_id = (decision == Some(ReloadDecision::Started))
            .then(|| runtime.equipped.as_ref())
            .flatten()
            .and_then(|weapon| {
                manifest
                    .as_deref()
                    .and_then(|manifest| reload_sound_form_id(manifest, weapon))
            });
        runtime.last_reload = decision;
        runtime.last_reload_sound_form_id = reload_sound_form_id;
        if let Some(form_id) = reload_sound_form_id {
            sounds.write(super::audio::PlaySound {
                form_id,
                position: None,
                gain_db: 0.0,
            });
        }
    }
    for _ in fire_requests.read() {
        if runtime.state.is_none() {
            runtime.last_fire = FireReport {
                status: FireStatus::NoWeapon,
                ..Default::default()
            };
            runtime.last_fire_sound_form_id = None;
            runtime.last_muzzle_flash_seconds = None;
            continue;
        }
        if runtime.action() == WeaponAction::Idle {
            let consumed = runtime.equipped_instance_id.is_some_and(|weapon_id| {
                canonical
                    .ledger
                    .consume_weapon_round(HolderId::Player, weapon_id)
                    .is_ok()
            });
            if !consumed {
                runtime.last_fire = FireReport {
                    status: FireStatus::BlockedEmpty,
                    ..Default::default()
                };
                runtime.last_fire_sound_form_id = None;
                runtime.last_muzzle_flash_seconds = None;
                continue;
            }
        }
        let decision = runtime
            .state
            .as_mut()
            .expect("weapon state checked above")
            .request_fire();
        match decision {
            FireDecision::Fired { shot_index } => {
                let Some(weapon) = runtime.equipped.clone() else {
                    runtime.last_fire = FireReport {
                        status: FireStatus::NoWeapon,
                        ..Default::default()
                    };
                    runtime.last_fire_sound_form_id = None;
                    runtime.last_muzzle_flash_seconds = None;
                    continue;
                };
                runtime.last_fire = FireReport {
                    status: FireStatus::Miss,
                    shot_index: Some(shot_index),
                    ..Default::default()
                };
                let muzzle_flash_seconds = bevyout_core::weapon::DEFAULT_MUZZLE_FLASH_SECONDS;
                runtime.muzzle_flash_remaining = muzzle_flash_seconds;
                runtime.last_muzzle_flash_seconds = Some(muzzle_flash_seconds);
                runtime.last_fire_sound_form_id = weapon
                    .fire_sound_2d_form_id
                    .or(weapon.fire_sound_3d_form_id);
                if let Some(form_id) = weapon.fire_sound_2d_form_id {
                    sounds.write(super::audio::PlaySound {
                        form_id,
                        position: None,
                        gain_db: 0.0,
                    });
                } else if let Some(form_id) = weapon.fire_sound_3d_form_id {
                    let position = cameras
                        .iter()
                        .next()
                        .map_or(Vec3::ZERO, GlobalTransform::translation);
                    sounds.write(super::audio::PlaySound::at(form_id, position));
                }
                accepted_shots.write(AcceptedWeaponShot { shot_index, weapon });
            }
            FireDecision::BlockedFiring => {
                runtime.last_fire = FireReport {
                    status: FireStatus::BlockedFiring,
                    ..Default::default()
                };
                runtime.last_fire_sound_form_id = None;
                runtime.last_muzzle_flash_seconds = None;
            }
            FireDecision::BlockedReloading => {
                runtime.last_fire = FireReport {
                    status: FireStatus::BlockedReloading,
                    ..Default::default()
                };
                runtime.last_fire_sound_form_id = None;
                runtime.last_muzzle_flash_seconds = None;
            }
        }
    }
}

fn reload_sound_form_id(
    manifest: &crate::viewer::LoadedSceneManifest,
    weapon: &EquippedWeapon,
) -> Option<u32> {
    let fire_form_id = weapon
        .fire_sound_2d_form_id
        .or(weapon.fire_sound_3d_form_id)?;
    let fire_editor_id = manifest
        .audio_clips
        .iter()
        .find(|clip| clip.form_id == fire_form_id)
        .and_then(|clip| clip.editor_id.as_deref())?;
    let stem = fire_editor_id
        .strip_suffix("Fire2D")
        .or_else(|| fire_editor_id.strip_suffix("Fire3D"))?;
    [
        "Reload",
        "ReloadOut",
        "ReloadInOut",
        "ReloadIn",
        "ReloadChamber",
    ]
    .iter()
    .find_map(|suffix| {
        let candidate = format!("{stem}{suffix}");
        manifest.audio_clips.iter().find_map(|clip| {
            clip.editor_id.as_deref().and_then(|editor_id| {
                editor_id
                    .eq_ignore_ascii_case(&candidate)
                    .then_some(clip.form_id)
            })
        })
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vsa::PreparedItemCategory;

    #[test]
    fn prepared_weapon_prefers_first_person_asset_and_keeps_action_audio() {
        let item = PreparedItemDefinition {
            base_form_id: 0x434f,
            record_kind: "WEAP".into(),
            category: PreparedItemCategory::Weapons,
            editor_id: Some("Weap10mmPistol".into()),
            display_name: Some("10mm Pistol".into()),
            source_model_path: None,
            icon_asset_path: None,
            world_asset_path: Some("assets/world.glb".into()),
            physics_asset_path: None,
            drop_collider: Default::default(),
            value: None,
            weight: None,
            quest_item: false,
            stats: PreparedItemStats::Weapon {
                damage: Some(9),
                max_condition: None,
                clip_size: Some(12),
                speed: None,
                reach: None,
                ammo_form_id: Some(0x4241),
                animation_type: Some(3),
                first_person_model_object_form_id: Some(0x100),
                first_person_asset_path: Some("assets/first.glb".into()),
                fire_sound_3d_form_id: Some(0x200),
                fire_sound_2d_form_id: Some(0x201),
            },
            audio: Default::default(),
        };
        let weapon = EquippedWeapon::from_item(&item).unwrap();
        assert_eq!(
            weapon.viewmodel_asset_path.as_deref(),
            Some("assets/first.glb")
        );
        assert_eq!(weapon.damage, 9.0);
        assert_eq!(weapon.fire_sound_2d_form_id, Some(0x201));
    }

    #[test]
    fn reload_sound_candidates_follow_fire_sound_family() {
        let fire_editor_id = "WPNPistol10mmFire2D";
        let stem = fire_editor_id.strip_suffix("Fire2D").unwrap();
        let candidates = [
            "Reload",
            "ReloadOut",
            "ReloadInOut",
            "ReloadIn",
            "ReloadChamber",
        ]
        .iter()
        .map(|suffix| format!("{stem}{suffix}"))
        .collect::<Vec<_>>();
        assert_eq!(candidates[1], "WPNPistol10mmReloadOut");
    }
}
