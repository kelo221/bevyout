# M3 wave 4 — console additem and pickup activation (#84)

Single-issue wave under epic #7, stacked on wave 3 (branch `m3-wave4` off
`m3-wave3`, PR based on the wave-3 branch) because pickup activation must
carry #81's steal classification. Executor: one Sonnet agent on the wave
branch; orchestrator reviews and evaluates per AGENTS.md model routing.

## Fixed feature list

- **F83.1 `additem`.** `player.additem <FormID> [count]` (default 1) adds
  to the authoritative `PlayerInventory` via `add_stack`, seeding
  condition from the catalog's `max_condition` exactly like the E-key
  pickup path; uncataloged ids add with condition `None`. Deterministic
  result line `additem <formid> x<count>; inventory now has <total>`,
  standard console errors for arity and non-positive counts.
- **F83.2 `activate` on pickups.** New `interaction::scripted_pickup`
  seam mirroring `activate_focused_placement`'s Pickup arm minus
  raycast/distance checks: inventory add with catalog condition,
  runtime-item save-state handling, pickup sound, notice, #81 steal
  classification/log, entity despawn. `activate <FormID>` then supports
  doors, containers, and pickups; the unsupported-semantic error message
  names all three.

## Order

1. Console-harness unit tests in `src/viewer/console.rs` (the existing
   `activate_*` test pattern) — Bevy-side behavior, no cucumber feature.
2. Implementation until green; gates; agent-bridge acceptance on a real
   cell (`player.additem f 100`, `activate` on a real pickup reference).

## Shipped amendments

(none yet — added only if acceptance testing forces changes)
