# M6 Wave 6 — Bounded-route convergence preparation

## Request

Advance W6-A only: consolidate the existing M6 diagnostics into one
deterministic, machine-readable convergence report that can later feed gate
#87. This is report/protocol preparation, not a runtime feature and not a gate
claim.

W6-B is intentionally held. The current workspace does not contain the Fallout
v21 data/config needed to validate the final actor, door, water, and exact
save-point fixtures. Do not invent FormIDs or turn stale manual values into
acceptance evidence; W6-B will be dispatched after a current-data preflight.

## Execution model

Codex runtime recommendation: **GPT-5.6 Luna, Max reasoning**. The
orchestrator owns the child issue, integration, full gates, and the eventual
real-data preflight.

## W6-A ownership

Own only the existing report/diagnostic boundaries and dedicated tests:

- `src/viewer/diagnostics.rs`
- `src/viewer/world/exterior/diagnostics.rs`
- `src/viewer/console/world_commands.rs` only for the report projection
- dedicated diagnostics/report tests under `src/viewer/tests/` and
  `src/viewer/world/exterior/diagnostics_tests.rs`

The report must deterministically expose the current evidence domains:
streaming/lifecycle, actor/navigation, travel/save, environment, presentation,
cache/preparation, frame timing, and process memory. Every domain must carry an
explicit availability/status contract (`measured`, `not_yet_sampled`,
`unsupported`, `not_run`, or equivalent) rather than silently presenting a
package estimate or fabricated value. Keep unavailable measurements explicit;
do not claim the value was measured.

Preserve existing `worldstream status`, `worldstream cells`, presentation, and
render-report behavior unless a focused report-shape correction is required.
Do not change exterior lifecycle transitions, collision ownership, actor/nav
runtime, travel/save behavior, scene integration, environment application, or
prepared serialized assets. No `tests/features.rs` or shared plugin wiring.

## Handoff

Start with focused tests for deterministic JSON/report ordering, status/value
separation, and process-memory/package-estimate separation. Leave one focused
commit and report the exact files, tests, and gates. No live M6 gate evidence is
valid from this lane alone.
