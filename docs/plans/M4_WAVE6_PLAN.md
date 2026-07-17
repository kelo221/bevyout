# M4 wave 6 — nav clearance, note text, real corpses, Pip-Boy click actions (#136, #123, #120, #121)

Wave on branch `m4-wave6` off master. Four executors in parallel
worktrees with disjoint file ownership; the shared merge seam is
`tests/features.rs` (each issue appends World fields at the end of the
struct and a delimited step section at the end of the file). Model
routing per AGENTS.md: executors (Sonnet) write all production and test
code; the orchestrator owns this plan, merges, review, gates, and
real-data acceptance.

## Issue #136 — navmesh clearance for agent radius

The wave-5 measurement (comment on #136) settled the decision: physics
alone does not give clearance — the capsule wedges at constrictions in
Vault 101 Entrance. Build the scoped fix.

### Feature list

- **F136.1 Radius-aware clearance in `landmass_graph` conversion.**
  Prefer landmass's own radius/smoothing options if the pinned version
  exposes them; otherwise erode the converted navmesh polygons inward by
  agent radius (0.35 m) at runtime conversion. Pure-module erosion
  logic, deterministic, unit- and cucumber-tested on synthetic meshes.
- **F136.2 Corridor-pinch safety.** Erosion must not pinch narrow FO3
  corridors shut: where full erosion would disconnect a previously
  connected walkable region, fall back per-polygon to partial/no erosion
  (deterministic rule, tested with a synthetic narrow-corridor mesh).
- **F136.3 Diagnostics.** Conversion logs a stable line
  (`nav erosion: polys <n> eroded <m> pinch-guard <k>`) so acceptance
  can assert the fix is active.

### Files owned

`src/viewer/nav/landmass_graph.rs`, new pure module under
`src/viewer/nav/` (e.g. `erosion_policy.rs`), `features/nav_erosion.feature`
(new), own delimited section of `tests/features.rs`. If the fix instead
lands in prepared data (`src/vsa/prepare/nav_graph.rs`), bump
`NAV_GRAPH_REVISION` — runtime-conversion is preferred to avoid cache
invalidation.

### Acceptance (orchestrator, real data)

Vault 101 Entrance (00024512): the wave-5 wedge route — `tna goto`
toward door 00028579 and past it — completes `reached` with no
`collision-blocked`/`stuck`; FranklinMetro02 (0001a273) narrow metro
corridors remain routable.

## Issue #123 — NOTE record text decode

### Feature list

- **F123.1 Decode NOTE text.** Decode the FO3 NOTE text subrecord
  (issue hints `TNAM`/`XNAM`/`DESC`; verify against real records) in
  `src/vsa/openmw_esm4/`, following that isolated folder's porting
  conventions.
- **F123.2 Catalog plumbing.** Prepared item catalog carries the note
  text; bump `ITEM_CATALOG_REVISION`.
- **F123.3 Tests.** Synthetic record fixture proves text decodes;
  cucumber scenario proves a prepared NOTE lands in the catalog with
  text.

### Files owned

`src/vsa/openmw_esm4/` NOTE decode, `src/vsa/prepare/items.rs`,
`features/note_text.feature` (new), own section of `tests/features.rs`.

### Acceptance (orchestrator, real data)

Re-prepare a cell with a known holotape; the Pip-Boy reader renders its
text.

## Issue #120 — real dead actors as lootable corpses

Feature list F119.1–F119.4 as written on the issue: detect
source-authored dead state from parsed actor/reference data (no
editor-ID string matching), classify as `PreparedSemantic::Corpse`
preserving reference/transform/identity/inventory, existing activation
and transfer UI just work. No combat/death simulation.

### Files owned

`src/vsa/prepare/actor_catalog.rs`, `src/vsa/prepare/placements.rs`,
dead-state field decode in `src/vsa/openmw_esm4/` if missing (flags in
ACHR/NPC_ records — coordinate note: #123 also touches this folder, keep
to separate record types), `features/real_corpses.feature` (new), own
section of `tests/features.rs`. Bump `ACTOR_CATALOG_REVISION` (and any
other prepared type whose serialized shape changes).

### Acceptance (orchestrator, real data)

Cell 00028138: activate reference 00054398 (`CG04DeadOldLady`), transfer
UI opens, take/take-all persist across save/reload.

## Issue #121 — Pip-Boy Items click = primary action

### Feature list

- **F121.1 Click dispatch.** Clicking an eligible Items row triggers its
  primary action — equip/unequip for Weapons/Apparel/Ammo, use for Aid,
  read for books/notes — via the existing rules modules
  (`player::equipment`, `interaction::item_use`), unchanged. E key and
  the details-pane button keep working.
- **F121.2 Tests.** Pure eligibility/dispatch decision covered by unit
  or cucumber tests; UI system covered by a minimal-`App` test per the
  console-harness pattern.

### Files owned

`src/viewer/pipboy.rs` and `src/viewer/pipboy_reader*` UI wiring only,
`features/pipboy_click.feature` (new if cucumber is used), own section
of `tests/features.rs`.

### Acceptance (orchestrator, real data)

In a prepared cell with a weapon and an aid item in inventory: single
click on the weapon row equips it; click on aid consumes it; book opens
the reader.

## Gates

Per executor before reporting: `cargo fmt --check`,
`cargo clippy --all-targets -- -D warnings`, `cargo test`. Orchestrator
after merge: same gates on `m4-wave6`, then real-data acceptance above,
then `docs/plans/M4_WAVE6_MANUAL.md` and one PR with `Closes` lines for
all four issues.

## Shipped amendments (real-data acceptance)

- **A1 (#136) Shared-vertex relaxation.** The per-polygon pinch guard
  couldn't see a neighboring triangle inverting through a shared
  boundary vertex; Vault 101 Entrance failed landmass validation
  entirely (`nav_mesh_invalid`, polygon 164). Replaced by a two-phase
  global relaxation over the whole mesh (proportional factor-halving,
  then hard zero-displacement fallback) with a termination guarantee.
- **A2 (#136) Sliver pinning.** The same polygon 164 turned out to be a
  source-data sliver (original XZ area −7.7e-6, four orders of magnitude
  below any genuine triangle); erosion "resurrected" it with arbitrary
  winding. Polygons whose original area is below
  `MIN_RELIABLE_ORIGINAL_AREA` now pin to zero displacement.
- **A3 (#136) Arrival resets stuck.** After erosion, a raw `tna goto`
  target's reachable point can sit past the 0.5 m distance check even
  though landmass reports `ReachedTarget`; the no-progress detector then
  latched a false `stuck` on finished routes. Arrival (either signal)
  now resets stuck detection.
- **A4 (#136) Protected seam edges.** Cross-mesh connectivity uses
  generated links between matched triangle midpoints (wave 4), so seam
  edges look like walls to a single mesh; eroding both sides opened a
  ~0.7 m gap and made cross-seam targets `unreachable` on
  FranklinMetro02. Door-link and merge-triangle edges are now excluded
  from erosion (`protected` count in the log line).
- **A5 (#136) FranklinMetro02 corridor wedge is pre-existing.** With all
  of the above, the metro seam route exists but the agent still
  collision-blocks ~0.5 m from spawn — verified **identical on the
  pre-wave master build**, so it is not an erosion regression; filed as
  #148 (interior collider overlapping walkable navmesh) instead of
  expanding the wave.
- **A6 (#120) The FO3 starts-dead flag is on the base record.** OpenMW's
  documented ACHR header bit 0x200 appears on 0 of 1454 ACHRs in
  Fallout3.esm; the real marker is NPC_ record-header bit 0x80000
  (174 bases, all corpse actors — survey in the constant's doc comment).
  Classification reworked accordingly; the ACHR bit remains as a
  documented secondary condition.
- **A7 (#120) Corpse placeholder body.** Actor placements never spawn
  scene entities (no asset path), so a prepared corpse was unreachable
  by the MeshRayCast activation path. Corpse-semantic placements now
  spawn a placeholder prone primitive (explicitly temporary until
  #106–#108 actor bodies) carrying the #118 activation components.
- **A8 (#120) Prepare revision bumps.** The classification change alters
  what cached `scene.ron` placements mean without changing their shape;
  `CURRENT_PREPARE_REVISION` → `prepare-corpse-v2` and
  `PREPARE_PIPELINE_REVISION` → `prepare-corpse-v1` so both the viewer
  staleness check and the batch-resume fingerprint reject stale scenes.
- **A9 (#121) No cucumber feature.** The only pure logic is the one-row
  `row_primary_action` decision, unit-tested directly; UI dispatch is
  covered by App-level tests. A feature file would have added no value.
