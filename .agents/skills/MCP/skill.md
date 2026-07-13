# Bevyout agent guide

This file is the shared reference for agents working with the live Bevy viewer.
Codex and Claude do not automatically load an arbitrary `skills.md`; explicitly
read this file when a task involves the Bevy scene or the `bevyout` MCP server.

## Start or attach to a viewer

The bridge is opt-in and listens on loopback HTTP. No WebSocket is required.

```powershell
# Render a cached cell and expose its live ECS.
cargo run-dev -- render SuperDuperMart --agent-bridge

# Open a known manifest.
cargo run-dev -- view --manifest .bevyout/cache/scenes/000151e3/scene.ron --agent-bridge
```

Through MCP, prefer `viewer_launch` when the agent should own the process. It
can attach to an existing viewer or launch one from a selector or manifest.
Use `viewer_status`, `viewer_logs`, and `viewer_stop` for lifecycle checks.

## Read the scene

- Use `scene_snapshot` first for compact bevyout-aware rows: placements,
  players, cameras, lights, names, parent IDs, local/global transforms, and
  Fallout placement metadata.
- Use `world_query` for reflected ECS components and filters. Component names
  are fully qualified Bevy type names, for example:

  ```json
  {
    "components": ["bevy_transform::components::transform::Transform"],
    "limit": 20
  }
  ```

- Use `brp_call` for any instantaneous BRP method that is not covered by a
  convenience tool. Use `brp_watch` for methods ending in `+watch`.

Entity IDs returned by snapshots and queries are the IDs to pass to later BRP
calls. Treat them as valid only for the current viewer session.

## Modify entities

The `brp_call` tool forwards Bevy Remote Protocol methods. Common runtime-only
operations include:

```json
{
  "method": "world.mutate_components",
  "params": {
    "entity": 4294966836,
    "component": "bevy_transform::components::transform::Transform",
    "path": "translation.x",
    "value": 45.0
  }
}
```

Other built-in methods include `world.insert_components`,
`world.remove_components`, `world.spawn_entity`, `world.despawn_entity`, and
`world.reparent_entities`. Only reflected/registered component types can be
read or changed through BRP. Confirm the result with a fresh query or snapshot.

Changes are runtime-only. They are not written to RON manifests, GLB assets, or
source Fallout data, and normal game systems may overwrite an agent's change on
the next update. Avoid despawning entities that own scene infrastructure unless
the task explicitly requires it.

## Screenshots and diagnosis

Use `viewport_capture` after a scene change to return the visible primary window
as MCP image content. Pair it with `scene_snapshot` so visual results can be
matched to entity IDs and transforms. Use `viewer_logs` when the MCP server
launched the viewer; externally launched viewers must be inspected through
their own terminal.

The bridge binds to `127.0.0.1` by default. Do not expose it on a LAN address
without adding authentication and an explicit security review: raw BRP access
can mutate the running ECS.

## Installation

Install all integrations with:

```powershell
bun run tools/bevyout-mcp/src/install.ts --all
```

Individual targets are available with `--codex`, `--claude-desktop`, and
`--claude-code`; add `--dry-run` to preview changes. The installer merges only
the `bevyout` entry, preserves unrelated configuration, and creates a
timestamped `.bevyout` backup before changing an existing file.
