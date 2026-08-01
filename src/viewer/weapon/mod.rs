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
use bevyout_core::combat::CombatRngState;
use bevyout_core::combat::WeaponConditionPolicy;
use bevyout_core::item_transaction::{
    CombatTransactionOutcome, CombatTransactionReceipt, HolderId, ItemInstanceId, TransactionError,
    WeaponReloadRequest,
};
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
    pub(crate) max_condition: Option<u32>,
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
            max_condition,
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
            max_condition: *max_condition,
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

    fn condition_policy(&self) -> WeaponConditionPolicy {
        WeaponConditionPolicy::new(self.max_condition)
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
    BlockedJammed,
    Jammed,
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
            Self::BlockedJammed => "blocked_jammed",
            Self::Jammed => "jammed",
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
    pub(crate) last_combat: Option<CombatTransactionReceipt>,
    pub(crate) last_combat_block: Option<String>,
    pub(crate) viewmodel_entity: Option<Entity>,
    pub(crate) spawned_viewmodel_asset_path: Option<String>,
    pub(crate) muzzle_light_entity: Option<Entity>,
    pub(crate) muzzle_flash_remaining: f32,
}

#[derive(Resource, Clone, Debug, Default)]
pub(crate) struct CombatRngRuntime(pub(crate) CombatRngState);

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

#[derive(Message, Clone, Copy, Debug, Default)]
pub(crate) struct ClearWeaponJamRequested;

#[derive(Message, Clone, Debug)]
struct AcceptedWeaponShot {
    shot_index: u64,
    weapon: EquippedWeapon,
    damage: f32,
}

#[derive(SystemParam)]
struct WeaponActionMessages<'w, 's> {
    fire_requests: MessageReader<'w, 's, FireWeaponRequested>,
    reload_requests: MessageReader<'w, 's, ReloadWeaponRequested>,
    clear_jam_requests: MessageReader<'w, 's, ClearWeaponJamRequested>,
    accepted_shots: MessageWriter<'w, AcceptedWeaponShot>,
    sounds: MessageWriter<'w, super::audio::PlaySound>,
}

#[derive(SystemParam)]
struct WeaponActionResources<'w> {
    inventory: ResMut<'w, PlayerInventory>,
    equipment: ResMut<'w, PlayerEquipment>,
    canonical: ResMut<'w, CanonicalItemLedger>,
    runtime: ResMut<'w, PlayerWeaponRuntime>,
    combat_rng: ResMut<'w, CombatRngRuntime>,
}

pub(crate) struct WeaponPlugin;

impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerWeaponRuntime>()
            .init_resource::<CombatRngRuntime>()
            .init_resource::<animation::PendingWeaponAnimationDiscovery>()
            .add_message::<FireWeaponRequested>()
            .add_message::<ReloadWeaponRequested>()
            .add_message::<ClearWeaponJamRequested>()
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
    runtime.last_combat = None;
    runtime.last_combat_block = None;
    runtime.muzzle_flash_remaining = 0.0;
}

fn process_action_requests(
    messages: WeaponActionMessages,
    cameras: Query<&GlobalTransform, With<Camera3d>>,
    manifest: Option<Res<crate::viewer::LoadedSceneManifest>>,
    mut action: WeaponActionResources,
) {
    let WeaponActionMessages {
        mut fire_requests,
        mut reload_requests,
        mut clear_jam_requests,
        mut accepted_shots,
        mut sounds,
    } = messages;
    let inventory = &mut action.inventory;
    let equipment = &mut action.equipment;
    let canonical = &mut action.canonical;
    let runtime = &mut action.runtime;
    let combat_rng = &mut action.combat_rng.0;

    for _ in clear_jam_requests.read() {
        let Some(weapon_id) = runtime.equipped_instance_id else {
            runtime.last_combat_block = Some("no_weapon".into());
            continue;
        };
        let id = canonical.ledger.next_transaction_id();
        match canonical
            .ledger
            .clear_weapon_jam_with_id(id, HolderId::Player, weapon_id)
        {
            Ok(receipt) => {
                runtime.last_combat = Some(receipt);
                runtime.last_combat_block = None;
                if let Some(condition) = runtime
                    .last_combat
                    .as_ref()
                    .and_then(|receipt| receipt.condition_after)
                {
                    equipment.set_equipped_weapon_condition(Some(condition));
                }
                canonical.write_player_projection(inventory);
            }
            Err(error) => runtime.last_combat_block = Some(combat_error_label(&error)),
        }
    }

    for _ in reload_requests.read() {
        let can_start = runtime.action() == WeaponAction::Idle;
        let reload_result = if can_start {
            Some(
                match runtime.equipped.as_ref().zip(runtime.equipped_instance_id) {
                    Some((weapon, weapon_id)) => match weapon.ammo_form_id {
                        Some(ammo_form_id) => {
                            let id = canonical.ledger.next_transaction_id();
                            canonical.ledger.reload_weapon_with_policy(
                                id,
                                HolderId::Player,
                                weapon_id,
                                WeaponReloadRequest {
                                    ammo_form_id,
                                    capacity: weapon.magazine_capacity,
                                    policy: weapon.condition_policy(),
                                },
                                combat_rng,
                            )
                        }
                        None => Err(TransactionError::IncompatibleAmmo),
                    },
                    None => Err(TransactionError::InsufficientItems),
                },
            )
        } else {
            None
        };

        let decision = reload_result
            .as_ref()
            .and_then(|result| result.as_ref().ok())
            .filter(|receipt| receipt.outcome == CombatTransactionOutcome::Reloaded)
            .and_then(|_| runtime.state.as_mut().map(WeaponState::request_reload));
        match reload_result {
            Some(Ok(receipt)) => {
                runtime.last_combat = Some(receipt.clone());
                runtime.last_combat_block = None;
                equipment.set_equipped_weapon_condition(receipt.condition_after);
                runtime.equipped_instance_id = Some(receipt.weapon_id);
            }
            Some(Err(error)) => {
                runtime.last_combat_block = Some(combat_error_label(&error));
            }
            None => {}
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
            let Some(weapon_id) = runtime.equipped_instance_id else {
                runtime.last_fire = FireReport {
                    status: FireStatus::NoWeapon,
                    ..Default::default()
                };
                runtime.last_fire_sound_form_id = None;
                runtime.last_muzzle_flash_seconds = None;
                continue;
            };
            let Some(weapon) = runtime.equipped.clone() else {
                runtime.last_fire = FireReport {
                    status: FireStatus::NoWeapon,
                    ..Default::default()
                };
                runtime.last_fire_sound_form_id = None;
                runtime.last_muzzle_flash_seconds = None;
                continue;
            };
            let id = canonical.ledger.next_transaction_id();
            let result = canonical.ledger.fire_weapon_with_policy(
                id,
                HolderId::Player,
                weapon_id,
                weapon.damage,
                weapon.condition_policy(),
                combat_rng,
            );
            match result {
                Ok(receipt) if receipt.outcome == CombatTransactionOutcome::Jammed => {
                    runtime.last_combat = Some(receipt);
                    runtime.last_combat_block = None;
                    equipment.set_equipped_weapon_condition(
                        runtime
                            .last_combat
                            .as_ref()
                            .and_then(|receipt| receipt.condition_after),
                    );
                    runtime.last_fire = FireReport {
                        status: FireStatus::Jammed,
                        ..Default::default()
                    };
                    runtime.last_fire_sound_form_id = None;
                    runtime.last_muzzle_flash_seconds = None;
                    canonical.write_player_projection(inventory);
                }
                Ok(receipt) => {
                    runtime.last_combat = Some(receipt.clone());
                    runtime.last_combat_block = None;
                    equipment.set_equipped_weapon_condition(receipt.condition_after);
                    canonical.write_player_projection(inventory);
                    let decision = runtime
                        .state
                        .as_mut()
                        .expect("weapon state checked above")
                        .request_fire();
                    let FireDecision::Fired { shot_index } = decision else {
                        runtime.last_fire = FireReport {
                            status: FireStatus::BlockedFiring,
                            ..Default::default()
                        };
                        continue;
                    };
                    let damage = receipt
                        .damage_milli
                        .map_or(weapon.damage, |milli| milli as f32 / 1_000.0);
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
                    accepted_shots.write(AcceptedWeaponShot {
                        shot_index,
                        weapon,
                        damage,
                    });
                }
                Err(error) => {
                    runtime.last_combat_block = Some(combat_error_label(&error));
                    runtime.last_fire = FireReport {
                        status: match error {
                            TransactionError::Jammed(_) => FireStatus::BlockedJammed,
                            TransactionError::InsufficientItems => FireStatus::BlockedEmpty,
                            _ => FireStatus::BlockedEmpty,
                        },
                        ..Default::default()
                    };
                    runtime.last_fire_sound_form_id = None;
                    runtime.last_muzzle_flash_seconds = None;
                }
            }
            continue;
        }
        match runtime
            .state
            .as_mut()
            .expect("weapon state checked above")
            .request_fire()
        {
            FireDecision::Fired { .. } => unreachable!("non-idle action cannot fire"),
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

fn combat_error_label(error: &TransactionError) -> String {
    match error {
        TransactionError::Jammed(reason) => format!("jammed_{}", reason.label()),
        TransactionError::InsufficientItems => "empty_or_no_reserve".into(),
        TransactionError::InvalidMagazine => "invalid_magazine".into(),
        TransactionError::InvalidCombatRng => "invalid_combat_rng".into(),
        TransactionError::InvalidWeaponCondition => "invalid_weapon_condition".into(),
        other => format!("{other:?}"),
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
#[path = "tests/mod.rs"]
mod tests;
