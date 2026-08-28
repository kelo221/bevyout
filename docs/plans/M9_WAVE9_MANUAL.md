# M9 wave 9 — manual acceptance script

What this wave shipped, in plain language: **one integer game clock owns
wait, restock, cell reset, and fast travel.** Lighting hours are a
projection. `settime` still previews the sky; it does not write the
save clock. `passtime`, `fasttravel`, `resetcell`, and `showgametime`
live on the existing world console provider.

- Clock unit is whole milliseconds. Timescale is game seconds per real
  second. Remainder is integer, not float.
- 72 hours is `259200000` ms. That boundary is exclusive then
  inclusive: `71:59:59.999` does not restock or reset; `72:00:00.000`
  does.
- Occupied cells do not reset. Unique and player-owned holders keep
  their items. Fast travel validates before it advances time.

Wave 8 (V.A.T.S.) is still blocked. Do not look for `vatsstate`.

## 0. One-time setup

```
cargo run-dev -- prepare --cell 000151e3
```

Launch the viewer with the agent bridge (or use the in-game console `~`):

```
cargo run-dev -- view --manifest .bevyout/cache/scenes/000151e3/scene.ron --agent-bridge --agent-port 15702
```

Megaton. Cell `000151e3`. Bridge examples use `curl` against port 15702.

```
curl -X POST http://127.0.0.1:15702/ -H "Content-Type: application/json" \
  -d "{\"jsonrpc\":\"2.0\",\"id\":1,\"method\":\"bevyout.console.exec\",\"params\":{\"line\":\"help showgametime\"}}"
```

## A. Lighting preview is not save authority

1. `showgametime`
   Expected: `game_ms` is `0` (or the restored clock), calendar
   `2277-10-23 00:00` at epoch, integer `timescale` (viewer default `0`
   so frames do not advance while you inspect lighting).
2. `settime 12`
   Expected: `gettime` reports hour 12. `showgametime` `game_ms` is
   unchanged.
3. `passtime 1`
   Expected: `game_ms` grows by `3600000`. Lighting hour becomes the
   integer clock's projection (`1` from epoch, or twelve hours later
   if you started at noon on the integer clock).

## B. Wait processes due tasks in order

4. `addchem 15167`
   (Buffout, if the prepared catalog has it) or any timed chem you
   already verified in wave 3. Note remaining duration from `effects`.
5. `passtime 1` again until that chem expires.
   Expected: `effects` no longer lists the timed buff. Withdrawal still
   follows the wave-3 addiction machine; this wave only supplies the
   clock.

## C. Cell reset refuses occupancy, then runs vacant

6. `resetcell 000151e3`
   Expected: either a generation increment if the cell is vacant and
   due, or `reset_rejected` if occupied / not due / already applied.
   Re-running the same due event must stay `AlreadyApplied`.
7. Unique and player-owned containers must keep their stacks. Do not
   expect a unique NPC to vanish.

## D. Fast travel validates, then advances, then arrives

8. `fasttravel 000151e3 1`
   Expected: clock advances one hour, then a cell-travel request for
   `000151e3`. The command does not invent a second travel pipeline.
9. Interior / encumbered / combat / radiation / undiscovered blocks are
   core-tested; the console path here assumes a discovered destination
   and all-clear evidence.

## E. Persistence

10. `passtime 12` then `save testclock`. Quit and relaunch with that
    slot.
    Expected: `showgametime` restores the integer milliseconds and
    calendar. Lighting hour matches the projection. Encounter-zone
    lock (if you entered one in a later cell) stays on the LIFE
    snapshot and does not reroll.
