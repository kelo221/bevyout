# M6 wave 2 plan

## Fixed feature list

1. Add a platform-aware process-memory sampler for the viewer's exterior
   diagnostics. The report must identify its method/platform, expose current,
   peak, and ending samples when supported, and return explicit unsupported or
   unmeasured metadata when it is not.
2. Extend `worldstream summary` and the trace workflow without changing the
   meaning of `resident_package_bytes_estimate` or
   `peak_package_bytes_estimate`. Runtime process memory and package byte
   estimates remain separate fields.
3. Make bounded lifecycle behavior deterministic under ordinary two-way
   movement, rapid reversal, cancellation while loading, completion after
   eviction, repeated load/evict loops, and collision-pending transitions.
4. Preserve one root and one collision ownership record per grid, reject stale
   generations, and remove all cell-owned state after final eviction.
5. Add focused pure and minimal-App regression coverage for both lanes, then
   add executable feature coverage through the integrator's append-only seam in
   `tests/features.rs`.

## Tests-first order

### W2-A — process memory

- Define deterministic report fixtures for supported, unsupported, and not-yet-
  sampled states.
- Test that process-memory fields are not populated from package estimates.
- Test that repeated samples update current/peak/ending deterministically and
  preserve the method/platform label.
- Add a feature scenario for the separation of process memory and package
  estimates; the integrator adds its shared step definitions.

### W2-B — lifecycle and reversal

- Add pure planner cases for reversal while a cell is queued, loading, spawned
  but collision-pending, ready/resident, and evicting.
- Add minimal-App/runtime cases for stale completion rejection, root uniqueness,
  collision-ledger teardown, and zero residual cell-owned entities.
- Add a feature scenario for generation-safe cancellation/reversal; the
  integrator adds its shared step definitions.

## File authority

| Lane | May edit | Must not edit |
| --- | --- | --- |
| W2-A | `src/viewer/world/exterior/diagnostics.rs`, `src/viewer/console/world_commands.rs`, focused diagnostic tests, and the smallest required dependency declaration | Residency transitions, `crates/bevyout-core/src/manifest/exterior.rs`, lifecycle/loading policy, actor/environment/LOD code, `tests/features.rs` |
| W2-B | `crates/bevyout-core/src/manifest/exterior.rs`, `src/viewer/world/exterior/{policy.rs,lifecycle.rs,loading.rs,mod.rs}`, focused lifecycle tests | Process-memory implementation, `src/viewer/console/world_commands.rs`, actor/environment/LOD code, `tests/features.rs` |
| Integrator | `tests/features.rs`, shared plugin/console registration, `M6_WAVE2_MANUAL.md`, merge seam, issue/PR evidence | Changing either lane's authority without a recorded amendment |

If a lane needs a shared re-export or registration hunk, leave it for the
integrator or document the exact conflict and keep the semantic owner intact.

## Exit evidence

- Focused lane tests pass.
- `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, and
  `cargo test` pass after integration.
- A short real-data route crosses both directions, performs rapid reversal,
  repeats at least ten crossings, and records `stale_completions=0`, bounded
  roots/collision ownership, and no residual cell-owned state after eviction.
- `worldstream summary` records process-memory current/peak/ending samples on
  supported platforms, with method/platform metadata; unsupported platforms
  remain explicitly unsupported/unmeasured.
- The manual records machine, build profile, route, sample window, and whether
  the run was clean or warm. It does not freeze final numeric budgets; that is
  Wave 6/7 work.

## Execution model recommendation

Codex runtime: **GPT-5.6 Luna, Max reasoning**, per the kickoff request. The
orchestrator remains responsible for integration, gates, and judging real-data
evidence.

## Shipped amendments

### Implementation integrated — 2026-08-01

The W2-A/W2-B implementation and executable coverage are integrated on
`M6-OutCell`:

- W2-A commit: `95e42a92` (`Measure exterior process memory diagnostics`).
- W2-B commit: `370b1456` (`Harden exterior lifecycle reversal`).
- Integrator/docs commit: `edf504a0` (`M6 wave 2: add roadmap and executable exterior coverage`).

The repository gates for the integrated lanes passed as recorded in the parent
epic. Real current-data route evidence remains a separate acceptance step.

### Current v21 route evidence — 2026-08-02

The current Fallout v21 data/config preflight is green:

- The exact 14-cell selector list resolves with `--list-only`.
- Native `--check-fingerprints` reports `14 cells valid, 0 stale`.
- Prepared revision is `prepare-v21-m6-worldspace-lod-imad-screen-fx`.
- The Windows process-resident-set sampler reports method
  `sysinfo_process_resident_set`, platform `windows`, and real current/peak/
  ending samples, separate from package estimates.

A bounded local BRP run crossed the deterministic route in both directions
with ten-plus `tp` crossings and a final re-anchor to `(4,-5)`. The route
window observed `requests=22`, `evictions=16`, `resident_cells=7` at the final
anchor, `stale_completions=0`, `failed=0`, and `cancellations=0`; the final
cell list contained seven unique collision-ready entries. The sampled process
memory peak was `1,523,015,680` bytes and ending sample
`1,522,896,896` bytes, while the peak package estimate was `3,696,696` bytes
against the separate `134,217,728`-byte estimate budget. The route-window
frame sample had p50 `6.3027 ms`, p95 `7.7809 ms`, max `93.0192 ms`, and four
over-budget samples; the cooldown-only window was separately recorded and is
not substituted for the route result.

The follow-up deterministic probe completed five out-and-back loops. Every
loop returned to `(4,-5)` with no step failure; the clean-loop endpoint was
`requests=71`, `evictions=65`, `resident_cells=7`, `failed=0`,
`cancellations=0`, and `stale_completions=0`. That trace measured a process
memory peak of `1,598,648,320` bytes and ending sample `1,319,567,360` bytes;
the cooldown frame window was p50 `7.1569 ms`, p95 `7.9774 ms`, max
`9.0933 ms`, with zero samples over the existing 16.6667 ms report budget.

The reversible missing-package probe then removed the already-indexed
`00000c4c.ron` package, crossed toward grid `(1,-5)`, and observed the target
entry as `lifecycle=Failed`, `collision_ready=false`, `failed=1`, while the
current `(4,-5)` cell remained resident and `stale_completions=0`. The package
was restored and the viewer stopped; the expected failure is not part of the
clean-loop counts.

The frozen W7 clean-preparation command was also attempted with the exact 14
selectors, explicit current config, native conversion, and `--jobs 1` in a
new disposable cache. After the ten-minute command timeout the resumable job
manifest still listed every job as `Pending`, although the cache had reached
5,171 files and `1,013,366,580` bytes. The run was stopped and the partial
ignored cache was preserved only for diagnosis; it has no clean elapsed-time,
completion, warm-run, or threshold value and must not be reused as acceptance
evidence.

A separate diagnostic rerun with the same selectors and native converter at
`--jobs 4` also timed out after five minutes without a completion line. Its
resumable manifest had only `1/14` jobs marked `Done` at
`1,309,667,625` bytes across `5,419` files. That alternate-throughput attempt
is likewise not a clean or warm result and supplies no threshold.

An ordinary-input diagnostic was attempted against the same prepared
`00000c49` manifest in a fresh bridge viewer. The selected viewer window
accepted `Escape` (the pause menu opened and closed), but the available local
keyboard surface emitted W taps rather than a held `ButtonInput<KeyCode>` state:
one tap, repeated taps, and a focused-window retry left the player at
`(263.323120,159.345642,275.312469)` and the active grid at `(4,-5)`. A
temporary `tcl` no-clip probe produced the same no-displacement result and was
reverted before shutdown. No ordinary boundary, reversal, loop, or performance
measurement is claimed from this diagnostic; a true held-input pass remains
`not_yet_sampled` and requires a supported input-hold path or human keyboard
control.

### Reflected BRP held-input diagnostic — 2026-08-02

A fresh viewer session validated the existing runtime-write BRP path without
adding a new input system. After querying the primary window entity,
`world.write_message` accepted a reflected
`bevy_input::keyboard::KeyboardInput` with `KeyA`/`Pressed`; the real
`FixedUpdate` player controller moved the player from `x=263.3230` to
`x=254.7199` during a five-second hold. The matching `Released` message and a
`bevy_input::keyboard::KeyboardFocusLost` cleanup stopped movement at the same
position. This proves a supported synthetic bridge-input diagnostic path and
provides a way to measure lifecycle behavior while avoiding tap-only input.
It does not prove focused physical OS-keyboard traversal, so ordinary route,
reversal, loop, and gate-grade performance fields remain `not_yet_sampled`.

### Explicit trace loop diagnostic — 2026-08-02

The same clean current-data viewer was rerun with an explicit
`worldstream trace 1` before the route and `worldstream trace 0` after a
two-second cooldown. Five deterministic `tp` out-and-back loops completed
without resetting the viewer or cache. Each loop returned to `(4,-5)`; the
cumulative final counters were `requests=149`, `evictions=143`,
`resident_cells=7`, and `peak_resident_cells=11`, with
`failed=0`, `cancellations=0`, and `stale_completions=0`. The closed trace
contained 14 process-memory samples, with peak
`1,860,186,112` bytes and ending `1,857,990,656` bytes. The final convergence
projection reported frame p95 `10.0086 ms`, max `11.0126 ms`, and zero samples
over the `16.6667 ms` reporting budget. These are deterministic bridge/teleport
diagnostics only; they provide a closed memory window but do not replace the
ordinary focused-input route required by W7.

### Held-input boundary diagnostic — 2026-08-02

A fresh route viewer also exercised the real player physics and streaming
handoff through the reflected message path. Starting at `x=240.0` in grid
`(4,-5)`, a five-second `KeyA` `Pressed` lease moved the player to
`x=228.2097` in grid `(3,-5)`; the matching `Released` and
`KeyboardFocusLost` cleanup stopped it. A five-second `KeyD` lease then moved
the player back to `x=249.3505` in `(4,-5)`, followed by the same cleanup. The
closed trace ended with `requests=8`, `evictions=2`, `resident_cells=7`,
`peak_resident_cells=9`, `failed=0`, `cancellations=0`, and
`stale_completions=0`; six memory samples measured peak
`1,313,513,472` bytes and ending `1,294,901,248` bytes. This proves a
synthetic held-input physics/cell-boundary handoff, but remains distinct from
focused physical OS-keyboard acceptance and does not satisfy the W7 route gate.

This remains partial acceptance evidence because the five-loop route used
deterministic `tp` probes and the held-input run covered only one boundary in
each direction, not the full ordinary route. Actor, travel/save, and final
budget sign-off were not run. Issues #274, #275, and #285 remain open; no W2,
#87, or final M6 completion claim is made from these measurements.

### Acceptance continuation — 2026-08-03

The next acceptance pass is being run from the authenticated human's local
macOS checkout (`nippongun`) against the configured Fallout 3 data, on the
current clean `m5-wave3` worktree. Issues #274 and #275 were corrected to be
assigned to `nippongun`; #10 remains open, so #278 runtime actor integration is
out of scope and will not be started.

This pass keeps the original W2 exit boundary fixed:

- prepare the frozen 14-selector v21 route in a new ignored cache, then repeat
  the identical command warm and run the fingerprint check;
- launch the exact prepared `00000c49` manifest with physics and the loopback
  bridge, enable `worldstream trace`, and drive the six cells with ordinary
  focused keyboard input in both directions;
- perform rapid reversal/cancellation at a boundary and at least ten complete
  crossings, recording status/cells/summary after collision readiness;
- verify stale-generation, root, collision-ledger, teardown, and residual
  ownership fields from the live report, with focused lifecycle tests as the
  deterministic regression evidence;
- close the trace and record current/peak/ending process-resident-set samples,
  frame-window statistics, preparation timings/cache bytes, and the configured
  route/budget values as separate measured or unavailable fields.

Input evidence is labelled precisely: focused OS keyboard holds are
`ordinary_input_measured`; reflected `KeyboardInput` BRP messages are
`synthetic_input_measured`; `tp` is `deterministic_streaming_diagnostic` only.
Synthetic or teleport runs cannot satisfy the ordinary-input row. The route
is not promoted to W2-complete, and the #13 checklist is not ticked, until the
ordinary-input route and acceptance-grade preparation/budget rows are both
demonstrated. Any newly observed lifecycle defect becomes a separate assigned
child issue under #13 rather than an unrecorded Wave 2 expansion.

### Native macOS sampler and current-data acceptance — 2026-08-03

The local macOS acceptance run found that `sysinfo` reported a supported
platform but enumerated no process for the viewer, leaving the #274 sampler at
`sample_count=0`. This was an in-scope platform adapter defect, not a new
Wave 2 issue. The diagnostics adapter now uses macOS `libproc` directly for
`proc_taskinfo.pti_resident_size` and reports the method as
`libproc_process_resident_set`; Linux and Windows retain
`sysinfo_process_resident_set`. A focused macOS regression test was added and
passes.

Preparation evidence from the new ignored cache
`.bevyout/m6-w2-acceptance-20260803`:

- clean native preparation with the frozen 14 selectors and `--jobs 1`:
  `14 done, 0 failed`, `1608/1608` native assets successful, `/usr/bin/time`
  real `619.64s`;
- warm repeat: `14 cells valid, 0 stale`, `0 done, 0 failed`, real `8.31s`;
- the current prepared scene reports source fingerprint
  `24efdfcef26d1ebb3d347c976da6c85cd8a17e313b8a22c2709ff90b180941d0` and
  prepare revision `prepare-v22-m6-worldspace-lod-imad-screen-fx-combat-condition`.

The real viewer ran on macOS 26.5 / Apple M5 Max / Metal, dev profile, bridge
port `15736`, from that cache. Targeted CoreGraphics `KeyA`/`KeyD` events were
posted to the viewer process, so the normal player input, physics, collision
handoff, and streaming path—not BRP `tp` or reflected `KeyboardInput`—was
exercised. This is an OS-injected input measurement, not literal human
frontmost-keyboard acceptance; macOS Accessibility/focus control prevented
the latter in this environment.

The trace closed at grid `(4,-5)` with `requests=18`, `ready_total=18`,
`evictions=12`, `resident_cells=7`, `peak_resident_cells=9`,
`failed=0`, `cancellations=0`, `stale_completions=0`,
`invalid_unload_count=0`, `collision_tracked=7`, and
`collision_pending=0`. Repeated live logs show collision teardown before
eviction for `00000c6b` and `00000c4c`. Process memory was supported with
`13` samples, method `libproc_process_resident_set`, peak
`1,600,290,816` bytes, ending `1,312,915,456` bytes. Package estimates stayed
separate at resident `1,998,243`, peak `2,687,447`, against the
`134,217,728` byte package budget. The 600-sample frame window was p50
`6.225959ms`, p95 `6.82575ms`, max `9.283125ms`, over-budget `0`.

The 32 exterior unit tests cover queued/loading/collision-pending/evicting
cancellation, stale completion, duplicate-root/ownership, collision-ledger,
and final teardown paths; the 640-scenario executable feature suite also
passes. The live OS-injected run did not produce a nonzero cancellation
counter because prepared tasks completed before the rapid reversal window;
therefore cancellation remains test-demonstrated rather than live-count
demonstrated in this pass. Issues #274 and #275 remain open, the #13 Wave 2
checklist remains unticked, and #10 continues to block #278.
