# PERF wave 1 manual — idle-frame quick wins

This wave removed work the viewer used to do **every frame even when nothing
was changing**: the disabled realtime-shadow path rewrote every candidate
light, four debug HUDs rewrote their `Text` every frame (one through an
exclusive `&mut World` system), AO/glow-card classification re-counted the
whole mesh set on a lossy count sentinel (and could miss a freshly spawned
glow card), and the metallic/dielectric/roughness clamps each rescanned
every material while engaged. All of it is now change-driven or
event-driven. **Nothing visible should be lost or changed** — this script
proves that on a real prepared cell, and also shows the toggles still do
exactly what they did before.

## Setup

No new prepare is needed; any previously prepared interior cell works. The
steps below use `000151e3` (the reference cell from AGENTS.md):

```text
cargo run-dev -- view --manifest .bevyout/cache/scenes/000151e3/scene.ron --agent-bridge --agent-port 15702
```

Wait for the `loaded ... with N placements` log line and the window to show
the scene. All console lines below can be typed in-game (backquote) or sent
over the agent bridge:

```text
curl -X POST http://127.0.0.1:15702/ -H "Content-Type: application/json" -d "{\"jsonrpc\":\"2.0\",\"id\":1,\"method\":\"bevyout.console.exec\",\"params\":{\"line\":\"<command>\"}}"
```

## 1. Debug info HUD still toggles and refreshes (#268)

1. `tdi` — expected: top-left block under the player-position line shows
   `Debug info: On` with live player pos, cell identity, and nav agent
   lines (if any agents are spawned).
2. Walk with WASD for a few seconds — expected: the position line updates
   in small discrete steps (it refreshes at ~8 Hz, not every frame; before
   this wave it updated every frame). The display is otherwise identical
   in content to before.
3. `tdi` again — expected: the block flips to `Debug info: Off` exactly
   once and stays static.
4. `tdt` on/off — expected: the whole diagnostic UI (including the debug
   info block) still folds/unfolds as before.

## 2. Material clamps behave exactly as before (#269)

1. `getrender metallic` — expected: prints the current value (`1`).
2. `setrender metallic 0` — expected: metallic highlights on shiny props
   flatten out (same visual as before the wave); `getrender metallic`
   echoes `0`.
3. `setrender metallic 1` — expected: the original highlights return
   exactly (value-exact restore from baselines).
4. `setrender dielectric_specular 0` then `setrender dielectric_specular 1`
   — expected: specular sheen disappears, then returns exactly.
5. `setrender roughness_scale 1.5` — expected: surfaces look visibly more
   matte. `setrender roughness_scale 1` — expected: original roughness
   returns exactly.
6. With `metallic 0` still (or set it again) travel through a door to a
    neighbor cell and back, or wait for preloaded content — expected:
    newly visible materials load already clamped; re-enabling restores
    them too.

## 3. AO and glow cards behave as before, without per-frame scans (#270)

 1. `setrender ao 0` — expected: baked quick-AO vertex shading
    lifts toward white on AO'd meshes. `setrender ao 0.5` —
    partial. `setrender ao 1` — back to authored values exactly.
 2. Look at any light-glow geometry while doing steps above — expected: no
    `lightglow*` billboards are ever visible (classification now happens on
    entity events rather than a count sentinel; the remove+add blind spot
    is closed by construction).

## 4. Realtime shadows still opt in on one startup light (#267)

 1. `setrender realtime_shadows 1` — expected: exactly one camera-relevant
    startup-cell point light casts realtime shadow maps (dynamic props
    under it gain realtime shadows). `setrender realtime_shadows 0` —
    expected: they disappear.
 2. Leave realtime shadows off (default) and idle for a minute — expected:
    no per-frame shadow churn in the log; the disabled path now performs
    zero light writes per frame.

## 5. Idle sanity

 1. With everything at defaults, let the viewer idle on the prepared cell
    — expected: the scene renders normally, controls stay responsive, and
    nothing flickers or re-applies periodically. (This wave's CPU-side
    wins are per-frame-work removals; they are asserted mechanically by
    the new change-detection probe tests and can be re-measured with the
    viewer's performance snapshot if wanted.)

Any deviation = note it with the exact command + observed vs expected, and
file it as a follow-up issue rather than amending the wave silently.
