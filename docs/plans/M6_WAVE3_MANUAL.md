# M6 wave 3 — actor residency and NAVM policy manual

This manual exercises the policy contracts shipped by W3-A and W3-B. It does
not claim the W3-C gameplay runtime gate: actor binding and exterior runtime
integration remain blocked by issue #10 until the M4 actor gate closes.

## 1. Run the executable policy scenarios

From the repository root, run:

```text
cargo test --test features -- --input features/m6_wave3_policy.feature --concurrency 1
```

Expected: all five M6 wave 3 scenarios pass. The scenarios show:

- one actor reference and one canonical `HolderId::Actor` through bind and
  handoff;
- stale-generation and duplicate-projection rejection;
- unload/restore decisions that keep the canonical actor state location;
- cross-cell NAVM links appearing only when both sides are valid and
  navigation-ready; and
- eviction removing old links before a reload accepts a new-generation portal.

## 2. Run the focused policy tests

```text
cargo test --lib actor_residency
cargo test --lib resident_
```

Expected: the actor policy tests and resident-topology tests pass. These are
pure decisions over caller-owned observations; they do not create an ECS
actor store or mutate the save/item authorities.

## 3. Runtime acceptance after issue #10 closes

Do not attempt this step on the policy-only wave. After #10 is closed and W3-C
lands, prepare the fixed M6 exterior route from `M6_WAVE2_MANUAL.md`, launch
the bridge on the prepared route, and use the W3-C manual's exact actor and
navigation commands. The required result is one gameplay actor binding to a
resident NAVM, crossing a cell border, surviving source eviction/reload, and
retaining one canonical saved state without duplicate ECS projections.

Record the exact prepared cell, actor/reference FormID, generation transitions,
console transcript, build/cache state, and any stale-link or duplicate-owner
diagnostics. A failed real-data check becomes a focused child issue under #13;
it does not silently expand this policy wave.
