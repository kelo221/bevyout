# Dialogue Wave 0 prompt — Bevy 0.19 Yarn compatibility spike

Establish that the pinned Yarn Spinner runtime can coexist with Bevyout's
Bevy 0.19 dependency graph before adding production dialogue architecture.

- Add the optional `dialogue-yarn` feature and pin the exact revision from a
  Bevyout-controlled Bevy 0.19-compatible YarnSpinner fork.
- Add a small explicit Yarn fixture and `examples/yarn_smoke.rs`.
- Drive one node through a line, an option, one pure function, one deferred
  command, and completion without using Yarn's example dialogue view.
- Prove the default build remains Yarn-free and no Bevy 0.18 runtime copy is
  introduced.

Do not add Fallout import, production UI, save integration, voice playback,
quest behavior, or gameplay code in this wave.
