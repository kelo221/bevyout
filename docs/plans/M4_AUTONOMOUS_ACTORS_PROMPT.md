# M4 autonomous-actors wave — kickoff prompt

**Issues:** #215, #218, #224, #225. Epic #9.

## What was requested (verbatim intent from the human)

> "The point of M4 is that the actors have patrols, animations, colliders, and
> all of the necessary things we already implemented. This needs to be bound
> automatically as this is a game."

Today, making an NPC patrol requires manual console commands (`tna bind` +
`runpackage`) per actor, the nav roster is capped at 4 agents, driven actors
twitch instead of walking smoothly, and animation clips are off unless prepare
is run with an explicit converter flag. The result is that launching a cell
shows raiders standing still. This wave makes it work like a game: **load the
cell → every alive actor binds a nav agent, selects and runs its package, and
walks its route with locomotion animation — no console commands.**

## The four pieces

- **#215** — replace the fixed 4-slot nav agent roster with a real ECS
  collection (no cap), so every actor in a cell can be an agent at once.
- **#218** — a gameplay system that, on cell load, binds an agent + starts the
  selected package for each alive actor, retiring `tna bind`/`runpackage` as the
  *only* entry point (they remain as debug tools).
- **#224** — fix the locomotion state flapping (idle↔run ~25×/sec) so
  auto-driven actors animate smoothly.
- **#225** — default the actor-animation converter to `native` (pure Rust, no
  Blender) so clips are built and animation is on out of the box.

## Acceptance

Launch SuperDuperMart (`00017f37`) with **no** `tna`/`runpackage` commands:
multiple raiders bind agents automatically, select their Patrol package, walk
their linked-reference marker routes, and play walk/run/idle locomotion clips
without twitching. `runpackage <ref> status` (debug) confirms `marker=i/n`
advancing; the viewer log shows `actor-animation play state=run clip=…` and no
sub-100 ms `nav actor locomotion` flapping.

## Runtime / model

Claude runtime. All four pieces touch the same runtime seam (`src/viewer/nav/
agent.rs` + `src/viewer/ai/`), so per the AGENTS.md sequential-exception they
run **sequentially on the `m4-autonomous-actors` branch**, one Sonnet executor,
committing per feature. Orchestrator (Opus) plans, reviews, runs gates + real-
data acceptance, opens the PR.

## Branch note

Branched off `m4-package-points` (unmerged PR #223) so #218 has #213's
resolution/chain-walk code. When #223 merges to master, rebase this branch onto
master before the PR.
