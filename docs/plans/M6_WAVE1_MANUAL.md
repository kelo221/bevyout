# M6 wave 1 manual acceptance

This wave adds deterministic native exterior preparation and the first
prepared-package streaming surface. It does not claim the final Super-Duper
Mart route gate; that requires the later M6 waves and real Fallout 3 data.

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
   cargo run-dev -- prepare --game-root <FALLOUT3> --plugin Fallout3.esm <EXTERIOR_CELL>
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

4. Launch the prepared exterior scene:

   ```text
   cargo run-dev -- view <EXTERIOR_CELL> --cache-dir .bevyout/cache --agent-bridge
   ```

   In the viewer console, run:

   ```text
   worldstream status
   worldstream cells
   worldstream trace 1
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

5. Stop the viewer and repeat the preparation command without
   `--rebuild-assets`.

   Expected: native cache outputs are reused where valid, and the package
   remains byte-stable apart from intentionally separate timing/report fields.
