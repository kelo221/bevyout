# M4 wave 10 — manual acceptance script

**What this wave shipped, in plain language:** Fallout 3's external actor KF
animations can now be prepared as a reusable, animation-only GLB pack without
changing the native scene/actor converter. The prepared catalog records each
clip's authored sequence name, time range, loop mode, root-motion policy,
required and animated targets, controller/interpolator classes, and text keys.
An isolated animation zoo makes the result visible and lets a human or the
loopback agent bridge cycle and inspect every compatible clip. Missing and
unsupported clips stay explicit diagnostics.

Prerequisite: Blender 5.1.2 with the Blender NifTools addon enabled. PyNifly is
not required for this compatibility path. The first full humanoid pack build
is intentionally expensive (about 17 minutes, 147 MiB output, and 6.65 GiB
peak RSS on the acceptance machine); subsequent prepares validate and reuse
it.

## A. Default native preparation does not invoke Blender

1. Run:

   ```powershell
   cargo run-dev -- prepare SuperDuperMart --converter native
   ```

2. **Expected:** preparation succeeds without resolving Blender. The actor
   animation summary reports two discovered sets but no built/reused packs and
   no conversion failures; discovered source clips have `NotConverted` status
   under the informational `conversion_not_requested` diagnostic, not a failed
   status.

## B. Build and reuse the humanoid compatibility pack

1. Run:

   ```powershell
   cargo run-dev -- prepare SuperDuperMart --converter native --actor-animation-converter blender
   ```

2. **Expected on the first run:**
   `actor animation catalog: 11 actor mappings, 2 sets, 1385 ready clips, packs built 1, reused 0, failed clips 27`.
   The 27 failures are retained as `conversion_failed` with the stable reason
   `KF produced no animated channels on the prepared skeleton`.
3. Run the same command again. **Expected:** the cell is skipped as valid by
   the prepare job manifest. If another prepared revision made the cell stale,
   the inner summary instead reports `packs built 0, reused 1`; either result
   proves the validated warm artifact was not rebuilt.

## C. See and control a real humanoid KF

1. Launch raider reference `00041600` at its pistol-equip clip:

   ```powershell
   cargo run-dev -- animation-zoo SuperDuperMart --actor 00041600 --start-clip 1hpequip --agent-bridge --agent-port 15702
   ```

2. **Expected within about one second:** the window shows `LvlRaiderGun`,
   `Clip 79/1385`, source `meshes/characters/_male/1hpequip.kf`, authored
   sequence `Equip`, source range approximately `0.0..0.3667`, source loop
   `clamp`, root-motion policy `preserve_authored` with accumulation root
   `Bip01`, and five text keys. The actor visibly plays the equip motion.
   The startup log/probe should report `bound_targets: 66`; this confirms the
   Blender-space clip pack was retargeted onto the native actor hierarchy
   rather than merely accepted by metadata.
3. Press `Space`. **Expected:** HUD state changes to `paused` and elapsed time
   stops. Press `Space` again to resume.
4. Press Right Arrow, then `R`. **Expected:** the next compatible clip is
   selected and restarts from elapsed zero.
5. Press `L`, then Up Arrow. **Expected:** playback loop becomes `true` and
   speed becomes `2.00x`; the current clip repeats instead of advancing.
6. Optional structured check from another terminal:

   ```powershell
   curl -X POST http://127.0.0.1:15702/ -H 'Content-Type: application/json' -d '{"jsonrpc":"2.0","id":1,"method":"bevyout.animation_zoo_probe","params":{}}'
   curl -X POST http://127.0.0.1:15702/ -H 'Content-Type: application/json' -d '{"jsonrpc":"2.0","id":2,"method":"bevyout.animation_zoo_control","params":{"action":"toggle_pause"}}'
   ```

   **Expected:** the probe reports `count: 1385`, `bound_targets: 66`,
   `error: null`, the source
   metadata named above, required/animated/missing target arrays,
   controller/interpolator arrays, and the original text-key values (including
   `Enum: Equip`, `prn: Bip01 R Hand`, and `Attach`). The control response is
   `accepted: true`.

## D. See and control a real creature KF set

1. Exit the humanoid zoo, then prepare Vault 101 Atrium with the same explicit
   compatibility backend:

   ```powershell
   cargo run-dev -- prepare 00024511 --converter native --actor-animation-converter blender
   ```

2. **Expected:** the summary reports 17 actor mappings, two sets, 1,401 ready
   clips, one pack built and the humanoid pack reused. The added Radroach pack
   contains 16 ready clips with no failures.
3. Launch Radroach reference `0005443b` at a real attack clip:

   ```powershell
   cargo run-dev -- animation-zoo 00024511 --actor 0005443b --start-clip h2hattackleft --agent-bridge --agent-port 15702
   ```

4. **Expected:** the window shows `CG04Radroach`, 16 clips, zero skipped, and
   visible movement. The startup log/probe should report `bound_targets: 48`.
   The attack metadata reports sequence `AttackLeft`, source
   range approximately `0.0..0.6667`, source loop `clamp`, root-motion policy
   `preserve_authored`, accumulation root `Bip01`, transform controllers, and
   authored text keys including `Hit` and `Sound: NPCRoachWings`.
5. Repeat the controls from section C. **Expected:** pause/resume, next,
   restart, loop, and 2x speed all update the HUD and playback exactly as for
   the humanoid pack.

## E. A missing creature set remains honest

1. Exit the zoo and run:

   ```powershell
   cargo run-dev -- animation-zoo SuperDuperMart --actor 0006d921
   ```

2. **Expected:** startup fails clearly with `actor animation set has no clip
   pack: KF asset was not found in loose files or loaded archives`. Protectron's
   20 authored `KFFZ` filenames resolve relative to its model directory, as
   required by the ESM4 contract, but those KF assets are absent. They are not
   silently substituted with the humanoid files of the same basename.

## F. Reproduce the representative weapon-controller row

1. Convert the real laser-pistol model to temporary outputs (use any paths
   outside the repository):

   ```powershell
   cargo run-dev -- nif-convert --asset meshes/weapons/1handpistol/laserpistol.nif --output /tmp/bevyout-laserpistol.glb --report /tmp/bevyout-laserpistol.json --allow-lossy --force
   ```

2. **Expected:** the deterministic summary reports 4 meshes, 3 animations, 3
   channels, 10 keyframes, 6 embedded textures, 0 missing textures, and no
   lossy issues. The GLB retains the named controller nodes/clips
   `##LPSideLatch`, `##LPSmallEnergyCell`, and `##LPTrigger`, plus
   `ProjectileNode`.
3. Delete the two temporary outputs after inspection. They are derived game
   data and must never be committed.

Door Open/Close controller playback remains covered by M2 wave 3 / issue #57;
this wave changes only the new external-KF clip-pack path and does not alter the
door converter.
