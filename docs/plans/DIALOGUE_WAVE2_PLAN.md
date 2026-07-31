# Dialogue Wave 2 plan — runner, lifecycle, UI, and input gating

## Execution model recommendation

Use **Sol X-High** in the Codex runtime. Execute sequentially because viewer
schedule ordering, state transitions, and UI focus are one runtime seam.

## Fixed interfaces

- Add `ViewerSet::Dialogue` so the schedule is
  `Input -> Interaction -> Dialogue -> WorldSync -> Ui`.
- Add a persistent `DialogueRuntime` resource with readiness, runner entity,
  active session, pending starts, and the explicit default `Reject` busy
  policy.
- Add `PrimaryDialogueRunner` as a runtime marker only.
- Add `DialogueUiPhase` with hidden, revealing, continue, choice, command, and
  closing states.
- Keep the private mapping from Bevyout `DialogueChoiceId` to Yarn `OptionId`
  inside the viewer adapter.

## Behavior

- Use `try_start_node`; never call the panic-on-busy start path.
- A line continues through `continue_in_next_update` exactly once per input
  edge. Holding the button cannot skip lines.
- An option calls `select_option` and never calls the line continuation path.
- Completion clears presentation, session variables, camera focus, and input
  gating through existing state systems.
- The runner survives NPC despawn and cell unload.

## Test-first order

1. Add `@dialogue-wave2` scenarios for lifecycle, input edges, choices,
   malformed nodes, and control restoration.
2. Add dedicated minimal-App and pure state-machine tests.
3. Add the viewer schedule phase and runtime adapter.
4. Add native UI and keyboard/controller/mouse input paths.

## Acceptance gate

- An authored request starts a session and changes input mode in the same
  frame.
- One press advances at most one line.
- Choice selection maps consistently across keyboard, controller, and mouse.
- Completion restores every changed control and camera state.
- Malformed content reports an in-game diagnostic and closes cleanly.
- Production code contains no example dialogue-view plugin.

Depends on Wave 1. Before the PR, write `DIALOGUE_WAVE2_MANUAL.md` with a
synthetic authored conversation and visible runtime evidence.

## Shipped amendments

<!-- Record acceptance-driven changes here; do not rewrite the fixed plan. -->
