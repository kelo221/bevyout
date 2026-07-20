# M4 wave 10 — manual acceptance script

What this wave shipped, in plain language: the prepared navmesh is now
validated against the cell's real collision — triangles with no floor
under them are removed, corridors genuinely narrower than the agent are
disconnected up front, and a connectivity diagnostic proves the mesh
stays whole (99–100% on the test cells). The old interim runtime erosion
is deleted outright. Locked doors now behave correctly in three more
ways: preferred-path polygons carry a cheaper routing cost, a `setlock`
issued before any agent exists is no longer lost, and an agent that
fails a door wait no longer freezes forever. Blocked seam portals now
quarantine just the bad link instead of abandoning the route. Two
hard geometry/physics classes were measured, root-caused, and split to
follow-ups #171 (sub-triangle clearance: the metro entrance frame and
the restroom void edge) and #172 (KCC stair risers) — their standing
wedges are documented below as *expected* outcomes, not regressions.

## 0. One-time setup

```
cargo run-dev -- prepare --cell 0001a273
cargo run-dev -- prepare --cell 00024512
```

Expected: each prints a `nav clearance:` line, e.g.:

```
nav clearance: collision triangles 58352, meshes 2, removed unsupported 12, cut obstructed 10, dropped unfit 9, walkable 1307, smallest largest-component share 99%
nav clearance: collision triangles 5055, meshes 1, removed unsupported 0, cut obstructed 1, dropped unfit 1, walkable 291, smallest largest-component share 100%
```

The `smallest largest-component share` percentage is the wave's headline
guarantee: validation never shreds the mesh (≥95%).

## A. #169 — early `setlock` is no longer lost

1. Launch FranklinMetro02:
   `cargo run-dev -- view --manifest .bevyout/cache/scenes/0001a273/scene.ron`
2. In the console, FIRST thing after load (before any `tna` command):
   `setlock 0007f7e3 0`.
3. `player.setpos x 90`, `player.setpos y 96.2`, `player.setpos z -144`,
   then `tna spawn`, then `tna travel 0007f7e3`.
4. Expected: `nav agent 0 handed off to cell 0001a280` within ~6 s.
   (Before this wave, step 2's unlock was silently lost and this exact
   sequence ended `unreachable state=NoPath` — issue #169.)

## B. #165-adjacent fix — a failed door wait no longer freezes the agent

5. `tna spawn`, `setlock 0007f7e3 25`, `tna travel 0007f7e3`; poll
   `tna status` until it settles at `unreachable` (~3 s of `paused`
   first — that is the wait window).
6. `setlock 0007f7e3 0`, then `tna travel 0007f7e3`.
   Expected: `handed off`. (Before this wave the agent stayed `paused`
   forever after step 5 — the `PauseAgent` component leaked on the
   failed terminal.)

## C. #153 — collision validation, disconnect semantics, fall-guard interplay

7. Step 0's `nav clearance` lines are the primary surface; `tnm` in the
   viewer shows the surviving walkable triangles.
8. Restroom void (known #171 residual, guard backstops it):
   `player.setpos x -16.6`, `player.setpos y 103.4`,
   `player.setpos z -57.2`, `tna spawn`, `tna goto -15 103.3 -57`.
   Expected: the agent steps onto the void edge and exactly one
   `nav agent fell out of world … kill_z=…` warning fires — no infinite
   fall. (Query-time removal of this straddling triangle needs the #171
   sub-triangle clip.)
9. Sub-diameter invariant (synthetic): `cargo test nav_clearance` — the
   `a_one_metre_doorway_stays_connected…` and
   `a_sub_diameter_pinch_disconnects…` tests are the doorway-regression
   guards.

## D. Known wedges, expected and tracked (not regressions)

10. #148 / #171: on 0001a273, `player.setpos x 9.6` / `y 106` /
    `z -73.1`, `tna spawn`, `tna goto -19 103.4 -59.5`. Expected: the
    agent walks ~0.5 m and ends `stuck=true blocked=true` near
    (9.90, 106.05, −73.84) — the `MetHallEntrance01` frame posts, which
    triangle-level clearance provably cannot cut (see #171).
11. #172: on 00024512, `tna spawn`, `tna goto 152.5 36.6 -37`. Expected:
    the agent descends the stairs and wedges near (154.4, 39.4, −80.4)
    — the authored riser seam the KCC cannot yet climb. The route being
    *found* (status `moving`, not `unreachable`) is this wave's
    guarantee: no wave-6-style over-shrink regression.

## E. #162 / #168 — no player-visible surface on the test cells

12. #162 (portal quarantine): both test cells' surviving portals are
    physics-validated, so no blocked link exists to demonstrate live;
    the behavior is pinned by `features/nav_portal_quarantine.feature`
    and the `nav agent portal quarantined <id> link=<kind>` log line
    fires on a real timeout. #168 (preferred-path cost 0.5): active on
    every archipelago build; visible routing differences await cells
    with authored preferred-path flags. Both are unit/cucumber-pinned.
