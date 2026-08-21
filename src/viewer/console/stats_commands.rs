//! Player RPG stats console commands (M9 wave 1, #310).
//!
//! All five commands are player-scoped like `additem` (issue #84): the
//! Bethesda `player.` prefix is accepted via `.reference_callable(false)`
//! but the target is always the FPS player's `ActorStats`. NPC actor values
//! keep their existing surface (`actorstate`/`setactorvalue`) because NPCs
//! resolve through persisted save state, not this sheet.

use bevyout_core::actor_state::ActorValue;
use bevyout_core::stats as core_stats;

use super::stats::{ActorStats, Experience, StatsSettings};
use super::*;

pub(super) struct StatsCommandProvider;

impl ConsoleCommandProvider for StatsCommandProvider {
    fn register_commands(&self, registry: &mut ConsoleRegistry) -> Result<(), ConsoleError> {
        for command in [
            ConsoleCommand::new(
                "getav",
                "[player.]getav <value>",
                "Read a player actor value: a SPECIAL attribute, a skill, or health (max).",
                get_actor_value,
            )
            .reference_callable(false),
            ConsoleCommand::new(
                "setav",
                "[player.]setav <value> <n>",
                "Set a base player SPECIAL attribute or skill value (clamped to range).",
                set_actor_value,
            )
            .reference_callable(false)
            .mutating(),
            ConsoleCommand::new(
                "modav",
                "[player.]modav <value> <delta>",
                "Shift a base player SPECIAL attribute or skill value (clamped to range).",
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

/// Resolves the stats-carrying player entity: the resolved reference when it
/// carries `ActorStats`, otherwise the single spawned FPS player.
fn player_stats_entity(
    world: &mut World,
    invocation: &ConsoleInvocation,
) -> Result<Entity, ConsoleError> {
    if let Some(entity) = invocation.target
        && world.get::<ActorStats>(entity).is_some()
    {
        return Ok(entity);
    }
    let mut query = world.query_filtered::<Entity, With<player::FpsPlayer>>();
    let entity = query
        .single(world)
        .map_err(|_| ConsoleError::new("player_unavailable", "the FPS player does not exist"))?;
    if world.get::<ActorStats>(entity).is_none() {
        return Err(ConsoleError::new(
            "player_unavailable",
            "the FPS player has no stats sheet",
        ));
    }
    Ok(entity)
}

fn parse_actor_value(raw: &str) -> Result<ActorValue, ConsoleError> {
    ActorValue::parse(raw).ok_or_else(|| {
        ConsoleError::new(
            "unknown_actor_value",
            format!("unknown actor value {raw:?}; expected e.g. health, strength, or small_guns"),
        )
    })
}

/// Reads an actor value from the sheet. Derived values are computed
/// synchronously from the sheet plus active settings (not the possibly
/// one-frame-stale `DerivedAttributes` component) so console batches read
/// exactly what earlier commands wrote.
fn read_actor_value(world: &World, entity: Entity, value: ActorValue) -> Result<f64, ConsoleError> {
    let stats = world.get::<ActorStats>(entity).ok_or_else(|| {
        ConsoleError::new("player_unavailable", "the FPS player has no stats sheet")
    })?;
    let settings = world.resource::<StatsSettings>().0;
    let resolved = match value {
        ActorValue::Health => stats.0.derived(&settings).max_health,
        ActorValue::Special(attribute) => f32::from(stats.0.effective_special(attribute)),
        ActorValue::Skill(skill) => f32::from(stats.0.skill_value(skill)),
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
    let entity = player_stats_entity(world, invocation)?;
    let resolved = read_actor_value(world, entity, value)?;
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
    let entity = player_stats_entity(world, invocation)?;
    let mut entity_mut = world
        .get_entity_mut(entity)
        .map_err(|_| ConsoleError::new("player_unavailable", "the FPS player does not exist"))?;
    let mut stats = entity_mut
        .get_mut::<ActorStats>()
        .expect("player_stats_entity verified the sheet");
    let applied = match value {
        ActorValue::Special(attribute) => {
            let clamped = amount.clamp(
                i16::from(core_stats::SPECIAL_MIN),
                i16::from(core_stats::SPECIAL_MAX),
            );
            i16::from(stats.0.set_special(attribute, clamped as u8))
        }
        ActorValue::Skill(skill) => {
            let current = i16::from(stats.0.skill_value(skill));
            let target = amount.clamp(
                i16::from(core_stats::SKILL_MIN),
                i16::from(core_stats::SKILL_MAX),
            );
            // add_skill_points floors the stored increase at zero, so a
            // target below the current base is unreachable by design;
            // report the effective value either way.
            stats.0.add_skill_points(skill, target - current);
            i16::from(stats.0.skill_value(skill))
        }
        ActorValue::Health => {
            return Err(ConsoleError::new(
                "unsupported_actor_value",
                "health is derived; change endurance or level instead",
            ));
        }
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
    let entity = player_stats_entity(world, invocation)?;
    let mut entity_mut = world
        .get_entity_mut(entity)
        .map_err(|_| ConsoleError::new("player_unavailable", "the FPS player does not exist"))?;
    let mut stats = entity_mut
        .get_mut::<ActorStats>()
        .expect("player_stats_entity verified the sheet");
    let applied = match value {
        ActorValue::Special(attribute) => i16::from(stats.0.mod_special(attribute, delta)),
        ActorValue::Skill(skill) => {
            stats.0.add_skill_points(skill, delta);
            i16::from(stats.0.skill_value(skill))
        }
        ActorValue::Health => {
            return Err(ConsoleError::new(
                "unsupported_actor_value",
                "health is derived; change endurance or level instead",
            ));
        }
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
    let entity = player_stats_entity(world, invocation)?;
    let settings = world.resource::<StatsSettings>().0;
    let amount = {
        let stats = world
            .get::<ActorStats>(entity)
            .expect("player_stats_entity verified the sheet");
        if stats.0.level >= settings.max_player_level {
            return Err(ConsoleError::new(
                "at_level_cap",
                format!("player is at the level cap {}", settings.max_player_level),
            ));
        }
        core_stats::xp_threshold(stats.0.level.saturating_add(1), &settings)
            .saturating_sub(stats.0.xp)
    };
    apply_award(world, entity, amount, "advlevel")
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
    let entity = player_stats_entity(world, invocation)?;
    apply_award(world, entity, amount, "rewardxp")
}

/// Applies the award through the kernel and folds the granted skill points
/// into the `Experience` projection (the recalc system owns the other
/// fields and never touches these two).
fn apply_award(
    world: &mut World,
    entity: Entity,
    amount: u32,
    command: &str,
) -> Result<ConsoleCommandResult, ConsoleError> {
    let settings = world.resource::<StatsSettings>().0;
    let outcome = {
        let mut entity_mut = world.get_entity_mut(entity).map_err(|_| {
            ConsoleError::new("player_unavailable", "the FPS player does not exist")
        })?;
        let mut stats = entity_mut
            .get_mut::<ActorStats>()
            .expect("player_stats_entity verified the sheet");
        core_stats::award_xp(&mut stats.0, amount, &settings)
    };
    if outcome.skill_points_gained > 0
        && let Ok(mut entity) = world.get_entity_mut(entity)
        && let Some(mut experience) = entity.get_mut::<Experience>()
    {
        experience.unspent_skill_points += outcome.skill_points_gained;
        experience.total_skill_points += outcome.skill_points_gained;
    }
    Ok(ConsoleCommandResult::new(
        json!({
            "command": command,
            "xp": outcome.xp,
            "level": outcome.level,
            "levels_gained": outcome.levels_gained,
            "skill_points_gained": outcome.skill_points_gained,
        }),
        vec![format!(
            "{command}: level {} ({} XP, +{} skill points)",
            outcome.level, outcome.xp, outcome.skill_points_gained
        )],
    ))
}
