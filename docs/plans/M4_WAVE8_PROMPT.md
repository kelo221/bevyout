# M4 wave 8 — kickoff prompt

Requested 2026-07-19, following M4 waves 6/7 (PR #149, #160) and the
external nav-architecture review that produced #153–#157:

> Time to continue on the work of the navmesh. recently we discussed the
> correctness of the implementation. I need you first to check all the
> sub issues and evaluate their correctness, too. Then I need you either
> delete or edit them. Once done, take subissues regarding the NavMesh
> and work on the parallely

Pre-wave audit outcome (2026-07-19, verified against master@72ec705):
all six open nav sub-issues (#148, #153–#157) remain factually correct
against current code — nothing deleted; re-verification evidence
commented on each issue; epic #9 checklist amended (#136/#151/#152
ticked, #153–#157 added).

Wave composition chosen by the orchestrator:

- **#154** (P1) — cross-mesh portals: preserve edge identity, validate
  candidates, sweep traversal with the KCC.
- **#155** (P1) — doors as conditional route topology: query-time lock
  exclusion, corridor-based gating, distinct failure status.
- **#157** (P2) — stuck detection measures corridor progress, not
  final-target distance.

#154 and #155 rework the same runtime seam (`src/viewer/nav/agent.rs`
door-link/portal machinery plus `landmass_graph.rs` conversion), so they
run **sequentially on the wave branch** per the AGENTS.md sequential
exception (precedent: M4 wave 4). #157's seam is
`src/viewer/nav/movement_policy.rs` plus one delimited stuck-detection
block of `agent.rs`, so it runs in a **parallel worktree**.

Deliberately deferred:

- **#153** — gated on #148's direction decision by its own scope note
  ("implement this only in whatever direction that lands").
- **#156** — shares the prepared nav-graph shape and `landmass_graph`
  seam with #154/#155; its preferred-path costs ride on the
  `polygon_type_indices` mechanism #155 introduces. Next nav wave.
- **#148** — re-measure after this wave lands (#154's swept traversal
  and #157's progress rework change the failing route's behavior)
  before concluding on the interior-collider hypothesis.
