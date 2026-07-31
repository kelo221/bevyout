# Dialogue Wave 0 plan — Bevy 0.19 Yarn compatibility spike

## Execution model recommendation

Use **Sol X-High** in the Codex runtime. Execute directly and sequentially;
the dependency and smoke fixture establish the contract for every later wave.

## Fixed feature list

- Add optional `bevy_yarnspinner` behind the `dialogue-yarn` feature.
- Use a Bevyout-controlled fork and pin the exact mirrored Bevy 0.19 commit;
  record the final fork URL in the shipped amendment once available.
- Add one explicit `.yarn` fixture under the repository's synthetic fixture
  area and an opt-in `examples/yarn_smoke.rs` runner.
- Observe line, options, and completion events directly. Register one pure
  function and one deferred command in the smoke harness.
- Keep all production code independent of
  `bevy_yarnspinner_example_dialogue_view`.

## Test-first order

1. Add the `@dialogue-wave0` scenarios and a small Rust smoke test.
2. Check the feature graph and exact lockfile revision.
3. Implement the optional dependency and smoke runner.
4. Run default and Yarn-enabled checks plus a dependency-tree inspection.

## Acceptance gate

- `cargo check --all-targets` passes with default features.
- `cargo check --all-targets --features dialogue-yarn` passes.
- The exact Yarn revision is present in `Cargo.lock`.
- `cargo tree` contains no accidental Bevy 0.18 runtime copy.
- The smoke runner compiles one Yarn file, starts one node, receives a line,
  receives an option, invokes one function and command, and completes.
- No production Bevyout target references the example dialogue view plugin.

## Out of scope

Production domain types, prepared manifests, viewer UI, save records, Fallout
record decoding, and real-game acceptance.

## Manual and shipped records

Before the wave PR, write `DIALOGUE_WAVE0_MANUAL.md` if the smoke harness has a
player-visible or agent-driven runtime surface. Keep a `Shipped amendments`
section here for the final fork URL, measured dependency tree, and deviations.

## Shipped amendments

- The compatibility spike uses the standalone, Bevy-free `yarnspinner` 0.8.0
  runtime behind `dialogue-yarn`. The upstream `bevy_yarnspinner` 0.8.0
  integration currently requires Bevy 0.18.0, so it cannot be enabled in this
  Bevy 0.19 workspace without introducing a second Bevy runtime. The
  Bevyout-controlled fork URL and pinned Bevy 0.19 revision remain a Wave 0
  prerequisite for replacing this boundary; no URL is invented here.
- The synthetic smoke fixture now invokes a pure `bo_smoke` function and emits
  a deferred `bo_smoke_command` event while observing line, options, and
  completion events. It has no example UI dependency.
