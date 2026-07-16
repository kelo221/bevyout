# M4 wave 1 — manual acceptance (#103)

## What this wave shipped

The preparation pipeline now produces a per-cell **actor catalog**. When you
prepare a cell, every NPC (`ACHR`) and creature (`ACRE`) placed in it is
resolved into an `ActorBlueprint`: its base record plus everything inherited
field-by-field through the Fallout 3 template system (`TPLT` chains), its
race, class, faction memberships with rank titles, SPECIAL/skills, initial
inventory, AI package ids, model/animation file candidates, placement
transform, and diagnostics. Supporting `RACE`, `CLAS`, `FACT`, and `PACK`
records are decoded from the plugins, and `NPC_`/`CREA` records now carry
their full FO3 actor subrecords. Nothing spawns in the viewer yet — that is
later M4 work (#106/#107); this wave is preparation-only, verified through
`prepare` output and the written catalog artifact.

## One-time setup

Requires the Fallout 3 masters configured as for any `prepare` run, and
cargo on PATH (`export PATH="$HOME/.cargo/bin:$PATH"` if needed).

## Steps

1. Prepare Moriarty's Saloon:

   ```
   cargo run-dev -- prepare --cell 00003a35
   ```

   Expected: the output contains the deterministic line
   `actor catalog: prepared 6, inherited 3, unresolved 0, unsupported 0, skipped 0`.

2. Confirm the catalog artifact sits next to the scene manifest:

   ```
   ls .bevyout/cache/scenes/00003a35/actors.ron
   grep -o 'actor_catalog_path: Some("[^"]*")' .bevyout/cache/scenes/00003a35/scene.ron
   ```

   Expected: the file exists and the manifest records
   `scenes/00003a35/actors.ron` (per-cell path, not a shared
   `catalogs/<hash>/` path).

3. Confirm the saloon's actual residents are in the blueprint set:

   ```
   grep -o 'display_name: Some("[^"]*")' .bevyout/cache/scenes/00003a35/actors.ron | sort -u
   ```

   Expected: `Colin Moriarty`, `Gob` (ghoul), and `Nova` appear, along with
   their inventory item names (e.g. drinking glasses, caps-related items).

4. Prepare the ghoul-populated Underworld Concourse and a creature cell:

   ```
   cargo run-dev -- prepare --cell 00024d6b
   cargo run-dev -- prepare --cell 000151e3
   ```

   Expected report lines:
   `actor catalog: prepared 8, inherited 3, unresolved 0, unsupported 0, skipped 0`
   and
   `actor catalog: prepared 1, inherited 1, unresolved 0, unsupported 0, skipped 0`
   (the latter is a Mister Gutsy creature resolved through its template).

5. Prepare the Vault 101 Atrium (largest actor set, heavy template use):

   ```
   cargo run-dev -- prepare --cell 00024511
   ```

   Expected:
   `actor catalog: prepared 17, inherited 11, unresolved 0, unsupported 0, skipped 0`,
   and `grep -o 'display_name: Some("[^"]*")'` on
   `.bevyout/cache/scenes/00024511/actors.ron` shows `The Overseer`,
   security officers (with `10mm Pistol` / `10mm Round` inventory entries),
   and `Radroach` creatures.

6. Regression check — per-cell catalogs must not clobber each other. After
   step 5, re-check step 3's saloon grep: it must still show the saloon
   residents (not Vault 101 actors), and each prepared scene directory has
   its own `actors.ron`:

   ```
   for c in 00003a35 00024d6b 00024511 000151e3; do \
     grep -o 'actor_catalog_path: Some("[^"]*")' .bevyout/cache/scenes/$c/scene.ron; done
   ```

   Expected: four distinct `scenes/<cell>/actors.ron` paths.

7. Determinism: re-run step 1. Expected: the identical
   `actor catalog: …` line, and the catalog artifact is reused byte-for-byte
   (its recorded `actor_catalog_hash` is unchanged).
