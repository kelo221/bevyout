# M9 wave 6 — manual acceptance script

What this wave shipped, in plain language: **sneaking detection, ownership,
and crime now share one authority each.** Observers still use the existing
awareness component; taking an owned item is classified once; one crime
report yields one bounty and Karma change. Hidden/Caution/Danger is a HUD
label, not a second stealth state.

- Detection uses integer millimetres, millidegrees, basis points, and
  milliseconds. Light comes from prepared cell ambient, never the GPU.
- Faction-owned loot is legal only when the taker holds the required
  rank in a known faction. Runtime player membership is still empty.
- Unwitnessed theft marks the item stolen without bounty. Two witnesses
  do not double the bounty.
- `detectstate`, `crime`, `getkarma`, `modkarma`, and `setownership` are
  the visible runtime surface.

## 0. One-time setup

```
cargo run-dev -- prepare --cell 000151e3
```

Launch the viewer with the agent bridge (or use the in-game console `~`):

```
cargo run-dev -- view --manifest .bevyout/cache/scenes/000151e3/scene.ron --agent-bridge --agent-port 15702
```

Megaton player house. Bridge examples use `curl` against port 15702.

```
curl -X POST http://127.0.0.1:15702/ -H "Content-Type: application/json" \
  -d "{\"jsonrpc\":\"2.0\",\"id\":1,\"method\":\"bevyout.console.exec\",\"params\":{\"line\":\"detectstate\"}}"
```

## A. Detection surface

1. `detectstate`
   Expected: `hud=hidden` (or caution/danger if an NPC has already
   acquired you), `observers` lists live actors with `confidence_milli`,
   acquired target, and last player LOS/distance.
2. Walk into an NPC's view cone in a lit interior, then `detectstate`
   again. Confidence should rise in milli units. If they acquire you,
   `acquired` is `player` and the HUD projection becomes `danger` when
   that observer is hostile.
3. The sneak label (`[ HIDDEN ]` / `[ CAUTION ]` / `[ DANGER ]`) only
   renders while `HudSneaking` is true. This wave does not toggle sneak
   from a key; if the label is absent, `detectstate` is still the
   authority.

## B. Karma and crime ledger

4. `getkarma` — expected `karma 0` on a fresh player.
5. `modkarma -5` then `getkarma` — expected `-5`.
6. `crime` — expected `bounty=0 karma=-5 sequence=0` (or the next
   sequence if a crime has already been allocated). Reported CrimeIds
   are listed when present.

## C. Ownership

7. `setownership <world-item-ref> 0001a2b3`
   Expected: log `setownership <ref> owner=0001a2b3`. Picking that
   reference up should log `steal … owner 0001a2b3`. If a live NPC has
   LOS inside 40 m, `crime` bounty becomes 40 and karma drops another 5.
8. `setownership <world-item-ref> none`
   Expected: owner cleared. A later pickup is not theft.
9. Optional rank: `setownership <container-ref> 00022457 1`
   Faction-legal takes require player membership, which this viewer does
   not yet load. Expect steal unless you are testing the core cucumber.

## D. Persistence

10. `modkarma -5`, then `save testcrime`, quit, relaunch that slot.
    `getkarma` and `crime` must restore bounty/karma/sequence. Actor
    awareness restores from `AWRS` on the actor instance; HUD Hidden/
    Caution/Danger is recomputed and is not in the save.
    Format remains v9; missing `CRIM`/`AWRS` blobs default empty/zero.
