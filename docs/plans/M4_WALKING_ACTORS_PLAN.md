# M4 walking-actors wave — plan (#188, #186, #189)

Wave on branch `m4-walking-actors`, off master @ `62797f8`. Named rather than
numbered: the actor-animation track independently used "wave 10" and "wave 12",
so nav wave numbers past 11 collide (same reasoning as the doors wave).

Epic: #9. All three issues are OPEN and assigned to `nippongun` — the
assignment invariant holds at kickoff.

**Execution model recommendation (Claude runtime): `Opus` for all three
executors.** Every lane sits on a seam where this project has already shipped a
defect behind a check that agreed with it: #188 introduces a second potential
movement authority (the exact shape that cost four waves, verdict §2.3), #186
is a state-desync whose existing tests bypass the signal they should exercise
(§1), and #189's point-in-polygon family has produced two shipped epsilon
defects (§2.5). None of these are mechanical. The orchestrating session (Opus)
plans, reviews diffs, runs gates and owns real-data acceptance, and writes no
production or test code.

## Wave shape

Two lanes.

| Lane | Issues | Execution | Rationale |
|---|---|---|---|
| **A — actor/nav seam** | #188 → #186 | Sequential, one executor, directly on the wave branch | Both rework `src/viewer/nav/agent.rs` and the interaction/nav state seam. AGENTS.md's sequential exception; #186's brief builds on #188's landed binding. |
| **B — nav invariants** | #189 | Parallel, isolated worktree | Prepare-side guards plus one viewer-side primitive. Disjoint files from lane A. |

**File ownership boundary (hard).**

- Lane A owns: `src/viewer/nav/agent.rs`, `src/viewer/nav/movement_policy.rs`,
  `src/viewer/actor.rs`, `src/viewer/actor_animation/**`,
  `src/viewer/interaction/activation.rs`, `src/viewer/interaction/state.rs`,
  `src/viewer/console/world_commands.rs`, and any new
  `src/viewer/nav/locomotion.rs`.
- Lane B owns: `src/vsa/prepare/navmesh.rs`, `src/vsa/prepare/nav_doors.rs`,
  `src/vsa/prepare/nav_clearance.rs`, `src/viewer/nav/landmass_graph.rs`, and
  the new shared point-in-polygon module.
- `src/viewer/nav/landmass_graph.rs` is the only viewer-side file in lane B and
  lane A must not touch it. Lane B must not touch `agent.rs`.
- Shared merge seam is `tests/features.rs` per the usual convention: each lane
  appends World fields at the end of the struct and a delimited step section at
  the end of the file.

---

## Issue #188 — bind nav agents to projected actors, drive animation from locomotion

### Fixed feature list

1. **F188.1 Binding.** A projected actor (`ActorRuntime`) can own a nav agent so
   one entity has routing + KCC movement + skeleton + animation. The `tna`
   debug-capsule path keeps working unchanged — it is the harness every nav
   wave has relied on and it is not to be migrated onto the new path.
2. **F188.2 Locomotion policy.** A pure, std-only module mapping **achieved**
   horizontal velocity and turn rate to `ActorAnimationState`
   (`Idle`/`Walk`/`Run`/`TurnLeft`/`TurnRight`), with hysteresis on every
   threshold so an agent hovering at the walk/run boundary does not flap.
   Thresholds and hysteresis bands carry derivation comments (repo convention:
   every non-obvious constant explains itself).
3. **F188.3 Thin consumer system.** A Bevy system reading the `desired`/
   `achieved` pair *already computed* at `agent.rs:2679-2697` for
   `decide_collision_outcome` — not a recomputation — and calling
   `request_actor_animation`.
4. **F188.4 One movement authority.** The KCC remains sole authority for
   position. The selected clip's `root_motion_policy()` is checked and
   gameplay-owned accumulation roots cannot feed back into the agent transform.
5. **F188.5 Lifecycle coherence.** Hidden/inactive-cell actors stay paused
   (#106 already does this); agents removed by the fall guard (#164) release
   animation state; cell hand-off (#134) carries or rebuilds it.

### Non-goals

No package or schedule logic (#115). No combat/equip-driven state beyond what
#106 already selects. No root-motion-driven movement, ever.

### Tests (written before implementation)

- Pure policy unit tests for speed/turn → state, **including both edges of each
  hysteresis band** — assert that a speed oscillating across a raw threshold
  produces a stable state, which is the actual requirement.
- Minimal-`App`: a moving bound actor requests a locomotion state; a stationary
  one requests `Idle`.
- **The one-authority invariant test:** agent transform is bit-identical across
  a fixed number of ticks whether or not clips are playing. This is the test
  that would have caught verdict §2.3's shape. It must fail if root motion is
  wired into translation.
- Regression: `tna spawn`/`goto` capsules still route; `actoranim` and
  `actorinspect` still work.

### Acceptance (real data)

In a prepared cell with real NPCs: a bound actor given a `tna goto`-equivalent
destination walks its route with feet-on-floor locomotion clips playing,
transitions Idle↔Walk↔Run at the documented thresholds without flapping, turns
in place with turn clips, and its logged position sequence is identical with
clips on and off.

### Implementation notes

- Put the policy in a std/serde-only module so it is cucumber-reachable via
  `#[path]` (repo pattern: `src/viewer/world/policy.rs`).
- Resist adding the binding as a new parallel spawn path. The narrowest change
  is a component that marks an existing `ActorRuntime` entity as agent-owning.
- `agent.rs` is already 9,389 lines (verdict §2.6). Do not grow its module root
  materially; new logic goes in new modules.

---

## Issue #186 — Activator blockers animate open but never register as open

Runs **after** #188 lands on the wave branch.

### Fixed feature list

1. **F186.1 Observable blocker state.** A runtime open/close state for solid
   non-`Door` blockers that nav can observe — either by populating
   `InteractionState.open` from the Activator path (which already computes the
   open/close transition it feeds to the animation) or by giving nav a general
   blocker-state source that is not door-specific.
2. **F186.2 Console parity.** `activate` drives that state for Activator
   blockers, so the behaviour is human-testable — the same gap #177 closed for
   ordinary doors. Currently `world_commands.rs:358-363` hard-rejects anything
   that is not door/container/corpse/pickup.
3. **F186.3 Invariant framing, not a second allow-list.** Verdict §2.2 is
   explicit that this defect recurred three times *because* it was fixed as an
   allow-list keyed on `PreparedSemantic::Door`. Record type must select
   *behaviour* (openable, needs key, never opens), not *whether the rule
   applies*. A fix that adds `Activator` to a match arm alongside `Door` is the
   fourth costume of the same bug and will be rejected at diff review.

### Tests

- **The signal, not the pricing.** Activate the blocker through the real
  interaction boundary and assert the nav override lifts. #177's existing cost
  tests (`agent.rs:8029-8069`) construct override state directly and bypass
  `InteractionState` — that is exactly why this shipped (§1). Do not add
  another test of that shape.
- A test that fails if the open-state population is removed from the Activator
  path.
- #177's door suites and the wave 8–11 lock/travel suites stay green.

### Acceptance (real data)

On cell `00024512`: with `VaultGearDoor` activated open, a route crossing it
completes instead of reporting `unreachable`; closed, it remains `unreachable`.
No FormID- or coordinate-specific logic anywhere in the fix.

---

## Issue #189 — make the nav invariants actually fail

Lane B, isolated worktree, parallel with lane A.

### Fixed feature list

1. **F189.1 Test the landmass-rejection guard.** `verify_landmass_acceptance`
   (`src/vsa/prepare/navmesh.rs:567`) appears exactly twice in the repo —
   definition and its single call site at `:1016` — with **no test**. It is the
   guard added because Vault 101 shipped with zero navigation while every
   metric read 98%. Add a synthetic test constructing a mesh landmass rejects
   (concave or reverse-wound polygon) asserting the gate fails the build, plus
   one asserting a valid mesh passes.
2. **F189.2 Promote `unreported interior polygons` to a hard failure.** It is
   currently a `format!` field in the `nav doors:` summary (`navmesh.rs:1378`),
   so it can report a non-zero count and still succeed. It is the direct
   descendant of #148/#177's root cause. Include an explicit, documented escape
   hatch if any cell legitimately needs one — and say in the code comment what
   would justify using it.
3. **F189.3 Break the shared-primitive dependency.** `nav_doors.rs`'s invariant
   check (`:463`) and `derive_door_associations` (`:149`) both call the same
   `point_in_convex_polygon` (`:242`), so the invariant cannot catch a bug in
   the primitive they share — the 98% failure shape in miniature. The check
   must verify by an independent means (e.g. sampling against the blocker's
   collision geometry directly).
4. **F189.4 Consolidate four primitives.** `landmass_graph.rs:838`
   (`point_in_triangle_xz`), `nav_clearance.rs:284` (`barycentric_xz`),
   `nav_doors.rs:242` (`point_in_convex_polygon`), and the inline copy at
   `navmesh.rs:693` — tolerances currently range from exact zero to 1e-4 to
   1e-9. One shared implementation with a documented epsilon rationale, **while
   preserving F189.3**: the invariant check keeps an independent path.

### Ordering constraint

F189.3 before F189.4. Consolidating first would make the invariant and its
subject share the *new* primitive, re-creating the defect this issue exists to
remove. The consolidated primitive is for the production paths; the invariant
deliberately does not use it.

### Tests

Each of the four has a test that **fails if the guard is removed or weakened** —
that is the acceptance bar, not merely "a test exists". For F189.4, a test
pinning the documented epsilon behaviour at the boundary, given this family has
already produced two shipped defects (1e-4 acting as a shape filter, 1e-9
retaining unwindable slivers).

### Acceptance

`cargo test` green and both test cells' `prepare` green — including
`prepare` still succeeding on cells that currently report a non-zero
`unreported interior polygons` count. **If promoting F189.2 to a hard failure
breaks a real cell's prepare, that is a finding, not a blocker to work around:
report the count and the cell, and escalate to the orchestrator rather than
widening the escape hatch to make it pass.**

### Revision discipline

If any prepared type's serialized shape changes — including serde-defaulted
fields — bump its `*_REVISION`. Verdict §3 credits this project's
`NAV_GRAPH_REVISION` v3→v8 discipline; keep it.

---

## Gates and hand-off

Per AGENTS.md, before the PR: `cargo fmt --check`, `cargo clippy --all-targets
-- -D warnings`, `cargo test`, and representative `cargo run-dev` commands.
Real-data acceptance on both test cells before the PR, run by the orchestrator.

Manual acceptance script `docs/plans/M4_WALKING_ACTORS_MANUAL.md` is written
before the wave PR and linked from its body. Its player-visible surface is
strong for this wave — actors that actually walk — so no additional test-surface
sub-issue is needed for #188/#186. **#189 has no runtime surface**: its
deliverable is deterministic `prepare` output, which the manual script drives
via CLI output rather than the viewer.

One PR with `Closes #188`, `Closes #186`, `Closes #189`. Master is
PR-protected; no direct push.

## Shipped amendments

_(Appended during execution rather than rewriting the above, per
`docs/plans/README.md`.)_
