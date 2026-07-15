# Parallelism audit reference

## Constraint model

Bevy schedules systems opportunistically in parallel when dependency order,
run conditions, thread affinity, and ECS access allow it.

- An explicit `.before()`, `.after()`, set edge, or `.chain()` adds order.
- Conflicting mutable/read access prevents overlap even without an order edge.
- An exclusive `&mut World` system conflicts with the world broadly.
- A non-send system is pinned to the main thread.
- Deferred commands can introduce application barriers and visibility rules.
- A mutex or channel can serialize work even when ECS access appears disjoint.
- Compute, async-compute, I/O pools, Rayon, renderer threads, and external
  workers are distinct execution domains; avoid oversubscription assumptions.

`schedule_snapshot` reports the assembled main-world schedules that have been
initialized. Its conflict pairs are access constraints, not measured stalls.
An uninitialized schedule has no runtime metadata yet. The current remote
schedule may be temporarily absent while it executes.

## Audit procedure

1. Group systems by schedule and feature/plugin.
2. Mark exclusive and non-send systems; inspect their work and frequency.
3. Review long chains and fan-in/fan-out edges. Write down the invariant for
   every edge before considering removal.
4. Rank conflict pairs that involve project systems, hot resources, broad
   queries, or world access. Ignore raw pair count as a performance metric.
5. Check whether shared mutable resources can become messages, split resources,
   local state, disjoint components, or phase-specific buffers.
6. Check whether expensive independent work has enough granularity. Tiny tasks
   lose to scheduling overhead; huge tasks leave cores idle and delay results.
7. Check when async results are consumed. Main-thread spawning, asset creation,
   entity insertion, and GPU upload may remain the actual critical section.
8. Validate with traces or controlled frame probes under a representative load.

## Recommendation standard

For each opportunity, name the current constraint, the correctness invariant,
the proposed access/order change, expected parallel overlap, overhead risk, and
measurement that decides whether to keep it. Do not estimate speedup from core
count alone.
