# M9 wave 10 — manual acceptance script

What this wave shipped, in plain language: **one RPG inspection snapshot
feeds Pip-Boy, `showstats`, and BRP probes.** The Pip-Boy only formats
that snapshot. Save v9 is frozen. V.A.T.S. is unavailable until wave 8.

- Default level-1 SPECIAL-5 vitals: HP `200/200`, AP max `75` with no
  current AP, XP `0/200`.
- Radiation stages are 0/200/400/600/800, fatal at 1000.
- Data World shows integer game time from the same snapshot `showgametime`
  uses. It does not invent quests or a map.

Wave 8 (V.A.T.S.) is still blocked. `bevyout.vats_probe` must report
`available: false` / `planned_wave: 8`.

## 0. One-time setup

```
cargo run-dev -- prepare --cell 00017f37
```

Launch the viewer with the agent bridge (or use the in-game console `~`):

```
cargo run-dev -- view --manifest .bevyout/cache/scenes/00017f37/scene.ron --agent-bridge --agent-port 15702
```

Super-Duper Mart. Cell `00017f37`. Bridge examples use `curl` against
port 15702.

## A. Shared snapshot

1. `showstats`
   Expected: JSON with `schema_revision` 1, HP `200/200` (or current/max
   after earlier waves mutated health), `ap_available` false, `ap_max` 75
   at default SPECIAL, XP next 200 at level 1, `vats.available` false.
2. Open the Pip-Boy (`Tab`).
   Expected: header LVL/HP/AP/XP matches `showstats`. AP shows `—/75` (or
   the live max), not `85/85`. Caption remains `Player - Level 1` at the
   default sheet.
3. Stats RAD / Data World
   Expected: RADS line from the snapshot; World includes `GAME TIME` with
   integer milliseconds, matching `showgametime`.

## B. BRP probes

4. `bevyout.rpg_stats_probe`
   Expected: same player HP/AP/XP/SPECIAL as `showstats`.
5. `bevyout.active_effects_probe`
   Expected: the snapshot `effects` object.
6. `bevyout.vats_probe`
   Expected: `available: false`, `reason: unavailable`, `planned_wave: 8`.
   Do not look for a queued V.A.T.S. session.

## C. Mutation still projects

7. `modav endurance 1` then `showstats` / reopen Pip-Boy.
   Expected: max HP rises with the shared derived formula; Pip-Boy does
   not keep the factory 100/100.
8. `setav rads 200` (or `modrads` if present) then Stats RAD.
   Expected: stage 200 from the snapshot, not a Pip-Boy-local table.

## D. Save freeze

9. `save test-m9-w10` then `load test-m9-w10`.
   Expected: RPG sections round-trip. Unknown future RPGS tags are
   skipped on decode; missing HEAD is rejected. v1–v8 loads still default
   RPG rather than inventing radiation or perks.

## E. Milestone chain (VATS blocked)

The Total plan's steps 11–12 (queue/execute V.A.T.S., save during an
active session) stay **unavailable**. Continue from chem/limb/repair/
barter/crime/minigame/clock proofs already shipped in waves 3–9.
