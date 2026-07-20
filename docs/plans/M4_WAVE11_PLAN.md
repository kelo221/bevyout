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

### Acceptance (orchestrator, real data) — REVISED, see amendment A1

The original criterion (both Vault routes end `reached`) is **void**: the
investigation proved the wedge is the closed `VaultGearDoor` activator,
not a riser, so no KCC change could make those routes complete. #172 is
closed as premise-disproven; the real defect is #177. What this wave
ships instead is the missing stair regression coverage, gated on the
synthetic `boxddd` tests being green and no regression in the wave-10
manual's metro checks.

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


## Shipped amendments

- **A1 — #172's premise was disproven; closed, replaced by #177.** The
  Vault 101 "stair-top wedge" tracked since wave 8 is the **closed vault
  door**: `VaultGearDoor` (RefID 149264, `base_kind: ACTI`,
  `semantic: Activator`, `physics_classification: Kinematic`, collision
  face z ≈ −80.0). Agent radius 0.35 puts a blocked capsule centre at
  z ≈ −80.35, matching every measurement across three builds
  (−80.4/−80.37/−80.38) and explaining why two different targets wedge
  at the same z. An isolated collider replay reproduced it at
  (154.360, 39.787, −80.380); removing only that collider clears the
  path; the two stair statics are traversable in isolation and both
  carry `step_support: true`. Verified independently by the orchestrator
  against `scene.ron` and the code. F172.2 was also moot: the nav agent
  already reuses the player controller's step handling with an identical
  capsule, so no second policy existed to reconcile. The wave ships the
  missing stair regression tests only.
- **A2 — new issue #177 (P1):** nav route topology is gated exclusively
  on `PreparedSemantic::Door(_)` (`nav/mod.rs:151`, `nav/agent.rs:1018`,
  `nav/agent.rs:3786`; no `Activator` handling anywhere in nav), so
  solid closed activators/kinematic placements are planned straight
  through. Must generalize door topology to state-dependent blocking
  placements, reusing #155's query-time cost overrides and #137's
  mid-route gating. Explicitly *not* a job for #171's clearance pass:
  that validates **static** collision, and a state-dependent blocker
  must never be baked into the prepared navmesh as permanent geometry.
- **A3 — #171 shipped after four acceptance-driven iterations, with two
  defects caught only by real-data runs.** (i) The clip's sliver filter
  was a *shape* filter, not a degeneracy epsilon (1573 spurious discards
  on 0001a273 → 3), each discard punching an adjacency-severing hole in
  the conformal cover. (ii) Obstruction was judged at the query point
  rather than at the collider's own footing, so every stair riser read as
  a wall — fixing it took Vault 101 from 90%→98% component share and
  91%→95% authored-reachable (its stranded islands *were* its
  staircases). (iii) The near-collinear pieces the corrected epsilon
  retained had no reliable winding, and landmass rejected the **whole
  mesh**: Vault 101 shipped briefly with *zero navigation* while prepare
  reported 98% connectivity. Resolved by welding ill-conditioned pieces
  (conformal, area-conserving, protected vertices win), refusing welds
  that would flip a neighbour's winding, and a validity gate — plus, on
  the executor's initiative, running **landmass's own validator at
  prepare time as a hard build failure**. That last item is the durable
  fix: replicating the convexity rule catches the class, only the real
  validator proves the runtime will accept the result.
- **A4 — three orchestrator measurement errors, corrected by executors.**
  A collider scan testing the capsule *centre* (missing every wall the
  capsule rests against, which is how #148 stayed misattributed for four
  waves); `tna goto` probes issued at a fixed y down a descending ramp
  (producing a phantom "severance" that did not exist — every corridor
  point was in the MAIN component); and reading focus-stalls as viewer
  crashes. The durable outcome is that diagnostics now live in the code —
  the `BEVYOUT_NAV_PROBE` point-explainer, the prepare validator gate,
  and the runtime blocking-plane line — rather than in ad-hoc
  orchestrator scripts.
- **A5 — #148 does not close.** Its root cause is now known
  (`MetroGateLoad`, a closed travel door whose interior the navmesh marks
  walkable; the `tna spawn` point sits 0.041 m from its face, inside the
  collision) and its stated acceptance criterion is void — the route it
  specifies crosses a closed door. Carried by #177.
- **A6 — #180/#181 filed and #180 shipped mid-wave.** Acceptance kept
  "hanging"; the cause was that no `WinitSettings` was configured, so the
  app stopped ticking when unfocused and the main-thread agent-bridge
  methods never ran. Fixed with `WinitSettings::continuous()` plus an
  `--unfocused` flag implied by `--agent-bridge`; verified 12/12 bridge
  polls over 60 s while covered. True headless rendering with screenshot
  capture is #181.
