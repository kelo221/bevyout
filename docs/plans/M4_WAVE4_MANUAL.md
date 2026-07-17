# M4 wave 4 manual acceptance — travel doors and intercell agents (#113, #134)

## What this wave shipped

The test nav agent can now do the two things wave 3 could not: cross
between a cell's separate nav meshes (they used to be disconnected
islands — a target on the "other" mesh reported `NoPath`), and actually
use a travel door. `tna travel <door-formid>` sends the agent to a door;
it waits for the door to open, crosses, and *leaves the cell* — when you
follow through the same door, it is standing at the destination marker.
Agents you leave behind in a cell are no longer deleted on cell change:
they freeze in place and are restored exactly where they stood when you
come back. Under the hood the NAVI record's undocumented NVMI tail was
reverse-engineered, cross-mesh seams are generated geometrically, doors
are off-mesh links gated by live door state (a locked door is a blocked
link), and a pure ledger carries agents across cell swaps.

## One-time setup

The three cells below must be prepared once (skip any whose
`.bevyout/cache/scenes/<formid>/navmesh/navgraph.ron` already exists —
but re-prepare all three if it predates this wave, the graph gained new
fields):

```
cargo run-dev -- prepare --cell 00024512
cargo run-dev -- prepare --cell 00024511
cargo run-dev -- prepare --cell 0001a273
```

Expected in each prepare output: a `nav graph:` line; for 0001a273 it
reports `merges 13`, for 00024511 `merges 3`.

## Steps

### A. Travel door + follow-through: Vault 101 Entrance (00024512)

1. Launch the viewer:
   `cargo run-dev -- view --manifest .bevyout/cache/scenes/00024512/scene.ron`
2. Open the console (backtick) and type `tna spawn`. Expected:
   `nav agent spawned at (…)` and a cyan capsule at your position.
3. Type `tna travel 00028579` and close the console. Expected:
   `nav agent travel requested to door 00028579`; the capsule walks to
   the vault-interior door, the door opens (`nav agent door wait
   00028579` then `door resume` in the log), the capsule crosses and
   disappears. Log shows `nav agent travel reached 00028579 -> cell
   00024511` and `nav agent handoff 00000001 -> cell 00024511`.
4. Open the console; type `tna status`. Expected:
   `nav agent handed off to cell 00024511`.
5. Type `activate 00028579` to follow it. Expected: the cell swaps to
   Vault 101 Atrium; the log shows `nav agent restore 00000001 cell
   00024511` and the capsule is standing at the door you just came
   through (destination marker of door 0005398d).
6. Type `tna status`. Expected: `status=idle
   position=(-64.46,117.03,-32.91)` (±0.1).

### B. Freeze in place and restore: Vault 101 Atrium (00024511)

7. Still in the Atrium with the agent idle: type `activate 0005398d` to
   go back without it. Expected: cell swaps back to the Entrance; log
   shows `nav agent freeze 00000001 cell 00024511` (frozen, not
   deleted).
8. Type `activate 00028579` to return to the Atrium. Expected: log shows
   `nav agent restore 00000001 cell 00024511`; `tna status` reports the
   *identical* position as step 6 — the agent never moved.
9. Type `tna despawn`, then quit the viewer.

### C. Cross-mesh route: FranklinMetro02 (0001a273)

This cell's two nav meshes were disconnected islands in wave 3; the same
route below used to report `NoPath`.

10. Launch:
    `cargo run-dev -- view --manifest .bevyout/cache/scenes/0001a273/scene.ron`
11. Open the console. The player start here stands just off the nav
    mesh, so move onto it first — either walk a couple of metres toward
    the platform edge before spawning, or type exactly:
    `player.setpos x 9.6` then `player.setpos y 106` then
    `player.setpos z -73.1`.
12. Type `tna spawn`. Expected: spawned at your position, no
    `AgentNotOnNavMesh` in the log.
13. Type `tna goto -19 103.4 -59.5` and close the console. Expected: the
    capsule walks off the platform area, through the connecting corridor
    onto the *other* nav mesh (~35 m), and stops. Log shows
    `nav agent path latency_ms=…` (≈16 ms) and `nav agent reached`.
14. Optional: `tnm` shows the two meshes' triangles; the route you just
    watched crossed the seam between them.

### D. Blocked door (log-only check)

15. Still in FranklinMetro02, the startup log lists this cell's travel
    doors, including `nav agent travel door 0007f7e3 -> cell 0001a280`
    — that door is locked (lock level 25) and is treated as a blocked
    link: `tna travel 0007f7e3` routes the agent to the door, where the
    lifecycle deterministically fails rather than crossing (status
    becomes `unreachable`; the agent stops at the door). The cells
    behind FranklinMetro02's doors are unprepared, so the full handoff
    is only demonstrable on the Vault 101 pair above.

## Known limits

- Vault 101 Atrium (00024511) has one nav mesh (0007350c) that fails
  landmass validation (concave polygon 177, pre-existing data gap
  logged at build); its own travel doors sit on that mesh, so drive
  `tna travel` from the Entrance side, as above.
- Door-marker restores arrive idle (the door was the route's terminus;
  multi-hop cross-cell routes are out of scope this wave). Frozen
  restores resume a coordinate target; a `goto player` target is
  dropped at freeze.
- Cell 00028138 (7 meshes) generates 0 merges — its meshes are
  genuinely 2.06–140 m apart, beyond the 2 m seam threshold.
