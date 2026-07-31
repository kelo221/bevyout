# Dialogue Wave 9 — explicit voice import and completion timing

This manual verifies that supplied Moira WAV lines are prepared into the
dialogue bundle, played from Moira's prepared mouth anchor, and advance the
Fallout-style dialogue UI when playback finishes.

The WAV files and `dialogue/voice/moira_brown.ron` manifest are external local
inputs. Do not commit them. The manifest must map stable keys such as
`MoiraBrown:0` to workspace-relative `.wav` files.

Use this deterministic RON shape (the revision is optional and defaults to
`dialogue-voice-source-v1`):

    (entries: [
        (line_key: "MoiraBrown:0", source_path: "dialogue/voice/moira_000.wav"),
    ])

## 1. Prepare the scene and voice index

From the repository root, with the supplied WAV files and manifest present:

    cargo run-dev -- prepare --cell 00003a2a --dialogue-source dialogue/authored/moira_brown.yarn --dialogue-voice-manifest dialogue/voice/moira_brown.ron

Expected output includes `voice line(s)` in the dialogue bundle summary. The
prepared `dialogue/voice_index.ron` and content-addressed WAV files exist only
under `.bevyout/cache/`.

## 2. Launch the prepared scene

    cargo run-dev -- view --manifest .bevyout/cache/scenes/00003a2a/scene.ron --agent-bridge --agent-port 15702 --trace-seconds 120

Aim at Moira Brown in MegatonCratersideSupply and press E. The bridge state
should report:

    voice_anchor: Mouth
    voice_spatial: true
    voice_state: Playing
    timing_source: Audio

## 3. Verify audio-driven continuation

While the WAV plays, the spoken line remains visible. After the audio ends,
the trace contains `voice complete line=<key> timing=Audio` and the runner
advances to the next line or options. Options must not appear before the WAV
has finished.

Clicking the spoken-line hit target must stop the current voice and advance
immediately. Selecting an option with the mouse and with its number key must
take the same dialogue branch.

## 4. Verify fallback and cleanup

Temporarily remove or rename one mapped WAV, rerun preparation, and start the
conversation again. After the one-second load grace period,
`dialoguestate` reports `voice_state: Fallback` and `timing_source: Text`; the
line still advances and dialogue does not hang.

Complete the final line. The dialogue modal closes, `active` becomes false,
`input_gated` becomes false, and normal FPS controls return. All runtime logs,
screenshots, and prepared artifacts remain under `.bevyout/`.
