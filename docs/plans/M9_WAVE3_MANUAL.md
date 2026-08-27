# M9 wave 3 — manual acceptance script

What this wave shipped, in plain language: **chems, radiation, and
addiction now work.** Before it, eating a Stimpak or walking through a
radiation zone did nothing — the game had no effect engine. Now:

- `prepare` decodes every ingestible (ALCH) and its magic effects (MGEF)
  into a versioned `effects.ron` catalog — 71 ingestibles, 163 effects,
  13 addictive chems on Fallout 3 GOTY.
- Using a chem applies its authored effects: Stimpaks restore health,
  RadAway removes rads, Buffout buffs Strength/Endurance for four minutes,
  Jet buffs Action Points.
- Radiation is a real pool (0–1000 rads) with the vanilla SPECIAL
  penalties at 200/400/600/800 and death at 1000.
- Addictive chems roll against their authored chance on a deterministic,
  inspectable PRNG — the same seed always produces the same rolls.

One real-data finding worth knowing: RadAway's effect is authored as
**+50** on the Rads value ("RestoreRadiationLevel"). Positive magnitude
on Rads *removes* rads in vanilla semantics; this wave implements that
polarity rather than naively adding rads.

## 0. One-time setup

`openmw-effects-v2` preserves ingestible CTDA details, so re-run prepare even
if this cell was prepared by the earlier wave build:

```
cargo run-dev -- prepare --cell 000151e3
```

Expected new line among the catalog summaries (counts from GOTY):

```
effect catalog: 71 ingestibles, 163 effects, 13 addictive, 16 unresolved effect items, 47 conditioned effect items -> catalogs/<fingerprint>/effects.ron
```

Launch the viewer with the agent bridge (or use the in-game console `~`
and type the same commands):

```
cargo run-dev -- view --manifest .bevyout/cache/scenes/000151e3/scene.ron --agent-bridge --agent-port 15702
```

Megaton player house. The bridge examples below use `curl` against port
15702; in-game just type the quoted command text.

## A. Radiation: dose, thresholds, reversal, death

1. Baseline: `rads` → expect `0 rads (no penalties)`.
2. `addrads 300`, then `rads`.
   Expected log: `+300 rads -> 300`, then
   **`300 rads (endurance-1)`** — the first threshold is 200.
3. `addrads 550`, then `rads` → **`850 rads (strength-2, endurance-3,
   intelligence-1, agility-2)`** — thresholds stack cumulatively:
   200 costs END−1, 400 adds AGI−1, 600 adds STR−1, 800 adds INT−1 and
   another STR/AGI point.
4. Reversal: `removerads 400`, then `rads`.
   Expected: `-400 rads -> 450` and penalties drop back to the tiers
   your remaining rads still cross (`removerads` uses RadAway semantics).
5. Death boundary: `addrads 600` (or enough to reach 1000).
   Expected: `+… rads -> 1000 FATAL` and `rads` reports
   `1000 rads FATAL …`. `removerads 1000` returns to 0 and non-fatal.

## B. Chems: RadAway polarity, Buffout timed buffs

6. `additem 00015167 1` then `useitem <instance-id>` printed by the
   additem/use flow (or skip inventory: `addchem 00015167`).
   Expected: **`RadAway: -50 rads`** — the +50-authored magnitude
   *removed* 50 rads. Repeat until 0; extra doses do not go negative.
7. `addchem 00015163` (Buffout). Expected:
   `Buffout: 2 timed modifier(s), no addiction`, and `effects` shows two
   active effects — `special_strength magnitude 2` and
   `special_endurance magnitude 3`, `source: chem`,
   `remaining_ms` counting down from 240 000 (four minutes).
8. `player.getav strength`: the effective value adds the buff on top of
   base while the effect lives; after ~4 real minutes (or by watching
   `remaining_ms`) the effects expire and `effects` reports
   `0 active effect(s)`.

9. Verify projected values while Buffout is active: `player.getav strength`
   reports base +2, and `player.getav health` reports current health rather
   than maximum health. `player.getav action_points` exposes the projected AP
   maximum, including Jet's +30 modifier.
10. Rad-X resistance: `addchem 00015168`, then `addrads 100`.
    Expected: `+75 rads -> 75`; `player.getav rad_resist` reports `25`.
11. Stimpak without Fast Metabolism:
    `player.setav health 140`, `addchem 00015169`, `player.getav health`.
    Expected: `Stimpak: health 170, 1 conditioned effect(s) false`, then
    `health = 170`.
12. Stimpak with Fast Metabolism:
    `addperk 00094ebf`, `player.setav health 140`, `addchem 00015169`,
    `player.getav health`. Expected health is **176**. The real Stimpak has
    two mutually exclusive `HasPerk FastMetabolism` branches: 30 HP without
    the perk and 36 HP with it. Unsupported CTDA functions remain skipped.

## C. Jet addiction: deterministic rolls

13. Fresh viewer session (the PRNG seeds at startup; seed 0 default).
   `effects` → expect `rng at draw 0`.
14. Dose Jet repeatedly: `addchem 00015164` eleven times, watching the log.
    With the default seed the draw sequence is fixed: draws 0–8 fail,
    **draw 9 (390 bps < 2000 bps chance) addicts**, as does draw 10
    (201 bps). Expect `Jet: 1 timed modifier(s), ADDICTED` on the 10th
    dose, and `effects` shows
    `"addictions":[{"chem":"Jet","phase":"addicted","withdrawal_form_id":"00033067"}]`.
15. Same-seed reproducibility: restart the viewer, repeat step 14 — the
    addicted doses land on exactly the same draw indices.
16. Cure: `cureaddiction` → `cured 1 addiction(s)`; `effects` shows
    `0 addiction(s)` and the rng counter unchanged.

## D. Canonical seam: useitem consumes and applies

17. `player.setav health 140`, then `additem 00015169 2` → inventory has 2
    Stimpaks.
18. `useitem 0000000000000003` (instance ids allocate sequentially from the
    ledger; the first additem in a fresh session starts at id 3).
    Expected: `used item 0000000000000003` plus `Stimpak: health 170, 1
    conditioned effect(s) false`; inventory count drops to 1 and current
    health is 170. Using Aid from the Pip-Boy performs the same canonical
    consume-and-apply operation.

## E. Console reference

| Command | Effect |
| --- | --- |
| `rads` | current rads, threshold, penalty list, fatal flag |
| `addrads <n>` / `removerads <n>` | environmental dose / RadAway-semantics removal |
| `addchem <FormID>` | apply an ingestible without consuming inventory |
| `cureaddiction [FormID\|all]` | clear addictions |
| `effects` | active effects, chem dose timers, addictions, rng draw index |
| `[player.]useitem <ItemInstanceId>` | canonical consume + apply |

Real FormIDs used above: Buffout `00015163`, Jet `00015164`, Psycho
`00015166`, RadAway `00015167`, Rad-X `00015168`, Stimpak `00015169`;
Jet's withdrawal spell `00033067`.
