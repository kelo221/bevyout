# Architecture wave 4 — viewer console capability modules (#146)

Wave under epic #142 on branch `Refactor`; it builds on waves 1–3.

## Fixed feature list

1. Keep `src/console/` as the engine-independent grammar/registry/executor
   layer and make `viewer::console` a thin Bevy adapter/plugin.
2. Split viewer command installation and execution by domain: common parsing,
   player/world, items/interaction, render/debug, and navigation adapters.
3. Move the large unit-test module beside the capability modules while keeping
   its minimal-App harness shared.
4. Keep `console_ui` and `agent_bridge` on the same structured
   request/result/session types.

## Tests before implementation

- Existing registry/help/alias tests and every viewer console test remain the
  behavior specification.
- Script golden transcripts remain byte deterministic.
- Direct executor and BRP execution return the same structured result shape.
- Add a source-size/module-surface guardrail for the former God module.

## Gate

Focused console/script/bridge tests, Cucumber, then the full Rust gate.

## Shipped amendments

None yet.
