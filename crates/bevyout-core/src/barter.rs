//! Fallout 3 barter quotes and merchant restock policy (M9 wave 5).
//!
//! Pricing uses only GMSTs present in Fallout3.esm:
//! `fBarterBuyBase` 1.55, `fBarterBuyMult` -0.45, `fBarterSellBase` 0.45,
//! `fBarterSellMult` 0.45. Charisma, disposition, and haggle GMSTs are absent
//! from that ESM and are not invented here. Restock is a 72-hour game-time
//! policy; Wave 9 owns the scheduler.

use serde::{Deserialize, Serialize};

use crate::chems::RpgRngState;
use crate::item_transaction::{
    HolderId, ItemInstanceId, ItemLedger, TransactionError, TransactionId, TransactionReceipt,
    TransactionRequest,
};
use crate::time::GameTime;

pub const BARTER_SETTINGS_REVISION: &str = "fo3-barter-v1";
pub const BARTER_BUY_BASE_MILLI: i32 = 1550;
pub const BARTER_BUY_MULT_MILLI: i32 = -450;
pub const BARTER_SELL_BASE_MILLI: i32 = 450;
pub const BARTER_SELL_MULT_MILLI: i32 = 450;
pub const MERCHANT_RESTOCK_INTERVAL_MS: u64 = 72 * 3_600 * 1_000;

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum BarterDirection {
    Buy,
    Sell,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BarterQuoteInput {
    pub direction: BarterDirection,
    pub merchant: HolderId,
    pub player: HolderId,
    pub item_id: ItemInstanceId,
    pub count: u32,
    pub base_value: u64,
    pub player_barter: u8,
    pub player_revision: u64,
    pub merchant_revision: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct BarterQuote {
    pub direction: BarterDirection,
    pub merchant: HolderId,
    pub player: HolderId,
    pub item_id: ItemInstanceId,
    pub count: u32,
    pub base_value: u64,
    pub player_barter: u8,
    pub factor_milli: i32,
    pub unit_price: u64,
    pub total: u64,
    pub player_revision: u64,
    pub merchant_revision: u64,
    pub settings_revision: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BarterError {
    Transaction(TransactionError),
    StaleQuote,
    InvalidCount,
    InvalidPrice,
}

impl std::fmt::Display for BarterError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Transaction(error) => write!(formatter, "{error}"),
            Self::StaleQuote => formatter.write_str("barter quote is stale"),
            Self::InvalidCount => formatter.write_str("barter count must be at least 1"),
            Self::InvalidPrice => formatter.write_str("item has no non-negative value"),
        }
    }
}

impl std::error::Error for BarterError {}

impl From<TransactionError> for BarterError {
    fn from(error: TransactionError) -> Self {
        match error {
            TransactionError::StaleRevision => Self::StaleQuote,
            other => Self::Transaction(other),
        }
    }
}

#[must_use]
pub fn barter_factor_milli(direction: BarterDirection, player_barter: u8) -> i32 {
    let skill = i32::from(player_barter.min(100));
    match direction {
        BarterDirection::Buy => BARTER_BUY_BASE_MILLI + BARTER_BUY_MULT_MILLI * skill / 100,
        BarterDirection::Sell => BARTER_SELL_BASE_MILLI + BARTER_SELL_MULT_MILLI * skill / 100,
    }
}

#[must_use]
pub fn barter_unit_price(base_value: u64, factor_milli: i32) -> u64 {
    if base_value == 0 {
        return 0;
    }
    let factor = i64::from(factor_milli.max(0));
    let price = (base_value as i64).saturating_mul(factor) / 1_000;
    price.max(1) as u64
}

pub fn quote_barter(input: BarterQuoteInput) -> Result<BarterQuote, BarterError> {
    if input.count == 0 {
        return Err(BarterError::InvalidCount);
    }
    let factor_milli = barter_factor_milli(input.direction, input.player_barter);
    let unit_price = barter_unit_price(input.base_value, factor_milli);
    let total = unit_price
        .checked_mul(u64::from(input.count))
        .ok_or(BarterError::Transaction(TransactionError::CapsOverflow))?;
    Ok(BarterQuote {
        direction: input.direction,
        merchant: input.merchant,
        player: input.player,
        item_id: input.item_id,
        count: input.count,
        base_value: input.base_value,
        player_barter: input.player_barter,
        factor_milli,
        unit_price,
        total,
        player_revision: input.player_revision,
        merchant_revision: input.merchant_revision,
        settings_revision: BARTER_SETTINGS_REVISION.into(),
    })
}

pub fn commit_barter(
    ledger: &mut ItemLedger,
    id: TransactionId,
    quote: &BarterQuote,
) -> Result<TransactionReceipt, BarterError> {
    let request = match quote.direction {
        BarterDirection::Buy => TransactionRequest::Buy {
            merchant: quote.merchant,
            player: quote.player,
            item_id: quote.item_id,
            count: quote.count,
            unit_price: quote.unit_price,
        },
        BarterDirection::Sell => TransactionRequest::Sell {
            player: quote.player,
            merchant: quote.merchant,
            item_id: quote.item_id,
            count: quote.count,
            unit_price: quote.unit_price,
        },
    };
    Ok(ledger.execute_quoted(id, request, quote.player_revision, quote.merchant_revision)?)
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct MerchantRestockState {
    pub generation: u32,
    pub last_restock_game_ms: u64,
    pub next_restock_game_ms: u64,
    pub seed_state: RpgRngState,
}

impl Default for MerchantRestockState {
    fn default() -> Self {
        Self {
            generation: 0,
            last_restock_game_ms: 0,
            next_restock_game_ms: MERCHANT_RESTOCK_INTERVAL_MS,
            seed_state: RpgRngState::default(),
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct MerchantStockCatalog {
    pub entries: Vec<(u32, u32)>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RestockOutcome {
    pub due: bool,
    pub generation: u32,
}

pub fn restock_if_due(
    now: GameTime,
    state: &mut MerchantRestockState,
    _catalog: &MerchantStockCatalog,
) -> RestockOutcome {
    if now.as_ms() < state.next_restock_game_ms {
        return RestockOutcome {
            due: false,
            generation: state.generation,
        };
    }
    state.generation = state.generation.saturating_add(1);
    state.last_restock_game_ms = now.as_ms();
    state.next_restock_game_ms = now.as_ms().saturating_add(MERCHANT_RESTOCK_INTERVAL_MS);
    let _ = state.seed_state.next_u64();
    RestockOutcome {
        due: true,
        generation: state.generation,
    }
}

#[cfg(test)]
#[path = "tests/barter.rs"]
mod tests;
