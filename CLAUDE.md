@AGENTS.md

Reminder (rule lives in AGENTS.md "Way of working"): every issue a wave
works on must be assigned to the human user — new issues at creation,
adopted issues at wave kickoff.

## Claude-runtime notes (orchestrating session)

- Executor subagents can die mid-run to transient API/network errors.
  Never restart from scratch: resume the same agent (its context is
  intact) with an instruction to reconcile `git status`/`git diff`
  against its own last step before re-applying anything. Both M4 wave 4
  executors survived one such crash each this way.
- Brief executors to commit in logical increments as features go green,
  not one giant commit at the end — a crash then loses minutes, not
  hours.
- Long gates/acceptance waits: use `run_in_background` plus an
  until-loop on the stable log line (`nav agent handoff …`), never
  chained sleeps.
- While an executor is running, do not commit on the wave branch (its
  in-progress files could be swept into the wrong commit) and do not
  edit files it owns; queue orchestrator doc edits until it reports.
