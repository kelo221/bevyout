# M6 wave 4 kickoff prompt

Continue the M6 exterior roadmap on branch `M6-OutCell` after the Wave 3
policy lanes landed.

## Current state

- Wave 2 streaming policy and diagnostics are integrated, but the bounded
  two-way real-data route is still gated by the current prepared-data
  fingerprint and must not be claimed from the stale local route cache.
- Wave 3 actor-residency and lifecycle-safe NAVM policy are integrated as
  #276/#277. Gameplay actor integration #278 remains correctly blocked until
  M4 gate #10 closes.
- The continuation roadmap explicitly permits pure policy and contract work
  ahead of that runtime dependency.

## Requested wave

Land the two independent Wave 4 policy lanes:

1. **W4-A / Canonicalize travel anchors and save location**
   Establish one explicit contract for exterior/interior identity, authored
   destination transforms, and exact player-location persistence. Reuse the
   existing `WorldLocation`, prepared door destinations, `SaveGame.location`,
   and `CurrentWorldLocation` authorities; do not create a second runtime
   location or return-anchor store.
2. **W4-B / Complete water, breath, and fall policy**
   Make water contact, swimming/breath consequences, and landing/fall
   thresholds deterministic and frame-rate independent in pure policy code.
   Reuse the existing core water-contact resolver and OpenMW-derived
   locomotion state; do not integrate the runtime movement seam in this wave.

The two lanes are disjoint and may execute in parallel. W4-C is intentionally
deferred: it will own `world::swap`, streamed persistence integration,
activation, the exterior lifecycle adapter, and `player::movement` after the
travel/policy contracts are reviewed.

## Required delivery

- Read the repository `AGENTS.md`, the continuation roadmap, this prompt, and
  the W4 plan before changing files.
- Follow tests-first ordering: add or update focused tests before production
  implementation.
- Keep pure decisions dependency-light and retain the existing authority
  boundaries. Do not guess a replacement actor, save, water, or movement
  authority.
- Do not touch `tests/features.rs`, W4-C runtime files, unrelated formatter
  drift, generated Bethesda-derived data, or the other W4 lane.
- Commit the lane's implementation and focused tests on its worktree branch.
- Report the commit, exact tests, and any unresolved integration dependency.

Execution model recommendation: Codex runtime — **GPT-5.6 Luna, Max
reasoning**.
