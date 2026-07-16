# M3 wave 7 — manual acceptance script (#117, #118)

## What this wave shipped (summary)

- **#117 Recipe records**: the OpenMW-derived ESM4 importer decodes RCPE
  recipe records (ingredients, outputs, conditions) into a deterministic,
  content-fingerprinted `recipes.ron` catalog referenced from every scene
  manifest, with pure validation (missing/duplicate ingredients, missing
  outputs, non-positive quantities are rejected without partial state).
  Note: RCPE is a Fallout: New Vegas record type — **Fallout 3 data
  contains none**, so the prepared catalog is correctly empty on FO3;
  the decode path is proven by synthetic fixtures. No crafting execution
  or UI yet.
- **#118 Corpse loot holders**: a `Corpse` placement semantic that opens
  the existing container transfer UI (same take-one/take-stack/take-all
  rules) via player activation or console `activate`, persisting through
  the existing reference-inventory save seam (no format bump, old saves
  load unchanged). Real FO3 scenes author dead actors as NPCs, not
  corpse placements, so there is **no real corpse to loot in-game until
  follow-up #120** — the boundary is proven by console-harness and save
  round-trip tests.

## 1. Recipe catalog on real data (#117)

```
cargo run-dev -- prepare --cell 000151e3
```

Expected:

1. Prepare completes; the deterministic `item catalog: ...` line prints.
2. `.bevyout/cache/catalogs/<fingerprint>/recipes.ron` exists and reads:
   `revision: "openmw-recipes-v1"`, `source_fingerprint` matching the
   catalogs directory name, and `recipes: []` (empty on FO3 — correct).
3. `.bevyout/cache/scenes/000151e3/scene.ron` contains
   `recipe_catalog_path`, `recipe_catalog_revision: Some("openmw-recipes-v1")`,
   and a non-empty `recipe_catalog_hash`.

## 2. Corpse seam in the viewer (#118)

```
cargo run-dev -- view --manifest .bevyout/cache/scenes/000151e3/scene.ron
```

1. Open the console (`) and run `activate` against any non-activatable
   reference (e.g. a static's FormID) — expected error message now names
   the full set: `activate supports only door, container, corpse, and
   pickup references`.
2. Container looting still works end to end: activate a container (E or
   `activate <FormID>`), take an item, close — unchanged behavior.
3. `save wave7`, quit, relaunch with `--save-slot wave7` — container and
   world-item state round-trips exactly as before (corpse holders reuse
   this same seam; old saves without corpse data load unchanged).
4. There is deliberately no in-game corpse to click yet: real dead
   actors are authored as `Npc` placements (see plan amendment A10);
   playable corpse looting lands with #120.
