# M1 First Wave — Grand Plan

Epic: [#3 — M1 Runtime foundation, mutability, and persistence](https://github.com/kelo221/bevyout/issues/3)

Issue #3 defines four **first-wave child issues with no blockers**. All four are planned
here and executed in parallel by one agent per issue, each in an isolated git worktree:

| Issue | Branch | Slice touched | Conflict surface |
|-------|--------|---------------|------------------|
| #35 Application states | `codex/m1-35-application-states` | new `src/app_state/`, `src/viewer/` | `viewer/app.rs`, `lib.rs` |
| #39 ContentIndex | `codex/m1-39-content-index` | new `src/vsa/content_index/` | `vsa/mod.rs`, `cli.rs` |
| #38 Runtime mutability | `codex/m1-38-runtime-mutability` | `src/vsa/prepare/`, `src/vsa/manifest/` | manifest schema, `prepare/placements.rs` |
| #37 Compatibility reports | `codex/m1-37-compatibility-reports` | new `src/vsa/report/` | `cli.rs`, `main.rs` dispatch |

Later checklist items in #3 (PersistentWorldState, SaveGame, atomic saves, deterministic
time/RNG, batching rules, QA fixtures) **depend on these four** (stable FormIDs from #39,
mutability classes from #38, states from #35) and are deliberately NOT in this wave.
The epic itself says later branches start only when dependencies are ready.

**Ordering rule inside every issue: feature list is fixed below → tests are written
first → implementation makes them green.** No implementation before its tests exist.

**Base commit:** current local `master` (`1fd93de`). The epic names `9e7ad6c` on
`origin/master`, but local master is ahead (physics, interpolation); branching from an
older commit would just create guaranteed conflicts with work already done.

---

## Issue #35 — Explicit application and gameplay states

### Feature list

- **F35.1** `AppState` top-level Bevy `States` enum: `Boot`, `Loading`, `MainMenu`, `InGame`.
- **F35.2** `GameplayModal` sub-state (exists only while `AppState::InGame`): `None`, `Paused`, `Dialogue`, `PipBoy`. Modals are mutually exclusive by construction.
- **F35.3** Transition request/validation: a `RequestStateTransition` message plus a legal-transition table. Legal: Boot→Loading, Loading→MainMenu, Loading→InGame, MainMenu→Loading, InGame→MainMenu (quit-to-menu), InGame modal enter/exit `None↔Paused`, `None↔Dialogue`, `None↔PipBoy`. Everything else is rejected.
- **F35.4** Invalid transitions fail safely: rejected request logs a `warn!` diagnostic naming from/to states; app state is unchanged.
- **F35.5** A narrow `AppStatePlugin` in a new VSA slice `src/app_state/` — state types, transition validation, and gating live there; `main.rs` stays a dispatcher.
- **F35.6** Existing view/render flow routed through states: manifest loading work happens in `Loading` (OnEnter), scene spawn happens on `OnEnter(InGame)` instead of `Startup`. Current CLI behavior preserved: `view`/`render` auto-advance Boot→Loading→InGame with no menu stop.
- **F35.7** System gating: player movement, camera, interaction, footsteps run only `in_state(AppState::InGame)` + `in_state(GameplayModal::None)`. `Paused` pauses `Time<Virtual>`; modal exit resumes it. No duplicate input/simulation/presentation systems across states.
- **F35.8** Modal input: `Esc` toggles Paused, `Tab` toggles PipBoy (placeholder — no UI content required), Dialogue entered/exited via the transition message only (no UI yet).
- **F35.9** `LoadingTarget` resource (`NewGame { manifest }` now; `Continue`/`LoadSave` variants reserved) — the clear Loading entry point issue #35 requires for later save flows.
- **F35.10** Deterministic restart: launching twice against the same prepared manifest produces the identical initial state (same AppState path, same spawn set).
- **F35.11** `MainMenu` is a placeholder state (empty screen + log) — reachable in the state graph, skipped by CLI. No menu UI in this issue (YAGNI; later milestone owns menus).

### Test list (write these before implementing)

- **T35.1** Every legal transition in the F35.3 table applied on a headless `App` (MinimalPlugins + StatesPlugin) lands in the expected state.
- **T35.2** Representative illegal transitions (Boot→InGame, Loading→Paused, Paused→Dialogue, MainMenu→PipBoy) are rejected: state unchanged after update.
- **T35.3** Modal round trip: InGame→Paused→InGame→PipBoy→InGame→Dialogue→InGame; a counter system gated on `GameplayModal::None` does not tick while any modal is active, and ticks again after exit.
- **T35.4** No duplicate execution: an input-shaped probe system increments exactly once per frame across a full modal round trip.
- **T35.5** Paused pauses `Time<Virtual>`; exiting resumes it.
- **T35.6** Synthetic scenario: Boot→Loading→InGame with a fake `LoadingTarget`; OnEnter(InGame) spawn hook fired exactly once.
- **T35.7** Determinism: two fresh apps driven through Boot→Loading→InGame report identical initial state (state value + spawn-hook count).

### Implementation steps

1. Create `src/app_state/mod.rs` + `plugin.rs` slice: enums, `RequestStateTransition`, validation system, plugin. Register in `lib.rs` as `mod app_state;` (exposed to viewer only).
2. Write T35.1–T35.7 as `#[cfg(test)]` in the slice (headless apps, no render). They fail/don't compile → stub types until they compile and fail meaningfully.
3. Implement enums + validation until T35.1–T35.5 pass.
4. Wire `viewer/app.rs`: add plugin, move `spawn_prepared_scene`/`capture_cursor`/`spawn_reticle` to `OnEnter(InGame)`, insert `LoadingTarget`, add auto-advance systems for Boot/Loading, gate the Update system sets (F35.7). T35.6–T35.7 pass.
5. Add Esc/Tab modal input systems in the slice.
6. `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test`.

---

## Issue #39 — Load-order-wide ContentIndex

### Feature list

- **F39.1** Load-order resolution: configured plugin list → resolved order with master-before-dependent validation.
- **F39.2** Single pass over each plugin building one index keyed by **resolved FormID** (master-index remapping applied), reusing the existing `vsa/openmw_esm4` reader.
- **F39.3** Winning-override semantics: last plugin in load order wins; full provenance retained (every source plugin per FormID + the winner).
- **F39.4** Per-record metadata: record type, EditorID where present, source plugin.
- **F39.5** Narrow query API on `ContentIndex`: `get(FormId)`, `records_of_type(...)`, `cells()`, `by_editor_id(...)`. No parser types leak out of the slice.
- **F39.6** Diagnostics: missing masters, invalid ordering, duplicate plugin entries, unsupported record types — each an actionable error/count, failing before preparation.
- **F39.7** Load-order fingerprint: stable hash (sha2, already a dependency) over plugin names + versions/sizes, exposed for later save-compatibility checks.
- **F39.8** Deterministic index summary (counts per record type per plugin, sorted) printable for regression comparison.
- Skipped: integrating the index into the `prepare` path — that rewires cell selection and belongs to a follow-up once #38 merges; the query API is the contract.

### Test list (write these before implementing)

- **T39.1** Hermetic synthetic two-plugin fixture (master + dependent): index contains records from both, resolved FormIDs remapped correctly.
- **T39.2** Override fixture: dependent overrides a master record → winner is dependent, provenance lists both.
- **T39.3** Missing master → error naming the missing master and the requiring plugin.
- **T39.4** Invalid order (dependent before master) → error naming both plugins.
- **T39.5** Duplicate plugin entry → error.
- **T39.6** EditorID and record-type queries return expected records.
- **T39.7** Fingerprint is stable across two builds of the same fixture set, and changes when a plugin changes.
- **T39.8** Summary snapshot: byte-identical across two runs.

Fixtures are built by a small in-test ESM byte-writer (synthetic, no Bethesda data),
mirroring how existing `openmw_esm4` tests construct records if such helpers exist —
check `src/vsa/openmw_esm4/` first and reuse.

### Implementation steps

1. Check what `vsa/openmw_esm4/reader.rs` + `esplugin` already give (load order, masters) — reuse, don't reimplement.
2. New module `src/vsa/content_index/` (mod.rs, builder, diagnostics). Write T39.1–T39.8 with the synthetic fixture writer.
3. Implement builder + queries until green.
4. Wire an internal `pub(crate)` constructor from the existing config/plugin paths. No CLI change required.
5. fmt / clippy / test.

---

## Issue #38 — PreparedRuntimeMutability classification

### Feature list

- **F38.1** `PreparedRuntimeMutability` enum on every prepared placement: `Immutable`, `EnableGroup`, `ScriptAddressable`, `Unknown`.
- **F38.2** `PreparedSceneManifest` schema version bump. Old cached manifests fail compatibility with a precise "re-run prepare for <cell>" instruction (extend the existing `ensure_prepared_manifest_compatible` path — the mechanism already exists).
- **F38.3** Conservative classification: enable-parent chain (reuse `openmw_esm4/enable.rs`) → `EnableGroup` with its root FormID preserved on the placement; known script-referenced records → `ScriptAddressable`; anything uncertain → `Unknown`, never silently `Immutable`.
- **F38.4** Classification counts logged at prepare time and stored in the manifest for QA (`immutable: N, enable_group: N, ...`).
- **F38.5** Stable reference identity: each classified placement keeps its resolved FormID so PersistentWorldState can address it later.

### Test list (write these before implementing)

- **T38.1** Synthetic record fixtures producing each of the four classifications; assert one placement of each class after prepare-time classification.
- **T38.2** Enable-parent fixture: children share the parent root FormID in their `EnableGroup` classification.
- **T38.3** Ambiguous fixture (record type not in the known-safe set) → `Unknown`.
- **T38.4** Golden schema test: serialize a minimal manifest, assert schema version + field presence (RON round-trip).
- **T38.5** Old-version manifest (previous schema number) → compatibility error message contains the regeneration instruction.
- **T38.6** Determinism: classifying the same fixture set twice yields identical results.

### Implementation steps

1. Read `vsa/manifest/` + `vsa/prepare/placements.rs` + `openmw_esm4/enable.rs` to find the schema version constant and enable-parent data already available.
2. Write T38.1–T38.6 (fixtures at the placement/record level, no real game data).
3. Add the enum + field + schema bump + compat error; implement classification in the prepare placement path.
4. fmt / clippy / test. Real-data check on Super-Duper Mart is a **manual step for Simon** (licensed data), listed in the PR description.

---

## Issue #37 — Compatibility reports

### Feature list

- **F37.1** Report schema (serde JSON, already a dependency): entries with `class` (record/subrecord/condition/script-function/asset-format/quest), `key`, `status` (`Supported`/`Partial`/`Unsupported`/`IgnoredByDesign`/`Unknown`), `provenance`, and `save_affecting: bool`.
- **F37.2** Human-readable summary (plain text, sorted counts per class/status) alongside the machine report.
- **F37.3** Deterministic output: fully sorted, stable field order → diffable; two runs on the same input are byte-identical.
- **F37.4** Support registry: a declared table of what bevyout currently supports (record types the importer handles → `Supported`/`Partial`; everything encountered but undeclared → `Unknown`, never `Supported` by default).
- **F37.5** New CLI subcommand `report` in a tooling slice `src/vsa/report/`, taking plugin inputs like `prepare` does; `main.rs` gains only a dispatch arm.
- **F37.6** Licensed-data hygiene: reports derived from real ESMs are written under `.bevyout/` (gitignored); committed fixtures/goldens are synthetic only.
- **F37.7** Unknown/unsupported entries that touch persistent state are flagged `save_affecting` so save-compat risk is explicit (F37.1 field, populated from a small known list; conservative default `true` for unknown record types that carry runtime state).
- Skipped: quest-matrix generation and load-order-wide execution against the ContentIndex — the report consumes what the current reader parses per plugin; switching its input to `ContentIndex` is a one-line follow-up after #39 merges.

### Test list (write these before implementing)

- **T37.1** Synthetic ESM fixture → report JSON snapshot test (golden file, synthetic content only).
- **T37.2** Every status variant appears via crafted fixtures (supported record, undeclared record → Unknown, declared-ignored → IgnoredByDesign, partial, unsupported).
- **T37.3** Determinism: generate twice, assert byte equality.
- **T37.4** Unknown is never counted as Supported in summary totals.
- **T37.5** `save_affecting` set for a fixture record class known to carry state.
- **T37.6** CLI contract test (extend `tests/cli_contract.rs`): `report` subcommand parses.

### Implementation steps

1. Write T37.1–T37.6 with synthetic fixtures (reuse/extend the fixture byte-writer approach; if #39's writer lands first, fine, but don't block on it — each worktree is independent).
2. Implement schema types + support registry + generator walking parsed records.
3. Add `Report` variant to `cli.rs` + dispatch in `main.rs` + `lib.rs` export.
4. fmt / clippy / test.

---

## Swarm execution plan

**Wave 1 — 4 Sonnet agents in parallel**, one per issue, each in an isolated git
worktree branched from local `master`, branch names per the table above. Each agent:

1. Reads `AGENTS.md` and its issue section in this file (absolute path
   `/Users/simon/projects/bevyout/M1_PLAN.md`).
2. Writes the tests from its test list **first**, then implements until green.
3. Sets `CARGO_TARGET_DIR=/Users/simon/projects/bevyout/target` so all four worktrees
   share the already-compiled Bevy dependency cache (concurrent builds serialize on the
   cargo lock — acceptable; four independent 20 GB debug targets are not).
4. Finishes with `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`,
   `cargo test`, then commits on its branch. No pushes.

**Wave 2 — integration (orchestrator, after all four report done):** merge into
`master` in the order **#35 → #39 → #38 → #37** (viewer-only first, then import slices;
#37 last because `cli.rs`/`main.rs` conflicts are cheapest to resolve at the end).
Re-run the full check suite after each merge.

**Manual verification for Simon (licensed data, real window):**
- `cargo run-dev -- render <selector>` — states route Boot→Loading→InGame, Esc/Tab modals work.
- #38: prepare Super-Duper Mart, eyeball classification counts.
- #37/#39: run report/index against local `Fallout3.esm`; confirm nothing
  Bethesda-derived lands outside `.bevyout/`.
