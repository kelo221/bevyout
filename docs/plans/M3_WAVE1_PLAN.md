# M3 wave 1 plan — OpenMW-based inventory and persistent dropping

## Fixed feature lists

### #70 — prepared item catalog

- Parse and prepare WEAP, ARMO, AMMO, ALCH, MISC, BOOK, NOTE, and KEYM item
  metadata, icons, typed stats, and world/drop asset references.
- Write one content-fingerprinted catalog and reference it from scene manifests.
- Keep adapted OpenMW layouts and notices in `src/vsa/openmw_esm4`.

### #71 — authoritative inventory and Pip-Boy Items

- Use condition-aware stack identity and catalog-derived carried weight.
- Pause virtual time and release the cursor while the Tab modal is open.
- Provide five tabs, item selection, art/stats, and the fixed right-click
  quantity policy. Equip/use/read behavior is deliberately absent.

### #72 — dropped items and save v2

- Validate then atomically remove a stack quantity, spawn it in front of the
  player with dynamic ownership, and atomically retrieve it with `E`.
- Persist stable runtime IDs, item state, transform, and body state per cell.
- Read v1 saves and deterministically write/validate v2 saves.

## Tests before implementation

- `features/inventory.feature`: stack identity, weight, quantity policy, and
  atomic removal.
- Synthetic ESM4/unit coverage: each item category, icon fallback, typed stats,
  catalog determinism, and missing assets.
- Minimal Bevy coverage: modal time/cursor/UI behavior and drop/retrieve.
- Save coverage: v1 migration, v2 round trip, invalid runtime records, and exact
  pickup/drop/reload/retrieve conservation.

## Acceptance and gates

- Run format, clippy with warnings denied, all tests, and representative prepare.
- Through the agent bridge, capture the Items UI, prove pause, drop a single and
  multi-item stack, retrieve one, reload the other, and compare exact counts.
- Post measured evidence on #70/#71/#72. Keep gate #8 open for equipment, use,
  barter, quest restrictions, recipes, containers, and leveled lists.

## Shipped amendments

- The prepared catalog is global to the content fingerprint and runtime drops
  resolve their art, model, and collider from that catalog rather than from a
  cell-local placement. This keeps a dropped stack valid across cell saves.
- Real-data acceptance used Super-Duper Mart (`00017f37`): the warm prepare
  reused all 595 prepared assets and emitted 1,761 item definitions, 1,149
  icons, and 1,044 world assets. The Pip-Boy showed exact weight and typed
  weapon/aid details; a one-item Nuka-Cola drop and a three-item Frag Grenade
  stack held player/body transforms fixed for two seconds while paused.
- Save v2 restart restored the Nuka-Cola at
  `(37.845337, 96.621101, -37.780788)` and the condition-5 Frag Grenade stack
  at `(37.551468, 96.496429, -37.627174)` with the remaining inventory count
  and weight conserved. Synthetic desktop input could open `Tab` and drive
  pointer UI actions, but did not produce Bevy's physical `KeyE`; the live
  `[E] Take` focus prompt and the pickup/retrieve path remain covered by the
  automated Bevy and conservation tests.
