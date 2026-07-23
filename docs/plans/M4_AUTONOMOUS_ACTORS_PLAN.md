# M4 autonomous-actors wave — plan (#215, #218, #224, #225)

**Execution model recommendation:** **Sonnet** (Claude runtime), one executor,
**sequential** on the `m4-autonomous-actors` branch (all four pieces touch the
same nav/actor seam — AGENTS.md sequential-exception). Orchestrator (Opus)
evaluates. Do the features in the order below (small independent wins first, then
the spine), committing per feature as each goes green.

## Architecture (mapped at plan time — file:line seams)

- **Nav roster (#215):** `TestNavAgentState { entities: [Option<Entity>; 4] }`
  resource + `const MAX_TEST_AGENTS: usize = 4` (`src/viewer/nav/agent.rs:555,
  922`). All per-agent state already lives on the entity (`AgentKcc`,
  `AgentRuntime`, `TestNavAgentMarker`, `NavBoundActor`); the array only maps a
  debug index → entity and caps how many agents can exist. `bind_agent`
  (`agent.rs:2468`) rejects when the slot is occupied — this cap is the blocker.
- **Actor lifecycle (#218):** `project_prepared_actors` inserts `ActorRuntime`
  (`src/viewer/actor.rs:200, 39`); `seed_actor_states` inserts
  `ActorStateRuntime { life_state: ActorLifeState }` one system later via
  `Added<ActorRuntime>` (`src/viewer/actor_state.rs:114, 72`). The autonomous
  driver mirrors this with `Added<ActorStateRuntime>` (so life_state is present)
  and filters `ActorLifeState::Alive`.
- **Reusable package logic (in `src/viewer/ai/`, not console):**
  `selection::select_package`, `resolution::{resolve_location, resolve_target,
  linked_reference_chain}`, `families::{PackageFamily, select_interaction_point}`,
  `ActorPackageController::{start, start_follow, start_wander}`
  (`family_runtime.rs:80-131`). The per-tick driver `drive_actor_packages`
  (`family_runtime.rs:147`) and the nav release-hook teardown
  (`release_actor_package`) already exist — **only the *start* side is missing**.
- **Console-coupling to lift:** `build_resolution_context` (`ai_package_commands
  .rs:1159`) and `resolve_family_point` (`:808`) are private in the console
  module but pure w.r.t. world reads — move them to `ai/`. `start_package`
  (`:462`) returns `ConsoleError`/`ConsoleCommandResult` — the system version
  skips-and-logs (`warn!`) instead. Console `start_package` hardcodes
  `GameInstant::default()` (noon) — the system uses the live clock (see F2).
- **Locomotion flap (#224):** nav-bound classifier `next_locomotion_state`
  (`src/viewer/nav/locomotion.rs:177`) consumed by `drive_bound_actor_locomotion`
  (`src/viewer/nav/actor_binding.rs:228`, emits the `nav actor locomotion … ->`
  log at `:247`). It HAS full hysteresis bands but feeds them
  `kcc.last_achieved_horizontal_speed` raw (`actor_binding.rs:237`); `NavSolveRate`
  gating makes achieved speed swing ~0↔full across solve/non-solve ticks, so it
  crosses the run band every tick → the idle↔run flap observed live. Fix =
  smooth the achieved-speed input.
- **Converter default (#225):** `ActorAnimationConverter::Disabled` is
  `#[default]` (`src/cli.rs:101`); native builds clips in pure Rust
  (`run_native_actor_animation_batch`, `src/vsa/assets/actor_animation.rs:160`).

## Features (fix the list → tests → implement, in this order)

### F1 — #225: default actor-animation converter to `native`
- Change `ActorAnimationConverter`'s `#[default]` from `Disabled` to `Native`
  (`src/cli.rs`). Keep `blender`/`disabled` as explicit opt-ins. Verify a plain
  `cargo run-dev -- prepare --cell 00017f37` now reports non-zero `ready clips`.
- **Test:** a unit/CLI-contract test asserting the default backend resolves to
  Native. Confirm the existing converter-revision cache key still distinguishes
  native vs blender vs disabled (no stale-cache reuse across a default change).

### F2 — #224: stop locomotion flapping
- In the nav-bound locomotion path, smooth the speed fed to
  `next_locomotion_state`: add an exponential moving average (or a short
  min-dwell) of `last_achieved_horizontal_speed` on `NavBoundActor` before
  classification (`actor_binding.rs:234-241`), so `NavSolveRate` ripple no
  longer crosses the run/walk bands each tick. Do **not** remove the existing
  hysteresis bands (`locomotion.rs`) — this is a smoothing addition on top.
- **Unit test** (pure, `locomotion.rs`-adjacent, std-only so it can be driven
  from `tests/features.rs`): feed an alternating high/near-zero speed sequence
  (simulating solve/non-solve ticks) and assert the classified state does NOT
  change every step — it settles to a single state. A regression test that
  fails on the raw-instantaneous classifier.

### F3 — #215: ECS agent roster (remove the 4-slot cap)
- Replace `TestNavAgentState`'s `[Option<Entity>; MAX_TEST_AGENTS]` with a
  growable structure (e.g. `Vec<Entity>` with dynamic index assignment, or drive
  purely off the `TestNavAgentMarker` component and keep a `Vec` only for the
  `tna` debug index API). Remove `MAX_TEST_AGENTS`. The `tna
  spawn/bind/goto/status/despawn [<index>]` console API must keep working
  (dynamic indices), but binding must no longer fail when >4 agents exist.
- Extract a **non-console** core bind entry the autonomous driver (F4) calls —
  e.g. `bind_agent_entity(world, entity) -> Result<()>` that inserts the
  `agent_components(...)` bundle + `NavBoundActor` for an already-projected actor
  entity, without the index/`ConsoleCommandResult` layer. `bind_agent` (console)
  becomes a thin wrapper that also records the debug index.
- Keep the release contract intact (`release_bound_actor`, the registered hooks).
- **Tests:** bind 5+ agents without rejection (the old cap would fail at 5); a
  released agent frees its debug index; the existing `tna` scenarios still pass.

### F4 — #218: autonomous package driver
- Move `build_resolution_context` and `resolve_family_point` from
  `ai_package_commands.rs` into `src/viewer/ai/` (mechanical; they only read
  world state). The console command calls the moved versions (no behavior
  change) so there is one implementation.
- Add a gameplay system in `src/viewer/ai/` (new module, e.g. `autonomous.rs`,
  registered by `AiPackagePlugin`) that:
  - queries `Query<(Entity, &ActorRuntime, &ActorStateRuntime), Added<
    ActorStateRuntime>>`, filters `life_state == Alive`;
  - for each, calls the F3 core bind, then runs the same select→resolve→start
    logic as `start_package` (select_package with the **live game clock**, not
    hardcoded noon; resolve; `ActorPackageController::{start|start_follow|
    start_wander}`), replacing every `ConsoleError` early-return with
    `warn!(...)` + skip-and-continue (a system cannot fail a user);
  - is **idempotent**: skip an actor already nav-bound or already carrying an
    `ActorPackageController`, so console `tna`/`runpackage` still work alongside
    it and re-entry never double-binds.
- **Live clock:** find the game-time resource (grep `GameInstant`, game hour,
  `Time<Virtual>` / any world clock); use it for selection. If no gameplay clock
  resource exists yet, keep noon but leave a `ponytail:`-style comment naming the
  follow-up — do not invent a clock system in this wave.
- **Enablement:** on by default (this is the deliverable), but the system must be
  a no-op when there are no alive actors, and must not fight the console. Add a
  simple resource toggle (default on) so tests and future headless flows can
  disable it.
- **Tests** (minimal-`App`/bare-`World`, mirroring `family_runtime.rs` tests):
  spawning an alive actor with a resolvable package auto-binds an agent and
  attaches an `ActorPackageController`; a corpse/dead actor does not; an actor
  already bound by the console is not double-bound; the moved
  `build_resolution_context` still resolves a type-6 patrol.

### F5 — cucumber + manual
- `features/autonomous_actors.feature`: (a) an alive actor with a Patrol package,
  once its lifecycle state is seeded, is selected for auto-bind + package start;
  (b) the locomotion classifier does not flap under alternating speed input.
  Steps appended to the `tests/features.rs` merge seam (World fields at end of
  struct, delimited step block at end of file); `fail_on_skipped()`.
- `docs/plans/M4_AUTONOMOUS_ACTORS_MANUAL.md`: plain-language summary, then
  numbered steps — one-time `prepare --cell 00017f37` (now native clips by
  default, no flag), launch the viewer, and **without any `tna`/`runpackage`
  commands** observe raiders patrolling with animation; use `runpackage <ref>
  status` only to read `marker=i/n`, and grep the log for `actor-animation play
  state=run` and the absence of sub-100 ms `nav actor locomotion` flapping. Real
  FormIDs from the prepared catalog (raiders `00041600`/`00041604`/… confirmed
  during #213 acceptance).

## Gates
`cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test`;
plus real-data `cargo run-dev -- prepare --cell 00017f37` (native clips) and a
viewer acceptance run (the manual). Commit per feature.

## Out of scope (do not expand)
Combat/perception behavior (#11 big-brain, deferred to M5); schedule/time-of-day
gameplay clock beyond wiring what exists; persistence of the roster/driver state
(#217); splitting `agent.rs` (#190 — deferred by explicit decision, work in the
big file). The package radius/`FO3_SCALE` scaling (#222) is a separate follow-up.

## Shipped amendments
_(to be filled during acceptance)_
