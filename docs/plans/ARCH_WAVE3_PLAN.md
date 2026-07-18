# Architecture wave 3 — interaction capability modules (#145)

Wave under epic #142 on branch `Refactor`; it builds on waves 1 and 2.

## Fixed feature list

1. Turn `viewer::interaction` into a curated module root/plugin boundary.
2. Separate shared components/messages/resources, focus/raycast policy,
   activation dispatch, door lifecycle, pickup/equipment, and container
   runtime/UI into cohesive modules.
3. Keep `ItemLedger` authoritative and preserve the persistence seams used by
   `viewer::world::persist`.
4. Keep audio and animation requests typed; do not introduce a second event or
   state authority while moving code.

## Tests before implementation

- Existing focus/probe, lock/key, door travel, pickup, equip, container
  transfer, corpse, and persistence tests define behavior.
- Add a module-surface architecture assertion so the large coordinator cannot
  silently return as one production file.
- No serialized type changes; therefore no prepared-asset revision bump.

## Gate

Focused interaction/console/world tests, Cucumber, then the full Rust gate.

## Shipped amendments

- The module root is held to a 250-line architecture gate (currently 144
  lines). Shared authorities moved to `state.rs`, delayed travel to `door.rs`,
  and scripted focus-bypassing operations to `scripted.rs`; this keeps the
  console/BRP seam explicit without duplicating player activation state.
