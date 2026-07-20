//! Actor-state inspection and developer mutation commands for issue #110.

use std::collections::BTreeSet;

use bevyout_core::actor_state::{
    ALL_ACTOR_VALUES, ActorLifeState, ActorPackageCheckpoint, ActorValue,
};

use super::*;

pub(super) struct ActorStateCommandProvider;

impl ConsoleCommandProvider for ActorStateCommandProvider {
    fn register_commands(&self, registry: &mut ConsoleRegistry) -> Result<(), ConsoleError> {
        for command in [
            ConsoleCommand::new(
                "actorstate",
                "actorstate <actor-reference>",
                "Report immutable values/factions plus persisted lifecycle, mutations, package checkpoint, reference delta, and canonical actor inventory/equipment.",
                actor_state,
            ),
            ConsoleCommand::new(
                "setactorvalue",
                "setactorvalue <actor-reference> <value> <mutation>",
                "Set a persisted additive mutation for one prepared actor value; zero clears it.",
                set_actor_value,
            )
            .mutating(),
            ConsoleCommand::new(
                "setactorlife",
                "setactorlife <actor-reference> <alive|dead>",
                "Set the persisted actor lifecycle flag. Corpse conversion remains owned by combat/death work.",
                set_actor_life,
            )
            .mutating(),
            ConsoleCommand::new(
                "setactorpackage",
                "setactorpackage <actor-reference> <none|package-formid> [procedure-index elapsed-seconds]",
                "Set or clear the persisted package checkpoint consumed by the later AI package executor.",
                set_actor_package,
            )
            .mutating(),
        ] {
            registry.register(command)?;
        }
        Ok(())
    }
}

fn actor_reference(world: &World, selector: &str) -> Result<u32, ConsoleError> {
    if let Ok(entity) = resolve_reference(world, selector) {
        return world
            .get::<actor::ActorRuntime>(entity)
            .map(|runtime| runtime.reference_form_id)
            .ok_or_else(|| ConsoleError::new("not_actor", "reference is not a projected actor"));
    }
    let reference_form_id = parse_item_form_id(selector).ok_or_else(|| {
        ConsoleError::new(
            "bad_type",
            "actor reference must be a live selector or a 1-8 hex digit FormID",
        )
    })?;
    let found = world
        .get_resource::<super::super::world::ActiveSaveState>()
        .is_some_and(|save| {
            save.0
                .cells
                .values()
                .any(|cell| cell.actors.contains_key(&reference_form_id))
        });
    if !found {
        return Err(ConsoleError::new(
            "unknown_actor",
            format!("no live or persisted actor {reference_form_id:08x}"),
        ));
    }
    Ok(reference_form_id)
}

fn actor_cell_and_state(
    world: &World,
    reference_form_id: u32,
) -> Result<(u32, &bevyout_core::actor_state::ActorInstanceState), ConsoleError> {
    world
        .get_resource::<super::super::world::ActiveSaveState>()
        .and_then(|save| {
            save.0.cells.iter().find_map(|(cell_form_id, cell)| {
                cell.actors
                    .get(&reference_form_id)
                    .map(|state| (*cell_form_id, state))
            })
        })
        .ok_or_else(|| {
            ConsoleError::new(
                "actor_state_unavailable",
                format!("actor {reference_form_id:08x} has no seeded mutable state yet"),
            )
        })
}

fn actor_state_mut(
    world: &mut World,
    reference_form_id: u32,
) -> Result<&mut bevyout_core::actor_state::ActorInstanceState, ConsoleError> {
    world
        .get_resource_mut::<super::super::world::ActiveSaveState>()
        .and_then(|save| {
            save.into_inner()
                .0
                .cells
                .values_mut()
                .find_map(|cell| cell.actors.get_mut(&reference_form_id))
        })
        .ok_or_else(|| {
            ConsoleError::new(
                "actor_state_unavailable",
                format!("actor {reference_form_id:08x} has no seeded mutable state yet"),
            )
        })
}

fn actor_definition(
    world: &World,
    reference_form_id: u32,
) -> Option<bevyout_core::actor_state::ActorDefinition> {
    let live = world.iter_entities().find_map(|entity| {
        let runtime = entity.get::<actor::ActorRuntime>()?;
        (runtime.reference_form_id == reference_form_id).then(|| {
            entity
                .get::<actor_state::ActorStateRuntime>()
                .map(|state| state.definition.as_ref().clone())
                .or_else(|| {
                    world
                        .get_resource::<actor_state::ActorDefinitionCatalogs>()
                        .and_then(|catalogs| {
                            catalogs
                                .definition(reference_form_id, runtime.base_form_id)
                                .map(|(_, definition)| definition)
                        })
                })
        })
    });
    live.flatten().or_else(|| {
        world
            .get_resource::<actor_state::ActorDefinitionCatalogs>()
            .and_then(|catalogs| catalogs.definition(reference_form_id, 0))
            .map(|(_, definition)| definition)
    })
}

fn actor_state(
    world: &mut World,
    invocation: &ConsoleInvocation,
) -> Result<ConsoleCommandResult, ConsoleError> {
    let [selector] = invocation.args.as_slice() else {
        return Err(ConsoleError::new(
            "bad_arity",
            "actorstate requires exactly one actor reference",
        ));
    };
    let reference_form_id = actor_reference(world, selector)?;
    let (cell_form_id, state) = actor_cell_and_state(world, reference_form_id)?;
    let state = state.clone();
    let definition = actor_definition(world, reference_form_id);
    let mut value_keys = BTreeSet::new();
    if let Some(definition) = &definition {
        for layer in [
            &definition.template_values,
            &definition.base_values,
            &definition.race_modifiers,
            &definition.class_modifiers,
            &definition.faction_modifiers,
        ] {
            value_keys.extend(layer.keys().copied());
        }
    }
    value_keys.extend(state.value_mutations.keys().copied());
    let values = value_keys
        .into_iter()
        .map(|value| {
            let resolved = definition
                .as_ref()
                .map(|definition| definition.resolve_value(&state, value));
            json!({
                "name": value.label(),
                "inherited_or_base": resolved.map(|resolved| resolved.inherited_or_base),
                "authored_modifier": resolved.map(|resolved| resolved.authored_modifier),
                "runtime_mutation": state.value_mutations.get(&value).copied().unwrap_or_default(),
                "effective": resolved.map(|resolved| resolved.effective),
            })
        })
        .collect::<Vec<_>>();
    let holder = HolderId::Actor { reference_form_id };
    let canonical = world.get_resource::<interaction::CanonicalItemLedger>();
    let holder_state = canonical.and_then(|canonical| canonical.ledger.holders().get(&holder));
    let bindings = canonical.and_then(|canonical| canonical.ledger.bindings().get(&holder));
    let reference_delta = world
        .get_resource::<super::super::world::ActiveSaveState>()
        .and_then(|save| save.0.cells.get(&cell_form_id))
        .and_then(|cell| cell.references.get(&reference_form_id));
    let summary = format!(
        "actorstate {reference_form_id:08x} cell={cell_form_id:08x} life={} values={} factions={} items={} equipped={}",
        state.life_state.label(),
        values.len(),
        definition
            .as_ref()
            .map_or(0, |definition| definition.factions.len()),
        holder_state.map_or(0, |holder| holder.items.len()),
        bindings
            .and_then(|bindings| bindings.equipped)
            .map_or_else(|| "none".to_owned(), |item| item.0.to_string()),
    );
    Ok(ConsoleCommandResult::new(
        json!({
            "reference_form_id": reference_form_id,
            "cell_form_id": cell_form_id,
            "definition": definition.as_ref().map(|definition| json!({
                "base_form_id": definition.base_form_id,
                "kind": format!("{:?}", definition.kind),
                "race_form_id": definition.race_form_id,
                "class_form_id": definition.class_form_id,
                "factions": definition.factions.iter().map(|membership| json!({
                    "faction_form_id": membership.faction_form_id,
                    "rank": membership.rank,
                    "title": membership.title,
                })).collect::<Vec<_>>(),
                "package_form_ids": definition.package_form_ids,
            })),
            "life_state": state.life_state.label(),
            "values": values,
            "package": state.package.map(|package| json!({
                "form_id": package.package_form_id,
                "procedure_index": package.procedure_index,
                "elapsed_seconds": package.elapsed_seconds,
            })),
            "reference_delta": reference_delta.map(|delta| json!({
                "enabled": delta.enabled,
                "deleted": delta.deleted,
                "activated": delta.activated,
                "moved": delta.transform.is_some(),
            })),
            "canonical": {
                "present": holder_state.is_some(),
                "items": holder_state.map_or_else(Vec::new, |holder| holder.items.iter().map(|item| json!({
                    "instance_id": item.id.0,
                    "base_form_id": item.base_form_id,
                    "count": item.count,
                    "condition": item.state.condition,
                })).collect::<Vec<_>>()),
                "equipped_instance_id": bindings.and_then(|bindings| bindings.equipped).map(|item| item.0),
            },
        }),
        vec![summary],
    ))
}

fn set_actor_value(
    world: &mut World,
    invocation: &ConsoleInvocation,
) -> Result<ConsoleCommandResult, ConsoleError> {
    let [selector, value, mutation] = invocation.args.as_slice() else {
        return Err(ConsoleError::new(
            "bad_arity",
            "setactorvalue requires an actor reference, value, and mutation",
        ));
    };
    let reference_form_id = actor_reference(world, selector)?;
    let value = ActorValue::parse(value).ok_or_else(|| {
        ConsoleError::new(
            "bad_value",
            format!(
                "unknown actor value; expected one of {}",
                ALL_ACTOR_VALUES
                    .iter()
                    .map(|value| value.label())
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
        )
    })?;
    let mutation = mutation.parse::<f32>().map_err(|_| {
        ConsoleError::new("bad_type", "actor value mutation must be a finite number")
    })?;
    actor_state_mut(world, reference_form_id)?
        .set_value_mutation(value, mutation)
        .map_err(|error| ConsoleError::new("bad_value", error.to_string()))?;
    Ok(ConsoleCommandResult::new(
        json!({
            "reference_form_id": reference_form_id,
            "value": value.label(),
            "mutation": mutation,
        }),
        vec![format!(
            "setactorvalue {reference_form_id:08x} {} {mutation}",
            value.label()
        )],
    ))
}

fn set_actor_life(
    world: &mut World,
    invocation: &ConsoleInvocation,
) -> Result<ConsoleCommandResult, ConsoleError> {
    let [selector, life] = invocation.args.as_slice() else {
        return Err(ConsoleError::new(
            "bad_arity",
            "setactorlife requires an actor reference and alive or dead",
        ));
    };
    let reference_form_id = actor_reference(world, selector)?;
    let life_state = match life.as_str() {
        "alive" => ActorLifeState::Alive,
        "dead" => ActorLifeState::Dead,
        _ => {
            return Err(ConsoleError::new(
                "bad_value",
                "actor life state must be alive or dead",
            ));
        }
    };
    actor_state_mut(world, reference_form_id)?.life_state = life_state;
    Ok(ConsoleCommandResult::new(
        json!({
            "reference_form_id": reference_form_id,
            "life_state": life_state.label(),
        }),
        vec![format!(
            "setactorlife {reference_form_id:08x} {}",
            life_state.label()
        )],
    ))
}

fn set_actor_package(
    world: &mut World,
    invocation: &ConsoleInvocation,
) -> Result<ConsoleCommandResult, ConsoleError> {
    if invocation.args.len() < 2 {
        return Err(ConsoleError::new(
            "bad_arity",
            "setactorpackage requires an actor reference and none or a package FormID",
        ));
    }
    let selector = &invocation.args[0];
    let package = &invocation.args[1];
    let tail = &invocation.args[2..];
    let reference_form_id = actor_reference(world, selector)?;
    let checkpoint = if package == "none" {
        if !tail.is_empty() {
            return Err(ConsoleError::new(
                "bad_arity",
                "setactorpackage <reference> none takes no checkpoint fields",
            ));
        }
        None
    } else {
        let [procedure, elapsed] = tail else {
            return Err(ConsoleError::new(
                "bad_arity",
                "setactorpackage requires procedure index and elapsed seconds",
            ));
        };
        let package_form_id = parse_item_form_id(package).ok_or_else(|| {
            ConsoleError::new("bad_type", "package FormID must be 1-8 hex digits")
        })?;
        let procedure_index = procedure.parse::<u32>().map_err(|_| {
            ConsoleError::new("bad_type", "package procedure index must be an integer")
        })?;
        let elapsed_seconds = elapsed.parse::<f32>().map_err(|_| {
            ConsoleError::new("bad_type", "package elapsed seconds must be a number")
        })?;
        Some(ActorPackageCheckpoint {
            package_form_id,
            procedure_index,
            elapsed_seconds,
        })
    };
    let state = actor_state_mut(world, reference_form_id)?;
    let previous = state.package;
    state.package = checkpoint;
    if let Err(error) = state.validate() {
        state.package = previous;
        return Err(ConsoleError::new("bad_value", error.to_string()));
    }
    Ok(ConsoleCommandResult::new(
        json!({
            "reference_form_id": reference_form_id,
            "package": checkpoint.map(|checkpoint| json!({
                "form_id": checkpoint.package_form_id,
                "procedure_index": checkpoint.procedure_index,
                "elapsed_seconds": checkpoint.elapsed_seconds,
            })),
        }),
        vec![format!(
            "setactorpackage {reference_form_id:08x} {}",
            checkpoint.map_or_else(
                || "none".to_owned(),
                |checkpoint| format!(
                    "{:08x} {} {}",
                    checkpoint.package_form_id,
                    checkpoint.procedure_index,
                    checkpoint.elapsed_seconds
                )
            )
        )],
    ))
}
