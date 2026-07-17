# M4 wave 2 — manual acceptance (#111)

## What this wave shipped

`prepare` now decodes each cell's Fallout 3 `NAVM` navigation-mesh records
(vertices, triangles, adjacency, door links, external connections) and the
plugin-wide `NAVI` index, and writes a validated, backend-neutral polygon
navigation graph as `navgraph.ron` beside the retained raw `*.navm.bin`
sources. The scene manifest points at the graph with counts and a hash.
Nothing consumes the graph at runtime yet — that is the follow-up
bevy_landmass spike (#112) — so the viewer looks and behaves exactly as
before. Acceptance is therefore prepare-side: run `prepare`, read the
deterministic counter line, and inspect the artifact.

## Prerequisites

Fallout 3 game data configured as for any `prepare` run. All cells
re-prepare once because the pipeline revision changed
(`prepare-nav-graph-v1`); that is expected.

## Steps

1. Prepare the small reference interior:

   ```
   cargo run-dev -- prepare --cell 000151e3
   ```

   Expected in the output, exactly (the `merges` field and `nav-graph-v2`
   revision below were added by M4 wave 4's #113 cross-mesh connections;
   this manual's expected strings track the current output):

   ```
   nav graph: meshes 1, polygons 183, vertices 165, doors 1, external 0, merges 0, diagnostics warn 0 error 0
   ```

2. Confirm the artifact exists beside the raw source:

   ```
   ls .bevyout/cache/scenes/000151e3/navmesh/
   ```

   Expected: `000c71da.navm.bin` and `navgraph.ron` (~72 KB).

3. Open `.bevyout/cache/scenes/000151e3/navmesh/navgraph.ron` in a text
   editor. Expected: `revision: "nav-graph-v2"`, one mesh with
   `form_id: 819162` (0x000c71da), 165 `vertices` in metres (values in the
   single-digit range, not the thousands of raw FO3 units), 183 `polygons`
   each with `adjacency`, and one entry in `doors`.

4. Confirm the manifest points at the graph:

   ```
   grep -A 4 "nav_graph" .bevyout/cache/scenes/000151e3/scene.ron
   ```

   Expected: `nav_graph: Some((asset_path: "scenes/000151e3/navmesh/navgraph.ron", revision: "nav-graph-v2", ...)` with the same counts as step 1.

5. Re-run step 1. Expected: the identical counter line, and
   `navgraph.ron`'s modification time unchanged (byte-identical output is
   detected and the file left untouched).

6. Prepare a multi-door and a multi-mesh interior:

   ```
   cargo run-dev -- prepare --cell 00003a35
   cargo run-dev -- prepare --cell 0001a273
   ```

   Expected counter lines, exactly:

   ```
   nav graph: meshes 1, polygons 193, vertices 187, doors 2, external 0, merges 0, diagnostics warn 0 error 0
   nav graph: meshes 2, polygons 1338, vertices 1198, doors 3, external 0, merges 13, diagnostics warn 0 error 0
   ```

7. See the nav graph in game (#128). Launch the viewer:

   ```
   cargo run-dev -- view --manifest .bevyout/cache/scenes/000151e3/scene.ron
   ```

   Open the console and run `tnm` (alias `togglenavmesh`). Expected reply:

   ```
   nav mesh visualization on (1 meshes, 183 triangles)
   ```

   and the walkable floor is covered by flat triangles, each a visibly
   different color, lifted just above the ground. `tnm` again replies
   `nav mesh visualization off` and hides the overlay; a third `tnm`
   shows it again without rebuilding. In a cell prepared without any
   NAVM records, `tnm` replies `no nav graph prepared for this cell`.

8. (Regression) Aside from the `tnm` overlay, the cell loads and plays
   exactly as before this wave; the nav graph has no other runtime
   consumer yet.

## Known diagnostics on real data

One info-level manifest diagnostic per content set:
`NAVI 00014b92: ignored unsupported NAVI.NVCI subrecord; layout not
documented for Fallout 3/New Vegas` — expected, see the plan's shipped
amendments.
