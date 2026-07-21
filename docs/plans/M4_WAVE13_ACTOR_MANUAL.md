# M4 wave 13 — actor state and persistence manual acceptance

This wave gives gameplay actors an immutable prepared definition and one
mutable state keyed by their stable ACHR/ACRE reference. Actor-value mutations,
lifecycle, and package checkpoints survive eviction and save/load; actor items
and equipment remain canonical item instances, so revisiting a cell cannot
duplicate them. The commands below use normal gameplay `view`, not an actor
debug scene.

1. Prepare Super-Duper Mart and Vault 101 Atrium into the external wave cache:

   ```sh
   cd /Users/simon/projects/bevyout-worktrees/actor-state-persistence
   cargo run-dev -- --config /Users/simon/projects/bevyout/.bevyout/config.toml \
     prepare SuperDuperMart \
     --converter native \
     --actor-animation-converter native \
     --cache-dir /Users/simon/projects/bevyout-worktrees/actor-state-persistence-cache
   cargo run-dev -- --config /Users/simon/projects/bevyout/.bevyout/config.toml \
     prepare Vault101b \
     --converter native \
     --actor-animation-converter native \
     --cache-dir /Users/simon/projects/bevyout-worktrees/actor-state-persistence-cache
   ```

   Expect the actor catalog revision
   `openmw-actors-v6-runtime-values-race-skill-modifiers`, 11 prepared actors
   for Super-Duper Mart, and a prepared Radroach in Vault 101. No derived NIF,
   GLB, RON, or audio asset is written into the repository.

2. Launch the Super-Duper Mart gameplay manifest:

   ```sh
   cargo run-dev -- view \
     --manifest /Users/simon/projects/bevyout-worktrees/actor-state-persistence-cache/scenes/00017f37/scene.ron \
     --agent-bridge --agent-port 15702
   ```

   Expect `actor state seeded 00041600 ... life=alive` once for the female
   Raider (and one line for each other spawned actor). It must not repeat every
   frame.

3. Open the console with backquote and inspect Raider `00041600`:

   ```text
   actorstate 00041600
   ```

   Expect reference `00041600`, a non-zero base, race/class identity when
   authored, faction FormIDs/ranks, effective values, an alive lifecycle, and
   a canonical holder. The equipped pistol has one stable item-instance ID.

4. Mutate the same authoritative state and inspect it again:

   ```text
   setactorvalue 00041600 health -12
   setactorpackage 00041600 0002c6f1 3 4.5
   setactorlife 00041600 dead
   actorstate 00041600
   ```

   Expect `health.runtime_mutation=-12`, life `dead`, and package `0002c6f1`
   at procedure 3 / 4.5 seconds. This command tests persistence only: combat
   death animation, ragdoll/corpse conversion, and respawn remain later work.

5. Save the slot and close the viewer:

   ```text
   save actor-state-wave13
   ```

   Expect `save write actor-state-wave13 ...`. The slot is format v4 and
   contains one deterministic `ACTR` record per seeded actor, including this
   actor, alongside the canonical item ledger.

6. Restart from that slot:

   ```sh
   cargo run-dev -- view \
     --manifest /Users/simon/projects/bevyout-worktrees/actor-state-persistence-cache/scenes/00017f37/scene.ron \
     --save-slot actor-state-wave13 \
     --agent-bridge --agent-port 15702
   ```

   Expect `actor state restored 00041600 ... life=dead`. Run
   `actorstate 00041600`; the health mutation and package checkpoint must be
   unchanged, and the canonical holder must contain the same equipped
   item-instance ID and quantities as step 3. The measured Super-Duper Mart
   slot was 61,201 bytes with the mutation/checkpoint and 61,159 bytes after
   step 7 reset them.

7. Restore the Raider to the neutral acceptance state and save once more:

   ```text
   setactorlife 00041600 alive
   setactorvalue 00041600 health 0
   setactorpackage 00041600 none
   save actor-state-wave13
   actorstate 00041600
   ```

   Expect life `alive`, no health mutation, no package checkpoint, and no
   inventory/equipment duplication.

8. Stop the viewer, launch Vault 101 Atrium, and inspect Radroach `0005443b`:

   ```sh
   cargo run-dev -- view \
     --manifest /Users/simon/projects/bevyout-worktrees/actor-state-persistence-cache/scenes/00024511/scene.ron \
     --agent-bridge --agent-port 15702
   ```

   ```text
   actorstate 0005443b
   setactorvalue 0005443b health -1
   actorstate 0005443b
   ```

   Expect kind `Creature`, stable reference/base identity, creature values,
   and an effective health exactly one below its prepared base. No humanoid
   faction/equipment data is invented.
