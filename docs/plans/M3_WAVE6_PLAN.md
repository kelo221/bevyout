# M3 wave 6 — equipment, consumable use and reading, Pip-Boy Data views (#98, #99, #100)

Wave under epic #7 on branch `m3-wave6` off `master`. Executors are Sonnet
subagents per AGENTS.md model routing: #98 and #99 run in parallel isolated
worktrees; #100 is stacked and runs directly on the wave branch after #99
merges (it consumes #99's reader overlay). The orchestrator merges, resolves
conflicts, runs gates, and does real-data acceptance before one PR with
`Closes #98`, `Closes #99`, `Closes #100`.

## Fixed feature lists

### #98 — equipment

- **F98.1 Catalog equipment data.** `openmw_esm4/records.rs` decodes ARMO
  BMDT biped-slot mask and WEAP ammo base form id; `prepare/items.rs`
  carries them into serde-defaulted `PreparedItemDefinition`/
  `PreparedItemStats` fields so wave 1/2 catalogs keep deserializing.
- **F98.2 Pure equip-rules module.** New std/serde-only
  `src/viewer/player/equipment.rs` (cucumber-testable via `#[path]`):
  Fallout 3 biped-slot model, equip/unequip keyed by `StackKey`, slot
  conflicts unequip the previous occupant, weapon+ammo pairing,
  condition-aware equipped identity, equipped items cannot be dropped or
  transferred while equipped.
- **F98.3 Pip-Boy equip toggle and hotkeys.** Items view equips/unequips
  eligible rows (Weapons/Apparel/Ammo) with an equipped marker; hotkeys
  1–8 assignable from the Pip-Boy and usable outside it (`bindings.rs`
  upgrades `UnsupportedAction::Hotkey` to a real action).
- **F98.4 Persistence and console staging.** Equipped set and hotkey
  bindings persist in the save: `CURRENT_SAVE_FORMAT_VERSION` bumps to 3
  with v1/v2 load compatibility. Console gains `player.equipitem
  <FormID>` with deterministic result lines.

### #99 — consumable use and reading

- **F99.1 Pure use/read rules module.** New std/serde-only
  `src/viewer/interaction/item_use.rs`: Aid is usable, Book/Note with
  text is readable, Key/Misc are inert; using a consumable removes
  exactly one from its stack; quest-item interplay per #81 (readable,
  never consumed away while flagged).
- **F99.2 Consumable use.** Using an Aid item from the Pip-Boy Items view
  decrements the stack, plays authored use audio when present, and posts
  a notice listing the prepared effect labels (magnitudes are
  later-milestone scope).
- **F99.3 Reader overlay.** New `src/viewer/pipboy_reader.rs` renders
  BOOK/NOTE text from `PreparedItemStats` in an overlay opened from the
  Items view; closing returns to the Pip-Boy. Books with skill flags log
  a deterministic placeholder line. Public seam: a
  `ReaderRequest`-style API that #100 can invoke.

### #100 — initial Pip-Boy Data views

- **F100.1 Data tab.** Pip-Boy gains a Data tab beside Items using the
  existing tab-switch interaction pattern.
- **F100.2 Notes view.** Lists readable stacks (per #99 rules) from the
  authoritative inventory; selecting one opens the #99 reader.
- **F100.3 World view.** Read-only session info already available at
  runtime: current cell name/form id and play stats. No map, quests, or
  radio in M3.

## Order inside each issue

Fix the feature list → write tests → implement until green:

- #98: cucumber `features/equipment.feature` + steps appended to
  `tests/features.rs` (World fields at end of struct, delimited step
  section at end of file); Bevy-side Pip-Boy/console/save behavior gets
  unit tests in the existing harnesses.
- #99: cucumber `features/item_use.feature` + steps appended the same
  way; reader open/close and use-path unit tests in the Pip-Boy harness.
- #100: pure view-model selection unit tests; tab/row activation tests in
  the Pip-Boy harness (no cucumber feature — Bevy-side behavior, wave 4
  precedent).

## File ownership

- #98: `openmw_esm4/records.rs`, `prepare/items.rs`, manifest item
  fields, `player/equipment.rs` (new), `bindings.rs`, `save/`,
  `console.rs`, `features/equipment.feature`.
- #99: `interaction/item_use.rs` (new), `pipboy_reader.rs` (new),
  `features/item_use.feature`, its own `console.rs`/audio touches only if
  needed.
- Shared hotspot: `src/viewer/pipboy.rs` (#98 equip toggle, #99 use/read
  actions, #100 Data tab). #98/#99 edit it concurrently; the orchestrator
  resolves the merge. #100 edits it after both land.
- Merge seam: `tests/features.rs` end-of-struct / end-of-file appends.

## Gates and acceptance

`cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`,
`cargo test`, then agent-bridge acceptance on a real prepared cell
(e.g. `000151e3`): additem armor/weapon/ammo → equip via console and
Pip-Boy → save/load retains equipment and hotkeys; additem consumable and
note → use drops the stack with a notice → note opens in the reader;
Data → Notes lists it and World matches `bevyout.session`. Results are
commented on each issue.

## Shipped amendments

(none yet — added only if acceptance testing forces changes)
