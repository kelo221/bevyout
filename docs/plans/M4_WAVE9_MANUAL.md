# M4 wave 9 — manual acceptance script

What this wave shipped, in plain language: nav agents can no longer be
handed through a **locked** travel door — not when it is closed, and not
when an earlier traveller left it standing open; a locked travel now ends
in a calm, stable "unreachable" instead of sneaking through or flickering
between paused and unreachable. Agents that walk onto navmesh with no
floor under it (a real data defect in FranklinMetro02) no longer fall
forever — a guard despawns them with one log line. The prepared nav graph
now carries the authored NAVM semantics Bethesda shipped (preferred-path
polygon types, per-edge external-edge evidence, NVEX/NVCI correlation
diagnostics). The two long-standing FranklinMetro02/Vault-101 wedge and
pit defects were root-caused to authored game data (details on #164 and
#148), which sets the direction for the upcoming #153 clearance work.

## 0. One-time setup

The nav graph revision changed (`nav-graph-v4`), so prepared caches
rebuild once:

```
cargo run-dev -- prepare --cell 0001a273
cargo run-dev -- prepare --cell 00024512
```

Expected: each prepare prints a `nav graph:` line in the new extended
format, e.g. for 0001a273:

```
nav graph: meshes 2, polygons 1338, vertices 1198, doors 3, external 0, merges 11 (rejected 89, authored 0 geometric 11, candidates authored 0 geometric 16), diagnostics warn 89 error 0, nvex correlation (outside-cell 0 inside-cell 0), nvci correlation (subrecords 7198 entries 11150 door-matches 1 navmesh-matches 2)
```

`candidates authored 0` is correct: interior cells carry no NVTR
external-edge flags (see #156's issue comment).

## A. #165 — locked travel door, closed variant

1. Launch FranklinMetro02:
   `cargo run-dev -- view --manifest .bevyout/cache/scenes/0001a273/scene.ron`
2. Open the console and move next to the door:
   `player.setpos x 90`, then `player.setpos y 96.2`, then
   `player.setpos z -144`.
3. `tna spawn` — expected: `nav agent 0 spawned at (90.00, 96.20, -144.00)`.
   (Spawn **before** touching `setlock`: an early `setlock` in a fresh
   session is lost for query-time lock exclusion — known follow-up #169.)
4. `setlock 0007f7e3 25` — expected: `setlock 0007f7e3 level 25`.
5. `tna travel 0007f7e3`, then repeat `tna status` for ~5 s.
   Expected: at most a brief `paused ... link=door 0007f7e3` while the
   agent waits at the door, then a **stable** `status=unreachable
   target=none` that does not change on repeated polls. The agent never
   hands off; the log shows `gave up waiting for it to open` and never a
   scripted door-open line.

## B. #165 — locked travel door, left-open variant (the wave-8 bug)

6. `setlock 0007f7e3 0`, then `tna travel 0007f7e3`, poll `tna status`.
   Expected: `nav agent 0 handed off to cell 0001a280` within ~6 s. The
   door visibly opens and stays open.
7. `setlock 0007f7e3 25`, `tna spawn`, `tna travel 0007f7e3`, poll
   `tna status` for ~5 s.
   Expected: same stable `unreachable` terminal as step 5 even though
   the door is standing open — no hand-off, no walking through.
8. `setlock 0007f7e3 0`, `tna travel 0007f7e3`.
   Expected: `handed off to cell 0001a280`. This completes the issue's
   A-B-A invariant: handed-off / unreachable / handed-off.

## C. #164 — fall-out-of-world guard

9. Relaunch the 0001a273 viewer (fresh session).
10. `player.setpos x -16.31`, `player.setpos y 103.4`,
    `player.setpos z -57.26`, then `tna spawn`.
11. Watch `tna status` / the log for a few seconds. The agent stands on
    walkable navmesh that has no collision floor (authored data defect,
    see #164) and falls. Expected: exactly one warning line —

    ```
    nav agent fell out of world 0 y=<about 89> kill_z=89.16826
    ```

    — the agent entity is despawned (no endlessly descending Y), and a
    subsequent `tna spawn` works normally.

## D. #156 — authored NAVM semantics visible surface

12. The step-0 prepare lines are the acceptance surface: the extended
    `nav graph:` diagnostics (authored/geometric candidate split, nvex
    and nvci correlation counts) match the values printed above for both
    cells. Optionally launch either cell and run `tnm` — the nav overlay
    renders exactly as before (no runtime behavior change is expected
    from #156; the preferred-path base cost lands with #168).

## E. #148 — status check (stays open by design)

13. Optional: on 0001a273, `tna spawn` after `player.setpos x 9.6` /
    `y 106` / `z -73.1`, then `tna goto -19 103.4 -59.5`. Expected
    (unchanged, now explained): the agent wedges near x≈9.90 with
    `blocked=true stuck=true` — root-caused to the authored
    `MetHallEntrance01` collision overlapping the NAVM (see #148's
    wave-9 comment). The fix direction belongs to #153.
