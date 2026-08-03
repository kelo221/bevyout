# M6 continuation — next-agent handoff

Updated: 2026-08-03
Issue: [#13 — M6 Exterior conversion, streaming, and dynamic lighting](https://github.com/kelo221/bevyout/issues/13)

## Purpose and boundary

This is a continuation checkpoint, not an M6 completion report. Current
`master` includes merged PR #288, which added the native macOS process-memory
adapter and its acceptance evidence. W2 acceptance, W6-C threshold sign-off,
the dependency-held runtime waves, gate #87, and final gate #14 are still open.

The earlier instruction to stop PR work around #286 is historical; #286 and
#288 are now merged. The current handoff records the post-#288 preparation
checkpoint and the exact remaining acceptance blockers.

## Start here

Read these in order:

1. [`M6_CONTINUATION_ROADMAP.md`](M6_CONTINUATION_ROADMAP.md)
2. [`M6_WAVE6_PLAN.md`](M6_WAVE6_PLAN.md) and [`M6_WAVE7_MANUAL.md`](M6_WAVE7_MANUAL.md)
3. [`M6_WAVE8_MANUAL.md`](M6_WAVE8_MANUAL.md) and [`M6_WAVE9_MANUAL.md`](M6_WAVE9_MANUAL.md)
4. The live state of [issue #13](https://github.com/kelo221/bevyout/issues/13),
   [issue #285](https://github.com/kelo221/bevyout/issues/285), and the dependency
   issues before making any acceptance claim.

The prior handoff branch was `M6-OutCell` at:

```text
f4f0bc8f72fb2c1db85b7093757294684a793862
Merge remote-tracking branch 'origin/master' into M6-OutCell
```

`origin/master` was an ancestor of that historical handoff, including the M5
combat commit. The current-master baseline for this checkpoint is `e139af26`
(`fix(m6): use native macOS exterior memory sampling (#288)`). Inspect
`git status --short --branch` before changing anything.

## What is verified

### Local repository gates

The following passed after reconciling with current `master`:

- `cargo check-dev`
- `cargo test` — 1,664 Rust tests, 636 Cucumber scenarios, 3,154 steps
- `cargo clippy --all-targets -- -D warnings`
- `cargo fmt --all -- --check`
- `git diff --check`
- `cargo test -p bevyout-core combat` — 11 tests
- `cargo test --lib invalid_evictions_are_counted_without_tearing_down_unowned_cells`
- `cargo test -p bevyout-core terrain_lod_selection_covers_base_bands_and_hysteresis_boundaries`

The focused LOD and invalid-unload tests are useful regression anchors, but
they do not replace real-data acceptance.

### W2/W6 deterministic route diagnostic

The latest documented sample used the v21 native prepared cache, commit
`8bb7f244`, `--disable-physics`, cache
`.bevyout/m6-w6c-route-clean-20260802`, and bridge port `15757`.

Five explicit deterministic `tp` loops were traced. Every loop returned to
`(4,-5)` and produced these cumulative counters:

| loop | requests | evictions |
| ---: | ---: | ---: |
| 1 | 19 | 13 |
| 2 | 32 | 26 |
| 3 | 45 | 39 |
| 4 | 58 | 52 |
| 5 | 71 | 65 |

The closed trace ended with 7 resident cells, a peak of 11 resident cells,
and zero failed loads, cancellations, stale completions, or invalid unloads.
The eight-sample closed-process RSS peak was `1,389,645,824` bytes and the
ending sample was `1,389,551,616` bytes. The per-loop RSS peaks are recorded
in the W6 plan and W7 manual.

This proves deterministic streaming/lifecycle behavior only. It is not proof
of collision-ready travel, ordinary OS keyboard input, actor navigation,
reversal, save/return, water traversal, or an accepted memory plateau. The
sample used `--disable-physics` deliberately.

### Current-master preparation checkpoint

The post-#288 master preflight is recorded in
[`M6_WAVE2_PLAN.md`](M6_WAVE2_PLAN.md) and
[`M6_CONTINUATION_ROADMAP.md`](M6_CONTINUATION_ROADMAP.md). It used cache
`.bevyout/m6-w2-overnight-master-20260803` and recorded clean `14 done, 0
failed` in `873.20s`, warm `14 cells valid, 0 stale` in `9.40s`, and a
report-only fingerprint check of `14 cells valid, 0 stale` in `5.46s`.
This is preparation evidence only; it does not satisfy the W2 human-input or
live-cancellation rows.

### W8-v21 presentation diagnostics

The current v21 evidence is:

- Native preflight: 14 frozen selectors, 0 stale selectors.
- Default-off presentation: active LOD 0, terrain 0, blocks 0; near 7,
  middle 0, distant 0; full-land mesh remains the gameplay/collision path.
- Bounded opt-in presentation: 48 tiles total — 40 terrain and 8 blocks;
  levels 4/8/16/32 contain 24/12/8/4 tiles. Turning it off returns to zero
  optional LOD tiles without changing full-land collision.
- CPU visibility measurements: `585 / 110 / 475` candidates/visible/CPU-
  culled with the default presentation, and `655 / 134 / 521` with the
  bounded opt-in presentation.
- Duplicate catalog and active visual identity counts: zero.
- Staged-load peak/cap: 8.
- 120-sample frame snapshot: average `7.7676 ms`, p50 `7.593 ms`, p95
  `8.7887 ms`, max `10.2409 ms`, and 0 samples over `16.6667 ms`.
- Paired Windows same-pose viewport captures were non-black. The opt-in
  capture shows the optional horizon; default-off removes that optional
  presentation. This is a qualitative single-pose sanity check, not a route
  crack/pop-in acceptance run.
- GPU occlusion is explicitly unmeasured: `measured=false`, `culled=null`.

The CPU visibility method is
`active_camera_visible_entities_cpu`. It must not be described as GPU
occlusion.

### Issue #13 cleanup already performed

The malformed opening body of issue #13 was reformatted into structured
Markdown, and the literal comment whose body was exactly `undefined` was
deleted. At handoff, the issue body is structured and there are no comments
with the exact body `undefined`. Re-check before posting any new issue update;
do not recreate that comment.

## What is still missing

| Area | Current state | Required proof before calling it accepted |
| --- | --- | --- |
| W2 | Implementation is present; deterministic `tp` trace exists | Focused ordinary OS-input route, collision-ready travel, reversal, repeated loops, cancellation/stale behavior, and a defensible process-memory plateau |
| W6-C / #285 | Live issue is closed, but its latest evidence says numeric sign-off is still open | Reconcile the issue/document state and record agreed measured thresholds with provenance, separating package estimates from process RSS and configured limits from observations |
| W3-C / #278 | Dependency-held | Resolve or explicitly re-evaluate the #10 dependency before runtime work; do not mark complete from preparation-only evidence |
| W4-C | Pending | Runtime implementation and real-data acceptance described by the W4 plan |
| W5-C | Pending | Runtime implementation and real-data acceptance described by the W5 plan |
| W7 / #87 | Open | Bounded gate acceptance after its W2/W6 prerequisites hold |
| W8 | Only the bounded presentation-diagnostics slice is evidenced | GPU/visual route acceptance, ordinary input route evidence, and any remaining actor/integration requirements; do not promote this slice to a full W8 gate |
| W9 / #14 | Draft/future final route | Actor crossing, bidirectional ordinary traversal, interior travel/return anchors, water, save/reload, repeated-loop memory, and final frame budgets |

The values `25` resident cells, `128 MiB` / `134,217,728` bytes, `64`
streamed lights, and `16.6667 ms` convergence reporting are configured
provenance values in the current W6 material. They are not automatically
accepted thresholds. The historical M2 `<=33 ms` figure is not an M6 gate.

## Runtime commands and operational notes

Use the dynamic-linking development alias. A direct launch of
`target/debug/bevyout.exe` previously failed on Windows with
`0xC0000135` (`-1073741515`) because the Bevy dynamic DLL was not available.

Representative diagnostic launch:

```powershell
cargo run-dev -- view --manifest .bevyout\m6-w6c-route-clean-20260802\scenes\00000c49\scene.ron --disable-physics --agent-bridge --agent-port 15757 --trace-seconds 180 --unfocused
```

Use a distinct free port and confirm it with `bevyout.session`. The bridge
bind happens asynchronously during viewer startup and the detached bind task
can fail without making the viewer process exit. A live process is therefore
not proof that the bridge is listening.

Useful bridge operations are:

```text
bevyout.session
bevyout.scene_snapshot
bevyout.console.exec  {"line":"help"}
bevyout.console.exec  {"line":"worldstream status"}
bevyout.console.exec  {"line":"worldstream summary"}
bevyout.console.exec  {"line":"worldstream presentation"}
bevyout.console.exec  {"line":"worldstream trace 1"}
bevyout.console.exec  {"line":"worldstream trace 0"}
bevyout.capture_viewport
```

For ordinary-input acceptance, run a focused window without `--unfocused` and
hold physical keys across fixed ticks. A BRP `KeyboardInput` Pressed/Released
message and `KeyboardFocusLost` are useful synthetic diagnostics, but they do
not prove the operating-system input route.

## Findings and traps

1. **Synthetic input is not OS input.** Reflected BRP keyboard events can prove
   the event/state path, but the ordinary-input gate needs a focused window and
   real key transitions. The synthetic held-input test crossed `(4,-5)` to
   `(3,-5)` and back with zero failed/cancelled/stale loads; it remains
   synthetic.
2. **`--disable-physics` changes the claim.** The clean five-loop trace is a
   streaming/lifecycle diagnostic, not a playable route. Do not use it to
   close collision, actor, or ordinary traversal acceptance.
3. **Do not mix memory units.** Estimated package size, resident-cell budgets,
   and process RSS are different measurements. Preserve the sampling method
   and provenance in any new report.
4. **Configured is not measured.** The W6 numbers above need a written
   acceptance decision and real samples before #285 can close.
5. **Unmeasured is not zero.** `culled=null` with `measured=false` means GPU
   occlusion was not measured. CPU `VisibleEntities` visibility is a separate
   diagnostic.
6. **The CPU counts are broader than the name suggests.** The current
   candidate count is derived from every `Mesh3d` in the main world, while
   visible IDs come from `VisibleEntities`. It is not yet an exterior-only
   candidate set and must not be presented as one.
7. **The captures are narrow evidence.** The paired non-black captures are a
   same-pose qualitative check. They do not establish route-wide horizon
   continuity, crack-free transitions, or absence of pop-in.
8. **The memory sampler has overhead.** The diagnostic currently constructs a
   new `sysinfo::System` for each process-memory sample/status query. Treat
   the current RSS series as useful evidence with a measurement caveat; a
   future performance pass may need a persistent sampler.
9. **Zero invalid unloads is scoped.** It means no invalid unload was observed
   in that clean trace. It does not prove that every invalid-unload path has
   been deliberately exercised.
10. **Bridge startup can be racy.** Confirm the port with a session call after
    startup, use a fresh port, and stop both the cargo wrapper and viewer
    process after a diagnostic run.
11. **Cargo filters can hit the cucumber harness.** For a root unit-test
    filter, use `cargo test --lib <filter>`. A bare `cargo test <filter>` can
    pass the unit test and then fail because `tests/features.rs` receives the
    unexpected filter argument.
12. **Do not commit derived Fallout data.** Prepared RON/GLB/DDS/WAV/NIF
    outputs belong in the ignored `.bevyout` cache. Keep only synthetic,
    explicitly allowed fixtures in the repository.
13. **Preserve the acceptance manuals.** The user-added partial-PR
    checkpoint in the roadmap is intentional. Amend plans with a dated
    “Shipped amendments” or evidence section; do not rewrite away the
    distinction between diagnostic evidence and a passed gate.
14. **Review findings already identified.** Greptile correctly noticed the
    per-sample `sysinfo::System` construction and the broad `Mesh3d` candidate
    query. These are follow-up opportunities, not reasons to inflate the
    current W8-C claim or to silently widen this checkpoint.

## Suggested continuation order

1. Inspect `git status`, the current-master cache/fingerprints, and the live
   issue states before relying on any numbers in this document. The latest
   preparation checkpoint is recorded in the W2 plan and roadmap.
2. Finish W2 acceptance with focused ordinary OS input and physics enabled. If
   the environment cannot provide reliable physical-input evidence, record the
   blocker instead of substituting BRP events or `tp`.
3. Reconcile the closed #285 issue with its latest comment, which says
   threshold sign-off remains open. Record an explicit W6-C measurement
   decision with repeatable provenance, separating RSS/package/configuration
   values, before executing W7; do not tick the gate from the issue state alone.
4. Re-evaluate the #10 dependency, then execute W3-C, W4-C, and W5-C in the
   order required by their plans. Keep each runtime change tied to its
   acceptance script and issue.
5. Revisit W7/#87 only after W2 and W6-C prerequisites are genuinely met.
6. Extend W8 from diagnostics to route-wide visual/GPU/input evidence, then
   perform W9/#14 final-route acceptance. Do not use the current partial slice
   to close #87, #13, or #14.
7. Keep issue #13 readable: append dated evidence and links, preserve the
   structured body, and never post placeholder text such as `undefined`.

## Handoff stop condition

The next agent should treat this document as a map of verified evidence and
known limits, not as permission to claim M6 completion. Before any gate is
closed, update the relevant plan/manual and issue with the exact run, commit,
cache, command line, and measured result that supports the claim.
