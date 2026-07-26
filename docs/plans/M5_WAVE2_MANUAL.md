# M5 wave 2 manual acceptance — canonical 10mm ammunition

Wave 2 makes the 10mm Pistol consume canonical 10mm rounds, reload from the
player inventory, and preserve its loaded magazine across save/load.

## Revision baseline

- Save format: `5`
- Item catalog: `openmw-items-v8-ammo-magazines`
- Prepare pipeline: `prepare-v9-m5-ammo-magazines`
- Combat policy: `m5-combat-v2`
- Inspection schema: `1`
- Verified Fallout 3 content fingerprint:
  `24efdfcef26d1ebb3d347c976da6c85cd8a17e313b8a22c2709ff90b180941d0`

## Setup

1. Prepare Super-Duper Mart:

   ```powershell
   cargo run-dev -- prepare SuperDuperMart --converter native
   ```

2. Launch its manifest with the agent bridge:

   ```powershell
   cargo run-dev -- view --manifest .bevyout/cache/scenes/00017f37/scene.ron --agent-bridge --agent-port 15702
   ```

3. In the viewer console, add and equip the pistol and add 10mm rounds:

   ```text
   player.additem 0000434f 1
   player.equipitem 0000434f
   player.additem 00004241 24
   ammostate player
   ```

   Expect active weapon `0000434f`, compatible ammo `00004241`, an empty
   magazine, and 24 reserve rounds.

## Reload, fire, and dry fire

4. Run `weaponreload`, wait for the reload duration, then run `ammostate
   player`. Expect the magazine to contain the pistol's prepared clip capacity
   and reserve ammunition to fall by the same amount.

5. Run `weaponfire`, then `ammostate player`. Expect exactly one fewer loaded
   round and one additional accepted shot. Capture a viewport showing the
   equipped pistol.

6. Fire until the magazine is empty, then run `weaponfire`. Expect
   `blocked_empty`; shot count, reserve count, audio, recoil, muzzle light, and
   actor state must not change.

## Persistence and inspection

7. Reload, save, restart the viewer, load the save, and run:

   ```text
   ammostate player
   combatstate player
   vatsstate player
   hitboxdebug state
   ```

   Expect magazine, reserve, and active weapon identity to match the pre-save
   state. `vatsstate` and unimplemented combat capabilities must return
   `available=false` with their planned wave rather than an unknown command.

8. Capture the BRP `before`, `action`, `after`, and post-load JSON responses
   plus a non-black viewport under `artifacts/m5/wave-2/10mm-ammo/`.

## Recorded implementation acceptance

The Wave 2 implementation run prepared `00017f37` successfully, loaded 12
rounds from a reserve of 24, consumed exactly one round on an accepted shot,
reached `blocked_empty` at zero without another accepted shot, and restored
`loaded=12`, `reserve=0`, and weapon instance `16` after a save/restart. The
runtime screenshot was captured at
`.bevyout/screenshots/m5-wave2-ammo.png`; it shows the equipped 10mm viewmodel
in Super-Duper Mart. Generated Fallout-derived data remains untracked.
