//! Player RPG stats console commands (M9 wave 1, #310).
//!
//! All five commands are player-scoped like `additem` (issue #84): the
//! Bethesda `player.` prefix is accepted via `.reference_callable(false)`
//! but the target is always the persistent player progression resource. NPC
//! actor values keep their existing surface (`actorstate`/`setactorvalue`)
//! because NPCs resolve through persisted save state, not this sheet.

use bevyout_core::actor_state::ActorValue;
use bevyout_core::stats as core_stats;

use super::stats::{PlayerProgression, StatsSettings};
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
                "Set a player SPECIAL attribute or effective skill value (clamped to range).",
                set_actor_value,
            )
            .reference_callable(false)
            .mutating(),
            ConsoleCommand::new(
                "modav",
                "[player.]modav <value> <delta>",
                "Shift a player SPECIAL attribute or effective skill value (clamped to range).",
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
fn read_actor_value(world: &World, value: ActorValue) -> Result<f64, ConsoleError> {
    let progression = player_progression(world)?;
    let settings = world.resource::<StatsSettings>().0;
    let resolved = match value {
        ActorValue::Health => progression.stats.derived(&settings).max_health,
        ActorValue::Special(attribute) => f32::from(progression.stats.effective_special(attribute)),
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
    let mut progression = player_progression_mut(world)?;
    let applied = match value {
        ActorValue::Special(attribute) => {
            i16::from(progression.stats.mod_special(attribute, delta))
        }
        ActorValue::Skill(skill) => i16::from(progression.stats.mod_skill_value(skill, delta)),
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
/// into the `Experience` projection (the recalc system owns the other
/// fields and never touches these two).
fn apply_award(
    world: &mut World,
    amount: u32,
    command: &str,
) -> Result<ConsoleCommandResult, ConsoleError> {
    let settings = world.resource::<StatsSettings>().0;
    let mut progression = player_progression_mut(world)?;
    let outcome = core_stats::award_xp(&mut progression.stats, amount, &settings);
    progression.unspent_skill_points = progression
        .unspent_skill_points
        .saturating_add(outcome.skill_points_gained);
    progression.total_skill_points = progression
        .total_skill_points
        .saturating_add(outcome.skill_points_gained);
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
