# M4 walking-actors wave — kickoff prompt (what was requested)

Verbatim intent as handed to the orchestrating session, master @ `62797f8`.

> M4 nav track is done and merged. Read `docs/postmortem/VERDICT.md` first —
> it's the post-mortem of the whole navmesh saga and its findings drive what's
> next.
>
> Next wave: #188 (P1) — bind nav agents to projected actors and drive
> animation from locomotion. Right now `agent.rs` has zero references to
> `ActorRuntime`: `tna spawn` routes without a body, `actoranim` animates
> without navigation, so nothing in the game walks. It's the prerequisite
> for #115.
>
> Wave shape: #188 → #186 sequentially (same actor/nav seam), #189 in parallel
> (prepare-side, disjoint files). Hard constraint from the post-mortem: the KCC
> stays the single movement authority — animation is display-only, root motion
> must never feed back into the agent transform.
>
> Also pending: split #115 into executable sub-issues (it's still an
> eight-bullet umbrella).
>
> Follow AGENTS.md waves — plan + manual script, orchestrator plans and
> reviews, executors write all code, real-data acceptance before the PR.
>
> #189 fixes a bug.

## Orchestrator notes on the request

Three corrections/refinements established during planning, none of which
change the requested shape:

1. **Wave name, not number.** The actor-animation track independently used
   "wave 10" and "wave 12"; the doors wave already set the precedent of naming
   rather than numbering past that collision. This wave is
   `M4_WALKING_ACTORS_*`, branch `m4-walking-actors`.

2. **Verdict §2.7 is stale.** It reports "~2,700 lines across six commits
   (#177, #184) with no wave plan or manual-script addendum". Both
   `docs/plans/M4_DOORS_WAVE_PLAN.md` and `M4_DOORS_WAVE_MANUAL.md` exist and
   are committed. The process was restored before that branch merged; verdict
   recommendation §4.8 is already discharged. No action carried into this wave.

3. **#189's file scope is not purely prepare-side.** Its items 1–3 are
   (`src/vsa/prepare/navmesh.rs`, `nav_doors.rs`), but item 4's primitive
   consolidation names `src/viewer/nav/landmass_graph.rs` and
   `src/vsa/prepare/nav_clearance.rs`. That is still disjoint from the
   #188/#186 lane (which owns `agent.rs`, `activation.rs`, `world_commands.rs`),
   so the parallel shape holds — but the ownership boundary is stated
   explicitly in the plan rather than assumed.

## Verified starting state

- `grep -rn "ActorRuntime" src/viewer/nav/` → **zero hits**. #188's premise
  confirmed: the two populations genuinely do not meet.
- `src/viewer/nav/agent.rs` is 9,389 lines (verdict §2.6).
- The `desired`/`achieved` pair #188 asks to reuse is live in
  `apply_agent_physics_movement` (`agent.rs:2679-2697`), feeding
  `movement_policy::decide_collision_outcome`.
- `request_actor_animation(world, entity, ActorAnimationState)` exists at
  `src/viewer/actor_animation/mod.rs:255`; `ActorAnimationState` at
  `policy.rs:9` is `Idle|Walk|Run|TurnLeft|TurnRight|Equip|Unequip`.
- The Activator gap in #186 is confirmed at
  `src/viewer/interaction/activation.rs:373-386`: sound, notice,
  `activated {name}` log and `ClipTransition::Opening` are all written, and
  `InteractionState.open` (`state.rs:54`) is never touched.
- #188, #186, #189, #115 are all OPEN and all assigned to `nippongun` — the
  AGENTS.md assignment invariant holds at kickoff with no re-assignment needed.
