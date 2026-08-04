# Navigation slice

The navigation viewer is composed as a vertical slice under `src/viewer/nav`.
`nav/mod.rs` is the public composition boundary; prepared graph decoding lives
in `input.rs`, the runtime schedule in `plugin.rs`, and the external callers'
contract in `api.rs`.

## Ownership

- `agent/` owns the `NavAgent` component set, lifecycle, routing, locomotion,
  KCC movement, actor binding, and the pure fall policy.
- `world/` owns the active archipelago state and its build, resident exterior,
  portal, link, and player-obstacle projections.
- `doors/` owns access policy, availability diffs, door-link runtime state,
  travel requests, and door traversal effects.
- `traversal/` owns same-cell merge traversal and the Landmass input-refresh
  synchronization required by that traversal.
- `handoff/` owns cell-transition detection and the persistent agent ledger.
- `debug/` is the console-facing `tna` adapter and debug-agent roster.
- `diagnostics/` owns stable state logs and HUD projection formatting.

AI, world interaction, and non-debug runtime callers use `api.rs`. Console
commands are the only callers that need the debug roster or `ConsoleError`.
The active `NavArchipelagoState` is the single authority for runtime links,
door availability, resident exterior identity, and the current archipelago;
the other modules project or mutate it through their named boundaries.

## Runtime ordering

`NavRuntimeSet` preserves the fixed-step order: world lifecycle, ledger restore,
door availability, door-link lifecycle, physics movement, fall guard, actor
presentation, same-cell traversal, and diagnostics. The Landmass input/output
systems retain their explicit pre-update ordering around link-marker refresh,
cost restoration, velocity blending, and deferred merge repathing.

## Test seams

Pure policies remain directly includable by feature tests. Bevy runtime tests
are split by capability in `src/viewer/nav/tests/`; `tests/support.rs` contains
shared fixtures while lifecycle, movement, traversal, doors, handoff, debug,
world, diagnostics, and prepared-cell replay cases stay in separate modules.
