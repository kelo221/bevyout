# M9 wave 4 — manual acceptance script

What this wave shipped, in plain language: **limbs are real now.** Hitting a
body part damages that pool, crippling a leg slows you, a crippled head
drops Perception, and a Stimpak restores the limb you selected. Player RPG
state including those limbs survives save/load as format v9 `RPGS`.

- Six pools: head, torso, left/right arm, left/right leg. Unknown mesh
  names fall back to torso.
- One cripple transition at 0 milli. One crippled leg is 60% speed; two
  are 40%. Each crippled arm adds 50% reload time.
- `showlimbs`, `cripple`, and `selectlimb` inspect and debug-mutate the
  player. Pip-Boy Stats meters use live fractions, not demo numbers.
- Owned-bed healing exists as a core policy on explicit game time; it
  does not run from a Bevy timer yet (wave 9).

## 0. One-time setup

```
cargo run-dev -- prepare --cell 000151e3
```

Launch the viewer with the agent bridge (or use the in-game console `~`):

```
cargo run-dev -- view --manifest .bevyout/cache/scenes/000151e3/scene.ron --agent-bridge --agent-port 15702
```

Megaton player house. Bridge examples use `curl` against port 15702.

## A. Inspect healthy limbs

1. `showlimbs`
   Expected: every part `current_milli` 100000, `crippled` false,
   `locomotion_bps` 10000, `arm_reload_bps` 10000, `selected` `torso`.
2. Open Pip-Boy Stats. The six condition meters should be full (not the
   old demo 82%/72%/… bars).

## B. Cripple and locomotion

3. `cripple left_leg`, then `showlimbs`.
   Expected: left_leg `current_milli` 0, `crippled` true,
   `locomotion_bps` 6000. Walking in FPS should feel slower.
4. `cripple right_leg`, then `showlimbs`.
   Expected: `locomotion_bps` 4000.
5. `player.getav perception` while healthy head → 5 (default sheet).
   `cripple head`, then `player.getav perception` → **1** (base 5 − 4,
   clamped).

## C. Targeted Stimpak

6. `selectlimb left_arm` then `cripple left_arm`.
7. `additem 00015169 1` (Stimpak). Use the instance id printed by additem:
   `useitem <instance-id>`.
   Expected: the item is consumed (one ledger unit) and `showlimbs`
   reports left_arm `current_milli` 30000 (0 + 30000) and not crippled.
8. Pip-Boy Stats left-arm meter should match that fraction (0.30).

## D. Save v9

9. `save testlimbs` (or the slot command this build uses), quit, relaunch
   with `--save-slot testlimbs` (or load the slot).
   `showlimbs` must restore the same milli/cripple flags. Format is v9;
   older v8 saves load with healthy/default RPG limbs.

## E. Combat proof (optional)

10. Equip the 10mm, `weaponfire` at an NPC. `combatstate` reports
    `"limbs": true`. A head hit that newly cripples requests the dedicated
    head-blur FX (presentation only; limb state is still core).
