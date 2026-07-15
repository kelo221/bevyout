# M3 Wave 2 — Leveled Loot, Container Transfer, Container Persistence

Epic: [#7 — M3 Inventory, equipment, and Pip-Boy foundation](https://github.com/kelo221/bevyout/issues/7)
Kickoff: [M3_WAVE2_PROMPT.md](M3_WAVE2_PROMPT.md)

Goal: the loot pipeline behind gate #8 — a container's leveled entries
resolve deterministically on first open (#74), the player moves items both
ways through a real transfer screen (#75), and the container's stacks plus
its resolved-leveled marker survive save/restart/reload without re-rolling
(#76).

| Issue | Scope | Executor |
|-------|-------|----------|
| [#74](https://github.com/kelo221/bevyout/issues/74) leveled-list records + deterministic resolver | importer/manifest/pure resolver only; no UI | agent A (Sonnet, worktree) |
| [#75](https://github.com/kelo221/bevyout/issues/75) container transfer interface | runtime container store + transfer UI; consumes A's resolver via the fixed seam below | agent B (Sonnet, worktree) |
| [#76](https://github.com/kelo221/bevyout/issues/76) persist container state + leveled results | save subrecord + persist capture/apply | agent C (Sonnet, worktree) |

**Ordering rule inside every issue (repo convention): feature list fixed
below → Cucumber feature + tests written first → implementation makes them
green.**

## Wave-1 coexistence

#70/#71/#72 (kelo221) are in flight and unmerged. This wave touches their
seams at exactly two points, both designed to swap without rework:

- **Player inventory** (#71): B routes all player-side mutation through the
  existing `PlayerInventory` count map behind one small `pub(crate)` API
  (`grant`, `remove`); when #71 replaces the count map with authoritative
  stacks, only that API's body changes.
- **Save format version** (#72 introduces v2): C adds its subrecords
  additively and does **not** bump `CURRENT_SAVE_FORMAT_VERSION` itself; the
  orchestrator applies the single bump at integration with whichever PR
  lands second.

## Fixed seams (agents code against these signatures, stubbing if needed)

- Manifest (A owns, appended with serde defaults for compatibility):
  `PreparedSceneManifest.leveled_lists: BTreeMap<u32, PreparedLeveledList>`;
  `PreparedLeveledList { chance_none: u8, flags: u8, entries: Vec<PreparedLeveledEntry> }`;
  `PreparedLeveledEntry { level: u16, base_form_id: u32, count: i32 }`.
- Resolver (A owns, pure, std/serde-only):
  `resolve_leveled(list_form_id: u32, lists: &BTreeMap<u32, PreparedLeveledList>, seed: LeveledSeed, player_level: u16) -> Vec<(u32, i32)>`
  with `LeveledSeed::derive(playthrough_seed: u64, cell_form_id: u32, reference_form_id: u32)`
  (splitmix64-style mixing). Deterministic, cycle-safe, chance-none honored.
- Runtime container store (B owns, resource in the interaction slice):
  `ContainerStates(HashMap<u32 /* reference form id */, ContainerState>)`;
  `ContainerState { stacks: Vec<(u32, i32)>, resolved: bool }`.
- Persistence (C owns): container stacks reuse the existing
  `PersistentReferenceDelta.inventory: Option<Vec<ItemStack>>` field; a new
  boolean `leveled_resolved` marker subrecord (`REFR.LVLR`) records that the
  first-open roll happened, so an emptied container is distinguishable from
  an unopened one.
- Playthrough seed: the existing `SaveGame.rng_state: u64`.

## File-ownership boundaries

Shared merge seam: `tests/features.rs` — append-only World fields at the end
of the struct, one delimited step section at the end of the file, one new
`features/*.feature` per issue. Cucumber-driven modules must stay
std/serde-only (no Bevy imports; `#[path]` inclusion).

- **A (#74):** `src/vsa/openmw_esm4/records.rs` (+ its tests) for
  LVLI/LVLN/LVLC parsing (LVLO entries, LVLD chance-none, LVLF flags),
  `src/vsa/manifest/mod.rs` (append-only new types),
  `src/vsa/prepare/placements.rs` (collect referenced lists into the
  manifest), new pure `src/viewer/interaction/leveled.rs`,
  `features/leveled_lists.feature`.
- **B (#75):** `src/viewer/interaction.rs` (container branch + the
  `PlayerInventory` API), new pure
  `src/viewer/interaction/container_policy.rs`, new
  `src/viewer/interaction/transfer_ui.rs`,
  `features/container_transfer.feature`. Stubs A's resolver signature
  locally if #74 is not yet merged in its worktree.
- **C (#76):** `src/save/mod.rs` (LVLR subrecord; inventory field already
  exists), `src/viewer/world/persist_policy.rs` + `persist.rs` (capture
  `ContainerStates` into deltas, apply deltas back on load — behind a small
  local type so B's resource wires in at integration),
  `features/container_persistence.feature`.

No agent touches `src/main.rs`, another agent's files, or wave-1 surfaces
(`src/vsa/catalog.rs`-adjacent item-catalog work, Pip-Boy UI, dropped-item
spawning).

## Issue #74 — Leveled-list records and deterministic resolution

Today: `prepare/placements.rs` marks container inventory entries
`leveled: true` when the base record is LVLI/LVLN/LVLC and drops the list
body; nothing can resolve them.

### Feature list

- **F74.1** ESM4 importer parses LVLI/LVLN/LVLC: LVLO entries
  (level, form id, count), LVLD chance-none, LVLF flags, per OpenMW-derived
  layouts (ported code goes in the isolated OpenMW area per `AGENTS.md`).
- **F74.2** Manifest carries `leveled_lists` for every list reachable from a
  prepared container inventory (transitively, nested lists included);
  old manifests without the field still deserialize.
- **F74.3** Pure resolver per the fixed seam: deterministic for identical
  (seed, lists, level); nested lists recurse; cycles terminate safely with a
  warn-and-skip; chance-none rolls per entry-set semantics; the
  "use-all" / "each" flag variants honored.
- **F74.4** `LeveledSeed::derive` mixes playthrough seed + cell + reference
  so distinct references roll independently and identically across runs.
- **F74.5** `features/leveled_lists.feature` covers F74.3–F74.4.

### Tests before code

- **T74.1** Synthetic ESM4 fixture bytes for flat, nested, chance-none=100,
  and cyclic lists; parser unit tests.
- **T74.2** Resolver determinism: same inputs → identical output, twice;
  different reference form id → independent streams.
- **T74.3** Manifest round-trip with and without `leveled_lists`.

## Issue #75 — Container open and world-loot transfer interface

Today: activating a container plays audio and shows a one-line
`inventory_summary` notice (`interaction.rs`); no items move.

### Feature list

- **F75.1** Pure `container_policy.rs`: seed-from-manifest (non-leveled
  entries become stacks; leveled entries resolve once via the resolver seam
  on first open, setting `resolved`), transfer operations (take one, take
  stack, take all, store one/stack) with conservation guaranteed.
- **F75.2** `ContainerStates` resource per the fixed seam; container
  activation opens a paused modal transfer screen (existing
  `GameplayModal` pattern) listing container stacks and player stacks;
  prepared open/close sounds play on open/close.
- **F75.3** Transfer targets `PlayerInventory` only through the new
  `grant`/`remove` API (the #71 swap point).
- **F75.4** Modal behavior matches the repo pattern: gameplay input blocked,
  cursor released, state restored on close; Esc closes.
- **F75.5** `features/container_transfer.feature` covers F75.1.

Non-goals: corpse containers and barter (no actors/economy yet — noted on
the epic, checklist item stays unticked); item art/stats in the transfer
list beyond name + count (that is #70/#71 catalog territory).

### Tests before code

- **T75.1** Policy: every transfer op conserves total counts; take-all
  empties; store into empty container; zero/negative counts rejected.
- **T75.2** First open resolves leveled entries exactly once; reopen does
  not re-roll (`resolved` short-circuits).
- **T75.3** Minimal Bevy `App`: activate opens modal + pauses, Esc closes +
  restores, input blocked while open.

## Issue #76 — Persist container state and resolved leveled results

Today: `PersistentReferenceDelta.inventory` exists in save v1 but is dead —
`persist_policy.rs` explicitly leaves `lock_level`/`inventory` out of its
capture/apply seam; nothing records whether a leveled roll happened.

### Feature list

- **F76.1** Save format: new `LVLR` boolean subrecord on reference deltas
  (resolved-leveled marker); container stacks ride the existing `inventory`
  field. Additive only — no version bump in this issue (see wave-1
  coexistence).
- **F76.2** Capture: departing-cell snapshot includes, per container
  reference, current stacks + `resolved` when they differ from the manifest
  baseline (unchanged containers produce no delta).
- **F76.3** Apply: on cell load, deltas rebuild container state (stacks +
  `resolved`) before first activation; an unopened container (no delta)
  still rolls on first open.
- **F76.4** v1 saves without these subrecords load unchanged; malformed
  container payloads fail the load safely (existing error path), not
  silently corrupt.
- **F76.5** `features/container_persistence.feature` covers the
  capture/apply policy.

### Tests before code

- **T76.1** Round trip: delta with stacks + LVLR survives encode/decode and
  re-encodes byte-identically.
- **T76.2** Capture diffing: manifest-identical container → no delta;
  looted container → minimal delta; resolved-but-untouched-counts container
  → LVLR-only delta.
- **T76.3** Backward compatibility: v1 fixture without LVLR loads; truncated
  inventory payload errors.

## Integration (orchestrator)

Branch `m3-wave2`. Merge A → B → C, wire B's `ContainerStates` into C's
capture/apply and A's resolver into B's first-open path, apply the save
version reconciliation with #72, then gates: `cargo fmt --check`,
`cargo clippy --all-targets -- -D warnings`, `cargo test`, representative
`cargo run-dev -- prepare` + `view` with BRP evidence (open container →
loot → store → save → restart → reload → exact counts, two-run determinism
of a first-open roll). Measured results go as comments on #74–#76; one PR
closes all three.

## Shipped amendments

(amended during acceptance, not rewritten)
