# M2 Import Wave — Plan

Epic: [#5 — M2 Bulk preparation and connected interiors](https://github.com/kelo221/bevyout/issues/5)

Claimed area/import sub-issues, dependency-ordered:

| Issue | Scope | Status this wave |
|-------|-------|------------------|
| [#45](https://github.com/kelo221/bevyout/issues/45) Cell map catalogue | worldspace grids, XCLC coords, door connectivity | **executed now** (priority: unblocks cell loading) |
| [#46](https://github.com/kelo221/bevyout/issues/46) Batch selectors | `--all`, `--all-interiors`, `--worldspace`, explicit lists | **executed now** |
| [#47](https://github.com/kelo221/bevyout/issues/47) Batch session | parse chain/BSA once, shared caches | later — depends on #46 |
| [#48](https://github.com/kelo221/bevyout/issues/48) Resumable prepare | workers, job manifest, retry-failed | later — depends on #47 |
| [#49](https://github.com/kelo221/bevyout/issues/49) Fingerprints | plugin/converter/physics/prep validation | later — depends on #48 |

Cell mapping was not on the original M2 checklist; #5 was amended to include it because
M2 door travel ("Resolve destination cell, door, position, and facing") and M6 exterior
streaming both need a static cell map that exists before any cell is loaded.

**Ordering rule inside every issue (repo convention from M1): feature list fixed below →
Cucumber feature + tests written first → implementation makes them green.**

Execution: one Sonnet agent per issue, isolated git worktrees (M1 pattern), branches
`m2-45-cell-map` and `m2-46-batch-selectors` off local `master`. The orchestrator merges,
runs `cargo fmt --check`, `clippy -D warnings`, `cargo test`, then verifies against real
Fallout 3 GOTY data and the live viewer over the bevyout MCP / BRP bridge.

---

## Issue #45 — Cell map catalogue

Everything OpenMW-derived stays inside `src/vsa/openmw_esm4/` (repo rule). OpenMW
references: `components/esm4/loadcell.hpp` (XCLC grid), `loadwrld.hpp` (worldspace),
`apps/openmw/mwworld/worldmodel.*` (static cell store shape).

### Feature list

- **F45.1** Parse `XCLC` on exterior `CELL` records → `CellInfo.grid: Option<(i32, i32)>`.
- **F45.2** Parse `WRLD` records (FormID, EDID, FULL) and attach `CellInfo.worldspace_form_id`
  to cells found inside world-child GRUPs during traversal.
- **F45.3** Content-set-wide door graph: generalise the per-cell XTEL resolution already in
  `ParsedContentSet::select` into a whole-set pass producing door edges
  `{source_cell, door_ref, destination_cell, destination_door_ref, position, rotation}`.
  Unresolved teleports are counted, not fatal.
- **F45.4** `cells --map [--out <path>]`: deterministic RON `CellMap` artifact
  (content fingerprint, cell entries incl. grid/worldspace, worldspace table, door edges,
  sorted by FormID). Stdout when `--out` is absent.
- **F45.5** Logging: one summary line per run — total cells, exteriors with grid,
  worldspaces, door edges, unresolved doors (matches `prepare`'s println convention).
- **F45.6** `features/cell_map.feature` + steps in `tests/features.rs`: synthetic parsed
  content → map building is pure and deterministic; golden test pins the RON shape.

### Tests before code

- **T45.1** XCLC bytes → grid coords (unit, synthetic subrecord).
- **T45.2** Cell inside a WRLD group gets that worldspace FormID; interior gets `None`.
- **T45.3** Two cells joined by opposing XTEL doors produce two directed edges with correct
  destination cell/position; dangling XTEL counts as unresolved.
- **T45.4** RON output is byte-identical across two runs on the same input (golden).
- **T45.5** Cucumber scenarios covering T45.2–T45.4 shapes.

### Real-data acceptance (orchestrator)

`cargo run-dev -- cells --map` against Fallout3.esm: Vault 101 interiors
(`Vault101a/b/c` etc.) appear with door edges linking them and to the Wasteland exterior
(worldspace + grid present on the exterior side).

## Issue #46 — Batch selectors

### Feature list

- **F46.1** `prepare` accepts `--all`, `--all-interiors`, `--worldspace <EditorID|FormID>`,
  and multiple positional selectors; all mutually combinable except `--all` (which subsumes).
- **F46.2** Pure function: catalogue + selector spec → ordered (FormID-sorted, deduplicated)
  cell list; unknown/ambiguous selector fails with an actionable error naming candidates.
- **F46.3** `prepare --list-only` prints the resolved set (one `formid<TAB>editor_id` line
  per cell) and exits before any extraction — the verification seam until #47/#48 make
  batch runs cheap and resumable.
- **F46.4** Without `--list-only`, prepare iterates the resolved set sequentially with the
  existing single-cell path; per-cell failures are reported at the end, not mid-run panics.
- **F46.5** `features/prepare_selectors.feature` + steps: selector resolution scenarios.

### Tests before code

- **T46.1** `--all-interiors` yields exactly the interior subset, sorted.
- **T46.2** Explicit list mixing EditorID and FormID resolves, dedupes, sorts.
- **T46.3** Unknown worldspace errors and names available worldspaces.
- **T46.4** Cucumber scenarios for T46.1–T46.3.

### Real-data acceptance (orchestrator)

`cargo run-dev -- prepare --all-interiors --list-only` lists all Fallout3.esm interiors;
`prepare --worldspace <wasteland> --list-only` lists exterior grid cells.

---

## Verification via bevyout MCP / BRP

After merge + green local gates:

1. `cells --map` real-data acceptance above; inspect Vault 101 edges.
2. Launch viewer (`render <cell> --agent-bridge`), then over BRP: `bevyout.session`,
   `bevyout.scene_snapshot` (placements present), `bevyout.console.exec` smoke, and
   `viewer_logs`-equivalent stderr tail — confirms the parser changes did not regress
   scene preparation or rendering.
3. Update #45/#46 with results; tick #5 checklist items when the gate criteria hold.
