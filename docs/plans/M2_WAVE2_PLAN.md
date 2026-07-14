# M2 Wave 2 — Instant Cell Transitions

Epic: [#5 — M2 Bulk preparation and connected interiors](https://github.com/kelo221/bevyout/issues/5)

Goal: the player enters a **batch-prepared** neighbor cell through a door with no
loading screen. Instant = preloaded; preloading is driven by the cell map's door
graph (`cellmap.ron`, #45). Unprepared destinations always take the loading-screen
fallback (epic #5 runtime policy) — Blender conversion is never inline.

| Issue | Scope | Wave |
|-------|-------|------|
| [#47](https://github.com/kelo221/bevyout/issues/47) Batch session | parse chain/BSA once, shared caches, cellmap into cache | **2a** |
| [#51](https://github.com/kelo221/bevyout/issues/51) Runtime cell map + predictive preloader | resource, policy state machine, background loads, LRU eviction | **2a** |
| [#48](https://github.com/kelo221/bevyout/issues/48) Resumable prepare | job manifest, bounded workers, retry-failed | **2b** |
| [#52](https://github.com/kelo221/bevyout/issues/52) Instant swap + fallback | same-frame root swap, teleport, loading-UI fallback | **2b** |

**Ordering rule inside every issue (repo convention): feature list fixed below →
Cucumber feature + tests written first → implementation makes them green.**

Execution: one Sonnet agent per issue in an isolated git worktree
(`.claude/worktrees/`), branches `m2-47-batch-session`, `m2-51-preloader` (wave 2a),
then `m2-48-resumable`, `m2-52-instant-swap` (wave 2b) — 2b branches start from the
`m2-wave2` integration branch after 2a merges. Orchestrator merges, runs
`cargo fmt --check`, `clippy --all-targets -- -D warnings`, `cargo test`, then
batch-prepares the Vault 101 corpus and verifies the live viewer over BRP.

File-ownership boundaries (no agent touches another's files; the shared seam is
`tests/features.rs`, where each agent appends only its own clearly-delimited
section, and `features/*.feature`, one new file per issue):

- #47: `src/vsa/prepare/**` (new `session.rs`), `src/vsa/catalog.rs` (reuse map
  builder), `features/batch_session.feature`.
- #51: `src/viewer/world/**` (new), `src/viewer/scene.rs`, `src/viewer/app.rs`,
  `src/viewer/mod.rs`, `src/config.rs`, `config.example.toml`,
  `features/preload_policy.feature`.
- #48: `src/vsa/prepare/**` (new `jobs.rs`), `src/cli.rs`,
  `features/resumable_prepare.feature`.
- #52: `src/viewer/world/swap.rs` (new), `src/viewer/interaction.rs`,
  `src/viewer/scene.rs`, `features/instant_swap.feature`.

---

## Issue #47 — Batch session

Today `prepare_batch` calls `prepare_one` per cell, and every call re-reads the
plugin chain from disk, re-runs esplugin validation, re-parses the full content
set, and re-indexes every BSA plus the audio archives and footstep staging.
OpenMW prior art (isolated-port rule applies if code is ported):
`apps/openmw/mwworld/cellpreloader.cpp` shares one resource system per session.

### Feature list

- **F47.1** `BatchSession` (`src/vsa/prepare/session.rs`) built once per batch:
  loaded plugin chain bytes + fingerprint, esplugin validation result, BSA
  archive indexes, audio archive indexes, staged footstep sets. `prepare_one`
  becomes `prepare_cell(&mut BatchSession, …)`; the single-cell CLI path builds
  a one-cell session (behavior unchanged).
- **F47.2** No per-cell re-reading: plugin bytes read once, BSA/audio indexes
  built once, footsteps staged once per batch.
- **F47.3** Shared per-batch caches with hit/miss counters: physics sidecar
  reads and the existing content-addressed asset cache decisions aggregate
  across cells; one end-of-batch summary line
  (`batch cache: assets reused N, built M, physics hits P, …`).
- **F47.4** A successful batch run writes `cellmap.ron` into the cache dir
  (reusing the `cells --map` builder in `catalog.rs` — the content set is
  already parsed). This is the artifact the runtime preloader (#51) reads.
- **F47.5** `features/batch_session.feature` + steps: session-level cache
  accounting is pure and deterministic (second occurrence of a key is a hit).

### Tests before code

- **T47.1** Batch of two cells sharing an asset key: second occurrence is a
  cache hit; counters say so (pure seam, synthetic keys).
- **T47.2** Session construction happens once for N cells (e.g. a
  load-counter on the session shows 1 chain load, 1 archive index).
- **T47.3** Batch run drops `cellmap.ron` next to `scenes/` in the cache dir.
- **T47.4** Cucumber scenarios for T47.1 shapes.

## Issue #51 — Runtime cell map + predictive preloader

### Design

New viewer slice `src/viewer/world/`:

- `policy.rs` — **pure, std-only** (includable verbatim by `tests/features.rs`
  like `cell_map.rs`): door-graph adjacency from `CellMap`, BFS graph distance,
  and `plan(active, resident, prepared, budget) -> PreloadPlan { load, evict }`.
  Desired set = active cell + 1-hop door neighbors that have a prepared
  manifest. Evict farthest-by-graph-distance first when over the resident
  budget; the active cell is never evicted.
- `preload.rs` — Bevy systems: `CellMapIndex` resource loaded at startup from
  `<asset_root>/cellmap.ron` (`asset_root` in the manifest *is* the cache dir;
  missing file ⇒ preloader inert with one warning), `ActiveCell(u32)` resource
  seeded from the startup manifest, `ResidentCells` map
  `form_id -> { root: Entity, state: Loading | Ready }`. On `ActiveCell`
  change, run the policy; background-parse neighbor `scene.ron` files on
  `AsyncComputeTaskPool`; spawn each preloaded cell's placements under one
  hidden per-cell root (`Visibility::Hidden`); GLB handles load through the
  existing `AssetServer`. Physics sidecar data is loaded and stored with the
  resident cell, but collider entities are #52's job on activation.
- `spawn_prepared_scene` in `scene.rs` is refactored so per-cell content
  (placements, lights) spawns under a per-cell root entity via a function the
  preloader reuses; camera and global resources stay where they are.

Config knob: `[world] resident_cell_limit` (default 4) in `config.toml` /
`src/config.rs`. `ponytail:` cell-count budget, not bytes — upgrade to a byte
budget when residency cost is actually measurable.

Logging (viewer = tracing): `preload start <formid>`, `preload ready <formid>
(<placements> placements)`, `preload evict <formid> (graph distance <d>)`.

### Feature list

- **F51.1** `CellMapIndex` resource from `<asset_root>/cellmap.ron`; inert+warn
  when absent.
- **F51.2** Pure preload policy: desired set, prepared-only filter, LRU/BFS
  eviction, never-evict-active.
- **F51.3** Background manifest parse + hidden per-cell root spawn; `Ready`
  when spawned and all scene handles are loaded.
- **F51.4** `resident_cell_limit` config knob.
- **F51.5** Preload start/ready/evict log lines.
- **F51.6** `features/preload_policy.feature` + steps against `policy.rs`.

### Tests before code

- **T51.1** Active cell with two door neighbors, both prepared ⇒ plan loads
  both; unprepared neighbor is never planned.
- **T51.2** Budget 2, resident {A,B,C} with C farthest by graph ⇒ evict C;
  active cell never evicted.
- **T51.3** Re-entering a resident cell produces an empty plan (idempotent).
- **T51.4** Cucumber scenarios for T51.1–T51.3.

## Issue #48 — Resumable prepare

### Feature list

- **F48.1** Job manifest `<cache_dir>/prepare_jobs.ron` (pure, std-only module
  `src/vsa/prepare/jobs.rs`): per-cell status pending/done/failed(reason),
  keyed by content fingerprint; stale fingerprint ⇒ fresh manifest.
- **F48.2** Interrupted runs resume: cells already `done` for the same
  fingerprint are skipped (unless `--force`).
- **F48.3** `--retry-failed` reruns only failures; every batch ends with a
  failure summary (`N done, M failed: <ids and reasons>`).
- **F48.4** Bounded worker pool, `--jobs N` (default: available parallelism)
  running `prepare_cell` against the shared `BatchSession` (#47); the job
  manifest is written through after every cell so interruption is safe.
- **F48.5** `features/resumable_prepare.feature` + steps against `jobs.rs`.

### Tests before code

- **T48.1** Status transitions pending→done / pending→failed persist and
  reload byte-identically.
- **T48.2** Resume skips `done`, keeps `failed`; `--retry-failed` selects
  exactly the failed set.
- **T48.3** Fingerprint change invalidates the manifest.
- **T48.4** Cucumber scenarios for T48.1–T48.3.

## Issue #52 — Instant swap + fallback

### Feature list

- **F52.1** Pure swap-eligibility seam (std-only, in `world/policy.rs` or
  `swap.rs`): door destination × residency state ⇒
  `Instant | Fallback | ReturnToSource`.
- **F52.2** Door activation with a `Ready` resident destination: same-frame
  swap — source root hidden (stays resident), destination root visible,
  player teleported to the XTEL translation/facing already stored in
  `PreparedDoorDestination`, collider entities built from the preloaded
  physics data staggered across frames, `ActiveCell` updated (preloader
  replans), manifest-dependent resources (fog/lighting/audio) repointed.
- **F52.3** Persistent + enable-parent state applied from the save layer (M1)
  **on activation**, not preload, through a pure function over
  `PersistentCellState` + spawned placements, so resident copies can't stale.
- **F52.4** Non-resident destination ⇒ fade + loading UI fallback (never
  inline Blender); load failure returns the player to the source cell.
- **F52.5** Swap telemetry: log `swap <src>-><dst> instant max_frame_ms=<x>`
  from the frame diagnostics; acceptance is no frame > 33 ms during a
  preloaded swap.
- **F52.6** `features/instant_swap.feature` + steps: eligibility + fallback
  selection scenarios.

### Tests before code

- **T52.1** Ready resident ⇒ Instant; Loading or absent ⇒ Fallback; missing
  manifest on disk ⇒ Fallback; fallback failure ⇒ ReturnToSource.
- **T52.2** Save-layer application marks disabled/deleted placements hidden
  and applies transforms (pure, synthetic `PersistentCellState`).
- **T52.3** Cucumber scenarios for T52.1–T52.2.

---

## Shipped amendments (found during real-data acceptance)

The feature lists above were implemented as specified; acceptance against the
live viewer added the following, all in PR #54:

- **A1 — `activate <reference>` console command** (`src/viewer/console.rs`):
  the BRP bridge had no way to drive a door (no activate/travel command
  existed), so scripted travel writes the same `DoorTravelRequested` message
  as the player's Enter activation, bypassing focus/distance/lock checks.
  Unit tests cover arity, non-door, no-destination, and the written message.
- **A2 — staggered preload spawning** (amends F51.3): one-shot spawning of
  Vault101d's 1,371 placements cost a 130 ms frame; parse completion now
  queues placements and a system drains 128 raw entries per frame
  (`PRELOAD_SPAWN_BUDGET_PER_FRAME`); `Ready` waits for the queue.
- **A3 — physics sidecars read inside the preload task** (amends F51.3/F52.2):
  the swap-window collider build was doing first-time sidecar file I/O on the
  main thread; the background task now reads/parses them and merges into
  `PreparedPhysicsAssets`, with the lazy read kept as fallback.
- **A4 — rejected experiment, kept for the record**: stashing preloaded cells
  visible below the world to pre-warm rendering (commit `4da3b13`, reverted in
  `51d4d68`) — frustum culling keeps off-screen content out of the render
  queue, so it warms nothing and regresses steady-state frame times.
- **A5 — V camera-toggle binding (bug #56)**: the controls message always
  advertised V, but only the console `tfc` command was wired; the key is now
  bound (gameplay-only). Found by the user pressing V and nothing happening.

Measured outcome (cool machine, chain Vault101a→b→d→b→a over BRP): 4/4
instant swaps, zero loading screens; swap-window max frames 34.3 / 84.0 /
24.8 / 32.7 ms. Revisits meet the ≤33 ms bar; the first-ever reveal of the
largest cell exceeds it (one-time first-render cost) — follow-up issue #55.
`capture_viewport` yields black PNGs under the occluded-window automated
setup on macOS; snapshots + logs are the recorded evidence.

## Orchestrator: real-data acceptance (after 2b merges)

1. `cargo run-dev -- prepare Vault101a Vault101b Vault101d 00017f37` (batch
   session exercised; cellmap.ron lands in `.bevyout/cache/`). Interrupt and
   rerun once to see resume skip completed cells.
2. Launch `cargo run-dev -- render Vault101a --agent-bridge`; over BRP
   (`bevyout.session`, `scene_snapshot`, `console.exec`, `capture_viewport`):
   drive the Vault101a → b → d → b → a door chain; `scene_snapshot`
   before/after each hop shows the destination cell's placements; capture a
   viewport image per hop.
3. Pull the swap telemetry lines; assert max frame ≤ 33 ms on every preloaded
   hop; preload start/ready/evict lines visible.
4. Comment measured results on #47/#48/#51/#52, tick #5 checklist items, PR
   `m2-wave2` → master with `Closes` footers.
