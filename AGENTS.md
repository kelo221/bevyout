# Agent guidance

## Project architecture

This project uses Vertical Slice Architecture (VSA):

- `src/main.rs` is a dispatcher only. Keep command routing there; do not add
  Fallout parsing, conversion, or Bevy systems to it.
- `src/cli.rs` owns clap command and option definitions.
- The Fallout cell feature owns its complete path from CLI input through plugin
  and BSA parsing, NIF-to-GLB conversion, manifest generation, and Bevy
  rendering. Keep those feature concerns together rather than creating a
  global layer for every parser, asset type, or system.
- `src/vsa/` contains the current Fallout cell slice internals; `viewer.rs`
  contains its Bevy presentation boundary and consumes only the prepared RON
  manifest.
- `PreparedSceneManifest` is the explicit hand-off contract inside the slice.

When adding a feature, add a new slice directory or module with its own input,
data, preparation, and runtime code. Expose only a narrow command/plugin API
to `main.rs`. Prefer Bevy `Plugin` values for new runtime feature groups
instead of growing `main.rs` or one large startup system.

## OpenMW
If any code is ported from OpenMW to Rust it must be placed in the isolated to folder.

## Local Bevy documentation

Use `BevyCheatSheet/` as the local API and architecture reference before relying on
generic Bevy examples. In particular:

- `BevyDocs/programming/app-builder.md` and `plugins.md` describe app/plugin
  composition.
- `BevyDocs/programming/systems.md` describes system organization.
- `BevyDocs/setup/bevy-config.md` documents dynamic linking.
- `BevyDocs/setup/perf.md` covers development and release profiles.

`bevy_markdown_docs/` is an offline copy of the cargo

If the local docs do not cover a version-specific detail, verify it against the
Bevy version in `Cargo.toml`.

## Build policy

Use dynamic linking for iterative desktop development:

```powershell
cargo check-dev
cargo test-dev
cargo run-dev -- prepare --cell 000151e3
```

These aliases expand to Bevy's `dynamic_linking` feature. Keep that feature
development-only; release builds must not depend on the Bevy DLLs unless they
are intentionally bundled and tested.

Before handing off changes, run `cargo fmt --check`, `cargo clippy --all-targets
-- -D warnings`, `cargo test`, and a representative `cargo run-dev` command.

## Prepared point shadows

- Point-shadow depth is generated automatically during `prepare`, after GLB
  conversion and physics classification. Do not add Blender shadow baking or
  per-frame runtime cubemap rendering to this preparation path.
- Casters must be initially enabled placements with resolved GLBs. Prepared
  static geometry excludes `PreparedSemantic::Door`, pickups, and
  `PreparedPhysicsClassification::Dynamic`; dynamic physics placements remain
  individually spawned so their current pose can cast through the viewer's
  realtime pass.
- The cache is a validated `D32_SFLOAT` KTX2 cubemap array keyed by generator
  revision, resolution/near plane, caster geometry/transforms, and light
  identity/position/range. Color, intensity, and camera changes must remain
  outside the fingerprint.
- The default cubemap face resolution is the high-quality 512 setting. Keep
  128 and 256 available only as explicit lower-quality overrides.
- KTX-Software is resolved only on a cache miss or `--rebuild-shadows`.
  Viewers never regenerate artifacts; `shadowcache rebuild` must direct users
  back to `prepare --rebuild-shadows`.
- WebGPU cannot copy CPU bytes directly into `Depth32Float`. The local
  `bevy_pbr` patch stages the decoded data through `R32Float` and writes the
  depth array once with a GPU render pass. This upload pass must not enqueue
  scene meshes or become a per-frame shadow pass.
- The viewer may enable exactly one camera-relevant startup-cell point light's
  native runtime shadow cubemap with the explicit `--realtime-shadows` viewer
  or render opt-in; it is disabled by default for performance. The console can
  toggle it at runtime with `setrender realtime_shadows 0|1`. The combined prepared scene is marked
  `NotShadowCaster`, while individually spawned dynamic/interactive meshes
  remain runtime casters and prepared receivers.
- Forward shading performs at most two cubemap lookups for one dominant point
  light: prepared and realtime visibility are combined with `min`. The
  `setrender shadow_samples 0|1` switch remains the benchmark control; there
  is no configurable multi-light gameplay shadow budget.

## Prepared container audio

- Record-level `CONT` open/close sounds are authoritative. When either field
  is absent, `prepare` may fill it from the earliest matching `Open`/`Close`
  `sound:` cue authored in the model's `NiTextKeyExtraData`.
- NIF animation sound cues are preserved as GLB metadata during conversion,
  resolved case-insensitively against `SOUN`/`SNDR` EditorIDs, and staged
  through the existing prepared-audio path. Do not add a separate runtime
  animation-event audio system for these start-of-animation container cues.
- Sound records whose `FNAM` is a directory resolve to the lexicographically
  first direct child (loose `Data` files before archive entries); exact-file
  paths retain their existing resolution and precedence.

## Prepared asset revisions

Every prepared serialized asset carries a `*_REVISION` constant
(`NAV_GRAPH_REVISION`, `ACTOR_CATALOG_REVISION`, `ITEM_CATALOG_REVISION`,
…). Bump it whenever the asset's serialized shape changes — **including
new serde-defaulted fields**, which otherwise let stale caches parse
cleanly with silently missing data. This is part of the orchestrator's
pre-PR diff review: a wave that touches a prepared type's fields without
bumping its revision is not ready. (Caught late in M4 wave 4 by external
review: `mesh_merges` shipped without a `nav-graph-v2` bump.)

## Way of working (waves)

Multi-issue work runs as "waves" against a milestone epic (e.g. #5 for M2):

- Every task gets a GitHub sub-issue under the epic (labels `area/*`,
  `enhancement` or `bug`, priority, milestone), linked via the
  sub-issue REST API (`POST /repos/{owner}/{repo}/issues/<epic>/sub_issues`
  with the issue's database id). Amend the epic's checklist when scope is
  added; tick items only when the gate criteria hold on real data.
- **Assignment invariant:** every issue a wave works on is assigned to
  the human user (`--assignee @me`, the authenticated gh account) — new
  issues at creation, pre-existing issues at wave kickoff, before any
  executor starts. An unassigned issue is not in the wave.
- Each wave has a kickoff `*_PROMPT.md` (what was requested) and a
  `*_PLAN.md` (fixed feature lists → tests → implementation) in
  `docs/plans/` — see `docs/plans/README.md` for the traceability
  convention. Amend the plan (a "Shipped amendments" section) rather than
  rewriting it when acceptance testing changes the design.
- Parallel implementation uses one agent per issue in isolated git worktrees
  with explicit file-ownership boundaries; the shared merge seam is
  `tests/features.rs` (each issue appends World fields at the end of the
  struct and a delimited step section at the end of the file). An
  integration branch (`m<milestone>-wave<n>`) collects the merges; the
  orchestrator resolves conflicts, runs gates, and does real-data
  acceptance before opening one PR with `Closes #NN` per issue.
- **Sequential exception:** when wave issues rework the same runtime seam
  (same module/file), run their executors sequentially on the wave branch
  instead of parallel worktrees — a later brief builds on the earlier
  issue's landed seam. Precedent: M4 wave 4 (#113 then #134, both in
  `src/viewer/nav/agent.rs`).
- PR review findings (human or bot) are verified by the orchestrator
  against the actual code before acting: confirmed code fixes are
  dispatched to an executor like any other change; wrong or
  out-of-scope findings are answered on the PR with the evidence.
- Measured results are commented on each issue; follow-ups discovered during
  acceptance get their own issues rather than silently expanding the wave.
- **External posting:** nothing is posted outside this repository
  (upstream bug reports, third-party PRs/comments) without showing the
  human a full draft and getting an explicit yes — it publishes under
  their identity. Housekeeping inside this repository's issues/PRs is
  pre-authorized by these conventions.
- Every wave ends with a manual acceptance script,
  `docs/plans/M<m>_WAVE<n>_MANUAL.md`, written before the wave PR and
  linked from its body. It opens with a short plain-language summary of
  what the wave shipped, then numbered step-by-step instructions a human
  can follow verbatim: which prepared cell to launch, the exact console
  commands with real FormIDs from the prepared catalog, which keys to
  press, and the expected result of every step (including any one-time
  cache/prepare setup the wave requires).
- The human must be able to *see* what a wave shipped. If the wave's
  behavior has no player-visible runtime surface (prepare-side data,
  decoded records, internal graphs), add the missing test surface as part
  of the wave — typically a viewer console command or deterministic CLI
  output — as its own small sub-issue, and drive it in the manual script.
  Precedent: `tnm` (#128) visualizing the #111 nav graph, which would
  otherwise only be inspectable as a RON file.
- Model split: see "Model routing" below; it applies to every wave,
  including single-issue waves, in the Claude runtime. In the Codex runtime,
  the orchestrating session executes directly because subagents are slow.

## Model routing

- **Claude runtime**: Strict split, no exceptions — single-issue waves included.
  The executor model executes, the orchestrator model plans; the executor can
  even write the tests, the orchestrator evaluates:
  - The orchestrating session (Opus-class or above) owns planning, architecture,
    task decomposition, GitHub housekeeping (issues, plans, PRs, comments),
    merges/conflict resolution, diff review, and evaluation: running gates,
    real-data acceptance, and judging evidence. It never writes implementation or
    test code directly.
  - Executor subagents (Sonnet) own all execution: production code and all test
    writing (feature files, cucumber steps, unit tests), each with a tightly
    scoped, self-contained brief — in an isolated worktree for parallel waves,
    or directly on the wave branch for single-issue waves.
- **Codex runtime**: Codex does not spawn subagents because they are slow. The
  orchestrating session (Sol high) executes and plans directly on the wave branch.

## Testing (feature-first)

Mandatory order inside every issue: fix the feature list → write the
Cucumber feature (`features/*.feature`) and unit tests → implement until
green. `tests/features.rs` runs cucumber with `fail_on_skipped()`, so every
scenario line needs a step. The suite has no lib target: modules under test
are included verbatim via `#[path]`, which means any module driven from
cucumber must be std/serde-only (no Bevy imports) — see `src/vsa/cell_map.rs`
and `src/viewer/world/policy.rs` for the pattern, and nest modules with
relative `super::super::` imports the way `prepare/selectors.rs` is.

Design for this: keep decision logic (planning, eligibility, caching,
state machines) in pure modules and let thin Bevy systems consume them.
Bevy-side behavior gets ordinary `#[cfg(test)]` unit tests against `World`
or a minimal `App` (see `src/viewer/console.rs` tests for the console
harness and `src/viewer/player/tests/` for bare-`World` helpers).

## Logging

- CLI commands (`prepare`, `cells`, `bake`, …): deterministic `println!`
  lines — stable wording, no timestamps, suitable for asserting in tests
  and issue comments (e.g. `batch cache: assets reused N, built M, ...`).
- Viewer/runtime: `tracing` macros (`info!`/`warn!`), never `println!`.
  Lifecycle events get grep-able stable prefixes: `preload start/ready/evict
  <formid>`, `swap <src>-><dst> instant|fallback max_frame_ms=<x>`.

## Verification over the agent bridge (BRP/MCP)

Use the bevyout MCP server if registered; otherwise the agent bridge is
plain BRP JSON-RPC over HTTP:

```
cargo run-dev -- view --manifest .bevyout/cache/scenes/<formid>/scene.ron \
    --agent-bridge --agent-port 15702 [--trace-seconds N]
curl -X POST http://127.0.0.1:15702/ -H 'Content-Type: application/json' \
    -d '{"jsonrpc":"2.0","id":1,"method":"bevyout.session","params":{}}'
```

Methods: `bevyout.session` (active cell), `bevyout.scene_snapshot`
(placements/entities), `bevyout.console.exec` (`{"line": "activate
00028579"}` drives door travel; `setrender`, `tfc`, `getpos`, … also work),
`bevyout.capture_viewport`. `help` lists every console command;
`player.setpos <x|y|z> <metres>` repositions the player one axis at a
time — the standard fix when acceptance needs an on-mesh start
(`tna spawn` at an off-mesh player start reports `AgentNotOnNavMesh`). Known limits: `capture_viewport` returns black
PNGs when the window is occluded (macOS) — use snapshots + logs as evidence;
frame-time measurements are only comparable on a cool machine (the startup
"BoxDDD prepared collision ... cook" line is the canary: ~10 ms cool,
20 ms+ means thermally degraded numbers); a transient Metal `DeviceLost`
can kill the viewer under load — retry.

## Git cautions

## Canonical item transaction invariants (#95)

- Runtime item movement goes through `src/item_transaction.rs`'s canonical
  `ItemLedger`; `PlayerInventory`, container state, and dropped-world entities
  are projections/adapters, not independent authorities.
- Every canonical stack has a stable `ItemInstanceId`. Full moves preserve it;
  partial moves allocate a destination ID; compatible merges retain the
  deterministic lowest ID and remap equipment/hotkey references atomically.
- Stack compatibility includes condition, ownership provenance, and every
  namespaced opaque extra-state tag/payload. A holder transfer must not discard
  any of those fields or partially mutate either side.
- Save v3's `ITMS` snapshot is authoritative for canonical fields. v1/v2 saves
  are read and migrated deterministically; legacy `ItemStack` values are DTOs,
  not a reason to reintroduce condition-less transfer paths.
- Static merchant buy/sell is a fixed-value, two-holder atomic transaction;
  quest items and caps are rejected, and restocking/services/crime effects are
  deferred to later slices.

- Never `git add -A`/`git add .` from the repo root: `.claude/worktrees/*`
  and scratch files get swept in. Stage explicitly by path.
- **Never commit Bethesda-derived data.** `.gitignore` blocks `*.ron`
  wholesale (exports like `cellmap.ron` and prepared `scene.ron` manifests
  are derived from game data) with explicit re-allows only for hand-written
  synthetic fixtures (`tests/goldens/`, `features/fixtures/`) and doc
  examples. A new fixture must be synthetic and needs its own `!` rule —
  never weaken the blanket ignore. The same rule extends to every other
  asset format (GLB, DDS, WAV, NIF): converted game content lives only
  under the untracked `.bevyout/` cache.
