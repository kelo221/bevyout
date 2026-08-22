# M9 wave 3 plan — active effects, ingestibles, radiation, addiction

## Execution model recommendation

Roadmap recommendation: **Sol X-High** (Codex runtime) / **Opus** (Claude
runtime) — the wave crosses ESM4 decode, three new pure kernels, the
canonical item-use seam, and PRNG determinism. ZCode runtime: the
orchestrating session dispatches a single executor on branch `m9-wave3`
(stacked on `m9-wave2`) in dependency order #316 → #317 → #318, then runs
gates, live acceptance, and the PR itself.

## Verified wire formats (real-data probe, 2026-08-21/22)

### ALCH (ingestible)

Subrecords in order: `EDID`, `FULL`, (OBND/MODL/MODT/ICON/MICO/YNAM/ETYP/
BIPL — skip content), `ENIT`, then one or more effect pairs `EFID` + `EFIT`
(each optionally followed by `CTDA`).

- `ENIT` (20 bytes): `{ u32 value_caps, u8 flags, [3 pad 0xCD],
  u32 use_sound_formid, f32 weight, u32 addiction_formid }`
  - Stimpak 00015169: value 25, weight 0, addiction 0.
  - Jet 00015164: weight 0.2, addiction perk 0x0008C77B.
  - Psycho 00015166: weight 0.1, addiction perk 0x00090A25.
- `EFID` (4B): MGEF FormID. `EFIT` (20B): `{ i32 magnitude, u32 area,
  u32 duration, i32 range, u32 raw }`.
  - Stimpak: RestoreHealthStimpak mag 30 (heals 30 HP).
  - RadAway 00015167: mag 50 (−50 rads). RadX 00015168: mag 25 (rad resist).
  - Buffout 00015163: STR+2 (0x0006697C), END+3 (0x0006697D),
    health 60 (0x00065960). Jet: AP+30 (0x00066EB8), duration 108000.
  - Common trailing effect MGEF 0x0000014F (mag 30, long duration) on
    chems — the chem-duration umbrella; identify via MGEF decode.
  - Stimpak's second effect carries a CTDA with function 0x000001C1
    (GetHealthPercentage, value 1.0) — the "don't overheal" condition.

### MGEF (effect definition)

`EDID`/`FULL` + `DATA` (72 bytes): `{ u32 flags/archetype, …, u32
associated_actor_value_formid (offset 28; e.g. 0x6329 for health), f32
multiplier (1.0 observed at offset 44), … }`. EDIDs are self-describing on
real data (`RestoreHealthStimpak`, `ChemIncSTBuffout`, `ChemIncAPJet`,
`ChemIncHealthBuffout`, `ChemIncENBuffout`). Decode archetype flags +
associated AV FormID; resolve the AV through the AVIF catalog entries
already stored in `gmst.ron`'s `actor_values` list (EditorIDs like
`AVHealth`, `AVActionPoints`).

**Open verification item (do not guess):** where the addiction *chance*
lives on real data. ENIT has no chance field; check the addiction PERK
records (0x0008C77B, 0x00090A25) for data or conditions, and the GECK-Notes
addiction-formula page. If genuinely absent from the base-game data, ship
the engine parameterized with the documented vanilla default (per chem,
commonly 10%) and record it as a plan amendment.

## Fixed feature list

### #316 — decode + catalog

- `parse_alch`/`parse_mgef` (+ `SharedCatalog` plumbing) exactly per the
  wave-1/wave-2 patterns; `src/vsa/prepare/effect_catalog.rs`
  (`EFFECT_CATALOG_REVISION = "openmw-effects-v1"`) →
  `catalogs/<fp>/effects.ron`, containing ingestible entries (effects,
  addiction FormID, ENIT facts) and the MGEF semantic table (archetype
  flags, associated AV). Orchestrator wiring + summary line; re-exports.

### #317 — pure kernels

- `effects.rs`: `ActiveEffect { source: EffectSource, actor_value:
  ActorValue, magnitude: f32, remaining_ms: u32 }`;
  `ActiveEffectsLedger` (ordered Vec/BTreeMap) with `apply` (merge or
  append per source), `tick(delta_ms) -> Vec<expired>`, and
  `special_projections(&CharacterSheet) -> effective SPECIAL` (base +
  modifiers clamped 1..=10).
- `radiation.rs`: `RadiationPool { rads: u16 (0..=1000) }`,
  `apply_radiation(current, dose_bps_resisted)`, threshold table
  (200/400/600/800/1000) → `radiation_penalties(rads) -> BTreeMap<
  SpecialAttribute, i8>` per the roadmap values, fatal at 1000.
- `chems.rs`: `RpgRngState { state: u64, draw_index: u32 }`
  (splitmix64-style next(); serializable; `draw_bps(limit_bps) -> u32`),
  `AddictionState` machine (Clean / Addicted(active) / Withdrawing),
  `roll_addiction(chance_bps, chem_resist_bps, &mut rng) -> bool`
  (succeeds when draw < chance × (1 − resist)).
- Wave-1 sheet integration: `effective_special` honors active effect
  modifiers + radiation penalties through an explicit projection function
  (the stored sheet stays authoritative).

### #318 — runtime + console

- Startup catalog loading (perk/gmst pattern); components
  `RadiationPool`, `ActiveEffectsList`, `Addictions` on the player;
  `RpgRngState` app resource seeded deterministically at startup
  (document the seed).
- `useitem` integration: when the used item is a cataloged ingestible,
  apply its effects (health restore clamps at derived max, rad changes,
  SPECIAL/AP modifiers with EFIT durations in ms), roll addiction for
  chems with an addiction FormID; `addchem <FormID>` consumes without
  inventory for testing.
- Systems in WorldSync: `tick_active_effects(delta_ms)` (expired
  withdrawal starts), `update_radiation_penalties`.
- Console: `rads`, `addrads <n>`, `removerads <n>`, `addchem <FormID>`,
  `cureaddiction [FormID|all]`, `effects`; tests in the console harness.
- `features/rpg_effects.feature` + steps (ledger tick/expiry, thresholds,
  PRNG determinism, addiction transitions); World fields appended at the
  end of `tests/features.rs`.

## Tests-first order

1. `features/rpg_effects.feature` scenarios.
2. Harness fields/steps + `mod effects/radiation/chems` shims.
3. Parser fixtures (Stimpak/Jet/Buffout/RadAway/RadX ground truth) +
   catalog tests.
4. Kernel unit tests (thresholds, ticks, PRNG, addiction machine).
5. Console tests.
6. Implement until green; gates; real `prepare`; agent-bridge acceptance;
   evidence on issues; `M9_WAVE3_MANUAL.md`.

## Acceptance gates

- `prepare` prints the effect-catalog summary (record counts from real
  data) and writes `effects.ron`.
- Live: `player.getav health` after damage + `useitem`/`addchem` stimpak
  restores exactly its magnitude (clamped at max); `addrads 600` applies
  the 600-rads penalty to effective SPECIAL; `removerads`/RadAway reverses
  it; `addchem 00015164` (Jet) adds AP modifier and (under a fixed seed)
  addiction rolls are reproducible; `cureaddiction` clears.
- All repository gates pass; one PR closes #316, #317, #318.

## Shipped amendments

- **Addiction chance location (open verification item) — RESOLVED.**
  The chance lives in `ALCH.ENIT` at offset 12 as an authored f32
  fraction (Jet 0.2 = 20%, Buffout 0.1), *not* in the addiction PERK
  records and not a global default. `ENIT` also carries the withdrawal
  spell FormID at offset 8 (e.g. Jet 0x00033067). The catalog stores it
  as `addiction_chance_percent`; the kernels consume basis points.
- **RadAway polarity.** Real data authors RadAway's
  "RestoreRadiationLevel" MGEF with a **positive** EFIT magnitude (+50)
  on the Rads actor value; positive on Rads means *remove* rads
  ("restore" semantics). Instant positive-Rads ingestible effects remove
  rads; negative magnitudes irradiate. Verified by direct ESM byte probe
  (MGEF 0x0001517A, av index 54, flags 0x70) and live bridge.
- **Conditioned effect policy (#316/#318).** 47 of the catalog's effect
  items carry CTDA conditions (Stimpak heals condition on
  GetHealthPercentage). Wave 3 decodes and stores `conditioned: true`
  but does not evaluate conditions at runtime — application skips them
  with an explicit count in the JSON/log so the behavior is visible.
  Condition evaluation is future work.
- **`PlayerVitals` component added beyond the plan list.** No damage/
  health surface existed on the player before this wave, so instant
  Health effects had nothing to heal into; wave 3 adds a minimal
  `PlayerVitals { current_health }` seeded at derived max (heals clamp
  there; negative Health effects are out of scope until combat W4).
- **Chem-dose attribution.** The core ledger cannot name which chem an
  expiring effect belonged to, so the runtime keeps a parallel
  `chem_doses_ms` map keyed by withdrawal spell FormID; expiry emits
  `ExpiredChemDose` for withdrawal handling.
- **Manual acceptance numbers.** With the default seed the first Jet
  addiction lands on PRNG draw 9 (390 bps < 2000 bps); draws 0–8 fail.
  Cited in `M9_WAVE3_MANUAL.md` step C.
