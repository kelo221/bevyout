# Dialogue Wave 9 plan — voice, localization, and presentation polish

## Execution model recommendation

Use **Sol X-High** in the Codex runtime. Execute last, after the authored and
imported dialogue paths and checkpoint saves are accepted.

## Fixed provider boundaries

- Voice assets resolve through a prepared index keyed by stable line key.
- Localization and speaker names resolve through explicit prepared manifests
  and the runner's selected language.
- Subtitle timing, typewriter, skip, fast-forward, prefetch, and accessibility
  are presentation policies over `DialoguePresentation`.
- Voice/camera/animation waits use task-backed completion signals; playback
  itself never mutates dialogue state.
- Authored hot reload is opt-in and never becomes production folder scanning.
- Release builds validate explicit source lists and content hashes.

## Test-first order

1. Add `@dialogue-wave9` scenarios for language selection, missing assets,
   subtitle timing, skip behavior, accessibility settings, and provider
   completion.
2. Add provider and timing unit tests with synthetic assets.
3. Add stable diagnostics for runner failures, unsupported records, missing
   lines, and dialogue timing.
4. Run final authored and imported runtime acceptance.

## Acceptance gate

- Presentation providers remain separate from world-state mutation.
- Missing voice/localization assets produce recoverable diagnostics.
- Language and subtitle behavior is deterministic and testable.
- Production builds never scan implicit dialogue folders.
- Coverage and timing reports retain source provenance.

Depends on Waves 5, 7, and 8. Write `DIALOGUE_WAVE9_MANUAL.md` before the PR
with final visual, audio, and accessibility evidence.

## Shipped amendments

<!-- Record acceptance-driven changes here; do not rewrite the fixed plan. -->
