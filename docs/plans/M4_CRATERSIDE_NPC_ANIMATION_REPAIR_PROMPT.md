# M4 Craterside NPC animation repair — kickoff prompt

Repair the static-NPC failure in the prepared `MegatonCratersideSupply`
(`00003a2a`) scene without inventing a second animation pipeline or weakening
Fallout-authored package/idle behavior. The approved execution plan is
`M4_CRATERSIDE_NPC_ANIMATION_REPAIR_PLAN.md`; it is the fixed source of truth
for scope, ownership, tests-first ordering, revision bumps, merge order, and
acceptance.

The wave is split into four assigned issues and four isolated executor lanes:

- **Lane A — render/cache readiness:** thread `RenderArgs`' native/disabled
  actor-animation converter through render preparation and make compatible
  animationless caches repairable, with deterministic readiness diagnostics.
- **Lane B — package repair:** accept real 4/8/12-byte `PKDT`, use positive
  `PSDT` durations as hours, preserve package idle collections, and retain
  package-linked editor-marker points without rendering them.
- **Lane C — authored IDLE preparation:** after Lane B, decode winning IDLE
  records, authored parent/sibling order, CTDA/DATA/group metadata, and build
  deterministic prepared definitions in the existing actor-animation catalog.
- **Lane D — idle runtime/console:** after Lane C, add pure lifecycle/condition/
  cooldown/collection selection, Special Idle and Whole Body playback, and the
  additive `playidle`/`actorinspect` surface.

Use the merge order A+B concurrently, then C, then D. Production-file
ownership is exclusive between concurrent lanes; the orchestrator resolves
only the append-only `tests/features.rs` seam and any explicitly reported
cross-lane seam. Every executor writes tests before implementation, runs its
focused gates, commits its work, and reports exact evidence. The orchestrator
reviews every diff, audits all serialized revision bumps, runs the full gates,
performs real-data acceptance, writes the manual, and opens one PR to
`master`.

The visible gate is native clip-pack repair and playback for Mercenary
`0001ff18` and Moira Brown `0002d2bc`: base idle while stationary, Walk/Run
while navigating, authored package selection at the specified hours, authored
IDLE preparation and forced `playidle` verification, with a ready second render
performing no unnecessary prepare or bake.
