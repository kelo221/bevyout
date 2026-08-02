# M6 wave 3 prompt

Wave 3 begins with policy work that is explicitly allowed before gate #10
closes. This kickoff does not authorize runtime actor/exterior integration:
W3-C remains blocked until the M4 actor gate is closed.

Run two disjoint Luna Max lanes:

- W3-A (#276): define the pure actor-residency and canonical handoff policy.
  Preserve the existing ActorRuntime, ActorStateRuntime, prepared actor
  identity, and canonical item/state authority. Cover bind, retain, handoff,
  unload, restore, and duplicate-authority rejection.
- W3-B (#277): make resident NAVM topology lifecycle-safe in
  src/viewer/nav/landmass_graph.rs and its focused tests. A cross-cell link
  exists only while both resident sides are valid; eviction removes stale
  archipelago/link state deterministically.

Tests come first in each lane. The integrator owns the shared executable
feature seam and any later manual script. Do not modify the shared
tests/features.rs from an executor worktree.

Execution model: GPT-5.6 Luna, Max reasoning, per the user kickoff request.
