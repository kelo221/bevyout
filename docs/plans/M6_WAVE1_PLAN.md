# M6 wave 1 plan

## Fixed feature list

1. Make native NIF preparation authoritative for exterior assets and actor
   animation. Reject production converter-selection flags that would imply a
   second backend.
2. Keep the legacy Blender script as a non-production reference artifact, with
   its remaining native gaps documented at the top of the script; no Blender
   installation or CLI backend is required.
3. Decode and retain LAND, ROAD counts, exterior cell ownership, persistent and
   distant reference flags, climate/weather/image-space links, and water data.
4. Add revisioned Bevy-free exterior contracts and deterministic coordinate
   conversion, index, package, terrain, environment, and residency policies.
5. Prepare exterior packages with native asset cache outputs and explicit
   diagnostics for missing or lossy assets.
6. Expose `exterior-conversion-report`, `exterior-catalog`, and the runtime
   `worldstream` console surface.

## Tests-first list

- native converter flags and report inputs are deterministic and Blender
  selection is rejected;
- coordinate round trips cover negative and positive grids and border epsilon;
- prepared revisions and serde-default compatibility are pinned;
- LAND arrays, normals, colors, and texture layers produce stable terrain;
- index ordering, persistent ownership, and catalog output are deterministic;
- residency actions are bounded and generation-safe;
- the prepared exterior loader cannot call a converter.

## Execution model recommendation

Codex runtime: Sol X-High. This wave crosses parser, prepared schemas, native
asset caching, and Bevy lifecycle integration.

## Shipped amendments

### A1 — Native-only production path

The requested decision changed the original comparison shape. Native Rust is
now authoritative for exterior preparation and actor animation. Blender is not
selected by prepare/render and is not reachable from runtime streaming. The
legacy script remains for explicitly requested preview/reference work only.

Native gaps currently recorded: LAND terrain material/layer fidelity,
VWD/distant-geometry generation, and unsupported/lossy NIF blocks. These are
diagnosed in prepared artifacts instead of being hidden behind runtime fallback.

### A2 — Wave 1 merged with contract/runtime scaffolding

Because the existing parser already had a stable cell-selection seam, the
revisioned exterior contracts, index/package preparation, native package asset
staging, and first streaming lifecycle landed together. Later waves retain
ownership of terrain material, full navigation, environment transitions,
distance LOD, and real-data gate evidence.

## Verification record

- `cargo fmt --check`: passing after the wave-1 edits.
- `cargo check-dev`: passing.
- Focused Avian ragdoll regression: passing after removing an unnecessary
  `collider-from-mesh` feature that initialized a mesh-event system in isolated
  tests.
- Full library suite: 1,514 passed, 0 failed, 3 ignored.

### A3 — Real-data launch placement and Bethesda terrain decoding

The first real Super-Duper Mart capture exposed an empty-world presentation
failure: the viewer initialized the FPS player at the default origin while the
prepared cell was at grid `(4, -5)`. The runtime now waits for the player entity
to exist, places it at the prepared terrain center, and gates residency updates
until that placement is complete. Duplicate worldspace grid entries choose the
lowest deterministic cell FormID, so prepared route packages win over the
unprepared detail-cell variants.

The Bethesda VHGT decoder now follows the row-wise differential format and its
8x height scale. On cell `00000c49`, terrain samples moved from roughly 20 m to
the authored 157–166 m elevation occupied by the native-prepared structures.
Invalid `CELL.XCLW` sentinel values are omitted from the package and retained as
stable `invalid_water_height` diagnostics. The real adjacent route packages
`00000c29`, `00000c2a`, `00000c2b`, `00000c48`, `00000c68`, and `00000c69` were
prepared successfully with native conversion. A physics-enabled bridge launch
held the player at `263.314, 158.129, 263.314` on the prepared terrain collider.

### A4 — BoxDDD collision and existing navigation backend reuse

The initial physics proof was tightened after checking the actual player
controller: FPS movement is BoxDDD-authoritative, so exterior LAND is now
inserted as a prepared static triangle mesh in BoxDDD during startup. The same
startup pass lazily loads native static-object physics sidecars and builds
their shapes, producing a real-data log of `117` bodies and `11,740` packed
triangles for `00000c49`.

Exterior NAVM packages expose a visible diagnostic tile and border report, but
the current flattened adapter is explicitly preview-only: it does not yet
preserve NAVM clearance, walkability metadata, adjacency, doors, NVEX, or
source/merge identities, so `tna spawn` refuses exterior scenes. Full
resident-cell navigation and cross-cell border stitching remain later-wave
work rather than silently becoming production actor pathing.

The streamed runtime now attaches each loaded package's terrain and native
static-object sidecars to the existing BoxDDD ledger before marking the cell
`Ready`/`Resident`. Eviction tears that ledger down before despawning the
package root, and the player transition is gated while a destination cell is
still render-only. Portal points are documented and tested as producer-emitted
world-space Bevy metres, including positive and negative non-zero grids.

The scene-focus fallback also uses the prepared exterior terrain center, so a
fresh viewer no longer computes its initial camera focus at world origin.

The visible navigation diagnostic returns `cell_form_id=3145`, grid `(4,-5)`,
`223` vertices, `279` triangles, and `54` border portals for the real package.

### A5 — focused review correction: collision-safe handoff

Acceptance found one transition-ordering edge: if the physical player position
entered an unready neighbor, planning around that target could evict the old
active cell before the new BoxDDD shapes existed. The residency planner now
focuses the physical target to request it immediately while temporarily
pinning the last collision-ready logical cell. The old cell is therefore kept
playable until the target logs `collision_ready=1`; only then can activation
and old-cell teardown proceed.

### A6 — Native LAND winding correction

The real-data viewer exposed a presentation bug in the initial native terrain
mesh: rows advance toward negative Bevy Z, but the generated triangle order
produced downward geometric normals while LAND/fallback vertex normals pointed
upward. The render mesh, BoxDDD terrain collider, and prepare-side terrain
clearance triangles now share upward winding. A runtime mesh regression test
pins the first flat quad's indices and geometric normal direction.

The remaining coarse appearance is expected Wave 1 behavior, not a normal
direction failure. LAND currently supplies a 33x33 height grid per cell, and
the package still renders vertex colors with a scalar material; texture-layer
splat/material fidelity remains the documented Wave 3 gap and full seam-safe
LOD/distant presentation remains Wave 8 work.

### A7 — native LAND material and persistence follow-up

The native terrain path now resolves LAND `BTXT`/`ATXT` assignments and `VTXT`
weights through `LTEX` -> `TXST`, applies `VCLR` modulation, and writes
revisioned cell-local 1024² albedo plus optional normal/specular assets during
prepare. The runtime consumes those prepared assets rather than source game
files. The explicit zero-form overlay sentinel is ignored. LAND remains an
authored 33x33 height surface, so seam-safe terrain LOD, VWD/distant geometry,
and unsupported or lossy NIF blocks remain later work.

The save/runtime follow-up now records an exact v7 world location, restores it
when the launched manifest matches the saved exterior worldspace or interior
cell, captures streamed exterior dynamic transforms before save, and applies
those deltas after package load. Wave 6 now packages all selected WTHR
keyframes for data-driven transitions, and exposes streamed-light budget,
environment, weather, and water diagnostics. These are implementation seams,
not substitutes for the Wave 7 real-data route gate.

Wave 8's resident terrain representation now selects near/middle/distant
meshes with hysteresis and clamps cardinal neighbours to one LOD step. The
gameplay collider remains the full prepared LAND mesh; presentation LOD does
not change collision or navigation state.
