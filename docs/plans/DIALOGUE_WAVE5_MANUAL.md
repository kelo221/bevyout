# Dialogue Wave 5 — Moira Brown manual acceptance

This slice adds an authored Moira Brown conversation and a Fallout-style
interactive dialogue view to the prepared MegatonCratersideSupply scene.
Aim at Moira, press the existing use key (E), click through the spoken line,
choose one of the three option rows with the mouse, and confirm that the
dialogue modal and input gate close when the conversation ends.
The authored text is an original smoke fixture; it is not a canonical
Fallout DIAL/INFO import.

## 1. Prepare the exact scene and source

From the repository root:

    cargo run-dev -- prepare --cell 00003a2a --dialogue-source dialogue/authored/moira_brown.yarn

Expected output includes:

    dialogue bundle: 4 conversation(s), 1 source(s), 1 stable placement binding(s) -> dialogue/catalog.ron

Expected prepared files are written below .bevyout/cache/scenes/00003a2a/
and the shared prepared asset root .bevyout/cache/dialogue/. The scene manifest has a
non-empty dialogue bundle whose source list contains
authored/moira_brown.yarn; no dialogue artifact is written outside
.bevyout/.

## 2. Launch the prepared viewer

    cargo run-dev -- view --manifest .bevyout/cache/scenes/00003a2a/scene.ron --agent-bridge --agent-port 15702 --trace-seconds 90

Expected result: MegatonCratersideSupply opens and the player can look
around in FPS mode. The prepared Moira reference is 0002D2BC.

## 3. Inspect the prepared runtime

From another PowerShell window:

    $body = '{"jsonrpc":"2.0","id":1,"method":"bevyout.session","params":{}}'
    Invoke-RestMethod http://127.0.0.1:15702/ -Method Post -ContentType 'application/json' -Body $body

    $body = '{"jsonrpc":"2.0","id":2,"method":"bevyout.scene_snapshot","params":{}}'
    Invoke-RestMethod http://127.0.0.1:15702/ -Method Post -ContentType 'application/json' -Body $body

Expected result: the session reports the Megaton supply cell, and the scene
snapshot contains Moira Brown at reference 0002D2BC.

If Moira is not near the initial player position, use the existing bridge
position command one axis at a time, then turn the camera toward her:

    player.setpos x 7.0
    player.setpos y -0.96
    player.setpos z -0.35

## 4. Start by pressing the use button

Aim at Moira until the HUD reads:

    [E] Talk to Moira Brown

Press E.

Expected result: the existing GameplayModal::Dialogue opens, the mouse
cursor is visible for UI interaction, normal gameplay input is gated, the
camera is focused for dialogue, and Moira says inside a dark translucent
panel with no visible border:

    I have a few things to ask about, if you have a moment.

The speaker name is aligned to the upper-right of the panel. The spoken line
has no visible button border, but its transparent hit target remains clickable.
With no prepared voice asset in this authored smoke fixture, the line uses the
deterministic text-duration fallback and advances automatically after that
duration. Clicking the spoken line advances immediately instead.

Inspect it through the bridge:

    $body = '{"jsonrpc":"2.0","id":3,"method":"bevyout.console.exec","params":{"line":"dialoguestate"}}'
    Invoke-RestMethod http://127.0.0.1:15702/ -Method Post -ContentType 'application/json' -Body $body

Expected result: ok is true, active is true, phase is PresentingLine,
modal is Dialogue, and input_gated is true. With Moira's prepared actor
hierarchy loaded, the same state reports `voice_anchor: Mouth` and
`voice_spatial: true`. The authored smoke conversation currently has no
prepared voice asset, so this bridge check verifies the mouth-anchor
resolution; a voiced line is required to verify the audible spatial playback.

## 5. Continue to the options

Wait for the opening line to advance automatically, or click the spoken line
once. Space and Enter remain valid keyboard alternatives. The spoken line
should disappear, leaving the speaker name and three selectable options inside
one bordered outer frame:

    1. Ask about the crater
    2. Ask about supplies
    3. Say goodbye

Move the mouse over an option. Expected result: its phosphor background
highlight appears without adding a row border. Click the row.

The same transition can be driven through the bridge for non-pointer smoke
testing with:

    $body = '{"jsonrpc":"2.0","id":4,"method":"bevyout.console.exec","params":{"line":"dialoguecontinue"}}'
    Invoke-RestMethod http://127.0.0.1:15702/ -Method Post -ContentType 'application/json' -Body $body

Expected result: dialoguestate reports phase PresentingOptions and three
enabled options.

## 6. Exercise a branch and deferred action

Choose option 1 by clicking its row, pressing 1, or issue:

    $body = '{"jsonrpc":"2.0","id":5,"method":"bevyout.console.exec","params":{"line":"dialoguechoice 1"}}'
    Invoke-RestMethod http://127.0.0.1:15702/ -Method Post -ContentType 'application/json' -Body $body

Expected result: Moira presents the crater response. The response auto-
advances after its prepared voice duration, or clicking its line skips
forward. Continue once, choose the branch's Continue option, and continue
through the closing line. After
the next update, dialoguestate reports active false, modal None, and
input_gated false. The deferred moira_crater_questioned action is applied
exactly once in the dialogue host trace.

Repeat from step 4 and click option 2 (or use dialoguechoice 2). Expected result:
the supplies response appears and the host trace contains
moira_supplies_questioned, also exactly once.

## 7. Verify the no-op goodbye branch

Repeat from step 4, continue to the options, and click option 3 (or use
dialoguechoice 3).

Expected result: Moira presents the closing line, the dialogue ends normally,
and the next dialoguestate shows the gameplay modal and input gate restored.

All generated preparation and runtime evidence stays under .bevyout/.
