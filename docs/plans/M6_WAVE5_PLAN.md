# M6 Wave 5 plan — Weather, ImageSpace, and lighting isolation

## Fixed feature list

This wave advances the policy half of the environment/lighting slice while
runtime integration remains sequential. It does not close W4, W5, or epic #13.

| Lane | Authority | Fixed implementation scope | Explicitly deferred |
| --- | --- | --- | --- |
| W5-A | Existing prepare resolver plus `viewer::day_night` | Deterministic environment-source precedence, authoritative/preview fallback behavior, bounded weather transition progress, and reproducible time/keyframe blending | Travel-triggered reapplication, streamed-cell environment swaps, ImageSpace runtime ownership, scene integration |
| W5-B | New or existing pure core light-policy module | Stable candidate ordering, fixed-budget selection, finite/invalid handling, and cell ownership contract for exterior local lights | Bevy light spawning, visibility writes, teardown systems, frustum/occlusion integration, gameplay effects |
| W5-C | Sequential viewer integration lane | Exterior environment/light application across travel and streaming; interior ImageSpace and irradiance isolation | Not dispatched by this wave |

The existing prepared manifest and runtime resources remain authoritative. Avoid
adding duplicate weather, clock, or light-budget resources. Any serialized shape
change requires the relevant prepared asset revision bump, migration behavior,
and focused round-trip tests.

## Executor briefs and merge order

W5-A and W5-B are parallel policy lanes with disjoint semantic ownership. Each
gets one isolated Luna Max worktree and one child issue under #13. The
orchestrator alone edits shared feature wiring, manuals, and integration
documentation after both commits land.

| Order | Issue | Executor-owned files | Exit evidence |
| --- | --- | --- | --- |
| Parallel 1 | W5-A | Existing prepare/day-night policy seams and dedicated tests named in `M6_WAVE5_PROMPT.md` | Same inputs resolve to the same source/profile; parent-climate and preview fallback are explicit; transition progress stays in `[0,1]`; keyframe colors remain finite and reproducible |
| Parallel 1 | W5-B | Pure core light-policy module, registration, and dedicated tests/fixtures | Stable distance then FormID ordering; budget is enforced including zero/empty cases; invalid candidates are deterministic; ownership is cell-scoped and presentation-only |
| Sequential 2 | W5-C | `src/viewer/scene.rs`, exterior presentation/light systems, post-processing and dedicated viewer tests | Deferred until W4-C's transition contract is stable; no executor in this plan may claim it |

## Tests-first order

### W5-A

1. Add focused tests for cell metadata versus worldspace/parent climate
   selection and missing-record fallback.
2. Add focused tests for preview weather selection and explicit source labels.
3. Add tests for transition progress at negative, zero, midpoint, overrun, and
   non-finite durations/elapsed values as applicable to the existing contract.
4. Add reproducibility and finite-output tests for time/keyframe interpolation.
5. Implement the smallest policy change that makes the tests pass.

### W5-B

1. Add pure tests for nearest-light ordering and deterministic FormID tie-breaks.
2. Add budget tests for zero, fewer-than-budget, and more-than-budget inputs.
3. Add finite/invalid position and range tests with deterministic rejection or
   ordering behavior.
4. Add cell-ownership tests showing selection cannot leak an owner across the
   policy boundary.
5. Implement the policy without Bevy imports or runtime world access.

## Integration gates

After both commits are integrated on `M6-OutCell`, run:

```powershell
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test
cargo check-dev
cargo run-dev -- prepare --help
```

The current repository has known unrelated full-format drift in
`src/cli/tests/mod.rs`, `src/viewer/console/render_commands.rs`,
`src/viewer/controls.rs`, `src/viewer/tests/agent_bridge.rs`,
`src/viewer/tests/controls.rs`, and `src/viewer/tests/ragdoll_lab.rs`.
The orchestrator must report that baseline separately and use targeted rustfmt
for changed files; those files are outside this wave.

No live W5 exit claim is valid until W5-C runs against prepared real data and
proves exterior weather transition, interior ImageSpace/irradiance isolation,
and cell-owned light teardown. W2 current-v21 route evidence and W3 runtime
actor acceptance remain independent pending gates.

## Shipped amendments

_Wave kickoff; no implementation commits integrated yet._
