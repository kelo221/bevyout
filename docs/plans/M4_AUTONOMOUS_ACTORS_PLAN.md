# M4 autonomous-actors wave — plan (#215, #218, #224, #225)

**Execution model recommendation:** **Sol High** in the Codex runtime; **Sonnet**
in the Claude runtime, one executor **sequential** on the
`m4-autonomous-actors` branch (all four pieces touch the same nav/actor seam —
AGENTS.md sequential-exception). In the Claude runtime the Opus orchestrator
evaluates; in Codex the orchestrating session executes directly. Do the features
in the order below (small independent wins first, then the spine), committing per
feature as each goes green.

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

- **PR #228 repair on current master:** the first #224 implementation averaged
  scalar speed, which cannot cancel equal-and-opposite collision jitter and can
  latch an actor into Run while stationary. The repaired implementation averages
  signed horizontal velocity and classifies its magnitude. Regression tests pin
  both net-zero jitter (Idle) and sustained travel (Run).
- **Catalog startup cost:** autonomous startup originally deserialized
  `actors.ron` and the content-wide `packages.ron` once per actor inside an
  exclusive system. The AI-owned cache now keys actor data by manifest
  path/hash/revision and package data by content fingerprint; a multi-actor
  regression requires one disk load of each. Failed package startup also rolls
  back its newly-created nav bind rather than leaving an orphan agent.
- **Clock and ordering:** current master now has `day_night::GameClock`, so the
  autonomous selector consumes its live hour (with noon only as the resource-
  absent fallback). Its queue is explicitly ordered after actor-state seeding.
- **Agent-set coverage and input safety:** gameplay nav systems now enumerate the
  agent component set, not only console roster slots, so autonomous actors retain
  door/fall/telemetry behavior. Growable dense debug indices are validated before
  allocation/restoration; inputs above the defensive 65,535 ceiling are rejected
  instead of resizing toward an arbitrary `usize`.

- **F4 reuse shape differs from the literal plan text, on purpose.**
  `build_resolution_context` and `resolve_family_point` moved exactly as
  specified (into `ai::family_runtime`/`ai::resolution` respectively —
  `resolve_family_point`'s signature changed from taking a whole
  `PreparedPackageEntry` to plain `PackageLocation`/`PackageTarget` mirrors,
  since `ai::resolution` must stay decoupled from `vsa` types to keep
  compiling verbatim into `tests/features.rs`). But rather than *also*
  duplicating `start_package`'s ~150-line select→resolve→dispatch body a
  second time inside the new `ai::autonomous` module ("the same logic … as
  start_package"), the executor made `start_package` itself `pub(crate)`,
  added an explicit `instant: GameInstant` parameter (the console call site
  passes `GameInstant::default()`, unchanged), and had the autonomous driver
  call that exact function. This is strictly stronger than "the same logic
  in two places" — it is *one* function, zero duplication, zero drift risk
  — at the cost of `viewer::console::ai_package_commands` needing
  `pub(crate)` visibility so `viewer::ai::autonomous` can reach it (no
  architecture test enforces a console→ai-only direction; `tests/
  architecture.rs` only asserts bevyout-core/Bevy and vsa/viewer
  boundaries).
- **Live clock at the original wave head:** no gameplay clock resource existed,
  so the first implementation used deterministic noon. Current master gained
  `day_night::GameClock`; the PR repair now reads its live hour and retains noon
  only for minimal/headless worlds where that resource is absent.
- **`Added<ActorStateRuntime>` needed two systems, not one.** An ad-hoc
  `World::query_filtered` built fresh inside an exclusive system (the shape
  every other exclusive system in `nav/agent.rs` uses) does not track
  `Added<>` correctly across frames — only a `Query` system parameter Bevy
  caches does. `ai::autonomous` splits into an ordinary system with a real
  `Query<..., Added<..>>` that queues candidates into a resource, and a
  separate exclusive system that drains the queue and does the `&mut World`
  mutation (bind + start).
- **Real-data acceptance (SuperDuperMart, 00017f37), no console commands
  before observing:** all five raiders (`00041600`, `00041604`, `00041606`,
  `0004160c`, `00041611`) auto-bound and started their Patrol package within
  the first frame after cell load (`autonomous package driver: bound +
  started actor <formid>` for each); a sixth actor (`0005cf10`) logged a
  `warn` skip for a genuine per-actor data gap (no authored `XLKR`), which is
  correct behavior, not a bug. `actor-animation play state=run/walk/
  turn_left/turn_right clip=...` all logged (native clips play). `nav actor
  locomotion` transitions for the same agent landed seconds apart, never a
  sub-100 ms burst — the #224 flap is gone. `getpos` sampled twice on
  `00041600` a few seconds apart moved (`(18.55,96.45,-89.52)` →
  `(18.23,96.45,-89.32)`) with zero console movement commands issued. Full
  steps and evidence: `docs/plans/M4_AUTONOMOUS_ACTORS_MANUAL.md`.
- **New follow-up found, not fixed here (out of scope):** a brief (~300 ms)
  `turn_left`/`turn_right` oscillation was observed on one raider mid-route.
  This is the yaw-rate turn classifier, which #224/`smooth_achieved_speed`
  never touched (that fix is achieved-*speed* only, by design). It
  self-resolved within a third of a second and did not recur elsewhere in a
  ~50-second run. Recommend filing as its own issue if it proves visually
  noticeable in play (candidate fix: the same EMA treatment applied to yaw
  rate).
