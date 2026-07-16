# M4 wave 2 — NAVM/NAVI nav graph (#111)

Single-issue wave under epic #9 on branch `m4-wave2` off master. One
executor works directly on the wave branch per AGENTS.md model routing;
the orchestrator owns GitHub housekeeping, diff review, gates, and
real-data acceptance.

## Fixed feature list

### 1. NAVM subrecord decode (`src/vsa/openmw_esm4/navmesh.rs`)

- Extend the existing `parse_navmesh` seam (records.rs keeps dispatch; the
  decode lives in a new `navmesh.rs` module inside the isolated OpenMW
  boundary) to decode, with strict bounds/count validation:
  - `NVER` version (already read),
  - `DATA` cell/worldspace owner FormID and vertex/triangle/external
    connection/door/cover counts,
  - `NVVX` vertices (3×f32 source coordinates),
  - `NVTR` triangles (vertex indices, edge-neighbour indices, edge/cover
    flags),
  - `NVCA` cover triangles (retained decoded but minimal),
  - `NVDP` door associations (door reference FormID + triangle index),
  - `NVGD` grid data (retained as validated metadata; not required by the
    runtime graph),
  - `NVEX` external connections (linked NAVM FormID + triangle/edge).
- Field layouts are verified against the fopdoc FalloutNV NAVM/NAVI pages
  and OpenMW `loadnavm.hpp`/`loadnavi.hpp` — not assumed. FO3 vs NV layout
  differences must be resolved in favour of what the real FO3 plugin data
  parses cleanly as (version gate on `NVER` if needed).
- Counts that disagree with actual payload sizes, truncated subrecords,
  and out-of-range indices produce recoverable diagnostics on the record
  (pattern: `ignored_subrecords`/diagnostic strings like existing
  parsers), never panics. Malformed records degrade to the current
  raw-payload-only behaviour.
- FormID subrecords go through the existing `FormIdResolver` so
  load-order adjustment stays uniform. Raw payload is retained as today.

### 2. NAVI capture (`reader.rs` + `navmesh.rs`)

- Add a `"NAVI"` dispatch arm (top-level record, no cell context):
  decode `NVER`, per-navmesh `NVMI` info entries (NAVM FormID, owning
  cell/worldspace, bounds/position, linked doors/merged and preferred
  merges as documented), and `NVPP` preferred pathing, retaining
  undocumented tails as opaque bytes with diagnostics.
- `ParsedState`/`ParsedPlugin` gain a `navigation: Option<NaviRecord>`
  (last-loader-wins override semantics like other singletons; deleted
  flag clears it).

### 3. Backend-neutral nav graph (`src/vsa/prepare/nav_graph.rs`)

- Pure std/serde-only module (cucumber-testable via `#[path]`, no Bevy
  imports). Input: decoded NAVM records for one cell plus the optional
  NAVI record. Output: `PreparedNavGraph`:
  - vertices in Bevy metres, converted exactly once with the
    `paths.rs` convention (`[x, z, -y] * FO3_SCALE`);
  - polygons (triangles) with per-edge adjacency (intra-mesh from NVTR
    edge fields, cross-mesh from NVEX), walkability/area flags, cover
    data where decoded;
  - door/off-mesh associations (door reference FormID + polygon);
  - external connections retained with source FormIDs for M6 stitching;
  - per-mesh and whole-graph AABB bounds; source FormID + cell owner on
    every mesh.
- Validation with severity-tagged diagnostics, deterministic ordering
  (sort by FormID, then index): duplicate/out-of-range indices,
  degenerate triangles, non-manifold edges, asymmetric adjacency,
  disconnected islands (reported, not fatal), invalid external links
  (target NAVM absent), door triangle indices out of range.
- `NAV_GRAPH_REVISION` versions the asset (pattern:
  `ACTOR_CATALOG_REVISION`), serde-defaulted fields for forward
  compatibility.

### 4. Prepare/manifest wiring

- `stage_navmeshes` additionally builds and writes the graph asset
  (`scenes/<cell>/navmesh/navgraph.ron` beside the retained raw
  `*.navm.bin` sources) and the manifest gains a
  `PreparedNavGraphSource { asset_path, revision, mesh/vertex/polygon
  counts, diagnostics summary }` (or equivalent) next to
  `navmeshes`. Raw source metadata stays for diagnostics.
- Bump `PREPARE_PIPELINE_REVISION` so caches invalidate.
- `prepare` report counters: navmeshes decoded, polygons, vertices,
  door links, external links, diagnostics by severity — deterministic
  `println!` lines per the logging policy.
- No runtime consumption in this wave (no A*, no steering — #112/#113).
  The viewer must not need ESM bytes: the graph is complete from the
  prepared asset alone.

## Tests (feature-first, before implementation)

- `features/nav_graph.feature` + steps in `tests/features.rs` (World
  fields appended at the end of the struct, delimited step section at the
  end of the file): synthetic in-memory byte fixtures — no committed
  binary files — covering each subrecord decode, count/payload mismatch,
  truncation, adjacency construction, door and external links, NAVI
  override semantics, coordinate conversion (one known vertex →
  expected Bevy metres), deterministic ordering, and diagnostic
  severities.
- Unit tests in `nav_graph.rs` for island detection, non-manifold and
  asymmetric-adjacency validation, and revision constant.
- Gates: `cargo fmt --check`, `cargo clippy --all-targets -- -D
  warnings`, `cargo test`, representative `cargo run-dev -- prepare`.

## Real-data acceptance (orchestrator)

- Prepare a representative interior cell; record polygon/vertex counts,
  invalid-link counts, graph asset size, prepare time, and cache
  invalidation behaviour (second run reuses, `--rebuild`-class flags
  rebuild). Comment measured results on #111.

## Out of scope

Runtime pathfinding/steering (#112/#113), grounded movement (#114),
exterior tile stitching (M6 #13/#87), runtime NAVM generation, Blender.
