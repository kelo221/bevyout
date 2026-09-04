# M9 wave 4 plan — body parts, limb health, crippling, medical aid, save v9

## Execution model recommendation

Roadmap recommendation: **Sol X-High** (Codex runtime) / **Opus** (Claude
runtime) — the wave crosses a new core combat kernel, save-format v9, the
existing hitscan damage path, Pip-Boy, and console. ZCode runtime: the
orchestrating session executes directly on `M9-Work`.

## Fixed feature list

### Core

- `combat/body.rs`: `BodyPartId` + `from_node_name` (viewer names mapped at
  the adapter; core never sees mesh paths).
- `combat/limbs.rs`: milli condition (`LIMB_MAX_MILLI = 100_000`), one
  cripple transition at 0, `ShotId` duplicate rejection, locomotion
  100%/60%/40% bps, arm reload +5000 bps and spread +2500 bps per crippled
  arm, head PER −4.
- `combat/medical.rs`: `restore_limbs` for TargetedStimpak (+30_000 milli
  one limb), Doctor, OwnedBed (explicit `GameTime`).
- `weapon::resolve_actor_impact` extended with optional `BodyPartId` /
  `ShotId` / `TargetId`. Duplicate `ShotId` rejects **before** health apply.
  Limb milli = `weapon.damage * 1000`.
- `projected_special_with_limbs` / `projected_derived_with_limbs`. Effect
  index 25 stays unmapped.

### Persistence

- `CURRENT_SAVE_FORMAT_VERSION = 9`.
- `RPGS` after `CRNG`, before `CHKS`: HEAD/STAT/PERK/EFCT/RADS/ADDI/LIMB/RNG0.
- v1–v8 decode defaults empty/full RPG and healthy NPC limbs.
- Player limbs live on `PlayerProgression` / `RPGS.LIMB`. NPC limbs live on
  `ActorInstanceState` / `ACTR LIMB`.

### Viewer

- Hitscan maps `Name` → `BodyPartId`, records `ShotId(shot_index)`.
- Ground/noclip/swim speed scales by locomotion bps; reload duration scales
  by arm-reload bps.
- Pip-Boy six condition meters use live `LimbState::fraction()`.
- Console: `showlimbs`, `cripple <part>`, `selectlimb <part>`.
- Stimpak `useitem` / Pip-Boy Use restore the selected limb after consuming
  one ledger unit.
- Head-cripple requests a dedicated blur FX; blur remains presentation-only.

## Tests-first order

1. `features/rpg_limbs.feature`.
2. Cucumber World fields/steps at the end of `tests/features.rs`.
3. Core unit tests (`combat/tests/limbs.rs`, `medical.rs`).
4. Save v9 round-trip + v8 default tests.
5. Console harness (`showlimbs` / `cripple`).
6. Implement until green; gates; `M9_WAVE4_MANUAL.md`.

## Acceptance gates

- Six healthy parts; unmarked nodes → torso; one cripple transition.
- One/two crippled legs → 6000/4000 locomotion bps.
- Duplicate `ShotId` does not apply health or limb damage twice.
- Targeted Stimpak +30_000 milli on the selected limb.
- Save/load restores player and NPC limb milli exactly.
- Pip-Boy meters and `showlimbs` agree with `LimbState`.
- `cargo fmt --check`, clippy `-D warnings`, tests.

## Shipped amendments

- **A1.** Save v9 lands in this wave, not wave 10, so limbs and waves 1–3
  RPG state persist immediately.
- **A2.** Owned-bed restoration is a core policy on explicit `GameTime`;
  runtime activation waits for wave 9.
- **A3.** Effect AV index 25 remains unmapped. Stimpak limb restore is
  keyed off the ingestible (`00015169` / Stimpak editor id), not a new
  actor-value mapping.
- **A4.** Arm spread penalty is stored and inspectable on `LimbState`;
  hitscan still has no spread cone (M5 ballistics). Reload duration is the
  weapon-calculation path this wave actually owns.
