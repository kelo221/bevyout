# M6 wave 9 — Super-Duper Mart ↔ Megaton final gate manual

This is the final human acceptance script for issue #14. Run it only after
the bounded Wave 7 script is green. Record the machine, GPU, build mode, cache
state, and every numeric result in the route summary; do not replace a budget
with a visual judgement.

1. Run the Wave 7 preparation command once from an empty derived exterior
   cache, then once again without `--force`. Record cold and warm preparation
   seconds, cache bytes, assets built/reused, lossy diagnostics, and the
   native converter invocation count. The runtime converter/Blender count must
   remain zero.

2. Launch the exact prepared Mart manifest with physics and bridge enabled:

   ```text
   cargo run-dev -- view --manifest .bevyout/cache/scenes/00000c49/scene.ron --agent-bridge --agent-port 15726 --trace-seconds 900
   ```

   Leave out `--worldspace-lod` for this gameplay gate. The far-worldspace
   tile layer is intentionally deferred/opt-in; it must not affect the route's
   per-cell terrain LOD, collision, navigation, or streaming budgets.

3. Capture baseline diagnostics and a viewport image:

   ```text
   worldstream status
   worldstream cells
   nav exterior
   nav borders
   tna spawn
   environment status
   lights streamed
   waterstate
   worldstream presentation
   worldstream summary
   ```

   Expected: the active cell is SuperDuperMartExterior `00000c49`, the
   collision-ready safety ring is visible, the resident budget is respected,
   and the viewport shows upward-facing textured terrain with no fallback
   white/black material holes. `worldstream presentation` reports terrain
   near/middle/distant counts, distance-hidden objects, and the separate
   engine-managed frustum/true-occlusion fields.

4. Walk the route in both directions using ordinary movement input. Do not use
   `player.setpos` for this step. At each grid boundary record the active grid,
   transition latency, resident count, collision-ready state, failed count,
   cancellations, and stale completions. The route is:

   ```text
   (4,-5) 00000c49 Super-Duper Mart
   (3,-5) 00000c4a
   (2,-5) 00000c4b
   (1,-5) 00000c4c
   (0,-5) 000010d5
   (-1,-5) 00001245 Megaton Main Gate
   ```

   Expected: the player remains on collision, no duplicate cell roots appear,
   the current cell is never evicted before its replacement is ready, and the
   return reaches the original Mart anchor.

5. At the Mart, Megaton, and one cell boundary, run:

   ```text
   save m6-route-center
   worldlocation
   ```

   Restart each slot with `--save-slot` and verify the exact worldspace,
   grid, position, rotation, and dynamic-reference state. Visit one prepared
   interior if the selected route door is available, save there, return to the
   same exterior anchor, and repeat the check.

6. Repeat the route at noon, sunset, midnight, and sunrise. During one pass:

   ```text
   setweather 00015425 10
   environment status
   ```

   Capture the clear-to-adverse transition, an interior transition while the
   blend is active, the return to exterior, and a valid water entry/exit. The
   prepared WTHR source/target IDs and blend progress must be visible in the
   environment response.

7. Bind one test actor with `tna bind` at a prepared navigation point and
   route it across at least one resident-cell border. Record path latency and
   final `tna status`. A failed target caused by an off-mesh manual coordinate
   is not a passing result; choose a point shown by `nav borders` or the
   prepared navigation artifact.

8. Perform one rapid reversal at a boundary and five complete out-and-back
   loops. Record `worldstream status`, `worldstream presentation`, and
   `worldstream summary` after every loop. The summary consolidates live
   streaming/presentation/frame fields; offline preparation and transition
   timings remain required inputs. The final report must
   include cold/warm preparation, p50/p95 ready and transition times, peak
   residents/memory, ending memory, frame median/p95/max, nav latency,
   visible LOD transitions, failed/cancelled/stale counts, and converter count.

9. Only declare the final gate complete when all route steps pass on real data,
   both cache modes pass, memory returns to a stable plateau, and the numeric
   budgets have been agreed and recorded in the PR/issue evidence.
