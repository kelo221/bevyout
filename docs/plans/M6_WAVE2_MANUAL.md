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
