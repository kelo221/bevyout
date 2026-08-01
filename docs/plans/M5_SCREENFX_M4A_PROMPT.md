# M5 screen feedback + M4A gate-support wave — kickoff prompt

Requested on 2026-07-31: work on the two actionable blockers identified for
M5 — #96 (ImageSpace/IMAD runtime and first-person screen feedback) and #86
(the M4A representative-actor gate).

This is a gate-support wave, not combat-architecture Wave 3 from
`M5_COMBAT_ARCHITECTURE_ROADMAP.md`. The combat roadmap's Wave 3 remains the
later condition/degradation/jam/RNG slice.

## Scope

- #96 owns the missing production path: Fallout 3 IMAD preparation, a pure
  transient screen-FX policy, and a thin Bevy presentation boundary.
- #86 owns real-data evidence for the actor runtime already merged to
  `master`: one representative humanoid and one representative creature,
  their animation/equipment/lifecycle behavior, and measured frame, memory,
  load, and cache evidence.

The two workstreams may proceed independently. #96 is not blocked by #86.
The results converge at M5A gate #88, which needs both screen feedback and the
M4A actor gate.

## Starting point

- Branch: `m5-gates-wave1`, based on `origin/master` at the wave kickoff.
- M5 Wave 1 and Wave 2 are already merged.
- Existing IMGS parsing, prepared base ImageSpace resolution, camera color
  grading, bloom, and auto-exposure are preserved.
- Existing actor assembly, animation, equipment, lifecycle, persistence, and
  autonomous package behavior are treated as the implementation under test;
  no new actor gameplay scope is assumed for #86.

## Required working rules

- Keep gameplay authority outside presentation. Combat/effects emit typed
  requests; screen FX never decides damage, health, or status.
- Keep decision and combination rules Bevy-free and suitable for Cucumber
  inclusion. Bevy systems only adapt requests to camera/post-process state.
- Bump every prepared/manifest revision whose serialized shape or meaning
  changes, including serde-defaulted fields.
- Use synthetic IMAD fixtures for parser and policy tests. Never commit
  Bethesda-derived RON, GLB, DDS, WAV, NIF, or other generated content.
- Record runtime evidence with the agent bridge and bounded performance
  probes. Do not treat a static code match or one favorable frame average as
  a measured bottleneck or a passing budget.
