# M4 wave 11 — manual acceptance script

What this wave shipped, in plain language: the prepared navmesh is now cut
against real collision at **sub-triangle** resolution — a polygon is split
exactly along a wall's radius-expanded footprint or a void's edge, instead
of whole triangles being kept or dropped. Prepare now runs *landmass's own
validator* and **fails the build** if the mesh it produced would be
rejected at runtime, which caught a real defect that every connectivity
metric had reported as healthy. AI packages gained their data foundation:
every authored `PACK` record is staged into a prepared catalog and can be
inspected in the viewer with the new `showpackages` command. And the
viewer no longer freezes when its window loses focus — the single change
that most improves day-to-day testing on macOS.

Two long-standing "nav defects" were disproven this wave and re-filed as
what they actually are (see §E).

## 0. One-time setup

```
cargo run-dev -- prepare --cell 0001a273
cargo run-dev -- prepare --cell 00024512
```

Expected `nav clearance:` lines (the key fields are `invalid rejected 0`
and the two share percentages):

```
… polygons 58068 (clipped 1132, vertices +29306, slivers welded 311 via 162 weld(s), invalid rejected 0), … smallest largest-component share 90%, smallest authored-reachable share 87%
… polygons 11939 (clipped 258, vertices +5998, slivers welded 106 via 55 weld(s), invalid rejected 0), … smallest largest-component share 98%, smallest authored-reachable share 95%
```

`0001a273`'s 90/87% is a property of that cell's **broken authored
collision** (confirmed: the player itself falls through the world at
(8.5, 105.1, −73)), not of the clearance pass. Vault 101's 98/95% is the
number to watch for regressions.

Also expected, from the package catalog:

```
package catalog: 3021 packages, 0 unsupported type, 0 unsupported subrecord, 3021 deferred subrecord, 0 unresolved location, 3 unresolved target, 2356 out-of-scope location, 715 out-of-scope target -> …
```

## A. #180 — the viewer no longer steals focus or freezes when unfocused

1. Launch with the new flag:
   ```
   cargo run-dev -- view --manifest .bevyout/cache/scenes/00024512/scene.ron \
       --agent-bridge --agent-port 15702 --unfocused
   ```
2. Expected: the window appears **without** taking focus or jumping in
   front of what you were doing. (`--agent-bridge` implies this even
   without the flag.)
3. Click into another application so the viewer is fully covered, then
   poll the bridge repeatedly for a minute:
   ```
   curl -s -X POST http://127.0.0.1:15702/ -H 'Content-Type: application/json' \
       -d '{"jsonrpc":"2.0","id":1,"method":"bevyout.session","params":{}}'
   ```
   Expected: **every** call answers. Measured on this branch: 12/12 polls
   over 60 s. Before this wave the bridge stopped responding entirely the
   moment the window lost focus, for as long as it stayed unfocused.

## B. #171 — the navmesh is valid and agents traverse it

4. In that same Vault 101 viewer: `tna spawn`.
   Expected: `nav agent 0 spawned at (154.66, 41.10, -108.22)`.
   (Before the final fix this failed with `nav_mesh_invalid: prepared nav
   graph produced no valid landmass islands` — the whole cell had no
   navigation while prepare reported 98% connectivity. That is the defect
   the new validator gate now makes impossible to ship.)
5. `tna goto 152.5 36.6 -37`, then poll `tna status`.
   Expected: the agent descends the stairs and travels roughly 44 m, from
   z ≈ −108 to z ≈ −64.5. It then stops at (154.10, 41.13, −64.47) —
   **expected and tracked**, see §E.

## C. #175 / #176 — AI package foundation

6. `showpackages <actor-reference-or-base-formid>` for an NPC in a
   prepared cell (use an actor FormID from the prepared actor catalog).
   Expected: one line per package **in authored priority order**, each
   with FormID, EditorID, type, schedule (month/day/date/time/duration),
   location (type/target/radius), target (type/target/count-distance) and
   condition count; a clear `has no packages` line for an actor without
   any; and a deterministic error for an unknown FormID.
7. The step-0 `package catalog:` line is the prepare-side surface. Note
   `3021 deferred subrecord`: every FO3 package authors script/idle action
   blocks (`POBA`/`POCA`/`POEA` and friends) which belong to #115/#15 —
   they are *deferred*, not unsupported. The numbers that matter are
   `0 unsupported subrecord` and `3 unresolved target`.

## D. Stair regression coverage (from the closed #172)

8. `cargo test agent_kcc` — three synthetic FO3-scale stair tests (climb,
   descend across a two-collider seam, and refusing a ledge taller than
   step height). These exist because #172's premise turned out to be
   false and the real gap was missing coverage.

## E. Known, tracked, and *not* regressions

9. **Vault 101 stop at (154.10, 41.13, −64.47)** — the runtime diagnostic
   reports a single flat blocking plane, normal (0,0,−1): a solid blocker
   the route topology does not model. Same family as the closed
   `VaultGearDoor` and `MetroGateLoad`. Tracked in **#177**. Before this
   wave the agent never got past z ≈ −80.4, so this is newly-exposed
   ground, not a regression.
10. **FranklinMetro02 route from (9.6, 106, −73.1)** — that spawn point is
    *inside the closed `MetroGateLoad` collision* (0.041 m from its face),
    so the agent is ejected by depenetration rather than walking. The old
    "walks 0.5 m then wedges" measurement was never walking. Tracked in
    **#177**; **#148** carries the full root cause.
11. **27 MB `navgraph.ron` for 0001a273** — sub-triangle clipping emits
    both sides of every cut. Loads fine (bridge ready in ~4 s) but will
    not scale to M6 exteriors. Tracked in **#179**.
