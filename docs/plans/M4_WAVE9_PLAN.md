# M4 wave 9 — nav follow-ups: lock-respecting travel hand-off, fall-out guard + collision root cause, authored NAVM semantics (#165, #164, #148, #156)

Wave on branch `m4-wave9` off master (719ead7). Two lanes: #165 → #164/#148
sequentially on the wave branch (same `agent.rs` seam, AGENTS.md sequential
exception; the #148/#164 investigation also needs the main checkout's
`.bevyout/` cache and real-data viewer runs); #156 in a parallel worktree
(prepare-side decoder/graph seam, disjoint files). Shared merge seam is
`tests/features.rs` (each issue appends World fields at the end of the
struct and a delimited step section at the end of the file).

Model routing (Claude runtime): orchestrator (Fable) owns this plan, merges,
diff review, gates, and real-data acceptance and writes no code. Plan
Execution Model Recommendation — #165: `Sonnet` (tightly scoped FSM bug fix);
#164/#148: `Opus` (real-data investigation + root-cause judgement); #156:
`Sonnet` (mechanical decode-to-DTO plumbing with fixed semantics).

## Issue #165 — travel-door hand-off respects runtime lock state (P1, wave branch, first)

### Feature list

- **F165.1 Lock check in the travel-arrival lifecycle.** The travel-target
  door's open request (the path that deliberately exempts the agent's own
  `travel_intent` door from the mid-route crossing gate) consults the same
  runtime lock state `setlock` updates (`NavArchipelagoState.door_lock_info`
  / `PlacementRoot`). Locked → the documented deterministic terminal:
  no door open, no hand-off, no clip-through; status surfaces as
  `Unreachable` (per #155's F155.4 mapping), not `Paused`.
- **F165.2 Unlock retriggers the existing one-repath retry.** Unlocking the
  door while the agent is at the failed terminal and reissuing the travel
  completes normally.
- **F165.3 Invariant test.** Pure-policy unit tests on the arrival/open
  decision plus a cucumber scenario: A-B-A (unlocked travel → locked travel
  → unlocked travel) ends `handed-off` / `unreachable` / `handed-off`.
  Mid-route lock exclusion (#155) invariants stay untouched and green.

### Files owned

`src/viewer/nav/agent.rs` (travel-arrival + `request_door_open` boundary),
`src/viewer/nav/door_link.rs` (FSM if state is needed),
`features/nav_travel_lock.feature` (new), own delimited section of
`tests/features.rs`.

### Acceptance (orchestrator, real data)

FranklinMetro02 (0001a273), door 0007f7e3, agent grounded at
(90.00, 96.20, -144.00): the issue's exact A-B-A with `setlock` ends
`handed-off` / `unreachable` (no hand-off within the wait budget) /
`handed-off`.

## Issues #164 + #148 — collision gaps vs walkable navmesh: root cause + fall-out-of-world guard (wave branch, after #165)

One executor, because they are siblings of one investigation (inverse
defects in the same cell family: #164 = walkable mesh with no collision
under it, #148 = collision blocking walkable mesh interior).

### Feature list

- **F164.1 Fall-out-of-world guard.** A nav agent whose Y falls below a
  kill-Z derived from the prepared cell's geometry bounds (margin, not a
  magic constant) is despawned/reset deterministically with a stable
  `warn!` line (`nav agent fell out of world <id> y=<y> …`) instead of
  descending forever with `status=unreachable stuck=true`. Pure decision
  policy (bounds + position → verdict) unit/cucumber-tested; thin Bevy
  system applies it. Player handling is out of scope (noted, not built).
- **F164.2 Collision-gap root cause (investigation, evidence required).**
  On 0001a273, identify which placement/collider should cover
  x≈-16.5..-14.2, z≈-57 and why conversion/classification dropped it
  (compare prepared physics manifest vs authored NIF collision for the
  platform placements). Deliverable is a written diagnosis on the issue
  with the offending placement/FormID — plus the fix if it is a
  conversion/classification bug in our code.
- **F148.1 Post-wave-8 re-measurement.** Re-run the #148 route
  (`tna spawn` (9.6, 106, -73.1), `tna goto` (-19, 103.4, -59.5)) on the
  wave branch (swept portals #154 + corridor-progress stuck #157 active),
  and record the Vault 101 Entrance stair-top reproduction in the issue's
  measurement format, as its amendment requires.
- **F148.2 Interior-obstruction root cause (investigation, evidence
  required).** Identify the collider overlapping walkable navmesh near
  (9.9, 106, -72)–(10.2, 106, -74) (which placement, which shape). A
  steering workaround does not close #148; the diagnosis decides #153's
  direction. Fix only if it is a classification/conversion bug; if it is
  authored-data reality, report and leave #148 open with the evidence.

### Files owned

`src/viewer/nav/agent.rs` (guard system only — nothing in door/travel
paths #165 owns; executor runs after #165 lands on the branch),
`src/viewer/nav/movement_policy.rs` or a new pure `fall_guard` policy
module, `features/nav_fall_guard.feature` (new), own delimited section of
`tests/features.rs`. Investigation may read prepare/physics code anywhere;
fixes there need the orchestrator's per-file go-ahead first.

### Acceptance (orchestrator, real data)

On 0001a273: `tna spawn` at the platform edge (-16.31, 103.31, -57.26)
does not fall through the world — a forced fall ends in one logged
despawn/reset line, no infinite descent. Both issues carry a posted
root-cause diagnosis with placement identity. #148's route either
completes `reached` (if a code bug was fixed) or the issue stays open
with the diagnosis comment — that outcome still counts as this wave's
deliverable for #148.

## Issue #156 — consume authored NAVM semantics (parallel worktree)

### Feature list

- **F156.1 Preferred-pathing costs.** Preferred-path triangle flags map to
  a landmass node type with a cheaper cost in the `landmass_graph`
  conversion (today every polygon is type zero). Deterministic mapping,
  unit-tested; coexists with #155's door type indices.
- **F156.2 NVTR external-edge evidence for portals.** Per-edge
  external-edge flags flow from the decoder into
  `PreparedNavMeshMerge` candidate generation/validation as authored
  evidence: candidates on NVTR-flagged edges are annotated (and
  prioritized over pure geometric proximity); the prepare diagnostic
  line reports authored-vs-geometric counts per cell.
- **F156.3 NVEX/NVCI correlation, not consumption.** NVEX external
  connections and NAVI `NVCI` are decoded/correlated into a prepare
  diagnostic (counts + FormID cross-references against the cell's doors
  and NAVMs) so future exterior stitching starts from evidence.
  No runtime behavior change; document findings in the issue.
- **F156.4 Revision bump.** `PreparedNavMeshMerge`/nav-graph shape changes
  bump `NAV_GRAPH_REVISION` to `nav-graph-v4` (serde-defaulted fields
  included, per AGENTS.md).

### Files owned

`src/vsa/openmw_esm4/navmesh.rs` (decode additions only),
`src/vsa/prepare/nav_graph.rs`, `src/viewer/nav/landmass_graph.rs`,
`src/viewer/nav/mod.rs` (DTO fields), `features/nav_authored_semantics.feature`
(new), own delimited section of `tests/features.rs`.

### Acceptance (orchestrator, real data)

Re-prepare 0001a273 and 00024512: diagnostics show authored NVTR edge
counts and portal-candidate annotation; routes from the wave-8 manual
still complete; any portal-candidate set change is explained by authored
evidence in the diagnostic (this is the input that later shrinks #162).

## Gates (every lane, before merge)

Feature-first order inside each issue: feature list → cucumber + unit
tests → implementation. `cargo fmt --check`, `cargo clippy --all-targets
-- -D warnings`, `cargo test`, representative `cargo run-dev -- prepare`.
Manual script `docs/plans/M4_WAVE9_MANUAL.md` before the PR; PR closes
#165 and #164 (`Closes`), references #148 and #156 per their outcome.

## Shipped amendments

- **A1 — #165 had a second mechanism, found only in real-data acceptance.**
  The planned lock check on the open-request path (`06c41eb`) fixed a
  demonstrated oscillation (`Failed` cleared `travel_intent` but not
  `AgentTarget3d`), but the issue's measured hand-off was a different
  bypass: a prior hand-off leaves the door physically open, so no open
  request — and therefore no lock check — ever fires. `021ab30` adds
  `door_link::effective_door_open`: for a `Travel` destination the
  scripted hand-off is lock-gated regardless of physical open state;
  intra-cell crossings keep the "already open passes" rule.
- **A2 — acceptance found a latent wave-8 seam bug, filed as #169.** A
  `setlock` issued before the nav archipelago exists updates
  `InteractionState`/`PlacementRoot` but is lost for #155's query-time
  cost override: an early unlock of an authored-locked door leaves a
  stale impassable cost (`unreachable state=NoPath` on an unlocked
  door). Workaround in the manual script: spawn the agent before
  `setlock`.
- **A3 — #156 executor boundary deviations, reviewed and accepted.**
  Necessary wiring in `src/vsa/prepare/navmesh.rs` (input conversion +
  extended summary line) beyond the planned file list, and mechanical
  `is_preferred_pathing: false` additions to test-only `PolygonInput`
  literals in `agent.rs`/`nav_overlay.rs` (no logic touched). The
  preferred-pathing base cost is assigned/tested but intentionally
  unwired (`set_type_index_cost` lives in `agent.rs`'s archipelago
  build, another seam) — follow-up #168.
- **A4 — both collision root causes are authored-data reality, not
  pipeline bugs.** #164: the restroom strip's room-shell statics
  (`OffRmCorInExSmL01` 370287, `OffRmCorIn05` 370299) end at x≈−15.6
  while the NAVM runs to x≈−14.2 over empty-collision clutter. #148:
  `MetHallEntrance01` (370250) TriangleMesh overlaps the walkable NAVM
  at the doorway threshold; Vault 101's stair-top wedge sits at the
  `VURmGearExit01` (149187) / `CaveHallVaultTrans01` (149223) seam.
  #148 stays open per its amendment; the evidence decides #153 toward
  collision-aware navmesh validation/rebuild.
- **A5 — real-data NVTR evidence is zero in interior cells.** Both test
  cells show `candidates authored 0`: interior NAVM seams are not
  NVTR-flagged, so #162 should not wait on authored evidence (it only
  materializes for exterior stitching, M6). NVCI's fopdoc layout parses
  cleanly on real FO3 bytes (7198 subrecords / 11150 entries, plausible
  per-cell FormID matches).
