//! Player chem/aid/radiation console commands (M9 wave 3, #318).
//!
//! Player-scoped like the stats and perk commands: the Bethesda `player.`
//! prefix is accepted via `.reference_callable(false)` and every target is
//! the FPS player's wave-3 components (`PlayerRadiation`,
//! `ActiveEffectsList`, `Addictions`, `PlayerVitals`). Definitions come
//! from the prepared effect catalog (#316), and addiction rolls consume
//! the seeded core PRNG resource so runs reproduce exactly.

use bevyout_core::chems::AddictionPhase;
use bevyout_core::effects::{EffectSource, PERMANENT_MS};

use super::stats::{ActorStats, StatsSettings};
use super::*;

pub(super) struct EffectCommandProvider;

impl ConsoleCommandProvider for EffectCommandProvider {
    fn register_commands(&self, registry: &mut ConsoleRegistry) -> Result<(), ConsoleError> {
        for command in [
            ConsoleCommand::new(
                "rads",
                "[player.]rads",
                "Report the player's radiation dose and its SPECIAL penalties.",
                rads,
            )
            .reference_callable(false),
            ConsoleCommand::new(
                "addrads",
                "[player.]addrads <n>",
                "Apply n rads of environmental dose through active RadResist.",
                add_rads,
            )
            .reference_callable(false)
            .mutating(),
            ConsoleCommand::new(
                "removerads",
                "[player.]removerads <n>",
                "Remove up to n rads from the player (RadAway semantics).",
                remove_rads,
            )
            .reference_callable(false)
            .mutating(),
            ConsoleCommand::new(
                "addchem",
                "[player.]addchem <FormID>",
                "Apply a cataloged ingestible's effects without consuming inventory.",
                add_chem,
            )
            .reference_callable(false)
            .mutating(),
            ConsoleCommand::new(
                "cureaddiction",
                "[player.]cureaddiction [FormID|all]",
                "Cure one withdrawal-FormID addiction or every addiction.",
                cure_addiction,
            )
            .reference_callable(false)
            .mutating(),
            ConsoleCommand::new(
                "effects",
                "[player.]effects",
                "List active effects, chem dose timers, addictions, and rng draws.",
                list_effects,
            )
            .reference_callable(false),
        ] {
            registry.register(command)?;
        }
        Ok(())
    }
}

/// Resolves the wave-3-carrying player entity (same contract as
/// `player_stats_entity`).
fn effects_player_entity(
    world: &mut World,
    invocation: &ConsoleInvocation,
) -> Result<Entity, ConsoleError> {
    if let Some(entity) = invocation.target
        && world
            .get::<super::super::effects::PlayerRadiation>(entity)
            .is_some()
    {
        return Ok(entity);
    }
    let mut query = world.query_filtered::<Entity, With<player::FpsPlayer>>();
    let entity = query
        .single(world)
        .map_err(|_| ConsoleError::new("player_unavailable", "the FPS player does not exist"))?;
    if world
        .get::<super::super::effects::PlayerRadiation>(entity)
        .is_none()
    {
        return Err(ConsoleError::new(
            "player_unavailable",
            "the FPS player has no radiation component",
        ));
    }
    Ok(entity)
}

/// Parses a FormID argument like `additem` (1..=8 hex digits).
fn parse_ingestible_form_id(raw: &str) -> Option<u32> {
    super::common::parse_item_form_id(raw)
}

/// Applies a cataloged ingestible to the player components. Shared by
/// `addchem` and `useitem`; the PRNG draw (when the chem is addictive)
/// consumes the seeded app resource so acceptance runs are reproducible.
pub(super) fn apply_ingestible_to_player(
    world: &mut World,
    definition: &bevyout_core::effects::IngestibleDefinition,
) -> super::super::effects::AppliedIngestible {
    let entity = {
        let mut query = world.query_filtered::<Entity, With<player::FpsPlayer>>();
        query
            .single(world)
            .expect("ingestible commands require the FPS player")
    };
    let settings = *world.resource::<StatsSettings>();
    let stats = world
        .get::<ActorStats>(entity)
        .expect("the FPS player carries ActorStats")
        .clone();
    let perks = world
        .get::<super::super::stats::ActorPerks>(entity)
        .expect("the FPS player carries ActorPerks")
        .clone();
    // The PRNG state is Copy: mutate a local copy while borrowing the
    // player's components, then write it back so the resource stays the
    // single stream authority.
    let mut rng = **world.resource::<super::super::effects::RngResource>();
    let outcome = {
        // One query yields the four player components as disjoint mutable
        // borrows (the same shape the tick systems use).
        let mut query = world.query_filtered::<(
            &mut super::super::effects::PlayerVitals,
            &mut super::super::effects::PlayerRadiation,
            &mut super::super::effects::ActiveEffectsList,
            &mut super::super::effects::Addictions,
        ), With<player::FpsPlayer>>();
        let (vitals, radiation, effects, addictions) = query
            .get_mut(world, entity)
            .expect("wave-3 components attach with the FPS player");
        super::super::effects::apply_ingestible(
            definition,
            &stats,
            &perks,
            &settings,
            super::super::effects::PlayerEffectComponents {
                vitals: vitals.into_inner(),
                radiation: radiation.into_inner(),
                effects: effects.into_inner(),
                addictions: addictions.into_inner(),
            },
            &mut rng,
        )
    };
    let mut rng_resource = world.resource_mut::<super::super::effects::RngResource>();
    rng_resource.0 = rng;
    outcome
}

pub(super) fn rads(
    world: &mut World,
    invocation: &ConsoleInvocation,
) -> Result<ConsoleCommandResult, ConsoleError> {
    if !invocation.args.is_empty() {
        return Err(ConsoleError::new("bad_arity", "rads takes no arguments"));
    }
    let entity = effects_player_entity(world, invocation)?;
    let pool = world
        .get::<super::super::effects::PlayerRadiation>(entity)
        .expect("verified above");
    let penalties = bevyout_core::radiation::radiation_penalties(pool.0.rads);
    let penalty_text = penalties
        .iter()
        .map(|(attribute, penalty)| {
            format!(
                "{}{penalty}",
                bevyout_core::actor_state::ActorValue::Special(*attribute).label()
            )
        })
        .collect::<Vec<_>>()
        .join(", ");
    Ok(ConsoleCommandResult::new(
        json!({
            "rads": pool.0.rads,
            "threshold": pool.0.threshold_reached(),
            "fatal": pool.0.is_fatal(),
            "penalties": penalty_text,
        }),
        vec![format!(
            "{} rads{} ({})",
            pool.0.rads,
            if pool.0.is_fatal() { " FATAL" } else { "" },
            if penalty_text.is_empty() {
                "no penalties"
            } else {
                &penalty_text
            }
        )],
    ))
}

pub(super) fn add_rads(
    world: &mut World,
    invocation: &ConsoleInvocation,
) -> Result<ConsoleCommandResult, ConsoleError> {
    let [amount] = invocation.args.as_slice() else {
        return Err(ConsoleError::new(
            "bad_arity",
            "addrads expects one rad count",
        ));
    };
    let amount = amount
        .parse::<u16>()
        .map_err(|_| ConsoleError::new("bad_type", "addrads expects a whole number of rads"))?;
    let entity = effects_player_entity(world, invocation)?;
    let outcome = {
        let mut query = world.query::<(
            &mut super::super::effects::PlayerRadiation,
            &super::super::effects::ActiveEffectsList,
        )>();
        let (mut pool, effects) = query
            .get_mut(world, entity)
            .expect("the FPS player carries radiation and effects");
        super::super::effects::apply_player_radiation(&mut pool, effects, amount)
    };
    Ok(ConsoleCommandResult::new(
        json!({
            "dose_rads": amount,
            "absorbed_rads": outcome.absorbed_rads,
            "rads": outcome.rads,
            "fatal": outcome.fatal,
        }),
        vec![format!(
            "+{} rads -> {}{}",
            outcome.absorbed_rads,
            outcome.rads,
            if outcome.fatal { " FATAL" } else { "" }
        )],
    ))
}

pub(super) fn remove_rads(
    world: &mut World,
    invocation: &ConsoleInvocation,
) -> Result<ConsoleCommandResult, ConsoleError> {
    let [amount] = invocation.args.as_slice() else {
        return Err(ConsoleError::new(
            "bad_arity",
            "removerads expects one rad count",
        ));
    };
    let amount = amount
        .parse::<u16>()
        .map_err(|_| ConsoleError::new("bad_type", "removerads expects a whole number of rads"))?;
    let entity = effects_player_entity(world, invocation)?;
    let removed = {
        let mut pool = world
            .get_mut::<super::super::effects::PlayerRadiation>(entity)
            .unwrap();
        bevyout_core::radiation::remove_rads(&mut pool.0, amount)
    };
    let remaining = world
        .get::<super::super::effects::PlayerRadiation>(entity)
        .unwrap()
        .0
        .rads;
    Ok(ConsoleCommandResult::new(
        json!({ "removed_rads": removed, "rads": remaining }),
        vec![format!("-{removed} rads -> {remaining}")],
    ))
}

/// Looks up an ingestible in the loaded effect catalog.
fn ingestible_definition(
    world: &World,
    form_id: u32,
) -> Result<bevyout_core::effects::IngestibleDefinition, ConsoleError> {
    world
        .get_resource::<super::super::effects::EffectCatalog>()
        .and_then(|catalog| catalog.get(form_id).cloned())
        .ok_or_else(|| {
            ConsoleError::new(
                "unknown_ingestible",
                format!(
                    "no ingestible {form_id:08x} in the loaded effect catalog; run `prepare` and relaunch"
                ),
            )
        })
}

pub(super) fn add_chem(
    world: &mut World,
    invocation: &ConsoleInvocation,
) -> Result<ConsoleCommandResult, ConsoleError> {
    let [raw] = invocation.args.as_slice() else {
        return Err(ConsoleError::new(
            "bad_arity",
            "addchem expects exactly one ingestible FormID",
        ));
    };
    let form_id = parse_ingestible_form_id(raw).ok_or_else(|| {
        ConsoleError::new("bad_type", "addchem expects a hex FormID, e.g. 00015164")
    })?;
    let definition = ingestible_definition(world, form_id)?;
    let application = apply_ingestible_to_player(world, &definition);
    Ok(ConsoleCommandResult::new(
        json!({
            "form_id": form_id,
            "application": application_json(&application),
        }),
        vec![application_summary(&application)],
    ))
}

pub(super) fn cure_addiction(
    world: &mut World,
    invocation: &ConsoleInvocation,
) -> Result<ConsoleCommandResult, ConsoleError> {
    if invocation.args.len() > 1 {
        return Err(ConsoleError::new(
            "bad_arity",
            "cureaddiction takes one optional withdrawal FormID or 'all'",
        ));
    }
    let entity = effects_player_entity(world, invocation)?;
    match invocation.args.first().map(String::as_str) {
        None | Some("all") => {
            let cured = {
                let mut addictions = world
                    .get_mut::<super::super::effects::Addictions>(entity)
                    .unwrap();
                addictions.0.cure_all()
            };
            // Cured players stop withdrawing immediately.
            world
                .get_mut::<super::super::effects::ActiveEffectsList>(entity)
                .unwrap()
                .ledger
                .clear_source(EffectSource::Withdrawal);
            Ok(ConsoleCommandResult::new(
                json!({ "cured": cured }),
                vec![format!("cured {cured} addiction(s)")],
            ))
        }
        Some(raw) => {
            let form_id = parse_ingestible_form_id(raw).ok_or_else(|| {
                ConsoleError::new("bad_type", "cureaddiction expects a hex FormID or 'all'")
            })?;
            let cured = {
                let mut addictions = world
                    .get_mut::<super::super::effects::Addictions>(entity)
                    .unwrap();
                addictions.0.cure(form_id)
            };
            if !cured {
                return Err(ConsoleError::new(
                    "not_addicted",
                    format!("the player has no addiction with withdrawal {form_id:08x}"),
                ));
            }
            // Withdrawal entries are permanent until cured; dropping the
            // addiction drops every withdrawal modifier with it.
            world
                .get_mut::<super::super::effects::ActiveEffectsList>(entity)
                .unwrap()
                .ledger
                .clear_source(EffectSource::Withdrawal);
            Ok(ConsoleCommandResult::new(
                json!({ "cured": true, "withdrawal_form_id": form_id }),
                vec![format!("cured withdrawal {form_id:08x}")],
            ))
        }
    }
}

pub(super) fn list_effects(
    world: &mut World,
    invocation: &ConsoleInvocation,
) -> Result<ConsoleCommandResult, ConsoleError> {
    if !invocation.args.is_empty() {
        return Err(ConsoleError::new("bad_arity", "effects takes no arguments"));
    }
    let entity = effects_player_entity(world, invocation)?;
    let catalog = world.resource::<super::super::effects::EffectCatalog>();
    let rng = world.resource::<super::super::effects::RngResource>();
    let list = world
        .get::<super::super::effects::ActiveEffectsList>(entity)
        .unwrap();
    let addictions = world
        .get::<super::super::effects::Addictions>(entity)
        .unwrap();
    let mut entries = Vec::new();
    for entry in &list.ledger.entries {
        entries.push(json!({
            "source": entry.source.label(),
            "actor_value": entry.actor_value.label(),
            "magnitude": entry.magnitude,
            "remaining_ms": entry.remaining_ms,
            "permanent": entry.remaining_ms == PERMANENT_MS,
        }));
    }
    let doses = list
        .chem_doses_ms
        .iter()
        .map(|(&form_id, remaining)| {
            json!({ "withdrawal_form_id": format!("{form_id:08x}"), "buff_remaining_ms": remaining })
        })
        .collect::<Vec<_>>();
    let addiction_list = addictions
        .0
        .0
        .iter()
        .map(|(&form_id, phase)| {
            let editor_id = catalog
                .ingestibles
                .values()
                .find(|def| def.withdrawal_form_id == form_id)
                .map(|def| def.editor_id.clone());
            json!({
                "withdrawal_form_id": format!("{form_id:08x}"),
                "phase": match phase {
                    AddictionPhase::Addicted => "addicted",
                    AddictionPhase::Withdrawing => "withdrawing",
                    AddictionPhase::Clean => "clean",
                },
                "chem": editor_id,
            })
        })
        .collect::<Vec<_>>();
    Ok(ConsoleCommandResult::new(
        json!({
            "active_effects": entries,
            "chem_doses": doses,
            "addictions": addiction_list,
            "rng_draw_index": rng.draw_index,
        }),
        vec![format!(
            "{} active effect(s), {} running chem dose(s), {} addiction(s), rng at draw {}",
            entries.len(),
            doses.len(),
            addiction_list.len(),
            rng.draw_index
        )],
    ))
}

/// One applied ingestible as JSON (shared by `addchem` and `useitem`).
pub(super) fn application_json(application: &super::super::effects::AppliedIngestible) -> Value {
    json!({
        "editor_id": application.editor_id,
        "healed_to": application.healed_to,
        "rads_removed": application.rads_removed,
        "rads_added": application.rads_added,
        "applied_modifiers": application.applied_modifiers,
        "condition_false": application.condition_false,
        "condition_unsupported": application.condition_unsupported,
        "skipped_conditioned_effects": application.condition_false + application.condition_unsupported,
        "addiction_roll": application.addiction_roll,
        "rng_draw_index": application.rng_draw_index,
    })
}

/// One-line human summary of an applied ingestible.
pub(super) fn application_summary(
    application: &super::super::effects::AppliedIngestible,
) -> String {
    let mut parts = Vec::new();
    if let Some(health) = application.healed_to {
        parts.push(format!("health {health:.0}"));
    }
    if application.rads_removed > 0 {
        parts.push(format!("-{} rads", application.rads_removed));
    }
    if application.rads_added > 0 {
        parts.push(format!("+{} rads", application.rads_added));
    }
    if application.applied_modifiers > 0 {
        parts.push(format!(
            "{} timed modifier(s)",
            application.applied_modifiers
        ));
    }
    if application.condition_false > 0 {
        parts.push(format!(
            "{} conditioned effect(s) false",
            application.condition_false
        ));
    }
    if application.condition_unsupported > 0 {
        parts.push(format!(
            "{} conditioned effect(s) unsupported",
            application.condition_unsupported
        ));
    }
    if let Some(rolled) = application.addiction_roll {
        parts.push(if rolled {
            "ADDICTED".to_string()
        } else {
            "no addiction".to_string()
        });
    }
    format!("{}: {}", application.editor_id, parts.join(", "))
}
