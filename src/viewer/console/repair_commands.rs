//! Field repair and schematic crafting console commands (M9 wave 5).

use bevyout_core::actor_state::ActorSkill;
use bevyout_core::crafting::{CraftRequest, CraftingActorSnapshot, SchematicTier, craft};
use bevyout_core::item_transaction::HolderId;
use bevyout_core::repair::{RepairRequest, repair};
use serde_json::json;

use super::stats::PlayerProgression;
use super::*;

pub(super) struct RepairCommandProvider;

impl ConsoleCommandProvider for RepairCommandProvider {
    fn register_commands(&self, registry: &mut ConsoleRegistry) -> Result<(), ConsoleError> {
        for command in [
            ConsoleCommand::new(
                "repairitem",
                "[player.]repairitem <target> <donor>",
                "Repair a player item using one compatible donor of the same base form.",
                repair_item,
            )
            .reference_callable(false)
            .mutating(),
            ConsoleCommand::new(
                "craftitem",
                "[player.]craftitem <recipe-formid> [count]",
                "Craft from a prepared schematic, consuming ingredients atomically.",
                craft_item,
            )
            .reference_callable(false)
            .mutating(),
        ] {
            registry.register(command)?;
        }
        Ok(())
    }
}

fn repair_item(
    world: &mut World,
    invocation: &ConsoleInvocation,
) -> Result<ConsoleCommandResult, ConsoleError> {
    let [target, donor] = invocation.args.as_slice() else {
        return Err(ConsoleError::new(
            "bad_arity",
            "repairitem requires a target and donor item instance id",
        ));
    };
    let target = item_commands::parse_item_instance_id(target).ok_or_else(|| {
        ConsoleError::new("bad_type", "target item instance id must be hexadecimal")
    })?;
    let donor = item_commands::parse_item_instance_id(donor).ok_or_else(|| {
        ConsoleError::new("bad_type", "donor item instance id must be hexadecimal")
    })?;
    let skill = world
        .get_resource::<PlayerProgression>()
        .map(|progression| progression.stats.skill_value(ActorSkill::Repair))
        .unwrap_or(0);
    let (max_condition, transaction_id, expected_holder_revision) = {
        let canonical = world.resource::<interaction::CanonicalItemLedger>();
        let item = canonical
            .ledger
            .holders()
            .get(&HolderId::Player)
            .and_then(|state| state.find(target))
            .ok_or_else(|| {
                ConsoleError::new(
                    "item_not_found",
                    "target item is not in the player inventory",
                )
            })?;
        let max_condition = world
            .get_resource::<PreparedItemCatalog>()
            .into_iter()
            .flat_map(|catalog| &catalog.items)
            .find(|definition| definition.base_form_id == item.base_form_id)
            .and_then(|definition| match &definition.stats {
                PreparedItemStats::Weapon { max_condition, .. }
                | PreparedItemStats::Apparel { max_condition, .. } => *max_condition,
                _ => None,
            })
            .ok_or_else(|| {
                ConsoleError::new(
                    "missing_max_condition",
                    "item has no prepared max condition",
                )
            })?;
        (
            max_condition,
            canonical.ledger.next_transaction_id(),
            canonical
                .ledger
                .holders()
                .get(&HolderId::Player)
                .map(|state| state.revision)
                .unwrap_or(0),
        )
    };
    let receipt = {
        let mut canonical = world.resource_mut::<interaction::CanonicalItemLedger>();
        repair(
            &mut canonical.ledger,
            RepairRequest {
                transaction_id,
                holder: HolderId::Player,
                target,
                donor,
                repair_skill: skill,
                max_condition,
                expected_holder_revision,
            },
        )
        .map_err(|error| ConsoleError::new("repair_failed", error.to_string()))?
    };
    Ok(ConsoleCommandResult::new(
        json!({
            "target": receipt.target.0,
            "donor": receipt.donor.0,
            "condition_before": receipt.condition_before,
            "condition_after": receipt.condition_after,
            "cap": receipt.cap,
            "transaction_id": receipt.id.0,
        }),
        vec![format!(
            "repaired {:016x} to {} (cap {})",
            receipt.target.0, receipt.condition_after, receipt.cap
        )],
    ))
}

fn craft_item(
    world: &mut World,
    invocation: &ConsoleInvocation,
) -> Result<ConsoleCommandResult, ConsoleError> {
    if invocation.args.is_empty() || invocation.args.len() > 2 {
        return Err(ConsoleError::new(
            "bad_arity",
            "craftitem expects a recipe FormID and an optional count",
        ));
    }
    let form_id = parse_item_form_id(&invocation.args[0]).ok_or_else(|| {
        ConsoleError::new(
            "bad_type",
            "craftitem FormID must be 1-8 hex digits, e.g. f, 0x1f, or 0000000f",
        )
    })?;
    let count = item_commands::parse_positive_count(invocation.args.get(1))?;
    let recipe = world
        .get_resource::<super::super::recipes::RecipeCatalog>()
        .and_then(|catalog| catalog.0.get(&form_id).cloned())
        .ok_or_else(|| {
            ConsoleError::new("unknown_recipe", "recipe is not in the prepared catalog")
        })?;
    let skill = world
        .get_resource::<PlayerProgression>()
        .map(|progression| progression.stats.skill_value(ActorSkill::Repair))
        .unwrap_or(0);
    let (transaction_id, expected_holder_revision) = {
        let canonical = world.resource::<interaction::CanonicalItemLedger>();
        (
            canonical.ledger.next_transaction_id(),
            canonical
                .ledger
                .holders()
                .get(&HolderId::Player)
                .map(|state| state.revision)
                .unwrap_or(0),
        )
    };
    let receipt = {
        let mut canonical = world.resource_mut::<interaction::CanonicalItemLedger>();
        craft(
            &mut canonical.ledger,
            CraftRequest {
                transaction_id,
                holder: HolderId::Player,
                recipe: &recipe,
                count,
                expected_holder_revision,
                actor: CraftingActorSnapshot { skill_value: skill },
                schematic_tier: SchematicTier::V1,
            },
        )
        .map_err(|error| ConsoleError::new("craft_failed", error.to_string()))?
    };
    Ok(ConsoleCommandResult::new(
        json!({
            "recipe": form_id,
            "count": count,
            "created": receipt.created.iter().map(|(id, form, qty)| json!({
                "item_id": id.0,
                "form_id": form,
                "count": qty,
            })).collect::<Vec<_>>(),
            "transaction_id": receipt.id.0,
        }),
        vec![format!("crafted recipe {form_id:08x} x{count}")],
    ))
}
