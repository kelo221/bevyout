# M4 HeadAnims animation-fix wave — kickoff prompt

## What was requested

Bug report (human, real data): actors such as Raiders in **SuperDuperMart**
do not play any animation while walking. It was previously claimed the
"walking actors animate" behaviour was already implemented (wave 12 #187,
walking-actors #202), but on real data it does not hold.

## What the orchestrator found (pre-wave investigation)

Reproduced live against `.bevyout/cache/scenes/00017f37/scene.ron`:

- **All 6** actor animation sets fail at bind time with
  `prepared actor is missing required animation targets: HeadAnims:0`;
  every actor is `phase=Failed`, so **nothing animates — walk or idle.**
- It is **not** a missing-facegen / missing-head problem. The prepared
  raiders have heads, teeth, eyes, and hair (`hairraiderf02.nif` etc.) all
  present in the assembly descriptor. FaceGen coefficient reconstruction is
  correctly deferred (#109) and is not the cause.
- A live BRP hierarchy query shows **no `HeadAnims` node** in the spawned
  actor (only `Bip01 Head Dome`), and no hair mesh — even though the
  descriptor lists the hair as a `head_anim_part` to be attached under a
  `HeadAnims` node (`native.rs:246-251`). The failing cache was built
  2026-07-21, after the HeadAnims wiring landed (wave 7 #160, 2026-07-19),
  so this is a live merge bug, not a pre-feature stale cache.

Two independent defects, two issues:

- **#206** — assembled actors have no `HeadAnims` node at runtime (the hair
  head-anim attachment does not survive to the spawned scene).
- **#205** — the animation binder fails the *entire* set (including leg
  locomotion) when *any* required target is missing, so one absent facial
  node kills body animation.

## Scope

Fix both. #206 restores the missing head node; #205 makes the binder robust
so a missing facial target can never again silently kill body locomotion.
FaceGen coefficient reconstruction (#109) stays out of scope.
