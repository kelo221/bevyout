# Proposed M6 execution plan

I would split #13 into nine waves. The critical ordering is:

**conversion decision → deterministic exterior artifacts → prepared cell packages → streaming → gameplay continuity → environment → bounded-route gate → LOD/scale → final route gate**

I would not start with the streaming system. Issue #13 explicitly requires a measured native-versus-Blender decision first, and it prohibits Blender during runtime grid loading. ([GitHub][1])

Repository housekeeping and planning can start immediately. Production integration should wait for the M4 gate in #10 to close: #6 is closed, but #10 remains open and owns actor navigation and persistence contracts that M6 must extend. ([GitHub][2])

## Execution model recommendation

Use **Sol X-High** in the Codex runtime. M6 crosses ESM4 parsing, prepared schemas, NIF conversion, terrain generation, physics, asynchronous Bevy lifecycle management, navigation, persistence, custom rendering, and real-data performance evaluation.

In a Claude runtime, use an **Opus orchestrator**. Use Opus executors for the conversion decision, streaming, navigation integration, and environment integration; Sonnet is sufficient for isolated parser, CLI, diagnostic, and fixture issues.

Every wave should follow the existing repository convention:

1. Create and assign all child issues under #13.
2. Write `docs/plans/M6_WAVE<N>_PROMPT.md`.
3. Write `docs/plans/M6_WAVE<N>_PLAN.md` with a fixed feature list.
4. Write feature and unit tests before implementation.
5. Merge through `m6-wave<N>`.
6. Write `docs/plans/M6_WAVE<N>_MANUAL.md` before opening the wave PR.
7. Comment measured results on each issue.
8. Put newly discovered scope in separate follow-up issues.
9. Amend the plan under “Shipped amendments”; do not rewrite the original plan.

Those steps, including isolated worktrees, explicit file ownership, visible diagnostic surfaces, and mandatory manual scripts, are required by `AGENTS.md`. ([GitHub][3])

## Architectural guardrails

The implementation should extend existing seams rather than introduce a second exterior engine:

* Extend the deterministic `CellMap`, which already records worldspace membership and exterior grid coordinates. 
* Extend `PreparedSceneManifest` with optional exterior data and introduce one small worldspace index that references per-cell manifests. Do not create a monolithic world manifest containing all geometry.
* Put project-owned preparation code under a cohesive `src/vsa/exterior/` slice. Any OpenMW-derived decoding remains isolated under `src/vsa/openmw_esm4/`.
* Put streaming under `src/viewer/world/exterior/`, exposed through one typed plugin added to `ViewerPlugins`.
* Keep residency, prioritization, cancellation, ownership, and weather-selection decisions in Bevy-free policy modules. Bevy systems should only apply their outputs.
* Extend the existing day/night policy and `DayNightPlugin`; do not introduce a second clock or environmental-lighting authority. The repository already has a pure time-of-day policy and a runtime day/night system. 
* Retain the existing interior preparation path. Native Rust NIF preparation is
  now the production path for exterior assets and actor animation; the
  retained `blender_script.py` is a legacy reference/preview artifact, not a
  required backend.
* Every serialized shape change must bump its prepared revision. ([GitHub][3])
* Runtime loading accepts prepared artifacts only. It must not know how to invoke Blender, the native NIF converter, or any external preparation command.

A suggested ownership layout is:

```text
src/vsa/exterior/
    index.rs
    coordinates.rs
    terrain/
    objects/
    materials/
    package/
    diagnostics.rs

src/viewer/world/exterior/
    mod.rs
    plugin.rs
    policy.rs
    lifecycle.rs
    loading.rs
    ownership.rs
    diagnostics.rs
    tests/

crates/bevyout-core/src/manifest/
    exterior.rs
```

The exact split can change during implementation, but the authorities should remain clear:

* The worldspace index owns grid-to-package lookup.
* The streaming state owns cell lifecycle.
* The persistence layer owns canonical mutations and player location.
* The environment state owns time, climate, weather, and transitions.
* Each spawned cell has one entity root so unload is complete and deterministic.
* Persistent worldspace references have worldspace ownership, not duplicate per-cell ownership.

---

# Wave 1 — Decide the exterior conversion pipeline

**Branch:** `m6-wave1`

**Recommended shape:** single-issue decision wave.

### Proposed child issue

`[M6/import] Decide the exterior conversion pipeline from a measured real-data corpus`

### Fixed feature list

1. Build a deterministic exterior model corpus containing:

   * Assets used by the bounded M6A route.
   * Large architectural statics.
   * Rocks and terrain-adjacent statics.
   * Alpha-tested foliage.
   * Emissive and environment-mapped materials.
   * Collision-bearing models.
   * Doors or other externally placed animated objects.
   * At least one known lossy or unsupported NIF.

2. Run both preparation paths against the same source bytes:

   * Experimental native conversion.
   * Existing Blender-based conversion in offline bulk mode.

3. Produce a machine-readable comparison report containing:

   * Success, warning, and failure counts.
   * Source and output hashes.
   * Mesh, vertex, triangle, material, and texture counts.
   * Bounds and root-transform differences.
   * Missing textures and lossy features.
   * Collision shape/body counts.
   * Cold preparation time.
   * Warm-cache preparation time.
   * Output and cache bytes.
   * Cache hits, misses, and rebuilt artifacts.

4. Add a visible inspection surface:

   * Prefer a deterministic `nif compare` or `exterior-conversion-report` CLI command.
   * Optionally add a viewer mode that places native and Blender outputs side by side.

5. Record one explicit decision:

   * **Native exterior preparation**, or
   * **Bulk Blender exterior preparation**, or
   * **Documented offline hybrid**, where unsupported classes use Blender during preparation only.

6. Do not add a generic converter abstraction before the comparison establishes that two production backends genuinely need to coexist.

### Tests first

* Identical corpus inputs produce byte-stable reports apart from explicitly separated timing fields.
* Warm-cache runs do not invoke either converter for valid artifacts.
* Failed assets produce deterministic diagnostics rather than aborting the corpus.
* Cache-key changes invalidate only the affected output.
* Native and Blender report rows use the same source identity.

### Exit criterion

A measured issue comment and plan amendment records the selected pipeline and unresolved asset classes. No later wave is allowed to silently change the pipeline.

If neither path produces acceptable route coverage, stop production waves and open focused converter blockers rather than hiding failures behind runtime fallback.

---

# Wave 2 — Establish the exterior data and coordinate contracts

**Branch:** `m6-wave2`

### Proposed child issues

* `[M6/import] Index WRLD and exterior CELL group hierarchy`
* `[M6/core] Define prepared worldspace, cell-package, and coordinate contracts`
* `[M6/qa] Add deterministic exterior-index diagnostics`

The contract issue lands first. Parser and diagnostic work can then use separate worktrees.

### Fixed feature list

1. Decode and associate:

   * `WRLD` records.
   * Exterior `CELL` records.
   * World-child groups.
   * Persistent, temporary, and visible-distant reference groups.
   * `LAND`, `ROAD`, and exterior `NAVM` ownership.
   * Worldspace climate linkage.
   * Worldspace and cell water defaults.
   * Exterior grid coordinates and absolute placements.

2. Define one coordinate authority:

   * Plugin units to Bevy metres.
   * Axis and rotation conversion.
   * Grid coordinate to absolute origin.
   * Absolute world position to grid coordinate.
   * Border ownership and epsilon rules.
   * Negative-grid behavior.

3. Measure whether global Bevy `f32` coordinates remain stable across the planned route and world bounds.

   * Use global coordinates if measured precision is sufficient.
   * Add origin rebasing only if the measurement demonstrates a real need.
   * Record the decision; do not allow individual subsystems to invent local offsets.

4. Introduce a deterministic prepared worldspace index containing:

   * Content fingerprint and schema revision.
   * Worldspace identity and environmental links.
   * Coordinate policy/revision.
   * Sorted cell entries.
   * Grid coordinate, cell FormID, cell origin, and prepared package path.
   * Persistent/worldspace package references.
   * Missing or malformed cell diagnostics.

5. Extend each existing per-cell manifest with optional exterior blocks rather than creating a parallel manifest model.

6. Freeze the bounded M6A route after indexing:

   * Record exact worldspace, grid coordinates, and cell FormIDs.
   * Include both an exterior/interior transition and water where the real route requires it.
   * Do not guess FormIDs in the plan; obtain them from the deterministic catalog output.

### Tests first

* Synthetic nested groups attach cells and records to the correct worldspace.
* Persistent references appear exactly once.
* Temporary and distant references retain their classification.
* Negative and positive grid coordinates round-trip correctly.
* Adjacent cell origins differ by exactly one coordinate-policy cell span.
* Index output is byte-identical regardless of source traversal order.
* Unknown or malformed records are reported without corrupting neighboring cells.
* Golden tests pin all prepared schema revisions.

### Visible surface

Extend the existing catalog/map functionality or add one narrow command that prints:

```text
worldspace <formid> cells=<n> persistent=<n> land=<n> navm=<n> roads=<n>
cell <formid> grid=<x>,<y> origin=<x>,<y>,<z> package=<path>
```

### Exit criterion

The route can be described entirely by deterministic prepared identities and coordinates without loading Bevy or converting any geometry.

---

# Wave 3 — Prepare self-contained exterior cell packages

**Branch:** `m6-wave3`

### Proposed child issues

* `[M6/import] Convert LAND geometry and terrain materials`
* `[M6/import] Bulk-prepare exterior objects with the selected converter`
* `[M6/physics] Prepare terrain, object, and water collision`
* `[M6/world] Assemble deterministic exterior cell packages`

Terrain and object conversion may run in parallel after their serialized contracts are fixed. Package assembly follows both.

### Fixed feature list

#### Terrain

1. Decode LAND heights, normals, vertex colors, base textures, additional textures, and blend weights.
2. Generate geometry in the Wave 2 coordinate system.
3. Derive shared edge samples deterministically so adjacent cells do not form cracks.
4. Preserve source normals where valid; generate deterministic replacements when absent or malformed.
5. Establish one terrain material representation after a focused fixture comparison:

   * Layered/splat terrain material, or
   * Offline-composed PBR cell textures.
6. Record the disk, preparation, and runtime consequences of the chosen representation.
7. Generate collision from the same height source used by rendering.

#### Exterior objects

1. Resolve each unique model once through the Wave 1 pipeline.
2. Reuse the existing model, texture, physics-sidecar, and cache infrastructure.
3. Preserve absolute placement, rotation, scale, enable state, persistence classification, and relevant semantics.
4. Separate:

   * Cell-owned static objects.
   * Cell-owned dynamic references.
   * Worldspace-persistent references.
   * Distant representations.
5. Do not perform premature static batching that would prevent interaction, persistence, LOD, or deterministic unload.
6. Record unsupported assets in the cell package instead of silently dropping them.

#### Water and collision

1. Resolve effective water type and height from cell/worldspace data.
2. Prepare the water surface descriptor separately from terrain.
3. Generate terrain and static-object collision entirely during preparation.
4. Confirm that dynamic and interactive objects remain individually owned.
5. Include enough metadata for later swimming, breath, and visual-water integration.

#### Package assembly

Each cell package should contain references to:

* Terrain render data.
* Terrain collision.
* Water descriptor.
* Cell-owned placements.
* Dynamic reference descriptors.
* Local-light descriptors.
* Navigation artifact.
* Environmental overrides.
* Diagnostics and unsupported-record summaries.

The package must not contain source NIF conversion instructions.

### Tests first

* Flat, sloped, and irregular synthetic LAND fixtures decode correctly.
* Adjacent LAND fixtures produce identical border positions and compatible normals.
* Layer ordering and blend weights are deterministic.
* Rendered terrain and collision surface agree within a fixed tolerance.
* Water precedence and height selection are deterministic.
* Duplicate object models resolve to one prepared asset.
* Persistent references cannot be emitted into two cell-owned packages.
* Package serialization is deterministic and revisioned.
* Missing models or textures are represented by explicit diagnostics/fallbacks.

### Real-data acceptance

Load the frozen M6A cell set all at once, without streaming. Verify:

* Terrain seams.
* Terrain and object collision.
* Object placement and scale.
* Water height.
* Persistent reference ownership.
* Cache reuse.
* No converter process after preparation has completed.

### Exit criterion

A fixed exterior cell set can be prepared, cached, and rendered as self-contained packages. Runtime viewing succeeds with Blender unavailable.

---

# Wave 4 — Add bounded, cancellable exterior streaming

**Branch:** `m6-wave4`

### Proposed child issues

* `[M6/world] Add a pure exterior residency planner`
* `[M6/world] Stream prepared exterior cell packages`
* `[M6/qa] Add streaming lifecycle diagnostics and cancellation soak`

The pure planner can be developed independently. Runtime lifecycle and unload integration should be sequential because they share the world ownership seam.

### Fixed feature list

#### Pure residency policy

Inputs should include:

* Player absolute position and current grid.
* Player velocity or recent movement direction.
* Resident and pending cells.
* Pinned cells.
* Near, prefetch, and optional distant rings.
* Resident-cell or byte budget.
* Failed-cell retry state.

Outputs should be ordered actions such as:

* Request.
* Raise priority.
* Cancel.
* Activate.
* Deactivate.
* Evict.

Priority should be deterministic:

1. Current cell.
2. Immediate safety neighbors.
3. Cells ahead of movement.
4. Remaining cells by distance.
5. FormID or grid tie-break.

#### Runtime lifecycle

Use an explicit lifecycle such as:

```text
Unloaded
  -> Queued(generation)
  -> Loading(generation)
  -> Ready(generation)
  -> Resident
  -> Evicting
  -> Unloaded
```

Include `Failed` with an explicit retry policy.

Requirements:

* A canceled or stale completion may never resurrect a cell.
* Logical cancellation must work even if an underlying asset read cannot be physically interrupted.
* Every load carries a generation/token checked at completion.
* Every cell has one root entity and one ownership record.
* Unload removes terrain, collision, objects, water, lights, navigation, and cell-local handles.
* Dynamic mutations are handed to persistence before entity destruction.
* Worldspace-persistent entities survive cell churn.
* Loading failure leaves the current playable cell intact.
* Runtime code has no path to a converter or Blender invocation.
* A hard resident budget prevents unbounded growth.
* Cache residency and entity residency are reported separately.

#### Diagnostics

Add stable runtime commands such as:

```text
worldstream status
worldstream cells
worldstream cell <formid>
worldstream trace 0|1
worldstream budget
```

Report:

* Desired, queued, loading, ready, resident, and evicting counts.
* Current grid and prefetch direction.
* Resident and estimated asset bytes.
* Load latency.
* Cancellations and stale completions.
* Eviction reason.
* Failed cells.
* Converter invocation count, which must remain zero.

### Tests first

* Desired sets at cell centers and boundaries.
* Velocity-biased prefetching.
* Deterministic ordering under equal distances.
* Resident-budget enforcement.
* A reversed movement direction cancels obsolete requests.
* A stale completion is discarded.
* Current and safety cells cannot be evicted prematurely.
* Rapid teleports cannot leave duplicate roots.
* Minimal Bevy `App` tests cover spawn, ready, activate, cancel, unload, and failure.
* Repeated loops return to the same resident/entity counts.

### Real-data acceptance

Perform:

1. Normal traversal through the bounded cell set.
2. Rapid back-and-forth boundary crossing.
3. Teleports across the bounded set.
4. Several complete loops.
5. A deliberately missing or invalid neighboring package.

Record p50/p95 load-to-ready time, peak residents, peak memory, cancellations, stale completions, and memory after returning to the initial cell.

### Exit criterion

Static exterior traversal is bounded and cancellable, and resident memory returns to a stable plateau after repeated loops. This wave does not yet claim the M6A gate.

---

# Wave 5 — Integrate NAVM, actors, travel, water movement, and persistence

**Branch:** `m6-wave5`

This wave starts after #10 has closed and its actor/persistence contracts are stable.

### Proposed child issues

* `[M6/nav] Stitch streamed exterior NAVM across cell boundaries`
* `[M6/world] Transfer actor and dynamic state through exterior unload`
* `[M6/world] Complete exterior/interior travel and exact return positioning`
* `[M6/physics] Cover route-required swimming, breath, and fall behavior`

Navigation and persistence policies can begin separately. Runtime integration should be merged sequentially because all four issues touch cell activation and player/actor ownership.

### Fixed feature list

#### Exterior navigation

1. Prepare one deterministic navigation artifact per exterior cell.
2. Instantiate one navigation island/tile per resident navigation cell.
3. Match compatible border edges using world coordinates and fixed tolerances.
4. Add cross-cell connections only when both endpoints are resident.
5. Remove links before either tile unloads.
6. Preserve off-mesh door/interior connections.
7. Report unmatched or ambiguous borders.

#### Actors and dynamic references

1. Give each actor and dynamic reference a canonical persistent identity.
2. Distinguish entity residency from world-state existence.
3. On cell unload:

   * Capture authoritative transform and mutable state.
   * Despawn presentation/physics/navigation entities.
   * Keep canonical persisted state.
4. On reload, reconstruct from persisted state rather than original placement when a mutation exists.
5. Ensure schedules and actor destinations remain expressed in absolute world coordinates.

#### Travel

Introduce or extend a canonical location contract along these lines:

```text
WorldLocation::Interior {
    cell_form_id,
    local_position,
    local_rotation,
}

WorldLocation::Exterior {
    worldspace_form_id,
    absolute_position,
    rotation,
}
```

Travel requirements:

* Entering an interior records the exact exterior return anchor.
* Exiting seeds the destination streaming set first.
* Player placement waits until required collision is ready.
* Failure returns safely to the source location.
* Exterior streaming state itself is not serialized; it is reconstructed around the saved location.
* Save/reload works near cell boundaries and during stable interior/exterior states.

#### Water and movement

Give explicit route coverage to:

* Walking.
* Crouching.
* Jumping.
* Swimming entry and exit.
* Breath depletion/recovery.
* Water-surface transitions.
* Fall damage where the route exercises it.

### Tests first

* Two synthetic NAVM tiles create one deterministic border connection.
* Mismatched borders produce diagnostics.
* Unloading either tile removes the cross-cell connection.
* An actor can path from one resident cell into another.
* Actor state survives unload/reload.
* Interior entry and exit preserve the exterior anchor.
* Save/reload chooses the correct active worldspace and grid.
* Boundary saves do not select different cells nondeterministically.
* Swimming and breath transitions use water state rather than rendered-surface visibility.
* Travel failure leaves the source state valid.

### Visible surface

Extend navigation and world console commands to report:

```text
nav exterior
nav borders
worldlocation
worldstate <formid>
waterstate
```

### Exit criterion

A player and test actor can cross exterior cell boundaries, use one exterior/interior door, return to the exact exterior location, and survive save/reload without losing canonical state.

---

# Wave 6 — Implement climate, weather, ImageSpace, and dynamic exterior lighting

**Branch:** `m6-wave6`

### Proposed child issues

* `[M6/render] Resolve worldspace climate, weather, time, and ImageSpace`
* `[M6/render] Extend exterior sun, sky, fog, moon, and water presentation`
* `[M6/render] Budget streamed local lights and preserve interior irradiance`
* `[M6/qa] Add environment diagnostics and transition automation`

### Fixed feature list

#### Pure environment resolver

Add one pure resolver whose inputs include:

* Interior versus exterior state.
* Worldspace.
* Climate.
* Current and target weather.
* Weather-transition progress.
* Game time.
* Cell ImageSpace override.
* Worldspace/climate/weather fallbacks.

It should return a complete resolved environment snapshot:

* Base ImageSpace/HDR parameters.
* Bloom parameters.
* Sun direction, color, and intensity.
* Sky/ambient colors.
* Fog color, near distance, and far distance.
* Moon state.
* Water visual parameters.
* Weather identity and blend state.
* Dynamic-lighting eligibility.

Define and test explicit precedence and fallback rules.

#### Runtime integration

1. Extend the existing `GameClock` and `DayNightPlugin`.
2. Do not add a second time resource or duplicate weather state.
3. Support deterministic weather transitions.
4. Update sun, sky ambient, fog, moon, water, and relevant postprocessing.
5. Activate local lights according to streamed ownership and a deterministic budget.
6. Remove local lights before their owning cell unloads.
7. Preserve baked irradiance for ordinary interiors.
8. Reapply the active interior `CELL.XCIM -> IMGS -> manifest` values on every interior transition.
9. Complete the required data-driven bloom and HDR mapping.
10. Prevent exterior environmental values from leaking into an interior after travel.

These points are required by the bounded-route gate, which specifically calls out interior ImageSpace preservation, climate/weather linkage, deterministic ImageSpace blending, water movement, and recorded performance budgets. ([GitHub][4])

### Tests first

* Time wraps correctly over midnight.
* Weather transition output is deterministic at 0%, 50%, and 100%.
* Climate fallback is deterministic when a linked weather is missing.
* Interior cells use their prepared ImageSpace.
* Ordinary interiors disable exterior dynamic lighting.
* Exterior-like interiors use the existing explicit policy.
* Entering an interior clears exterior fog/sky and reapplies interior values.
* Exiting restores worldspace climate/weather.
* Local-light selection is deterministic and respects the budget.
* A cell unload removes only its owned lights.

### Visible surface

Provide commands such as:

```text
environment status
settime <hour>
setweather <formid> [seconds]
weather clear
lights streamed
```

`environment status` should print resolved source identities and blend values, not only final colors.

### Real-data acceptance

Automate:

1. Noon, sunset, midnight, and sunrise captures.
2. Clear-to-adverse-weather transition.
3. Exterior-to-interior transition during weather blending.
4. Interior-to-exterior return.
5. Cell unload while local lights are active.
6. Water entry under at least two time/weather states.

### Exit criterion

Exterior environment changes dynamically and deterministically, while interiors retain their prepared lighting and ImageSpace behavior.

---

# Wave 7 — Close the bounded exterior gate, #87

**Branch:** `m6-wave7`

This is primarily a gate and hardening wave, not a new architecture wave.

### Proposed issue

Use the existing gate #87. Any nontrivial defect found during acceptance gets a child bug under #13 rather than being hidden in the gate issue.

### Fixed acceptance route

Freeze the exact:

* Plugin set and content fingerprint.
* Worldspace FormID.
* Exterior cell FormIDs and grids.
* Start and destination points.
* Interior transition door.
* Actor used for navigation.
* Water area.
* Weather records.
* Save points.

### Required runs

1. Clean preparation from an empty exterior cache.
2. Warm-cache preparation.
3. Normal bounded-route traversal.
4. Rapid reversal and cancellation traversal.
5. Actor cross-cell route.
6. Exterior/interior/exterior transition.
7. Save/reload in:

   * A cell center.
   * Near a cell boundary.
   * An interior.
   * A water-relevant location where valid.
8. Time and weather transition.
9. Several traversal loops for memory stability.
10. Repository checks and manual script.

### Budgets

Numeric budgets must be frozen before declaring the gate passed. Record hardware and build mode alongside:

* Cold preparation time.
* Warm preparation time.
* Cache size and rebuild size.
* Cell load-to-ready p50/p95.
* Transition latency.
* Peak resident cells.
* Peak process/GPU memory where measurable.
* Memory after repeated loops.
* Frame-time median/p95/max.
* Navigation path latency.
* Stale completion and invalid-unload count.
* Runtime converter/Blender invocation count.

Do not use “looks acceptable” as a budget.

### Exit criterion

Close #87 only when every listed criterion passes on real Fallout data. The gate requires terrain, collision, objects, water, actors, navigation, safe transitions, bounded cancellation, save/reload, ImageSpace, weather, movement, and recorded budgets. ([GitHub][4])

---

# Wave 8 — Add LOD, distant landmarks, occlusion, and controlled pop-in

**Branch:** `m6-wave8`

### Proposed child issues

* `[M6/render] Add seam-safe terrain LOD`
* `[M6/render] Add exterior object LOD and distant landmarks`
* `[M6/render] Add occlusion/culling and controlled pop-in`
* `[M6/qa] Measure route LOD stability and performance`

### Fixed feature list

#### Terrain LOD

1. Define deterministic near, middle, and distant terrain representations.
2. Enforce a bounded LOD difference between neighboring terrain cells.
3. Use skirts, shared-edge topology, or another documented seam solution.
4. Use hysteresis so camera jitter does not repeatedly switch levels.
5. Keep collision at the gameplay representation rather than changing underneath the player.
6. Keep LOD selection separate from residency; a resident cell can change representation without reloading its gameplay state.

#### Object and landmark LOD

1. Preserve source distant/VWD classification where usable.
2. Keep important worldspace-persistent landmarks visible independently of near-cell residency.
3. Select object LOD by deterministic distance and projected-size policy.
4. Prevent duplicate near and distant representations.
5. Use transition hysteresis and, where viable, short fades.
6. Ensure interaction and collision always resolve to the authoritative near representation.

#### Occlusion and culling

1. Measure the available engine-supported occlusion path on the target Bevy version before committing to it.
2. Keep frustum, distance, and true occlusion statistics separate.
3. Apply conservative fallback behavior when occlusion data is unavailable.
4. Never cull navigation, collision, or persistent simulation merely because presentation is hidden.

### Tests first

* Neighboring terrain cells cannot select an invalid LOD delta.
* Shared borders remain compatible across all allowed LOD pairs.
* Hysteresis prevents threshold oscillation.
* Near and distant instances are mutually exclusive.
* Persistent landmarks survive near-cell unload.
* Culling changes presentation only.
* Camera teleports converge to a stable LOD set.
* LOD decisions are deterministic for equal distances.

### Real-data acceptance

Capture and measure:

* Long views toward Megaton.
* Terrain borders at multiple distances.
* Approach and retreat from a large landmark.
* Rapid camera rotation.
* Cell loading while distant representations are visible.
* Frame time with LOD/occlusion enabled and disabled.
* Visible transition counts during the route.

### Exit criterion

The full route can be viewed and traversed without terrain cracks, duplicated landmarks, uncontrolled oscillation, or gameplay state being coupled to presentation LOD.

Once every #13 checklist item has measured evidence, close the implementation epic #13. Issue #14 is explicitly blocked by #13 and #87, so the implementation epic and bounded gate should be complete before the final route gate is declared. ([GitHub][5])

---

# Wave 9 — Pass the Super-Duper Mart ↔ Megaton milestone gate, #14

**Branch:** `m6-wave9`

### Scope

This wave validates the complete route from Super-Duper Mart to Megaton and back. It should contain only gate automation, diagnostics consolidation, documentation, and focused fixes discovered during acceptance.

The gate requires bounded streaming, terrain/collision/placement/LOD agreement, stitched navigation, dynamic environment and water, interior irradiance, correct return positions, save/reload, and recorded long-route budgets. ([GitHub][5])

### Required traversal matrix

Run the route:

1. From a clean exterior cache.
2. From a fully warm cache.
3. In both directions.
4. At daytime and nighttime.
5. During a weather transition.
6. With at least one interior visit and exact return.
7. With save/reload at multiple exterior points.
8. With a test actor traversing part of the route.
9. With one deliberate rapid reversal to exercise cancellation.
10. In repeated loops for memory and ownership stability.

### Consolidated diagnostics

Produce one deterministic route summary containing:

```text
conversion:
  selected_pipeline
  assets_built
  assets_reused
  lossy_assets
  cache_bytes
  cold_seconds
  warm_seconds

streaming:
  cells_requested
  cells_ready
  cells_evicted
  cancellations
  stale_completions
  failed_cells
  peak_resident_cells
  peak_memory
  ending_memory

runtime:
  frame_ms_median
  frame_ms_p95
  frame_ms_max
  transition_ms_p95
  nav_path_ms_p95
  visible_lod_transitions
  blender_invocations
```

### Final repository gates

Run:

```text
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
```

Then run the exact preparation and viewer commands documented in `M6_WAVE9_MANUAL.md`, including MCP/BRP assertions and viewport captures.

### Closure rule

Close #14 only after:

* #87 is closed.
* #13 is closed.
* Both clean-cache and warm-cache routes pass.
* The player can walk there and back.
* Repeated traversal has bounded memory.
* No invalid cancellation or unload state occurs.
* Runtime Blender invocations remain zero.
* All numeric budgets pass.
* The manual script is reproducible by another person.

---

## Dependency and parallelization summary

```text
#10 closes
    |
Wave 1: conversion decision
    |
Wave 2: index, schema, coordinates
    |
Wave 3: terrain + objects + collision + packages
    |
Wave 4: bounded streaming
    |
    +----------------------+
    |                      |
Wave 5: nav/gameplay   Wave 6 policy/parser work
    |                      |
    +---- sequential runtime integration
               |
Wave 7: bounded M6A gate (#87)
               |
Wave 8: LOD/distant/occlusion
               |
Close implementation epic #13
               |
Wave 9: final route gate (#14)
```

The highest-risk seam is not terrain conversion itself; it is coordinating cell ownership across rendering, physics, navigation, actors, local lights, persistence, cancellation, and unload. The plan therefore establishes a single cell-package contract and lifecycle authority before those systems are integrated.

---

## Shipped amendments

### M6 wave 1 — native exterior preparation is authoritative

The requested implementation decision supersedes the original wave-1
comparison requirement: native Rust NIF preparation is now the only production
converter for exterior assets and actor animation. Prepare/render no longer
accept or select a Blender backend, and runtime exterior streaming consumes
prepared RON/GLB artifacts only. `src/vsa/assets/blender_script.py` is retained
as a legacy explicitly-requested preview/reference script and documents the
remaining native gaps at its top.

The native terrain material follow-up now resolves the LAND layer stack into a
revisioned albedo plus optional tangent-space normal/specular map, and the
runtime supplies the terrain tangent contract. Remaining native gaps include
VWD/distant geometry generation and unsupported or lossy NIF blocks. They are
represented as deterministic diagnostics. Wave 1 also shipped the revisioned
exterior index/package contracts, LAND/ROAD ownership fields, native package
asset staging, deterministic catalog output, and the first generation-safe
runtime residency loader. Waves 2–9 still own full route acceptance,
navigation/gameplay continuity, environment transitions, LOD and occlusion,
and the bounded/final route gates.

### M6 wave 1 — real-data empty-world correction

The first real Super-Duper Mart capture revealed that the initial player still
spawned at the default origin while the prepared exterior cell was at grid
`(4, -5)`. Runtime placement now waits for the FPS entity, moves it to the
prepared terrain center, and prevents residency from recalculating around the
origin during that startup race. Bethesda `VHGT` is decoded as row-wise
differentials with the format's 8x height scale; the corrected `00000c49`
terrain now shares the authored 157–166 m elevation range of its placed native
structures. Invalid water-height sentinels are omitted and reported as
`invalid_water_height`. Deterministic duplicate-grid selection and real
adjacent packages close the first empty-world reproduction without adding an
OpenMW runtime dependency. A physics-enabled bridge launch held the player at
`263.314, 158.129, 263.314` on the prepared terrain collider.

The follow-up physics check confirmed that the player controller is
BoxDDD-authoritative. Exterior LAND is therefore inserted into the existing
BoxDDD static mesh path at startup, and native static-object physics sidecars
are loaded and built in the same pass. Real `00000c49` startup reported `117`
static bodies and `11,740` packed triangles. Streamed packages now attach the
same terrain/object path before becoming `Ready`/`Resident`, and eviction
tears down the existing per-cell BoxDDD ledger before removing the package
root. Portal points are producer-emitted world-space Bevy metres; matching is
covered for positive and negative non-zero grids.

The visible `nav exterior` diagnostic reports `223` vertices, `279` triangles,
and `54` border portals for the real package. Prepared navigation now carries
collision-cleared graph data and resident-cell links; `nav exterior` remains a
compact diagnostic view, while `tna spawn` succeeds when the graph and cell
residency are ready.

Acceptance also corrected the streaming handoff ordering: an unready physical
target is requested immediately, but the last collision-ready logical cell is
temporarily pinned so its BoxDDD shapes cannot be evicted first. Real logs now
show `preload generated` -> `preload ready ... collision_ready=1` -> `Activate`
-> old-cell `Evict` -> `colliders teardown`, and the player remains above the
destination terrain during the pending interval.

The real-data viewer also corrected the native LAND triangle winding: the
Bevy-Z row direction had made the terrain's geometric faces point down while
the prepared normals pointed up. Render, BoxDDD, and prepare-side clearance
now use the same upward winding. The remaining coarse 33x33 source height
topology and scalar/vertex-color material were known Wave 1 presentation
limits. The native material follow-up now resolves the LAND texture stack
(`VTEX`, `BTXT`/`ATXT`, `VTXT`, `VCLR`, `LTEX`, and `TXST`) into a revisioned 1024²
cell-local albedo asset, with a live capture confirming that the terrain is
attached and shaded without the former white/black fallback holes. Near
presentation now uses deterministic bilinear subdivision of that source
topology, while full LAND collision remains authoritative; seam-safe
terrain/distant LOD, VWD geometry, and occlusion remain Wave 8 work.

The save/runtime follow-up added backward-compatible v7 exact world-location
records, exact-location startup when the selected manifest matches, streamed
exterior dynamic transform capture/restore, and Wave 6 diagnostics for
environment, weather, water, and the streamed-light budget. Prepared exterior
packages now include the selected content set's compact WTHR keyframes, so
known weather IDs blend their authored sky/ambient/sunlight values; missing
IDs retain a deterministic fallback diagnostic path. Resident terrain LOD
uses hysteresis and cardinal-neighbour clamping while leaving gameplay
collision at the full LAND mesh. These seams still need the bounded real-data
route acceptance in Wave 7 and the final Super-Duper Mart ↔ Megaton gate in
Wave 9; this amendment does not mark those gates complete.

### M6 bounded-route preparation and streaming evidence

The real Fallout 3 route strip is now prepared with the native pipeline from
Super-Duper Mart grid `(4,-5)` through Megaton Main Gate grid `(-1,-5)`:
`00000c49`, `00000c4a`, `00000c4b`, `00000c4c`, `000010d5`, and `00001245`.
The prefetched `y=-6` safety ring is prepared as well, so the runtime route
does not manufacture failures from stale neighboring manifests. A physics-
enabled bridge run crossed all six grids at authored terrain heights with
`failed=0`, `stale_completions=0`, a steady resident count of `4` after
convergence, and successful `tna spawn` at every stop. The prefetch safety ring
can temporarily raise the measured peak above that steady route count. The
observed forward run ended at `1,129,295` resident bytes after `13` evictions;
this is evidence for the bounded streaming seam, not a frozen performance
budget.

The exterior preparation path now applies its existing failed-native-asset
fallback before reading physics sidecars. Non-renderable `giantant/skeleton`
and `marker_radiation` references are retained as explicit diagnostics instead
of aborting the whole Megaton package.

This evidence does not close Wave 7 or Wave 9: the exact clean-cache/warm-cache
timings, long-route player input traversal, interior return, water route,
actor path, ImageSpace captures, and final frame/memory budgets still require
the dedicated acceptance manuals and a human-run route session.

### M6 — current native terrain and package-availability verification

The native terrain follow-up now emits a revisioned 1024² cell-local albedo,
plus an optional tangent-space normal map and specular-strength channel from
the resolved `TXST` source. The source LAND height topology remains authored
33x33, while near presentation uses deterministic bilinear subdivision and the
runtime keeps full-resolution LAND collision; the remaining faceted far-field
appearance is therefore a geometry/LOD limitation, not evidence of inverted
normals. The focused mesh regression computes a positive geometric Y normal
for the first flat quad, and the current bridge capture shows the player
standing on the textured, top-facing surface.

Exterior residency is now a separate 25-cell budget (the intended 5x5
`uGridsToLoad` footprint). At startup the runtime filters the full worldspace
index to package files that exist and whose header carries the current package
revision, so stale v3/v6 cache files are not requested as if they were valid
neighbors. The real `00000c49` bridge run reports `failed=0`,
`stale_completions=0`, `resident_budget=25`, `requests=6`, `ready=6`, and
`peak_resident_cells=1` at startup; the prepared route currently contributes
14 current-revision packages. This is a cache-availability proof, not the
final long-route budget gate.

### M6 — cross-cell navigation rebuild guard

The first resident-graph route probe exposed a panic in Landmass `0.9.2`
(`i_float` converting `NaN` while clipping a boundary). The trigger was an
animation link with a semantically valid point source portal sharing a node
with a native terrain boundary link; Landmass's boundary clipping path
normalized that point even though it was not a boundary link. The viewer
adapter now gives each animation-link source a finite 1 cm horizontal portal
while preserving the point destination used for animation-link sampling.
This is a local compatibility guard until the upstream filtering fix is
available, and it is covered by the link-spawn regression test.

With the guard, the physics-enabled bridge probe crossed the real route strip
through grids `(4,-5)`, `(3,-5)`, `(2,-5)`, `(1,-5)`, `(0,-5)`, and `(-1,-5)`
at authored terrain heights. Every stop kept the bridge alive with
`failed=0`, `stale_completions=0`, and the 25-cell exterior budget; the final
probe reported four resident cells. This is stronger route-streaming and
navigation-rebuild evidence, but it used `player.setpos` to isolate cell
seams, so it does not close the Wave 7/Wave 9 ordinary-input, reversal,
interior, water, actor, or performance gates.

### M6 — presentation diagnostics and atomic acceptance teleports

The viewer now exposes `worldstream presentation`. It reports resident terrain
representation counts and LOD transitions, distance-hidden object roots,
persistent/distant landmark counts, and separate frustum, distance, and true
occlusion fields. Bevy's camera `OcclusionCulling` marker is reported as
engine-enabled, but true per-object occlusion counts remain explicitly
unmeasured because the target Bevy renderer does not expose a stable public
counter; presentation therefore falls back conservatively and never removes
collision, navigation, or persistent simulation.

The Wave 7 manual uses the atomic `tp x y z` command for seam probes so a
physics tick cannot observe a player after only one axis has changed. This is
diagnostic isolation only; ordinary keyboard traversal remains required for
the Wave 7 and Wave 9 gates.

`worldstream summary` now consolidates the live streaming counters, the
presentation report, and the current frame window in the same shape as the
Wave 9 route summary. Preparation/cache and transition/path timings remain
explicit `null` inputs until the corresponding clean/warm and ordinary-input
acceptance runs record them; the summary marks those offline measurements as
required instead of presenting placeholders as passing budgets.

### M6 — terrain LOD elevation-center correction and route recheck

The first westward LOD probe exposed a presentation-only distance bug: terrain
LOD centers used the cell origin's zero elevation instead of the authored LAND
center vertex around 160–176 m. The viewer now shares the authored terrain
center with player placement, so the real route no longer collapses to the
distant mesh merely because the player crossed west of the Mart. The new
regression covers this elevation reference, while full LAND collision remains
unchanged.

The fresh bridge run reached grids `(3,-5)`, `(2,-5)`, `(1,-5)`, `(0,-5)`, and
`(-1,-5)` after the initial `(4,-5)` cell with `failed=0`,
`stale_completions=0`, successful `tna spawn` at every stop, and terrain LOD
sets retaining near/middle representations throughout the route. The live
summary reached `peak_resident_cells=9` and `peak_memory=2218157` while the
steady resident count stayed at 2–4; it recorded 13 requests and 6 evictions.
The current frame window was p50 `7.1869 ms`, p95 `9.4428 ms`, max `9.8995 ms`
over 64 samples against the 16.6667 ms probe budget. This remains diagnostic
teleport evidence, not closure of the ordinary-input, clean/warm, interior,
water, actor, or final-loop gates.

The route preparation was also measured in an isolated temporary cache with
four workers: the clean pass took `131.691 s` and finished `14 done, 0 failed`;
the warm pass took `8.794 s` with `14` current fingerprints and `0` stale;
the resulting cache occupied `772183616` bytes across `951` files. Native
conversion attempted `191` model jobs; three source-authored non-renderable
models (`creatures/eyebot/skeleton.nif`, `creatures/giantant/skeleton.nif`,
and `marker_radiation.nif`) remained explicit fallback diagnostics while all
14 exterior packages completed. These figures are offline evidence and are
not injected into the live viewer's intentionally-null measurement fields.

### M6 — LAND orientation and true layer-stack verification

The real `00000c49` terrain is not upside down. The native mesh and BoxDDD
collider use the same counter-clockwise winding; the focused mesh regression
computes a positive geometric Bevy-Y normal. The source `VNML` samples also
convert to positive Bevy-Y normals for all 1,089 LAND vertices (minimum
observed Y component `0.7197` after normalization). The earlier close-up free
camera images were taken with `tfc` and were therefore not a reliable test of
surface orientation.

The native material path now preserves the ESM4 LAND texture graph instead of
collapsing it to four channels. `VTEX` IDs are retained, `BTXT` and `ATXT`
FormIDs are resolver-adjusted, `ATXT.layerIndex` is read as its authored
little-endian `u16`, zero BTXT slots use Fallout 3's default wasteland dirt,
and each quadrant composites its base and ordered `VTXT` alpha maps. The bake
uses the OpenMW ESM4 sampling density of six texture tiles per quadrant (12
across one cell) and still produces one self-contained 1024² albedo/normal
asset for runtime streaming. A synthetic seven-layer fixture and the real
`00000c49` preparation both pass; the real baked albedo has no fallback white
holes. Terrain reflectance is bounded to `0.25` so the source normal alpha does
not make the matte ground read as polished metal.

The apparent low resolution is a separate limitation. Fallout LAND provides
only a 33x33 height/normal/color grid per 4096-unit cell; the near 129x129
mesh is bilinear subdivision and cannot invent additional height detail. The
middle and distant representations are intentionally coarser, and pre-baked
VWD/distant geometry plus its atlas remains Wave 8 work. This is why the
far-field screenshot can still look sparse or faceted even though the near
terrain orientation and material layer composition are now correct.

### M6 — far-worldspace LOD is optional and deferred from the route gate

Native preparation now discovers Fallout 3's Wasteland worldspace LOD archive
and records 1,608 deterministic descriptors (1,360 terrain tiles and 276
block/landmark meshes). Terrain tiles use a dedicated native conversion profile
that removes only the authored vertical/degenerate border-skirt triangles that
render as detached walls in Bevy; block meshes retain their authored faces.
The preserved NIF root already contains the tile's worldspace origin, so the
viewer attaches each imported GLB at identity rather than applying a second
grid translation. Empty post-trim meshes are dropped before GLTF emission.

The runtime support remains bounded and presentation-only: at most 48 tiles
are active, at most eight new imports are staged per frame, and terrain bands
have separate budgets so Level 4 tiles cannot starve the farther bands. It is
now opt-in with `view`/`render --worldspace-lod` or
`setrender worldspace_lod 1`; normal launches leave it disabled, while the
near/middle/distant per-cell terrain LOD and full LAND collision remain on.
Disabling the setting removes active far tiles without touching residency,
collision, navigation, or persistent objects.

Real `00000c49` bridge verification launched without the flag and reported
`worldspace_lod.active=0`; enabling the setting reached the bounded
`48`-tile set (`40` terrain and `8` blocks), and disabling it returned to zero
without a crash or stack-overflow diagnostic. This defers far-horizon visual
polish from the Wave 7/Wave 9 gameplay route gate while preserving a measured
experimentation path for later hardware/visual review. The separate
`M6_WAVE8_MANUAL.md` records the opt-in check.

### M6 — short cross-cell merge handoff correction

The focused c49 navigation probe confirmed that the Super-Duper Mart seam link
was installed, present in the agent corridor, and selected as the next path
step. The runtime failure was therefore in the handoff lifecycle: the far
portal endpoint was close enough to the capsule that the fixed `0.5 m` merge
arrival tolerance completed the crossing on the source side. Capturing a
per-crossing tolerance capped at half the initial distance guarantees a real
KCC step before completion. Acceptance then exposed a second lifecycle edge:
`ReachedAnimationLink3d` remains present during the sweep, while
`door_link_system` runs before `merge_traversal_system`; the former was
restarting the traversal and resetting its timeout every fixed tick. The
handoff now leaves an existing `MergeTraversal` untouched.

The regression covers both the clear and collision-blocked traversal paths,
the active-traversal re-entry guard, and the full repository tests/clippy gate.
A live physics-enabled c49 bridge run followed the same
`(180,176.35,275.30)` to `(235.92,158.53,243.29)` route through the seam and
ended `status=reached`, `blocked=false`, and `stuck=false`. This closes the
specific short-portal runtime defect; the Wave 7/Wave 9 ordinary-input,
interior, water, actor, save/reload, loop, and agreed-budget matrix remains
separate acceptance work.

[1]: https://github.com/kelo221/bevyout/issues/13 "[Epic] M6 — Exterior conversion, streaming, and dynamic lighting · Issue #13 · kelo221/bevyout · GitHub"
[2]: https://github.com/kelo221/bevyout/issues/10 "[Gate] M4 — Actors navigate, schedule, and persist · Issue #10 · kelo221/bevyout · GitHub"
[3]: https://github.com/kelo221/bevyout/blob/master/AGENTS.md "bevyout/AGENTS.md at master · kelo221/bevyout · GitHub"
[4]: https://github.com/kelo221/bevyout/issues/87 "[Gate] M6A — A bounded exterior route streams and plays correctly · Issue #87 · kelo221/bevyout · GitHub"
[5]: https://github.com/kelo221/bevyout/issues/14 "[Gate] M6 — Super-Duper Mart to Megaton exterior route works · Issue #14 · kelo221/bevyout · GitHub"
