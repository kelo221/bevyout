# Architecture wave 2 — typed viewer composition (#144)

Wave under epic #142 on branch `Refactor`; it builds on architecture wave 1.

## Fixed feature list

1. Define a documented `ViewerSet` for shared runtime phases where ordering is
   a cross-feature contract.
2. Convert mature top-level viewer subsystems from public `install` functions
   to typed `Plugin` values. Configuration lives in plugin fields.
3. Add one typed `ViewerPlugins` composition boundary used by `viewer::app`.
4. Keep nested private helpers where they remain useful; the goal is an
   explicit composition API, not mechanical plugin proliferation.
5. Preserve optional agent-bridge installation and all existing run
   conditions/schedules.

## Tests before implementation

- Minimal-App test proving the plugin group installs its shared resources and
  messages exactly once.
- Plugin configuration test for physics and resident-cell-limit values.
- Existing app-state, schedule snapshot, viewer, and subsystem tests remain
  unchanged and green.

## Gate

Focused viewer tests, then the full Rust gate from wave 1.

## Shipped amendments

- The shared phase chain is deliberately limited to `Input -> Interaction ->
  WorldSync -> Ui`. Existing subsystem-local `.before`/`.after` constraints
  remain authoritative inside each phase; unrelated systems were not forced
  into a global serial schedule.
