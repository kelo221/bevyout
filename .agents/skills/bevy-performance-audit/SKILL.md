---
name: bevy-performance-audit
description: Audit Rust Bevy games and engines for performance bottlenecks, architecture costs, and missed or constrained parallelism. Use when Codex needs to review Bevy ECS code statically, inspect schedules and system access conflicts, profile frame-time behavior through bevyout MCP/BRP probes, investigate hitches or regressions, compare performance configurations, recommend instrumentation, or verify that a proposed optimization addresses measured evidence.
---

# Audit Bevy Performance

Build an evidence chain from source structure to live behavior. Treat static
matches as investigation leads, schedule conflicts as concurrency constraints,
and frame/trace data as runtime evidence. Do not call a regex hit a bottleneck.

## Establish the contract

1. Read repository guidance, `Cargo.toml`, build aliases, and architecture docs.
2. Identify the scenario, target platform, build profile, resolution, present
   mode, warm/cold cache state, and frame budget. State unknowns.
3. Preserve architecture boundaries. For bevyout, keep `main.rs` dispatch-only,
   use pure policy modules for decisions, and keep bridge code thin.
4. Define the comparison before measuring. Change one variable per A/B run.

## Form static hypotheses

Run the bundled scanner from the project root:

```powershell
bun run .agents/skills/bevy-performance-audit/scripts/scan_bevy_code.ts .
```

Use `--json` for machine-readable output and `--max-per-category N` to bound
large repositories. Read [static-audit.md](references/static-audit.md) before
interpreting the candidates.

Inspect relevant systems and their registration together. Trace data flow
across plugins, schedules, run conditions, resources, events/messages, asset
loading, extraction, rendering, physics, and async work. Record each hypothesis
with a source location, expected runtime signature, and a probe that could
falsify it.

## Audit parallelism

Read [parallelism.md](references/parallelism.md). Review both source and the
assembled app:

1. Inventory `.chain()`, `.before()`, `.after()`, exclusive systems, non-send
   systems, broad `ResMut` access, shared locks, task pools, Rayon, and blocking
   waits.
2. Call `schedule_snapshot` with `includeSystems: true`. Start with
   `conflictLimit: 100`; filter by schedule when needed.
3. Separate explicit dependency edges from ECS data-access conflicts. Both can
   constrain concurrency, but only the former expresses required order.
4. Check whether mutable access can be narrowed, work partitioned by entity or
   phase, commands deferred, or computation moved to an appropriate task pool.
5. Preserve correctness ordering. Never recommend removing an edge without
   identifying the invariant it protects.

Report parallelism as a map of constraints and opportunities. Do not infer CPU
utilization or speedup from schedule shape alone.

## Measure the live scenario

Read [runtime-profiling.md](references/runtime-profiling.md), then use the
bevyout MCP workflow:

1. Call `viewer_status`; attach or use `viewer_launch` for prepared content.
2. Warm the exact scenario. Use `console_exec`, `scene_snapshot`, and logs to
   establish state and drive reproducible actions.
3. Call `performance_probe` with an explicit `warmupMs`, `durationMs`,
   `budgetMs`, and `latestLimit`. Use `includeSamples: true` when frame-level
   correlation matters.
4. Capture before/after probes around transitions or configuration changes.
   Compare sample count, average, p50, p95, p99, max, and over-budget count.
5. Correlate hitches with stable lifecycle logs and current diagnostics. Use
   `schedule_snapshot` to test parallelism hypotheses.
6. Escalate to a Chrome/Tracy trace when aggregate frame data cannot attribute
   cost. Use the repository's Bevy version and local docs for feature flags.

Prefer a cool machine, warm caches, identical camera/content, and repeated
runs. Label results incomparable when those controls differ.

## Add missing instrumentation

Add probes only when the current evidence cannot answer the question.

- Write the feature and pure/unit tests first.
- Keep statistics and eligibility logic std/serde-only where practical.
- Expose bounded, read-only snapshots through BRP; orchestrate waits in MCP,
  not inside a Bevy system or remote handler.
- Include monotonic markers so a client can measure an exact window.
- Bound histories, rows, conflicts, and output sizes.
- Avoid per-frame file I/O, synchronous waits, scene traversal added solely for
  a probe, or instrumentation that materially changes the workload.
- Keep development tracing and dynamic linking out of release defaults.

## Deliver findings

Lead with the highest-impact verified result. For every finding include:

- severity and affected scenario;
- evidence label: **measured**, **runtime-corroborated**, **static risk**, or
  **hypothesis**;
- source file and line;
- runtime evidence and measurement controls;
- mechanism, not just symptom;
- smallest corrective experiment and expected signature;
- validation command or probe.

Include a separate parallelism section covering exclusive systems, non-send
systems, conflict hot spots, unnecessary serialization, task-pool use, and
correctness-critical ordering. End with limitations and unmeasured areas.

Do not prescribe broad rewrites, task spawning, query splitting, batching, or
render changes unless evidence identifies the constrained resource and the
proposal has a falsifiable acceptance measure.
