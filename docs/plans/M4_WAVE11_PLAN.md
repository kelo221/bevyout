# M4 wave 11 — finish the navmesh saga (#171, #148, #172) and start AI packages (#175, #176)

Wave on branch `m4-wave11` off master (175eae4). Three lanes with disjoint
seams:

- **Lane A (wave branch, main checkout):** #171 sub-triangle clearance,
  closing #148. Needs the `.bevyout/` cache for prepare runs.
- **Lane B (worktree):** #172 KCC stair/step capability — agent physics
  seam only.
- **Lane C (worktree):** #175 + #176 sequentially — prepared AI package
  catalog and its console surface.

Shared merge seam is `tests/features.rs` (append-only World fields at the
end of the struct, one delimited step section per issue at the end).
`src/vsa/prepare/orchestrator.rs` is owned by **lane C only** (lane A's
clearance hook already exists in master and must not need changes there;
if it does, lane A stops and coordinates).

Model routing (Claude runtime): orchestrator (Opus) owns plan, merges,
diff review, gates, real-data acceptance; writes no code. Plan Execution
Model Recommendation — #171: `Opus` (the geometry lift the last three
iterations proved is hard); #172: `Opus` (physics/KCC correctness);
#175/#176: `Sonnet` (mechanical staging + console command over an
existing decoder).

## Issue #171 (+ #148) — local re-triangulation for sub-triangle clearance (lane A)

### Non-negotiable: holistic only

The fix is a general geometric rule applied to every polygon and every
static collider in every cell. **Forbidden:** any branch keyed on a
FormID/RefID/EditorID/cell, any threshold tuned to a test cell's
coordinates, any steering or runtime workaround that routes agents around
an unidentified obstruction (#148's amendment forbids this explicitly),
any "if near x≈9.9" special case. `MetHallEntrance01` and the restroom
strip are *acceptance cases that prove the general rule*, never targets.
The orchestrator greps the diff for placement identifiers and coordinate
constants before merge.

### Feature list

- **F171.1 Polygon clipping against obstruction footprints.** Each nav
  polygon is clipped in 2D against the footprints of wall-like (non
  step-overable) static colliders expanded by the agent radius,
  producing sub-polygons whose edges lie on the obstruction boundary.
  The clipped-away area is unwalkable; the remainder stays walkable and
  correctly triangulated. Generic: it fires wherever a collider intrudes,
  including a post flanking a triangle's interior opening.
- **F171.2 Polygon clipping against the support/void boundary.** The
  same clip runs against the collision-support boundary, so a triangle
  straddling supported floor and void splits into a supported walkable
  sub-polygon and a removed void sub-polygon.
- **F171.3 Existing machinery applies to sub-polygons.** The shipped
  passage-width disconnect, connectivity guard (never strand a large or
  seam/door-bearing component), protected seam/door handling, and
  per-drop/island diagnostics all operate on the clipped output
  unchanged in intent.
- **F171.4 Prepared shape + revision.** Clipped geometry changes the
  prepared nav graph (new vertices/polygons, adjacency); bump
  `NAV_GRAPH_REVISION` to `nav-graph-v6`. The runtime consumes it through
  the existing `mesh_inputs` path — landmass keys adjacency on shared
  vertex indices (verified in wave 10), so clipping must produce
  consistently welded vertices where areas stay connected and split
  vertices where they must not.
- **F171.5 Robustness.** Degenerate/sliver output is filtered
  deterministically; the pass is deterministic in ordering and stable
  across runs; prepare cost stays acceptable (report timings).

### Approach

Preferred: 2D constrained-Delaunay / polygon-boolean clip of the authored
NAVM polygons against collision-derived boundary segments (preserves
authored topology, doors, merges, preferred-path types). The alternative
(Recast-style heightfield rebuild) discards authored NAVM fidelity and is
out of scope unless the executor proves the clip approach unworkable and
reports before switching. A small, well-tested pure geometry helper is
expected — evaluate an existing dependency only if it is already vendored
(no new deps without asking).

### Files owned

`src/vsa/prepare/nav_clearance.rs`, new pure geometry module(s) under
`src/vsa/prepare/`, `src/vsa/prepare/nav_graph.rs`,
`src/vsa/prepare/navmesh.rs`, `features/nav_collision_clearance.feature`,
own delimited section of `tests/features.rs`. NOT `orchestrator.rs`
(lane C), NOT `src/viewer/nav/agent.rs` (lane B).

### Acceptance (orchestrator, real data)

- 0001a273: #148's route — spawn (9.6, 106, −73.1) → goto
  (−19, 103.4, −59.5) — ends `status=reached stuck=false blocked=false`,
  **or** the measured post gap is < 0.7 m and the route is `unreachable`
  fail-fast with the width recorded (either closes #148, per #171's text).
- 0001a273: `tna goto -15 103.3 -57` from supported floor is
  `unreachable` at query time (no fall-guard trigger).
- Connectivity ≥95% on both cells (`smallest largest-component share`);
  wave-10's doorway/pinch invariants stay green; Vault 101 doorway route
  still plannable.

## Issue #172 — KCC stair/step capability (lane B)

### Feature list

- **F172.1 Step resolution over authored risers.** The nav agent's swept
  KCC climbs and descends authored FO3 stair geometry, including the
  seam between two adjacent TriangleMesh statics. Compare against the
  player controller, which traverses the same Vault 101 stairs today —
  reuse its step handling rather than inventing a second policy.
- **F172.2 Deterministic policy, tested.** Step-up/step-down decision
  logic lives in a pure, unit-testable form (step height, ground probe,
  seam tolerance) consumed by the Bevy system, per the repo's
  pure-policy convention.
- **F172.3 No regressions.** Grounding, fall-guard (#164), slopes, and
  the wave-9/10 route suites stay green.

### Files owned

`src/viewer/nav/agent.rs` (KCC/step section only),
`src/viewer/nav/movement_policy.rs` or a new pure step module,
`features/nav_stairs.feature` (new), own delimited section of
`tests/features.rs`. May *read* the player controller freely; changes
there need orchestrator go-ahead.

### Acceptance (orchestrator, real data)

00024512: spawn (154.66, 41.10, −108.22) → goto (152.5, 36.6, −37) ends
`reached`; the doorway route (154, 36.5, −34) also completes. No
regression in the wave-10 manual's metro checks.

## Issues #175 + #176 — AI package foundation (lane C, in this order)

### Feature list

- **F175.1 Prepared package catalog.** Decoded `PackageRecord` data
  (type/flags, `PSDT` schedule, `PLDT` location, `PTDT` target, `CTDA`
  conditions) is staged into a prepared, revisioned catalog
  (`PACKAGE_CATALOG_REVISION`) during `prepare`.
- **F175.2 Per-actor priority order.** Each actor's package list is
  resolved in authored order (FO3 priority semantics), including
  template-inherited packages via the existing
  `TEMPLATE_USE_AI_PACKAGES` path.
- **F175.3 Diagnostics.** Deterministic stable-wording lines for
  unsupported package types/subrecords, unresolved location/target
  FormIDs, and actor-referenced-but-missing packages; a summary count
  line in the existing catalog style.
- **F176.1 `showpackages <formid>` console command.** Prints each
  resolved package in priority order (FormID, EditorID, type, schedule,
  location, target, condition count), a clear no-packages line, and a
  deterministic unknown-FormID error. Works over the agent bridge.

Conditions are carried as data only — no evaluation this wave (that is
#115's runtime layer; full GECK functions stay with #15).

### Files owned

`src/vsa/prepare/actor_catalog.rs`, a new prepared package catalog module
under `src/vsa/prepare/`, `src/vsa/prepare/orchestrator.rs` (staging
hook — lane C owns this file this wave), `src/vsa/catalog.rs` if the
prepared type belongs there, `src/viewer/console/` (the command, in the
existing `ConsoleCommandProvider` module style — keep the console module
root under its 150-line cap), `features/ai_packages.feature` (new), own
delimited section of `tests/features.rs`. NOT any nav file.

### Acceptance (orchestrator, real data)

A prepared cell with real NPCs reports package counts in the prepare
diagnostics; `showpackages` in the viewer prints a real NPC's authored
schedule, and that output goes verbatim into the manual script.

## Gates (all lanes, before merge)

Feature-first per issue: feature list → cucumber + unit tests →
implementation. `cargo fmt --check`, `cargo clippy --all-targets --
-D warnings`, `cargo test`, representative `cargo run-dev -- prepare`.
Manual script `docs/plans/M4_WAVE11_MANUAL.md` before the PR; PR closes
#171, #148, #172, #175, #176.
