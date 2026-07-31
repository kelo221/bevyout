# M6 wave 1 plan

## Fixed feature list

1. Make native NIF preparation authoritative for exterior assets and actor
   animation. Reject production converter-selection flags that would imply a
   second backend.
2. Keep the legacy Blender script available only for explicit preview or
   comparison work, with its remaining native gaps documented at the top of the
   script.
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
