# Dialogue Wave 5 plan — authored NPC vertical slice

## Execution model recommendation

Use **Sol X-High** in the Codex runtime. Execute sequentially after the
runner, UI, host bridge, and boundary-save seams are stable.

## Fixed behavior

- Resolve an NPC's `DialogueKey` through a prepared catalog keyed by stable
  base/reference identity.
- Use a lightweight runtime `DialogueBinding`; never attach the runner or
  authoritative variables to the NPC entity.
- Activation emits `DialogueStartRequest`; the dialogue phase owns starting the
  runner and changing `GameplayModal::Dialogue`.
- Default busy behavior is `Reject`.
- Speaker disable, death, despawn, or cell unload produces a structured
  interruption and restores controls/camera state.
- Provide deterministic agent-bridge inspection and control for dialogue
  state, continue, and choice selection.

## Test-first order

1. Add `@dialogue-wave5` scenarios for activation, two choices, host query,
   mutation, completion, busy policy, and interruption.
2. Add minimal-world tests for stable binding and cleanup.
3. Wire the activation seam and camera/input integration.
4. Run a real authored conversation through the viewer and agent bridge.

## Acceptance gate

- NPC activation starts the expected conversation and changes input mode in the
  same frame.
- One choice queries state and another applies one deferred mutation.
- The same NPC can unload/reload without losing persistent narrative values.
- Speaker/listener host context uses stable IDs.
- Every interruption path restores gameplay state.
- No gameplay system references Yarn `LineId`, `OptionId`, or runner internals.

Depends on Waves 2–4. Write `DIALOGUE_WAVE5_MANUAL.md` before the PR with a
prepared authored NPC and visible runtime evidence.

## Shipped amendments

<!-- Record acceptance-driven changes here; do not rewrite the fixed plan. -->

### 2026-07-31 — Moira Brown authored vertical slice

- Added the explicit authored source
  dialogue/authored/moira_brown.yarn; preparation accepts repeatable
  --dialogue-source paths and never discovers dialogue implicitly.
- NPC roots whose editor_id matches a prepared conversation receive only a
  DialogueBinding; the persistent runner and narrative state remain in the
  dialogue runtime.
- The existing E interaction prompt becomes [E] Talk to Moira Brown when the
  prepared catalog contains MoiraBrown. The bridge exposes dialoguestate,
  dialoguecontinue, and dialoguechoice <index> for deterministic inspection
  and control.
- The authored text is an original smoke conversation, not a Fallout DIAL/INFO
  import. Canonical Moira inventory and compatibility work remains in Waves
  6–7.
- Manual acceptance is recorded in DIALOGUE_WAVE5_MANUAL.md.
