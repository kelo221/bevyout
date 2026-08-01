# M6 wave 3 plan

## Fixed policy scope

1. Add a pure actor-residency policy with one canonical actor identity and
   deterministic bind, retain, handoff, unload, restore, and duplicate-owner
   decisions.
2. Keep prepared actor data, mutable actor state, canonical item ownership,
   and live ECS projection as distinct authorities; the policy may describe
   transitions but must not create a second runtime store.
3. Make resident NAVM topology decisions depend on both sides' validated
   residency and navigation readiness.
4. Remove or invalidate cross-cell links and archipelago membership when either
   side evicts, and make reload/rebuild deterministic.
5. Add focused unit tests first. The integrator adds shared executable feature
   steps after both lanes land.

## Lane ownership

| Lane | Owns | Must not touch |
| --- | --- | --- |
| W3-A / #276 | Pure actor-residency/handoff policy, actor catalog/state contract adapters only, dedicated policy tests | src/viewer/nav/landmass_graph.rs, W3-B tests, tests/features.rs, W3-C runtime integration |
| W3-B / #277 | src/viewer/nav/landmass_graph.rs, focused NAVM topology tests, prepare-side portal fixtures only if required | Actor state/AI ownership, exterior runtime integration, tests/features.rs |
| Integrator | Wave docs, shared feature/manual seam, issue evidence, merge resolution | Inventing a second actor or navigation authority |

## Tests-first acceptance

### W3-A

- bind an unowned actor to one resident cell;
- retain the same owner while the actor remains resident;
- hand off exactly once to a valid destination cell;
- reject a competing owner or stale source generation;
- unload and restore preserve the canonical actor identity/state without
  duplicating it.

### W3-B

- link two valid resident navigation sides;
- refuse links when either side is missing, loading, failed, or evicting;
- remove links and stale archipelago membership when either side evicts;
- rebuild the same topology deterministically after both sides return.

## Exit evidence

- focused lane tests pass;
- cargo fmt --check, cargo clippy --all-targets -- -D warnings, and the
  repository test suite pass after integration;
- the policy outputs are executable through the integrator's feature seam;
- W3-C / #278 remains explicitly blocked by #10 until the M4 gate closes.

## Execution model recommendation

Codex runtime: GPT-5.6 Luna, Max reasoning.

## Shipped amendments

- W3-A and W3-B policy lanes landed as commits `a9a68b95` and `f61e0116`.
  The integrator feature seam and manual cover the pure policy outputs;
  W3-C / #278 runtime integration remains blocked by gate #10.
