# Static audit reference

## Evidence ladder

- **Measured:** repeated probe or trace demonstrates the cost in the target
  scenario.
- **Runtime-corroborated:** live state or schedule data supports the mechanism,
  but cost is not directly attributed.
- **Static risk:** source structure could cause cost; runtime impact is unknown.
- **Hypothesis:** plausible explanation with an explicit falsification probe.

Never promote a lower level without new evidence.

## Review order

1. Map app/plugin composition, states, schedules, sets, and run conditions.
2. Find work that runs every frame versus startup, state entry, fixed update,
   render extraction, preparation, queueing, and cleanup.
3. Inspect high-cardinality queries and repeated full-world scans. Check change
   filters, events/messages, caching, spatial indices, and lifecycle gates.
4. Inspect entity and asset churn: spawn/despawn loops, handle lifetime,
   repeated loads, material/mesh cloning, pipeline specialization, and GPU
   uploads.
5. Inspect allocation and cloning in hot systems. Distinguish cheap handle or
   small-copy clones from deep collections and asset duplication.
6. Inspect synchronous filesystem/network work, mutex/RwLock contention,
   channel waits, thread joins, `block_on`, and CPU-heavy parsing/cooking.
7. Inspect rendering: light/shadow counts, visibility churn, extraction,
   prepared assets, bind-group/pipeline churn, overdraw, texture traffic, and
   CPU-versus-GPU attribution.
8. Inspect physics: cooking, broad/narrow phase load, ray/shape-cast counts,
   sleeping behavior, dynamic transform synchronization, and debug drawing.

## Interpretation traps

- `ResMut<T>` is not automatically expensive; it declares exclusive access to
  `T` and can serialize otherwise independent systems.
- `.chain()` is not automatically wrong; it is costly only when the full order
  is unnecessary or puts expensive independent work on one critical path.
- `Query::iter()` is not automatically a bottleneck; cardinality, frequency,
  archetype layout, filters, and work per entity determine cost.
- `clone()` may be a refcount bump or a deep allocation. Resolve the type.
- Async task creation is not proof of parallel progress. Look for immediate
  polling, blocking, main-thread application cost, and oversubscription.
- Frame time alone does not distinguish main-world CPU, render-world CPU, GPU,
  I/O, compilation, or thermal throttling.

## Architecture checks

Keep hot decision logic pure and narrow; let thin Bevy systems perform ECS
access. Prefer feature-local plugins and explicit hand-off contracts. Flag
global resources or all-purpose systems that make unrelated slices contend,
but recommend boundaries only when they preserve ownership and reduce a
measured or demonstrated constraint.
