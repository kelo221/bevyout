# ESC pause menu — manual acceptance

Short summary: pressing Esc while in-game opens a Fallout-style pause stack on
the right (Continue / Save / Load / Settings / Help / Quit), freezes gameplay
time, captures a low-res blurred freeze-frame of the 3D view, then suspends the
world camera so the menu is power-cheap. Only **Continue** and **Quit** do
anything; the rest are dimmed placeholders.

## Prerequisites

1. A prepared cell, e.g.:

   ```bash
   cargo run-dev -- prepare --cell 000151e3
   ```

2. Launch the viewer:

   ```bash
   cargo run-dev -- view --cell 000151e3
   ```

## Steps

1. Wait until the cell is fully visible and the cursor is locked (FPS look).
2. Press **Esc**.
   - **Expected:** Gameplay freezes (NPCs/player stop). The 3D view becomes a
     soft, amber-tinted blur. A right-aligned monofonto stack appears:
     Continue, Save, Load, Settings, Help, Quit. Continue is highlighted.
     CRT grid lines, corner ticks (▲/▼), scanlines, and a vignette frame the
     screen. Cursor is free.
3. Press **↓** / **S** a few times, then **↑** / **W**.
   - **Expected:** Highlight moves through every entry (including dimmed ones)
     and wraps at both ends. Dimmed entries stay unreadable-as-active.
4. Hover the mouse over **Quit**, then over **Continue**.
   - **Expected:** Selection follows the pointer on enabled rows only.
5. Click a dimmed row (Save / Load / Settings / Help).
   - **Expected:** No action; menu stays open.
6. Press **Enter** (or click **Continue**, or press **Esc** again).
   - **Expected:** Menu closes, cursor re-locks, 3D rendering resumes, gameplay
     time unpauses.
7. Press **Esc** again to reopen. Highlight **Quit** and press **Enter** (or
   click Quit).
   - **Expected:** The application exits cleanly (`AppExit::Success`).

## Visual reference

Match `Screenshot_20260721_112658.png` (FO3 pause chrome): right stack, green
phosphor type, heavy scene blur, grid + ticks, no title box.
