# M9 wave 5 kickoff — field repair, schematic crafting, barter, restock policy

Requested on 2026-08-28 (continuing remaining M9 work on `M9-Work` per
[M9_Total.md](M9_Total.md)):

- two-item field repair through canonical `ItemLedger` (integer condition,
  same `base_form_id` compatibility);
- schematic crafting over the existing prepared RCPE catalog (opaque CTDA
  is `UnsupportedCondition`, never true);
- barter quote/commit using Fallout3.esm GMSTs only; UI/console never pass
  a user-chosen `unit_price`;
- merchant restock policy `restock_if_due(now, state, catalog)` on explicit
  `GameTime` (72h = 259_200_000 ms); Wave 9 owns the scheduler.

The approved slice is wave 5 only: crime/stealth stay wave 6, lockpicking
stays wave 7, V.A.T.S. stays wave 8, restock *activation* waits for the
wave-9 clock.

Tracked work:

- repair/craft kernels as idempotent ledger mutations with receipts
- FO3 barter quotes committed through existing Buy/Sell
- restock policy without a Bevy timer
- `repairitem` / `craftitem` and quote-aware `buy` / `sell`
