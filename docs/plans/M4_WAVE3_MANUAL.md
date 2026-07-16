# M4 wave 3 manual acceptance — bevy_landmass navigation spike (#112)

## What this wave shipped

The viewer can now spawn a crude test actor (a cyan capsule) that walks
from A to B across the cell's prepared navigation mesh on its own. Under
the hood this is the first navigation backend integration: the prepared
nav graph from #111 is converted into a validated bevy_landmass
navigation mesh, and landmass computes the path, smooths it, and steers
the agent. You drive it entirely from the developer console with the new
`tna` (test nav agent) command family. Door traversal machinery (pause at
a door, open it, resume) is implemented and unit-tested, but real
Fallout 3 interior data turned out to contain only single-sided (travel)
door links, so you cannot see a door crossing in this wave — that is #113
(see "Known limits" below).

## One-time setup

Both cells used below must be prepared once (skip if
`.bevyout/cache/scenes/<formid>/navmesh/navgraph.ron` already exists):

```
cargo run-dev -- prepare --cell 000151e3
cargo run-dev -- prepare --cell 0001a273
```

## Steps

### A. Flat interior: MegatonPlayerHouse (000151e3)

1. Launch the viewer:
   `cargo run-dev -- view --manifest .bevyout/cache/scenes/000151e3/scene.ron`
2. Open the console (backtick) and type `tnm`, then press Enter.
   Expected: `nav mesh visualization on (1 meshes, 183 triangles)` and the
   floor shows flat colored triangles.
3. Type `tna spawn`. Expected: `nav agent spawned at (37.94, 106.89,
   -27.89)` (coordinates ≈ wherever you are standing) and a cyan capsule
   appears at your position.
4. Type `tna goto 48 106 -26` and close the console. Expected: the
   capsule walks ~10 m across the room on the nav mesh and stops.
5. Reopen the console; type `tna status`. Expected:
   `nav agent status=reached position=(...) target=(48.00, 106.00, -26.00)`.
6. Type `tna goto 500 500 500`, wait a second, then `tna status`.
   Expected: `nav agent status=unreachable ...` — the point is off the
   nav mesh; the agent does not move.
7. Walk a few steps away, then type `tna goto player` and close the
   console. Expected: the capsule walks back to you and stops ~0.5 m
   away; `tna status` reports `status=reached`.
8. Type `tna despawn`. Expected: `nav agent despawned`; the capsule
   disappears.
9. Type `tna goto 1 2 3`. Expected error: `no test nav agent is spawned;
   use tna spawn first`.

### B. Sloped multi-mesh interior: FranklinMetro02 (0001a273)

1. Launch the viewer:
   `cargo run-dev -- view --manifest .bevyout/cache/scenes/0001a273/scene.ron`
2. Console: `tnm`. Expected: `nav mesh visualization on (2 meshes, 1338
   triangles)`.
3. `tna spawn`, then `tna goto 39.36 101.49 -55.50`, close the console.
   Expected: the capsule walks down the sloped metro corridor, tracking
   the descending floor height (y ≈ 106 → 101.5), and stops at the
   target after ~30 seconds (`tna status` → `status=reached`; the final
   half-metre is slow by design — landmass decays the approach speed).
   This is the stairs/slopes verification #112 requires.
4. `tna goto -17.92 103.30 -62.87` (a point on the cell's second, smaller
   NAVM). Expected after a second: `tna status` → `status=unreachable` —
   the two metro meshes are separate landmass islands; connecting them
   needs NAVI merge data (#113).
5. `tna despawn`.

### C. Cell-swap teardown

1. Still in a viewer with a spawned agent (`tna spawn`), use `activate
   <a travel door formid>` or load another cell. Expected: the agent and
   its archipelago despawn with the old cell; `tna status` in the new
   cell errors with `no test nav agent is spawned; use tna spawn first`,
   and a fresh `tna spawn` works if the new cell has a prepared nav
   graph.

## Known limits (by design, this wave)

- No real door crossing: every door triangle in real FO3 interior NAVM
  data is single-sided (the other side lives in another cell's NAVM,
  linked via NAVI) — there are no intra-cell two-sided door links to
  attach an animation link to. The pause/open/resume state machine is
  fully unit- and cucumber-tested; wiring it to travel doors is #113.
- The capsule is kinematic: no collision, no gravity, no stepping — it
  tracks the nav-mesh surface height only. Grounded movement is #114.
- One test agent at a time; `tna spawn` twice errors by design.
