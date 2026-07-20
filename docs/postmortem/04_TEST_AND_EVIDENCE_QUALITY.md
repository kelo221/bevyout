# M4 navmesh post-mortem — axis 4: do the tests and diagnostics actually catch what they claim to?

Scope: `m4-wave11-177-doors` branch (base `175eae4`, tip `461d190`). All
line numbers below are against that tree.

## Verdict

The nav test suite is, on balance, unusually self-aware for a project this
size — several feature files carry inline commentary admitting a known
limitation instead of hiding it (`features/nav_stuck_progress.feature`), and
the wave-11/#184 fix is the one place in the codebase with an explicit
mutation-check discipline (`an_agent_crosses_finely_triangulated_ground_...`
proves its own fixture reproduces the bug under stock parameters before
proving the fix clears it). But the project's real failures were never
"missing a test" in the abstract — they were **tests and diagnostics that
measured the wrong thing while reading green**, and that specific failure
mode is still structurally possible in at least two load-bearing places
today. The single most consequential fix of the entire arc — `prepare`
failing the build when landmass's own validator would reject a mesh
(`verify_landmass_acceptance`, `src/vsa/prepare/navmesh.rs:567`) — has **zero
unit-test coverage of its own failure branch**; the only thing that has ever
proven it fires is one real Vault-101 prepare run. A second, quieter
instance of the same shape survives in `src/vsa/prepare/nav_doors.rs`, where
the function that is supposed to *catch* under-reported closed-door
interiors reuses the exact same containment primitive as the function whose
output it is checking, so a bug in that shared primitive is invisible to the
"invariant" test built on top of it. Both are the 98%-share-zero-navigation
failure shape in miniature: a check that agrees with the code it is
checking. Elsewhere the suite is genuinely strong — the door-gate,
stuck-progress, and fall-guard pure-policy tests pin real boundary
invariants with real negative cases, and the env-gated `wedge_replay`/
`stall_replay` harnesses are honestly reusable (four different issues —
#148, #172, #177, #184 — were diagnosed through the same generic
scene-replay code, not bespoke throwaway scripts). The pattern across the
whole wave-6→11 arc is consistent: **the cucumber/unit suite stayed green at
every step; real-data acceptance is what actually found every one of the
five defects documented in the plans' "Shipped amendments" sections.** That
is not a suite that is failing at its job in general — the pure-policy
modules genuinely earn their keep — but it does mean the project's own
history should be read as: trust the unit suite for regression protection
on logic already proven correct once, never as proof of correctness on new
geometry/runtime-integration work.

## Tests that would not catch their own defect

| Test / mechanism | File:line | Why it can pass with the target logic broken |
|---|---|---|
| `verify_landmass_acceptance` (the hard-failure gate that is the *only* fix for the "98% share, zero navigation" class) | `src/vsa/prepare/navmesh.rs:567`, called at `:1016` | No unit test constructs a `PreparedNavMesh` designed to fail landmass's `NavigationMesh3d::validate()` and asserts `apply_nav_clearance` returns `Err`. `navmesh.rs`'s only three `#[test]`s (`:1398,1421,1432`) are all `blocker_footprint` geometry, unrelated to this gate. The only thing that has ever exercised the bail path is real Vault-101 data during wave 11 (`docs/plans/M4_WAVE11_MANUAL.md` step 4). A regression that silently no-ops this check, swaps `Err` for `Ok`, or stops calling it at all would compile, pass `cargo test`, and only be caught by a human running `prepare` + `tna spawn` on a real cell. |
| `no_walkable_polygon_is_left_unreported_inside_a_blocker` | `src/vsa/prepare/nav_doors.rs:463-479` | This test's stated job (module doc at `:255`, "the invariant this issue is measured by") is proving no closed-door interior escapes detection. But `derive_door_associations` (`:149-188`, `contained` at `:180`) and `unreported_interior_polygons` (`:258-303`, `inside` at `:287`) both classify "wholly inside" with the identical call: `point_in_convex_polygon`. A bug in that one shared function (wrong sign convention, an off-by-epsilon at a footprint edge, wrong winding assumption) is applied identically on both sides of the check and cancels out — the test would still report zero unreported polygons. This is structurally the same shape as the 98%-share defect the module's own doc comment invokes by name: a verification pass built from the same code as the thing it verifies. |
| `landmass agent states map to nav agent status` (cucumber outline) / `map_agent_state` unit assertions | `features/nav_backend.feature:76-87`; `src/viewer/nav/landmass_graph.rs:1631-1649` (function at `:1071-1081`) | A straight enumeration of the `match` arms restated as assertions. It would catch a swapped RHS (e.g. `Paused` accidentally mapped to `Idle`) but the match is exhaustive over `landmass`'s own enum, so deleting or misordering an arm is a compile error the test never gets a chance to catch — the only bug class it guards is a *wrong but well-typed* mapping. Low but non-zero value; listed because it reads as much more thorough coverage than it is (12 assertions for one 9-arm match). |
| `the_pass_is_deterministic_across_calls` and the cucumber "building the nav graph again yields identical cross-mesh merges" / "both door-link descriptor extractions are identical" scenarios | `src/vsa/prepare/nav_clearance.rs:1972-1978`; `features/nav_adapter.feature:44`, `:130` | These prove repeatability, not correctness — a deterministically-wrong pass (e.g. a stable but incorrect tie-break) passes every one of them. Fine as a *supplement* to a correctness assertion (which each of these files also has elsewhere), but if read in isolation they look like coverage of the underlying algorithm and are not. |

## Coverage gaps by failure class

| Failure class (project actually hit this) | Guarding test today | Notes |
|---|---|---|
| Concave/invalid polygon rejected wholesale by landmass at runtime (98%-share-zero-nav) | `reject_invalid_geometry` unit tests (`nav_clearance.rs:1935-1966`) cover collinear + wrong-winding cases; **the authoritative gate `verify_landmass_acceptance` has no direct test** (see table above) | Partial. The approximate heuristic is tested; the real validator call that is the actual fix is not. |
| Mesh with no surviving islands / all-water | `features/nav_backend.feature` "An all-water mesh produces no navigation mesh" | Covered, and correctly distinguished from an error (empty mesh is documented as the runtime's own valid case). |
| Door interiors walkable while closed (`MetroGateLoad`, `VaultGearDoor`) | `features/nav_derived_doors.feature` (new, wave 11) + `nav_doors.rs` unit tests + `nav/agent.rs` `AgentTypeIndexCostOverrides`/door-availability `#[cfg(test)]`s | Covered for the mechanism, but see the shared-primitive caveat above; also **no synthetic real-shape regression fixture** exists for either of the two concrete cells that exposed this (both were found live, not by a test predicting them). |
| Closed/locked door routing (impassable vs. expensive-but-plannable, lock excludes the solver) | `features/nav_door_gate.feature`, `features/nav_adapter.feature` "Blocked-door exclusion", `nav/agent.rs` cost-override tests | Covered, including the wave-11 correction (closed-but-openable must be *expensive*, not infinite, or the agent can never reach the door to open it — `16c9a95`). |
| Agents falling out of the world (#164) | `features/nav_fall_guard.feature` (5 scenarios, real boundary cases: resting exactly at kill plane, unbounded descent) | Well covered — genuine boundary-value tests, not just happy path. |
| Portal/merge-link blocking (#162 quarantine) | `features/nav_portal_quarantine.feature` (pure kind-assignment/allow-list logic) | The pure half is covered; wave-10's own "Shipped amendments" A6 states the *live forced-block* real-data scenario was **not exercisable** post-#153 (no blocked portal existed on either test cell) — so the runtime wiring (`merge_traversal_system`'s timeout branch) has never been proven against real data, only against a live-`Archipelago3d` `#[cfg(test)]` harness. |
| Stuck-detection false positives (route progress vs. final-target distance) | `features/nav_stuck_progress.feature` (8 scenarios) | The strongest file in the suite: includes a deliberately-documented known limitation (oscillating-but-achieved steering) instead of hiding it, and a repath-reset regression test. |
| Velocity-space (ORCA border) stalls with zero contact (#184) | `an_agent_crosses_finely_triangulated_ground_without_its_steering_collapsing` + `archipelago_options_clamp_border_avoidance_but_keep_agent_avoidance` (`nav/agent.rs:7344,7371`) | Well covered and mutation-checked (asserted as a before/after pair). This is the model other nav fixes should have followed from the start. |
| Revision/stale-cache handling for the nav graph specifically | `nav_graph.rs:1606` `revision_is_pinned` (string literal pin) + generic `features/fingerprints.feature` | **Weak.** The pinned-string test only fires if someone edits the revision constant; it cannot detect the actual failure mode AGENTS.md documents (a new `#[serde(default)]` field added to a prepared type *without* remembering to bump the constant). No structural/shape-hash test exists that would force that bump automatically — see "Missing instruments" below. |
| Stalled agent short of a door crossing never gating (#177 second finding) | `a_stalled_agent_short_of_a_closed_door_still_opens_it`, `a_stalled_agent_never_opens_a_door_behind_it` (`nav/agent.rs`, ~line 6045-6160) | Covered, including the negative case (a door behind/beside the agent must not fire) that guards against re-widening the trigger too far. |

## Metrics that can lie

- **`smallest largest-component share N%`** (`navmesh.rs:1111`). This is the
  metric that directly caused the worst documented incident: Vault 101
  reported 98% while the runtime rejected the mesh outright (wave 11, A3(iii)
  in `docs/plans/M4_WAVE11_PLAN.md`). It measures connectivity of the graph
  *this pass built*, never whether landmass will accept that graph. It is now
  honest only because `verify_landmass_acceptance` runs immediately after and
  hard-fails the build (see above) — the percentage itself is still exactly
  as capable of lying as it ever was; the safety net is a separate,
  under-tested check bolted on afterward, not a property of the metric.
- **`nav clearance: … invalid rejected N`**. Healthy reads as `0`. But
  `reject_invalid_geometry`'s notion of "invalid" (collinear-by-area-
  threshold, wrong winding vs. majority) is a hand-replicated approximation
  of landmass's own convexity rule, sized "deliberately larger than
  `nav_clip`'s collapse threshold" (`nav_clearance.rs:797`) by construction,
  not by proof of equivalence. `0` here does not imply landmass will accept
  the mesh; only `verify_landmass_acceptance` proves that, and that check has
  no unit coverage of its own (see table above). Two numbers that *look* like
  the same guarantee are not.
- **`package catalog: … 3021 unsupported subrecord`** — the literal example
  the brief cites — is fixed on this branch. `package_catalog.rs:29-35`
  documents the exact fix: splitting **deferred** (documented-but-not-yet-
  decoded, expected on every record) from **out-of-scope** (a FormID this
  pass's universe cannot resolve by design) from **unsupported/unresolved**
  (the only counts that should ever be non-zero and actionable). Current
  healthy output is `0 unsupported subrecord, 3021 deferred subrecord, 0
  unresolved location, 3 unresolved target` (`M4_WAVE11_MANUAL.md`). This is
  the one clean example in the codebase of a metric that used to lie by
  aggregation and now doesn't — worth citing as the pattern to replicate
  elsewhere (see `nav doors: unreported interior polygons` below, which has
  not yet had the same treatment applied but is structurally similar).
- **`nav doors: unreported interior polygons N`** (`navmesh.rs:1378`). Only a
  `"warning"` diagnostic, never a hard failure — unlike the sibling
  `verify_landmass_acceptance` gate, a non-zero count here does not fail
  `prepare`. Given this exact diagnostic exists *because* two closed-door
  interiors were shipped walkable for the entire project history without
  anyone noticing (#148/#177's root cause), leaving it as a warning rather
  than a build failure repeats the original mistake's shape at one remove:
  a real, computed, correct-when-it-fires signal that a human still has to
  remember to read.
- **`collision_blocked` (pre-#177) / now `reason=obstructed` vs.
  `reason=no_contact_no_progress`** (`agent.rs:2578-2585`). This is the
  fixed version of the exact metric the brief calls out. Worth noting for
  future work: `reason=` is derived from a **snapshot at the moment of
  logging** (current contact planes + one forward sweep), not from the
  history that produced the stall. It correctly separates "wedged against
  geometry" from "steering commanded no motion," but it still cannot
  distinguish *why* steering commanded no motion (ORCA avoidance vs. a
  pending repath vs. a genuinely unreachable target) — that distinction is
  exactly what took #184 several more measurement cycles after `reason=`
  landed to run down. `reason=` narrowed the search space; it did not close it.

## Missing instruments, ranked by time-saved estimate

1. **A unit test that forces `verify_landmass_acceptance` to fail on a
   synthetic mesh, asserting `apply_nav_clearance` returns `Err` with the
   expected message.** This is the single highest-leverage gap in the whole
   suite: it is the *only* line of defense against the worst failure mode
   documented in this project's history, and it currently has no fast,
   deterministic, `cargo test`-speed coverage at all — only one full
   real-data `prepare` run has ever proven it fires. Cheapest fix in the
   report: construct a `PreparedNavMesh` with a real concave/self-
   intersecting polygon that survives `reject_invalid_geometry`'s coarser
   heuristic but not landmass's own `validate()`, and assert the bail.
2. **`nav doors: unreported interior polygons` promoted from a `"warning"`
   diagnostic to a hard `prepare` failure**, mirroring exactly what
   `verify_landmass_acceptance` already does for mesh validity. This
   diagnostic is the direct descendant of the #148/#177 root cause; leaving
   it advisory means the next occurrence of this exact class ships the same
   way the first two did — silently, discovered by an agent wedging in the
   viewer rather than by `prepare` refusing to finish.
3. **A world-contact / blocking-plane report reachable without the
   `#[ignore]`d env-gated harness**, i.e. a console command (`tnm`-adjacent)
   that runs `world_contact_report`'s logic (`agent.rs:2529-2588`) against
   the *live* running scene from the agent bridge. Today this exists only as
   `wedge_replay`/`stall_replay`, which require a separate `cargo test`
   invocation with env vars and a scene path, outside the loop of an
   interactive acceptance session. Given how much orchestrator time went
   into external collider scans before this machinery existed (four waves
   for #148 alone), a bridge-reachable equivalent would collapse "agent
   stopped, why" from a context-switch into one console command.
4. **A structural/shape fingerprint test per prepared type**, comparing a
   hash of the type's field list (or a round-trip through a stored golden
   RON fixture) against the corresponding `*_REVISION` constant, failing
   when they disagree without a matching revision bump. This is the durable
   fix AGENTS.md's own "Prepared asset revisions" section asks for by
   convention but does not yet enforce mechanically — the wave-4
   `mesh_merges`-without-a-bump incident it cites was caught by external
   review, not by a test, and nothing in this branch's `nav-graph-v5` → `v8`
   sequence of bumps (four revisions across four waves) is checked by
   anything stronger than the single pinned-string test at
   `nav_graph.rs:1606`.

## Acceptance-methodology recommendations

`docs/plans/M4_WAVE{9,10,11}_MANUAL.md` are genuinely reproducible as
written: every step gives an exact `cargo run-dev` invocation, exact
console commands with real FormIDs and coordinates pulled from the prepared
catalog, and an expected numeric or textual output to compare against
(e.g. `M4_WAVE11_MANUAL.md` step 4's exact spawn coordinate and step 5's
exact stopping point). That specificity is what makes them usable evidence
rather than vibes — keep it.

Two structural weaknesses, both visible in this branch's own history:

- **The manual script for this exact `#177`/`#184` fix does not exist yet.**
  `M4_WAVE11_MANUAL.md` predates the six commits on this branch
  (`3101ed8`…`461d190`) that actually close out #177's scope and add #184's
  fix; the plan/manual pair that should document *this* work — with the
  `MetroGateLoad`/`VaultGearDoor` routes re-measured as `unreachable`/expensive
  rather than walk-through, and the #184 corridor re-measured at full speed —
  has not been written. Per AGENTS.md's own "way of working," a wave does not
  finish without this; the risk of skipping it is exactly what happened
  three separate times already in this arc (#148, #172, #177 each had a
  "closes it" acceptance criterion written before the fix, then falsified by
  real-data measurement after the fact once the fix landed).
- **The three orchestrator measurement errors the brief names all trace to
  the same root problem: a probe issued from *outside* the prepared
  geometry's own frame (external coordinate guesses, a capsule-centre-only
  collider scan, an unfocused window silently not ticking) instead of *asking
  the running system to explain itself*.** The fixes that actually worked —
  `BEVYOUT_NAV_PROBE` sampling `y` from the covering polygon rather than a
  guessed constant (`navmesh.rs:643-731`), `world_contact_report` sweeping the
  real capsule instead of a point (`agent.rs:2529`), and `WinitSettings::
  continuous()` plus `--unfocused` removing the silent-freeze failure mode
  entirely (#180) — are all instances of the same fix shape: replace an
  external approximation with an in-process query. Recommendation for future
  waves: before an orchestrator writes a coordinate, radius, or "should be
  reachable" claim into a plan or acceptance script by hand, prefer a console
  command or bridge method that computes it from the loaded scene. The
  `tnm`-adjacent gap in "Missing instruments" item 3 above is the concrete
  next instance of this same fix.
