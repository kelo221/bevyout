//! Player RPG stats console commands (M9 wave 1, #310).
//!
//! All five commands are player-scoped like `additem` (issue #84): the
//! Bethesda `player.` prefix is accepted via `.reference_callable(false)`
//! but the target is always the persistent player progression resource. NPC
//! actor values keep their existing surface (`actorstate`/`setactorvalue`)
//! because NPCs resolve through persisted save state, not this sheet.

use bevyout_core::actor_state::ActorValue;
use bevyout_core::effects::{ActiveEffectsLedger, projected_derived, projected_special};
use bevyout_core::stats as core_stats;

use super::super::effects::{ActiveEffectsList, PlayerRadiation, PlayerVitals};
use super::stats::{PlayerProgression, StatsSettings};
use super::*;

pub(super) struct StatsCommandProvider;

impl ConsoleCommandProvider for StatsCommandProvider {
    fn register_commands(&self, registry: &mut ConsoleRegistry) -> Result<(), ConsoleError> {
        for command in [
            ConsoleCommand::new(
                "getav",
                "[player.]getav <value>",
                "Read a player actor value: a SPECIAL attribute, a skill, current health, or projected AP.",
                get_actor_value,
            )
            .reference_callable(false),
            ConsoleCommand::new(
                "setav",
                "[player.]setav <value> <n>",
                "Set a player SPECIAL attribute, effective skill, or current health (clamped to range).",
                set_actor_value,
            )
            .reference_callable(false)
            .mutating(),
            ConsoleCommand::new(
                "modav",
                "[player.]modav <value> <delta>",
                "Shift a player SPECIAL attribute, effective skill, or current health (clamped to range).",
                mod_actor_value,
            )
            .reference_callable(false)
            .mutating(),
            ConsoleCommand::new(
                "advlevel",
                "[player.]advlevel",
                "Advance the player one level, granting skill points.",
                advance_level,
            )
            .reference_callable(false)
            .mutating(),
            ConsoleCommand::new(
                "rewardxp",
                "[player.]rewardxp <xp>",
                "Award the player experience points.",
                reward_xp,
            )
            .reference_callable(false)
            .mutating(),
        ] {
            registry.register(command)?;
        }
        Ok(())
    }
}

fn player_progression(world: &World) -> Result<&PlayerProgression, ConsoleError> {
    world
        .get_resource::<PlayerProgression>()
        .ok_or_else(|| ConsoleError::new("player_unavailable", "player progression is unavailable"))
}

fn player_progression_mut(world: &mut World) -> Result<Mut<'_, PlayerProgression>, ConsoleError> {
    world
        .get_resource_mut::<PlayerProgression>()
        .ok_or_else(|| ConsoleError::new("player_unavailable", "player progression is unavailable"))
}

fn fps_player(world: &World) -> Option<Entity> {
    world
        .resource::<player::CameraModeState>()
        .player
        .filter(|&entity| world.entities().contains(entity))
}

fn live_or_stored_effects(world: &World) -> (ActiveEffectsLedger, u16) {
    let stored = world.get_resource::<PlayerProgression>();
    if let Some(entity) = fps_player(world) {
        let ledger = world
            .get::<ActiveEffectsList>(entity)
            .map(|effects| effects.ledger.clone())
            .or_else(|| stored.map(|progression| progression.effects.clone()))
            .unwrap_or_default();
        let rads = world
            .get::<PlayerRadiation>(entity)
            .map(|radiation| radiation.0.rads)
            .or_else(|| stored.map(|progression| progression.radiation.rads))
            .unwrap_or(0);
        return (ledger, rads);
    }
    (
        stored
            .map(|progression| progression.effects.clone())
            .unwrap_or_default(),
        stored
            .map(|progression| progression.radiation.rads)
            .unwrap_or(0),
    )
}

fn parse_actor_value(raw: &str) -> Result<ActorValue, ConsoleError> {
    ActorValue::parse(raw).ok_or_else(|| {
        ConsoleError::new(
            "unknown_actor_value",
            format!("unknown actor value {raw:?}; expected e.g. health, strength, or small_guns"),
        )
    })
}

fn projected_max_health(world: &World) -> Result<f32, ConsoleError> {
    let progression = player_progression(world)?;
    let settings = world.resource::<StatsSettings>().0;
    let (ledger, rads) = live_or_stored_effects(world);
    Ok(projected_derived(&progression.stats, &ledger, rads, &settings).max_health)
}

fn current_health(world: &World) -> Result<f32, ConsoleError> {
    let max_health = projected_max_health(world)?;
    if let Some(entity) = fps_player(world)
        && let Some(vitals) = world.get::<PlayerVitals>(entity)
    {
        return Ok(vitals.current_health);
    }
    Ok(player_progression(world)?
        .current_health
        .unwrap_or(max_health))
}

fn set_current_health(world: &mut World, applied: f32) -> Result<(), ConsoleError> {
    if let Some(entity) = fps_player(world)
        && let Some(mut vitals) = world.get_mut::<PlayerVitals>(entity)
    {
        vitals.current_health = applied;
    }
    player_progression_mut(world)?.current_health = Some(applied);
    Ok(())
}

/// Reads an actor value from the sheet. Derived values are computed
/// synchronously from the sheet plus active settings and live effects (not the
/// possibly one-frame-stale `DerivedAttributes` component) so console batches
/// read exactly what earlier commands wrote. Health is current vitals, not max.
fn read_actor_value(world: &World, value: ActorValue) -> Result<f64, ConsoleError> {
    let progression = player_progression(world)?;
    let settings = world.resource::<StatsSettings>().0;
    let (ledger, rads) = live_or_stored_effects(world);
    let derived = projected_derived(&progression.stats, &ledger, rads, &settings);
    let resolved = match value {
        ActorValue::Health => current_health(world)?,
        ActorValue::ActionPoints => derived.max_action_points,
        ActorValue::RadResist => ledger.modifier_for(ActorValue::RadResist).max(0.0),
        ActorValue::Special(attribute) => {
            f32::from(projected_special(&progression.stats, &ledger, rads)[&attribute])
        }
        ActorValue::Skill(skill) => f32::from(progression.stats.skill_value(skill)),
        other => {
            return Err(ConsoleError::new(
                "unsupported_actor_value",
                format!("{} is not a player stat", other.label()),
            ));
        }
    };
    Ok(f64::from(resolved))
}

pub(super) fn get_actor_value(
    world: &mut World,
    invocation: &ConsoleInvocation,
) -> Result<ConsoleCommandResult, ConsoleError> {
    if invocation.args.len() != 1 {
        return Err(ConsoleError::new(
            "bad_arity",
            "getav expects exactly one actor value name",
        ));
    }
    let value = parse_actor_value(&invocation.args[0])?;
    let resolved = read_actor_value(world, value)?;
    Ok(ConsoleCommandResult::new(
        json!({ "value": value.label(), "result": resolved }),
        vec![format!("{} = {}", value.label(), resolved)],
    ))
}

fn parse_amount(raw: &str, command: &str) -> Result<i16, ConsoleError> {
    raw.parse::<i16>().map_err(|_| {
        ConsoleError::new(
            "bad_type",
            format!("{command} expects a whole number between -32768 and 32767"),
        )
    })
}

pub(super) fn set_actor_value(
    world: &mut World,
    invocation: &ConsoleInvocation,
) -> Result<ConsoleCommandResult, ConsoleError> {
    if invocation.args.len() != 2 {
        return Err(ConsoleError::new(
            "bad_arity",
            "setav expects an actor value name and a number",
        ));
    }
    let value = parse_actor_value(&invocation.args[0])?;
    let amount = parse_amount(&invocation.args[1], "setav")?;
    if value == ActorValue::Health {
        let max_health = projected_max_health(world)?;
        let applied = f32::from(amount).clamp(0.0, max_health);
        set_current_health(world, applied)?;
        return Ok(ConsoleCommandResult::new(
            json!({ "value": value.label(), "result": applied }),
            vec![format!("{} set to {}", value.label(), applied)],
        ));
    }
    let mut progression = player_progression_mut(world)?;
    let applied = match value {
        ActorValue::Special(attribute) => {
            let clamped = amount.clamp(
                i16::from(core_stats::SPECIAL_MIN),
                i16::from(core_stats::SPECIAL_MAX),
            );
            i16::from(progression.stats.set_special(attribute, clamped as u8))
        }
        ActorValue::Skill(skill) => {
            let target = amount.clamp(
                i16::from(core_stats::SKILL_MIN),
                i16::from(core_stats::SKILL_MAX),
            );
            i16::from(progression.stats.set_skill_value(skill, target))
        }
        ActorValue::Health => unreachable!("health handled before borrowing PlayerProgression"),
        other => {
            return Err(ConsoleError::new(
                "unsupported_actor_value",
                format!("{} is not a settable player stat", other.label()),
            ));
        }
    };
    Ok(ConsoleCommandResult::new(
        json!({ "value": value.label(), "result": applied }),
        vec![format!("{} set to {}", value.label(), applied)],
    ))
}

pub(super) fn mod_actor_value(
    world: &mut World,
    invocation: &ConsoleInvocation,
) -> Result<ConsoleCommandResult, ConsoleError> {
    if invocation.args.len() != 2 {
        return Err(ConsoleError::new(
            "bad_arity",
            "modav expects an actor value name and a delta",
        ));
    }
    let value = parse_actor_value(&invocation.args[0])?;
    let delta = parse_amount(&invocation.args[1], "modav")?;
    if value == ActorValue::Health {
        let max_health = projected_max_health(world)?;
        let applied = (current_health(world)? + f32::from(delta)).clamp(0.0, max_health);
        set_current_health(world, applied)?;
        return Ok(ConsoleCommandResult::new(
            json!({ "value": value.label(), "result": applied }),
            vec![format!("{} now {}", value.label(), applied)],
        ));
    }
    let mut progression = player_progression_mut(world)?;
    let applied = match value {
        ActorValue::Special(attribute) => {
            i16::from(progression.stats.mod_special(attribute, delta))
        }
        ActorValue::Skill(skill) => i16::from(progression.stats.mod_skill_value(skill, delta)),
        ActorValue::Health => unreachable!("health handled before borrowing PlayerProgression"),
        other => {
            return Err(ConsoleError::new(
                "unsupported_actor_value",
                format!("{} is not a modifiable player stat", other.label()),
            ));
        }
    };
    Ok(ConsoleCommandResult::new(
        json!({ "value": value.label(), "result": applied }),
        vec![format!("{} now {}", value.label(), applied)],
    ))
}

pub(super) fn advance_level(
    world: &mut World,
    invocation: &ConsoleInvocation,
) -> Result<ConsoleCommandResult, ConsoleError> {
    if !invocation.args.is_empty() {
        return Err(ConsoleError::new(
            "bad_arity",
            "advlevel takes no arguments",
        ));
    }
    let progression = player_progression(world)?;
    let settings = world.resource::<StatsSettings>().0;
    if progression.stats.level >= settings.max_player_level {
        return Err(ConsoleError::new(
            "at_level_cap",
            format!("player is at the level cap {}", settings.max_player_level),
        ));
    }
    let amount = core_stats::xp_threshold(progression.stats.level.saturating_add(1), &settings)
        .saturating_sub(progression.stats.xp);
    apply_award(world, amount, "advlevel")
}

pub(super) fn reward_xp(
    world: &mut World,
    invocation: &ConsoleInvocation,
) -> Result<ConsoleCommandResult, ConsoleError> {
    if invocation.args.len() != 1 {
        return Err(ConsoleError::new(
            "bad_arity",
            "rewardxp expects exactly one XP amount",
        ));
    }
    let amount = invocation.args[0].parse::<u32>().map_err(|_| {
        ConsoleError::new("bad_type", "rewardxp expects a non-negative whole number")
    })?;
    apply_award(world, amount, "rewardxp")
}

/// Applies the award through the kernel and folds the granted skill points
/// into the persistent progression resource. M9 wave 2 (#314): the player's
/// active perk modifiers scale the awarded XP (Swift Learner) and add
/// bonus skill points per level gained (Educated).
fn apply_award(
    world: &mut World,
    amount: u32,
    command: &str,
) -> Result<ConsoleCommandResult, ConsoleError> {
    let settings = world.resource::<StatsSettings>().0;
    let modifiers = player_perk_modifiers(world);
    let mut progression = player_progression_mut(world)?;
    let outcome = core_stats::award_xp(
        &mut progression.stats,
        amount,
        modifiers.xp_award_multiplier_bps,
        &settings,
    );
    // The kernel's per-level points exclude the perk bonus by design
    // (#313 keeps exactly one parameter per kernel), so the adapter adds
    // `bonus_skill_points` for every level the award crossed.
    let bonus_points =
        u16::from(outcome.levels_gained).saturating_mul(modifiers.bonus_skill_points);
    let skill_points_gained = outcome.skill_points_gained.saturating_add(bonus_points);
    progression.unspent_skill_points = progression
        .unspent_skill_points
        .saturating_add(skill_points_gained);
    progression.total_skill_points = progression
        .total_skill_points
        .saturating_add(skill_points_gained);
    Ok(ConsoleCommandResult::new(
        json!({
            "command": command,
            "xp": outcome.xp,
            "level": outcome.level,
            "levels_gained": outcome.levels_gained,
            "skill_points_gained": skill_points_gained,
            "xp_multiplier_bps": modifiers.xp_award_multiplier_bps,
            "bonus_skill_points_per_level": modifiers.bonus_skill_points,
        }),
        vec![format!(
            "{command}: level {} ({} XP, +{} skill points)",
            outcome.level, outcome.xp, skill_points_gained
        )],
    ))
}

/// Projects the player's owned perks onto the active leveling modifiers;
/// an empty catalog stays neutral.
pub(super) fn player_perk_modifiers(world: &World) -> bevyout_core::perks::PerkModifiers {
    let catalog = world
        .get_resource::<super::stats::PerkCatalog>()
        .map(|catalog| catalog.0.clone())
        .unwrap_or_default();
    let Some(progression) = world.get_resource::<PlayerProgression>() else {
        return bevyout_core::perks::PerkModifiers::default();
    };
    bevyout_core::perks::active_perk_modifiers(&progression.perks, &catalog)
}
