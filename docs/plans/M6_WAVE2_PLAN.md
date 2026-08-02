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

### Implementation integrated — 2026-08-01

The W2-A/W2-B implementation and executable coverage are integrated on
`M6-OutCell`:

- W2-A commit: `95e42a92` (`Measure exterior process memory diagnostics`).
- W2-B commit: `370b1456` (`Harden exterior lifecycle reversal`).
- Integrator/docs commit: `edf504a0` (`M6 wave 2: add roadmap and executable exterior coverage`).

The repository gates for the integrated lanes passed as recorded in the parent
epic. Real current-data route evidence remains a separate acceptance step.

### Current v21 route evidence — 2026-08-02

The current Fallout v21 data/config preflight is green:

- The exact 14-cell selector list resolves with `--list-only`.
- Native `--check-fingerprints` reports `14 cells valid, 0 stale`.
- Prepared revision is `prepare-v21-m6-worldspace-lod-imad-screen-fx`.
- The Windows process-resident-set sampler reports method
  `sysinfo_process_resident_set`, platform `windows`, and real current/peak/
  ending samples, separate from package estimates.

A bounded local BRP run crossed the deterministic route in both directions
with ten-plus `tp` crossings and a final re-anchor to `(4,-5)`. The route
window observed `requests=22`, `evictions=16`, `resident_cells=7` at the final
anchor, `stale_completions=0`, `failed=0`, and `cancellations=0`; the final
cell list contained seven unique collision-ready entries. The sampled process
memory peak was `1,523,015,680` bytes and ending sample
`1,522,896,896` bytes, while the peak package estimate was `3,696,696` bytes
against the separate `134,217,728`-byte estimate budget. The route-window
frame sample had p50 `6.3027 ms`, p95 `7.7809 ms`, max `93.0192 ms`, and four
over-budget samples; the cooldown-only window was separately recorded and is
not substituted for the route result.

The follow-up deterministic probe completed five out-and-back loops. Every
loop returned to `(4,-5)` with no step failure; the clean-loop endpoint was
`requests=71`, `evictions=65`, `resident_cells=7`, `failed=0`,
`cancellations=0`, and `stale_completions=0`. That trace measured a process
memory peak of `1,598,648,320` bytes and ending sample `1,319,567,360` bytes;
the cooldown frame window was p50 `7.1569 ms`, p95 `7.9774 ms`, max
`9.0933 ms`, with zero samples over the existing 16.6667 ms report budget.

The reversible missing-package probe then removed the already-indexed
`00000c4c.ron` package, crossed toward grid `(1,-5)`, and observed the target
entry as `lifecycle=Failed`, `collision_ready=false`, `failed=1`, while the
current `(4,-5)` cell remained resident and `stale_completions=0`. The package
was restored and the viewer stopped; the expected failure is not part of the
clean-loop counts.

The frozen W7 clean-preparation command was also attempted with the exact 14
selectors, explicit current config, native conversion, and `--jobs 1` in a
new disposable cache. After the ten-minute command timeout the resumable job
manifest still listed every job as `Pending`, although the cache had reached
5,171 files and `1,013,366,580` bytes. The run was stopped and the partial
ignored cache was preserved only for diagnosis; it has no clean elapsed-time,
completion, warm-run, or threshold value and must not be reused as acceptance
evidence.

A separate diagnostic rerun with the same selectors and native converter at
`--jobs 4` also timed out after five minutes without a completion line. Its
resumable manifest had only `1/14` jobs marked `Done` at
`1,309,667,625` bytes across `5,419` files. That alternate-throughput attempt
is likewise not a clean or warm result and supplies no threshold.

This remains partial acceptance evidence because all crossings used
deterministic `tp` probes rather than ordinary keyboard traversal, and actor,
travel/save, and final budget sign-off were not run. Issues #274, #275, and
#285 remain open; no W2, #87, or final M6 completion claim is made from these
measurements.
