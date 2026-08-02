# M6 Wave 6 plan — Bounded-route convergence and frozen budgets

## Fixed feature list

Wave 6 prepares the acceptance protocol; it is not a feature bucket and does
not close gate #87. The real-data dependency is explicit.

| Lane | Status | Authority and scope | Deferred |
| --- | --- | --- | --- |
| W6-A | Integrated | Deterministic convergence-report projection over existing viewer/exterior diagnostics and focused tests | Lifecycle, actor/nav, travel, scene, environment, and presentation behavior |
| W6-B | Integrated — protocol freeze | Freeze `M6_WAVE7_MANUAL.md` and `M6_WAVE9_MANUAL.md`, exact route fixtures, command transcript, clean/warm matrix, and numeric budget recording | Live route acceptance, numeric threshold sign-off, and actor/door/water runtime integration remain deferred |
| W6-C (#285) | Open — pre-W7 orchestrator follow-up | Freeze numeric route thresholds with provenance; keep configured/reporting limits, package estimates, process memory, and measured acceptance distinct | Requires the agreed matrix and the runtime/clean-warm evidence that supplies dependency-held fields; does not close #87 |

W6-A must not turn `null`, package byte estimates, or unrun measurements into
claims. Each report domain needs a stable availability/status value and a clear
distinction between measured process memory, estimated package bytes, and
not-yet-run timing/cache/route evidence. Prepared serialized schemas and their
revisions are outside this lane.

## Executor brief and integration order

W6-B was dispatched as child issue #284 after the current v21 catalog and route
preflight provided exact actor, door, water, weather, and save-point identities.

| Order | Issue | Owned files | Exit evidence |
| --- | --- | --- | --- |
| Parallel preparation | W6-A | Existing diagnostics/report modules, the `worldstream summary` projection, and dedicated tests named in `M6_WAVE6_PROMPT.md` | Repeated identical inputs produce byte-identical output; all eight evidence domains are present with explicit status; estimates never populate real-memory fields |
| Integrated | W6-B | `docs/plans/M6_WAVE7_MANUAL.md`, `docs/plans/M6_WAVE9_MANUAL.md`, and command transcript shape | Exact route matrix and numeric-budget recording protocol are frozen; runtime evidence and threshold sign-off remain deferred |
| Pre-W7 orchestrator follow-up | #285 (W6-C) | Threshold matrix and provenance in the M6 plan/manuals and parent issue | Numeric limits are recorded before W7; missing actor/travel/scene/path values remain explicitly dependency-held rather than fabricated |

## Tests-first order for W6-A

1. Add tests for stable domain ordering and deterministic JSON serialization.
2. Add tests for measured, unsupported, not-yet-sampled, and not-run status
   states without fabricating values.
3. Add a regression proving process-memory fields cannot be populated from
   package-size estimates.
4. Add a report projection test covering the existing streaming and
   presentation diagnostics without changing lifecycle behavior.
5. Implement the narrow report/schema projection, then run the repository gates.

## Integration gates

After W6-A lands on `M6-OutCell`, run:

```powershell
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test
cargo check-dev
cargo run-dev -- prepare --help
```

The repository currently has known unrelated full-format drift in
`src/cli/tests/mod.rs`, `src/viewer/console/render_commands.rs`,
`src/viewer/controls.rs`, `src/viewer/tests/agent_bridge.rs`,
`src/viewer/tests/controls.rs`, and `src/viewer/tests/ragdoll_lab.rs`.
Changed-file rustfmt must still pass; those baseline files remain outside this
lane.

No gate #87, W7, or final M6 acceptance claim is valid until W6-B is frozen,
#285 records the numeric threshold matrix, and the orchestrator runs the
current-data preflight plus the real route matrix.

## Shipped amendments

### W6-A shipped — 2026-08-02

W6-A is integrated on `M6-OutCell`:

- Executor commit: `40318fba` (`M6 W6-A: add convergence report projection`)
- Integration commit: `4be8a5cf` (`M6 W6-A: add convergence report projection`)
- Follow-up report-shape compatibility adjustment retains the legacy
  `runtime.blender_invocations` field as explicit `null` with a `not_run`
  status; no fabricated invocation count is reported.
- `worldstream summary` now projects one deterministic `m6-convergence-v1`
  report with ordered streaming/lifecycle, actor/navigation, travel/save,
  environment, presentation, cache/preparation, frame-timing, and process-
  memory domains. Each domain carries an explicit status and value; process
  memory remains separate from package-byte estimates.
- Dedicated diagnostics tests cover byte-identical repeated reports, status /
  value separation, measured/unsupported/not-yet-sampled/not-run states, and
  the existing streaming/presentation surfaces.
- Validation passed: targeted rustfmt and diff check, `cargo check-dev`,
  `cargo clippy --all-targets -- -D warnings`, full `cargo test` (1,660 Rust
  tests plus 633 Cucumber scenarios / 3,130 steps), and
  `cargo run-dev -- prepare --help`.

W6-B is now integrated after the current Fallout v21 data/config preflight for
exact actor, door, water, weather, and save-point fixtures. W6-A/W6-B do not
claim gate #87, W7, or final M6 acceptance.

### W6-B shipped — 2026-08-02

W6-B is integrated on `M6-OutCell`:

- Issue #284 is the current-data protocol child under epic #13.
- Executor commit: `3b310120` (`M6 W6-B: freeze current-data route protocol`).
- Integration commit: `c44203a7` (`M6 W6-B: freeze current-data route protocol`).
- The W7 manual freezes the 14-cell native selector set, the six-cell
  Super-Duper Mart ↔ Megaton traversal order, clean/warm preparation and
  fingerprint checks, reversal and five-loop recording, and the synthetic
  `m6-route-center` save/reload checkpoint.
- Current-data boundaries are explicit: actor `000638e8`/`0001cf73` is an
  identity-only route fixture with no prepared actor asset/catalog; route cells
  are dry; water `00001262`/`0007e421` is a separate prepared fixture; and
  door `00003b24` in persistent cell `00002db4` is source-only until W4-C
  runtime integration.
- Current environment identities are worldspace `0000003c`, climate
  `00017907`, and weather `00064609`; stale manual weather `00015425` is not
  used.
- The main-checkout native preflight reported `14 cells valid, 0 stale`; the
  isolated executor worktree's missing ignored cache was recorded as a
  worktree limitation rather than current-data evidence.
- The manuals record numeric fields and required metadata but intentionally do
  not fabricate final thresholds. Numeric sign-off and runtime measurements
  remain pre-W7 orchestrator work tracked by #285. No gate #87 or final M6
  claim is made.

### W6-C threshold sign-off follow-up — 2026-08-02

Issue #285 is the assigned, linked child for the pre-W7 numeric decision. The
current tree provides grounded provenance for 25 exterior resident cells,
134,217,728 estimated package bytes, 64 active streamed local lights, and a
16.6667 ms convergence report budget. Those values are not silently promoted
to measured acceptance thresholds. #285 must record the remaining preparation,
ready/transition, frame, process-memory, path, and gameplay dependency fields,
or mark each one with the exact run that will supply it.

#### Pre-sign-off matrix

This is the current evidence boundary for #285; it is not a W7 pass report.

Configured/reporting provenance is explicit in the current tree: the exterior
resident default `25` is `DEFAULT_EXTERIOR_RESIDENT_CELL_LIMIT` in
`src/config.rs` and is passed into `ExteriorStreamBudget` by
`src/viewer/world/exterior/mod.rs`; the `134,217,728` byte value is that
plugin's `128 * 1024 * 1024` estimated-package budget; the `64` streamed-light
default is `DEFAULT_EXTERIOR_LOCAL_LIGHT_BUDGET` in
`crates/bevyout-core/src/local_light_policy.rs` and the exterior presentation
budget's matching default; and `16.6667 ms` is the convergence-report counter
in `src/viewer/diagnostics.rs`. The W7 manual is the collection path for
runtime values and records machine, build, cache, route, and clean/warm
metadata alongside them.

The older M2 `<=33 ms` swap bar and Vault101 measurements in
`docs/plans/M2_WAVE2_PLAN.md` and `docs/plans/M2_WAVE3_PLAN.md` are historical
evidence for that M2 route only. They are not promoted to an M6 v21 threshold:
the M6 matrix requires ordinary-input samples and a separately agreed
transition budget before any frame number can become a pass/fail criterion.

For the same reason, the older four-worker preparation figures in the M6
appendix of `docs/plans/WorldPlan.md` (clean `131.691 s`, warm `8.794 s`, and
`772183616` cache bytes from PR #261) are historical offline evidence, not the
frozen W6 one-worker protocol. The current one-worker and four-worker attempts
are recorded as timed-out diagnostics, so those older values must not be reused
as current clean/warm thresholds or completion proof.

| Measure | Current value or invariant | Status | Required proof before W7 |
| --- | --- | --- | --- |
| Exterior residency | `resident_cells <= 25` configured ceiling | Configured only | Ordinary bidirectional route must remain within the ceiling |
| Package accounting | `resident_package_bytes_estimate <= 134,217,728` bytes | Configured estimate bound | Record clean/warm cache and package results separately from process memory |
| Streamed local lights | `active_lights <= 64` configured cap | Configured only | Runtime route must record measured light peak and ownership teardown |
| Clean lifecycle | `stale_completions=0`, `failed=0`, `cancellations=0`, one root/collision owner per grid | Deterministic `tp` evidence plus one synthetic held-input boundary handoff; clean route only | Ordinary traversal, reversal, and repeated loops; the missing-package probe is an expected negative path with `failed=1` |
| Invalid unloads | `invalid_unload_count` is reported for rejected `Evict` actions or a final teardown that cannot find its cell; expected stale completions remain separate | Report field and focused rejected-eviction coverage; no route-grade sample yet | Record `invalid_unload_count` for the clean ordinary route and loops; keep the deliberate missing-package `failed=1` probe separate |
| Frame time | `16.6667 ms` convergence report budget | Reporting counter, not an accepted route threshold | Ordinary-input steady, 1% low, and worst-transition samples |
| Preparation | Frozen jobs=1 clean run: `288.3 s`, `14 done`, `0 failed`; identical warm rerun: `8.9 s`, `14 cells valid`, `0 stale`; cache `5,773` files / `1,697,002,779` bytes | Measured on Windows/dev/native, current commit `f600328b` | Preserve the exact clean/warm/check transcript in the W7 run record; older timeout attempts remain diagnostic only |
| Ready/transition latency | No ordinary-input request-to-ready or transition sample | Not measured | Record p50/p95/worst and collision-ready timing |
| Process memory | Explicit deterministic `worldstream trace 1/0` run measured 14 samples, peak `1,860,186,112` bytes, ending `1,857,990,656` bytes | Partial measurement; no threshold frozen | Ordinary five-loop peak, ending sample, and plateau rule |
| Actor/path, travel/save, water, and environment isolation | Route actor is identity-only; route is dry; door/water fixtures are separate | Dependency-held / not run | #10/W3-C, W4-C, and W5-C runtime integration |
