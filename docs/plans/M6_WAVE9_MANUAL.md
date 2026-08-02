# M6 wave 9 — Super-Duper Mart ↔ Megaton final gate manual

**Status: future acceptance draft.** Merged foundation PR #261 and W6-B now
freeze the current-data route selectors and protocol in
[M6_WAVE7_MANUAL.md](M6_WAVE7_MANUAL.md), but this manual is not satisfied by
the foundation alone. Actor crossing, ordinary
bidirectional traversal, interior travel/return anchors, water and save/reload
coverage, repeated-loop memory measurements, and agreed final budgets remain
unverified. The route is dry; the water fixture is separate, and the current
actor/door boundaries remain dependency-held.

This is the final human acceptance script for issue #14. Run it only after
the bounded Wave 7 script is green. Record the machine, GPU, build mode, cache
state, and every numeric result in the route summary; do not replace a budget
with a visual judgement. The pre-W7 threshold matrix is tracked in child issue
#285 and must be recorded before this final route is accepted.

1. Use the exact 14-selector list, current native pipeline, and clean/warm
   preparation matrix from Wave 7. For a standalone run, use an empty
   disposable cache for the first command and the same cache for the second;
   do not delete the evidence cache:

   ```powershell
   $cleanCache = 'C:\Users\V\Projects\Rust\bevyout\.bevyout\m6-route-clean-cache'
   if (Test-Path -LiteralPath $cleanCache) { Remove-Item -LiteralPath $cleanCache -Recurse -Force }
   New-Item -ItemType Directory -Force -Path $cleanCache | Out-Null
   ```

   Run the exact Wave 7 prepare command once from the empty cache, then once
   again without `--force`, and finish with its `--check-fingerprints`
   command. Record clean and warm preparation seconds, cache bytes, assets
   built/reused, recoverable diagnostics, and native converter invocation
   count. The runtime converter/Blender count must remain zero.

   ```text
   cargo run-dev -- --config C:\Users\V\Projects\Rust\bevyout\.bevyout\config.toml prepare 00000c49 00000c4a 00000c4b 00000c4c 000010d5 00001245 00000c67 00000c68 00000c69 00000c6a 00000c6b 000010eb 000010ec 000010ed --cache-dir $cleanCache --actor-animation-converter native --jobs 1
   cargo run-dev -- --config C:\Users\V\Projects\Rust\bevyout\.bevyout\config.toml prepare 00000c49 00000c4a 00000c4b 00000c4c 000010d5 00001245 00000c67 00000c68 00000c69 00000c6a 00000c6b 000010eb 000010ec 000010ed --cache-dir $cleanCache --actor-animation-converter native --jobs 1
   cargo run-dev -- --config C:\Users\V\Projects\Rust\bevyout\.bevyout\config.toml prepare 00000c49 00000c4a 00000c4b 00000c4c 000010d5 00001245 00000c67 00000c68 00000c69 00000c6a 00000c6b 000010eb 000010ec 000010ed --cache-dir $cleanCache --actor-animation-converter native --jobs 1 --check-fingerprints
   ```

2. Launch the exact prepared Mart manifest with physics and bridge enabled:

   ```text
   cargo run-dev -- view --manifest C:\Users\V\Projects\Rust\bevyout\.bevyout\cache\scenes\00000c49\scene.ron --agent-bridge --agent-port 15726 --trace-seconds 900
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
   engine-managed frustum/true-occlusion fields. `tna spawn`, if used, is a
   disposable navigation smoke check only; it is not evidence for the frozen
   actor fixture in route cell `00000c67`.

4. Walk the route in both directions using ordinary movement input. Do not use
   `player.setpos` for this measured step. At each grid boundary record the
   active grid, request-to-ready and transition latency, resident count,
   collision-ready state, failed count, cancellations, stale completions,
   invalid unload count, root/collider/light counts, and return-anchor result.
   The exact route is:

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
   return reaches the original Mart anchor. The deterministic `tp` probe in
   Wave 7 is setup/streaming evidence only and is not a substitute for this
   ordinary-input pass.

5. At the Mart, the frozen route center, Megaton, and one cell boundary, use
   distinct save slots. Reserve `m6-route-center` for the exact synthetic
   checkpoint from Wave 7:

   ```text
   player.setpos x 1
   player.setpos y 193.98508
   player.setpos z 275.31247
   save m6-route-center
   worldlocation
   ```

   The expected center WLOC is worldspace `0000003c`, position
   `(1.0,193.98508,275.31247)`, identity rotation, and prepared fingerprint
   `24efdfcef26d1ebb3d347c976da6c85cd8a17e313b8a22c2709ff90b180941d0`.
   Name the other slots `m6-route-mart`, `m6-route-megaton`, and
   `m6-route-boundary`; restart each with `--save-slot <slot>` and verify the
   exact worldspace, grid, position, rotation, and dynamic-reference state.
   The route-door fallback is source-only, so do not count an interior visit
   until W4-C makes the persistent door executable. The save binaries under
   ignored `.bevyout/saves` are synthetic runtime evidence and are not
   authored Fallout source.

6. Repeat the route at noon, sunset, midnight, and sunrise. For the frozen
   current environment baseline use worldspace `0000003c` (`Wasteland`),
   climate `00017907` (`WastelandClimate`), and weather `00064609`
   (`WastelandClear`):

   ```text
   setweather 00064609 10
   environment status
   ```

   Record the prepared WTHR source/target IDs and blend progress visible in
   the environment response. The six-cell route has no water FormID or
   surface; use separate water fixture cell `00001262`
   (`PotomacMirelurkNest02`, grid `(3,-2)`, raw `XCWT` `0007e421`, raw
   `XCLW` `10750.0`, prepared water form `0007e421`, prepared height
   `153.57143`) for water entry/exit once W4-C is available. Do not report
   the separate fixture as route water.

7. Actor crossing is currently `not_run`: the source fixture is reference
   `000638e8`, base `0001cf73` (`CrEyebotEnclave`) in route cell `00000c67`,
   with source `MODL` `Creatures\\Eyebot\\Skeleton.nif`; the prepared dynamic
   identity is reference `407784`, base `118643`, but no actor asset/catalog
   is present. Do not count `tna bind` as actor acceptance until #10/W3
   supplies the runtime integration. After that dependency closes, bind the
   exact fixture at a point shown by `nav borders`, route it across a
   resident-cell border, and record path latency and final `tna status`.

8. Perform one rapid reversal at a boundary and five complete out-and-back
   loops using the exact route order and numeric recording fields in Wave 7.
   Record `worldstream status`, `worldstream presentation`, and
   `worldstream summary` after every loop. The summary consolidates live
   streaming/presentation/frame fields; offline preparation and transition
   timings remain required inputs. The final report must include cold/warm
   preparation, p50/p95 ready and transition times, peak residents/memory,
   ending memory, frame median/p95/max, nav latency, visible LOD transitions,
   failed/cancelled/stale/invalid-unload counts, converter count,
   actor/door/water status, and the exact save/reload checkpoint result.

9. Only declare the final gate complete when #10/W3 actor integration, W4-C
   travel/water/door integration, all route steps, both cache modes, the five
   loops, and the numeric budgets pass on real data; memory returns to a
   stable plateau; and the evidence is recorded in the PR/issue material.
