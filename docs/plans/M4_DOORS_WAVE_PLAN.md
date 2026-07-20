# M4 doors wave — doors as real blockers, agents that open them, and the ORCA stall (#177, #184, #148)

Wave on branch `m4-wave11-177-doors`, off `m4-wave11` (PR #182). Named
rather than numbered: the actor-animation track independently used
"wave 10" and "wave 12", so nav wave numbers past 11 collide.

Single lane, single executor, sequential — every issue reworks the same
`src/viewer/nav/agent.rs` / `door_link.rs` seam (AGENTS.md's sequential
exception). Model routing: orchestrator (Opus) plans, reviews, and runs
real-data acceptance; executor (Opus) writes all production and test code.

## Why this wave existed

The navmesh post-mortem's structural finding (`docs/postmortem/VERDICT.md`
§2.2): **one defect appeared in three costumes across four waves** because
the nav layer had no model of doors as physical blockers.

- #148 (wave 6→11): agent "wedges" at FranklinMetro02 x≈9.9. Actually the
  `tna spawn` point sits **0.041 m inside** the closed `MetroGateLoad`
  travel door; the observed 0.3 m of "walking" was depenetration.
- #172 (wave 11): Vault 101 "stair riser wedge". Actually the closed
  `VaultGearDoor` activator. Closed as premise-disproven.
- The z≈−64 stop: `VDoor01`, an ordinary in-cell door with **zero** nav
  association — the authored Bethesda NAVM only associates *load* doors.

## Issue #177 — doors are route topology

- **F177.1 Derived door→polygon associations.** New pure module
  `src/vsa/prepare/nav_doors.rs`: a blocker's footprint is the XZ convex
  hull of its cooked collision under the placement transform; walkable
  polygons overlapping it (separating-axis, storey-guarded) are
  associated. Two classes — *gate* (overlaps; stays routable, feeds the
  crossing gate) and *blocking* (wholly inside the solid). Authored
  associations always win over derived reclassification. Flat door-leaf
  collision is thickened to a real footprint (zero-thickness planes hulled
  to collinear points otherwise).
- **F177.2 Typed and priced.** Blocking associations get their own
  landmass type index, priced through the existing per-agent override
  path: **closed + openable = 1000.0 (finite)**, **locked or not openable
  = INFINITY**, **open = no override**. The finite closed cost is the
  crux — an unbounded one prevents the agent ever reaching the door it is
  supposed to open.
- **F177.3 Lifecycle for non-travel doors.** The pause→open→wait→resume
  FSM was already generic; it was fed only from
  `travel_door_destinations`. Derived gate associations now feed
  `mid_route_doors`, so ordinary doors get the same lifecycle.
- **F177.4 Approach gate.** `point_in_door_triangle` containment can be
  starved — an agent stalling short never enters the crossing polygon.
  `door_link::approach_gate` adds a second trigger requiring **all three**
  of: no progress, crossing within 3.0 m, and the crossing closer to the
  target than the agent (so a door beside or behind it can never fire).
- **F177.5 Console parity.** `activate <door>` toggles an ordinary door
  through the same scripted boundary the agent uses.
- `NAV_GRAPH_REVISION` → `nav-graph-v8`.

## Issue #184 — the ORCA stall

Root cause: **`landmass`'s navmesh-border avoidance**, not the KCC.
`nav_mesh_borders_to_dodgy_obstacles` projects 3D border edges into 2D;
border edges from the staircase *below* the agent (y 39.17–40.47, finely
slivered by #171's clip) became hard velocity constraints on the landing.
Fingerprint: desired velocity decayed 2.500 → 0.759 → 0.218 → 0.038 at a
constant 0.9375/tick = `1 − dt/obstacle_horizon` = 1 − (1/64)/0.25,
reproduced to four digits across horizon sweeps.

Fix: border avoidance disabled via a shared `archipelago_options()`;
agent-to-agent avoidance keeps landmass defaults. Justification is
independent of this cell — since #114 movement is physics-authoritative
and since #153/#171 the prepared boundary *is* the agent-radius clearance
boundary, border avoidance was a redundant second wall-avoidance
authority acting in velocity space, which is exactly why four waves of
collision diagnostics could not see it.

Also: `decide_collision_outcome` conflated "wedged" with "no motion".
The rising-edge report now carries `reason=obstructed` vs
`reason=no_contact_no_progress`.

## Real-data acceptance (orchestrator, 00024512)

- Agent spawned between the vault door and `VDoor01`, `tna goto 153.6 40.3 -60`:
  walks −70.2 → −68.1 → −65.9 → −64.5 (crossing the old stall line),
  logs `door Door (00024657) opened (scripted, nav agent)` →
  `door wait` → `door resume`, continues −63.7 → −61.7, ends
  **`status=reached`**. ✔
- `setlock 00024657 25` → same route **`unreachable`**, door never
  opened. ✔
- Full-corridor route from the default spawn is **correctly**
  `unreachable`: it must cross the closed, non-openable `VaultGearDoor`.
  Before this wave the agent walked straight through it. ✔
- Prepare: `nav doors: blockers 13, associations 422 (blocking 219)
  across 9 blocker(s), unreported interior polygons 0` (00024512);
  `blockers 33, associations 350 (blocking 7) across 22, unreported 0`
  (0001a273). `VDoor01` went from **zero** associations to 62/28 blocking.

## Shipped amendments

- **A1 — #148 does not close here.** Its root cause is established
  (`MetroGateLoad`) and its stated acceptance criterion is void: the route
  it specifies crosses a closed travel door, so "ends `reached`" was never
  correct. Its remaining behaviour is #185's (key-aware) and #186's.
- **A2 — the fix is an allow-list, not an invariant.** Blocking keys on
  `PreparedSemantic::Door` plus a kinematic class. Post-mortem §2.2 argues
  the general rule (*any solid placement that blocks the capsule is route
  topology*) should be stated and enforced instead; #186 is the fourth
  instance of that gap.
- **A3 — post-mortem included.** `docs/postmortem/` (four analyses plus
  `VERDICT.md`) ships with this wave; its actions are tracked as #186,
  #188, #189, #190 and comments on #179/#185.
- **A4 — process gap, corrected here.** ~2,700 lines landed before this
  plan and manual existed, which analyst 2 flagged as the wave's own
  deviation from AGENTS.md.
