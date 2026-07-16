# M3 wave 6 — manual acceptance script (#98, #99, #100)

Follow the steps in order; each has the expected result. All FormIDs are
vanilla Fallout 3 and confirmed present in the prepared item catalog.

## What this wave shipped (summary)

- **#98 Equipment**: armor/weapon/ammo can be equipped and unequipped —
  from the Pip-Boy (E key), via hotkeys 1–8 (bindable in the Pip-Boy,
  usable outside it), or with the new `player.equipitem` console command.
  Armor uses real biped-slot data, weapons pair with their ammo type, and
  equipped state + hotkey bindings survive save/load (save format v3).
- **#99 Consumables and reading**: Aid items get a USE button in the
  Pip-Boy (consumes one, plays the item sound, shows its effect labels);
  books and notes with text get a READ button that opens a reader
  overlay. Quest-flagged items can be read but never consumed away.
- **#100 Pip-Boy Data views**: a Data tab beside Items with a Notes list
  (opens the reader) and a read-only World view (current cell, play
  stats).

## 0. One-time setup

The item catalog on disk may predate this wave's new equip fields.
Force a rebuild once:

```
rm -rf .bevyout/cache/catalogs
cargo run-dev -- prepare --cell 000151e3
```

Expected: prepare completes and reports the item catalog was rebuilt.

## 1. Launch

```
cargo run-dev -- view --manifest .bevyout/cache/scenes/000151e3/scene.ron
```

Keys used below: **`** (backquote) toggles the console, **Tab** toggles
the Pip-Boy, **E** activates/equips, **1–8** are hotkeys.

## 2. Stage items (console)

Open the console with ` and run, one line at a time:

```
player.additem 15169 3      → additem 00015169 x3; inventory now has 3   (Stimpak)
player.additem 434f         → 10mm Pistol added
player.additem 4241 24      → 10mm Round x24 added
player.additem 20423        → Leather Armor added
player.additem 2d3a3        → book "Motivational Secrets of the Stars" added
player.additem f 100        → 100 caps
```

## 3. Equip via console (#98)

Still in the console:

```
player.equipitem 434f       → equipitem 0000434f equipped
player.equipitem 4241       → ammo equipped (pairs with the 10mm Pistol)
player.equipitem 20423      → equipitem 00020423 equipped
player.equipitem 20423      → equipitem 00020423 unequipped   (toggle)
player.equipitem 20423      → equipped again (leave it on for step 6)
player.equipitem 15169      → error: not_equippable (Aid is not equippable)
```

## 4. Pip-Boy equip toggle and hotkeys (#98)

1. Close the console (`), press **Tab** → Pip-Boy opens on Items.
2. Select the Weapons tab → the 10mm Pistol row shows the equipped
   marker `[E]`.
3. Select the pistol row, press **E** → marker disappears (unequipped).
   Press **E** again → `[E]` returns.
4. With the pistol row selected, press **3** → notice "Bound hotkey 3".
5. Press **Tab** to close the Pip-Boy, press **3** → the pistol toggles
   (unequips; watch the notice). Press **3** again → equipped again.
6. Hotkey digits must do nothing while the Pip-Boy is open.

## 5. Consumable use and reading (#99)

1. **Tab** → Items → Aid tab. Select the Stimpak stack (x3).
2. Click **USE** → count drops to 2, notice "Used Stimpak: Restore
   Health", pickup sound plays.
3. Misc/Keys rows and quest-flagged Aid items show no USE button.
4. Items → find the book "Motivational Secrets of the Stars", click
   **READ** → reader overlay opens with the book text.
5. Click **CLOSE** → back on the Pip-Boy Items view (not the HUD).

## 6. Pip-Boy Data views (#100)

1. **Tab** → switch to the **Data** tab.
2. Notes view lists the book; selecting it opens the same reader.
3. World view shows the current cell name/form id — cross-check with
   `bevyout.session` if the agent bridge is up.
4. Switch back to Items → still works.

## 7. Persistence (#98, save format v3)

1. Console: `save wave6` → save confirmation line.
2. Quit. Relaunch with the slot:

```
cargo run-dev -- view --manifest .bevyout/cache/scenes/000151e3/scene.ron --save-slot wave6
```

3. **Tab** → pistol, ammo, and Leather Armor still show `[E]`; hotkey
   **3** still toggles the pistol; Stimpak count is still 2.
4. A pre-wave (v1/v2) save must still load, with nothing equipped.
