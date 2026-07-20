# Nav Postmortem — Code Smells (analyst 3 of 4)

Scope: `src/viewer/nav/*.rs`, `src/viewer/nav_overlay.rs`, `src/vsa/prepare/{nav_graph,nav_clearance,nav_clip,nav_doors,navmesh}.rs`,
`src/vsa/openmw_esm4/navmesh.rs`. Read-only; no code changed. Axis is code-as-code — hacks, inefficiency, dead code,
complexity debt — not design intent (analyst 2) or tests (analyst 4).

## Verdict

This is unusually disciplined code for its size. There are **zero** `ponytail:`/`TODO`/`FIXME`/`HACK`/`XXX`/`todo!()`/
`unimplemented!()` markers anywhere in the 21k-line surface, every non-obvious constant carries a derivation or an
explicit "why this number" comment, the one retired subsystem (runtime navmesh erosion) was actually deleted rather
than left as dead code, and `unwrap()`/`expect()` on the runtime path is rare and locally justified. The real debt is
not hidden shortcuts — it's the debt of *scale and iteration*: one 9,386-line file that grew by accretion across eight
waves, three-plus independently hand-rolled point-in-triangle/point-in-polygon primitives with inconsistent epsilon
conventions, a hot artifact (`navgraph.ron`) serialized in the most expensive format available for the job, and a
couple of genuinely quadratic-shaped loops that are cheap today only because the inputs they run on happen to be
small. None of this is on fire. All of it is exactly the kind of thing that bites six months from now when someone
prepares a cell with 10 NAVM meshes instead of 2, or adds a fourth point-in-triangle test that disagrees with the
other three at a seam.

## Top findings, ranked

**1. `agent.rs` is 9,386 lines because it is doing five or six jobs, not because any one job is huge.**
`src/viewer/nav/agent.rs:1-9386`. Production code is actually ~4,241 lines (`mod tests` starts at line 4242 and runs
to EOF — 5,145 lines, 82 `#[test]`s, all in one flat, unsegmented module). That's still the single largest file in
the nav surface by 2x, and it mixes: console command parsing/dispatch (`tna_command` and ~10 subcommand functions),
door lock/lifecycle state machinery (`door_open_and_locked`, `door_usable_now`, `apply_door_lock_overrides`,
`drive_door_link_for_agent` at 3256-3695, ~439 lines, `door_availability_system`), merge-portal traversal
(`spawn_link_pair`, `merge_traversal_system`, `resume_pending_merge_repath_system`), KCC/physics stepping
(`step_agent_kcc`, `world_contact_report`, `apply_agent_physics_movement` at 2633-2862, ~229 lines), cross-cell
ledger restore (`ledger_departing_agent`, `restore_ledgered_agent`), and archipelago lifecycle
(`ensure_archipelago`, `teardown_archipelago`). The repo has already solved this exact problem twice —
`viewer::interaction` split into `activation`/`door`/`focus`/`items`/`presentation`/`scripted`/`state`/`ui`, and
`viewer::console` split into `*_commands` provider modules (AGENTS.md) — so there's a proven pattern to apply here
that nobody has applied to `nav::agent` yet. See "Structural recommendations" below for the split. Impact: high
(this is the file every nav change touches, and every touch risks an unrelated merge conflict); likelihood: certain,
it will keep growing under the current shape. Smallest fix: split along the six job boundaries listed; can be done
incrementally, one extraction per PR, keeping `agent.rs` as the Bevy `Plugin`/system-registration root.

**2. `navgraph.ron` is pretty-printed RON, parsed as a full-text read+parse on every consumer.**
`src/vsa/prepare/nav_graph.rs:1510` writes with `ron::ser::to_string_pretty(graph, PrettyConfig::default())` — the
most verbose serialization this crate offers (indentation, struct-name headers, one array element per line) for an
artifact whose only consumers are `ron::de::from_str` in `src/viewer/nav/mod.rs:40-44` (runtime archipelago build,
every cell activation) and `src/viewer/nav_overlay.rs:278-282` (`tnm` toggle). Nothing ever hand-edits this file. For
a cell that clips 1,338 authored NAVM triangles up to ~58k output polygons (M4 wave 11's sub-triangle clipping), that
means: prepare pays pretty-print serialization cost for a 27MB text blob, every viewer launch on that cell pays
`read_to_string` + full RON parse of that same 27MB, and `write_nav_graph` additionally reads the *existing* 27MB
file back into memory just to byte-compare it for the `reused` flag (`nav_graph.rs:1513-1515`). None of this is
wrong, but it's the most expensive point on the write→read→re-read path for a machine-only artifact — compact RON
(`PrettyConfig` with `indentor`/newlines stripped, or simply skipping `to_string_pretty` for `to_string`) or a
length-prefixed binary encoding (bincode/postcard) would cut both the write time and every subsequent read time
substantially, and the `reused` check could hash-compare instead of byte-compare without reading the old file whole.
Impact: high on the ~70s prepare/every-launch cost the wave plans call out; likelihood: this fires on every prepare
and every cell load, not an edge case. Smallest fix: switch `write_nav_graph` to `ron::ser::to_string` (compact) as
a first, near-zero-risk cut; a binary format is a bigger but higher-payoff follow-up.

**3. Three-plus independently hand-rolled point-in-triangle/point-in-polygon tests, disagreeing on epsilon.**
- `src/viewer/nav/landmass_graph.rs:838-848` `point_in_triangle_xz`: cross-product sign test, **zero** epsilon
  tolerance (`d1 < 0.0`/`d1 > 0.0` exact comparisons).
- `src/vsa/prepare/nav_clearance.rs:284-304` `barycentric_xz`: barycentric weights, degeneracy epsilon `1.0e-9`
  (`nav_clearance.rs:290`), containment epsilon `1.0e-4` (`nav_clearance.rs:298`).
- `src/vsa/prepare/nav_doors.rs:241-248` `point_in_convex_polygon` (+`cross`, line 195): cross-product half-plane
  test for a convex polygon (not a triangle), containment epsilon `1.0e-4` (`CONTAINMENT_EPSILON`, line 61).
- `src/vsa/prepare/navmesh.rs:693-701` inline barycentric test inside `report_nav_probe`'s `covering` closure:
  same math as `nav_clearance.rs`'s `barycentric_xz` but reimplemented locally, degeneracy epsilon `1.0e-9`
  (line 701), containment epsilon `1.0e-4` (line 703) — a fourth copy of the same 20 lines of math.
Plus three separate point-to-segment/point-to-triangle distance helpers: `nav_graph.rs:811` `point_segment_distance_sq`
(full 3D), `nav_clearance.rs:307` `point_segment_dist_sq_xz` (XZ), `landmass_graph.rs:817` `distance_to_segment_xz`
(XZ, different clamp/epsilon shape). None of these are wrong in isolation — each is well-commented and locally
correct for its caller — but a boundary point that one module calls "inside" and another calls "outside" at exactly
the disagreement margin (zero tolerance vs. 1e-4) is exactly the kind of seam bug this codebase has already paid for
once (the wave 11 sliver-filter incident, see the ledger below). Impact: medium (latent, not currently observed
failing); likelihood: medium-high given the epsilon history this project already has. Smallest fix: promote
`barycentric_xz` + its two epsilons to a shared `nav::geometry` (or extend `nav_clip.rs`'s existing math helpers) and
have `report_nav_probe` call it instead of reimplementing it; leave `landmass_graph`'s sign-test alone only if its
zero-tolerance choice gets an explicit "why this one is exact" comment (currently unexplained why it alone needs no
epsilon).

**4. `compute_mesh_merges` recomputes `boundary_edges(mesh_a)` once per inner-loop iteration instead of once per outer.**
`src/vsa/prepare/nav_graph.rs:1040-1046`: the `for b_index in (a_index+1)..meshes.len()` loop recomputes
`edges_a = boundary_edges(mesh_a)` on every iteration even though `mesh_a` (and therefore `edges_a`) is fixed for the
whole inner loop — it should be hoisted to once per `a_index`, or better, `boundary_edges` computed once for every
mesh up front (`O(M)` instead of `O(M²)` calls). Separately, there is no mesh-level AABB pre-filter before the
`O(E_a × E_b)` edge-pair double loop (line 1058-1096) — every mesh pair pays the full boundary-edge cross product
even when their bounding boxes can't possibly be within `MESH_MERGE_DISTANCE` (2.0 m). **Scale caveat**: real cells
measured in the wave manuals have only 1-2 NAVM meshes per cell (`meshes 2, polygons 1338` for FranklinMetro02), so
today `M=2` makes this a non-issue in absolute terms — but the shape is genuinely `O(M²)` redundant work with no
bound on `M`, and the code has no guard if a future cell (or an exterior-worldspace cell with many more NAVM records)
has `M` in the dozens. Impact: low today, medium as a landmine; likelihood: only trips on unusual input. Smallest
fix: hoist `edges_a` out of the inner loop; add an AABB reject before the edge double loop.

**5. The same "recompute per-polygon floor inside a per-blocker loop" mistake appears twice in `nav_doors.rs`.**
`src/vsa/prepare/nav_doors.rs:159-165` (`derive_door_associations`) and `nav_doors.rs:281-286`
(`unreported_interior_polygons`) both compute `polygon.vertices.iter().fold(f32::INFINITY, ...)` (the polygon's own
floor height) **inside** the `for blocker in blockers { for mesh in meshes { for polygon in ... } }` triple loop —
but the fold result depends only on `polygon`, never on `blocker`. It is redundantly recomputed once per blocker
instead of once per polygon, in two near-identical functions that could also plausibly share one "candidate
polygons near this blocker's floor band" helper instead of duplicating the same triple-nested structure and
early-exit conditions verbatim. Impact: low in absolute terms (doors count is small per cell) but it's a clean,
mechanical, zero-risk fix. Smallest fix: precompute `Vec<f32>` of per-mesh per-polygon floors once, index into it
inside the blocker loop; factor the shared triple-loop skeleton into one helper both functions call with a
different per-polygon predicate.

**6. `collapse_ill_conditioned` rescans every triangle from scratch on every weld round, with no incremental tracking.**
`src/vsa/prepare/nav_clip.rs:453-573`: the outer `for _ in 0..vertices.len().max(1)` loop performs one weld per
round (correctly, per its own doc comment, "so the incidence used to prove a weld safe is never stale") but each
round's `'triangles: for triangle in triangles.iter()` candidate search, and each round's
`triangles.iter().any(...)` inversion check per candidate edge, walk the **entire** triangle list from index zero
again, even though only the neighborhood of the *previous* weld changed. Cost is `O(welds_performed × triangle_count)`
where the current codebase has driven `welds_performed` down close to zero for well-formed input (wave 11's A3
amendment fixed the root cause that used to produce far more welds), so this is not the dominant cost today — but
it is genuine unbounded-per-round recomputation with no memoization, and the loop bound (`vertices.len()` rounds) is
a correctness safety valve, not a performance one. Impact: low today given how few welds real cells need,
medium if a future cell's geometry needs many; likelihood: low-medium. Smallest fix: track a worklist of
triangles/edges touched by the last weld instead of a full rescan; not urgent given current weld counts.

**7. `nav/mod.rs::read_nav_graph` is an admitted, byte-identical duplicate of `nav_overlay::read_nav_graph`.**
`src/viewer/nav/mod.rs:37-44`: *"Duplicated from `nav_overlay::read_nav_graph` rather than shared: that function is
private to `nav_overlay.rs`, which this wave's file-ownership boundary does not include."* Compare
`nav_overlay.rs:278-282` — identical four-line body (`read_to_string` + `ron::de::from_str`, same two `.with_context`
messages). This is the cleanest smell in the whole surface: a wave's parallel-worktree file-ownership boundary
(AGENTS.md's own convention) produced a real, self-admitted duplication that nobody has since merged. Impact: very
low (four lines, no logic divergence yet); likelihood of drift: real — the two copies can silently diverge (e.g. one
gets an added file-size sanity check and the other doesn't) with nothing to catch it. Smallest fix: make one of them
`pub(crate)` in a shared spot (`nav::mod` is the natural owner since `nav_overlay` already imports from `nav`) and
delete the other.

**8. Six repeated `world.get_mut::<AgentRuntime>(agent_entity).unwrap()` calls whose safety is proven once, far away.**
`src/viewer/nav/agent.rs:3326,3461,3540,3611,3669,3676` (all inside `drive_door_link_for_agent`, 3256-3695). Every
one of these unwraps is safe *because* line 3257 already fetched `AgentRuntime` from the same entity and returned
early if absent, and nothing in between despawns the entity or removes the component (a direct `&mut World`
function, not a deferred-command system). That invariant is correct today but is re-proven by inspection, not by
the type system or a local assertion, six times over a 439-line function — a future refactor that inserts a
despawn or an early `return` between the top-of-function check and one of these unwraps turns a silent invariant
violation into a panic on the main thread mid-tick. Impact: medium (a panic here is a hard viewer crash, not a
graceful `ConsoleError`); likelihood: low today, rises with every future edit to this function. Smallest fix: fetch
`AgentRuntime` once via `world.entity_mut(agent_entity)` at the top and thread a live `&mut AgentRuntime` (or a
small local struct) through the rest of the function instead of six independent re-fetches — also removes the
repeated ECS lookup cost.

**9. Magic-number costs are directionally reasoned but numerically arbitrary.**
`CLOSED_DOOR_TYPE_INDEX_COST = 1000.0` (`agent.rs:361`) has a genuinely careful doc comment on *why* it must be
large-but-finite and *why* not `f32::MAX` (accumulated path costs must not overflow back to infinity) — but the
number `1000.0` itself has no derivation (not "typical route length × N" or similar); it's a round number picked to
be "big enough." Same shape for `MERGE_LINK_SWEEP_TOLERANCE = 0.6` (`agent.rs:418`, "deliberately looser than
[...], generous slack") and `MESH_MERGE_DISTANCE = 2.0`/`PORTAL_DIRECTION_COSINE_MAX = -0.5` (`nav_graph.rs:665,684`,
both anchored to one measured real-data gap — 0.09-0.9m at FranklinMetro02 — generalized by a safety margin, which
is actually a good example of a *properly* derived constant, worth contrasting). This is not a bug class, just a
consistency note: some constants in this file are empirically anchored to a measured real-data number
(`MESH_MERGE_DISTANCE`) and some are "sounds about right, and won't overflow" (`CLOSED_DOOR_TYPE_INDEX_COST`). Both
are documented; only the second kind would silently produce a wrong-feeling result (e.g., a door that's needlessly
avoided, or not avoided enough relative to a real detour) under different level geometry, with no test that would
catch a bad magnitude, only a bad *sign*. Impact: low-medium (wrong direction is tested; wrong magnitude is not);
likelihood: only shows up as a "feels off" gameplay report, hard to attribute. No action needed beyond awareness —
flagging for the next wave that touches door-cost tuning to add a magnitude-sensitivity test, not a fix now.

**10. Four `#[allow(clippy::too_many_arguments)]` escapes in the nav surface.**
`agent.rs:2450,2632,3027`, `nav_clip.rs:654`. Each suppresses a real clippy signal about parameter-list size
(`step_agent_kcc`, `apply_agent_physics_movement`, `merge_traversal_system`, and one `nav_clip.rs` builder). Not
inherently wrong — Bevy exclusive/world-access systems often legitimately need many resources — but it's worth
noting clippy's dead-code net (which AGENTS.md correctly says is already clean) does not catch parameter-list bloat,
and these four are exactly the functions independently flagged above as long/complex (#1, #8). Low priority on its
own; a byproduct of #1's split.

Everything else in the surface — `door_link.rs`, `movement_policy.rs`, `repath.rs`, `ledger_policy.rs`,
`fall_guard.rs`, `nav_clearance.rs`'s grid-based `CollisionIndex` broadphase, `nav_clip.rs`'s adaptive-refinement
clip pipeline — is clean: appropriately sized (109-1,988 lines), single-purpose, and the collision broadphase in
particular (`nav_clearance.rs:358-366`, uniform XZ grid, capped cell count) is the right data structure for its job,
not a naive scan. Say so plainly: this is not a codebase that needs a broad rewrite, it needs `agent.rs` split and
the RON format reconsidered.

## Deliberate-shortcut ledger

The project's `ponytail:` convention (used twice elsewhere in the repo — `src/viewer/scene.rs:535`,
`src/viewer/animation/policy.rs:63`) is **not used anywhere in the nav surface** — zero occurrences across all
scoped files. There is likewise no `TODO`/`FIXME`/`HACK`/`XXX`/`todo!()`/`unimplemented!()`/"for now"/"temporary" in
scope. The closest things to a tracked shortcut are:

| Marker | Location | Status |
|---|---|---|
| "interim runtime erosion pass" | `nav/mod.rs:72`, `landmass_graph.rs:322,455`, `movement_policy.rs:311,485`, `door_link.rs:525`, `agent.rs:1605,2790,3361` | **Stale prose, not a live shortcut.** The erosion pass itself was fully removed (M4 wave 10, issue #153) — clearance now runs prepare-side in `nav_clearance`/`nav_clip`. These are historical doc-comment references explaining *why* the current code looks the way it does (e.g. why `build_navigation_mesh` takes vertices "as-is"). Harmless, but worth trimming to reduce noise the next time any of these functions are touched — a reader has to know the history to know these comments describe something that no longer exists. |
| `_merges: &[MergeInput]` unused parameter | `landmass_graph.rs:352` (used by `build_navigation_mesh`) | **Still valid, but the "call-site compatibility" rationale is weak.** Doc comment (`landmass_graph.rs:323-325`) says it's kept so callers don't need updating, but every call site (10+, including 8 test call sites) already passes `&[]` explicitly — a mechanical `sed` across those call sites plus deleting the parameter is strictly less code than keeping the dead parameter. Low priority. |
| `nav_doors.rs:25` module doc: `f32::INFINITY` "until the blocker opens" (`MetroGateLoad`) | `nav_doors.rs` module doc | Still valid — describes the *locked*-door design (`LOCKED_DOOR_TYPE_INDEX_COST`), which the wave 11/#177 rework confirmed and shipped correctly. Not a shortcut, a correct description of current behavior. |
| `read_nav_graph` duplication, explicitly labeled as a file-ownership-boundary artifact | `nav/mod.rs:37-39` | **Now a minor bug-risk, not urgent.** See finding #7 above. |

No shortcut in this ledger has quietly turned into a live functional bug; the worst outcomes are documentation rot
and one small duplication.

## Dead and vestigial code

- **`_merges` parameter on `build_navigation_mesh`** (`landmass_graph.rs:350-355`) — genuinely unused inside the
  function body; see ledger above. The `MergeInput` *type* itself is not dead (it's consumed by a different function,
  `landmass_graph.rs:982`, for real cross-mesh link resolution) — only this one parameter is vestigial.
- **`#[allow(dead_code)]` items in `src/vsa/openmw_esm4/navmesh.rs`** (lines 60, 123, 479, 481, 483) — `NavMeshTriangle`'s
  documented `NVTR` flag-bit constants (`EDGE0_EXTERNAL` etc., re-derived locally per the module's own stated policy
  that `nav_graph` "must stay free of `openmw_esm4` imports"), `NavMeshRecord.grid` (retained metadata, explicitly
  documented as "not consumed by the runtime graph"), and `NaviRecord.form_id/flags/version` (retained for
  diagnostics/forward use, explicitly documented as such). These read as legitimate format-documentation /
  forward-compatibility retention, not rot — each has an inline comment explaining why it's kept unused. Lowest
  priority in this report; flagged only because the prompt asked to hunt the class clippy can't see.
- **No dead functions, types, or logically-unreachable branches were found** beyond the above. The retired erosion
  pass left doc-comment residue (see ledger) but no dead code — it was actually deleted when the wave 10 rework
  landed, which is the right outcome and worth calling out as a positive rather than a smell.

## Performance findings

1. **`navgraph.ron` pretty-printing** (`nav_graph.rs:1510`) is the single highest-leverage fix in this report: same
   data, `to_string_pretty` → `to_string` (or a binary codec) directly cuts write bytes, write time, and — more
   importantly, since it happens on every viewer launch, not just `prepare` — read/parse time on the hot runtime
   path (`nav/mod.rs:40-44`). See finding #2.
2. **`compute_mesh_merges`'s `O(M²)` mesh-pair loop and un-hoisted `boundary_edges(mesh_a)` recompute** (finding #4)
   is currently cheap in absolute terms because measured real cells have `M=1-2` NAVM meshes, but the shape is
   unbounded and worth fixing preemptively rather than waiting for a cell that makes it visible in a profile.
3. **Redundant per-polygon floor computation inside `nav_doors.rs`'s blocker loops** (finding #5) is a clean,
   zero-risk hoist; low absolute cost today (few doors per cell) but mechanical enough to fix in the same pass as #4.
4. **`collapse_ill_conditioned`'s full-list rescan per weld round** (finding #6) is bounded by the (now small)
   number of welds real data needs post-wave-11, so it is *not* the dominant contributor to the measured ~70s
   prepare time on an interior cell — the dominant cost is much more likely the adaptive-refinement clip pass itself
   (`nav_clip.rs`'s `refine_and_clip`, `ClipParams::default()`: `resolution=0.35`, `max_refinement_rounds=4`,
   `bisection_steps=10`), which is deliberately expensive by design (each boundary crossing costs up to 10 predicate
   evaluations via bisection, each predicate evaluation queries the collision broadphase) to fix real defect classes
   wave 11 measured and fixed (98% connectivity, was previously wrong). That cost is a considered tradeoff, not a
   smell — flagged here only so it isn't confused with the genuinely-fixable items above it.
5. **The collision broadphase (`nav_clearance.rs`'s `CollisionIndex`, a uniform XZ grid capped at
   `MAX_GRID_CELLS_PER_AXIS`) is the right structure and not a hot spot** — worth stating explicitly since it's the
   part of this pipeline most likely to be *suspected* of being a naive `O(n²)` scan and isn't.
6. **`nav_overlay.rs`'s `read_nav_graph` → full `ron::de::from_str` on every `tnm` toggle** inherits cost #1's fix
   for free once that's done; no separate action needed here.

## Structural recommendations for `agent.rs`, in priority order

Given the repo's own precedent (`viewer::interaction`'s `activation`/`door`/`focus`/`items`/`presentation`/
`scripted`/`state`/`ui` split; `viewer::console`'s `*_commands` trait-provider split), the natural extraction order
for `src/viewer/nav/agent.rs` (keeping the module root as the `Plugin`/system-registration/archipelago-lifecycle
owner) is:

1. **`nav::agent::console`** — `tna_command`, `usage_reply`, `solve_rate_command`, `parse_agent_index`,
   `spawn_agent`/`spawn_test_agent`, `travel_agent`, `parse_form_id`, `goto_agent`/`goto_player_target`/
   `parse_goto_point`, `agent_status`/`resolve_status`/`hud_agent_status_lines`/`active_link_description`/
   `describe_target`, `despawn_agent`. This is the single biggest, cleanest extraction — it's already a coherent
   "console command family" per the module's own doc comment, and the codebase has a named pattern
   (`ConsoleCommandProvider`) for exactly this.
2. **`nav::agent::door`** — `door_open_and_locked`, `door_usable_now`, `apply_door_lock_overrides`,
   `set_door_lock_level`, `request_door_open`, `door_link_system`, `drive_door_link_for_agent` (the 439-line
   function — the single largest extraction target), `door_availability_system`, `door_position_in_active_cell`.
3. **`nav::agent::merge`** — `spawn_link_pair`, `MergeLinkRejection`, `validate_merge_link_collision`,
   `permitted_animation_links_for`, `clear_merge_link_quarantine`, `merge_traversal_system`,
   `resume_pending_merge_repath_system`, `merge_traversal_timeout`.
4. **`nav::agent::kcc`** — `step_agent_kcc`, `world_contact_report`, `apply_agent_physics_movement`,
   `update_agent_desired_velocity_blend`, the `AgentKcc`/`AgentDesiredVelocityBlend` types.
5. **`nav::agent::ledger`** — `ledger_departing_agent`/`ledger_departing_one_agent`,
   `restore_ledgered_agents_system`/`restore_ledgered_agent` — a thin Bevy adapter over the already-pure
   `ledger_policy` module, consistent with the rest of the codebase's pure-core/thin-adapter split.
6. Module root keeps: `NavBackendPlugin`/`install`, `ensure_archipelago`/`teardown_archipelago`/
   `despawn_stale_navmesh_archipelago`, `apply_preferred_pathing_base_cost`, `spawn_player_nav_character`/
   `sync_player_nav_character`, the type-index cost constants (`LOCKED_DOOR_TYPE_INDEX_COST` etc. — these are
   genuinely cross-cutting and belong at the root), and `log_agent_state_changes`/`log_path_latency`.

One flagged-but-out-of-axis observation for whoever owns the split: the 5,145-line `mod tests` block (82 tests, no
sub-modules) should be split *alongside* its production code, not left as one flat block importing from six new
submodules via `use super::*` — but the test organization itself is analyst 4's territory, not this report's.
