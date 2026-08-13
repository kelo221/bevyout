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
  The integrator feature seam and manual cover the pure policy outputs.
- Gate #10 closed 2026-08-04, unblocking W3-C. Before dispatch, planning
  found the exterior prepare pipeline discarded all actor placements
  (`orchestrator.rs`'s exterior branch hardcoded `placements: Vec::new()`,
  `apply_staged_assets` dropped the resolved `PreparedActor` assembly) —
  independent of #10, so W3-C could not be a runtime-only change. Split
  into a sequential Stage 1 (prepare, #299) and Stage 2 (runtime, #278)
  on one branch/worktree (`m6-wave3-w3c`), per the roadmap's own
  Sonnet-for-policy/Opus-for-runtime-integration guidance.
- #299 (Sonnet): `367873bd`, `5bf11abb`, `c1c3f29a` — `ExteriorCellPackage`
  gained an `actors` field, ACHR/ACRE assemblies are preserved instead of
  discarded, exterior cells get actor/animation catalogs.
  `EXTERIOR_CELL_PACKAGE_REVISION`/`CURRENT_PREPARE_REVISION` bumped.
- #278 (Opus): `a802595e`..`a914248f` (7 commits) — new
  `src/viewer/world/exterior/actors.rs` observes residency, binds/hands
  off/unloads/restores one canonical actor via W3-A's policy, fixed two
  live defects (cross-cell package-catalog lookup, unconditional nav
  retargeting), added the `actorresidency` console command and
  diagnostics counters.
- Independent live re-verification (separate agent, fresh `curl` BRP
  session) confirmed bind/handoff/eviction/restore and, critically, exactly
  one live ECS entity for the tracked reference at every checkpoint via
  `scene_snapshot` — the core no-duplicate-projection guarantee holds.
  It also found `nav_bound` did not survive handoff/restore.
- #303 (Sonnet, same branch): `32cb923c` fixed the nav-bind gap. Root
  cause was not the hypothesized missing bind call on handoff (nav
  components are untouched by handoff) but the exterior fall-guard
  releasing the bind ~1.3s after a `setpos`-forced landing on
  not-yet-settled destination collision, and restore's ordinary spawn
  chain correctly leaving an actor unbound when its own AI package
  fails to start (a separate, already-tracked gap, not a defect). Added
  `reconcile_actor_nav_bindings`, a cheap idempotent per-frame retry for
  any live tracked-but-unbound actor; survives evict/restore because
  `bound_references` is deliberately never cleared on unload. Verified
  live: `nav_bound: true` holds after both a forced handoff and a full
  evict/restore cycle.
- Follow-ups filed under #13, not fixed in this wave: #301 (exterior
  actors lose their XLKR linked-reference chain, so packages using
  near-linked-reference points fail `unresolved_point`), #302
  (exterior-only `prepare` never writes the shared
  `catalogs/<fingerprint>/packages.ron`, so package start fails
  `catalog_unreadable` without an incidental interior cell in the same
  run), #304 (fall-guard rebinds continuously at one specific test
  coordinate — noisy, not incorrect, low priority).
- Repository-wide `cargo fmt --check`/`cargo clippy --all-targets -- -D
  warnings`/`cargo test` (699 Cucumber scenarios, 3470 steps) verified
  clean independently by the orchestrator on the final branch state, not
  only self-reported by executors.
