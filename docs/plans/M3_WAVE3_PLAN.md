# M3 wave 3 — caps, ownership, and quest-item flags (#81)

Single-issue wave under epic #7. No UI: prepare-side flag plumbing plus pure
runtime policy; existing screens only consume the new rejection results.

## Fixed feature list

- **F81.1 Quest-item flag in the catalog.** `parse_base` gains the record
  header flags it already sees at the reader call site;
  `BaseRecord.record_flags` feeds `PreparedItemDefinition.quest_item`
  (header flag `0x0000_0400`, serde-default `false` so wave 1/2 catalogs
  stay readable). `ITEM_CATALOG_REVISION` bumps so caches regenerate.
- **F81.2 Quest-item rules.** Pure `item_rules` module: quest items cannot
  be dropped to the world or stored into containers; carried weight
  excludes quest items (FO3 rules). Taking/picking up stays allowed.
- **F81.3 Caps as currency.** Caps base record `0x0000000F`: queryable
  total on the player inventory, never droppable to the world, still
  transferable to/from containers like FO3. No barter in M3.
- **F81.4 Ownership / theft classification.** Placements already carry
  `owner_form_id`/`owner_faction_rank`. Pure classification `Take` vs
  `Steal` (any owner counts as theft — no faction-membership model yet,
  ceiling noted in code), consumed by world pickup and container take,
  logged with stable prefix `steal <formid> owner <owner>`. No crime/karma
  consequences in M3. Player-dropped items stay owner-less.

## Order

1. `features/item_flags.feature` + steps appended to `tests/features.rs`
   (World fields at end of struct, delimited section at end of file).
2. Unit tests beside the touched modules.
3. Implementation until green; gates (`fmt --check`, `clippy -D warnings`,
   `test`, representative `run-dev`); real-data acceptance evidence on #81.

Design note: `container_policy` ops keep their #75 shapes; quest-item store
rejection lives in `item_rules::can_store`, checked by callers before the
policy op, so the conservation seam is untouched.

## Shipped amendments

- **A18** — the live `steal` log line is not demonstrable on the prepared
  cell: MegatonPlayerHouse's only two `XOWN` references are owned by the
  player (form `0x7`, correctly classified `Take`), and the console has no
  `additem`-style command to stage owned loot. Gate evidence is therefore
  the prepared-data counts plus the cucumber classification scenarios; a
  live capture waits for a cell with hostile-owned loot (M4+ actor cells).
