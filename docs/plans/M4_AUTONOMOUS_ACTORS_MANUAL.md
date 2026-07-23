# M4 autonomous-actors wave — manual acceptance (#215, #218, #224, #225)

## What this wave shipped, in plain language

Before this wave, launching a cell showed raiders standing still: making one
walk a patrol required a human to type `tna bind <index> <formid>` then
`runpackage <formid>` by hand, one actor at a time, capped at 4 concurrent
agents, and the actor twitched between idle and running (~25 times a second)
instead of walking smoothly — because animation clips were off unless
`prepare` was run with an extra flag.

This wave makes it work like a game:

- **Animation clips are on by default.** `prepare` now builds actor animation
  clips in pure Rust with no extra flag and no Blender step (#225).
- **The idle/run twitch is fixed.** The achieved speed fed to the locomotion
  classifier is smoothed before classification, so it no longer swings across
  every threshold every tick (#224).
- **Any number of actors can be nav agents at once**, not just 4 (#215).
- **Every alive actor binds a nav agent and starts its package automatically**
  the moment it is projected into the cell — no console command at all
  (#218). `tna bind`/`runpackage` still work exactly as before, as debug
  tools, and never fight the automatic driver.

## One-time setup

`prepare` needs to run once for SuperDuperMart. No extra flag — native actor
animation clips are now the default:

```sh
cargo run-dev -- prepare --cell 00017f37
```

Expect the summary line to end with a non-zero ready-clip count, e.g.:

```
actor animation catalog: 11 actor mappings, 2 sets, 1380 ready clips, ...
```

(Confirmed during this wave's acceptance: exactly `1380 ready clips`, the same
number the plan's kickoff prompt recorded for `--actor-animation-converter
native` explicitly — now that is what a plain `prepare` does.)

## Watch it work — SuperDuperMart raiders (00017f37)

Real raider FormIDs confirmed against the prepared catalog: `00041600`,
`00041604`, `00041606`, `0004160c`, `00041611` (all enabled, all patrol). A
sixth actor, `0006d921`, has a Travel package instead of Patrol.

1. Launch the viewer with the agent bridge, **no console commands yet**:
   ```sh
   cargo run-dev -- view --manifest .bevyout/cache/scenes/00017f37/scene.ron \
       --agent-bridge --agent-port 15702
   ```
2. Watch the log (or `viewer_logs`/a piped file) for lines like:
   ```
   autonomous package driver: bound + started actor 00041600
   autonomous package driver: bound + started actor 00041604
   autonomous package driver: bound + started actor 00041606
   autonomous package driver: bound + started actor 0004160c
   autonomous package driver: bound + started actor 00041611
   ```
   These appear within the first second after the cell loads, before you have
   typed anything. (One other actor in this cell, `0005cf10`, logs a `warn`
   instead — `start 0005cf10 skipped: near-linked-reference location has no
   linked reference` — a genuine per-actor data gap (no authored `XLKR`),
   not a wave bug; the driver skips it and moves on rather than failing.)
3. Confirm animation clips are actually playing — grep the log for:
   ```
   actor-animation play state=run clip=mtfastforward__2 set=animation-set-...
   actor-animation play state=walk clip=mtforward__2 set=animation-set-...
   actor-animation play state=turn_left clip=mtturnleft__2 set=animation-set-...
   ```
4. Use `runpackage <ref> status` (a **debug read**, not a start — the package
   is already running) to see a marker advancing on any of the five raiders,
   e.g.:
   ```sh
   curl -X POST http://127.0.0.1:15702/ -H 'Content-Type: application/json' \
       -d '{"jsonrpc":"2.0","id":1,"method":"bevyout.console.exec","params":{"line":"runpackage 00041600 status"}}'
   ```
   Expect something like:
   ```
   runpackage 00041600: patrol package 00023619 phase=running step=routing marker=1/2 target=(19.04,96.46,-89.83) ...
   ```
5. Confirm the actor is actually walking, not frozen: `00041600.getpos` twice,
   a few seconds apart. The position changes between calls with no `tna
   goto`/`setpos` ever issued — this is the autonomous package driving real
   nav movement.
6. Confirm the locomotion fix: grep the log for `nav actor locomotion` lines.
   Every state-change timestamp for the same agent should be **seconds**
   apart (a real walk/run/turn change), never a rapid burst — the opposite of
   the pre-#224 idle↔run flap, which fired roughly every 40 ms.

## What real-data acceptance found (this wave)

Confirmed live against SuperDuperMart with no console commands typed before
step 2 above:

- All five named raiders (`00041600`, `00041604`, `00041606`, `0004160c`,
  `00041611`) auto-bound and started their Patrol package within the first
  frame after cell load; `0006d921` auto-started its Travel package (which
  completed on its own, `phase=completed`).
- `actor-animation play state=run clip=mtfastforward__2 ...` and `state=walk`/
  `state=turn_left`/`state=turn_right` all logged — native clips are playing.
- `nav actor locomotion` transitions for the same agent landed seconds apart
  (idle→walk→run on spawn, then the next change 8+ seconds later as the
  actor's actual speed changed) — no sub-100 ms idle/run flap anywhere in the
  run.
- `runpackage <ref> status` on all five raiders showed `phase=running`
  patrol packages with `marker=1/2` advancing; `getpos` sampled twice a few
  seconds apart moved from `(18.55, 96.45, -89.52)` to `(18.23, 96.45,
  -89.32)` with zero console movement commands issued.

**One follow-up worth its own issue, not fixed here (out of this wave's
scope):** a brief (~300 ms) `turn_left ↔ turn_right` oscillation was observed
on one raider mid-route (`agent 1641v0`, ~14 rapid alternations). This is the
*yaw-rate* turn classifier, not the *achieved-speed* idle/run classifier
#224 targeted and fixed — `smooth_achieved_speed` only smooths the speed
input, by design (see `locomotion.rs`'s module doc). It self-resolved back
into a normal walk within a third of a second and did not recur elsewhere in
the run; it looks like a facing-correction wobble while routing around a
tight patrol-marker corner rather than a sustained flap. Recommended
follow-up: consider the same EMA treatment for yaw rate if it turns out to be
visually noticeable in play.

## Gates run before this manual

`cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo
test-dev --lib` (1313 passed), `cargo test-dev --test features` (561/561
scenarios, `fail_on_skipped()`), and the real-data run above.
