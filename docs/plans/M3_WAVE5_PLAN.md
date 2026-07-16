# M3 wave 5 plan — canonical item instances and atomic holder transactions (#95)

## Fixed design

`src/item_transaction.rs` is the Bevy-free authority. `ItemInstance` carries
`ItemInstanceId`, base FormID, positive count, condition, ownership provenance,
and sorted unique `ItemExtraEntry` values (`namespace_form_id`, four-byte tag,
opaque payload). Stack compatibility includes every mutable state field except
ID and count.

`ItemLedger` owns `ItemHolderState` snapshots for player, fixture containers,
fixture merchants, runtime world references, and future corpse adapters. A
transaction stages cloned source/destination holders and bindings, validates
all legs, then commits together. Full transfers preserve IDs, partial transfers
allocate a new destination ID, and compatible merges keep the lowest ID while
returning a remap. Transaction IDs are monotonic and successful receipts make
repeated requests idempotent; failed requests consume their transaction ID but
leave holder state unchanged.

The current Bevy inventory and container resources remain projections during
the migration. Their add/drop/pickup/container-transfer/save seams synchronize
with the canonical ledger, so the existing Pip-Boy and container modal keep
their public surface while conditions and opaque state survive the move.

## Persistence and API surface

Save format v3 adds an `ITMS` canonical snapshot encoded deterministically in
RON, including holder state, bindings, transaction receipts/high-water marks,
and item counters. v1/v2 inventory, container, and dropped records remain
decodable; migration assigns IDs in deterministic player/cell/reference order
and defaults new fields. The legacy `ItemStack` DTO remains for old records and
projection compatibility.

Console/API coverage includes stable-ID equip, unequip, hotkey, and one-unit
use, plus `setmerchant`, `buy`, and `sell`. Merchants are static, seeded once,
and priced from catalog base value in both directions. Quest items and caps are
not tradeable; combat, services, restocking, speech, and crime effects are
not part of this gate.

## Tests-first and acceptance

- `features/item_transactions.feature` covers partial identity-preserving
  transfer and atomic failure; `src/item_transaction.rs` adds unit and
  property coverage for conservation, exact compatibility, remaps,
  idempotency, opaque state, and merchant caps.
- Save tests cover v3 deterministic round-trip/re-save, opaque state, and
  deterministic legacy migration. Bevy tests cover the existing pickup/drop,
  container, and save seams.
- Run `cargo fmt --check`, `cargo check-dev`, `cargo test-dev`,
  `cargo clippy --all-targets -- -D warnings`, `cargo test`, and a representative
  `cargo run-dev -- prepare ...`. The real-data gate uses the existing
  Super-Duper Mart prepared scene, checks exact IDs/counts/conditions/ownership
  and caps before/after the full lifecycle, and records bridge/console evidence
  against #95 and #8.

## Shipped amendments

- Initial implementation is on the dedicated M3 wave 5 branch. Wave 4's
  `additem`/pickup branch remains an upstream integration dependency; the
  canonical calls are compatible with its runtime seams.
- Drop placement now uses the camera-relative path required by #95: it starts
  at 1.0 m, retreats in 0.1 m increments when the world ray is obstructed, and
  falls back to the top of the player capsule when every candidate is blocked.
  This removes the old ground/airborne rejection gate without changing the
  canonical item transaction or save contracts.
