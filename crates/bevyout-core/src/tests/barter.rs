use super::*;
use crate::item_transaction::{
    HolderId, ItemHolderState, ItemInstance, ItemInstanceId, ItemLedger, ItemState, TransactionId,
};
use crate::time::GameTime;

fn holder(items: Vec<ItemInstance>, caps: u64) -> ItemHolderState {
    ItemHolderState {
        items,
        caps,
        revision: 0,
    }
}

fn item(id: u64, form: u32, count: u32) -> ItemInstance {
    ItemInstance::new(ItemInstanceId(id), form, count, ItemState::default()).unwrap()
}

#[test]
fn barter_buy_uses_fallout_3_buy_gmsts() {
    let quote = quote_barter(BarterQuoteInput {
        direction: BarterDirection::Buy,
        merchant: HolderId::FixtureMerchant {
            reference_form_id: 9,
        },
        player: HolderId::Player,
        item_id: ItemInstanceId(1),
        count: 1,
        base_value: 100,
        player_barter: 0,
        player_revision: 0,
        merchant_revision: 0,
    })
    .unwrap();
    assert_eq!(quote.unit_price, 155);
    assert_eq!(quote.total, 155);
}

#[test]
fn barter_sell_uses_fallout_3_sell_gmsts() {
    let quote = quote_barter(BarterQuoteInput {
        direction: BarterDirection::Sell,
        merchant: HolderId::FixtureMerchant {
            reference_form_id: 9,
        },
        player: HolderId::Player,
        item_id: ItemInstanceId(1),
        count: 2,
        base_value: 100,
        player_barter: 100,
        player_revision: 0,
        merchant_revision: 0,
    })
    .unwrap();
    assert_eq!(quote.unit_price, 90);
    assert_eq!(quote.total, 180);
}

#[test]
fn stale_barter_quote_is_rejected() {
    let merchant = HolderId::FixtureMerchant {
        reference_form_id: 9,
    };
    let mut ledger = ItemLedger::new();
    ledger
        .insert_holder(HolderId::Player, holder(vec![], 200))
        .unwrap();
    ledger
        .insert_holder(merchant, holder(vec![item(2, 0x10, 1)], 50))
        .unwrap();
    let quote = quote_barter(BarterQuoteInput {
        direction: BarterDirection::Buy,
        merchant,
        player: HolderId::Player,
        item_id: ItemInstanceId(2),
        count: 1,
        base_value: 100,
        player_barter: 0,
        player_revision: ledger.holders()[&HolderId::Player].revision,
        merchant_revision: ledger.holders()[&merchant].revision,
    })
    .unwrap();
    ledger.holders_mut().get_mut(&merchant).unwrap().revision += 1;
    assert_eq!(
        commit_barter(&mut ledger, TransactionId(1), &quote).unwrap_err(),
        BarterError::StaleQuote
    );
}

#[test]
fn merchant_restock_is_due_after_72_game_hours() {
    let mut state = MerchantRestockState::default();
    let catalog = MerchantStockCatalog::default();
    let early = restock_if_due(GameTime::from_ms(259_199_999), &mut state, &catalog);
    assert!(!early.due);
    assert_eq!(early.generation, 0);
    let due = restock_if_due(GameTime::from_ms(259_200_000), &mut state, &catalog);
    assert!(due.due);
    assert_eq!(due.generation, 1);
}
