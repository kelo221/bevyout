# M4 wave 8 — nav correctness: portals, door topology, stuck progress (#154, #155, #157)

Wave on branch `m4-wave8` off master (72ec705). Two lanes: #154 → #155
sequentially on the wave branch (same `agent.rs`/`landmass_graph.rs`
seam, AGENTS.md sequential exception); #157 in a parallel worktree.
Shared merge seam is `tests/features.rs` (each issue appends World
fields at the end of the struct and a delimited step section at the end
of the file). Model routing per AGENTS.md: executors (Sonnet) write all
production and test code; the orchestrator owns this plan, merges,
review, gates, and real-data acceptance.

## Issue #154 — cross-mesh portals: edge identity, validation, KCC sweep

### Feature list

- **F154.1 Portal edge identity.** `PreparedNavMeshMerge` carries the
  matched boundary edge on both sides (vertex indices / world-space
  portal interval endpoints), not just triangle IDs. Bump
  `NAV_GRAPH_REVISION` to `nav-graph-v3`.
- **F154.2 Candidate validation.** Portal candidates require mutual-
  nearest one-to-one correspondence, near-opposing edge directions,
  overlapping portal intervals, and vertical/step clearance within the
  agent's step height. Pure prepare-side logic, deterministic,
  unit- and cucumber-tested on synthetic meshes. Rejected candidates
  log a stable diagnostic line with the rejection reason.
- **F154.3 Interval-based links with distance cost.** Runtime
  `landmass_graph` merge conversion builds animation links between
  portal-interval points (not triangle centroids) with cost from
  traversal distance.
- **F154.4 KCC-swept traversal.** Merge-link traversal moves the agent
  with the swept KCC toward the far portal point instead of the 0.6 s
  raw transform lerp; a traversal that collision-blocks fails the
  route visibly (feeds the existing stuck/blocked reporting) instead of
  teleporting through geometry.

### Files owned

`src/vsa/prepare/nav_graph.rs`, `src/viewer/nav/landmass_graph.rs`,
`src/viewer/nav/agent.rs` (merge-traversal section),
`src/viewer/nav/mod.rs` (merge input plumbing),
`features/nav_portals.feature` (new), own delimited section of
`tests/features.rs`.

### Acceptance (orchestrator, real data)

FranklinMetro02 (0001a273): `nav graph` prepare diagnostics show
validated portal counts (with rejects listed); the wave-5 section-F
route still finds its cross-mesh route; no agent teleports through
collision at a seam (swept traversal). Vault 101 Entrance (00024512)
routes unchanged-or-better.

## Issue #155 — doors as conditional route topology (after #154 lands)

### Feature list

- **F155.1 Door polygons are typed.** Door-associated triangles get
  per-door `polygon_type_indices` in the `landmass_graph` conversion
  (landmass 0.9.2 validates one type index per polygon; per-agent
  `override_type_index_cost` accepts any cost > 0.0).
- **F155.2 Query-time lock exclusion.** A locked door's type index is
  cost-overridden to an effectively-impassable value for every agent
  before pathing, and restored on unlock; repath after a lock change
  discovers the alternate route or fails fast. Invariant test: locking
  a door selects an alternate route when one exists; with no
  alternative the route is unreachable at query time (no walk-and-wait).
- **F155.3 Corridor-based gating.** Mid-route door gating triggers when
  the agent's corridor actually enters a door-typed polygon (current or
  imminent traversal polygon), replacing the 0.75 m
  centroid-proximity scan and its list-order ambiguity.
- **F155.4 Distinct failure status.** `resolve_status` maps failed door
  lifecycles to a failure status (not `Paused`); `tna status`, the HUD,
  and the log agree. Unit test alongside the existing
  `resolve_status_*` tests.

### Files owned

`src/viewer/nav/agent.rs` (door gating + status),
`src/viewer/nav/landmass_graph.rs` (polygon types),
`src/viewer/nav/door_link.rs` if state additions are needed,
`features/nav_door_topology.feature` (new), own delimited section of
`tests/features.rs`.

### Acceptance (orchestrator, real data)

Vault 101 Entrance (00024512): with door 00028579 locked via console,
`tna goto` across it reports failure/unreachable at query time instead
of walking to the door and waiting; unlocking and reissuing completes
`reached`. Routes merely passing near (not through) a doorway no longer
gate.

## Issue #157 — stuck detection measures corridor progress (parallel worktree)

### Feature list

- **F157.1 Corridor-progress observation.** `decide_stuck` consumes
  progress along the route — distance to the current steering
  waypoint, decreasing remaining path length, or windowed net
  displacement (executor picks the simplest signal available from
  landmass/KCC state and documents why) — instead of monotone
  best-distance-to-final-target.
- **F157.2 U-shaped invariant.** Synthetic U-shaped route (target
  behind a wall, detour initially moves away from the target)
  completes without any stuck-recovery activation. The existing
  wedge/blocked scenarios still latch `stuck` in the same tick budget.
- **F157.3 Arrival semantics preserved.** The wave-6
  `arrival_resets_stuck` behavior (trusting landmass
  `ReachedTarget`) is untouched and still tested.

### Files owned

`src/viewer/nav/movement_policy.rs`, `src/viewer/nav/agent.rs`
(**only** the stuck-observation block, ~lines 1630–1730 — nothing in
door/merge traversal), `features/nav_stuck_progress.feature` (new), own
delimited section of `tests/features.rs`.

### Acceptance (orchestrator, real data)

Vault 101 Entrance: a doorway route that detours away from the final
target completes with `stuck=false`; the #148 FranklinMetro02 wedge
route still ends `blocked=true` (genuine wedge is still detected).

## Gates

`cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`,
`cargo test`, representative `cargo run-dev -- prepare` +
real-data acceptance above. Wave manual: `M4_WAVE8_MANUAL.md` before
the PR. One PR closes #154, #155, #157; #148 re-measured and commented
after merge.

## Shipped amendments

- **A1 — External architecture review (2026-07-19, mid-wave).** A Codex
  (gpt-5.6-sol xhigh) audit of epic #9 endorsed the wave direction
  (keep landmass; authored semantics over geometric guessing; physics
  authoritative) and produced corrections that were verified by the
  orchestrator and dispatched to the running executors. Rejected from
  the review after verification: pulling NVTR evidence into #154
  mid-flight (unproven mapping — stays #156), and mandating literal
  `f32::INFINITY` lock costs for #155 (landmass only validates
  `cost > 0.0`; `0 × inf = NaN` risk — requirement recorded as
  "verified-safe exclusion semantics" on #155 instead).
- **A2 — F154.2 respecified.** "Mutual-nearest one-to-one" replaced by
  reciprocal, non-overlapping portal intervals with one-to-many edge
  subdivision (one long edge may match several short tessellated
  edges); shipped as full pairwise candidate generation + greedy
  longest-overlap-first resolution. Agent-class constants (step height)
  moved out of the universal prepared graph into the runtime landmass
  conversion (`MERGE_PORTAL_STEP_HEIGHT`). Adversarial fixtures added:
  parallel walls, stacked floors, mis-wound edges, one-long-to-two-short
  subdivision.
- **A3 — F154.4 quarantine scope.** Full per-link quarantine needs
  landmass-side per-link exclusion that does not exist for merge links;
  shipped minimum-viable mitigation clears the agent's route/travel
  intent on a blocked crossing (no repath loop, agent idles). Follow-up
  filed as its own issue rather than expanding the wave.
- **A4 — F157.1 option struck.** "Windowed net displacement" removed as
  a candidate signal per the review (oscillation = displacement without
  progress). Shipped signal: achieved velocity projected onto landmass's
  current steering direction, integrated; its own oscillation ceiling
  (fully-achieved oscillating steering never latches stuck) is
  documented in `movement_policy.rs` and pinned by a named
  known-limitation scenario, alongside avoidance-pause and
  repath-rebaseline pins.
- **A5 — Model routing (recorded per AGENTS.md).** Claude runtime:
  Fable orchestrator, Sonnet executors for #154/#155/#157. The review's
  Sol X-High recommendation applies to the Codex runtime only.
- **A6 — Real-data flag for acceptance.** FranklinMetro02 prepare:
  `merges 11 (rejected 89)`. Two `tna goto` targets across the merge
  cluster near (-15..-21, 103.3, -57..-59) ended
  `nav agent portal blocked` (swept crossing timeout) — correct failure
  reporting (no teleport), but the crossings did not complete. To
  investigate before sign-off: genuinely unwalkable seam accepted by
  geometric validation (evidence for #156's authored-NVTR refinement)
  vs. straight-line sweep limitation.
- **A7 — #163 `setlock` joined the wave.** Neither prepared cell has a
  usable authored-locked in-cell door, so #155's lock flow had no
  drivable runtime surface — the AGENTS.md visibility rule added
  `setlock <reference> <level>` (GECK `lock`/`unlock` parity) as its own
  sub-issue, updating both lock-state consumers (nav
  `door_lock_info`, interaction `PlacementRoot`) from one command.
- **A8 — Real-data acceptance outcomes.** (1) A6's blocked crossings
  root-caused: geometric validation accepted portals spanning 1.1–2.0 m
  of *empty space* — one led into literal void (agent fell out of the
  world). Fixed in-wave via #154's originally-specified capsule-sweep
  validation, implemented at runtime link building (`move_mover` slide +
  step-height ground probes): 10 of 11 FranklinMetro02 candidates drop
  with logged reasons, the genuine ~0.5 m seam survives, and the
  cross-mesh route persists through it. (2) #148 re-measured on both
  cells post-rework: identical wedge points (metro x=9.90; Vault
  stair-top (154.29, 39.61, -80.38)) — interior-collider hypothesis
  stands; both repros now have exact coordinates on the issue. (3) New
  defects found and filed rather than absorbed: #164 (walkable navmesh
  over missing collision in both cells; agents fall out of the world;
  no kill-Z guard) and #165 (travel-door arrival lifecycle hands off
  through a locked door — the setlock A-B-A on metro door 0007f7e3
  proved lock state is ignored by travel). (4) #155's query-time
  exclusion invariants are pinned by live-Archipelago unit tests; no
  honest in-cell real-data drive exists until #148/#164 land (see
  MANUAL section C).
