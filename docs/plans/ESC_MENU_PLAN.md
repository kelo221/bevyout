# ESC Pause Menu Implementation Plan (Refined from Screenshot Reference)

## Executive Summary

This plan details the implementation of the authentic Fallout ESC Pause Menu for `bevyout`, matched precisely against the reference screenshot (`Screenshot_20260721_112658.png`).

Key design elements verified from the reference:
- **Layout**: Right-aligned menu text stack on the right side of the viewport (no title frame or "PAUSED" box header).
- **Typography & Casing**: Title Case in `monofonto` font (`Continue`, `Save`, `Load`, `Settings`, `Help`, `Quit`).
- **Functional Scope**: `Continue` (resumes gameplay) and `Quit` (exits application). `Save`, `Load`, `Settings`, and `Help` will be rendered as disabled menu items (dimmed phosphor tone).
- **Visual Overlays**: CRT scanlines, grid lines overlay with tick indicators (`▲`, `▼`), curved dark vignette border, and amber/green CRT phosphor color grading over the frozen background.
- **Power Efficiency & Blur**: Captures a low-resolution snapshot of the frame before pausing, applies a downsample blur + phosphor tint, and disables `Camera3d.is_active` to suspend 3D GPU rendering while paused.

---

## Recommended Execution Model

- **Codex runtime**: `Luna X-High` (or `Sol High`)
- **Claude runtime**: `Opus` (Orchestrator), `Sonnet` (Executor)

---

## Visual & Structural Specification (From Reference Screenshot)

```
┌───────────────────────────────────────────────────────────────────────────┐
│ [▲] Grid Line                          Vignette Border [▲]                │
│ ┌───────────────────────────────────────────────────────────────────────┐ │
│ │                                                                       │ │
│ │                                                            Continue   │ │
│ │                         +-------------------+                Save     │ │
│ │                         |                   |                Load     │ │
│ │                         |    (Cyan Cursor)  |              Settings   │ │
│ │                         |         ▲         |                Help     │ │
│ │                         +-------------------+                Quit     │ │
│ │                                                                       │ │
│ └───────────────────────────────────────────────────────────────────────┘ │
│ [▼] Grid Line                     (Heavy Scene Blur)   Vignette Border [▼]│
└───────────────────────────────────────────────────────────────────────────┘
```

### 1. Palette & Typography
- **Font**: `resources/pipboy-monofonto.ttf`.
- **Text Alignment**: Right-aligned text column on the right (~85% to 92% screen width).
- **Active Color**: High-intensity Fallout green/phosphor (`srgb(0.55, 1.0, 0.45)` with subtle text glow/shadow).
- **Disabled Color**: Low-intensity green/phosphor (`srgba(0.35, 0.65, 0.30, 0.50)`).
- **Hover/Focus State**: Highlighted text item with increased scale / intensity.

### 2. Fullscreen Background & Overlays
- **Blurred Scene Snapshot**: Low-res (e.g. 320x180) bilinear-filtered frame capture with amber/greenish CRT tint overlay (`srgba(0.12, 0.15, 0.05, 0.40)`).
- **CRT Grid Lines**: Semi-transparent grid lines dividing screen space with top and bottom tick markers (`▲`, `▼`).
- **Scanlines**: Repetitive horizontal scanline texture overlay.
- **Vignette**: Radial gradient / border frame darkening towards the screen edges.

---

## Architectural Breakdown & Systems

### Phase 1: Pure Domain & State Machine (`crates/bevyout-core` & `src/viewer/pause_menu/`)

1. `PauseMenuOption` Enum:
   ```rust
   #[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter)]
   pub enum PauseMenuOption {
       Continue,
       Save,     // Disabled
       Load,     // Disabled
       Settings, // Disabled
       Help,     // Disabled
       Quit,
   }
   ```
2. `PauseMenuState` Management:
   - Tracks current selection index (defaulting to `Continue`).
   - Supports keyboard navigation (`Up`/`Down`, `W`/`S`), skipping disabled items or cycling cleanly.
   - Triggers `Continue` -> `GameplayModal::None` and `Quit` -> `AppExit::Success`.

### Phase 2: Frame Snapshot & 3D Render Suspension (`src/viewer/pause_menu/snapshot.rs`)

1. **Snapshot Capture**:
   - On transition `OnEnter(GameplayModal::Paused)`, copy/extract viewport render target to a low-res `Image` asset.
2. **3D Camera Suspension**:
   - Query `Camera3d` and set `camera.is_active = false`.
   - On transition `OnExit(GameplayModal::Paused)`, set `camera.is_active = true`.
3. **Power Efficiency**:
   - Suspends mesh rasterization, depth prepass, shadow passes, bloom, and lighting calculations during pause.

### Phase 3: Monofonto UI & CRT Overlay Assembly (`src/viewer/pause_menu/ui.rs`)

1. **Root UI Node (`ZIndex(1500)`)**:
   - Absolute positioning covering 100% of viewport.
2. **Layer Stack**:
   - **Layer 0**: Blurred scene snapshot `ImageNode`.
   - **Layer 1**: Scanline & vignette overlay node.
   - **Layer 2**: Grid line layout with top (`▲`) and bottom (`▼`) notch tick marks.
   - **Layer 3**: Right-aligned text column node (`right: Val::Percent(8.0)`).
3. **Menu Option Items**:
   - `Continue` (Active, hoverable, clickable)
   - `Save` (Disabled)
   - `Load` (Disabled)
   - `Settings` (Disabled)
   - `Help` (Disabled)
   - `Quit` (Active, hoverable, clickable)

### Phase 4: Input & System Integration (`src/viewer/pause_menu/plugin.rs`)

1. Register `PauseMenuPlugin` in `ViewerPlugins` (`src/viewer/plugins.rs`).
2. Input systems running in `ViewerSet::Ui` gated by `.run_if(in_state(GameplayModal::Paused))`:
   - Keyboard navigation (`Up`/`Down`, `W`/`S`, `Enter`, `Space`, `Escape`).
   - Mouse hover & click interactions for active buttons.
   - Cursor release while in `GameplayModal::Paused`.

### Phase 5: Verification & Automated Tests

1. `features/pause_menu.feature`:
   - Opening ESC menu pauses time and 3D camera.
   - Navigating options updates selection state.
   - `Continue` resumes gameplay.
   - `Quit` emits `AppExit`.
2. `docs/plans/ESC_MENU_MANUAL.md`:
   - Step-by-step human verification script matching visual appearance to `Screenshot_20260721_112658.png`.

---

## File Changes Summary

| File | Action | Purpose |
| :--- | :--- | :--- |
| `crates/bevyout-core/src/pause_menu.rs` | Create | Pure state machine & option list |
| `src/viewer/pause_menu/mod.rs` | Create | Feature module root |
| `src/viewer/pause_menu/snapshot.rs` | Create | Frame capture & 3D camera pause |
| `src/viewer/pause_menu/ui.rs` | Create | Monofonto right-aligned UI & CRT grid overlay |
| `src/viewer/pause_menu/plugin.rs` | Create | Bevy systems & modal state handlers |
| `src/viewer/plugins.rs` | Modify | Register `PauseMenuPlugin` |
| `features/pause_menu.feature` | Create | BDD feature scenarios |
| `tests/features.rs` | Modify | Cucumber step implementations |
| `docs/plans/ESC_MENU_MANUAL.md` | Create | Manual acceptance script matching reference image |

---

## Gate Criteria

- [ ] `cargo check-dev` compiles cleanly without warnings.
- [ ] `cargo test-dev` passes all tests.
- [ ] UI visually matches `Screenshot_20260721_112658.png` (right-aligned `monofonto` stack, CRT grid lines, scanlines, vignette).
- [ ] `Camera3d.is_active` is toggled off during pause, saving GPU power.
- [ ] `Continue` resumes gameplay; `Quit` exits application.
