# M2 Wave 2 — Kickoff Prompt

You are resuming work on bevyout (Fallout 3 → Bevy). A previous session completed
M2 wave 1. This file is your full briefing; follow the same configs, rules, and
workflow that produced wave 1.

## Read first (in this order)

1. `AGENTS.md` — VSA architecture, OpenMW-isolation rule, build/gate policy.
2. `M2_PLAN.md` — wave 1 plan and what shipped.
3. GitHub epic #5 (M2) and sub-issues #45–#49; PR #50 (wave 1: cell map + batch
   selectors). If #50 is not yet merged, ask the user whether to merge it first —
   wave 2 builds directly on it.
4. Memory: `m2-import-wave` (session memory dir) — baseline numbers and the
   BRP-over-curl workflow.

## State after wave 1

- `cells --map` emits a deterministic RON `CellMap`: 42,410 cells (41,989 exterior
  with grid), 32 worldspaces, 1,118 door edges, 0 unresolved (Fallout3.esm GOTY).
  Vault 101 route fully mapped: Vault101a ↔ Vault101b ↔ Vault101d, exits to
  Wasteland (0x3C). Artifact types: `src/vsa/cell_map.rs` (serde+std only).
- `prepare --all | --all-interiors | --worldspace <ws> | <selectors...> [--list-only]`
  resolves batches through a pure `resolve_selection` (`src/vsa/prepare/selectors.rs`).
  421 interiors total; only 000151e3 (MegatonPlayerHouse) and 00017f37 are prepared.
- Remaining claimed import issues, dependency-ordered: #47 batch session →
  #48 resumable prepare → #49 fingerprints.

## Wave 2 goal — instant cell transitions ("smooth, fast, smart cell loading")

The player enters the next cell through a door WITHOUT a loading screen. Instant.

Verdict from wave 1 analysis: **possible for batch-prepared interiors via
predictive neighbor preloading; impossible for unprepared cells** (Blender
conversion can never be inline — epic #5's runtime policy already mandates the
loading-screen fallback there). So instant = preloaded, and preloading is driven
by the cell map's door graph.

### Design sketch (validate against the code before committing to it)

1. **Content prerequisite — #47 + #48 first.** Instant travel needs destination
   manifests to exist. Implement the batch session (parse plugin chain + BSA
   indexes once, shared NIF/texture/audio/physics caches) and resumable
   `prepare --all-interiors` (bounded workers, job manifest, retry-failed,
   failure summary). Then actually batch-prepare the Vault 101 route (at
   minimum Vault101, Vault101a/b/d) as the test corpus.
2. **Runtime cell map.** Load the `CellMap` RON at viewer startup (new resource;
   `cells --map --out` already produces it — decide whether prepare should drop
   it into the cache dir automatically).
3. **Predictive preloader (new area/world issue — create and claim it).** On
   entering a cell, walk 1-hop door edges and background-load each neighbor's
   prepared scene: Bevy async `AssetServer` loading, spawn under a disabled
   world root per cell (the enable-parent machinery pattern), colliders built
   staggered across frames to avoid spikes. Memory budget + resident-cell LRU
   as a config knob (`config.toml`), evicting farthest-by-graph first.
4. **Instant swap (new area/world issue).** Door activation with a resident
   destination: same-frame root enable/disable swap, player teleport to the
   XTEL position/facing (through `placement_transform`), physics bodies
   activated, audio crossfade. Persistent + enable-parent state applied from
   the save layer (M1) on activation, not on preload, so preloaded copies
   can't go stale.
5. **Fallback path.** Non-resident destination → fade + loading UI (the M2
   checklist item), never inline Blender. Failure returns the player to the
   source cell (policy).

### Acceptance (real data, measured — not vibes)

- Vault101a → b → d → b → a chain of door activations with zero loading screens.
- Transition cost measured via the diagnostics/frame telemetry: no frame > 33 ms
  during a preloaded swap (define the exact metric in the plan; log it).
- Preloader observable in logs: per-cell preload start/ready/evict lines.
- Cucumber features for: preload policy state machine (what gets loaded/evicted
  when), swap eligibility (resident vs not), fallback selection.
- BRP verification of the whole loop (see workflow below): console-driven door
  activation or scripted travel, `scene_snapshot` before/after showing the new
  cell's placements, `viewport_capture` for visual confirmation.

## Workflow rules (identical to wave 1)

- Create a GitHub sub-issue under epic #5 for every task you claim (labels
  `area/import` or `area/world`, `enhancement`, `priority/P1`, milestone M2,
  assignee @me, linked via the sub-issue API). Amend #5's checklist if the
  preload/instant-swap items aren't on it (they currently are not — only
  "fades, loading UI" fallback is).
- Write the plan to `M2_WAVE2_PLAN.md` in the M1/M2 plan style: fixed feature
  list → Cucumber features + tests FIRST → implementation.
- Execute with a swarm of Sonnet subagents in isolated worktrees, one per issue,
  with explicit file-ownership boundaries; orchestrator merges into an
  `m2-wave2` integration branch, resolves conflicts, runs gates.
- Gates: `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`,
  `cargo test`, representative `cargo run-dev` commands. CLI logging =
  deterministic `println!`; viewer logging = `info!`/`warn!` (tracing).
- OpenMW ports only inside isolated folders with NOTICE/README (existing
  pattern: `src/vsa/openmw_esm4/`, `src/viewer/openmw_player/`). OpenMW
  checkout: `~/projects/openmw` — see `apps/openmw/mwworld/scene.cpp`
  (`preloadCells`, cell transition), `cellpreloader.cpp` (worker-thread
  preloading — this is prior art for exactly this feature).
- Verify with the bevyout MCP if registered; otherwise the agent bridge is
  plain BRP JSON-RPC: launch
  `cargo run-dev -- view --manifest .bevyout/cache/scenes/<formid>/scene.ron --agent-bridge --agent-port 15702`,
  then `curl -X POST http://127.0.0.1:15702/` with methods `bevyout.session`,
  `bevyout.scene_snapshot`, `bevyout.console.exec`, `bevyout.capture_viewport`.
- End state: integration branch pushed, PR opened (`Closes #NN` per issue,
  Claude Code footer), issues commented with measured results, memory updated.
- Caution: never `git add -A` from the repo root (`.claude/worktrees/` and the
  untracked `*_PLAN.md` files get swept in); stage explicitly.

## Suggested wave split

- Wave 2a (parallel): #47 batch session ∥ runtime cell-map resource + preloader
  skeleton (loads/evicts, no swap yet).
- Wave 2b (after 2a): #48 resumable prepare ∥ instant swap + fallback.
- Then: batch-prepare Vault 101 corpus, measure, verify over BRP, ship.
