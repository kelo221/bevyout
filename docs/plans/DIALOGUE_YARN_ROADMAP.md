# Standalone Yarn dialogue roadmap

This roadmap splits the Yarn Spinner integration into an independent
`DIALOGUE_WAVE0`–`DIALOGUE_WAVE9` sequence. It does not renumber the existing
M7 scripting waves.

## Recommended direction

Yarn Spinner is a replaceable dialogue-flow adapter:

- Yarn owns authored conversation flow, lines, choices, and narrative-local
  variables.
- Bevyout owns actors, quests, inventory, conditions, scripts, saves, input
  modes, camera state, voice assets, and world mutations.
- The merged scripting registry, condition evaluator, and deferred command
  pipeline remain the only authoritative route into game state.
- Imported Fallout dialogue is prepared and normalized by Bevyout before Yarn
  sources are generated. Yarn is not a second Fallout runtime.

```text
prepared authored/imported dialogue
              |
              v
     PreparedDialogueBundle
              |
NPC activation -> DialogueStartRequest -> persistent Yarn runner
                                      |             |
                              presentation       host bridge
                                      |             |
                               Bevyout UI     typed game services
```

## Permanent boundaries

- `crates/bevyout-core/src/dialogue/` contains Bevy-free dialogue IDs,
  requests, phases, presentation DTOs, snapshots, and errors.
- `src/vsa/dialogue/` owns prepared bundles, catalogs, source validation,
  generated Yarn, Fallout normalization, and deterministic reports.
- `src/viewer/dialogue/` owns Yarn types, runner entities, event routing, UI,
  input handling, host registration, and diagnostics.
- Stable Fallout/runtime object IDs may enter domain contracts and saves;
  Bevy `Entity`, Yarn `LineId`, Yarn `OptionId`, and runner internals may not.
- One persistent local-player runner is the initial runtime model. NPCs hold
  only stable `DialogueBinding` data.
- Generated and authored Yarn sources remain separate. Preparation never edits
  authored files and production runtime uses an explicit sorted source list.
- Yarn commands call typed Bevyout services and enqueue deferred commands; they
  never receive arbitrary `&mut World` access.

## Dependency alignment with M7

- Waves 0–2 can proceed before the M7 runtime exists.
- Dialogue Wave 3 consumes the typed registry and deterministic Bevy adapter
  from M7 Waves 4–5 rather than implementing a duplicate registry.
- Dialogue Wave 4 aligns its save barrier and snapshot compatibility with M7
  Wave 6.
- Dialogue Waves 6–8 consume the M7 record stream/script catalog and the
  eventual condition, quest, and effect authorities from M7 Wave 7.

## Delivery sequence

Each wave has a kickoff prompt and an executable plan. A manual acceptance file
is created before that wave's PR, after real prepared content and exact runtime
commands are known.

| Wave | Focus | Documents |
|---|---|---|
| 0 | Bevy 0.19 compatibility spike and Yarn smoke runner | [prompt](DIALOGUE_WAVE0_PROMPT.md), [plan](DIALOGUE_WAVE0_PLAN.md) |
| 1 | Bevyout domain contracts and prepared dialogue bundle | [prompt](DIALOGUE_WAVE1_PROMPT.md), [plan](DIALOGUE_WAVE1_PLAN.md) |
| 2 | Persistent runner, session lifecycle, schedule, UI, and input gating | [prompt](DIALOGUE_WAVE2_PROMPT.md), [plan](DIALOGUE_WAVE2_PLAN.md) |
| 3 | Yarn host bridge over shared scripting authorities | [prompt](DIALOGUE_WAVE3_PROMPT.md), [plan](DIALOGUE_WAVE3_PLAN.md) |
| 4 | Narrative variables and boundary-only saves | [prompt](DIALOGUE_WAVE4_PROMPT.md), [plan](DIALOGUE_WAVE4_PLAN.md) |
| 5 | Authored NPC activation vertical slice | [prompt](DIALOGUE_WAVE5_PROMPT.md), [plan](DIALOGUE_WAVE5_PLAN.md) |
| 6 | Fallout dialogue inventory and compatibility catalog | [prompt](DIALOGUE_WAVE6_PROMPT.md), [plan](DIALOGUE_WAVE6_PLAN.md) |
| 7 | First generated Fallout conversation | [prompt](DIALOGUE_WAVE7_PROMPT.md), [plan](DIALOGUE_WAVE7_PLAN.md) |
| 8 | Explicit checkpoint-node save/resume | [prompt](DIALOGUE_WAVE8_PROMPT.md), [plan](DIALOGUE_WAVE8_PLAN.md) |
| 9 | Voice, localization, accessibility, and coverage polish | [prompt](DIALOGUE_WAVE9_PROMPT.md), [plan](DIALOGUE_WAVE9_PLAN.md) |

## Cross-wave contracts

- Add the optional `dialogue-yarn` feature in Wave 0. Keep the exact
  Bevyout-controlled YarnSpinner fork URL and mirrored Bevy 0.19 revision in
  the Wave 0 plan amendment once the fork exists.
- Add `PreparedDialogueBundleRef` to the prepared manifest with explicit Yarn
  source paths, node index, optional voice/localization indexes, and a content
  fingerprint. Bump the prepared revision for every serialized shape change.
- Extend the viewer schedule to
  `Input -> Interaction -> Dialogue -> WorldSync -> Ui`.
- Reuse `GameplayModal::Dialogue`; do not add a second modal system.
- Define `DialogueSnapshot` with persistent variables, bundle hash, and an
  optional `ActiveDialogueCheckpoint`. Boundary-only saves leave the
  checkpoint empty; Wave 8 activates it without serializing the Yarn runner.
- Add a versioned `DLOG` save record under save format v6.
- Keep `PreparedDialogueCatalog` as the compatibility source of truth for
  imported conversations, condition-set keys, action-set keys, line keys,
  source mappings, and diagnostics.

## Common validation

- Every wave adds fully bound scenarios to `features/dialogue.feature` and
  dedicated unit tests; production implementation files do not contain inline
  test modules.
- Default-feature builds remain valid. Yarn-enabled validation uses
  `cargo test --all-targets --features dialogue-yarn`.
- Repository gates remain `cargo fmt --check`, strict clippy, and `cargo test`.
- Real-data runs write only under `.bevyout/`; Bethesda-derived Yarn, RON,
  GLB, audio, and extracted records are never staged.
- Every wave plan recommends **Sol X-High** for direct Codex execution on a
  sequential wave branch.

## Explicit exclusions

- No Yarn example dialogue-view plugin in production.
- No runner per NPC and no cell-scoped authoritative dialogue state.
- No direct serialization of `DialogueRunner` internals.
- No runtime folder scanning for production dialogue sources.
- No arbitrary Fallout condition reimplementation in Yarn.
- No FOSE/NVSE expansion before measured vanilla Fallout 3 coverage.

