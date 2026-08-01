# M6 wave 7 — bounded exterior route manual

This manual exercises the real six-cell Super-Duper Mart to Megaton strip and
its prefetched safety ring. It proves package preparation, collision-ready
streaming, bounded residency, and the visible exterior navigation surface. It
does not by itself close the final gate: the long-route walk, interior return,
water, actor path, and frozen performance budgets remain explicit checks.

1. Prepare the route and safety ring from the configured Fallout 3 install:

   ```text
   cargo run-dev -- prepare 00000c49 00000c4a 00000c4b 00000c4c 000010d5 00001245 00000c67 00000c68 00000c69 00000c6a 00000c6b 000010eb 000010ec 000010ed --jobs 1 --force
   ```

   Expected: every selected cell prints `prepared exterior` and the command
   ends with `14 done, 0 failed`. The two non-renderable references in the
   Megaton package may appear as native conversion diagnostics; they must not
   abort preparation or leave a missing physics sidecar reference.

2. Launch the Mart scene with physics and the bridge:

   ```text
   cargo run-dev -- view --manifest .bevyout/cache/scenes/00000c49/scene.ron --agent-bridge --agent-port 15726 --trace-seconds 180
   ```

   Do not pass `--worldspace-lod` for the route gate. Far-worldspace tiles
   are optional presentation polish; the route uses the native per-cell
   near/middle/distant terrain LOD and full LAND collision.

3. In the console, run:

   ```text
   worldstream status
   worldstream cells
   nav exterior
   tna spawn
   environment status
   lights streamed
   worldstream presentation
   worldstream summary
   ```

   Expected: the active grid is `(4,-5)`, the ready safety cells have
   `collision_ready=true`, `failed=0`, and `tna spawn` succeeds. `nav
   exterior` reports revision `exterior-nav-v3`; the Mart package currently
   reports 223 vertices, 279 triangles, and 54 border portals.

4. Exercise the six real grids. Because the cells are on a steep authored
   slope, set the approximate authored height before moving across each cell;
   otherwise a physics-enabled player can legitimately start below the next
   cell's ground:

   ```text
   tp 180 177 275.31
   tp 120 187 275.31
   tp 60 197 275.31
   tp 10 194 275.31
   tp -50 181 275.31
   ```

   `tp` writes all three axes atomically, avoiding a physics tick between
   separate axis changes. After each move, wait for `worldstream status`,
   then run `worldstream presentation`, `tna despawn`, and `tna spawn`.
   Expected grids are, in order, `(3,-5)`, `(2,-5)`,
   `(1,-5)`, `(0,-5)`, and `(-1,-5)`; every stop should report `failed=0`,
   successful agent spawn, resident count no greater than 4, no stale
   completions, and a presentation report whose terrain collision remains
   `full_land_mesh`. The updated real-data run reached all six grids with a
   steady resident count of 2–4, `peak_resident_cells=9` while the prefetched
   safety ring was being built, `peak_memory=2218157`, and 13 requests with 6
   evictions. This is streaming evidence; Wave 9 still owns the agreed final
   memory and transition budgets.

5. Exercise reversal and persistence at a boundary:

   ```text
   player.setpos y 194
   player.setpos x 1
   save m6-boundary
   worldlocation
   ```

   Expected: the save succeeds and `worldlocation` reports exterior worldspace
   `0000003c` with the exact player position. Restart with
   `--save-slot m6-boundary`; the player must return to the same grid/position.

6. Exercise environment and water diagnostics at the active exterior cell:

   ```text
   environment status
   settime 18
   setweather 00015425 0
   environment status
   waterstate
   worldstream presentation
   weather clear
   ```

   Expected: the environment response changes source weather/time identities
   deterministically, and `waterstate` remains an explicit contact/breath
   report rather than being inferred from rendered pixels.

7. Deliberately remove or rename one neighboring prepared package and repeat
   one boundary crossing. Expected: the current collision-ready cell remains
   playable, the failure appears in `worldstream status`, and no stale
   completion resurrects the missing cell. Restore the package before the
   final route run.
