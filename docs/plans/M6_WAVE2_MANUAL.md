# M6 wave 2 — streaming reliability and memory manual

This manual exercises the short real exterior route from the M6 wave 7 draft.
It proves the Wave 2 diagnostics and lifecycle invariants only; it does not
close #87 or freeze the final M6 performance budgets.

## 1. Prepare the fixed route

From the repository root, prepare the bounded route and safety ring:

```text
cargo run-dev -- prepare 00000c49 00000c4a 00000c4b 00000c4c 000010d5 00001245 00000c67 00000c68 00000c69 00000c6a 00000c6b 000010eb 000010ec 000010ed --jobs 1 --force
```

Expected: the command ends with `14 done, 0 failed`. Native conversion
diagnostics for unsupported references are allowed, but preparation must not
abort or omit the physics sidecar ownership records.

## 2. Launch the bridge

```text
cargo run-dev -- view --manifest .bevyout/cache/scenes/00000c49/scene.ron --agent-bridge --agent-port 15736 --trace-seconds 240
```

Do not enable worldspace LOD for this lifecycle run. Record the commit, build
profile, operating system, CPU/GPU, and whether the cache was clean or warm.

## 3. Capture the initial diagnostics

Run:

```text
worldstream trace 1
worldstream status
worldstream summary
```

Expected:

- `streaming.memory_measurement` identifies the platform/method when process
  memory is supported, or explicitly says unsupported/unmeasured.
- Current, peak, and ending process-memory fields are either real sampled
  values or explicit `null`; they are never copied from package estimates.
- `resident_package_bytes_estimate` and `peak_package_bytes_estimate` remain
  separately labelled serialization estimates.
- The current cell is collision-ready and `failed=0`.

## 4. Cross both directions normally

Use ordinary keyboard movement across the prepared cells. At each boundary,
wait for `worldstream status`, then record the JSON output. If a deterministic
teleport is needed to recover the authored slope, use the same atomic positions
as the bounded-route draft:

```text
tp 180 177 275.31
tp 120 187 275.31
tp 60 197 275.31
tp 10 194 275.31
tp -50 181 275.31
```

Return through the same stops in reverse order. Expected at every stop:

- the active grid becomes the intended neighbor;
- the current cell remains playable until its replacement is collision-ready;
- no grid has duplicate roots or duplicate collision ownership;
- `stale_completions=0`, `failed=0`, and resident counts stay within the
  configured budget.

## 5. Exercise rapid reversal and repeated eviction

At a cell border, alternate direction quickly for at least ten complete
crossings. Keep `worldstream trace 1` enabled and capture:

```text
worldstream status
worldstream cells
worldstream summary
```

Expected: no stale completion resurrects an evicted generation, no cell root is
duplicated, collision ownership is released before root despawn, and the final
summary reports no residual cell-owned entries for cells that have evicted.
Record the sample window and the peak/ending process-memory readings; do not
interpret this run as the final M6 budget.

## 6. Exercise a missing-package cancellation

Stop the viewer, move one neighboring prepared package out of its expected
cache path, relaunch, and repeat one boundary crossing. Expected: the current
collision-ready cell remains playable, the missing package appears as a stable
failure in `worldstream status`, and no stale completion resurrects it. Restore
the package before any later gate run.

## 7. Record the result

Before recording the final summary, close the bounded trace so the sampler
captures the ending process-memory value:

```text
worldstream trace 0
worldstream summary
```

Expected: `memory_trace_active=false`, and on a supported platform
`ending_memory` is populated from the final process-resident-set sample. On an
unsupported platform the report remains explicitly `unmeasured`/`null`.

Attach to the Wave 2 issue/PR:

- exact commands and commit;
- machine/build/cache metadata;
- initial, peak, post-loop, and cool-down process-memory samples plus method;
- `stale_completions`, requests, evictions, resident-root/collision counts;
- focused and repository gate results;
- any defect as a new child bug under #13 rather than silently expanding Wave 2.

## 8. 2026-08-03 local macOS acceptance addendum

This addendum is the executable continuation of the frozen protocol above. It
does not replace the six-cell route, the distinction between ordinary and
synthetic input, or the Wave 2 exit criteria. Run it from the repository root
on the configured checkout. The target cache is derived and ignored; preserve
any prior evidence cache and choose a new suffix if this exact path already
exists.

### 8.1 Preflight and preparation

Record `git rev-parse HEAD`, `git status --short`, macOS version, CPU/GPU,
Bevy/build profile, and the cache path. Verify the CLI and frozen route before
preparing:

```text
cargo run-dev -- prepare --help
cargo run-dev -- prepare 00000c49 00000c4a 00000c4b 00000c4c 000010d5 00001245 00000c67 00000c68 00000c69 00000c6a 00000c6b 000010eb 000010ec 000010ed --list-only
cargo run-dev -- prepare 00000c49 00000c4a 00000c4b 00000c4c 000010d5 00001245 00000c67 00000c68 00000c69 00000c6a 00000c6b 000010eb 000010ec 000010ed --check-fingerprints
```

Use one explicit new cache for clean and warm preparation (substitute the
actual chosen suffix in every command):

```text
cargo run-dev -- --config .bevyout/config.toml prepare 00000c49 00000c4a 00000c4b 00000c4c 000010d5 00001245 00000c67 00000c68 00000c69 00000c6a 00000c6b 000010eb 000010ec 000010ed --cache-dir .bevyout/m6-w2-acceptance-20260803 --actor-animation-converter native --jobs 1
cargo run-dev -- --config .bevyout/config.toml prepare 00000c49 00000c4a 00000c4b 00000c4c 000010d5 00001245 00000c67 00000c68 00000c69 00000c6a 00000c6b 000010eb 000010ec 000010ed --cache-dir .bevyout/m6-w2-acceptance-20260803 --actor-animation-converter native --jobs 1
cargo run-dev -- --config .bevyout/config.toml prepare 00000c49 00000c4a 00000c4b 00000c4c 000010d5 00001245 00000c67 00000c68 00000c69 00000c6a 00000c6b 000010eb 000010ec 000010ed --cache-dir .bevyout/m6-w2-acceptance-20260803 --actor-animation-converter native --jobs 1 --check-fingerprints
```

The first preparation is the clean row and the second is the warm row. Record
the actual completion line, elapsed seconds, cache bytes, assets built/reused,
native converter invocations, recoverable diagnostics, and the fingerprint
result. A timeout or partial resumable cache is diagnostic only and cannot be
used as a preparation budget.

### 8.2 Ordinary-input route

Launch the prepared manifest with physics and keep its window visible and
frontmost for the entire run:

```text
cargo run-dev -- --config .bevyout/config.toml view --manifest .bevyout/m6-w2-acceptance-20260803/scenes/00000c49/scene.ron --agent-bridge --agent-port 15736 --trace-seconds 600
```

Start with `worldstream trace 1`, `worldstream status`, `worldstream cells`,
and `worldstream summary`. Use the physical keyboard with held movement keys,
not `tp`, to traverse `(4,-5)` → `(3,-5)` → `(2,-5)` → `(1,-5)` → `(0,-5)` →
`(-1,-5)` and return in reverse order. At each boundary hold the direction
until the next cell is collision-ready, then record status, cells, summary,
and the exact position/grid. A focused key hold that does not produce a
`ButtonInput<KeyCode>` lease is not an acceptance run; label it
`not_yet_sampled` and retain the diagnostic output.

At one boundary, reverse direction before the replacement is ready, then
repeat the reversal after collision readiness. Repeat at least ten complete
out-and-back crossings without resetting the viewer or cache. The expected
live invariants are: the old playable cell remains until replacement collision
readiness, one root and collision claim per grid, no stale completion, no
invalid unload, and no cell-owned state after final eviction. Run
`worldstream trace 0` and `worldstream summary` after a fixed cooldown so
`ending_memory` is sampled.

### 8.3 Evidence rows

Record one row for the clean preparation, warm preparation, initial route,
reversal/cancellation window, ten-crossing loop, and post-loop cooldown. Use
`measured`, `synthetic_input_measured`, `deterministic_streaming_diagnostic`,
`not_yet_sampled`, `not_run`, or `unsupported` explicitly. Include:

- route direction, loop count, start/end grid, cache state, and trace sample
  window;
- `requests`, `ready_total`, `evictions`, `resident_cells_peak`, root/collider
  counts, `cancellations`, `stale_completions`, `invalid_unload_count`, and
  failures;
- process-memory method/platform/current/peak/ending/sample count and
  post-loop plateau, separately from resident/peak package estimates;
- frame p50/p95/max and over-budget count using an explicitly stated
  `performance_probe` window and `16.6667 ms` reporting budget;
- clean/warm preparation seconds, cache bytes, built/reused counts, and
  no-op fingerprint result.

Do not close #274 or #275, tick the #13 Wave 2 checklist, or infer a final M6
budget from this addendum unless all ordinary-input and preparation rows are
complete. Comment measured evidence on both issues; create and assign a new
child issue under #13 for every nontrivial defect.

### 8.4 Actual local macOS result — 2026-08-03

The clean and warm preparation commands above were run against the configured
Fallout 3 installation in `.bevyout/m6-w2-acceptance-20260803`:

- clean: `14 done, 0 failed`; native conversion `1608/1608 ok`; real
  `619.64s`;
- warm: `14 cells valid, 0 stale`; real `8.31s`;
- current manifest source fingerprint:
  `24efdfcef26d1ebb3d347c976da6c85cd8a17e313b8a22c2709ff90b180941d0`;
- current prepare revision:
  `prepare-v22-m6-worldspace-lod-imad-screen-fx-combat-condition`.

The viewer was launched exactly as in §8.2 on macOS 26.5 / Apple M5 Max / Metal
with bridge port `15736`. Direct CoreGraphics `KeyA`/`KeyD` events posted to
the viewer process crossed the real `(4,-5)` ↔ `(3,-5)` physics boundary in
both directions and repeated the crossing twelve times in total. This must be
recorded as `os_input_injected`, not `ordinary_input_measured`: it uses the
normal gameplay input path, but is not a human pressing a frontmost physical
keyboard. The local macOS focus/Accessibility restrictions prevented a
literal physical-keyboard run.

The closed trace evidence was:

```text
requests=18 ready_total=18 evictions=12 resident_cells=7
peak_resident_cells=9 failed=0 cancellations=0 stale_completions=0
invalid_unload_count=0 collision_tracked=7 collision_pending=0
memory_measurement_method=libproc_process_resident_set
memory_samples=13 process_peak=1600290816 process_ending=1312915456
package_resident=1998243 package_peak=2687447 package_budget=134217728
frame_p50_ms=6.225959 frame_p95_ms=6.82575 frame_max_ms=9.283125 over_budget=0
```

The direct macOS sampler is a narrow #274 fix: the old `sysinfo` path claimed
support but returned no viewer process on this host. The focused native
sampler test, 32 exterior unit tests, all 640 Cucumber scenarios, formatting,
clippy, standard tests, and dynamic-linking tests pass.

The live trace did not observe a nonzero cancellation counter; cancellation,
stale completion, root uniqueness, collision ownership, and final teardown are
demonstrated by the focused runtime tests, while the live route ended with no
stale, invalid, failed, or pending collision state. Do not close #274/#275 or
tick the #13 checklist from this result: literal physical ordinary-input
acceptance and a live rapid-cancellation sample remain incomplete. #10 is
still open, so #278 remains out of scope.
