# Implementation Fidelity — bevyout navmesh solution

Analyst 2 of 4. Axis: does the shipped code actually do what the docs, module
comments, plans, and issues say it does? Branch `m4-wave11-177-doors`
(HEAD `461d190`), read-only.

## Verdict

Fidelity is unusually high for the parts that have been through a full
plan → cucumber/unit → real-data-acceptance cycle: the prepare-side pure
modules (`nav_graph.rs`, `nav_clearance.rs`, `nav_clip.rs`, `nav_doors.rs`)
and the FSMs in `door_link.rs`/`fall_guard.rs` do exactly what their doc
comments claim, are exhaustively unit-tested against those exact claims, and
the revision-bump discipline AGENTS.md demands has been followed to the
letter every single time a serialized shape changed on this branch. That
discipline is the project's strongest asset.

But the branch's newest, still-open work (#177, generalizing door topology to
non-`Door` blocking placements) has a load-bearing gap between what the code
believes and what the rest of the runtime actually offers: the mechanism it
relies on to ever *un-block* a non-`Door` blocker (an `Activator` such as
`VaultGearDoor`, #177's own motivating example) does not exist anywhere in
the codebase today. `InteractionState.open` — the single signal
`door_open_and_locked` reads to decide a blocker is passable — is populated
for `Door`, `Container`, and `Corpse` semantics only; `PreparedSemantic::
Activator`'s activation handler (`src/viewer/interaction/activation.rs:374-387`)
plays a sound and an animation and never touches it, and the console
`activate` command explicitly rejects anything that isn't a door/container/
corpse/pickup (`src/viewer/console/world_commands.rs:358-363`). The unit
tests that exercise `apply_door_lock_overrides`'s "openable blocker" branch
construct `NavArchipelagoState.door_open` directly and never route through
`InteractionState`, so they cannot see this gap — they prove the cost
composition arithmetic is right without proving the state it composes over
is ever reachable. This is exactly the "state that can be entered but not
left" pattern the task asked to look for, on the issue's own headline case.

Also notable: substantial code (6 commits, ~1700 new lines in `agent.rs`
alone, a new `nav_doors.rs` module, a 502-line landmass-avoidance fix) has
landed on this branch under #177/#184 with **no accompanying wave plan or
manual acceptance script** — a direct process gap against AGENTS.md's "Way
of working," which is otherwise followed meticulously for waves 6-11.

## Where implementation matches intent

- **Revision discipline is real, not decorative.** Every struct in
  `nav_graph.rs` carries `#[serde(default)]` (so old caches deserialize
  instead of erroring) *and* `NAV_GRAPH_REVISION` is bumped on every shape
  change including a bare `bool` field. Commit `16c9a95` is a clean example:
  it adds `PreparedNavDerivedDoor::openable` and bumps v7→v8 *in the same
  commit*, with a doc comment explaining why the new field specifically
  needs the bump (`src/vsa/prepare/nav_graph.rs:30-35`). `PACKAGE_CATALOG_
  REVISION` and `ACTOR_CATALOG_REVISION` show the same pattern for the
  wave's other new prepared assets. `tests::revision_is_pinned` pins the
  literal string, so a bump-and-forget is caught immediately.
- **The authored/derived door separation genuinely holds.** `PreparedNavMesh
  ::derived_doors` is a field distinct from `::doors`
  (`src/vsa/prepare/nav_graph.rs:343-358`), the boundary conversion in
  `src/viewer/nav/mod.rs:92-113` carries both through without merging them,
  and `landmass_graph::build_navigation_mesh` applies authored typing first
  and only lets a derived association overwrite it when the derived one is
  the *stricter* (`blocks_when_closed`) class
  (`src/viewer/nav/landmass_graph.rs:373-398`). The doc comment's claim that
  merging them would "silently reclassify authored doors" is enforced by
  code, not just stated.
- **Polygon type-index allocation cannot collide by construction.**
  `closed_door_type_indices` explicitly bases its allocation at `door_type_
  indices.values().max() + 1` (`landmass_graph.rs:227`), and `preferred_
  pathing_type_index` bases itself one past the max of *both* maps
  (`landmass_graph.rs:248-258`). All three are unit-tested for the
  no-collision property directly.
- **`fall_guard.rs` and `door_link.rs` are exactly as pure and thin as their
  module docs claim.** No `bevy` import, callable and tested with plain
  values, and the one Bevy system that consumes each (`nav_fall_guard_
  system`, `drive_door_link_for_agent`) does nothing but sample state and
  apply the returned verdict — verified by reading both call sites.
- **`verify_landmass_acceptance` is a real hard gate, not a soft warning.**
  `src/vsa/prepare/navmesh.rs:1014-1025` turns a landmass-rejection into
  `anyhow::bail!`, which is what the wave-11 "shipped amendments" claims
  (A3: "running landmass's own validator at prepare time as a hard build
  failure... the durable fix").
- **`PreparedNavPolygon::walkable`'s Default/serde-default mismatch (found
  live in wave 10) stays fixed and guarded**: a manual `impl Default`
  matching `default_walkable()` with an explicit comment explaining why a
  derived `Default` would silently break `mesh_inputs`'s filter
  (`nav_graph.rs:214-237`).

## Drift and partial implementations

### 1. #177's escape hatch for non-`Door` blockers has no runtime path (High)

**Claim** (issue #177, and `apply_door_lock_overrides`'s own doc comment,
`agent.rs:1819-1830`): a closed, non-openable blocker is priced
`LOCKED_DOOR_TYPE_INDEX_COST` (near-impassable) while closed, and "opening
the blocker clears the entry entirely" — i.e. there is a way to open it and
observe that in the route cost.

**What the code does**: the only signal `apply_door_lock_overrides` and
`door_availability_system` ever read for "is this blocker open" is `state.
door_open`, itself populated purely from `interaction::InteractionState.
open.contains(&entity)` (`agent.rs:1733-1742`, `door_open_and_locked`). That
set is mutated in exactly five places, all gated on `PreparedSemantic::Door`
or `Container`/`Corpse`:
`src/viewer/interaction/activation.rs:254,299`,
`src/viewer/interaction/scripted.rs:71,200,253`. `PreparedSemantic::
Activator`'s own arm (`activation.rs:374-387`) plays a sound/animation and
returns — it never touches `state.open`. The console `activate` command
(the other place a human or the agent bridge could poke this state) refuses
anything that isn't `Door(_)`/`Container`/`Corpse`/`Pickup`
(`console/world_commands.rs:358-363`, message: `"activate supports only
door, container, corpse, and pickup references"`). There is also no
`enable`/`disable` console command that could satisfy the acceptance
criterion's "opened/disabled" alternative.

**Consequence**: `VaultGearDoor` — `base_kind: ACTI`, #177's own headline
reproduction case — can be driven into the *closed* state (trivially, it
starts that way) but never driven into the *open* state by anything in this
codebase. The half of #177's acceptance criterion that reads "with the
blocker opened/disabled the same route completes" is currently unverifiable
by any player action, console command, or BRP call. The unit tests at
`agent.rs:8029-8069` (`closed_blocker_override_world`,
`a_closed_unlocked_door_stays_passable_but_expensive`, etc.) construct
`NavArchipelagoState` fields directly and never exercise `InteractionState`,
so they pass without ever discovering this — they validate the cost
arithmetic is correct, not that the state it's computed over is reachable.

**Severity/consequence**: high for anyone continuing #177 — the natural next
step ("wire up a script/quest event to open the vault door") is scoped as
out-of-wave (#115/#15, per the wave-11 manual's package-catalog note), but
nothing currently in the issue or the code flags that the *acceptance
criterion as written* depends on that unscoped work landing first. A
reviewer reading the tests would reasonably conclude the open path is
covered; it is not.

### 2. #177/#184 landed with no wave plan or manual acceptance script (Medium-High, process fidelity)

AGENTS.md's "Way of working" states every wave gets a `*_PLAN.md` before
implementation and ends with a `docs/plans/M4_WAVE<n>_MANUAL.md` "written
before the wave PR." Six commits after `16a7dcd` ("M4 wave 11: shipped
amendments A3-A6, manual acceptance script...") land #177's real
implementation (`3101ed8` through `14995b0`) and #184's fix (`461d190`) —
substantial changes (`nav_doors.rs` new at 481 lines, `nav_clip.rs` new at
995 lines, `agent.rs` +1701 lines total on the branch) — with `git diff
master...HEAD -- docs/` showing **zero** new or amended plan/manual
content past `M4_WAVE11_MANUAL.md`, which itself predates all of #177's
actual fix commits and only records the *pre-fix* symptom (§E, "Known,
tracked, and not regressions"). There is no `M4_WAVE12_PLAN.md` and no
manual-script addendum for #177/#184 anywhere in `docs/plans/`. Whether this
reflects genuinely mid-flight work (plausible, given the issue is still
`state: OPEN` on GitHub) or a process step that's about to be skipped isn't
something the repository state can distinguish — but as of this snapshot,
the "human must be able to see what a wave shipped" step this project treats
as load-bearing has not happened for this branch's largest chunk of new
behavior.

### 3. `agent.rs` is 9,386 lines and decision logic is not uniformly pushed to pure modules (Medium)

The project's stated pattern ("keep decision logic... in pure modules and
let thin Bevy systems consume them") is followed well for the newer,
narrowly-scoped concerns (`door_link`, `fall_guard`, `nav_doors`,
`movement_policy`, `ledger_policy`, `repath`) — each is a small,
`#[path]`-included, std-only module with a thin Bevy caller. But `agent.rs`
itself still contains inline decision logic that reads like it belongs in
one of those modules and simply predates the pattern or grew alongside it:
- `apply_door_lock_overrides` (`agent.rs:1800-1857`) — the actual cost
  composition rule ("openable+usable → CLOSED_DOOR_TYPE_INDEX_COST; locked
  or not-openable → LOCKED_DOOR_TYPE_INDEX_COST") is a pure function of
  three booleans, written and tested entirely inside `agent.rs` as a `World`
  -mutating function rather than as a `(bool, bool, bool) -> f32` pure
  function callable from `tests/features.rs`. Contrast with `door_link::
  effective_door_open`, the same shape of decision, which *is* factored out
  pure. The tests at `agent.rs:8054+` prove this could be pure with almost
  no rewrite — they already isolate the three booleans into a fixture
  function.
- `door_usable_now`/`door_open_and_locked` (`agent.rs:1728-1770`) mix a pure
  decision (`repath::door_usable`, already factored out) with the
  world-reading boundary in the same function, which is fine, but the
  boundary logic itself (open = registry present AND reference resolves AND
  InteractionState contains it) has no test coverage independent of a full
  `World` — the gap in finding 1 above would have been visible sooner if
  this boundary had its own small, directly-testable seam.

This is not a correctness bug, but it is exactly the kind of thing that
makes `agent.rs` risky to extend: the file is too large for a reader to
build a mental model of "which parts are safe to unit-test in isolation
and which require a full harness" without reading linearly.

### 4. Two independent "kind index" numbering schemes share terminology (Low-Medium)

`landmass_graph::door_type_indices`/`closed_door_type_indices` allocate
`landmass` polygon `type_index` values (consumed by `AgentTypeIndexCostOverrides`,
costs). `landmass_graph::merge_link_kind`/`permitted_animation_link_kinds`
(#162) allocate a *different* index space — animation-link "kind" identifiers
consumed by `PermittedAnimationLinks` for quarantine — that also starts at 1
and is also called a "kind"/"index" throughout comments (`landmass_graph.rs:903-930`).
They are stored in genuinely different `bevy_landmass` component types so
there is no runtime collision, but nothing in either function's doc comment
cross-references the other's existence or clarifies they are unrelated
number lines. A future maintainer skimming for "where do type indices come
from" has a real chance of conflating the two, especially since both files
begin their allocation the same way (`BTreeMap` keyed by FormID, offset from
`enumerate()`).

## Boundary/data-loss findings

- **`walkable` flag**: correctly threaded end-to-end. `nav_clearance` sets
  it false with a named reason; `mesh_inputs` filters on it before
  `landmass_graph` ever sees the polygon (`nav/mod.rs:74-91`); nothing
  downstream re-derives or overrides it. Confirmed no double-filtering or
  silent re-inclusion.
- **Authored vs. derived door association separation**: holds under
  inspection (see "Where implementation matches intent"), including the
  precedence rule at the one point they interact
  (`landmass_graph.rs:373-398`). One residual, plausible-but-unconfirmed
  risk: `protected_edges_for_prepared_mesh` (`navmesh.rs:509-536`) protects
  *authored* door triangles from the clearance/clip pass, but derived-door
  associations are computed *after* clipping using post-clip triangle
  indices (per `PreparedNavDerivedDoor`'s own doc comment,
  `nav_graph.rs:356-357`) — so there is no equivalent protection need for
  them, and the sequencing is correct. But if clearance erodes a doorway
  footprint down to zero surviving polygons before `derive_door_associations`
  runs, that door simply gets zero associations with no diagnostic
  distinguishing "no door here" from "door was clipped away" — this is
  the same erosion-aggressiveness risk wave 10/11 already tracked for
  authored geometry (their A1/A2 amendments), just not yet demonstrated for
  the derived-door case specifically.
- **Polygon type-index collision**: verified collision-free by construction
  (see above) — no data loss here, contrary to what might be assumed from
  "landmass stores exactly one `type_index` per polygon." The precedence
  rule (door wins over preferred-pathing) is the one place information is
  deliberately dropped, and it is dropped in the direction the code and
  tests both document as safe (a locked door must never accidentally become
  cheaper because it's also flagged preferred-pathing).
- **Merge-link kinds (#162)**: `quarantined_merge_link_kinds` is per-agent
  (`AgentRuntime`, `agent.rs:843`) and reset both explicitly
  (`clear_merge_link_quarantine`) and on repath/despawn — matches the module
  doc's claimed lifecycle. Not independently re-verified against real
  blocked-link data on this pass (wave 10's own A6 amendment already
  recorded that no real-data forced-block scenario existed at the time;
  nothing on this branch changes that).
- **A structurally significant, previously-invisible boundary loss**: commit
  `461d190` documents that `bevy_landmass`'s default navmesh-border ORCA
  avoidance was silently projecting the agent's own staircase (connected
  walkable ground, finely sub-triangulated by #171's clipping) as hard 2D
  obstacles up to 1.35 m away with real clearance, decaying desired velocity
  to zero with **zero contact planes** — invisible to every collision
  diagnostic added over four prior waves because it acts entirely in
  landmass's internal velocity-space avoidance layer, never through the
  physics-authoritative KCC contacts the project's diagnostics are built
  around (module doc, `movement_policy`/`decide_collision_outcome`). This
  is now disabled globally via `archipelago_options()`. It's evidence that
  "physics is authoritative for movement" (#114) was violated by a
  dependency default for the whole life of the nav feature until this
  commit, undetected by the extensive KCC-contact-based test suite because
  that suite has no way to observe landmass's internal avoidance math.

## Invariants asserted in docs but not enforced anywhere

- **"src/vsa/ must not import viewer"** is enforced
  (`tests::preparation_does_not_depend_on_viewer`), but the check is a
  literal substring match for `crate::viewer` — it does not, and by
  construction cannot, catch a dependency inversion expressed through any
  other crate. `src/vsa/prepare/navmesh.rs:568` imports `bevy_landmass::
  NavigationMesh3d` directly specifically *to avoid* going through `viewer`
  (its own comment: `"bevy_landmass is imported directly rather than
  through viewer, which tests/architecture.rs forbids and which would
  invert the dependency"`, `navmesh.rs:565-566`). That reasoning is sound —
  it mirrors `landmass_graph.rs`'s own precedented "bevy_landmass but never
  bevy" exception (`landmass_graph.rs:24-36`) — but no test asserts the
  *general* rule ("prepare code may use `bevy_landmass`/`glam` but never a
  full ECS/`bevy::app` import"), only the narrower "no `crate::viewer`"
  string match. A future prepare-side module could import full `bevy::app`
  or spin up an ad-hoc `App`/`World` and nothing in `tests/architecture.rs`
  would catch it.
- **"cucumber-driven modules must be std/serde-only"** (AGENTS.md's testing
  section) is not asserted by any test either — it is enforced entirely by
  convention and by the fact that `#[path]`-including a file with a `bevy`
  import would fail to compile *inside the specific translation unit*
  `tests/features.rs` builds, which does catch real `bevy` (the full
  engine) imports at compile time, but does not catch and was never meant
  to catch `bevy_landmass`/`glam`-only files (both `landmass_graph.rs` and
  `door_link.rs`/`nav_doors.rs` are `#[path]`-included and non-`bevy`, by
  design, and this works). The invariant is real but its enforcement is
  "the build fails if violated," which is adequate but undocumented as the
  actual mechanism — nobody reading only `tests/architecture.rs` would find
  where this rule is checked.
- **"#177's opened/disabled blocker route completes"** — asserted as
  acceptance criteria in the issue text, not enforced by any test on this
  branch (see Drift finding 1). No cucumber scenario, no `agent.rs`
  integration test, and no manual-script step exercises the open path for a
  non-`Door` blocker end-to-end through `InteractionState`.
- **"A locked door never opens through the scripted... activation
  boundary... so real FO3 data is not expected to exercise [`MAX_WAIT_
  TICKS`] in this spike"** (`door_link.rs:11-17`) is asserted but not
  proven for the derived-blocker case introduced by #177 on this branch:
  the doc comment predates #177 and talks about authored `Door` semantics
  specifically; whether a derived-gate blocker with `openable: true` but
  the underlying entity permanently unresolvable (e.g. a `RefRegistry` miss)
  behaves the same way is untested.

## Highest-risk areas for future change

1. **`src/viewer/nav/agent.rs` at 9,386 lines.** It is the one place in the
   nav stack that has not been decomposed to the project's own stated
   pattern (small pure module + thin Bevy caller). Every wave so far has
   added to it rather than split out of it (#155, #156, #157, #162, #164,
   #165, #171, #177, #184 all touch it). The file's own internal test
   fixtures (e.g. `closed_blocker_override_world`,
   `door_topology_test_app`) show the pure decisions *could* be extracted
   with little friction — the next wave that touches door/blocker cost
   logic should do that extraction rather than add a tenth special case
   inline.
2. **The `InteractionState.open` / `PreparedSemantic` coupling.** Every nav
   mechanism that needs to know "is this thing open" goes through a set
   keyed by entity and populated by semantic-specific activation code
   scattered across `activation.rs` and `scripted.rs`. Adding a new
   semantic (as #177 needs for `Activator`, and as future quest-scripted
   object state will need) requires remembering to update this set from
   every new activation path — there is no single trait or table enumerating
   "which semantics can be open" the way `door_type_indices` enumerates
   "which FormIDs need a type index." Finding 1 is a direct symptom of this
   coupling being easy to miss.
3. **Clearance-vs-derived-door sequencing.** The system depends on clearance
   running to completion, correctly, before `apply_derived_door_associations`
   runs — correct today, but a future change that reorders `navmesh.rs`'s
   pipeline (the module doc's own multi-paragraph justification for the
   current order suggests this has already been reworked more than once)
   could silently break the derived-door mechanism with no test catching it
   except real-data acceptance, since the unit/cucumber suites for
   `nav_doors.rs` operate on synthetic geometry that is never run through
   `nav_clearance`/`nav_clip` first.
4. **No plan/manual doc trail for #177/#184 as of this snapshot** (Drift
   finding 2). Whoever picks this branch up next has to reconstruct scope
   and acceptance criteria from `gh issue view 177`'s comment history rather
   than a `PLAN.md`, which is exactly the kind of thing this project's own
   process was designed to prevent.
