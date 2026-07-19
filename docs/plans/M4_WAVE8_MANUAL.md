# M4 wave 8 — manual acceptance script

**What this wave shipped, in plain language:** nav agents no longer
teleport through walls at navmesh seams — cross-mesh portals are now
validated against real collision (a capsule sweep plus ground-support
probes), and invalid ones are dropped with a logged reason. Locked doors
are excluded from route planning *before* an agent commits to a route
(it fails fast as `unreachable` instead of walking to the door and
waiting), driven by the new `setlock` console command. Stuck detection
now measures progress along the route's own corridor instead of
straight-line distance to the destination, so detours that temporarily
move away from the target no longer false-trigger stuck recovery. A
failed door lifecycle now reports `unreachable` instead of `paused`.

One-time setup (rebuilds the nav graph as `nav-graph-v3`):

```
cargo run-dev -- prepare --cell 00024512
cargo run-dev -- prepare --cell 0001a273
```

Expected `prepare` output lines:

- Vault 101 Entrance: `nav graph: meshes 1, polygons 293, vertices 282,
  doors 3, external 0, merges 0 (rejected 0), diagnostics warn 0 error 0`
- FranklinMetro02: `nav graph: meshes 2, polygons 1338, vertices 1198,
  doors 3, external 0, merges 11 (rejected 89), diagnostics warn 89
  error 0`

## A. Portal collision validation (#154) — FranklinMetro02

1. Launch:
   `cargo run-dev -- view --manifest .bevyout/cache/scenes/0001a273/scene.ron --agent-bridge --agent-port 15702`
2. Open the console and run `tna spawn` (any position). The first spawn
   builds the navigation archipelago; the log now shows the runtime
   portal validation dropping physically-invalid seams, e.g.
   `nav merge link mesh 0005429e triangle 64 <-> mesh 0005429f triangle
   121: dropped (no ground support)`. **Expected: 10 of the 11 prepared
   merge candidates drop** (reasons `no ground support` or `swept
   blocked`); one survives (triangle 65 <-> 223, a genuine ~0.5 m seam).
3. `tna despawn`, then `player.setpos x 9.6`, `player.setpos y 106`,
   `player.setpos z -73.1`, `tna spawn`, `tna goto -19 103.4 -59.5`.
   **Expected:** the agent finds a route (status `moving` — the
   surviving seam link keeps the two meshes connected), walks ~0.7 m and
   wedges at x≈9.90 with `stuck=true blocked=true`. This wedge is the
   known pre-existing #148 collision defect, unchanged by this wave —
   the point of this step is that a wedge is *reported*, not teleported
   past.
4. `tna despawn`, `player.setpos x -16.6`, `player.setpos y 103.4`,
   `player.setpos z -57.2`, `tna spawn`, `tna goto -13 103.3 -57.2`.
   **Expected:** `tna status` shows `unreachable` within a few seconds —
   before this wave, the agent lerp-teleported across a 1.7 m void gap
   here; the invalid portal is now dropped. (Known issue #164: this
   platform-edge area has walkable navmesh but no collision floor — the
   idle agent may fall through the world after reporting. `tna despawn`
   when done.)

## B. Corridor-progress stuck detection (#157) — Vault 101 Entrance

1. Launch:
   `cargo run-dev -- view --manifest .bevyout/cache/scenes/00024512/scene.ron --agent-bridge --agent-port 15702`
2. `tna spawn` (defaults to (154.66, 41.10, -108.22)), then
   `tna goto 154 40 -90`.
   **Expected:** `tna status` ends `status=reached stuck=false
   blocked=false` — a real route with a descent and direction changes
   completes without any false stuck latch.
3. `tna goto 154 36.5 -34`.
   **Expected:** the agent wedges at (154.29, 39.61, -80.38) with
   `stuck=true blocked=true` — the known #148 Vault stair-top collision
   block. A genuine wedge still latches stuck under the new progress
   signal.
4. The U-shaped-detour invariant (a route that must initially move away
   from its target never triggers stuck recovery) and the oscillation/
   avoidance-pause/repath edge cases are pinned by the
   `features/nav_stuck_progress.feature` cucumber scenarios (`cargo test
   --test features`).

## C. Door locks: `setlock` and query-time exclusion (#155, #163) — FranklinMetro02

1. In the FranklinMetro02 viewer: `player.setpos x 90`,
   `player.setpos y 97`, `player.setpos z -144`, `tna spawn`.
   **Expected:** agent spawns grounded near door `0007f7e3`
   (OffDoorMetalSmR02b, authored lock level 25, travel door to cell
   107136).
2. `setlock 0007f7e3 0` — **expected console output:**
   `setlock 0007f7e3 unlocked`.
3. `tna travel 0007f7e3` — **expected:** `tna status` reports
   `handed-off` with `cell: 107136` (intercell continuity, #134).
4. `tna despawn`, `tna spawn`, `setlock 0007f7e3 25` — **expected
   output:** `setlock 0007f7e3 level 25`. Then `tna travel 0007f7e3`.
   **Current result: the agent still hands off — the travel-door
   arrival lifecycle does not yet consult runtime lock state. This is
   known issue #165** (filed from this acceptance run). Once #165
   lands, the expected result becomes a deterministic `unreachable`
   failure with no hand-off.
5. The #155 route-topology invariants themselves — locking a door on
   the only route fails at query time with no walk-and-wait; an
   alternate route is selected when one exists; unlocking restores the
   direct route — are pinned by live-Archipelago unit tests in
   `src/viewer/nav/agent.rs` (`cargo test door_topology`); neither
   prepared cell has an in-cell two-sided door on collision-sound
   ground both sides, so there is no honest real-data drive for them
   yet (see #148/#164 — the two cells' physical collision defects).

## D. Failure status (#155 F155.4)

Any `unreachable` read in the steps above comes through the reworked
`resolve_status`: a failed door lifecycle and a no-route solve now both
surface as `unreachable` in `tna status` and the debug HUD (`hud`), never
as a silent `paused`.
