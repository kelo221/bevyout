# M6 wave 8 — terrain and optional far-worldspace presentation manual

This manual checks the player-visible presentation work on the real
Super-Duper Mart exterior. Per-cell terrain LOD is part of the normal route
surface. Fallout's separate far-worldspace tile archive is deliberately
optional: modern hardware can rasterize the bounded set, but the archive is
1,608 independent prepared assets with importer/cache and authored-skirt
quirks, so it is not a prerequisite for gameplay acceptance.

## Current v21 runtime evidence — 2026-08-02

The current prepared route cache
`.bevyout/m6-w6c-route-clean-20260802/scenes/00000c49/scene.ron` was tested
with the dev viewer and BRP bridge. A default-off session reported
`worldspace_lod.active=0`, `blocks=0`, `terrain=0`, and empty level counts;
terrain remained `near=7`, `middle=0`, `distant=0`, with
`collision=full_land_mesh`. The report also kept
`collision_and_navigation_culled=false` and `presentation_only=true`.

The explicit `--worldspace-lod` session then reported the bounded opt-in set:
`active=48`, `terrain=40`, `blocks=8`, and levels `4=24`, `8=12`, `16=8`,
`32=4`. It retained `presentation_only=true`, full-land collision, and
`collision_and_navigation_culled=false`. `setrender worldspace_lod 0`
returned the report to `active=0`/`blocks=0`/`terrain=0` without changing
resident terrain or gameplay state; toggling back to `1` restored the same
48-tile bounded set, and the viewer was disabled again before shutdown.

The runtime report proves the active-tile and presentation-only invariants. The
per-frame import ceiling remains the code-level `8` spawn cap in
`src/viewer/world/exterior/mod.rs`; this report does not claim a separate
per-frame import counter.

1. Prepare the real Mart cell and its worldspace index:

   ```text
   cargo run-dev -- prepare 00000c49
   ```

   Expected: preparation ends with `prepared exterior` and the worldspace
   summary reports `worldspace LOD ... sources=1608 ... failed=0` on a current
   Fallout 3 cache. A warm rerun may reuse the 1,608 native GLBs.

2. Launch the normal route presentation without far-worldspace tiles:

   ```text
   cargo run-dev -- view --manifest .bevyout/cache/scenes/00000c49/scene.ron --agent-bridge --agent-port 15728 --trace-seconds 180
   ```

   Run:

   ```text
   worldstream presentation
   ```

   Expected: terrain reports the resident near/middle/distant representation
   and `collision=full_land_mesh`; `worldspace_lod.active` is `0`. The player
   remains on the authored textured terrain and no far tile import is needed.

3. Opt in to the far layer for a bounded visual experiment:

   ```text
   setrender worldspace_lod 1
   ```

   Wait several seconds, then run:

   ```text
   worldstream presentation
   ```

   Expected on the current `00000c49` cache: at most `48` active tiles, with
   separate level counts and no more than `8` imports staged in one frame.
   The report must still say `presentation_only=true`; terrain collision and
   navigation do not change.

4. Disable the optional layer again:

   ```text
   setrender worldspace_lod 0
   worldstream presentation
   ```

   Expected: `worldspace_lod.active=0` after the next update, while resident
   cells, terrain collision, and ordinary player movement remain available.

5. Repeat the check from the command line if a dedicated far-view capture is
   required:

   ```text
   cargo run-dev -- view --manifest .bevyout/cache/scenes/00000c49/scene.ron --worldspace-lod --agent-bridge --agent-port 15728 --trace-seconds 180
   ```

   This explicit opt-in is the only Wave 8 path that should be judged against
   far-horizon visual quality. Do not use it to claim the Wave 7/Wave 9 route
   gate: those gates intentionally run with the default-off setting.
