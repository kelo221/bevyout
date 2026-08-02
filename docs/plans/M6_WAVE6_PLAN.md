# M6 Wave 6 plan — Bounded-route convergence and frozen budgets

## Fixed feature list

Wave 6 prepares the acceptance protocol; it is not a feature bucket and does
not close gate #87. The real-data dependency is explicit.

| Lane | Status | Authority and scope | Deferred |
| --- | --- | --- | --- |
| W6-A | Dispatched | Deterministic convergence-report projection over existing viewer/exterior diagnostics and focused tests | Lifecycle, actor/nav, travel, scene, environment, and presentation behavior |
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

_Wave kickoff; W6-A dispatched, W6-B held pending current v21 route data._
