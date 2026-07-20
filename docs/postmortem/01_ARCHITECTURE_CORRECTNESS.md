# Postmortem 01 — Architecture Correctness of the bevyout Navmesh Solution

Analyst 1 of 4. Read-only. Branch `m4-wave11-177-doors`. No code changes.

## Verdict

The architecture is sound at the level that matters most — physics-authoritative
movement, doors as query-time route topology rather than baked geometry, and
validating (not blindly trusting) authored NAVM against cooked collision — and
every one of the seven "key choices" in the brief is a considered, documented,
acceptance-tested decision rather than an accident, which is unusual and worth
crediting. But it is a design that has been discovered one wedge point at a
time (`#148`→`#153`→`#171`→`#177`, four waves to find that a closed `ACTI`
door was invisible to route topology) rather than derived from a stated
invariant up front, and that reactive posture has left two open, self-admitted
structural risks — a 45× polygon-count blowup from the sub-triangle clip
(`#179`) and a cross-mesh portal system with zero authored evidence to
validate against on the only cells tested (`#156` wave-9 amendment A5) — that
will matter a great deal at M6 exterior scale and have not yet been
re-examined at that scale. The comparison to OpenMW is asymmetric in a way the
brief should account for: OpenMW's ESM4 `loadnavm.cpp` does not decode FO3/FNV
`NAVM` at all (it only handles the newer Skyrim/FO4 `NVNM` chunk and
explicitly skips `NVER`/`DATA`/`NVVX`/`NVTR`/`NVCA`/`NVDP`/`NVGD`/`NVEX`), so
OpenMW is useful here only as an AI-package/movement-model reference, never as
a working navmesh-*decode* reference — a fact this codebase's own module doc
already states correctly (`src/vsa/openmw_esm4/navmesh.rs:1-13`).

## What is architecturally sound

**1. Authored NAVM as base topology, validated against cooked collision,
rather than a Recast-style rebuild.** Defensible, for a reason the brief
doesn't mention: Bethesda's own GECK navmesh is *itself* collision-derived —
"NavMesh Generation is a group of tools responsible for creating NavMeshes
from a cell's Havok data" — then hand-finalized by designers (cover-edge
detection, door portal creation, cell-edge linking)
[GECK Wiki, NavMesh Generation](https://geckwiki.com/index.php/NavMesh_Generation).
A pure from-scratch Recast rebuild, OpenMW's approach
([PR #1633](https://github.com/OpenMW/openmw/pull/1633): "the navmesh should
probably be built at runtime… because different mods add different objects to
cells"), would discard exactly the hand-placed refinement GECK's finalize pass
added and that no geometry-only rebuild can recover. Keeping the authored
topology and validating it against the project's own cooked collision
(`src/vsa/prepare/nav_clearance.rs:1-45`) is the correct middle path for
content this static (no user mods changing cell geometry, unlike OpenMW's
motivating case). The cost is real and the codebase does not hide it: three
full waves (#153, #171, plus #177) to get from "authored NAVM" to "navmesh
that agrees with collision," versus OpenMW's single unified Recast build. That
OpenMW itself broke this consistency in its 0.46→0.47 release — "actors'
collision shape was equal in the physics movement solver and pathfinding, but
with 0.47 it became different" — is independent evidence that this is a hard
problem in *either* architecture, not one this project chose to make harder
for itself
([OpenMW #6138](https://gitlab.com/OpenMW/openmw/-/issues/6138)).

**2. Physics-authoritative movement.** `src/viewer/nav/agent.rs:8-15`:
"navigation proposes, physics disposes" — landmass computes desired velocity,
the `bevy_boxddd` capsule KCC resolves collision/steps/slopes and moves the
transform, and the *achieved* velocity feeds back into landmass. This is the
same shape OpenMW settled on: `MWPhysics::Actor` runs real Bullet collision
under every actor regardless of navmesh state, and the AI layer only steers a
desired direction into it (`apps/openmw/mwmechanics/aipackage.cpp`'s
`isDoorOnTheWay`/`hasCollisionWithDoor` calls query live physics, not
precomputed navmesh state — verified directly from
[`aipackage.cpp`](https://raw.githubusercontent.com/OpenMW/openmw/master/apps/openmw/mwmechanics/aipackage.cpp)).
Sound, and it is what makes choice 3 below correctness-safe: an agent that
somehow ends up standing where the navmesh says it shouldn't still doesn't
clip through a wall, because physics — not the mesh — owns collision.

**3. Doors as state-dependent route topology rather than baked geometry.**
`LOCKED_DOOR_TYPE_INDEX_COST` (finite 1000.0 for closed-but-openable,
`f32::INFINITY` for locked/non-openable, `src/viewer/nav/agent.rs:277-312`)
plus per-agent `AgentTypeIndexCostOverrides` rebuilt on every
`door_availability_system` flip is architecturally the right primitive:
landmass explicitly supports per-agent, per-type-index query-time costs, and
this is functionally the same shape as Detour's `dtTileCache` temporary
obstacles ("convex obstacles are ideal for marking a door… temporary obstacles
are sufficient for most situations, including doors" — general Recast/Detour
practice), just implemented as a cost multiplier instead of voxel carving. It
is a better fit for this project's per-actor lock/key semantics than a single
shared obstacle would be. Compared against OpenMW's actual reference model,
this project's own issue #185 already did the comparison rigorously (citing
`AiPackage::openDoors()` and `AiAvoidDoor` by function name) and the match is
good: unlocked-activates, locked-with-no-key-stays-shut, teleport-doors-on-a-
separate-path all line up. Where bevyout is *better* than OpenMW's reference:
because the door state is priced into the A* query itself, an unreachable
route fails at plan time (`AgentState::NoPath`) instead of OpenMW's actors
physically walking into a closed door and needing `AiAvoidDoor`'s reactive
"turn away and retry" recovery (`aiavoiddoor.cpp`, described in #185's body).
Where it is currently *behind*: OpenMW's `openDoors()` treats "locked" as a
property of the (door, actor) pair via key possession
(`invStore.search(keyId)`); bevyout's lock cost is currently door-only
(binary blocked for every agent) — tracked, open, `#185`.

**4. Navmesh-border ORCA avoidance disabled, agent-agent avoidance kept.**
This is a targeted, well-diagnosed workaround for a specific backend
limitation, not an abandonment of the general technique. Treating navmesh
borders as RVO/ORCA obstacles is standard elsewhere (the A* Pathfinding
Project's `RVONavmesh` component and RVO2's native polygon-obstacle support do
exactly this). The root cause bevyout found is specific to `landmass`'s
`dodgy_2d` backend: `nav_mesh_borders_to_dodgy_obstacles` walks connected
polygons and projects every border edge onto the XZ plane with no vertical
awareness, so a staircase's railing "gets flattened onto the landing's own
footprint" — measured concretely on cell `00024512`: 125 border edges
spanning `y 39.17..40.47`, and an agent with 1.35 m of genuinely free space
decaying to a permanent halt (`reason=no_contact_no_progress`,
`src/viewer/nav/agent.rs:441-487`, issue `#184`). Disabling it is defensible
*because* choices 1+2 already own wall clearance twice over (the prepared
boundary is already the agent-radius clearance boundary per `#153`/`#171`,
and physics resolves real contact every tick) — so the border-ORCA layer was
provably redundant *and* actively wrong on multi-level geometry, not a
capability being traded away. The horizon is set to `1e-4` rather than `0.0`
purely to avoid `dodgy_2d` dividing by zero — a correct, well-commented
numerical-safety detail, not a hack.

**5. Sub-triangle conformal clipping.** The clip method itself
(`src/vsa/prepare/nav_clip.rs:1-45`) is a reasonable, conservative choice:
conformal (no T-junctions, so shared-vertex adjacency survives), emits both
sides of every cut so a stranding connectivity guard can un-drop a piece
without ever breaking conformity, and protects seam/door edges by
construction. The *decision to clip* is proportionate for interior cells
today (measured: FranklinMetro02 prepares in ~70 s, agent bridge ready ~4 s
after launch, per `docs/plans/M4_WAVE11_MANUAL.md` items 9-11). What Recast-
based engines do instead is exactly what this project chose not to redo here
— rebuild the whole tile from raw geometry at the target resolution, so there
is no "authored polygon vs. sub-triangle remainder" split to reconcile at all.
Given choice 1 (keep authored topology), some form of sub-triangle refinement
against collision is required regardless of algorithm; the specific artifact-
size cost of *this* implementation is filed correctly as `#179` and discussed
under Open Risks below rather than dismissed.

**6/7. Cross-mesh portals and off-mesh/travel doors.** The cross-mesh portal
work (`#154`) does the right thing given the fopdoc/real-data situation: FO3
interior `NAVM` seams carry zero `NVTR` "external" evidence on either test
cell (`docs/plans/M4_WAVE9_PLAN.md` amendment A5: "real-data NVTR evidence is
zero in interior cells… `candidates authored 0`"), so geometric derivation is
the only available signal, and it is not naive — reciprocal, non-overlapping
portal-interval matching with capsule-sweep validation at runtime-link-build
time (`docs/plans/M4_WAVE8_PLAN.md` amendments A2/A8) caught 10 of 11
FranklinMetro02 candidates that geometric-only validation had accepted into
literal void. Off-mesh/travel-door continuity (`#134`, `ledger_policy.rs`)
correctly separates "frozen position" vs. "door-marker spawn" resume kinds and
is unit-tested against both.

## What is questionable or wrong

**Finding: the door-topology gate was an allow-list keyed on
`PreparedSemantic::Door`, not a general "anything that blocks the agent
capsule must be route topology" invariant — severity high (now mitigated,
but the pattern is structural).**
Every nav door mechanism — crossing gates, lock costs, mid-route gating — was
gated exclusively on `PreparedSemantic::Door(_)`
(`src/viewer/nav/mod.rs:151`, `src/viewer/nav/agent.rs:1018`,
`src/viewer/nav/agent.rs:3786`, per issue `#177`'s own investigation). A
closed, solid `Activator`/`Kinematic` placement — the real Vault 101
`VaultGearDoor` — was therefore architecturally invisible to the router: it
planned straight through and the agent wedged in physics, misdiagnosed across
three separate builds (`#148`, `#172`) as a stair/KCC defect before `#177`
found the real cause. This is exactly the class of bug general navmesh
practice guards against by deriving walkability from collision *first* and
treating semantic tags as an overlay, not the reverse. `#177`'s fix
(`src/vsa/prepare/nav_doors.rs`) is the right shape — derive blocker→polygon
associations from *any* blocking placement's collision footprint, not from a
semantic enum — but it took four waves to arrive at, and nothing in the
current design states the general invariant ("every state-dependent blocking
collider must be represented in route topology, discovered from collision,
not enumerated by type") as a standing rule the way, say, `AGENTS.md`'s
prepared-revision-bump rule is. The correct design is to write that invariant
down and gate future blocker types (rubble, script-toggled walls, anything
`#115`'s AI packages might introduce) against it before they ship, rather than
relying on the next wedge point to surface the gap again.

**Finding: `navgraph.ron` grows ~45× (up to 27 MB per interior cell) from the
sub-triangle clip's both-sides-of-every-cut emission — severity medium,
already tracked and honestly measured, but unresolved at review time.**
Measured directly on `#179`: FranklinMetro02 goes from 1,338 to 58,376
polygons (27 MB, ~70 s prepare); Vault 101 Entrance from 293 to 12,030 (5.5
MB, ~24 s). This is proportionate for two interior cells but the issue itself
states the risk plainly: "M6 exterior cells and world-scale streaming are far
larger than either test cell. At this per-cell cost the prepared nav
artifacts become a streaming and disk problem well before exteriors are
attempted." The cheapest fix (RON pretty-printing overhead) is unexplored;
the correct fix (drop unwalkable polygons and compact/remap indices) is
explicitly deferred because it touches the same door/merge/cover index
plumbing `#177`'s derived-door associations depend on, and landing it
alongside a correctness fix was "explicitly judged a bad trade during wave
11." Reasonable sequencing, but it means M6 will need to open this file
again, not just re-run prepare on bigger cells.

**Finding: cross-mesh portal derivation for M6 exteriors has no validated
authored-evidence path yet — severity medium, open architectural risk, not a
current bug.** `#156`'s own measurement is that interior NAVM carries *zero*
`NVTR`-external evidence, so `#154`'s reciprocal geometric matching plus
runtime capsule-sweep validation is the only mechanism proven on real data.
Exterior worldspace grids are exactly where Bethesda's tools are expected to
populate `NVEX` (external navmesh connections) meaningfully, and this
codebase's own module doc for `NVEX` notes the leading 4 bytes are still an
undocumented "Unknown" field never cross-checked
(`src/vsa/openmw_esm4/navmesh.rs:336-339`). The per-candidate runtime KCC
sweep validation that made `#154` correct on two hand-picked interior cells
(11 candidates on FranklinMetro02) has not been load-tested at exterior-grid
candidate counts, and the physics-sweep validation step is inherently
per-candidate work done at runtime link-build time, not at prepare time —
this is a real scaling question for M6, not resolved by anything shipped so
far.

**Finding: `NAVI`'s `NVCI` correlation decode is explicitly unverified
against real bytes and is architecturally quarantined from ever driving
pathing — severity low, honest but worth naming as a fidelity gap.** The
module's own doc comment is unusually candid: "This layout has **not** been
cross-checked against real Fallout3.esm/FalloutNV.esm bytes in this
repository… fopdoc labels every other field 'Unknown' and OpenMW's
`loadnavi.cpp` skips it entirely for FO3/FNV, so there is no second source to
verify against… it is correlation evidence only, never consumed for runtime
pathing" (`src/vsa/openmw_esm4/navmesh.rs:21-34`). This is the right call
given the uncertainty, but it means the one FO3 data source that could in
principle correlate doors to *specific* navmesh pairs (potentially useful for
`#185`'s key-aware and `#115`'s AI-package door reasoning) is currently
decorative. Worth re-deriving from real bytes before `#115`/`#185` lean on it,
rather than assuming the fopdoc layout is correct because it parses without
crashing.

**Finding: water polygons are excluded (dropped) rather than typed —
severity low, correctly scoped for M4, flagged as a gap by the code's own
doc comment.** `src/viewer/nav/landmass_graph.rs:294-301`: "`is_water`
polygons are dropped rather than [typed]… a future wave could give water a
type index (rather than exclusion)". FO3 has swimmable water and some actors
(mirelurks, etc.) that path over/through it; total exclusion is correct for
M4's land-actor scope but is a real fidelity loss that should not be
forgotten when aquatic actors are addressed.

## Fidelity gaps vs FO3/OpenMW

- **Consumed correctly today:** preferred-pathing (`NVTR` bit `0x40`) is
  wired to a landmass type-index base cost as of `#168`
  (`PREFERRED_PATHING_TYPE_INDEX_COST`, closed); door associations (`NVDP`)
  drive the two-sided/single-sided door-link classification; per-edge
  external flags feed the authored/geometric merge-candidate split (`#156`).
- **Decoded but not consumed for pathing:** `NVCA` cover-triangle candidates
  (parsed, range-checked, carried into `PreparedNavMesh`, never read by
  `landmass_graph.rs` — appropriate, since combat/cover AI is `#116`/M5+
  scope, but worth tracking as a known drop rather than an oversight).
  `NVCI` door/navmesh correlation, per the finding above.
- **Dropped by design:** water polygons (excluded, not typed); the `NVGD`
  per-cell triangle grid's trailing element block (kept only as opaque raw
  payload — not required by the runtime graph, reasonable).
- **Not modeled at all yet, both tracked on the open `#185`:** trap-aware
  door refusal (OpenMW refuses to open a trapped door; this project has no
  trap model and would currently treat a trapped door as an ordinary one) and
  key-based per-actor lock bypass (OpenMW's `invStore.search(keyId)`; this
  project's `LOCKED_DOOR_TYPE_INDEX_COST` is currently door-only, not
  door-plus-key-holder).
- **PACK / AI packages (`#115`) are entirely unaddressed** by the nav layer
  reviewed here — the door-as-route-topology machinery is the right
  foundation (per-agent cost overrides already exist), but package-level
  rules like OpenMW's "Wander packages never open doors" are only a written
  note on `#185`, not implemented.
- **Meta-point on the comparison itself:** OpenMW is not a working reference
  for the FO3/FNV `NAVM`/`NAVI` *decode* — its `loadnavm.cpp` only implements
  the newer Skyrim/FO4 `NVNM` combined chunk and explicitly skips every FO3
  subrecord (`NVER`/`DATA`/`NVVX`/`NVTR`/`NVCA`/`NVDP`/`NVGD`/`NVEX`), a fact
  independently confirmed by fetching that file directly from OpenMW's
  `master` branch. This project's own module doc says the same thing
  (`src/vsa/openmw_esm4/navmesh.rs:6-13`, referencing a `NOTICE.md`
  divergence). So "compare against OpenMW's ESM4 handling" can only mean the
  AI-package/movement-model layer (which this review did, extensively, via
  `#185`), never the navmesh format layer — a distinction the review brief
  blurs and future waves should keep explicit.

## Open architectural risks for M6 exteriors and #115 AI packages

1. **Artifact size (`#179`).** 45× polygon growth per interior cell is not
   something exterior-cell prepare has been measured against yet; the fix
   most likely to matter (index-compacted, unwalkable-polygon-dropped output)
   is deliberately deferred and shares index plumbing with `#177`'s derived
   doors — landing both together at M6 kickoff is lower-risk than landing
   either alone late.
2. **Cross-mesh portal validation cost and evidence gap at exterior scale.**
   Per-candidate KCC-sweep validation was proven at ~11 candidates on one
   interior cell; exterior world-grid candidate counts are unknown and
   authored `NVEX`/`NVTR` evidence — absent in both interior test cells — is
   exactly where exteriors are expected to differ.
3. **One-archipelago-per-active-cell is a stated spike-era assumption, not a
   world-streaming design.** `src/viewer/nav/mod.rs:1-9`: "one archipelago
   per active cell… this wave stays a spike." The fall-guard kill-plane is
   derived from "the active cell's" bounds minimum
   (`src/viewer/nav/fall_guard.rs:1-24`) — an exterior worldspace with many
   simultaneously-loaded or streamed cells has no single "active cell bounds"
   the way an interior does, and this has not been revisited since the spike
   comment was written.
4. **Per-agent full swept-capsule KCC resolution every tick, with only a
   manual solve-rate knob and no adaptive crowd throttling** (`#114`'s
   deferred adaptive-throttle scope, still open per the epic's "Performance
   constraints" section). OpenMW hit the same wall from the opposite
   direction and gave up on it for now — Detour crowd integration was
   implemented and then disabled because "actors as obstacles… implemented
   with some issues… disabled now" (per
   [PR #1633](https://github.com/OpenMW/openmw/pull/1633)'s discussion).
   `#115` populating exterior scenes with many simultaneously AI-driven
   actors will hit this before it hits any navmesh-correctness issue; it is
   a known-hard problem industry-wide, not a solved one here.
5. **Door/blocker generalization is now geometry-first (`#177`) which is the
   right foundation for `#115`,** but package-level behavioral rules
   (Wander-doesn't-open-doors, key possession, trap refusal) are specified on
   paper (`#185`) and not implemented. `#115` should not assume the routing
   layer is behaviorally complete just because it is structurally correct.

## Sources

**Repository (this tree, branch `m4-wave11-177-doors`):**
- `src/vsa/openmw_esm4/navmesh.rs` (module doc comment, lines 1-34; `NVEX`
  decode, lines 327-358)
- `src/vsa/prepare/nav_graph.rs` (revision history, lines 26-47)
- `src/vsa/prepare/nav_clearance.rs` (module doc, lines 1-45)
- `src/vsa/prepare/nav_clip.rs` (module doc, lines 1-45)
- `src/vsa/prepare/nav_doors.rs` (module doc, lines 1-45)
- `src/vsa/prepare/navmesh.rs` (staging/wiring doc, lines 1-6)
- `src/viewer/nav/mod.rs` (spike-scope doc, lines 1-9)
- `src/viewer/nav/agent.rs` (module doc lines 1-235; door lock cost
  `LOCKED_DOOR_TYPE_INDEX_COST` lines 277-312; border-avoidance disable
  rationale and `NAV_BORDER_AVOIDANCE_TIME_HORIZON`, lines 441-501)
- `src/viewer/nav/door_link.rs` (FSM doc, lines 1-25)
- `src/viewer/nav/landmass_graph.rs` (module doc lines 1-33; water exclusion,
  lines 294-301; merge-portal interval matching, lines 285-322)
- `src/viewer/nav/movement_policy.rs`, `repath.rs`, `ledger_policy.rs`,
  `fall_guard.rs` (module docs)
- `docs/plans/M4_WAVE6_PLAN.md`, `M4_WAVE8_PLAN.md`, `M4_WAVE9_PLAN.md`,
  `M4_WAVE10_PLAN.md`, `M4_WAVE11_PLAN.md` — "Shipped amendments" sections
- `docs/plans/M4_WAVE11_MANUAL.md` — items 9-11 (real-data measurements)
- GitHub issues (repo `kelo221/bevyout`): `#9` (epic/checklist), `#111`,
  `#112`, `#113`, `#114`, `#134`, `#136`, `#137`, `#148`, `#153`, `#154`,
  `#155`, `#156`, `#157`, `#162`, `#163`, `#164`, `#165`, `#171`, `#177`,
  `#179`, `#184`, `#185`

**External:**
- [OpenMW `components/esm4/loadnavm.cpp`](https://raw.githubusercontent.com/OpenMW/openmw/master/components/esm4/loadnavm.cpp) — confirms FO3/FNV per-field `NAVM` subrecords are parsed for the newer `NVNM` chunk only and explicitly skipped otherwise
- [OpenMW PR #1633 — switch to recastnavigation](https://github.com/OpenMW/openmw/pull/1633) — runtime Recast/Detour navmesh build from collision, tiled, Detour-crowd actor-avoidance implemented then disabled
- [OpenMW `apps/openmw/mwmechanics/aipackage.cpp`](https://raw.githubusercontent.com/OpenMW/openmw/master/apps/openmw/mwmechanics/aipackage.cpp) — `openDoors()`/`isDoorOnTheWay()`/`hasCollisionWithDoor()`, geometric runtime door detection
- [OpenMW issue #1113 — Improve NPC door AI](https://gitlab.com/OpenMW/openmw/-/issues/1113)
- [OpenMW issue #6138 — navmesh/physics collision-shape mismatch](https://gitlab.com/OpenMW/openmw/-/issues/6138)
- [fopdoc — Fallout3 `NAVM` record](https://tes5edit.github.io/fopdoc/Fallout3/Records/NAVM.html)
- [GECK Wiki — NavMesh Generation](https://geckwiki.com/index.php/NavMesh_Generation)
- [Recast/Detour dynamic obstacles discussion](https://github.com/recastnavigation/recastnavigation/issues/457) and general `dtTileCache` temporary-obstacle practice
- [`landmass` crate docs (0.9.2)](https://docs.rs/landmass/0.9.2/landmass/) — A*, SSFA string-pulling, `AnimationLink`s, `Archipelago`/`Island`, type-index costs
- A* Pathfinding Project `RVONavmesh` / RVO2 polygon-obstacle support — general precedent for navmesh-border-as-ORCA-obstacle (via web search; no single canonical URL)
- bevyout issue `#185` body — orchestrator's own direct citation and comparison table against `AiPackage::openDoors()`/`AiAvoidDoor`, independently corroborated by this analyst's own fetch of the same OpenMW source
