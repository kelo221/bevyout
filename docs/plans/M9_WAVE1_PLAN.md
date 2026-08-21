# M9 wave 1 plan — S.P.E.C.I.A.L., skills, derived stats, GMST, leveling

## Execution model recommendation

Roadmap recommendation: **Sol X-High** (Codex runtime) / **Opus**
(Claude runtime) — the wave crosses ESM4 preparation, pure formula
kernels, Bevy runtime, and console/agent-bridge acceptance. In the
ZCode runtime the orchestrating session executes directly on the wave
branch (Codex-runtime pattern); the three issues land sequentially in
dependency order #309 → #308 → #310.

## Fixed feature list

### #309 — pure stat and leveling kernels

- New Bevy-free `bevyout-core::stats` module under
  `crates/bevyout-core/src/stats/`; std/serde/glam only.
- `SpecialAttributes` (7 values, base and effective clamped `1..=10`)
  built on the existing `SpecialAttribute` enum; `ActorSkills` (13
  Fallout 3 skills, `0..=100`) built on `ActorSkill`.
- Skill base formula `2 + (2 * primary SPECIAL) + ceil(LCK / 2)` plus
  tag bonus `+15`; luck contributes to every skill.
- `GmstSettings` view type: named f32 multipliers with Fallout 3 GOTY
  defaults (`iAVDHealthBase=100`, `fAVDHealthEnduranceMult=20`,
  `fAVDHealthLevelMult=10`, `iAVDActionPointsBase=65`,
  `fAVDActionPointsAgilityMult=3`, `iAVDCarryWeightBase=150`,
  `fAVDCarryWeightStrengthMult=10`, `iMaxPlayerLevel=30`,
  `iLevelUpSkillPointsBase=10`).
- Derived attributes kernel: Max HP, Max AP, Carry Weight, Critical
  Chance in basis points, Damage/Poison/Rad Resistance with an 85%
  hard cap; every output clamped and validated.
- Leveling engine: XP curve `XP(N) = (N-1) * N / 2 * 150`, levels
  1..=30, `award_xp` returning level-up events, skill points per level
  `10 + INT` (no *Educated* bonus yet — wave 2).
- Skill checks: hard gates and percentile speech-style checks consume
  a caller-supplied draw, never `rand` (speech probability rolls are
  wave-7 adjacent; here only the deterministic threshold math ships).

### #308 — GMST/AVIF preparation

- Decode `GMST` in the openmw_esm4 reader (`reader.rs` match arm,
  `records.rs` `parse_gmst`, `SharedCatalog` fields on
  `ParsedState`/`ParsedPlugin`); EditorID carries the setting name,
  DATA carries the typed value (f/i/b/s variants by EditorID prefix).
- Decode `AVIF`: FormID, EditorID, name, description.
- New `src/vsa/prepare/gmst_catalog.rs` modeled on `package_catalog.rs`:
  pure std/serde module, `GMST_CATALOG_REVISION` const,
  `PreparedGmstCatalog { revision, source_fingerprint, settings }`,
  written to `catalogs/<source_fingerprint>/gmst.ron` (deterministic
  path, no manifest pointer — packages.ron precedent, no
  `CURRENT_PREPARE_REVISION` bump).
- Orchestrator step next to the package-catalog build/write/summary
  block; deterministic `println!` summary line.

### #310 — player stats runtime and console surface

- New `src/viewer/stats/` module + `StatsPlugin` in `ViewerPlugins`;
  components `ActorStats`, `DerivedAttributes`, `Experience` attach via
  `Added<FpsPlayer>` (the player entity is despawned/respawned on
  camera-mode changes); `recalculate_derived_stats` runs in
  `ViewerSet::WorldSync`.
- Console provider in `src/viewer/console/`: `getav <value>`,
  `setav <value> <n>`, `modav <value> <delta>` reference-callable so
  `player.getav health` works; `player.advlevel`,
  `player.rewardxp <xp>` player-scoped (item_commands pattern).
  Reuse `ActorValue::parse`/`label` for value names.
- `features/rpg_stats.feature` + World fields and step section
  appended at the end of `tests/features.rs`; console tests in
  `src/viewer/console/tests.rs` via the existing `test_app()`/`exec()`
  harness.

## Tests-first order

1. `features/rpg_stats.feature`: skill base formula, derived stats
   from SPECIAL+level, XP thresholds and level-up, skill-point award,
   console `getav`/`modav`/`rewardxp` scenarios.
2. Append the feature's World fields and step definitions at the end
   of `tests/features.rs` (merge seam).
3. Unit tests: `crates/bevyout-core/src/tests/stats.rs` formula tables
   against vanilla values; `src/vsa/prepare/tests/gmst_catalog.rs`
   revision pin + build; openmw_esm4 parser fixtures for GMST/AVIF.
4. Console command tests in `src/viewer/console/tests.rs`.
5. Implement until green; run `cargo fmt --check`,
   `cargo clippy --all-targets -- -D warnings`, `cargo test`, and a
   representative native `prepare`.
6. Real-data acceptance through the agent bridge on a prepared cell;
   record evidence on the issues; write `M9_WAVE1_MANUAL.md`.

## Acceptance gates

- Kernel formulas match Fallout 3 GOTY defaults exactly for the
  documented test vectors (HP/AP/carry weight/skill bases/XP curve).
- `prepare` emits `catalogs/<fp>/gmst.ron` plus a deterministic
  summary line on real data; stale catalogs are rebuilt (revision
  check).
- In the viewer: `player.rewardxp 200` crosses the level-2 threshold
  (XP(2)=150), `player.advlevel` levels deterministically,
  `player.getav health` reflects `100 + END*20 + LVL*10`, `modav`
  clamps to valid ranges.
- All repository gates pass; one PR closes #308, #309, #310.

## Shipped amendments

(none yet)
