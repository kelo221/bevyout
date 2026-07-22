# M4 HeadAnims wave — manual acceptance

## What this wave shipped

A **finding**, not a behaviour change: the reported "Raiders don't animate
while walking in SuperDuperMart" was a **stale actor-GLB cache**, not a code
bug. It self-heals on any re-prepare. This wave adds regression tests that
lock the actor HeadAnims hair-attachment contract so an assembly that drops
the head node fails CI instead of silently shipping frozen actors.

There is no new runtime surface to click. Acceptance is: (1) the tests exist
and pass, and (2) a fresh prepare makes the actors animate on real data.

## Steps

1. Gates (from repo root):

   ```sh
   cargo fmt --check
   cargo clippy --all-targets -- -D warnings
   cargo test
   ```

   Expect green, including the two new tests in `src/vsa/prepare/native.rs`:
   `head_anim_parts_attach_to_headanims_others_to_bip01_head` and
   `hair_merged_with_headanims_is_parented_under_a_headanims_node`.

2. Refresh the SuperDuperMart cache (this is the actual fix for anyone who hit
   the stale-cache symptom):

   ```sh
   cargo run-dev -- --config .bevyout/config.toml prepare SuperDuperMart \
     --converter native --actor-animation-converter native --rebuild-assets
   ```

   Expect an `actor animation catalog: … ready clips …` summary with no failed
   native worker.

3. Launch and inspect a raider:

   ```sh
   cargo run-dev -- view \
     --manifest .bevyout/cache/scenes/00017f37/scene.ron \
     --agent-bridge --agent-port 15702
   ```

   In the console (backquote): `actorinspect 00041600`.
   Expect `animation.present=true`, `bound_targets=67`, `clip=mtidle__2`,
   and **no** animation `diagnostic` (the `reasons=missing_facegen` note is the
   deferred #109 facegen item, not an animation failure). The startup log
   shows six `actor-animation ready … targets=67` lines and zero
   `reason=missing-required-targets`.

4. Drive the walk clip: `actoranim 00041600 walk` → limbs move
   (`mtforward__2`), feet stay on the floor, root does not drift.

## Note

Nav-driven walking is still debug-only (`tna bind` / `tna goto`); there is no
gameplay AI that routes actors unprompted. Animation is driven from achieved
motion once an actor is routed — see the walking-actors wave manual.
