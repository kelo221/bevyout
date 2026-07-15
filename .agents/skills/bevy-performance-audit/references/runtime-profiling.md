# Runtime profiling reference

## MCP probes

Use the project-local bevyout MCP server and the `bevyout-mcp` skill for launch,
scene control, screenshots, and raw BRP access.

`performance_snapshot` accepts:

- `afterSample`: select samples strictly newer than this monotonic ID;
- `latestLimit`: retain the newest 1–600 eligible frames;
- `budgetMs`: count frames strictly greater than this budget;
- `includeSamples`: return raw selected frame records.

It returns `latest_sample`, bounded window statistics, finite Bevy diagnostics,
and compact world counts.

`performance_probe` waits outside the game, captures a start marker, waits for
the requested duration, then summarizes only new frames. Specify `warmupMs`,
`durationMs`, `latestLimit`, `budgetMs`, and `includeSamples` rather than relying
on defaults in recorded evidence. A 600-frame ring cannot preserve a longer
window at high frame rates; shorten the duration or accept truncation.

`schedule_snapshot` accepts `scheduleContains`, `includeSystems`, and
`conflictLimit`. It returns system counts, exclusive/non-send/deferred traits,
and bounded conflict pairs with component/resource names.

## Raw BRP fallback

Call `bevyout.performance_snapshot` or `bevyout.schedule_snapshot` over the
loopback JSON-RPC endpoint when MCP is unavailable. Keep waits in the client.
Never expose the bridge beyond loopback without authentication and review.

## Measurement protocol

1. Record commit/worktree state, Bevy version, build profile/features, platform,
   resolution, present mode, frame budget, and scenario.
2. Separate startup, cold-cache, warm steady-state, and transition windows.
3. Repeat the same window at least three times when variance matters. Report
   individual runs or range, not only a favorable average.
4. Hold content, camera, render settings, physics, and machine state constant.
5. Correlate raw frame IDs with stable logs around transitions.
6. Use A/B controls for render or gameplay settings; restore state afterward.
7. Escalate to tracing for attribution. Aggregate percentiles identify a
   regression but not the responsible system or GPU pass.

For bevyout, treat the collision-cook startup line as a thermal canary and
retry transient Metal `DeviceLost`. Occluded macOS captures may be black; use
snapshots and logs as evidence instead.
