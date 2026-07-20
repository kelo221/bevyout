# M4 wave 10 — collision-derived navmesh rebuild; portal quarantine, preferred costs, setlock init (#153, #148, #162, #168, #169)

Wave on branch `m4-wave10` off master (2c5f616). Two parallel lanes with
disjoint seams:

- **Lane A (wave branch, main checkout):** #153 — prepare-side collision
  validation/clearance rebuild. Needs the `.bevyout/` cache and real-data
  prepare runs. Closes #148 via its acceptance route.
- **Lane B (isolated worktree):** #162 → #168 → #169 sequentially inside
  one executor — all three rework the `src/viewer/nav/agent.rs` /
  `src/viewer/nav/landmass_graph.rs` runtime seam, which lane A does not
  touch.

Shared merge seam is `tests/features.rs` (append-only World fields at the
end of the struct, one delimited step section per issue at the end of the
file). Model routing (Claude runtime): orchestrator (Fable) owns plan,
merges, diff review, gates, real-data acceptance; writes no code. Plan
Execution Model Recommendation — #153: `Opus` (geometry pipeline with
subtle correctness trade-offs); #162/#168/#169: `Sonnet`.

## Issue #153 — collision-derived navmesh validation and clearance rebuild (lane A)

Scope per the 2026-07-20 rescope comment on the issue. Authored NAVM
remains the base topology; prepare validates it against cooked collision,
then does clearance on the validated surface. Interim erosion (#136) is
retired where superseded.

### Feature list

- **F153.1 Collision-support validation.** Per walkable triangle,
  collision support is tested under the surface within step height,
  against the cell's cooked static collision shapes (the prepared physics
  classification wave 9 verified faithful). Unsupported area is removed
  or split off; a prepare diagnostic reports removed area per mesh.
  Fixes the #164 class (walkable mesh over void) at query time.
- **F153.2 Interior-obstruction cutting.** Static collision volumes
  overlapping the interior of the walkable surface (entrance frames,
  collidable clutter) mark/cut the overlap unwalkable for the agent
  capsule radius. Fixes the #148 class (plannable-but-unwalkable wedge).
- **F153.3 Clearance with correct semantics.** Agent-radius offset with
  miter-corrected corners (radius / cos(θ/2), limit 2.5r — reuse the
  reverted wave-6 implementation from wave-6 branch history) computed on
  the validated mesh; sub-diameter corridors disconnect (the pinch-guard
  preservation behavior is inverted and its tests rewritten to assert
  disconnection); seam/door protected edges respected; new islands are
  legitimate output.
- **F153.4 Pipeline integration.** The validation/clearance pass runs in
  prepare after physics classification; `NAV_GRAPH_REVISION` bumps to
  `nav-graph-v5`; the standalone runtime erosion pass is removed or
  reduced to consuming prepared results. Deterministic `println!`
  diagnostics (stable wording) for removed/cut/disconnected counts.

### Files owned

`src/vsa/prepare/nav_graph.rs`, new pure module(s) under
`src/vsa/prepare/` (e.g. `nav_clearance.rs`), `src/vsa/prepare/navmesh.rs`
(pipeline wiring), `src/viewer/nav/erosion_policy.rs` (retire/reduce),
`src/viewer/nav/mod.rs` (only if DTO fields change),
`features/nav_collision_clearance.feature` (new), own delimited section of
`tests/features.rs`. NOT `agent.rs`/`landmass_graph.rs`/`door_link.rs`/
`movement_policy.rs`/`repath.rs` (lane B's seam).

### Acceptance (orchestrator, real data)

- 0001a273: #148's route — spawn (9.6, 106, −73.1) → goto
  (−19, 103.4, −59.5) — ends `status=reached stuck=false blocked=false`
  (**closes #148**); a goto into the restroom strip (−15, 103.3, −57) is
  `unreachable` at query time (no fall-guard trigger).
- 00024512: doorway route `tna goto 154 36.5 -34` through door 00028579
  completes — the wave-6 miter regression must not reappear; the stair
  route from #148's measurement is re-measured and recorded (a remaining
  riser wedge is a KCC follow-up, not a blocker).
- Synthetic invariant: corridors narrower than 2×agent radius produce no
  route.

## Issues #162 + #168 + #169 — nav runtime seam (lane B, one executor, in this order)

### Feature list

- **F168.1 Preferred-path base cost.** `ensure_archipelago` calls
  `Archipelago::set_type_index_cost(preferred_pathing_type_index, cost)`
  with a documented constant < 1.0. Per-agent overrides stay reserved for
  door locking. Unit test: preferred vs non-preferred alternative of
  comparable length picks preferred.
- **F169.1 Lock overrides derived at build.** Query-time lock cost
  overrides are (re)derived from current runtime lock state when the
  archipelago is built (not only from lock-change events), so a `setlock`
  issued before the archipelago exists behaves identically to a late one.
  Invariant test: build-after-unlock queries the door as passable.
- **F162.1 Per-link portal quarantine.** A timed-out merge-portal
  crossing quarantines that specific link for that agent's subsequent
  repaths (keeping the destination) instead of clearing the route. The
  landmass 0.9.2 mechanism is the executor's design call — extending
  polygon type indexing to seam-adjacent polygons with per-agent cost
  overrides is the expected shape (mirrors door locking); a per-agent
  quarantine set lives in nav state with the existing stuck/blocked
  reporting. Route around when an alternate exists; fail fast
  (`Unreachable`) when none does. Replaces the wave-8 `ponytail:` clear
  in the `merge_traversal_system` timeout branch.
- **F162.2 Quarantine lifecycle.** Quarantine is per-agent and clears on
  target change, hand-off, or despawn (simplest deterministic rule;
  document it). No global/persistent portal blacklist.

### Files owned

`src/viewer/nav/agent.rs`, `src/viewer/nav/landmass_graph.rs`,
`src/viewer/nav/repath.rs`, `src/viewer/nav/ledger_policy.rs` if needed,
`features/nav_portal_quarantine.feature` (new), own delimited section of
`tests/features.rs`. NOT the prepare-side files (lane A's seam). If a
prepared-shape change seems needed, stop and ask the orchestrator.

### Acceptance (orchestrator, real data)

0001a273: with the genuine seam portal available, a forced-block scenario
(if reproducible post-#153) repaths or fails fast without route-clearing;
`tna travel`/lock flows from the wave-9 manual unchanged. Early-`setlock`
repro from #169 (fresh session, `setlock 0007f7e3 0` before `tna spawn`,
then travel) ends `handed-off`. Preferred-cost: `tnm` route inspection on
a cell with preferred flags.

## Gates (both lanes, before merge)

Feature-first order per issue: feature list → cucumber + unit tests →
implementation. `cargo fmt --check`, `cargo clippy --all-targets --
-D warnings`, `cargo test`, representative `cargo run-dev -- prepare`.
Manual script `docs/plans/M4_WAVE10_MANUAL.md` before the PR; PR closes
#153, #148, #162, #168, #169.

## Shipped amendments

- **A1 — #153's clearance shipped as validation + sub-diameter disconnect,
  without boundary offset.** Three acceptance-driven iterations: (i) the
  planned miter offset + inversion-based disconnect over-fragmented both
  cells (Vault doorway/stairs unreachable, metro corridors shredded —
  the wave-6 regression in stricter form); (ii) rework measured the
  authored passage width instead of trusting offset arithmetic, added a
  connectivity guard (un-drop anything stranding a large or seam/door
  component) and per-drop/island diagnostics — connectivity 99%/100%;
  (iii) a bounded per-edge adjacency-cut spike (vertex-split severing,
  mechanism verified against landmass 0.9.2) failed real-data selection
  — obstruction cuts eroded every perimeter (86%) while missing the
  MetHallEntrance01 posts, void cuts missed the sub-triangle overhang —
  and was reverted per the iteration cap. Wall clearance is delegated to
  the KCC (no vertex movement); the sub-triangle classes moved to #171.
- **A2 — #148 stays open, carried by #171.** The route plans and the
  agent still wedges at (9.90, 106.05, −73.84); per-triangle/per-edge
  granularity provably cannot cut the flanking posts. The restroom
  void's query-time removal likewise deferred (fall guard backstops it).
- **A3 — Vault stair wedge re-measured, filed as #172 (KCC step
  capability).** Routes are plannable post-#153 (no miter regression);
  the capsule fails the authored riser seam at z≈−80.4 in both wave-9
  and wave-10 measurements.
- **A4 — cross-lane defects found by merge/acceptance, fixed in-wave.**
  `PreparedNavPolygon` derived `Default` disagreed with its serde
  default (`walkable: false` vs `true`), breaking Rust-side
  constructors; and `PauseAgent` was never removed on the door-wait
  `Failed` terminal, freezing any agent that ever failed a door wait as
  permanently `paused` (found live; the index-misalignment hypothesis it
  emerged from was disproven with a value-based-lookup audit).
- **A5 — prepare-side executor edited `orchestrator.rs`** (minimal
  post-physics hook for the clearance pass) beyond the planned file
  list; reviewed and accepted.
- **A6 — #162's real-data forced-block scenario was not exercisable**:
  post-#154/#153 both cells' surviving portals are physics-validated, so
  no blocked merge link exists to quarantine on real data. Coverage is
  the three-layer unit/cucumber composition per the plan's allowance.
