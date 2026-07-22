---
name: cinema
description: Use when an agent needs to visually verify motion in a live bevyout scene -- actor walking/navigation, door travel, animation playback, camera framing -- by driving the #209 cinema debug camera and capturing a filmstrip instead of reasoning from a single screenshot.
---

# Cinema Visual Verification

A single screenshot cannot show whether an actor is walking, a door swung
open, or an animation played. The cinema camera (`cam ...` console commands,
issue #209) frames a subject; `cinema_record` (issue #210, `tools/bevyout-mcp`)
captures several viewport frames over time so motion is visible across the
sequence. Use `.agents/skills/bevyout-mcp/SKILL.md` for the general
connect/inspect/act workflow -- this skill only covers the filmstrip step.

## Workflow

1. Launch the viewer with the bridge and keep the window visible/foreground
   for the whole session -- see the gotcha below.

   ```powershell
   cargo run-dev -- view --manifest .bevyout/cache/scenes/<formid>/scene.ron --agent-bridge
   ```

2. Resolve the subject's FormID. Use `scene_snapshot` or `console_exec` with
   `prid`/`dump` to confirm the reference exists in this session before
   pointing the camera at it.
3. Put the subject on-mesh before driving navigation. `tna bind <actor-formid>`
   binds a nav agent to an existing actor reference; `tna goto <x> <y> <z>`
   drives it (an optional leading `<index>` selects a non-default agent).
   `tna status` reports `grounded`/`AgentNotOnNavMesh` --
   an ungrounded agent will not move no matter what the camera shows. Prefer
   open floor: cluttered cells and doorways wedge agents on approach.
4. Engage the camera directly with `console_exec`, or let `cinema_record` do
   it for you (it issues the `cam` line itself):
   - `cam follow <FormID> [dist] [height]` trails a moving subject.
   - `cam orbit <FormID> <radius> [deg_per_sec]` circles a mostly-stationary
     one (animation playback, an opened door).
   - `cam path follow <FormID> [seconds]` dollies from the current camera
     position to the target -- useful for door-travel or long walks.
   - `cam release` restores the previous camera (fps/free) when done.
5. Drive the scenario in the same session the camera is watching: `tna goto`,
   `activate <door-formid>`, `actoranim <formid> <clip>`, etc.
6. Call `cinema_record` with the subject FormID:

   ```json
   { "subject": "0005cf10", "frames": 4, "intervalMs": 500, "mode": "follow" }
   ```

   It returns an ordered sequence of frame images, each preceded by a text
   line with that frame's `cam status` and `tna status` (when a nav agent is
   bound). Read the sequence in order -- a changing position/angle across
   frames is the motion evidence; a single frame is not.
7. Assess: does the subject's position/orientation change frame-to-frame in
   the direction the scenario should produce? Cross-check against the stable
   `cinema follow/orbit/path start/done` and `nav agent handoff`/`tna` log
   lines (`viewer_logs`) as scriptable, non-visual evidence for the same
   claim.

## Gotchas

- **Black/empty frames are a foreground problem, not a bug.** macOS returns a
  0-byte capture when the game window is occluded or backgrounded.
  `cinema_record` detects this per frame and marks it `EMPTY/black` in its
  text line rather than failing the call -- but the fix is to keep the window
  visible and frontmost while you drive the scenario, not to trust an empty
  filmstrip as evidence of anything.
- **Interiors are dark.** A subject framed correctly can still be hard to read
  visually; prefer `cam status`/`tna status`/log-line evidence over pixel
  inspection when the cell has no strong light near the subject.
- **Off-mesh actors don't move.** `tna goto`/`tna bind` on a reference that
  isn't grounded on the nav mesh reports `AgentNotOnNavMesh` and the camera
  will faithfully film a subject standing still. Reposition with
  `player.setpos <x|y|z> <metres>` (one axis at a time) or pick a different
  start before blaming the camera or the nav system.
- **Cluttered cells wedge agents at doors.** If a followed actor stalls
  mid-route, check whether it's an ORCA/door-clearance stall before assuming
  the camera lost track of it -- pick open floor for a clean demonstration.
- **Entity IDs and console sessions are per-viewer-session.** A filmstrip
  recorded against one viewer process means nothing once that process exits;
  re-resolve the subject after a relaunch.
