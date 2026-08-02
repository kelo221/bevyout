# M6 Wave 6 plan — Bounded-route convergence and frozen budgets

## Fixed feature list

Wave 6 prepares the acceptance protocol; it is not a feature bucket and does
not close gate #87. The real-data dependency is explicit.

| Lane | Status | Authority and scope | Deferred |
| --- | --- | --- | --- |
| W6-A | Integrated | Deterministic convergence-report projection over existing viewer/exterior diagnostics and focused tests | Lifecycle, actor/nav, travel, scene, environment, and presentation behavior |
| W6-B | Held for current-data preflight | Freeze `M6_WAVE7_MANUAL.md`, exact route fixtures, command transcript, clean/warm matrix, and numeric budget recording | Cannot choose actor/door/water/save fixtures from stale or missing Fallout data |

W6-A must not turn `null`, package byte estimates, or unrun measurements into
claims. Each report domain needs a stable availability/status value and a clear
distinction between measured process memory, estimated package bytes, and
not-yet-run timing/cache/route evidence. Prepared serialized schemas and their
revisions are outside this lane.

## Executor brief and integration order

Only W6-A is dispatched in this kickoff. W6-B will be created as a separate
child issue and assigned before execution once a current v21 catalog and route
preflight provide exact actor, door, water, weather, and save-point identities.

| Order | Issue | Owned files | Exit evidence |
| --- | --- | --- | --- |
| Parallel preparation | W6-A | Existing diagnostics/report modules, the `worldstream summary` projection, and dedicated tests named in `M6_WAVE6_PROMPT.md` | Repeated identical inputs produce byte-identical output; all eight evidence domains are present with explicit status; estimates never populate real-memory fields |
| Deferred | W6-B | `docs/plans/M6_WAVE7_MANUAL.md`, synthetic fixtures, and command transcript shape | Exact route matrix and numeric-budget recording protocol are frozen only after current-data preflight |

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

No gate #87, W7, or final M6 acceptance claim is valid until W6-B is frozen and
the orchestrator runs the current-data preflight plus the real route matrix.

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

W6-B remains held pending a current Fallout v21 data/config preflight for exact
actor, door, water, weather, and save-point fixtures. W6-A does not claim gate
#87, W7, or final M6 acceptance.
