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
