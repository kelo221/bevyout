# M6 wave 1 manual acceptance

This partial M6 PR adds deterministic native exterior preparation, prepared
package streaming, and focused correctness fixes for cancellation teardown,
water-cell ownership, streaming diagnostics, and short merge-portal handoff.
It is not the M6 completion PR: issues #13, #87, and #14 remain open, and the
deferred acceptance work is listed at the end of this manual.

1. From the repository root, verify the production CLI no longer accepts a
   converter backend:

   ```text
   cargo run-dev -- prepare --help
   ```

   Expected: no `--converter blender` option is shown; native preparation is
   the default and only production path.

2. With a real Fallout 3 installation configured, prepare one known exterior
   cell by its catalog selector:

   ```text
   cargo run-dev -- prepare 00000c49
   ```

   Expected: the command prints `prepared exterior`, writes
   `.bevyout/cache/scenes/<cell-formid>/scene.ron`, and writes the matching
   `worldspaces/<worldspace-formid>/index.ron` and cell package.

3. Print the prepared worldspace catalog:

   ```text
   cargo run-dev -- exterior-catalog --index .bevyout/cache/worldspaces/<worldspace-formid>/index.ron
   ```

   Expected: one stable worldspace summary followed by sorted `cell` lines with
   grid, origin, and package paths. Repeating the command produces identical
   output.

4. Launch the exact prepared Super-Duper Mart exterior scene:

   ```text
   cargo run-dev -- view --manifest .bevyout/cache/scenes/00000c49/scene.ron --agent-bridge --agent-port 15702
   ```

   In the viewer console, run:

   ```text
   worldstream status
   worldstream cells
   worldstream trace 1
   worldstream summary
   nav exterior
   nav borders
   tna spawn
   environment status
   lights streamed
   ```

   Expected: JSON reports the active worldspace, current grid, lifecycle
   counters, package-owned cells, and `collision_ready` per cell. `nav
   exterior`/`nav borders` report the prepared navigation evidence and `tna
   spawn` creates an exterior agent when the resident graph is ready. Moving
   across a prepared-cell boundary requests the next package, attaches BoxDDD
   collision before readiness, and never invokes Blender. The current CLI has
   no Blender converter flag. `environment status`
   reports the prepared climate/weather identities and `lights streamed`
   reports the deterministic local-light budget.

   The terrain package also contains a prepared cell-local albedo generated
   from LAND layer weights and LTEX/TXST sources. Capture the viewport once;
   the expected result is textured, upward-facing ground without the former
   white/black fallback holes. The full bounded route and long-run budgets
   remain the later M6A/M6 gates.

5. On the prepared c49 data, exercise the short resident-cell merge handoff:

   ```text
   tp 180 176.35 275.30
   tna spawn
   tna goto 235.92 158.53 243.29
   tna status
   ```

   Expected: the agent crosses the short `(3,-5) -> (4,-5)` merge seam once,
   continues toward the target, and ends with `status=reached`,
   `blocked=false`, `stuck=false`, and `merge_traversal=null`. This check must
   be recorded again before merging this partial PR: the latest bounded run
   verified the handoff branch and no collision block, but was stopped before
   the final post-repath route result was captured.

6. Stop the viewer and verify the one-cell fingerprint check is report-only:

   ```powershell
   $scene = '.bevyout/cache/scenes/00000c49/scene.ron'
   $before = (Get-FileHash -Algorithm SHA256 $scene).Hash
   cargo run-dev -- prepare 00000c49 --check-fingerprints
   $after = (Get-FileHash -Algorithm SHA256 $scene).Hash
   $before -eq $after
   ```

   Expected: the command reports the fingerprint result, does not prepare or
   rewrite the scene, and PowerShell prints `True`.

## Deferred from this partial PR

Do not close #13, #87, or #14 from this PR. The following still require later
implementation or recorded real-data acceptance:

- ordinary keyboard traversal, rapid reversal/cancellation, eviction ordering,
  duplicate-root checks, and collision teardown across repeated route loops;
- actor binding/pathing across cells, exterior/interior travel, exact return
  anchors, save/reload, and dynamic-state persistence;
- water entry/exit, breath and fall behavior; weather/time/ImageSpace,
  local-light budgets, and interior-lighting isolation;
- terrain LOD hysteresis/neighbour clamping, distant/VWD representations,
  duplicate near/far objects, conservative occlusion, and visible pop-in;
- agreed transition/frame/resident budgets and repeated-loop process-memory
  measurements. `worldstream summary` reports package-byte estimates, while
  `resident_bytes`, `peak_memory`, and `ending_memory` remain explicit `null`
  fields until real process-memory instrumentation supplies them.

7. Repeat normal preparation without `--rebuild-assets`.

   Expected: native cache outputs are reused where valid, and the package
   remains byte-stable apart from intentionally separate timing/report fields.
