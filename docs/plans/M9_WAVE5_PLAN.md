# M9 wave 5 plan — field repair, schematic crafting, barter, restock

## Execution model recommendation

Roadmap recommendation: **Sol X-High** (Codex runtime) / **Opus** (Claude
runtime) — the wave extends the canonical item ledger, adds three core
kernels, and rewires merchant console pricing. ZCode runtime: the
orchestrating session executes directly on `M9-Work`.

## Fixed feature list

### Core

- `repair.rs`: two-item field repair. Integer condition 0–`max_condition`.
  `bonus = MaxCond * 25 * skill / 10_000`;
  `cap = max(50, skill) * MaxCond / 100`;
  `NewCond = min(CondA+CondB+bonus, cap, MaxCond)`.
  Compatibility is the same `base_form_id`. Equipped donors and identical
  target/donor IDs are rejected. One donor unit is consumed. Replay of a
  finalized `TransactionId` returns the original receipt.
- `crafting.rs`: consume ingredients by sorted `ItemInstanceId`, allocate
  outputs through `insert_new_item` on a cloned candidate so a failed craft
  does not bump `next_item_id`. Any opaque CTDA (`has_conditions`) is
  `UnsupportedCondition`. Recipe `level` is the required skill value.
  `SchematicTier` is recorded on the receipt; v1 does not yet apply a
  starting-condition bonus (see A5).
- `barter.rs`: FO3 ESM millifactors only
  (`fBarterBuyBase` 1550, `fBarterBuyMult` −450, `fBarterSellBase` 450,
  `fBarterSellMult` 450). Unit price `(base * factor_milli) / 1000`, clamp
  ≥1 if base > 0. `quote_barter` then `commit_barter` via
  `ItemLedger::execute_quoted` with expected player/merchant revisions.
- `MerchantRestockState` + `restock_if_due`; interval
  `72 * 3_600 * 1_000` ms. Policy only.

### Ledger

- `TransactionError::StaleRevision`.
- `execute_quoted` replays finalized IDs, then checks holder revisions.
- Snapshot `repair_finalized` / `craft_finalized` with `#[serde(default)]`
  so existing ITMS RON still loads. Save format stays v9.

### Viewer

- Load `recipes.ron` into `RecipeCatalog` (empty + warn on miss/stale).
- `repairitem <target> <donor>` and `craftitem <recipe> [count]`.
- `buy` / `sell` quote from player Barter skill, then commit. Console never
  accepts a user `unit_price`.

## Tests-first order

1. `features/rpg_repair_barter.feature`.
2. Cucumber World fields after `rpg_actor_state`; steps at EOF.
3. Core unit tests (`repair.rs`, `crafting.rs`, `barter.rs`).
4. Console adapters.
5. Implement until green; gates; `M9_WAVE5_MANUAL.md`.

## Acceptance gates

- Repair 40+40 skill 50 max 100 → condition 50, donor stack loses 1.
- Repair 10+10 skill 100 max 100 → condition 45 (bonus 25, cap 100).
- Same-item, incompatible form, equipped donor rejected; replay tx 7 does
  not consume a second donor.
- Craft consumes 2 of 5, allocates 1 output; opaque CTDA changes nothing;
  missing ingredients leave `next_item_id` unchanged.
- Buy skill 0 base 100 → unit 155; sell skill 100 ×2 → unit 90 total 180.
- Stale merchant revision rejects commit.
- Restock not due at 259_199_999 ms; due at 259_200_000 ms, generation 1.
- `cargo fmt --check`, clippy `-D warnings`, tests.

## Shipped amendments

- **A1.** FO3 ESM barter GMSTs only. CHAR, disposition, and haggle GMSTs
  are absent from Fallout3.esm and are not invented here.
- **A2.** Repair compatibility is the same `base_form_id`. No NAM2 repair
  list was found in the probed ESM.
- **A3.** Condition units are canonical `Option<u32>` 0–max (catalog
  `max_condition`). Integer millicap math; not f32 and not limb milli.
- **A4.** Recipes with any opaque CTDA → `UnsupportedCondition`. Empty
  CTDA is allowed.
- **A5.** Schematic tiers v1/v2/v3 are an explicit request field because
  RCPE DATA carries skill/level, not v1–v3. The tier is stored on the
  receipt; a bonus starting condition is **not** applied in this wave.
- **A6.** Restock 72h = 259_200_000 ms; policy-only. Wave 9 activates.
- **A7.** Buy/sell console quotes then commits with expected revisions.
  The ledger still receives `unit_price` from the quote, never from the
  user. Existing core Buy/Sell tests that pass an explicit unit price stay
  valid as the commit authority.
- **A8.** Console `craftitem` compares the player's Repair skill to
  `recipe.level` (required skill). RCPE `skill` remains the AV index on
  the prepared catalog and is not re-interpreted in this wave.
