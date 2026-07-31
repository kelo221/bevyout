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
* Retain the existing interior preparation path. The current native NIF converter is explicitly experimental and leaves established Blender preparation unchanged. 
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

The current gaps are LAND terrain material/layer fidelity, VWD/distant
geometry generation, and unsupported or lossy NIF blocks. They are represented
as deterministic diagnostics. Wave 1 also shipped the revisioned exterior
index/package contracts, LAND/ROAD ownership fields, native package asset
staging, deterministic catalog output, and the first generation-safe runtime
residency loader. Waves 2–9 still own full route acceptance, terrain material
fidelity, navigation/gameplay continuity, environment transitions, LOD and
occlusion, and the bounded/final route gates.

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

[1]: https://github.com/kelo221/bevyout/issues/13 "[Epic] M6 — Exterior conversion, streaming, and dynamic lighting · Issue #13 · kelo221/bevyout · GitHub"
[2]: https://github.com/kelo221/bevyout/issues/10 "[Gate] M4 — Actors navigate, schedule, and persist · Issue #10 · kelo221/bevyout · GitHub"
[3]: https://github.com/kelo221/bevyout/blob/master/AGENTS.md "bevyout/AGENTS.md at master · kelo221/bevyout · GitHub"
[4]: https://github.com/kelo221/bevyout/issues/87 "[Gate] M6A — A bounded exterior route streams and plays correctly · Issue #87 · kelo221/bevyout · GitHub"
[5]: https://github.com/kelo221/bevyout/issues/14 "[Gate] M6 — Super-Duper Mart to Megaton exterior route works · Issue #14 · kelo221/bevyout · GitHub"
