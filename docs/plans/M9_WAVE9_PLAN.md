# M9 wave 9 plan — integer game clock and lifecycle

## Execution model recommendation

Roadmap recommendation: **Sol High** (Codex runtime) / **Opus** (Claude
runtime) — the wave expands a pure core clock plus thin viewer adapters
on existing effects, persist, lighting, and console seams. ZCode runtime:
the orchestrating session executes directly on `M9-Work`. Sequential
exception: `world_commands.rs`, persist, effects tick, and lighting
share files, so this wave does not use parallel worktrees.

## Fixed feature list

### Core

- Keep `GameTime` as the millisecond identifier used by restock/medical.
- `GameClockState` holds `absolute_game_ms`, `fractional_timescale_remainder`,
  and integer `timescale` (game seconds per real second).
- `TimeAdvanceReason`: Realtime, Wait, Sleep, FastTravel, Console.
- One `GameTimeAdvanced` interval per successful advance; overflow is
  rejected and leaves state unchanged.
- Calendar projection (versioned 30-day months, epoch 2277-10-23 00:00)
  and `hour_as_f32()` for lighting only.
- `LifecycleScheduler`: `BTreeMap<u64, Vec<LifecycleTask>>`. Same-timestamp
  order is kind then owner: effects, radiation/withdrawal, death,
  restock, cell reset, arrival.
- Cell reset refuses occupied cells, preserves unique/persistent/quest/
  player-owned holders, respawns eligible refs with the same id, and
  records a receipt so the due event cannot run twice.
- Encounter zones lock level on first entry.
- Fast travel: validate evidence → plan → commit (advance, tasks, move,
  request load, arrival). Each block is independent.

### Persistence / lighting

- Save remains **v9** / `RPG_SAVE_REVISION` 1.
- Optional RPGS `TIME` and `LIFE` subrecords, same skip-unknown pattern
  as `CRIM`.
- `src/viewer/day_night.rs` `GameClock.hour` is a projection. `settime`
  must not write f32 hours into the save authority.

### Viewer

- `GameTimePlugin` owns `GameTimeRuntime`. Realtime frames and `passtime`
  advance the same world.
- Effect/chem-dose ticks consume `GameTimeAdvanced` when the runtime
  exists; otherwise they keep the isolated-test frame fallback.
- Owned-bed / sleep restoration uses the advanced clock, not
  `GameTime::from_ms(0)`.
- Console stays at 20 providers / 150-line `console.rs` cap.
  `passtime`, `fasttravel`, `resetcell`, `showgametime` live in
  `world_commands.rs`.

## Tests-first order

1. `features/rpg_time.feature`.
2. Cucumber World fields after Wave 7 minigame fields; steps at EOF.
3. Core unit tests in `crates/bevyout-core/src/tests/time.rs` and
   `lifecycle.rs`.
4. Optional RPGS TIME/LIFE round-trip.
5. Viewer adapter, lighting projection, console commands.
6. Implement until green; gates; `M9_WAVE9_MANUAL.md`.

## Acceptance gates

- `71:59:59.999` does not restock/reset; `72:00:00.000` does.
- A large jump processes every crossed deadline in pinned order.
- Occupied cells do not reset; unique/player-owned holders survive.
- Fast travel is blocked independently by each precondition.
- Encounter-zone level is fixed at first entry and stable after
  save/load of the LIFE snapshot.
- Lighting hour does not feed back into authoritative milliseconds.
- `cargo fmt --check`, clippy `-D warnings`, tests.

## Shipped amendments

- **A1.** `settime` remains a lighting preview. Authoritative jumps use
  `passtime` / realtime / fast travel / sleep so f32 hours cannot become
  save authority.
- **A2.** Encounter-zone records are not decoded from ESM this wave;
  lock-on-first-entry is a core table keyed by zone FormID. ECZN wire
  decode is a follow-up.
- **A3.** Fast-travel destination load is a requested cell FormID, not a
  second travel pipeline. Viewer still uses existing cell-swap to honour
  the request.
- **A4.** Viewer integer timescale defaults to `0` so existing
  `settime`/`gettime` lighting tests and paused-by-default inspection
  stay frozen. Vanilla `30` remains the core `GameClockState` default
  and the cucumber/realtime remainder authority.
- **A5.** Effect ticks consume `GameTimeAdvanced` inside `LifecycleWorld`.
  The Bevy `ActiveEffectsList` ticker is a frame fallback only when
  `GameTimeRuntime` is absent, so one advance cannot tick twice.
