# M5 wave 2 plan — ammunition, loadout, and save v5

Cross-wave architecture and Waves 3–9 are preserved in
`M5_COMBAT_ARCHITECTURE_ROADMAP.md`.

## Execution model recommendation

Use **Sol X-High** in the Codex runtime. The wave crosses the canonical item
ledger, persisted item identity, preparation revisions, weapon input, console
automation, and real-data acceptance.

## Fixed feature list

### #244 — canonical ammunition

- Add per-instance magazine state and pure reload/fire decisions.
- Add one atomic, idempotent ledger operation for reserve consumption, returned
  rounds, and weapon mutation.
- Split mutable stacked weapons to singleton instances.
- Generalize the canonical binding to an active weapon plus apparel set.

### #245 — preparation and persistence

- Preserve WEAP clip size and AMMO compatibility in prepared combat profiles.
- Store Wave 2 item combat state and loadout in save v5.
- Migrate v1-v4 conservatively: magazines empty, reserve ammunition untouched.
- Bump and pin item, prepare, and save revisions.

### #246 — runtime adapter

- Resolve equipped weapons by `ItemInstanceId`.
- Consume one loaded round before collecting spatial evidence.
- Commit reload inventory changes at the core boundary, not animation events.
- Preserve Wave 1 presentation and actor-state persistence.

### #247 — inspection and acceptance

- Register `ammostate`, `combatstate`, `vatsstate`, and `hitboxdebug`.
- Return typed unavailable values for later-wave capabilities.
- Prove reload, fire, dry fire, and save/reload through BRP using real records.

## Tests-first order

1. Add Wave 2 Cucumber scenarios and pure core unit tests.
2. Add migration, prepared-catalog revision, adapter, and console golden tests.
3. Implement the core and save changes until focused suites pass.
4. Integrate player/console runtime paths.
5. Run formatting, workspace tests, architecture tests, clippy, native prepare,
   and real-data BRP acceptance.

## Acceptance gates

- A successful reload moves only missing compatible rounds into the magazine.
- One accepted shot consumes exactly one loaded round.
- Dry fire has no spatial or presentation effect.
- Ammo switching is atomic and rolls back completely on failure.
- Save v5 preserves magazine, reserve, and active-weapon identity.
- Existing weapon presentation and damage behavior remain intact.

