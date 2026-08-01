# M5 Wave 3 manual acceptance — weapon condition, degradation, jams, and RNG

This wave makes the player’s weapon condition change deterministically as it is
used, exposes the resulting fire/reload jams, and keeps those decisions stable
across save/load. The console provides the inspection and clear-jam path so a
human can see the shipped behavior in the prepared Super-Duper Mart scene.

## One-time setup

From the repository root, prepare the real cell with dynamic linking:

```text
cargo run-dev -- prepare --cell 00017f37
```

Launch the prepared scene with the agent bridge:

```text
cargo run-dev -- view --manifest .bevyout/cache/scenes/00017f37/scene.ron --agent-bridge --agent-port 15702
```

Use a fresh viewer process for each repeated-run comparison. The prepared
catalog contains the 10mm Pistol (`0000434f`) and 10mm Round (`00004241`).

## Acceptance steps

1. Inspect the initial combat state:

   ```text
   combatstate player
   ```

   Expected: the response reports Wave 3 policy `m5-combat-v3`, a finite
   condition/current maximum for the equipped weapon, `jam: none`, the RNG
   revision, draw index `0`, and the last-decision terms. No state is mutated.

2. Give the player the real weapon and ammunition, then inspect again:

   ```text
   player.additem 0000434f 1
   player.equipitem 0000434f
   player.additem 00004241 24
   combatstate player
   ```

   Expected: the pistol is equipped, the 10mm rounds are available, and the
   condition/jam/RNG fields are present. The item keeps a stable instance
   identity when inspected or moved.

3. Reload and fire one accepted shot:

   ```text
   weaponreload
   combatstate player
   weaponfire
   combatstate player
   ```

   Expected: reload consumes the expected magazine quantity; fire preserves the
   Wave 2 hit/audio/recoil/screen behavior, decreases condition once, records
   the decision terms, and advances the RNG draw index only for the accepted
   decision.

4. Attempt a blocked action and prove it does not consume RNG:

   ```text
   weaponclearjam
   combatstate player
   weaponfire
   combatstate player
   ```

   Expected: clearing an unjammed weapon is an explicit no-op; the blocked or
   otherwise rejected fire reports its reason and leaves the draw index and
   condition unchanged. If the shot is accepted, repeat the inspection after
   the next deterministic state transition and use the reported reason rather
   than treating a valid accepted shot as a failure.

5. Produce and clear a deterministic jam by repeating the accepted fire/reload
   sequence until `combatstate player` reports a jam:

   ```text
   weaponfire
   combatstate player
   weaponreload
   combatstate player
   weaponclearjam
   combatstate player
   ```

   Expected: while jammed, fire and reload are blocked with the reported jam
   reason and do not advance RNG. `weaponclearjam` transitions the same weapon
   instance back to `jam: none`; the next legal reload/fire can proceed.

6. Save and reload with a partially degraded weapon:

   ```text
   save wave3-condition
   ```

   Expected: the console reports `.bevyout/saves/wave3-condition.bevyoutsave`
   was written. Stop the viewer and relaunch the same prepared scene with the
   save slot:

   ```text
   cargo run-dev -- view --manifest .bevyout/cache/scenes/00017f37/scene.ron --agent-bridge --agent-port 15702 --save-slot wave3-condition
   ```

   Then run:

   ```text
   combatstate player
   weaponfire
   combatstate player
   ```

   Expected: the loaded condition, jam state, RNG revision, and draw index match
   the pre-restart state. The next decision continues the same deterministic
   sequence, rather than resetting the weapon or RNG.

7. Repeat steps 1–3 in a fresh viewer process with the same prepared manifest
   and inputs. Compare the two `combatstate` traces. Expected: condition,
   jam transitions, draw indices, blocked reasons, and decision terms are
   identical. Record the command output and frame-time observation in the issue
   comments and wave PR.

## Acceptance notes

- Do not commit generated Bethesda-derived manifests or converted assets; they
  remain under `.bevyout/`.
- If the viewer’s window is occluded, use `combatstate`, bridge logs, and scene
  snapshots as evidence; do not treat a black captured PNG as a rendering
  failure by itself.
- The shipped policy identifiers for this checkout are save format v8, item
  catalog `openmw-items-v10-combat-condition`, and prepare revision
  `prepare-v22-m6-worldspace-lod-imad-screen-fx-combat-condition`.
