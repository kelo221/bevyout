# M4 wave 11 — kickoff prompt

Verbatim request (after PR #173 merged and housekeeping):

> https://github.com/kelo221/bevyout/issues/9
> I need you to continue and eventually finish the navmesh saga
> Q: https://github.com/kelo221/bevyout/issues/148 is the solution to this a
> hollistic fix or a hacky local solution since the latter is entirely forbidden
> Pick the last Follow ups
> And then also start working on 115

Orchestrator decisions:

- **Answer to the Q (binding constraint on this wave):** #171's fix is
  holistic by construction and must stay that way. It is a general
  geometric operation — clip every nav polygon against every static
  collider footprint expanded by the agent radius, and against the
  support/void boundary — applied uniformly to every cell. Nothing in it
  keys on `MetHallEntrance01`, RefID 370250, cell 0001a273, or any
  coordinate. #148 is the *acceptance case* that proves the general rule,
  not the target of a special case. Any placement-specific branch,
  coordinate threshold tuned to a test cell, or steering workaround that
  shepherds agents around an unidentified obstruction is a wave failure —
  #148's own amendment already forbids the last one. The executor brief
  states this explicitly and the orchestrator checks the diff for it.
- **Last nav follow-ups** = #171 (with #148 as its acceptance gate) and
  #172. Landing both closes the navmesh saga: the nav track then has no
  open issues.
- **#115 is started, not finished.** It was an unscoped umbrella (prepare
  + runtime + persistence for seven package families). Split: #175
  (prepared package catalog) and #176 (`showpackages` console surface)
  ship this wave as the data foundation; #115 itself keeps the runtime
  behavior layer for the next wave. Splitting rather than half-building
  the runtime keeps each slice acceptance-testable.
