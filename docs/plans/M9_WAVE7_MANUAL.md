# M9 wave 7 — manual acceptance script

What this wave shipped, in plain language: **lockpicking and terminal
hacking are integer, headless sessions.** Bobby pins break through the
canonical item ledger. Doors only unlock after a successful step.
Saving is blocked while a session is open. The console can start,
inspect, and step both minigames without a second RNG or UI-owned
inventory.

- Pick angles and cylinder rotation are millidegrees. Stress is an
  integer. Force-lock chance is basis points.
- Hacking uses a synthetic word board in this wave (`VENT` is the
  password). Bethesda terminal text is not checked in.
- Owned locks can report trespass through the Wave 6 crime ledger.

## 0. One-time setup

```
cargo run-dev -- prepare --cell 000151e3
```

Launch the viewer with the agent bridge (or use the in-game console `~`):

```
cargo run-dev -- view --manifest .bevyout/cache/scenes/000151e3/scene.ron --agent-bridge --agent-port 15702
```

Megaton player house. Door `00024657` is an in-cell Door. Bridge
examples use `curl` against port 15702.

```
curl -X POST http://127.0.0.1:15702/ -H "Content-Type: application/json" \
  -d "{\"jsonrpc\":\"2.0\",\"id\":1,\"method\":\"bevyout.console.exec\",\"params\":{\"line\":\"help lockpick\"}}"
```

## A. Grant pins and lock a door

1. `additem a 5`
   Expected: five bobby pins (`0000000a`) on the player.
2. `setlock 00024657 25`
   Expected: `setlock 00024657 level 25`.
3. `lockpick 00024657`
   Expected: an active session with `difficulty=25`, integer
   `pick_angle_milli` / `cylinder_milli` / `stress`, and `bobby_pins=5`.

## B. Save is blocked while picking

4. `save testlock`
   Expected: failure mentioning `minigame save deferred`.
5. `lockpick cancel`
   Expected: session cancelled; door still locked (`setlock` level 25
   still in effect until a later success).
6. `save testlock` after cancel should be allowed (no minigame error).
   Delete or overwrite that slot if you do not want to keep it.

## C. Unlock by sweet-spot torque

7. `lockpick 00024657`
8. Read `difficulty` from the status JSON. Sweet-spot millidegrees are
   `difficulty * 900 - 45000` (for 25: `-22500`).
9. `lockpick angle -22500`
10. `lockpick torque 1000`
    Expected: `unlocked=true`, `bobby_pins` still 5, door lock cleared.
11. `setlock 00024657 25` then `lockpick 00024657` and `lockpick force`
    if you want the inspectable force-chance path. Chance bps is
    `skill*100 - difficulty*50 + 500` clamped to 10000.

## D. Persistence after success

12. Unlock the door again if needed, then `save testpicked`, quit, and
    relaunch with that slot.
    Expected: `00024657` stays unlocked. The active minigame is not in
    the save; you must start a new session to pick again.

## E. Synthetic hacking

13. `hackterminal 00024657`
    (Any placement root works; this wave does not decode TERM records.)
    Expected: board words include `VENT`, `DOOR`, `LOCK`, `SAFE`,
    `KEYS`; `attempts_remaining=4`.
14. `hackterminal guess DOOR`
    Expected: likeness 0, attempts 3, still locked.
15. `hackterminal guess VENT`
    Expected: terminal unlocked.
16. Start again and guess four wrong words (`DOOR`, `LOCK`, `SAFE`,
    `KEYS`). Expected: `locked_out=true`, attempts 0.

## F. Optional trespass

17. `setownership 00024657 0001a2b3` then pick/force the lock while an
    NPC has LOS. Expected: `crime` bounty 40 and a trespass report.
    Unwitnessed success still unlocks without multiplying bounty.
