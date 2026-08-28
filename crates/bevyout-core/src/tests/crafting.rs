use super::*;
use crate::item_transaction::{
    HolderId, ItemHolderState, ItemInstance, ItemInstanceId, ItemLedger, ItemState, TransactionId,
};

fn holder(items: Vec<ItemInstance>) -> ItemHolderState {
    ItemHolderState {
        items,
        caps: 0,
        revision: 0,
    }
}

fn item(id: u64, form: u32, count: u32) -> ItemInstance {
    ItemInstance::new(ItemInstanceId(id), form, count, ItemState::default()).unwrap()
}

fn recipe(form_id: u32, has_conditions: bool) -> RecipeDefinition {
    RecipeDefinition {
        form_id,
        skill: 0,
        level: 0,
        ingredients: vec![RecipeItem {
            item_form_id: 0x30,
            quantity: 2,
            order: 0,
        }],
        outputs: vec![RecipeItem {
            item_form_id: 0x40,
            quantity: 1,
            order: 0,
        }],
        has_conditions,
    }
}

#[test]
fn crafting_consumes_ingredients_and_allocates_output_ids() {
    let mut ledger = ItemLedger::new();
    ledger
        .insert_holder(HolderId::Player, holder(vec![item(1, 0x30, 5)]))
        .unwrap();
    let recipe = recipe(0x20, false);
    let request = CraftRequest {
        transaction_id: TransactionId(1),
        holder: HolderId::Player,
        recipe: &recipe,
        count: 1,
        expected_holder_revision: 0,
        actor: CraftingActorSnapshot { skill_value: 50 },
        schematic_tier: SchematicTier::V1,
    };
    let receipt = craft(&mut ledger, request).unwrap();
    assert_eq!(
        ledger.holders()[&HolderId::Player]
            .find(ItemInstanceId(1))
            .unwrap()
            .count,
        3
    );
    assert_eq!(receipt.created[0].1, 0x40);
    assert_eq!(
        ledger.holders()[&HolderId::Player]
            .items
            .iter()
            .find(|item| item.base_form_id == 0x40)
            .map(|item| item.count),
        Some(1)
    );
}

#[test]
fn unsupported_recipe_conditions_change_nothing() {
    let mut ledger = ItemLedger::new();
    ledger
        .insert_holder(HolderId::Player, holder(vec![item(1, 0x30, 5)]))
        .unwrap();
    let before = ledger.snapshot();
    let recipe = recipe(0x21, true);
    let request = CraftRequest {
        transaction_id: TransactionId(1),
        holder: HolderId::Player,
        recipe: &recipe,
        count: 1,
        expected_holder_revision: 0,
        actor: CraftingActorSnapshot { skill_value: 50 },
        schematic_tier: SchematicTier::V1,
    };
    assert_eq!(
        craft(&mut ledger, request).unwrap_err(),
        CraftError::UnsupportedCondition
    );
    assert_eq!(ledger.snapshot(), before);
}

#[test]
fn failed_crafting_does_not_consume_the_next_item_id() {
    let mut ledger = ItemLedger::new();
    ledger
        .insert_holder(HolderId::Player, holder(vec![item(1, 0x30, 1)]))
        .unwrap();
    let next = ledger.next_item_id();
    let recipe = recipe(0x20, false);
    let request = CraftRequest {
        transaction_id: TransactionId(1),
        holder: HolderId::Player,
        recipe: &recipe,
        count: 1,
        expected_holder_revision: 0,
        actor: CraftingActorSnapshot { skill_value: 50 },
        schematic_tier: SchematicTier::V1,
    };
    assert_eq!(
        craft(&mut ledger, request).unwrap_err(),
        CraftError::MissingIngredients
    );
    assert_eq!(ledger.next_item_id(), next);
}
