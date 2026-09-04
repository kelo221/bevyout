# M9 wave 10 plan — shared RPG inspection, Pip-Boy, v9 freeze

## Execution model recommendation

Roadmap recommendation: **Sol High** (Codex runtime) / **Opus** (Claude
runtime) — one inspection kernel plus thin viewer adapters on Pip-Boy,
console, BRP, and save tests. ZCode runtime: the orchestrating session
executes directly on `M9-Work`. Sequential exception: `pipboy.rs`,
`agent_bridge.rs`, `stats_commands.rs`, and `features.rs` share files.

## Fixed feature list

### Core

- `RpgInspectionSnapshot` with schema revision 1.
- No Bevy Entity, no asset handles, milliseconds, basis points.
- Arrays sorted by stable ID (perk FormID, body-part enum, addiction id).
- VATS sub-snapshot: `available: false`, `reason: unavailable`,
  `planned_wave: 8`.
- Current AP is `None` until Wave 8; max AP is derived.

### Pip-Boy / console / BRP

- `PlayerStatus` is a formatter fed by the snapshot.
- Stats CND/RAD/EFF stay as labels; RAD/EFF content and Data World clock
  come from the snapshot.
- Items VAL/WG/CND already live; do not invent quests.
- `showstats` on `StatsCommandProvider`.
- BRP probes wrap the same snapshot with serde_json in the viewer.

### Persistence

- Save remains **v9** / `RPG_SAVE_REVISION` 1.
- Unknown RPGS subrecords skipped; missing/corrupt HEAD rejected.
- v1–v8 still default RPG; v9 exact round-trip of active sections.

## Tests-first order

1. `features/rpg_pipboy_save.feature`.
2. Core unit tests in `crates/bevyout-core/src/tests/inspection.rs`.
3. Cucumber World fields after Wave 9; steps at EOF.
4. Save unknown-subrecord / HEAD tests.
5. Viewer projection, `showstats`, BRP methods.
6. Implement until green; gates; `M9_WAVE10_MANUAL.md`.

## Acceptance gates

- Default GOTY sheet: HP 200/200, AP max 75 unavailable current, XP 0/200.
- Radiation stage, limb cripple, and calendar text match the snapshot.
- `vats_probe` / VATS snapshot unavailable with planned wave 8.
- `cargo fmt --check`, clippy `-D warnings`, tests.

## Shipped amendments

- **A1.** Keep existing CND/RAD/EFF Stats labels rather than rewriting
  the tab strip to STATUS/SPECIAL/SKILLS/PERKS/EFFECTS. Live radiation
  and effects text is enough for inspection proof.
- **A2.** Current AP is omitted (`—/max` in the header) instead of
  repeating max as current. Hardcoded 85 is not live state.
- **A3.** VATS uses the M9 inspection schema (`planned_wave: 8`), not
  the M5 combat inspect `planned_wave_7` payload, so Pip-Boy/BRP/console
  share one model. `vatsstate` remains the M5 combat inspect command.
