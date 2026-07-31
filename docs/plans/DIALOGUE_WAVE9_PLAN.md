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

### Voice import and completion timing

- Added explicit repeatable `--dialogue-voice-manifest` input for workspace-
  relative Yarn line-to-WAV/OGG mappings. Preparation validates line keys and
  WAV/OGG headers, stages clips content-addressed below `.bevyout`, and writes the
  versioned `dialogue/voice_index.ron` artifact.
- Bumped prepared dialogue bundles to `dialogue-bundle-v4`; the bundle
  fingerprint now includes the prepared voice index. Older prepared bundles remain
  readable without voice playback.
- Dialogue voice entities use `PlaybackMode::Once`. `AudioSink` and
  `SpatialAudioSink` completion drives line continuation; missing or stalled
  assets enter deterministic text timing after a one-second load grace period.
- `dialoguestate` now reports `voice_state` and `timing_source`, and traces
  `voice complete line=<key> timing=Audio|Text`.
- Raw voice files remain external inputs; no audio assets are committed.

### Cell-scoped Fallout voice discovery

- Added `--dialogue-voice-discover` and `.bevyout`-validated
  `--dialogue-voice-report PATH` preparation inputs. Supplying an authored
  `--dialogue-source` enables discovery automatically; the flag remains a
  force/explicit form for cell-only discovery. Discovery is limited to
  initially enabled voice-capable actors present in the selected cell and
  writes `dialogue/voice_demand.ron` alongside the prepared bundle.
- The resolved Fallout `DIAL`, `INFO`, `QUST`, and `VTYP` data uses stable
  `fallout:<plugin>:<info-form-id>:<response-number>` line keys. Load-order
  winners, actor demands, missing voice types, missing responses, source paths,
  and archive provenance are retained in deterministic diagnostics.
- Loose voice files and the supported voice archives are searched by exact
  normalized virtual path. Identical bytes are content-addressed and reused
  across cell bundles; the viewer never scans archives or production voice
  folders at runtime.
- WAV and OGG are both accepted. The Bevy 0.19 `vorbis` audio feature is
  enabled explicitly, so preparation validates and fingerprints the source
  OGG and stages its original bytes with an `.ogg` extension. No unnecessary
  transcoding is performed; WAV remains supported as a direct input format.
- Cell-scoped artifacts are written below the shared prepared cache using
  `scenes/<cell-form-id>/dialogue/...`, while the scene manifest continues to
  resolve all prepared assets from its cache root. Content-addressed audio is
  shared at the cache-root `audio/` directory so identical bytes are staged
  once across cells. Fallout-derived audio stays under `.bevyout` and is never
  committed.

### Automatic readiness and exact actor conversations

- Normal cell preparation now always runs cell-scoped dialogue/voice discovery;
  `--dialogue-voice-discover` remains only as a compatibility no-op. Preparation
  reports total, mapped, authored-missing, and Fallout-discovery-missing lines,
  lists every missing stable key, and prints a concrete follow-up prepare
  command.
- Bumped the prepared bundle and voice index revisions to v5 and the prepared
  catalogue to v4. Voice identity is the compound `(line key, actor reference)`
  key, so a Fallout line never borrows another actor's clip. Source and staged
  hashes are validated before render readiness succeeds.
- Render performs the same prepared coverage check before viewer launch.
  Incomplete coverage intentionally allows visual rendering, but only after a
  labelled `TEXT-FALLBACK` warning containing the missing keys and follow-up
  command. A render-triggered prepare recovers the existing authored sources
  and explicit authored voice entries before merging fresh discovery output.
- Present actors receive generated actor-specific Yarn rooted at the lowest
  stable resolved `GREETING` INFO. Spoken text and OGG voice use the exact
  `fallout:<plugin>:<info>:<response>` identity. Player options come only from
  explicit INFO topic links or top-level DIAL records with non-empty authored
  `FULL` labels; EDID-only internal transitions are not shown as choices.
- The synthetic `dialogue/authored/moira_brown.yarn` remains a valid explicit
  authored source, but it is not bound to Moira and is never paired with
  approximate Fallout audio. Moira's placement is bound to the generated
  actor conversation instead.
