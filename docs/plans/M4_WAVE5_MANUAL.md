# M4 wave 5 manual acceptance — grounded movement, door gating, tnm path (#114, #137, #138)

## What this wave shipped

The nav test agent is now **physics-authoritative**: instead of gliding
along the navmesh, it walks with a real capsule that collides with the
world, stays grounded on floors/steps, and is driven on a fixed 64 Hz
clock (the same clock as the player), so its motion no longer depends on
your frame rate. On top of that:

- **Closed doors now stop the agent** (#137). Routing an agent through a
  closed door makes it pause, request the door open, wait, and cross once
  it opens — it never walks through a shut door. A locked door it cannot
  open stops it deterministically instead of clipping through.
- **Agents steer around the player** (#114). The player is registered as a
  navigation obstacle, so agents use local avoidance (RVO) to go around
  you rather than relying on bumping your collider.
- **The solve rate is a knob** (`tna solverate <n>`). The navigation solve
  (pathfinding + steering + avoidance) can run every N fixed ticks instead
  of every tick; movement still runs every tick, and the steering is
  interpolated between solves so throttling stays smooth. Default is every
  tick (unchanged behavior).
- **`tnm` draws the agent's route** and no longer blacks out dark interiors
  (#138).

Known limitation shipped with this wave: at tight constriction points the
agent can wedge against wall/prop colliders the un-eroded FO3 navmesh
routes it too close to (it used to clip through; now it correctly refuses
to). That agent-radius clearance is tracked in **#136** and is the next
task — the door tests below deliberately start the agent right by the door
to avoid the long wedge-prone corridor.

## One-time setup

These cells must be prepared once (skip any whose
`.bevyout/cache/scenes/<formid>/navmesh/navgraph.ron` already exists from
M4 wave 4 — the nav graph shape did not change this wave):

```
cargo run-dev -- prepare --cell 00024512
cargo run-dev -- prepare --cell 00024511
cargo run-dev -- prepare --cell 0001a273
```

## Steps

### A. Grounded movement on an open route — Vault 101 Entrance (00024512)

1. Launch with the agent bridge (the console commands below can also be
   sent over the bridge per AGENTS.md, but the keyboard console is fine):
   `cargo run-dev -- view --manifest .bevyout/cache/scenes/00024512/scene.ron`
2. Open the console (backtick). Place the player onto the flat area by the
   vault door and spawn the agent there:
   `player.setpos x 153.2`, `player.setpos y 37.6`, `player.setpos z -40`,
   then `tna spawn`. Expected: `nav agent 0 spawned at (153.20, 37.59,
   -40.00)`.
3. Move the player out of the way so it does not block the agent:
   `player.setpos x 175`.
4. `tna goto 152 36.5 -38`, close the console. Expected: the capsule walks
   the short flat stretch and stops; `tna status` reports `status=reached
   grounded=true stuck=false`. This confirms grounded movement reaches a
   target and does not false-report stuck.

### B. Closed door gates a crossing route (#137) — same cell

5. Re-place the agent on the door's approach side:
   `tna despawn`, `player.setpos x 153.2`, `player.setpos y 37.6`,
   `player.setpos z -40`, `tna spawn`, then `player.setpos x 175`.
6. `tna goto 154 36.5 -34`, close the console. This routes the agent
   *through* the closed vault door 00028579 (an ordinary crossing, not a
   travel). Expected in the log, in order:
   `nav agent door wait 00028579` → `nav agent door resume 00028579`;
   the door visibly opens and the capsule crosses to the far side and
   stays **in this cell** (no handoff). This is the #137 fix: before it,
   the agent clipped straight through the shut door.

### C. Travel door + follow-through (#113/#134 re-verify) — same cell

7. `tna despawn`, then place on the approach again:
   `player.setpos x 153.2`, `player.setpos y 37.6`, `player.setpos z -41`,
   `tna spawn`, `player.setpos x 175`.
8. `tna travel 00028579`, close the console. Expected log:
   `nav agent door wait 00028579` → `door resume 00028579` →
   `nav agent travel reached 00028579 -> cell 00024511` →
   `nav agent handoff 00000001 -> cell 00024511`; `tna status` reports
   `handed off to cell 00024511`.
9. `activate 00028579` to follow through. Expected: the cell swaps to
   Vault 101 Atrium and `nav agent restore 00000001 cell 00024511` — the
   agent followed you through the door. (This is the wave-4 behavior; it
   is re-listed because wave 5's movement rework briefly regressed the
   door-arrival distance check, now fixed — this step is the guard.)

### D. Player avoidance (#114) — Vault 101 Atrium or Entrance

10. With an agent idle near you and given a `tna goto` target on the far
    side of your position, walk the player slowly across the capsule's
    path. Expected: the agent steers around you (a visible arc/detour)
    rather than shoving straight into your collider. The steering is soft
    (RVO), so your physical capsule remains the hard backstop if you
    corner it.

### E. Solve-rate knob (#114) — same cell

11. `tna solverate 2` (get the current value with a bare `tna solverate`).
    Expected: confirmation the solve now runs every 2 ticks. Give the
    agent a `tna goto`; it still moves smoothly every tick (the steering
    is interpolated between the halved-rate solves). `tna solverate 1`
    restores every-tick solving.

### F. tnm route overlay + brightness (#138) — FranklinMetro02 (0001a273)

12. Launch:
    `cargo run-dev -- view --manifest .bevyout/cache/scenes/0001a273/scene.ron`
13. The player start is just off the mesh here; move on first:
    `player.setpos x 9.6`, `player.setpos y 106`, `player.setpos z -73.1`,
    then `tna spawn`.
14. `tna goto -19 103.4 -59.5`, then `tnm`. Expected: the mesh triangles
    appear **and** a distinct white polyline shows the agent's route
    crossing the seam between the two meshes.
15. Toggle `tnm` off and on while facing a dark corridor. Expected: the
    overlay no longer washes the scene bright enough to crush the dark
    interior to black (the #138 brightness fix).

### G. Locked door is deterministically blocked (#137/#113) — FranklinMetro02

16. This cell's startup log lists `nav agent travel door 0007f7e3 -> cell
    0001a280`; that door is locked (lock level 25). `tna travel 0007f7e3`
    routes the agent to it and the lifecycle deterministically fails
    (status `unreachable`) — the agent stops at the door rather than
    clipping through. (Cells behind FranklinMetro02's doors are unprepared,
    so the full handoff is only demonstrable on the Vault 101 pair above.)

## Known limits

- **Corridor wedge at constrictions (#136, next task).** The agent can
  wedge (`collision-blocked` then stuck) where the un-eroded navmesh routes
  the 0.35 m capsule within less than agent-radius of a collider — e.g. the
  long route from the Entrance player start down to the vault door. Start
  the agent near the door for the door tests, as above. Fix is navmesh
  erosion by agent radius, tracked in #136.
- Frame-time numbers are only comparable on a cool machine (the startup
  `BoxDDD prepared collision ... cook` line is the canary: ~10 ms cool,
  20 ms+ means thermally degraded).
- `capture_viewport` returns black PNGs when the window is occluded on
  macOS; use the console `tna status` and the log lines above as evidence.
