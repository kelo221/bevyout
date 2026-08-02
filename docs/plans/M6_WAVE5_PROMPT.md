# M6 Wave 5 — Environment and lighting isolation

## Request

Advance the two independent pure-policy lanes that can proceed while W4-C
runtime travel/physics integration and the W2/W3 real-data gates remain
pending:

- W5-A: make prepared environment selection and day/night/weather blending
  deterministic, bounded, and explicit about authoritative versus preview
  fallback behavior.
- W5-B: define engine-independent local-light ranking, ownership, and budget
  policy for streamed exterior cells.

These are policy lanes only. W5-C owns the later Bevy scene, streaming, travel,
ImageSpace, and light-system integration after W4's transition contract is
stable. Do not claim the Wave 5 runtime exit criterion from this work.

## Execution model

Codex runtime recommendation: **GPT-5.6 Luna, Max reasoning** per executor;
the orchestrator owns issue housekeeping, worktree integration, conflict
resolution, and the repository gates.

## Shared constraints

- Work from the M6 wave branch and the assigned isolated worktree only.
- Follow `docs/plans/M6_WAVE5_PLAN.md` as the fixed scope.
- Keep `bevyout-core` free of Bevy, filesystem, and process dependencies.
- Put pure decisions in dependency-light modules and keep Bevy adapters thin.
- Preserve the existing prepared manifest schema and runtime authorities. If a
  serialized prepared type must change, bump its `*_REVISION` and document why.
- Add focused tests first in the existing dedicated test files/modules. Do not
  add inline implementation test modules.
- Do not edit `tests/features.rs`, shared plugin/console wiring, W5-C runtime
  files, generated Fallout-derived data, or the other executor's lane.
- Report exact focused-test and validation results in the child issue comment;
  unit coverage is not Wave 5 acceptance.

## W5-A ownership

Own only the prepare-side environment resolver and the existing day/night
policy boundary:

- `src/vsa/prepare/orchestrator.rs`
- `src/vsa/prepare/tests/orchestrator.rs` or its dedicated test seam
- `src/viewer/day_night.rs`
- `src/viewer/tests/day_night.rs`
- `crates/bevyout-core/src/time_of_day.rs` and its dedicated tests only if a
  pure reusable policy extraction is required

Prove deterministic worldspace/parent-climate, climate/weather, and time-of-day
precedence/fallback behavior; bounded weather transition progress; and
reproducible color/keyframe blending. Reuse existing `CellInfo`, prepared
profiles, and time policy rather than creating a second environment authority.
Do not wire travel, streaming, ImageSpace reapplication, or scene integration.

## W5-B ownership

Own only an engine-independent local-light policy and focused core/fixture
coverage:

- a capability-named pure module under `crates/bevyout-core/src/` (or the
  existing exterior manifest module when that is the narrowest seam)
- `crates/bevyout-core/src/lib.rs` only for registering that module
- dedicated core tests and synthetic light-policy fixtures

Define stable distance/FormID ordering, fixed budget, invalid-input handling,
and explicit cell ownership semantics. The policy must not depend on Bevy
`Entity`, `Visibility`, runtime world queries, frustum/occlusion results, or
gameplay collision/AI state. Do not edit
`src/viewer/world/exterior/mod.rs`; W5-C owns runtime light spawning, teardown,
and presentation integration.

## Handoff

Each executor must leave one focused commit, state the commit hash in its issue
comment, and list the exact tests/gates run. The orchestrator will cherry-pick,
run the full repository gates, amend the plan, and keep W5-C and real-data
acceptance explicitly pending.
