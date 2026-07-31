# Dialogue Wave 3 plan — Yarn host bridge

## Execution model recommendation

Use **Sol X-High** in the Codex runtime. Execute after M7's typed registry and
deterministic Bevy adapter seams are available; do not duplicate them here.

## Fixed interfaces

- Centralize registration in `install_bevyout_yarn_api` and return structured
  registration errors.
- Use `bo_*` for stable Bevyout APIs and reserve `fo3_*` for compatibility
  operations.
- Pure functions are synchronous and side-effect-free.
- Mutating commands enqueue existing typed host commands and report completion
  through the existing deferred pipeline.
- Async commands return task/completion handles and never block the update
  thread.

Initial functions include condition, item count/ownership, globals, quest
stage/completion, actor values, skill checks, and reference enabled state.
Initial commands include action sets, quest stage, item add/remove, enable /
disable, script events, scene begin/end, and dialogue end.

## Test-first order

1. Add `@dialogue-wave3` scenarios for pure branching, deferred mutation,
   async completion, failure reporting, and exactly-once execution.
2. Add registry signature and deterministic report tests.
3. Add the narrow Yarn adapter over M7 services.
4. Run the same dialogue twice and compare command traces/state hashes.

## Acceptance gate

- A test dialogue queries game state, branches, performs one deferred mutation,
  waits for it, presents the next line, and completes.
- Repeated execution produces the same command sequence.
- A failed or unsupported function/command reports its descriptor and support
  level without mutating state.
- No Yarn function or command can mutate arbitrary `World` state.

Depends on Wave 2 and M7 typed-registry/adapter work. Write
`DIALOGUE_WAVE3_MANUAL.md` before the PR if the bridge is exposed through the
viewer or agent bridge.

## Shipped amendments

<!-- Record acceptance-driven changes here; do not rewrite the fixed plan. -->
