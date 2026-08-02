# M6 wave 7 — bounded exterior route manual

This manual freezes the current-data six-cell Super-Duper Mart to Megaton
protocol and its safety ring. It is an acceptance script, not a claim that
gate #87 has passed. The route data is current v21 Fallout 3 data; actor
crossing, a live persistent door, a route water crossing, repeated-loop
budgets, and the final gate remain explicitly unverified below.

## Frozen current-data baseline

Run the commands from the configured source checkout. The authoritative
current-data inputs are:

- Config: `C:\Users\V\Projects\Rust\bevyout\.bevyout\config.toml`.
- Source plugin: `C:\Program Files (x86)\Steam\steamapps\common\Fallout 3 goty\Data\Fallout3.esm`.
- Prepared revision: `prepare-v21-m6-worldspace-lod-imad-screen-fx`.
- Source plugin fingerprint: `d9fb0a33af495ddb43992b96ea74f2741b123fefdb1fcdcea28096f7649b0d06`.
- Prepared manifest/content fingerprint: `24efdfcef26d1ebb3d347c976da6c85cd8a17e313b8a22c2709ff90b180941d0`.
- Current native route selectors, exactly as selected for the preflight:

  ```text
  00000c49 00000c4a 00000c4b 000010d5 00001245 00000c67 00000c68 00000c69 00000c6a 00000c6b 000010eb 000010ec 000010ed 00000c4c
  ```

The six-cell traversal order is `00000c49` → `00000c4a` → `00000c4b` →
`00000c4c` → `000010d5` → `00001245`. The other eight selectors are the
current safety-ring cells. Selector order does not change the sorted
`--list-only` output.

The authoritative current-data preflight was run in the main checkout with
the configured native pipeline and returned `14 cells valid, 0 stale`; native
preparation completed 14/14. Reproduce that preflight before an acceptance
run. `--check-fingerprints` is report-only. An isolated worktree can instead
report `0 valid, 14 stale` because ignored `.bevyout` cache/config state is
not shared between worktrees; that is an isolated-worktree limitation, not a
current-data route result. Do not fabricate a local cache copy or reuse stale
fixture values.

```text
cargo run-dev -- prepare --help
cargo run-dev -- prepare 00000c49 00000c4a 00000c4b 00000c4c 000010d5 00001245 00000c67 00000c68 00000c69 00000c6a 00000c6b 000010eb 000010ec 000010ed --list-only
cargo run-dev -- prepare 00000c49 00000c4a 00000c4b 00000c4c 000010d5 00001245 00000c67 00000c68 00000c69 00000c6a 00000c6b 000010eb 000010ec 000010ed --check-fingerprints
```

Expected `--list-only` output contains all 14 selectors, sorted by FormID,
including `00000c49 SuperDuperMartExterior`, `000010ec MegatonTown`, and
`00001245 MegatonMainGate`. Expected fingerprint output is 14 `valid` lines
and the summary `14 cells valid, 0 stale` in the main checkout.

## Clean and warm preparation recording

Do not delete the evidence cache to create a clean run. Use a disposable,
explicit cache directory and record the machine, GPU, OS, build profile,
converter setting, source fingerprint, cache path, elapsed seconds, cache
bytes, assets built/reused, recoverable diagnostics, and native converter
invocations for both runs.

1. From `C:\Users\V\Projects\Rust\bevyout`, select an explicit disposable
   cache and make it empty. The target below is derived data only:

   ```powershell
   $cleanCache = 'C:\Users\V\Projects\Rust\bevyout\.bevyout\m6-route-clean-cache'
   if (Test-Path -LiteralPath $cleanCache) { Remove-Item -LiteralPath $cleanCache -Recurse -Force }
   New-Item -ItemType Directory -Force -Path $cleanCache | Out-Null
   ```

2. Run the clean preparation once with the exact 14-selector list, native
   conversion, and one worker. Record the deterministic completion line,
   elapsed time, cache bytes, and build/reuse counters:

   ```text
   cargo run-dev -- --config C:\Users\V\Projects\Rust\bevyout\.bevyout\config.toml prepare 00000c49 00000c4a 00000c4b 00000c4c 000010d5 00001245 00000c67 00000c68 00000c69 00000c6a 00000c6b 000010eb 000010ec 000010ed --cache-dir $cleanCache --actor-animation-converter native --jobs 1
   ```

   Expected: all 14 selected cells complete without an unassigned failure;
   the recorded result must include the actual `done/failed` line, not a
   budget inferred from package estimates.

3. Run the identical command again without `--force` and record the warm
   elapsed time, cache bytes, assets reused/built, diagnostics, and converter
   invocation count. This is the warm preparation measurement, not the
   report-only check:

   ```text
   cargo run-dev -- --config C:\Users\V\Projects\Rust\bevyout\.bevyout\config.toml prepare 00000c49 00000c4a 00000c4b 00000c4c 000010d5 00001245 00000c67 00000c68 00000c69 00000c6a 00000c6b 000010eb 000010ec 000010ed --cache-dir $cleanCache --actor-animation-converter native --jobs 1
   ```

   Then run the report-only check against the same cache:

   ```text
   cargo run-dev -- --config C:\Users\V\Projects\Rust\bevyout\.bevyout\config.toml prepare 00000c49 00000c4a 00000c4b 00000c4c 000010d5 00001245 00000c67 00000c68 00000c69 00000c6a 00000c6b 000010eb 000010ec 000010ed --cache-dir $cleanCache --actor-animation-converter native --jobs 1 --check-fingerprints
   ```

   Expected: the warm run skips already-valid work and the check returns 14
   valid, 0 stale. A converter invocation count of zero applies only to the
   viewer/runtime; preparation may invoke the native converter on a clean
   cache. Record the actual values for the clean and warm rows separately.

### Recorded v21 native preflight — 2026-08-02

The frozen command was run from the main checkout against
`C:\Users\V\Projects\Rust\bevyout\.bevyout\m6-w6c-route-clean-20260802` on
Windows/dev/native with `--jobs 1` at commit `f600328b`:

- clean elapsed: `288.3 s`; completion: `14 done, 0 failed`;
- warm elapsed: `8.9 s`; completion skipped all 14 valid cells;
- report-only check: `14 cells valid, 0 stale`;
- final cache: `5,773` files, `1,697,002,779` bytes;
- final deterministic batch line: `assets reused 0, built 0, rebuilt 0`,
  `physics reads 305`, `physics hits 153`.

The clean run also reported native LOD reuse (`sources=1608`,
`reused=1608`, `converted=0`, `failed=0`, `assets=1608`) and two recoverable
non-renderable source-NIF diagnostics in route cells. They did not fail a
selected cell. These are the current preparation measurements; the older
timed-out jobs=1/jobs=4 attempts and PR #261's historical four-worker figures
remain diagnostic only.

## Exact fixture boundaries

### Actor and navigation

The current source actor fixture is in route cell `00000c67`:

- reference `000638e8`;
- base `0001cf73` (`CrEyebotEnclave`);
- source `MODL`: `Creatures\\Eyebot\\Skeleton.nif`.

The prepared dynamic identity is reference `407784`, base `118643`, but this
prepared route has no actor asset or actor catalog. Therefore actor binding,
cross-cell pathing, unload/reload continuity, and runtime actor navigation are
`not_run`, blocked by the existing #10/W3 dependency. A disposable `tna`
capsule, if used for a nav-surface smoke check, is not evidence for this
source actor and must not be reported as actor acceptance.

### Water

All 14 selected route/safety-ring cells have no actual water FormID or water
surface. In particular, the sentinel height in `000010ec` is not water and
must not be used as a route water crossing.

Use this separate current-data water fixture when the water/runtime lane is
ready:

- cell `00001262`, `PotomacMirelurkNest02`, grid `(3,-2)`;
- raw `XCWT` `0007e421`;
- raw `XCLW` `10750.0`;
- prepared water form `0007e421`;
- prepared water height `153.57143`.

This is a separate prepared water fixture, not a claim that the six-cell
route contains water. Water entry/exit and breath acceptance are `not_run`
until the W4-C runtime integration is available. If the fixture is prepared
for that later run, use its own manifest and record `waterstate`; do not
append it to the six-cell route selector list or claim route water evidence.

### Door

The 14 selected route/safety-ring cells contain zero DOOR references and zero
door edges. Do not invent a route door. The only frozen door boundary is this
source-only fallback:

- persistent source cell `00002db4`;
- reference `00003b24`, base `00019496` (`MegatonMainGate01`);
- EDID `MegatonExteriorGateRef`;
- authored `XTEL` target `00003b2c` in cell `00000a96`;
- authored arrival position `(-3037.3999,-22132.5332,13000.5234)`;
- authored arrival rotation `(0.0,-0.0,-3.0297887)`;
- source transform `(-3008.5171,-21976.3535,12971.8740)`;
- source rotation `(0.0,-0.0,0.1090063)`.

This fallback is source-only. It cannot be live-accepted until W4-C makes
the persistent door executable and integrates its travel/save behavior. Do
not run or report `activate` on this fallback as a route-door result.

### Environment and weather

Freeze the environment identities from current data:

- worldspace `0000003c` (`Wasteland`);
- climate `00017907` (`WastelandClimate`);
- current weather `00064609` (`WastelandClear`).

Use the current weather ID in every transcript; do not reuse an older weather
value:

```text
environment status
settime 18
setweather 00064609 0
environment status
weather clear
```

Record the resolved worldspace, climate, weather, time, and any transition
status. Environment output is not a substitute for water, actor, door, or
frame-budget evidence.

## Route launch and baseline

1. Launch the prepared Mart manifest with physics and the bridge. Do not pass
   `--worldspace-lod`; far-worldspace tiles are optional presentation polish
   and are not part of this gameplay route:

   ```text
   cargo run-dev -- view --manifest C:\Users\V\Projects\Rust\bevyout\.bevyout\cache\scenes\00000c49\scene.ron --agent-bridge --agent-port 15726 --trace-seconds 900
   ```

2. Capture these baseline commands before moving:

   ```text
   worldstream status
   worldstream cells
   nav exterior
   nav borders
   environment status
   lights streamed
   waterstate
   worldstream presentation
   worldstream summary
   ```

   Record the active grid, request/ready/eviction counters, collision-ready
   state, failures, cancellations, stale completions, resident roots,
   colliders, streamed lights, package-byte estimate, and the explicit status
   of each unavailable diagnostic. `peak_package_bytes_estimate` is not
   process memory. Actor crossing, persistent-door execution, route water,
   and numeric performance budgets remain unrun at this point.

## Six-cell traversal and reversal

The route cells and grids are:

| Order | Grid | Cell | Role |
| --- | --- | --- | --- |
| 0 | `(4,-5)` | `00000c49` | Super-Duper Mart start |
| 1 | `(3,-5)` | `00000c4a` | route cell |
| 2 | `(2,-5)` | `00000c4b` | route cell |
| 3 | `(1,-5)` | `00000c4c` | route cell |
| 4 | `(0,-5)` | `000010d5` | route center |
| 5 | `(-1,-5)` | `00001245` | Megaton Main Gate end |

The deterministic boundary probe below avoids a physics tick starting below
the next authored slope. It proves streaming/collision handoff only; it is
not ordinary keyboard traversal and does not prove actor navigation.

Automation boundary: there is no high-level console command for holding a
movement key, and a tool-level W/A/S/D tap is diagnostic input only. The raw
runtime-write BRP surface does, however, expose Bevy's reflected keyboard
message path for a separate synthetic-input diagnostic. Query the primary
`bevy_window::window::Window` entity, then use the MCP `brp_call` wrapper (or
the equivalent raw bridge JSON-RPC request) with these parameters. Replace the
placeholder with the numeric `u64` entity returned by the query:

```json
{
  "method": "world.write_message",
  "params": {
    "message": "bevy_input::keyboard::KeyboardInput",
    "value": {
      "key_code": "KeyA",
      "logical_key": {"Character": "a"},
      "state": "Pressed",
      "repeat": false,
      "text": null,
      "window": "<primary-window-entity>"
    }
  }
}
```

Keep the `Pressed` lease active while sampling lightweight position/status
commands. End every lease with the same payload using `"state": "Released"`
and then write `bevy_input::keyboard::KeyboardFocusLost` with a null value as
cleanup. A fresh v21 viewer held `KeyA` for five seconds and moved the real
player from `x=263.3230` to `x=254.7199`; the matching release/focus-loss pair
stopped movement. Record this as `synthetic_input_measured`, never as ordinary
OS-input acceptance. The measured ordinary pass still requires a focused OS
keyboard with keys held long enough for fixed ticks to advance; otherwise keep
ordinary-input fields as `not_yet_sampled`.

A second fresh diagnostic started at `x=240.0` in `(4,-5)`, held `KeyA` for
five seconds, and crossed to `(3,-5)` at `x=228.2097`; after release and focus
cleanup, a five-second `KeyD` hold returned to `(4,-5)` at `x=249.3505`. The
closed trace recorded `requests=8`, `evictions=2`, `resident_cells=7`,
`peak_resident_cells=9`, and zero failures/cancellations/stale completions.
This is useful synthetic physics-handoff evidence, but it remains
`synthetic_input_measured` and does not replace focused physical OS input.

1. At `(4,-5)`, record the baseline, then move through the remaining five
   route cells in this exact order. After each command, wait for the cell to
   become collision-ready and record `worldstream status`,
   `worldstream presentation`, and `worldstream summary`:

   ```text
   tp 180 177 275.31
   tp 120 187 275.31
   tp 60 197 275.31
   tp 10 194 275.31
   tp -50 181 275.31
   ```

   Expected grids are `(3,-5)`, `(2,-5)`, `(1,-5)`, `(0,-5)`, and
   `(-1,-5)`. At every stop, the current cell remains collision-ready, no
   selected cell fails, stale completions remain zero for a passing run, and
   terrain presentation remains `full_land_mesh`. Record actual counts;
   do not reuse an earlier package or memory number.

2. Reverse from Megaton to the Mart using the exact reverse sequence:

   ```text
   tp 10 194 275.31
   tp 60 197 275.31
   tp 120 187 275.31
   tp 180 177 275.31
   ```

   Record the same fields at each boundary. A valid reversal has no duplicate
   cell root, no stale completion resurrecting an evicted generation, no
   premature collision teardown, and no residual cell-owned entities after
   an eviction. This probe is not a substitute for the later ordinary-input
   route pass.

3. Repeat the out-and-back route with ordinary movement input when the W7
   runtime dependencies are available. Do not use `tp` during that measured
   pass. At each boundary record request-to-ready time, collision-ready time,
   transition frame time, active grid, resident count, root/collider/light
   counts, failures, cancellations, stale completions, and the return-anchor
   result. If a deterministic `tp` re-anchor is needed because the physics
   start is below the steep authored slope, mark that traversal discontinuity
   and do not count it as uninterrupted ordinary traversal.

## Synthetic save/reload checkpoint

Create the one frozen command-created checkpoint at the route center:

```text
player.setpos x 1
player.setpos y 193.98508
player.setpos z 275.31247
save m6-route-center
worldlocation
```

Expected WLOC evidence is worldspace `0000003c`, position
`(1.0,193.98508,275.31247)`, identity rotation, and the current prepared
manifest fingerprint `24efdfcef26d1ebb3d347c976da6c85cd8a17e313b8a22c2709ff90b180941d0`.
Restart the exact manifest with `--save-slot m6-route-center`, run
`worldlocation`, and compare those fields exactly.

The save is synthetic runtime evidence created by commands. The binary under
the ignored `.bevyout/saves` directory must not be committed and is not
authored Fallout source. A successful synthetic save/reload does not close
the persistent-door or W4-C travel gate.

## Five-loop recording protocol and numeric fields

After the baseline and one rapid reversal at a boundary, perform five
complete out-and-back loops from `(4,-5)` to `(-1,-5)` and back using ordinary
movement input. Number them `loop=1` through `loop=5`; do not reset the
cache, process, or viewer between loops. Record one row after each loop and
one row for the final post-loop cooldown.

Each row must contain the following fields, with units and a clear
`measured`, `not_yet_sampled`, `not_run`, or `unsupported` status where a
value is unavailable:

- run metadata: machine, OS, GPU, build profile, commit, route direction,
  loop number, start/end grid, cache state, and sample window;
- preparation: `clean_prepare_s`, `warm_prepare_s`, `cache_bytes`,
  `assets_built`, `assets_reused`, `native_converter_invocations`, and
  `no_op_rebuild_s`;
- streaming: `ready_p50_ms`, `ready_p95_ms`, `transition_p50_ms`,
  `transition_p95_ms`, `transition_worst_ms`, `resident_cells_peak`,
  `resident_roots_peak`, `colliders_peak`, `lights_peak`,
  `stale_completions`, `failed`, `cancelled`, and `invalid_unload_count`;
- process/frame measurements: `steady_frame_ms`, `frame_p01_ms`,
  `frame_max_ms`, `process_memory_peak_mb`, `process_memory_ending_mb`,
  and `post_loop_memory_plateau_mb`;
- gameplay/dependency fields: `actor_path_ms`, `actor_status`,
  `actor_stuck`, `actor_blocked`, `return_anchor_error_m`,
  `water_status`, `door_status`, `worldspace`, `climate`, `weather`, and
  `save_reload_status`.

No numeric budget is fabricated here. Threshold sign-off is tracked by child
issue #285 and must be agreed and recorded before the gate run, alongside
machine/build/cache metadata. Package-byte estimates remain separate from
process memory, and a missing measurement is not a zero.

### Deterministic streaming diagnostic — 2026-08-02

The current commit `8bb7f244` was launched with the prepared v21 native cache
`.bevyout/m6-w6c-route-clean-20260802`, bridge port `15757`, and
`--disable-physics`. The exact `tp` sequence above was repeated five times
without resetting the viewer or cache. This is a synthetic streaming/lifecycle
probe, not the ordinary-input gate and not collision-ready evidence. Each row
is cumulative and was captured after returning to `(4,-5)`:

| Loop | Requests | Evictions | Resident | Peak resident | Failed | Cancelled | Stale | Invalid unload | RSS peak (bytes) |
| ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | 19 | 13 | 7 | 11 | 0 | 0 | 0 | 0 | 1,295,007,744 |
| 2 | 32 | 26 | 7 | 11 | 0 | 0 | 0 | 0 | 1,334,054,912 |
| 3 | 45 | 39 | 7 | 11 | 0 | 0 | 0 | 0 | 1,335,885,824 |
| 4 | 58 | 52 | 7 | 11 | 0 | 0 | 0 | 0 | 1,374,535,680 |
| 5 | 71 | 65 | 7 | 11 | 0 | 0 | 0 | 0 | 1,389,547,520 |

The closed trace recorded RSS peak `1,389,645,824` bytes, ending RSS
`1,389,551,616` bytes, and `8` samples. The missing-package negative path is
separate; it is not included in these clean-loop counters. Ordinary OS-input
traversal, collision-ready timing, frame budgets, actor/path, travel/save,
water, and post-loop plateau acceptance remain `not_yet_sampled` or
dependency-held.

## Failure and dependency status

- Actor source fixture and runtime actor/navigation crossing: `not_run`, held
  by #10/W3; no actor asset/catalog is present in the prepared route.
- Route door: zero route DOOR refs/edges; the Megaton fallback is source-only
  and not live-accepted until W4-C.
- Route water: dry in all 14 selected cells; water entry/exit uses separate
  fixture `00001262` and is `not_run` until W4-C.
- Ordinary-input traversal, rapid reversal, and numeric budgets remain
  `not_yet_sampled`; the explicit-trace deterministic `tp` diagnostic now has
  five clean out-and-back loops and closed process-memory peak/ending samples
  recorded above. The negative-path missing-package probe remains separate and
  is not part of the clean-loop counters.
- Gate #87 and final M6 acceptance: not claimed by this manual.
