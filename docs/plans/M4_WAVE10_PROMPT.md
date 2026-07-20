# M4 wave 10 — kickoff prompt

Verbatim request (after PR #170 merged and the nav progress review):

> Go ahead, rescope 153 and start the wave with 162,168,169 is possible to
> do parallely. 153 is the last big item, I assume. Rephrase and work on
> it, too

Orchestrator decisions:

- #153 retitled/rescoped (comment on the issue, 2026-07-20): collision-
  derived navmesh validation + clearance rebuild replacing the interim
  erosion, per #148/#164's wave-9 investigation verdict. #148's route is
  its acceptance gate and closes with it.
- Wave 10 = #153 (big lane) parallel with #162 + #168 + #169 (small lane).
  The three small items all touch the `agent.rs`/`landmass_graph.rs`
  runtime seam, so they run as ONE executor sequentially in a worktree
  while #153 works prepare-side on the wave branch — disjoint seams,
  AGENTS.md parallel-worktree rule with `tests/features.rs` as the only
  shared merge seam.
