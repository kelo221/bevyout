//! Stealth, crime, ownership, and Karma inspection (M9 wave 6).

use bevyout_core::detection::DetectionHud;
use serde_json::json;

use super::super::hud::HudDetection;
use super::super::perception::ActorAwareness;
use super::stats::PlayerProgression;
use super::*;

pub(super) struct CrimeCommandProvider;

impl ConsoleCommandProvider for CrimeCommandProvider {
    fn register_commands(&self, registry: &mut ConsoleRegistry) -> Result<(), ConsoleError> {
        for command in [
            ConsoleCommand::new(
                "detectstate",
                "detectstate",
                "Report live observer awareness and the Hidden/Caution/Danger HUD projection.",
                detectstate,
            ),
            ConsoleCommand::new(
                "crime",
                "crime",
                "Report player bounty, Karma, next crime sequence, and reported CrimeIds.",
                crime,
            ),
            ConsoleCommand::new(
                "setownership",
                "setownership <reference> <owner-formid|none> [rank]",
                "Set a placement's XOWN owner and optional XRNK required faction rank.",
                setownership,
            )
            .mutating(),
            ConsoleCommand::new(
                "getkarma",
                "[player.]getkarma",
                "Read player Karma from the crime ledger.",
                getkarma,
            )
            .reference_callable(false),
            ConsoleCommand::new(
                "modkarma",
                "[player.]modkarma <delta>",
                "Add a signed integer to player Karma.",
                modkarma,
            )
            .reference_callable(false)
            .mutating(),
        ] {
            registry.register(command)?;
        }
        Ok(())
    }
}

fn detectstate(
    world: &mut World,
    invocation: &ConsoleInvocation,
) -> Result<ConsoleCommandResult, ConsoleError> {
    no_args(invocation)?;
    let hud = world
        .get_resource::<HudDetection>()
        .map(|hud| hud.0)
        .unwrap_or(DetectionHud::Hidden);
    let mut observers = Vec::new();
    let mut query = world.query::<(&actor::ActorRuntime, &ActorAwareness)>();
    for (runtime, awareness) in query.iter(world) {
        observers.push(json!({
            "reference_form_id": runtime.reference_form_id,
            "confidence_milli": awareness.state.confidence_milli,
            "acquired": awareness.state.target().map(|target| json!({
                "class": target.class.label(),
                "form_id": target.form_id,
            })),
            "time_since_seen_ms": awareness.state.time_since_seen_ms,
            "distance": awareness.last_player.map(|inputs| inputs.distance),
            "line_of_sight": awareness.last_player.map(|inputs| inputs.has_line_of_sight),
        }));
    }
    Ok(ConsoleCommandResult::new(
        json!({
            "hud": hud.label(),
            "observers": observers,
        }),
        vec![format!(
            "detectstate hud={} observers={}",
            hud.label(),
            observers.len()
        )],
    ))
}

fn crime(
    world: &mut World,
    invocation: &ConsoleInvocation,
) -> Result<ConsoleCommandResult, ConsoleError> {
    no_args(invocation)?;
    let progression = world.get_resource::<PlayerProgression>().ok_or_else(|| {
        ConsoleError::new("player_unavailable", "player progression is unavailable")
    })?;
    let reported: Vec<_> = progression
        .crime
        .reported
        .iter()
        .map(|id| {
            json!({
                "class": id.actor.class.label(),
                "form_id": id.actor.form_id,
                "sequence": id.sequence,
            })
        })
        .collect();
    Ok(ConsoleCommandResult::new(
        json!({
            "bounty": progression.crime.bounty,
            "karma": progression.crime.karma,
            "next_sequence": progression.crime.next_sequence,
            "reported": reported,
        }),
        vec![format!(
            "crime bounty={} karma={} sequence={}",
            progression.crime.bounty, progression.crime.karma, progression.crime.next_sequence
        )],
    ))
}

fn setownership(
    world: &mut World,
    invocation: &ConsoleInvocation,
) -> Result<ConsoleCommandResult, ConsoleError> {
    let (selector, owner_raw, rank_raw) = match invocation.args.as_slice() {
        [selector, owner] => (selector.as_str(), owner.as_str(), None),
        [selector, owner, rank] => (selector.as_str(), owner.as_str(), Some(rank.as_str())),
        _ => {
            return Err(ConsoleError::new(
                "bad_arity",
                "setownership requires a reference, owner FormID or none, and an optional rank",
            ));
        }
    };
    let entity = resolve_reference(world, selector)?;
    let owner_form_id =
        if owner_raw.eq_ignore_ascii_case("none") {
            None
        } else {
            Some(parse_item_form_id(owner_raw).ok_or_else(|| {
                ConsoleError::new("bad_type", "owner must be none or 1-8 hex digits")
            })?)
        };
    let owner_faction_rank = rank_raw
        .map(|raw| {
            raw.parse::<i32>()
                .map_err(|_| ConsoleError::new("bad_type", "rank must be a signed integer"))
        })
        .transpose()?;
    let form_id = {
        let mut root = world
            .get_mut::<interaction::PlacementRoot>(entity)
            .ok_or_else(|| {
                ConsoleError::new("not_placement", "reference is not a prepared placement")
            })?;
        root.set_ownership(owner_form_id, owner_faction_rank);
        root.placement().reference_form_id
    };
    if let Some(mut active) = world.get_resource_mut::<interaction::ActiveContainerTarget>()
        && let Some(container) = active.0.as_mut()
        && container.matches_entity(entity)
    {
        container.set_ownership(owner_form_id, owner_faction_rank);
    }
    Ok(ConsoleCommandResult::new(
        json!({
            "reference_form_id": form_id,
            "owner_form_id": owner_form_id,
            "owner_faction_rank": owner_faction_rank,
        }),
        vec![format!(
            "setownership {form_id:08x} owner={}",
            owner_form_id.map_or_else(|| "none".to_string(), |id| format!("{id:08x}"))
        )],
    ))
}

fn getkarma(
    world: &mut World,
    invocation: &ConsoleInvocation,
) -> Result<ConsoleCommandResult, ConsoleError> {
    no_args(invocation)?;
    let karma = world
        .get_resource::<PlayerProgression>()
        .ok_or_else(|| {
            ConsoleError::new("player_unavailable", "player progression is unavailable")
        })?
        .crime
        .karma;
    Ok(ConsoleCommandResult::new(
        json!({ "karma": karma }),
        vec![format!("karma {karma}")],
    ))
}

fn modkarma(
    world: &mut World,
    invocation: &ConsoleInvocation,
) -> Result<ConsoleCommandResult, ConsoleError> {
    let [raw] = invocation.args.as_slice() else {
        return Err(ConsoleError::new(
            "bad_arity",
            "modkarma requires a signed integer",
        ));
    };
    let delta = raw
        .parse::<i32>()
        .map_err(|_| ConsoleError::new("bad_type", "modkarma delta must be a signed integer"))?;
    let mut progression = world
        .get_resource_mut::<PlayerProgression>()
        .ok_or_else(|| {
            ConsoleError::new("player_unavailable", "player progression is unavailable")
        })?;
    progression.crime.karma = progression.crime.karma.saturating_add(delta);
    let karma = progression.crime.karma;
    Ok(ConsoleCommandResult::new(
        json!({ "karma": karma, "delta": delta }),
        vec![format!("modkarma {delta} -> {karma}")],
    ))
}
