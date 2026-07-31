# Dialogue Wave 9 — automatic Fallout voice readiness and completion timing

This manual verifies that prepared Moira voice lines are discovered only for
actors present in `MegatonCratersideSupply`, played from Moira's prepared mouth
anchor, and advance the Fallout-style dialogue UI when playback finishes.

Normal cell preparation automatically performs cell-scoped Fallout voice
discovery. Source OGG/Vorbis files are preserved and staged content-addressed
below `.bevyout`; WAV remains supported for explicit authored input. Do not
commit raw or prepared Bethesda-derived audio.

For authored Yarn lines, an explicit manifest must map
stable keys such as `MoiraBrown:0` to workspace-relative `.wav` or `.ogg` files.
Cell discovery does not relabel imported `fallout:<plugin>:<info-form-id>:<response>`
keys as authored keys; it only supplies audio for generated Fallout Yarn sources.
The checked-in Moira Yarn is synthetic and deliberately has no approximate
Fallout mapping. Moira's prepared placement uses the generated
`fallout_actor_0002d2bc` conversation instead.

Use this deterministic RON shape (the revision is optional and defaults to
`dialogue-voice-source-v1`):

    (entries: [
        (line_key: "MoiraBrown:0", source_path: "dialogue/voice/moira_000.wav"),
    ])

## 1. Prepare the scene and voice index

From the repository root, with Fallout 3 configured normally:

    cargo run-dev -- prepare MegatonCratersideSupply

No dialogue discovery flag is required. Expected output includes deterministic
`dialogue voice coverage`, `dialogue voice missing keys`, and (when incomplete)
`dialogue voice next command` lines. The prepared
voice index is under `.bevyout/cache/scenes/00003a2a/dialogue/voice_index.ron`,
the demand report is under `.bevyout/cache/scenes/00003a2a/dialogue/voice_demand.ron`,
and content-addressed OGG/WAV files are shared under `.bevyout/cache/audio/`.
The generated Moira source is under
`.bevyout/cache/scenes/00003a2a/dialogue/generated/actors/0002d2bc.yarn`.

## 2. Verify render readiness

    cargo run-dev -- render MegatonCratersideSupply

If every prepared line is mapped, render prints a ready summary. If a preserved
authored source is still missing explicit mappings, render continues for visual
inspection only after printing a labelled `TEXT-FALLBACK` warning with every
missing key and the exact prepare command. That warning is intentional; it must
never be silent.

## 3. Launch the prepared scene and use Moira

    cargo run-dev -- view --manifest .bevyout/cache/scenes/00003a2a/scene.ron --agent-bridge --agent-port 15702 --trace-seconds 120

Aim at Moira Brown in MegatonCratersideSupply and press E. The bridge state
should report:

    line: fallout:fallout3.esm:0001d76a:1
    voice_anchor: Mouth
    voice_spatial: true
    voice_state: Playing
    timing_source: Audio

The trace should include `voice started
line=fallout:fallout3.esm:0001d76a:1` and a `dialogue voice anchor` line with
`voice_anchor="Mouth" voice_spatial=true`. The opening text, line key, and
FemaleUniqueMoira OGG source must agree exactly.

## 4. Verify audio-driven continuation and options

While the voice plays, the spoken line remains visible. After the audio ends,
the trace contains `voice complete line=<key> timing=Audio` and the runner
advances to the next line or options. Options must not appear before the voice
has finished.

Clicking the spoken-line hit target must stop the current voice and advance
immediately. Selecting an option with the mouse and with its number key must
take the same dialogue branch. Every shown option must be an authored DIAL
`FULL` label; internal EDIDs must not appear as player-facing choices.

## 5. Verify fallback and cleanup

Temporarily remove or rename one source voice asset, rerun preparation, and
start the conversation again. After the one-second load grace period,
`dialoguestate` reports `voice_state: Fallback` and `timing_source: Text`; the
line still advances and dialogue does not hang.

Complete the final line. The dialogue modal closes, `active` becomes false,
`input_gated` becomes false, and normal FPS controls return. All runtime logs,
screenshots, and prepared artifacts remain under `.bevyout/`.
