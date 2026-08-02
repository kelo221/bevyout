# M6 wave 2 — streaming reliability and measured process memory

## Kickoff

PR #261 merged the first exterior preparation and streaming foundation but left
the route gate's lifecycle and memory evidence unproven. This wave closes that
measurement/reliability gap on a short, fixed exterior route. It does not claim
gate #87, actor integration, travel, environment, or final-route acceptance.

The two child issues are intentionally disjoint:

- **W2-A — measured exterior process memory:** own the measurement adapter and
  the `worldstream summary`/`worldstream trace` report surface. Keep process
  memory separate from prepared-package byte estimates, and report an explicit
  unsupported/unmeasured result rather than fabricating a value.
- **W2-B — lifecycle reversal and cancellation:** own the pure residency
  planner/runtime lifecycle seam. Prove generation-safe reversal, stale task
  rejection, unique roots, collision-ledger teardown, and zero cell-owned
  state after eviction.

The integrator owns shared `tests/features.rs` steps, shared console/plugin
wiring, this wave's manual, conflict resolution, repository gates, and real
route evidence.

## Required executor model

Use **GPT-5.6 Luna with Max reasoning** for both focused executor tasks in this
Codex run, as explicitly requested by the human. The roadmap's generic Codex
recommendation is Sol X-High; Luna Max is the active execution override here.

## Non-goals

- Do not redesign the exterior residency contract or add a second lifecycle
  authority.
- Do not turn package serialization estimates into process-memory readings.
- Do not integrate gameplay actors, travel/save, environment, LOD, occlusion,
  or final-route budgets.
- Do not close #13, #87, or #14 from this wave.
