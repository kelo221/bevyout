# M4 wave 12 — actor animation game-flow manual acceptance

This wave makes prepared Fallout 3 actor animations play on the actors spawned
by the normal viewer. It selects sex-specific humanoid locomotion, creature
locomotion, and equipped-weapon equip/unequip clips; pauses actors outside the
active visible cell; and keeps the gameplay actor transform authoritative so
animation does not float the actor away from the floor.

The commands below use the isolated wave worktree and an external derived-data
cache. They do not write Bethesda data into the repository.

1. Prepare Super-Duper Mart with the native scene and animation converters:

   ```sh
   cd /Users/simon/projects/bevyout-worktrees/actor-animation-gameflow
   cargo run-dev -- --config /Users/simon/projects/bevyout/.bevyout/config.toml \
     prepare SuperDuperMart \
     --converter native \
     --actor-animation-converter native \
     --rebuild-assets \
     --cache-dir /Users/simon/projects/bevyout-worktrees/actor-animation-gameflow-cache
   ```

   Expect `actor catalog: prepared 11` and an `actor animation:` summary with
   two sets and no failed native conversion worker. The prepared scene is
   `scenes/00017f37/scene.ron` under the cache.

2. Launch the ordinary gameplay viewer, not `animation-zoo`:

   ```sh
   cargo run-dev -- view \
     --manifest /Users/simon/projects/bevyout-worktrees/actor-animation-gameflow-cache/scenes/00017f37/scene.ron \
     --agent-bridge --agent-port 15702
   ```

   Expect six `actor-animation ready` lines with `targets=67`. A Protectron in
   this cell has no prepared native pack and reports that explicitly; it is not
   the creature acceptance actor used below.

3. Open the viewer console with backquote and inspect female Raider
   `00041600`:

   ```text
   actorinspect 00041600
   ```

   Expect `animation.present=true`, `bound_targets=67`, a non-empty `set_id`,
   `clip=mtidle__2`, `loop_mode=Loop`, and no animation diagnostic.

4. Drive every humanoid state, running `actorinspect 00041600` after each:

   ```text
   actoranim 00041600 walk
   actoranim 00041600 run
   actoranim 00041600 turn_left
   actoranim 00041600 turn_right
   actoranim 00041600 equip
   actoranim 00041600 unequip
   actoranim 00041600 idle
   ```

   Expect sex-specific `mtforward__2`/`mtfastforward__2`, authored turn clips,
   and `1hpequip`/`1hpunequip` for the equipped pistol. Locomotion loops;
   equip/unequip clamp and return to idle after completion. Limbs must move,
   feet must remain aligned with the floor, and the actor root must not drift
   while a locomotion clip plays.

5. Stop the viewer, prepare the Vault 101 Atrium creature fixture, and launch
   its exact manifest:

   ```sh
   cargo run-dev -- --config /Users/simon/projects/bevyout/.bevyout/config.toml \
     prepare Vault101b \
     --converter native \
     --actor-animation-converter native \
     --cache-dir /Users/simon/projects/bevyout-worktrees/actor-animation-gameflow-cache
   cargo run-dev -- view \
     --manifest /Users/simon/projects/bevyout-worktrees/actor-animation-gameflow-cache/scenes/00024511/scene.ron \
     --agent-bridge --agent-port 15702
   ```

   Expect `actor-animation ready ... targets=48` for the live Radroach.

6. Inspect and drive Radroach `0005443b`:

   ```text
   actorinspect 0005443b
   actoranim 0005443b walk
   actoranim 0005443b run
   actoranim 0005443b turn_left
   actoranim 0005443b turn_right
   actoranim 0005443b idle
   ```

   Expect 48 bound targets and clips `mtidle`, `mtforward`, `mtfastforward`,
   `mtturnleft`, and `mtturnright`, all without a fallback diagnostic. The
   creature body animates while its gameplay root remains stationary.

7. Restart either exact manifest and repeat `actorinspect` once. Expect a new
   actor entity to bind to the same set and target count; no stale player,
   target, or clip state survives the reload.
