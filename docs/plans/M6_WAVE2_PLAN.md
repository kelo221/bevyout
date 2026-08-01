# M6 wave 2 plan

## Fixed feature list

1. Add a platform-aware process-memory sampler for the viewer's exterior
   diagnostics. The report must identify its method/platform, expose current,
   peak, and ending samples when supported, and return explicit unsupported or
   unmeasured metadata when it is not.
2. Extend `worldstream summary` and the trace workflow without changing the
   meaning of `resident_package_bytes_estimate` or
   `peak_package_bytes_estimate`. Runtime process memory and package byte
   estimates remain separate fields.
3. Make bounded lifecycle behavior deterministic under ordinary two-way
   movement, rapid reversal, cancellation while loading, completion after
   eviction, repeated load/evict loops, and collision-pending transitions.
4. Preserve one root and one collision ownership record per grid, reject stale
   generations, and remove all cell-owned state after final eviction.
5. Add focused pure and minimal-App regression coverage for both lanes, then
   add executable feature coverage through the integrator's append-only seam in
   `tests/features.rs`.

## Tests-first order

### W2-A — process memory

- Define deterministic report fixtures for supported, unsupported, and not-yet-
  sampled states.
- Test that process-memory fields are not populated from package estimates.
- Test that repeated samples update current/peak/ending deterministically and
  preserve the method/platform label.
- Add a feature scenario for the separation of process memory and package
  estimates; the integrator adds its shared step definitions.

### W2-B — lifecycle and reversal

- Add pure planner cases for reversal while a cell is queued, loading, spawned
  but collision-pending, ready/resident, and evicting.
- Add minimal-App/runtime cases for stale completion rejection, root uniqueness,
  collision-ledger teardown, and zero residual cell-owned entities.
- Add a feature scenario for generation-safe cancellation/reversal; the
  integrator adds its shared step definitions.

## File authority

| Lane | May edit | Must not edit |
| --- | --- | --- |
| W2-A | `src/viewer/world/exterior/diagnostics.rs`, `src/viewer/console/world_commands.rs`, focused diagnostic tests, and the smallest required dependency declaration | Residency transitions, `crates/bevyout-core/src/manifest/exterior.rs`, lifecycle/loading policy, actor/environment/LOD code, `tests/features.rs` |
| W2-B | `crates/bevyout-core/src/manifest/exterior.rs`, `src/viewer/world/exterior/{policy.rs,lifecycle.rs,loading.rs,mod.rs}`, focused lifecycle tests | Process-memory implementation, `src/viewer/console/world_commands.rs`, actor/environment/LOD code, `tests/features.rs` |
| Integrator | `tests/features.rs`, shared plugin/console registration, `M6_WAVE2_MANUAL.md`, merge seam, issue/PR evidence | Changing either lane's authority without a recorded amendment |

If a lane needs a shared re-export or registration hunk, leave it for the
integrator or document the exact conflict and keep the semantic owner intact.

## Exit evidence

- Focused lane tests pass.
- `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, and
  `cargo test` pass after integration.
- A short real-data route crosses both directions, performs rapid reversal,
  repeats at least ten crossings, and records `stale_completions=0`, bounded
  roots/collision ownership, and no residual cell-owned state after eviction.
- `worldstream summary` records process-memory current/peak/ending samples on
  supported platforms, with method/platform metadata; unsupported platforms
  remain explicitly unsupported/unmeasured.
- The manual records machine, build profile, route, sample window, and whether
  the run was clean or warm. It does not freeze final numeric budgets; that is
  Wave 6/7 work.

## Execution model recommendation

Codex runtime: **GPT-5.6 Luna, Max reasoning**, per the kickoff request. The
orchestrator remains responsible for integration, gates, and judging real-data
evidence.

## Shipped amendments

None yet.
