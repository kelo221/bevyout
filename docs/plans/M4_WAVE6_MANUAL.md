# M4 wave 6 — manual acceptance script

## What this wave shipped

Four independent fixes and features. **Nav agents no longer wedge at
corners** (#136): the walkable navmesh is eroded inward by the agent's
radius at load, with a pinch guard so narrow corridors stay open — the
route toward the Vault 101 entrance door, which reliably stuck the agent
in wave 5, now completes. **Holotapes and notes carry their real text**
(#123): the Pip-Boy reader can finally show what a note says instead of
an empty page. **Authored dead actors are lootable corpses** (#120):
Grandma Taylor in the Escape! cell classifies as a corpse from the real
game data, renders as a placeholder body (real actor bodies arrive with
the animation track), and opens the normal loot/transfer UI. **Clicking
a Pip-Boy Items row triggers its primary action** (#121): click a weapon
to equip it, an aid item to use it, a note or book to read it — the E
key and details-pane button still work as before.

## One-time setup

Prepared caches from before this wave are stale (this wave bumps both
the item-catalog and prepare revisions; the viewer will refuse them with
a "revision … is stale" error naming the fix). Re-prepare the three
cells this script uses:

```
cargo run-dev -- prepare --cell 00024512
cargo run-dev -- prepare --cell 00028138
cargo run-dev -- prepare --cell 0001a273
```

## A. Corner clearance — the wave-5 wedge route (#136) — Vault 101 Entrance

1. Launch:
   `cargo run-dev -- view --manifest .bevyout/cache/scenes/00024512/scene.ron`
2. Check the log for the erosion diagnostic (proof the fix is active):
   `nav erosion: polys <n> eroded <m> pinch-guard <k> relax-passes <r> protected <p>`
   and **no** `landmass validation failed` warning after it.
3. Open the console (backtick), place the player on the mesh and spawn
   the test agent: `player.setpos x 153.2`, `player.setpos y 37.6`,
   `player.setpos z -40`, `tna spawn`. Expected: `nav agent 0 spawned`.
   (In wave 5 this cell's agent could spawn; if you instead see
   `nav_mesh_invalid`, the erosion regressed — stop and report.)
4. Step aside so you don't block it: `player.setpos x 175`.
5. `tna goto 154 36.5 -34`, close the console. This is the exact route
   that wedged in wave 5 (`collision-blocked` → `stuck` partway to door
   00028579 and again ~1.2 m past it). Expected now: the capsule keeps
   visible daylight from wall corners, the door opens for it
   (`nav agent door wait 00028579` → `door resume 00028579`), and it
   reaches the far side: `tna status` reports `status=reached
   grounded=true stuck=false`. No `collision-blocked` or `stuck` lines
   in the log.

## B. Multi-mesh cell stays connected (#136 seam protection) — FranklinMetro02

6. Quit, then launch:
   `cargo run-dev -- view --manifest .bevyout/cache/scenes/0001a273/scene.ron`
7. Check the log: **two** `nav erosion:` lines (one per mesh), each with a
   nonzero `protected` count — the inter-mesh seam and door-link edges are
   excluded from erosion so the two meshes stay routable as one graph.
8. Console: `player.setpos x 9.6`, `player.setpos y 106`,
   `player.setpos z -73.1`, `tna spawn`, `tna goto -19 103.4 -59.5`, then
   `tnm`. Expected: the mesh triangles and the white route polyline
   appear, crossing the seam between the two meshes (this proves the
   route exists — during development, un-protected erosion made this
   target `unreachable`).
   *Known limit:* the agent does not physically complete this route — it
   collision-blocks near its spawn on both this build **and** the
   pre-wave master (verified side by side), an interior-collider issue
   unrelated to erosion, tracked as #148.

## C. Real note text in the Pip-Boy reader (#123) — any cell

9. Still in the viewer, console: `additem 000031c9` (Meresti Entry
   Password, a text-type NOTE). Expected: `added 1x Meresti Entry
   Password`.
10. Open the Pip-Boy (Tab), go to the note. Two routes, both must work:
   - Items → Misc → **click** the "Meresti Entry Password" row (#121's
     click-to-read), or
   - Data → Notes → select it and open it.
   Expected: the reader shows the actual note text — "The password to
   unlock the outer security door … is \"Nycteris\"." — not an empty
   page. (Before this wave every NOTE had `text: None`; sound-type
   holotapes intentionally still show no text.)

## D. Pip-Boy click = primary action (#121) — same session

11. Console: `additem 0000434f` (10mm Pistol), `additem 00015169 2`
    (2 Stimpaks).
12. Pip-Boy → Items → Weapons: **single-click** the "10mm Pistol" row.
    Expected: it equips directly (EQ marker, same as pressing E on the
    selected row). Click it again: unequips.
13. Items → Aid: click a "Stimpak" row. Expected: one Stimpak is
    consumed (`Used Stimpak: Restore Health` notice, count drops to 1).
14. Confirm the old paths still work: select a row and press E, and use
    the details-pane USE/READ button — both behave exactly as in wave 5.
15. Click a Misc row with no action (any junk item). Expected: it only
    selects; nothing triggers.

## E. Lootable authored corpse (#120) — Vault 101 Escape! cell

16. Quit, then launch:
    `cargo run-dev -- view --manifest .bevyout/cache/scenes/00028138/scene.ron`
17. Console: `activate 00054398`. Expected log: `corpse 00054398
    opened`, and the transfer UI titled **Grandma Taylor** opens with
    her authored inventory: Vault 101 Jumpsuit, Pip-Boy 3000, Pip-Boy
    Glove. (She is prepared as a corpse straight from the real ESM data
    — the starts-dead flag on her base record — not by name matching.
    Her body renders as a dark placeholder capsule until real actor
    bodies land with the animation track.)
18. Take all three items (right-click each, or the take-all binding
    shown in the footer), close with Esc. Reopen: `activate 00054398` —
    the corpse is now empty; your Pip-Boy Items shows the three items.
19. Persistence: console `save wave6corpse`, quit the viewer, relaunch
    with the save applied:
    `cargo run-dev -- view --manifest .bevyout/cache/scenes/00028138/scene.ron --save-slot wave6corpse`,
    then `activate 00054398` again. Expected: the corpse is still empty
    and the items are still in your inventory — lossless across
    save/reload.
20. Living-actor safety check: no other vault NPC in this cell became
    lootable; only authored-dead actors classify as corpses.

## F. Late additions: debug HUD and teleport (#151, #152) — any cell

21. Console: `tdi`. Expected: a top-left debug block appears — `Debug
    info: On`, live `player pos=(…)`, `cell=<formid> editor_id=…
    name=…`, and one `nav agent N …` line per spawned test agent —
    without overlapping the bottom diagnostics row. `tdi` again hides
    it.
22. Console: `tp 153.2 37.6 -40`. Expected: `tp: teleported player to
    (153.200, 37.600, -40.000).` — all three axes at once (no
    axis-by-axis falling).
23. Console: `tp 100 40 -55 00024511`. Expected: `tp: cell travel
    requested to 00024511 …`, the cell swaps to Vault 101 Atrium, and
    the player is placed at the given coordinates. (Coordinates are not
    ground-snapped — a mid-air target drops you, as in the original
    game's console.)
24. `tnm` overlay check (#138 follow-up): in a dark corridor, toggle
    `tnm`. Expected: the scene's brightness does not jump (exposure is
    locked while the overlay is visible), the floor texture stays
    visible through the subtly tinted triangles, and the white route
    polyline is crisp.
