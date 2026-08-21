//! Player perk console commands (M9 wave 2, #314).
//!
//! Player-scoped like the stats commands: the Bethesda `player.` prefix is
//! accepted via `.reference_callable(false)` and the target is always the
//! FPS player's `ActorPerks`/`ActorStats`. Definitions come from the
//! prepared perk catalog resource (#312), so `addperk` enforces the pure
//! `can_take_perk` evaluator (#313) exactly as the engine would gate
//! level-up selection.

use bevyout_core::perks::{PerkBlockReason, PerkDefinition, can_take_perk};

use super::stats::{ActorPerks, ActorStats, PerkCatalog};
use super::stats_commands::{player_perk_modifiers, player_stats_entity};
use super::*;

pub(super) struct PerkCommandProvider;

impl ConsoleCommandProvider for PerkCommandProvider {
    fn register_commands(&self, registry: &mut ConsoleRegistry) -> Result<(), ConsoleError> {
        for command in [
            ConsoleCommand::new(
                "addperk",
                "[player.]addperk <FormID>",
                "Grant the player the next rank of a perk (must be eligible).",
                add_perk,
            )
            .reference_callable(false)
            .mutating(),
            ConsoleCommand::new(
                "removeperk",
                "[player.]removeperk <FormID>",
                "Remove one rank of an owned perk.",
                remove_perk,
            )
            .reference_callable(false)
            .mutating(),
            ConsoleCommand::new(
                "hasperk",
                "[player.]hasperk <FormID>",
                "Report the player's owned rank of a perk (0 = not owned).",
                has_perk,
            )
            .reference_callable(false),
            ConsoleCommand::new(
                "showperks",
                "showperks [--eligible]",
                "List owned perks; with --eligible, every playable perk and its blocked reasons.",
                show_perks,
            )
            .reference_callable(false),
        ] {
            registry.register(command)?;
        }
        Ok(())
    }
}

/// Parses a perk FormID argument, accepting 1..=8 hex digits like
/// `additem` (so `addperk 31dd3` and `addperk 00031dd3` both work).
fn parse_perk_form_id(raw: &str, command: &str) -> Result<u32, ConsoleError> {
    parse_item_form_id(raw).ok_or_else(|| {
        ConsoleError::new(
            "bad_form_id",
            format!("{command} expects a perk FormID in hex, e.g. 00031dd3"),
        )
    })
}

/// Looks up a perk definition in the loaded catalog. The definition is
/// returned owned because the caller mutates the world (grant/remove)
/// after the lookup and still needs the editor id for its result.
fn perk_definition(world: &World, form_id: u32) -> Result<PerkDefinition, ConsoleError> {
    world
        .resource::<PerkCatalog>()
        .0
        .get(&form_id)
        .cloned()
        .ok_or_else(|| {
            ConsoleError::new(
                "unknown_perk",
                format!(
                    "no perk {form_id:08x} in the loaded perk catalog; run `prepare` and relaunch",
                ),
            )
        })
}

fn modifier_json(modifiers: bevyout_core::perks::PerkModifiers) -> Value {
    json!({
        "xp_award_multiplier_bps": modifiers.xp_award_multiplier_bps,
        "bonus_skill_points": modifiers.bonus_skill_points,
    })
}

pub(super) fn add_perk(
    world: &mut World,
    invocation: &ConsoleInvocation,
) -> Result<ConsoleCommandResult, ConsoleError> {
    if invocation.args.len() != 1 {
        return Err(ConsoleError::new(
            "bad_arity",
            "addperk expects exactly one perk FormID",
        ));
    }
    let form_id = parse_perk_form_id(&invocation.args[0], "addperk")?;
    let entity = player_stats_entity(world, invocation)?;
    let def = perk_definition(world, form_id)?;
    let (sheet, mut perks) = {
        let stats = world
            .get::<ActorStats>(entity)
            .expect("player_stats_entity verified the sheet");
        let perks = world
            .get::<ActorPerks>(entity)
            .cloned()
            .expect("ActorPerks attaches with ActorStats");
        (stats.0.clone(), perks)
    };
    // Enforce eligibility exactly like engine perk selection: a blocked
    // perk reports every reason rather than being granted.
    if let bevyout_core::perks::PerkEligibility::Blocked(reasons) =
        can_take_perk(&sheet, &def, &perks.0)
    {
        return Err(ConsoleError::new(
            "perk_ineligible",
            format!(
                "{}: {}",
                def.editor_id,
                reasons
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join("; ")
            ),
        ));
    }
    let rank = perks.0.rank(form_id) + 1;
    perks.0.set_rank(form_id, rank);
    let mut entity_mut = world
        .get_entity_mut(entity)
        .map_err(|_| ConsoleError::new("player_unavailable", "the FPS player does not exist"))?;
    entity_mut
        .get_mut::<ActorPerks>()
        .expect("ActorPerks attaches with ActorStats")
        .0 = perks.0;
    let modifiers = player_perk_modifiers(world, entity);
    Ok(ConsoleCommandResult::new(
        json!({
            "perk": def.editor_id,
            "rank": rank,
            "ranks": def.ranks,
            "modifiers": modifier_json(modifiers),
        }),
        vec![format!(
            "{} rank {}/{} ({})",
            def.editor_id,
            rank,
            def.ranks,
            modifier_summary(&modifiers)
        )],
    ))
}

fn modifier_summary(modifiers: &bevyout_core::perks::PerkModifiers) -> String {
    format!(
        "xp x{}.{:02}, +{} skill points/level",
        modifiers.xp_award_multiplier_bps / 10_000,
        (modifiers.xp_award_multiplier_bps % 10_000) / 100,
        modifiers.bonus_skill_points
    )
}

pub(super) fn remove_perk(
    world: &mut World,
    invocation: &ConsoleInvocation,
) -> Result<ConsoleCommandResult, ConsoleError> {
    if invocation.args.len() != 1 {
        return Err(ConsoleError::new(
            "bad_arity",
            "removeperk expects exactly one perk FormID",
        ));
    }
    let form_id = parse_perk_form_id(&invocation.args[0], "removeperk")?;
    let entity = player_stats_entity(world, invocation)?;
    let def = perk_definition(world, form_id)?;
    let mut perks = world
        .get::<ActorPerks>(entity)
        .cloned()
        .expect("ActorPerks attaches with ActorStats");
    let owned = perks.0.rank(form_id);
    if owned == 0 {
        return Err(ConsoleError::new(
            "perk_not_owned",
            format!("the player does not own {}", def.editor_id),
        ));
    }
    let rank = owned - 1;
    perks.0.set_rank(form_id, rank);
    let mut entity_mut = world
        .get_entity_mut(entity)
        .map_err(|_| ConsoleError::new("player_unavailable", "the FPS player does not exist"))?;
    entity_mut
        .get_mut::<ActorPerks>()
        .expect("ActorPerks attaches with ActorStats")
        .0 = perks.0;
    let modifiers = player_perk_modifiers(world, entity);
    Ok(ConsoleCommandResult::new(
        json!({
            "perk": def.editor_id,
            "rank": rank,
            "ranks": def.ranks,
            "modifiers": modifier_json(modifiers),
        }),
        vec![format!(
            "{} rank {}/{} ({})",
            def.editor_id,
            rank,
            def.ranks,
            modifier_summary(&modifiers)
        )],
    ))
}

pub(super) fn has_perk(
    world: &mut World,
    invocation: &ConsoleInvocation,
) -> Result<ConsoleCommandResult, ConsoleError> {
    if invocation.args.len() != 1 {
        return Err(ConsoleError::new(
            "bad_arity",
            "hasperk expects exactly one perk FormID",
        ));
    }
    let form_id = parse_perk_form_id(&invocation.args[0], "hasperk")?;
    let entity = player_stats_entity(world, invocation)?;
    let def = perk_definition(world, form_id)?;
    let perks = world
        .get::<ActorPerks>(entity)
        .expect("ActorPerks attaches with ActorStats");
    let rank = perks.0.rank(form_id);
    Ok(ConsoleCommandResult::new(
        json!({ "perk": def.editor_id, "rank": rank, "ranks": def.ranks }),
        vec![format!("{} rank {}/{}", def.editor_id, rank, def.ranks)],
    ))
}

pub(super) fn show_perks(
    world: &mut World,
    invocation: &ConsoleInvocation,
) -> Result<ConsoleCommandResult, ConsoleError> {
    let eligible_flag = invocation.args.first().map(String::as_str) == Some("--eligible");
    if !invocation.args.is_empty() && !eligible_flag {
        return Err(ConsoleError::new(
            "bad_arity",
            "showperks takes no arguments besides --eligible",
        ));
    }
    let entity = player_stats_entity(world, invocation)?;
    let catalog = &world.resource::<PerkCatalog>().0;
    let perks = world
        .get::<ActorPerks>(entity)
        .expect("ActorPerks attaches with ActorStats");
    if eligible_flag {
        // Every playable perk with its current eligibility and blocked
        // reasons, mirroring the level-up selection list.
        let sheet = world
            .get::<ActorStats>(entity)
            .expect("player_stats_entity verified the sheet")
            .0
            .clone();
        let mut eligible = Vec::new();
        for def in catalog.values().filter(|def| def.playable) {
            let eligibility = can_take_perk(&sheet, def, &perks.0);
            eligible.push(json!({
                "form_id": format!("{:08x}", def.form_id),
                "perk": def.editor_id,
                "min_level": def.min_level,
                "ranks": def.ranks,
                "eligible": eligibility.is_eligible(),
                "reasons": eligibility
                    .reasons()
                    .iter()
                    .map(reason_json)
                    .collect::<Vec<_>>(),
            }));
        }
        let count = eligible.len();
        return Ok(ConsoleCommandResult::new(
            json!({ "eligible": eligible, "count": count }),
            vec![format!("{count} playable perks in the catalog")],
        ));
    }
    let mut owned = Vec::new();
    for (form_id, rank) in perks.0.0.iter() {
        let Some(def) = catalog.get(form_id) else {
            continue;
        };
        owned.push(json!({
            "form_id": format!("{form_id:08x}"),
            "perk": def.editor_id,
            "rank": rank,
            "ranks": def.ranks,
        }));
    }
    let count = owned.len();
    let modifiers = player_perk_modifiers(world, entity);
    Ok(ConsoleCommandResult::new(
        json!({
            "perks": owned,
            "count": count,
            "modifiers": modifier_json(modifiers),
        }),
        vec![format!(
            "{count} perks owned ({})",
            modifier_summary(&modifiers)
        )],
    ))
}

/// One blocked reason as `{ kind, ...fields }` JSON.
fn reason_json(reason: &PerkBlockReason) -> Value {
    match reason {
        PerkBlockReason::MinLevel { required, current } => json!({
            "kind": reason.kind(),
            "required": required,
            "current": current,
        }),
        PerkBlockReason::MaxRanksReached { ranks } => {
            json!({ "kind": reason.kind(), "ranks": ranks })
        }
        PerkBlockReason::ConditionNotMet {
            actor_value,
            required,
            actual,
        } => json!({
            "kind": reason.kind(),
            "value": actor_value.label(),
            "required": required,
            "actual": actual,
        }),
        PerkBlockReason::UnknownCondition => json!({ "kind": reason.kind() }),
    }
}
