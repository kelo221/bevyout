# M9 wave 7 plan — lockpicking and terminal hacking

## Execution model recommendation

Roadmap recommendation: **Sol High** (Codex runtime) / **Opus** (Claude
runtime) — the wave adds a pure core state machine plus thin viewer
adapters on existing modal, persist, and console seams. ZCode runtime:
the orchestrating session executes directly on `M9-Work`. Sequential
exception: activation, persist, and `world_commands.rs` share files, so
this wave does not use parallel worktrees.

## Fixed feature list

### Core

- `minigames.rs`: `PickAngleMilliDegrees` (-90_000..=90_000),
  `CylinderAngleMilliDegrees` (0..=90_000), `PinStress`.
- `LockpickInput` / `HackingInput` with sequence numbers and terminal
  phases. A step mutates session + inventory together or leaves both
  unchanged.
- Force chance: `skill*100 - difficulty*50 + 500` clamped to 10_000 bps.
- In-spot torque: cylinder `+= delta_ms * 90`, unlock at 90_000 milli.
- Miss torque: `gain = (delta * delta_ms) / 2000`; pin break at 10_000
  via `ItemLedger::use_item`.
- Hacking: only board words; brackets once; fourth failed word locks
  out. Tests use synthetic words only.
- Owned lock success reports `CrimeKind::Trespass` through Wave 6
  `resolve_crime` (bounty 40 / karma -5).
- `MinigameRngState` wraps `RpgRngState` with lockpick/hacking salts.
  Combat RNG is unused.
- `saving_blocked` while either session is active.

### Persistence / modal

- Save remains **v9**. Active sessions are not serialized.
- `write_save_slot` fails with a deterministic minigame-deferred error
  while Lockpicking/Hacking is up or a session is active.
- Picked locks persist as `PersistentReferenceDelta.lock_level`
  (`Some(0)` means unlocked override).
- Cell swap/unload cancels the session.
- `GameplayModal` legal transitions: None ↔ Lockpicking/Hacking only.

### Viewer

- `MinigamesPlugin` owns `MinigameRuntime`. Activation starts lockpick
  from `ContainerActivation` resources (no `&mut World` from `Commands`).
- Console stays at 20 providers / 150-line `console.rs` cap.
  `lockpick`, `unlock`, `hackterminal` live in `world_commands.rs`.
- Hacking boards are synthetic (`VENT DOOR LOCK SAFE KEYS`).

## Tests-first order

1. `features/rpg_minigames.feature`.
2. Cucumber World fields after `rpg_hud`; steps at EOF.
3. Core unit tests in `crates/bevyout-core/src/tests/minigames.rs`.
4. Persist lock capture/apply + save-block.
5. Console adapters and viewer unit tests.
6. Implement until green; gates; `M9_WAVE7_MANUAL.md`.

## Acceptance gates

- Identical initial session + ordered input → same snapshot, pin count,
  lock/terminal mutation, and PRNG draw index without Bevy.
- Out-of-range pick angles rejected; RNG index unchanged.
- Pin break consumes exactly one canonical bobby pin (`0x0000000A`).
- Cancellation does not unlock or consume a pin.
- Saving is unavailable during an active session; lock override
  survives save/load after success.
- `cargo fmt --check`, clippy `-D warnings`, tests.

## Shipped amendments

- **A1.** Activation cannot obtain `&mut World` from `Commands`;
  lockpick start is a resource helper on `MinigameRuntime`.
- **A2.** TERM/ACTI still have no prepared password bank; hacking
  console uses a synthetic board. Real TERM decode is a follow-up.
- **A3.** Unlocked-from-locked persist uses `Some(0)` because
  `Option<i8>` cannot distinguish “no override” from “picked open”.
- **A4.** `unlock` is GECK-parity `setlock <ref> 0`, not a second lock
  authority.
