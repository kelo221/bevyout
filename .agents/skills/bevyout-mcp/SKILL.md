---
name: bevyout-mcp
description: Use when an agent needs to inspect or modify a live bevyout Bevy scene, execute structured console commands, profile bounded frame windows, inspect schedule parallelism constraints, call raw BRP methods, capture screenshots, or turn a runtime reproduction into a .bscript transcript.
---

# Bevyout MCP

## Connect

Install the local adapter with `bun run tools/bevyout-mcp/src/install.ts --all`.
Launch or attach with `viewer_launch`, or start the viewer directly:

```powershell
cargo run-dev -- render SuperDuperMart --agent-bridge
```

The viewer starts in FPS mode. If physics is disabled or the scene has no usable
collision, FPS starts in forced no-clip so WASD and Space/Ctrl still work.

## Inspect and act

1. Call `scene_snapshot` for compact placements, cameras, lights, names, parent
   IDs, transforms, and Fallout metadata.
2. Use `world_query` for reflected components and filters.
3. Call `console_help`, then use `console_exec` for Gamebryo-style commands.
   Useful commands include `prid`, `dump`, `getpos`, `setpos`, `tfc`, `tcl`,
   `tcg`, `tlights`, `stairdebug`, `tunlit`, `fov`, `getrender`, `setrender`,
   `renderreport`, `tm`, `tdt`, `sgtm`, and `screenshot`.
4. Use `brp_call` for reflected BRP mutations not covered by the console, and
   `brp_watch` for methods ending in `+watch`.
5. Re-query or take another snapshot after mutation. Normal game systems may
   overwrite component values on the next update.

## Profile performance

1. Use `performance_probe` for a timed, bounded frame window. Set warmup,
   duration, frame budget, and sample limit explicitly when recording evidence.
2. Use `performance_snapshot` to inspect recent samples or collect raw frames
   strictly after a known sample marker.
3. Use `schedule_snapshot` to inspect initialized schedules, exclusive/non-send
   systems, and ECS access-conflict pairs that constrain parallel execution.
4. Treat schedule conflicts as constraints, not measured bottlenecks. Correlate
   them with frame probes or a trace before recommending an optimization.

Use `.agents/skills/bevy-performance-audit/SKILL.md` for the full audit
workflow.

Console selection is per session. In the human console, click visible placement
geometry to select its placement root and show `(<formid>)` at the top; `prid`,
`dump`, and later reference commands use that same UI selection. MCP has its own
stable process session, so select agent targets with `prid` or explicit
`reference.command` syntax.

## Capture and reproduce

Use `viewport_capture` for an MCP image of the primary window. The console
`screenshot [name]` command accepts only sanitized names and writes under
`.bevyout/screenshots/`.

Turn repeatable bugs into line-oriented `.bscript` files and run them with:

```powershell
cargo run-dev -- script run .bevyout/scripts/repro.bscript --headless --transcript .bevyout/scripts/repro.jsonl
```

Treat entity IDs as valid only for the current viewer session. All console and
BRP changes are runtime-only: they do not modify prepared manifests, GLB assets,
or Fallout source data.

The bridge binds to `127.0.0.1`. Do not expose raw BRP to a LAN without adding
authentication and completing a security review.
