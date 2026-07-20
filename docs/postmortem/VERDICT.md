# Navmesh solution — post-mortem verdict

**Scope:** the whole navigation feature — ESM4 NAVM/NAVI decode, the prepared
nav pipeline (clearance, sub-triangle clipping, door derivation), the runtime
layer (`src/viewer/nav/`, `bevy_landmass` 0.12 / `landmass` 0.9.2), and the
tests and diagnostics around them. ~21k lines across 15 files, delivered over
M4 waves 2–11 (issues #111–#185).

**Method:** four independent read-only analyses, cross-checked and
spot-verified by the orchestrator. Detailed evidence lives in
[`01_ARCHITECTURE_CORRECTNESS.md`](01_ARCHITECTURE_CORRECTNESS.md),
[`02_IMPLEMENTATION_FIDELITY.md`](02_IMPLEMENTATION_FIDELITY.md),
[`03_CODE_SMELLS.md`](03_CODE_SMELLS.md), and
[`04_TEST_AND_EVIDENCE_QUALITY.md`](04_TEST_AND_EVIDENCE_QUALITY.md).
Findings below that the orchestrator independently re-verified against the
code are marked **[verified]**.

---

## Verdict

**The architecture is sound and the code is clean. The verification was
not, and that is what cost this project four waves.**

Every load-bearing design decision holds up against OpenMW, FO3/GECK and
general navmesh practice: physics-authoritative movement, authored NAVM
validated rather than trusted, doors as state-dependent route topology
rather than baked geometry. Code hygiene is genuinely above average — zero
`ponytail:`/`TODO`/`FIXME`/`HACK` markers anywhere in the nav surface, every
non-obvious constant carries a derivation comment, and the one retired
subsystem was actually deleted rather than left rotting.

What repeatedly failed was the layer that was supposed to tell us the truth.
Three separate defects shipped behind checks that **agreed with the bug**,
and two of those check-shapes are still in the tree today. The single most
valuable output of this post-mortem is not an architectural correction — it
is the list of instruments in §4.

A second, structural cause sits underneath: the door defect was fixed as an
**allow-list**, not an **invariant**. That is why one bug appeared in three
costumes (#148 metro gate, #172 vault door, the z≈−64 in-cell door) across
four waves.

---

## 1. The root cause: checks that agree with the bug

This pattern appears in all four analyses and is the through-line of the
entire saga.

| Instance | What the check said | What was true |
|---|---|---|
| `PauseAgent` on door-wait failure | test asserted the component was **present** | it was a leak that froze agents permanently; the test protected it |
| Vault 101 clearance | `smallest largest-component share 98%` | the cell had **zero navigation** — landmass rejected the whole mesh |
| Package catalog | `3021 unsupported subrecord` | 3021 of 3021 records — the 3 real findings were invisible |
| `collision_blocked` | "blocked" | conflated *wedged against a wall* with *steering produced no motion*, for four waves |

**Still live in the tree:**

- **[verified]** `verify_landmass_acceptance` (`src/vsa/prepare/navmesh.rs:567`)
  — the guard added *because* of the 98%-with-no-navigation failure — appears
  exactly twice in the repo: its definition and its single call site at
  `:1016`. **No test references it.** The guard against our worst shipped
  failure has no proof it still fires.
- **[verified]** `unreported interior polygons` is a `format!` field in a
  summary line (`navmesh.rs:1378`), not an assertion. It is the direct
  descendant of #148/#177's root cause and can report a non-zero count
  without failing anything.
- `nav_doors.rs`'s invariant check (`:463`) and the code it validates
  (`derive_door_associations`, `:149`) both call the same
  `point_in_convex_polygon` primitive (`:242`) — so the invariant cannot
  catch a bug in the thing they share. The 98% failure shape, in miniature.
- #177's cost-logic unit tests (`agent.rs:8029-8069`) bypass
  `InteractionState` entirely, which is precisely why finding §2.1 below
  survived review.

**The counter-example worth copying:** the package catalog's
`deferred` / `out-of-scope` / `unsupported` split. It took a diagnostic that
fired on 100% of records and reduced it to `0 unsupported, 3 unresolved` —
a number worth reading. That is what every metric here should look like.

---

## 2. Findings requiring action

### 2.1 Non-`Door` blockers can never be marked open — world/nav desync — **HIGH** **[verified]**

`PreparedSemantic::Activator` activation (`src/viewer/interaction/activation.rs:374-386`)
writes the sound, shows the notice, logs `activated {name}`, and **plays the
Opening animation** — but never inserts into `state.open`. Nav's
`apply_door_lock_overrides` reads only `InteractionState.open`, which is
populated solely for `Door`/`Container`/`Corpse`. The console `activate`
command hard-rejects anything else (`world_commands.rs:358-363`).

Consequence: **`VaultGearDoor` animates open in the world while nav still
models it as closed**, holding its polygons at `INFINITY` forever. Live
evidence from acceptance logs: `activated Vault Door (00024710)` followed by
`door anim 00024710 Open`, with routes past it still `unreachable`.

This makes #177's own acceptance clause — "with the blocker opened/disabled
the same route completes" — currently unsatisfiable for its headline case.
**Filed as its own issue.**

### 2.2 Doors were fixed as an allow-list, not an invariant — **HIGH**

The blocking rule keys on `PreparedSemantic::Door` (plus a kinematic class),
rather than expressing the general invariant: *any solid placement that
blocks the player capsule is route topology, and no walkable polygon may sit
inside one while it blocks.* The allow-list is why the same defect recurred
three times in different record types. §2.1 is the fourth instance of the
same gap, arriving on schedule.

Recommended: state the invariant, enforce it prepare-side as a **hard
failure** (see §4.2), and let record type select *behaviour* (openable, needs
key, never opens) rather than *whether the rule applies at all*.

### 2.3 Border ORCA silently contradicted the movement invariant for the feature's entire life — **HIGH (now fixed; lesson stands)**

`landmass`'s default navmesh-border avoidance projects 3D border edges into
2D (`dodgy_2d`) and applies them as hard velocity constraints. Since #114
declared movement physics-authoritative and #153/#171 made the prepared mesh
boundary *be* the agent-radius clearance boundary, this was a second,
competing wall-avoidance authority — invisible to every collision diagnostic
because it never produces contact planes. It caused the z≈−66.22 stall
(#184) and, by conflation, much of the four-wave collider hunt.

The lesson generalises: **when adopting a backend, audit which of its default
behaviours overlap an invariant you have claimed for yourself.** Nothing in
the codebase recorded that landmass was also doing wall avoidance.

### 2.4 Prepared artifact scale — **MEDIUM** (tracked in #179)

45× polygon growth (1338 → 58k on one interior cell), 27 MB `navgraph.ron`,
~70 s prepare. Two compounding causes: sub-triangle clipping emits both sides
of every cut, and the artifact is written with `to_string_pretty` /
`PrettyConfig::default()` (`nav_graph.rs:1510`) then full-text RON-parsed on
every viewer launch (`nav/mod.rs:40-44`) — the most expensive possible format
choice for a machine-only artifact. Analyst 3 notes the adaptive-refinement
clip pass, not any listed smell, is likely the dominant share of the 70 s.
Blocking for M6 exteriors; not blocking for interiors today.

### 2.5 Four hand-rolled point-in-polygon primitives with three different epsilons — **MEDIUM** **[verified]**

`landmass_graph.rs:838` (`point_in_triangle_xz`), `nav_clearance.rs:284`
(`barycentric_xz`), `nav_doors.rs:242` (`point_in_convex_polygon`), plus an
inline copy in `navmesh.rs:693` — with tolerances ranging from exact zero to
1e-4 to 1e-9. This codebase has *already* been bitten twice by an epsilon
being wrong in this exact family (1e-4 as a shape filter, then 1e-9 retaining
unwindable slivers). Four implementations mean four chances to be
inconsistent at a seam, and §1 shows one of them is also the shared primitive
that makes an invariant vacuous.

### 2.6 `agent.rs` is 9,386 lines doing six jobs — **MEDIUM**

Console commands, door lifecycle, merge traversal, KCC/physics, ledger
restore, archipelago lifecycle. The repo has already solved exactly this
problem twice (`viewer::interaction`, `viewer::console` capability modules
with enforced line caps). Six fragile `unwrap()`s in
`drive_door_link_for_agent` and four `too_many_arguments` clippy escapes
correlate with the size.

### 2.7 Process gap on the current branch — **MEDIUM**

~2,700 lines across six commits (#177, #184) have landed since the last
manual-acceptance document, with no wave plan or manual-script addendum.
AGENTS.md's "Way of working" was followed rigorously through wave 11 and then
lapsed exactly where the most subtle fixes landed.

### 2.8 Lower-severity

- `tests/architecture.rs`'s layering assertion is a literal `crate::viewer`
  string match, not a general rule; `src/vsa/prepare/navmesh.rs` importing
  `bevy_landmass` is deliberate and defensible but unenforced by any test.
- Cross-mesh portals carry **zero** authored NVTR evidence on either test
  cell — an unquantified M6 risk (#156's finding).
- `NVCI` correlation decode remains unverified against real bytes.
- Water polygons are dropped rather than typed.
- One self-admitted duplicate function (`nav/mod.rs:37-44` vs
  `nav_overlay.rs:278-282`), residue of a worktree ownership boundary.

---

## 3. What is genuinely sound (do not relitigate)

- **Authored NAVM as base topology, validated against collision.** Defensible
  because GECK's NAVM is itself collision-derived then hand-finalised — this
  is not OpenMW's "rebuild from arbitrary mods" case. Preserves doors,
  preferred-path types, water and NAVI correlation a Recast rebuild discards.
- **Physics-authoritative movement.** Matches OpenMW's own `MWPhysics::Actor`
  model.
- **Doors as query-time cost overrides, never baked geometry.** Matches
  Detour's temporary-obstacle pattern and is *better* than OpenMW's reactive
  `AiAvoidDoor`, which exists only because its actors physically walk into
  doors. Holding this line is why an unlocked door becomes passable on the
  next solve with no re-prepare.
- **Prepare-time landmass validation as a hard build failure.** The correct
  response to the 98% incident: replicating a rule catches the class, running
  the real validator proves the runtime will accept the artifact.
- **Revision discipline.** `NAV_GRAPH_REVISION` v3→v8, bumped in the same
  commit as the fields requiring it.
- **Authored/derived door lists kept separate**, verified never merged —
  authored per-door triangle *counts* are load-bearing.
- **The replay harnesses** (`wedge_replay`, `stall_replay`) are genuinely
  reusable, not one-shot: they cracked #148, #172, #177 and #184.
- **Meta:** OpenMW's `loadnavm.cpp` does not decode FO3/FNV `NAVM` at all
  (only the newer `NVNM`). OpenMW is a valid reference for the AI/movement
  layer and *never* for format decoding — a distinction this codebase's own
  docs already get right.

---

## 4. Recommended actions, in priority order

1. **Fix the Activator open-state desync** (§2.1). Real bug, live today.
2. **Promote `unreported interior polygons` to a hard prepare failure**, and
   add a synthetic test that forces `verify_landmass_acceptance` to reject.
   These two are the cheapest possible insurance against the exact class of
   failure that cost the most.
3. **Restate doors as an invariant** (§2.2) and enforce it prepare-side, so
   record type selects behaviour rather than applicability.
4. **Consolidate the four point-in-polygon primitives** into one shared,
   epsilon-documented implementation — and ensure invariant checks do not
   share their primitive with the code they validate.
5. **Ship #185** (key-aware locked doors; `Unreachable` that names the
   blocking door and why). Needed before #115 AI packages, which must
   distinguish "locked, find key" from "no path, abandon".
6. **Split `agent.rs`** along the repo's existing capability-module
   precedent.
7. **Artifact size** (#179): the RON pretty-printing lever first — it is
   near-free and independent of the risky index-remapping work.
8. **Restore the process** (§2.7): a plan and manual script covering #177 and
   #184 before this branch merges.

## 5. Note on analyst disagreement

Analyst 3 judged the code "unusually disciplined"; analyst 2 reported drift
and a process gap. Both are correct at different levels, and the contrast is
itself the finding: **hygiene is high wherever it is visible to a linter,
a reviewer or a metric, and weak wherever it is not.** No hacks, no dead
code, no stray TODOs — but a guard with no test, an invariant sharing its
primitive with its subject, a backend quietly fighting a stated invariant,
and a metric that read 98% for a cell with no navigation.

That asymmetry, not any individual defect, is the thing to correct.
