# M5 wave 1 manual acceptance — functional 10mm pistol

This wave makes Fallout 3's 10mm pistol (`0000434F`) usable as a modular
first-person weapon. It resolves the prepared first-person model and fire
audio, fires one center-screen hitscan per left click, flashes a short point
light, animates recoil and reload, and persists actor health/death. Ammunition
is deliberately not counted yet.

## One-time preparation

1. Prepare Super-Duper Mart with the native converter:

   ```powershell
   cargo run-dev -- prepare SuperDuperMart --converter native
   ```

   Expect `prepared cell 00017f37` and an item catalog revision of
   `openmw-items-v7-player-weapons`.

2. Launch the prepared viewer with the agent bridge:

   ```powershell
   cargo run-dev -- view --manifest .bevyout/cache/scenes/00017f37/scene.ron --agent-bridge --agent-port 15702
   ```

## Equip, presentation, audio, and fire

3. Open the viewer console and run:

   ```text
   player.additem 434f
   player.equipitem 434f
   weaponstate
   ```

   Expect `equipped=0000434f`, label `10mm Pistol`, a non-empty
   `viewmodel_asset_path`, `fire_sound_2d_form_id=223962`,
   `fire_sound_3d_form_id=223963`, and `ammo_accounting=false`. Close the
   console. The pistol model must be visible at the lower right of the FPS
   view.

4. Click the left mouse button once.

   Expect one pistol report, one brief recoil motion, and a warm point-light
   flash at the muzzle. There is no particle muzzle flash. Reopen the console
   and run `weaponstate`; expect `shots_fired=1`, `last_fire.audio_form_id`
   `223962`, `last_fire.muzzle_flash_seconds` `0.05`, and no ammo count or
   consumption.

## Reload gate

5. Close the console, press `R`, and immediately click the left mouse button.

   Expect the pistol to lower/rotate through the reload animation and the
   click to produce no shot, sound, recoil, or light. Reopen the console and
   run `weaponstate`; expect `last_reload=started`,
   `last_fire.status=blocked_reloading`, and an unchanged `shots_fired`.

6. Wait at least 1.5 seconds after starting reload, then click once.

   Expect the normal fire sound, recoil, light, and exactly one additional
   accepted shot.

## Deterministic actor hit and death

7. Open the console and place Raider `00041600` four metres in front of the
   player:

   ```text
   tp 38.365150 97.327148 -36.698757
   player.setangle y 0
   player.setangle x 0
   00041600.setactorlife alive
   00041600.setactorvalue health 0
   00041600.setpos x 38.365150
   00041600.setpos y 97.327148
   00041600.setpos z -40.698757
   actorstate 00041600
   ```

   Expect the Raider to report `life=alive` and effective health `10`. Close
   the console; the Raider's torso must be under the center crosshair.

8. Click once, reopen the console, and run:

   ```text
   weaponstate
   actorstate 00041600
   ```

   Expect `last_fire.status=actor_hit`, target `267776` (`00041600`), applied
   damage `9`, remaining health `1`, and actor life `alive`.

9. Close the console, click once more, then inspect the same two commands.

   Expect `last_fire.status=actor_killed`, remaining health `0`, and
   `actorstate 00041600` to report `life=dead`. The two accepted shots must
   require no `10mm Round` inventory stack.

## MCP evidence equivalent

The same checks are automation-safe through `bevyout.console.exec`. For
example, while the viewer from step 2 is running:

```powershell
$body = '{"jsonrpc":"2.0","id":1,"method":"bevyout.console.exec","params":{"line":"weaponstate"}}'
Invoke-RestMethod -Uri http://127.0.0.1:15702/ -Method Post -ContentType application/json -Body $body
```

Use the commands above as the `line` value. `bevyout.capture_viewport` must
also return a non-black image showing the equipped pistol.
