# M4 HeadAnims animation-fix wave — plan

Traceability: PROMPT → feature list → tests → implementation. See
`docs/plans/README.md`.

## Execution model recommendation (Claude runtime)

- **#206 (HeadAnims node, `native.rs` merge):** **Opus** — requires
  root-cause debugging of the NIF actor-merge path and a fresh native
  rebuild to verify. Investigation-heavy.
- **#205 (binder robustness, `mod.rs`):** **Sonnet** — well-scoped change
  with a deterministic unit test.

Run **sequentially on the wave branch `m4-headanims`** (not parallel
worktrees): both concern the same symptom and #205's real-data acceptance
reads better once #206 restores the head node. Do #206 first.

## Issue #206 — assembled actors expose a `HeadAnims` node

### Feature list
1. An assembled humanoid actor whose descriptor carries a visible
   `head_anim_part` (hair) exposes a node named `HeadAnims` in the merged
   output, with the hair mesh parented under it.
2. The idle KF's `HeadAnims:0` required target resolves against that node at
   bind time (`bound_targets > 0`, no missing-target diagnostic).
3. If the merged-actor output shape changes, the actor-assembly
   fingerprint/revision is bumped so stale caches invalidate.

### Tests (feature-first)
- Cucumber/unit over the pure assembly-descriptor → merge decision where
  possible (std/serde-only per the `#[path]` include rule); assert that a
  descriptor with a visible hair `head_anim_part` yields a `HeadAnims`
  attachment in the merge plan.
- `#[cfg(test)]` in `native.rs` (or its testable extraction) asserting
  `merge_actor_scene_attached(.., "HeadAnims")` produces a retained,
  correctly named node for a synthetic head+hair scene.

### Implementation
Root-cause why the `HeadAnims`-attached hair produces no runtime node.
Candidate causes to check in order: hair scene dropped as
`!has_visible_geometry` (native.rs:196); the head-anim branch unreachable
for the hair key (set membership of `head_parts` vs `head_anim_parts`);
`merge_actor_scene_attached` not creating/naming the node. Fix so the node
exists at runtime. Bump the relevant `*_REVISION`/fingerprint if output
changes.

### Acceptance (orchestrator, real data)
`prepare SuperDuperMart --actor-animation-converter native --rebuild-assets`
then `actorinspect 00041600` → `phase=Playing`, `bound_targets>0`, no
`HeadAnims` diagnostic; live hierarchy query shows a `HeadAnims` node.

## Issue #205 — binder must not fail the whole set on a missing facial target

### Feature list
1. An animation set whose idle clip requires a target absent from the
   assembled actor still reaches `Playing` and animates the targets that do
   resolve (body locomotion), instead of `Failed`.
2. Missing facial/head targets are reported as a non-fatal diagnostic, not a
   hard failure.

### Tests (feature-first)
- Unit test in `actor_animation` (bare `World`/minimal `App` harness per
  AGENTS.md testing section): a runtime whose required set includes an
  unbindable target reaches `Playing` and selects a body clip; assert it is
  not `Failed` and that `select_gameplay_animation` produces a walk clip.

### Implementation
In `src/viewer/actor_animation/mod.rs:595-614`, stop treating a non-empty
`missing_required_targets` as fatal. Reach `Playing` binding the resolved
targets; keep the missing set as a diagnostic. Lean on the existing
per-clip degradation at `mod.rs:736-769`. Do not regress the genuine
"no targets bound at all" failure (`mod.rs:561`).

### Acceptance (orchestrator, real data)
Even before #206, a SuperDuperMart raider driven with `actoranim <ref> walk`
plays `mtforward` limb motion (head static). After #206, both head node and
body animate.

## Manual acceptance
`docs/plans/M4_HEADANIMS_MANUAL.md` — written by the orchestrator before the
wave PR; drives both fixes on real SuperDuperMart data.

## Out of scope
FaceGen coefficient reconstruction (#109). Automatic gameplay AI that routes
actors without `tna` (separate backlog — nav binding is still debug-only).
