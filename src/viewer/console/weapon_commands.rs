//! Player-weapon inspection and MCP action controls (M5 wave 1).

use bevyout_core::item_transaction::HolderId;
use bevyout_core::weapon::{ReloadDecision, WeaponAction};

use super::*;
use crate::viewer::weapon::{FireWeaponRequested, PlayerWeaponRuntime, ReloadWeaponRequested};

const COMBAT_INSPECTION_SCHEMA_VERSION: u32 = 1;

pub(super) struct WeaponCommandProvider;

impl ConsoleCommandProvider for WeaponCommandProvider {
    fn register_commands(&self, registry: &mut ConsoleRegistry) -> Result<(), ConsoleError> {
        for command in [
            ConsoleCommand::new(
                "weaponstate",
                "weaponstate",
                "Report equipped player weapon, action, viewmodel, accepted-shot count, and last hitscan result.",
                weapon_state,
            ),
            ConsoleCommand::new(
                "weaponfire",
                "weaponfire",
                "Request one center-screen player-weapon shot through the normal action, audio, light, and hitscan path.",
                weapon_fire,
            )
            .mutating(),
            ConsoleCommand::new(
                "ammostate",
                "ammostate [player]",
                "Report canonical active-weapon magazine and reserve ammunition.",
                ammo_state,
            ),
            ConsoleCommand::new(
                "combatstate",
                "combatstate [player]",
                "Report implemented M5 combat capabilities and canonical ammunition state.",
                combat_state,
            ),
            ConsoleCommand::new(
                "vatsstate",
                "vatsstate [player]",
                "Report typed V.A.T.S. availability and state.",
                vats_state,
            ),
            ConsoleCommand::new(
                "hitboxdebug",
                "hitboxdebug state|on|off [reference-form-id]",
                "Report hitbox-debug availability; mutation becomes available in M5 wave 6.",
                hitbox_debug,
            ),
            ConsoleCommand::new(
                "weaponreload",
                "weaponreload",
                "Request the normal player-weapon reload action; firing remains blocked until it completes.",
                weapon_reload,
            )
            .mutating(),
        ] {
            registry.register(command)?;
        }
        Ok(())
    }
}

fn player_subject(invocation: &ConsoleInvocation) -> Result<(), ConsoleError> {
    if invocation.args.is_empty()
        || (invocation.args.len() == 1 && invocation.args[0].eq_ignore_ascii_case("player"))
    {
        Ok(())
    } else {
        Err(ConsoleError::new(
            "invalid_arguments",
            format!(
                "{} accepts only an optional player subject",
                invocation.command
            ),
        ))
    }
}

fn unavailable(command: &str, planned_wave: u32) -> ConsoleCommandResult {
    ConsoleCommandResult::value(json!({
        "schema": "bevyout.m5.inspect",
        "schema_version": COMBAT_INSPECTION_SCHEMA_VERSION,
        "command": command,
        "wave": 2,
        "available": false,
        "reason": format!("planned_wave_{planned_wave}"),
        "policy_revision": "m5-combat-v2",
        "diagnostics": [],
    }))
}

fn ammo_state(
    world: &mut World,
    invocation: &ConsoleInvocation,
) -> Result<ConsoleCommandResult, ConsoleError> {
    player_subject(invocation)?;
    let canonical = world
        .get_resource::<crate::viewer::interaction::CanonicalItemLedger>()
        .ok_or_else(|| ConsoleError::new("ammo_unavailable", "canonical item ledger is absent"))?;
    let holder = canonical.ledger.holders().get(&HolderId::Player);
    let weapon_id = canonical
        .ledger
        .bindings()
        .get(&HolderId::Player)
        .and_then(|binding| binding.equipped);
    let weapon = weapon_id.and_then(|id| holder.and_then(|state| state.find(id)));
    let magazine = weapon.map(|item| item.state.combat.magazine);
    let reserve = magazine
        .and_then(|magazine| magazine.ammo_form_id)
        .map(|ammo_form_id| {
            holder.map_or(0, |state| {
                state
                    .items
                    .iter()
                    .filter(|item| item.base_form_id == ammo_form_id)
                    .map(|item| item.count)
                    .sum::<u32>()
            })
        })
        .unwrap_or_default();
    Ok(ConsoleCommandResult::value(json!({
        "schema": "bevyout.m5.inspect",
        "schema_version": COMBAT_INSPECTION_SCHEMA_VERSION,
        "command": "ammostate",
        "wave": 2,
        "available": true,
        "subject": {"reference_form_id": "player"},
        "policy_revision": "m5-combat-v2",
        "state": {
            "weapon_instance_id": weapon_id.map(|id| id.0),
            "weapon_form_id": weapon.map(|item| format!("{:08x}", item.base_form_id)),
            "ammo_form_id": magazine.and_then(|state| state.ammo_form_id).map(|id| format!("{id:08x}")),
            "loaded": magazine.map_or(0, |state| state.loaded),
            "reserve": reserve,
        },
        "diagnostics": [],
    })))
}

fn combat_state(
    world: &mut World,
    invocation: &ConsoleInvocation,
) -> Result<ConsoleCommandResult, ConsoleError> {
    player_subject(invocation)?;
    let ammo = ammo_state(world, invocation)?.value;
    Ok(ConsoleCommandResult::value(json!({
        "schema": "bevyout.m5.inspect",
        "schema_version": COMBAT_INSPECTION_SCHEMA_VERSION,
        "command": "combatstate",
        "wave": 2,
        "available": true,
        "subject": {"reference_form_id": "player"},
        "capabilities": {
            "ammo": true,
            "condition": false,
            "ballistics": false,
            "armor": false,
            "limbs": false,
            "vats": false,
            "ai": false,
        },
        "policy_revision": "m5-combat-v2",
        "state": {"ammo": ammo["state"].clone()},
        "diagnostics": [],
    })))
}

fn vats_state(
    _world: &mut World,
    invocation: &ConsoleInvocation,
) -> Result<ConsoleCommandResult, ConsoleError> {
    player_subject(invocation)?;
    Ok(unavailable("vatsstate", 7))
}

fn hitbox_debug(
    _world: &mut World,
    _invocation: &ConsoleInvocation,
) -> Result<ConsoleCommandResult, ConsoleError> {
    Ok(unavailable("hitboxdebug", 6))
}

fn weapon_state(
    world: &mut World,
    invocation: &ConsoleInvocation,
) -> Result<ConsoleCommandResult, ConsoleError> {
    no_args(invocation)?;
    let runtime = world.get_resource::<PlayerWeaponRuntime>().ok_or_else(|| {
        ConsoleError::new("weapon_unavailable", "weapon runtime is not installed")
    })?;
    let equipped_form_id = runtime.equipped.as_ref().map(|weapon| weapon.base_form_id);
    let action = action_label(runtime.action());
    let summary = format!(
        "weaponstate equipped={} action={action} shots={} viewmodel={} last={}",
        equipped_form_id.map_or_else(|| "none".into(), |form_id| format!("{form_id:08x}")),
        runtime.shots_fired(),
        runtime
            .equipped
            .as_ref()
            .and_then(|weapon| weapon.viewmodel_asset_path.as_deref())
            .unwrap_or("none"),
        runtime.last_fire.status.label()
    );
    Ok(ConsoleCommandResult::new(
        json!({
            "equipped_form_id": equipped_form_id,
            "label": runtime.equipped.as_ref().map(|weapon| weapon.label.as_str()),
            "action": action,
            "shots_fired": runtime.shots_fired(),
            "viewmodel_asset_path": runtime.equipped.as_ref().and_then(|weapon| weapon.viewmodel_asset_path.as_deref()),
            "fire_sound_3d_form_id": runtime.equipped.as_ref().and_then(|weapon| weapon.fire_sound_3d_form_id),
            "fire_sound_2d_form_id": runtime.equipped.as_ref().and_then(|weapon| weapon.fire_sound_2d_form_id),
            "reload_sound_form_id": runtime.last_reload_sound_form_id,
            "muzzle_flash_active": runtime.muzzle_flash_remaining > 0.0,
            "last_reload": runtime.last_reload.map(reload_label),
            "last_fire": {
                "status": runtime.last_fire.status.label(),
                "shot_index": runtime.last_fire.shot_index,
                "target_reference_form_id": runtime.last_fire.target_reference_form_id,
                "hit_distance": runtime.last_fire.hit_distance,
                "applied_damage": runtime.last_fire.applied_damage,
                "remaining_health": runtime.last_fire.remaining_health,
                "audio_form_id": runtime.last_fire_sound_form_id,
                "muzzle_flash_seconds": runtime.last_muzzle_flash_seconds,
            },
            "ammo_accounting": true,
        }),
        vec![summary],
    ))
}

fn weapon_fire(
    world: &mut World,
    invocation: &ConsoleInvocation,
) -> Result<ConsoleCommandResult, ConsoleError> {
    no_args(invocation)?;
    world.write_message(FireWeaponRequested);
    Ok(ConsoleCommandResult::new(
        json!({"queued": true}),
        vec!["weaponfire queued".into()],
    ))
}

fn weapon_reload(
    world: &mut World,
    invocation: &ConsoleInvocation,
) -> Result<ConsoleCommandResult, ConsoleError> {
    no_args(invocation)?;
    world.write_message(ReloadWeaponRequested);
    Ok(ConsoleCommandResult::new(
        json!({"queued": true}),
        vec!["weaponreload queued".into()],
    ))
}

const fn action_label(action: WeaponAction) -> &'static str {
    match action {
        WeaponAction::Idle => "idle",
        WeaponAction::Firing => "firing",
        WeaponAction::Reloading => "reloading",
    }
}

const fn reload_label(decision: ReloadDecision) -> &'static str {
    match decision {
        ReloadDecision::Started => "started",
        ReloadDecision::BlockedFiring => "blocked_firing",
        ReloadDecision::AlreadyReloading => "already_reloading",
    }
}
