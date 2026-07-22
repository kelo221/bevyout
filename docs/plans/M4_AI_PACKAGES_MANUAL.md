# M4 AI-packages wave — manual acceptance

## What this wave shipped

Actors now have a real behavior layer. Before, an NPC only moved when you drove
it by hand with `tna`. This wave adds:

- **AI package engine** (#193/#194/#195): which package an actor should run
  (priority + schedule + conditions), its lifecycle (start/tick/preempt/
  complete/fail/retry, persisted), and resolution of package locations/targets
  to world points — all as pure, unit-tested policy.
- **Five package families** (#196/#197): Travel, Patrol, Idle, Eat, Sleep.
- **Two more** (#198): Follow (distance band with hysteresis) and Sandbox
  (deterministic bounded wander), plus the wander-no-open-doors gate.
- **Perception** (#116): disposition, faction hostility, and target awareness.
- **Key-aware locked doors** (#185): a routing NPC opens a locked door only if
  it holds the key.

Console surface: `showpackages`, `runpackage`, `perception`, `giveitem`,
`setlock` (+ `tna`).

## One-time setup (required — schema bumped 18→19)

```sh
cargo run-dev -- --config .bevyout/config.toml prepare SuperDuperMart \
  --converter native --actor-animation-converter native
```
Expect `package catalog: 3021 packages, 0 unsupported type ...` and
`actor animation catalog: 11 actor mappings ...`. Then launch:
```sh
cargo run-dev -- view --manifest .bevyout/cache/scenes/00017f37/scene.ron \
  --agent-bridge --agent-port 15702
```

## Steps

1. **Package selection / lifecycle / resolution (#193/#194/#195).** Console:
   `showpackages 0005cf10`. Expect the actor's 2 packages listed in priority
   order; `DefaultPatrolWeaponDrawn` (Patrol) **selected**, `DefaultSandbox…`
   **rejected: lower-priority**; `lifecycle phase=running … step=0`; and a
   resolution line. Note: this actor's Patrol location reports
   `unresolved: near-linked-reference location has no linked reference` — a
   correct **deterministic diagnostic** (not a crash), because patrol-marker
   subrecords aren't decoded yet (**#213**). That is expected here.

2. **Perception / hostility (#116).** `perception 0005cf10 player`. Expect e.g.
   `disposition=35 hostility=hostile rule=aggressive | awareness: distance=…
   los=true confidence=1.00 acquired=player`. Move the player far / behind a
   wall and re-run: confidence drops, target is lost after the forget timer.

3. **Key-aware locked doors (#185).** In a cell with a lockable door and an
   NPC (e.g. Megaton `000151e3`, door `ShackExitDoorReg01` 0x0007b279):
   `setlock 0007b279 25 4660`, then `tna bind 0x0008f6ae` + `tna goto` past the
   door → `status=unreachable`. Then `giveitem 0008f6ae 00004660` +
   `tna goto` again → the agent walks to the door, log shows
   `door … opened (scripted, nav agent)`, `status=reached`. A second keyless
   agent stays `unreachable` — lock is per-(door, actor).

4. **Package-driven movement (#196/#197/#198).** `tna bind 0x0005cf10` then
   `runpackage 0005cf10`. On this cell it returns `unresolved_point` (the #213
   marker gap) rather than moving — the family refuses to fake a route. To see
   the walk/run/turn the families *request* (nav + animation + correct facing),
   drive the same seam directly: `cam follow 0x0005cf10`, `tna goto 38 96.46
   -55` — the actor walks its route with locomotion clips, facing travel. Once
   #213 lands, `runpackage` drives this autonomously.

## Known limitation
Autonomous package movement on authored cells needs patrol-marker /
editor-location subrecord decoding — tracked as **#213**. The engine,
perception, doors, and the movement/animation the families request are all
verified; only the package→location data for these specific package types is
deferred.
