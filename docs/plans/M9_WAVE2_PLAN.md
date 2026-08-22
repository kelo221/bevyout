# M9 wave 2 plan — perk catalog, requirements evaluator, active modifiers

## Execution model recommendation

Roadmap recommendation: **Sol High** (Codex runtime) / **Opus** (Claude
runtime). In the ZCode runtime the orchestrating session executes directly
on the wave branch `m9-wave2` (stacked on `m9-wave1`), in dependency order
#312 → #313 → #314.

## Verified PERK wire format (real-data probe, 2026-08-21)

Probed directly against the installed GOTY `Fallout3.esm` with known perks
as ground truth. Record headers are 24 bytes; top-level GRUP headers are
24 bytes in this file (`GRUP sig, u32 size, 4CC label, u32 type, 16 more
bytes`) — the Rust `walk_container` already handles this; the numbers
below are only for writing parser fixtures.

Subrecords of a `PERK` record, in order:

- `EDID` — EditorID (cstring)
- `FULL` — display name (cstring)
- `DESC` — description (cstring)
- `ICON` — icon path (cstring; skip content)
- `CTDA` (0+ bytes, 28 bytes each) — requirement condition
  `{ u8 oper_flags (0x60 observed = AND/greater-equal form), f32 comparison_value, u32 function (0x000001EF = GetActorValue), u32 actor_value_index, 12 bytes raw }`.
  The actor-value index maps into the `AVIF` catalog's decoded records
  (wave 1 already decodes 60 of them with EditorIDs) — resolve through
  that list and verify against known perks in acceptance.
- `DATA` (5 bytes) — `{ u8 unknown0, u8 min_level, u8 ranks, u8 playable, u8 hidden }`.
  Verified: Swift Learner `00 02 03 01 00` (level 2, 3 ranks), Intense
  Training `00 02 0a 01 00` (level 2, 10 ranks), Educated `00 04 01 01 00`
  (level 4, 1 rank), Bloody Mess `00 06 01 01 00` (level 6).
- One or more perk entries, each:
  `PRKE` (3 bytes) `{ u8 entry_type (0 = quest, 1 = ability, 2 = entry point), u8 rank, u8 priority }`,
  followed by an inner `DATA` whose shape depends on the type:
  - quest (0): 8 bytes — quest FormID (u32) + 4 raw bytes
    (Intense Training: `b2380000 65cdcdcd`, ten entries, ranks 0..9)
  - ability (1): 4 bytes — ability `SPEL` FormID
    (Night Person rank 0 → `00094EBE`, Gun Nut ranks 0..2 → `0004494F/50/51`)
  - entry point (2): 3 bytes — `{ u8 entry_code, u8 param_count, u8 priority? }`
    (Swift Learner `09 03 01`, Educated `0a 02 01`), then `EPFT` (u8
    function, 1 observed) and `EPFD` (f32 for value entry points).
    Verified values: Swift Learner ranks 1..3 EPFD = 1.1 / 1.2 / 1.3
    (XP multiplier), Educated EPFD = 3.0 (bonus skill points), Bloody
    Mess entry `00 03 03` + EPFT 1 + EPFD 1.05 (generic damage ×1.05).
  - closed by `PRKF` (0 bytes; content after it belongs to the next entry).

Entry codes observed: `0x09` XP-award multiplier, `0x0a` bonus skill
points per level. Store unknown codes raw; do not guess semantics.

## Fixed feature list

### #312 — PERK decode and catalog

- `parse_perk` in `records.rs` + `PerkRecord` in `openmw_esm4/mod.rs` +
  `SharedCatalog` fields/clone (`reader.rs` match arm, exactly the GMST/
  AVIF pattern from wave 1, #308).
- `src/vsa/prepare/perk_catalog.rs` modeled on `gmst_catalog.rs`:
  `PERK_CATALOG_REVISION = "openmw-perks-v1"`, `PreparedPerkCatalog`
  with `entries` sorted by FormID + counters (playable/hidden,
  entry-type counts, unsupported CTDA functions), written to
  `catalogs/<fp>/perks.ron`, orchestrator build/write/summary after the
  GMST catalog block.
- Parser fixtures pinning all four ground-truth perks above; catalog
  revision pin + RON round-trip.

### #313 — pure evaluator and modifiers

- `bevyout_core::perks`: `PerkDefinition`, `PerkEntry` (typed quest/
  ability/entry-point variants), `PerkCondition` (ActorValue + threshold),
  `PerkProgression` (`BTreeMap<u32 /*FormId*/, u8 /*rank*/>`),
  `PerkEligibility` (ok / reasons), `can_take_perk`,
  `active_perk_modifiers` → `PerkModifiers { xp_award_multiplier_bps,
  bonus_skill_points, flags… }` (built only from entry codes 0x09/0x0a).
- Extend `stats::award_xp` with an optional xp multiplier parameter
  (basis points) and `skill_points_per_level` with a bonus-points
  parameter — wave-1 call sites updated; clamps keep existing behavior
  at multiplier 10000 bps / bonus 0.
- Unit tests: eligibility (level/rank/condition gates, AV-index mapping),
  modifier math (Swift Learner stacking 1.1·1.2·1.3 by rank, Educated
  +3), serde round-trips.

### #314 — runtime and console

- `catalogs/<fp>/perks.ron` loaded at view startup
  (`load_settings_for_manifest` pattern) into a `PerkCatalog` resource;
  `ActorPerks` component attached with the stats bundle.
- Console provider (extend `stats_commands.rs` or new `perk_commands.rs`;
  provider count 15 → 16): `addperk <FormID> [ranks]`,
  `removeperk <FormID>`, `hasperk <FormID>`, `showperks` (owned with
  ranks + eligible-with-blocked-reasons); `.reference_callable(false)`
  player-scoped, mutating where applicable. `addperk` enforces
  `can_take_perk` unless `--force`-style arg is passed? No: enforce
  eligibility, error with reasons (matches engine behavior).
- `features/rpg_perks.feature` + World fields + step section at the end
  of `tests/features.rs` (`mod perks { pub use bevyout_core::perks::*; }`
  shim); console tests for all four commands.

## Tests-first order

1. `features/rpg_perks.feature` scenarios (eligibility, rank gates, XP
   multiplier, skill-point bonus).
2. `tests/features.rs` fields/steps + shim.
3. Parser fixtures + catalog tests (#312).
4. Core kernel tests (#313).
5. Console tests (#314).
6. Implement until green; run `cargo fmt --check`,
   `cargo clippy --all-targets -- -D warnings`, `cargo test`, and a real
   `prepare --cell 000151e3`.
7. Agent-bridge acceptance; evidence on issues; write
   `M9_WAVE2_MANUAL.md`.

## Acceptance gates

- `prepare` prints a perk-catalog summary (~120 perks, 0 unsupported
  subrecords) and writes `perks.ron`.
- `player.addperk 00031dd3` succeeds; `player.rewardxp 1000` awards
  1100; `player.addperk 00031dd3` (second rank) upgrades rank 1→2;
  `showperks` lists Swift Learner rank 2/3; `player.removeperk 00031dd3`
  clears it and the multiplier reverts.
- Educated (`00031dd8`) grants +3 skill points on the next level-up.
- Ineligible perk (level gate) returns a blocked result with reasons.
- All repository gates pass; one PR closes #312, #313, #314.

## Shipped amendments

### A1 — AV condition indices are an engine enum, not AVIF FormIDs

The plan's suggested mapping (`AVIF form_id == condition index`) is false on
real data — `AVStrength` is FormID 1000, not 5. The executor probed all 87
PERK records and derived the engine's condition-index enum empirically by
pairing every CTDA with its published requirement: SPECIAL at indices 5–11
(STR 5, PER 6, END 7, CHA 8, INT 9, AGI 10, LUK 11; verified by Strong Back
gating on 5+7, Swift Learner/Educated on 9, Thief on 6+10, Better
Criticals on 6+11) and skills at 32–45 (verified by Master Trader 32,
Ninja 38/42, Computer Whiz 40, Paralyzing Palm 45). The mapping lives in
`bevyout_core::perks::actor_value_from_condition_index` with the evidence
documented; unmapped indices block eligibility as `unknown_conditions`.

### A2 — catalog counters describe reality, not the plan's guesses

The base `Fallout3.esm` carries **87 perks** (58 playable, 3 hidden), not
the plan's ~120 estimate (no DLC plugins in this content set). The "0
unsupported subrecords" gate materialized as **48 unsupported condition words
across 21 perks**, including non-`GetActorValue` CTDA functions (GetIsSex,
HasPerk, …). They are surfaced as `unknown_conditions` eligibility blockers
and in `showperks --eligible` reasons. Zero unknown subrecord signatures.

### A3 — one kernel parameter, not two

`stats::award_xp` gained the XP-multiplier parameter (bps); the per-level
skill-point bonus is applied by the console/runtime adapter at the
`skill_points_per_level` call site rather than threading a second
parameter through the kernel — same observable behavior, smaller seam.

### A4 — `addperk` is single-rank per invocation

`addperk <FormID>` grants exactly one rank per call (repeat calls take the
next rank, as in the engine); no `[ranks]`/force arguments shipped.
