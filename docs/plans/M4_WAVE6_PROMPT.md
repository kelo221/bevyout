# M4 wave 6 — kickoff prompt

Requested 2026-07-18, following M4 wave 5 (PR #140; #114, #137, #138
shipped):

> 114 just landed. Work on the follow ups and the next best tasks. If
> possible, check out parallel issues too.

Wave composition chosen by the orchestrator:

- **#136** — corner clearance respects agent radius. The wave-5
  measurement (issue comment, 2026-07-17) upgraded this from cosmetic to
  hard blocker: the physics capsule wedges (`collision-blocked` →
  `stuck`) where the un-eroded navmesh routes it within < agent radius
  of colliders. Highest-value nav follow-up; blocks #115.
- **#123** — NOTE records carry no text in the prepared catalog (P1
  bug; Pip-Boy reader has no real note to show).
- **#120** — prepare real dead actors as lootable corpses (P1; the
  wave-7 corpse seam exists but real `CG04DeadOldLady` still classifies
  as `Npc`).
- **#121** — Pip-Boy Items: clicking a row triggers its primary action
  (P1 UX follow-up from M3 wave 6 acceptance).

All four touch disjoint code areas, so all four executors run in
parallel worktrees. The next big roadmap item, the #104 KF/controller
spike, is deliberately deferred to its own wave: a spike's output
shapes #105–#108 planning and deserves undivided orchestration.
