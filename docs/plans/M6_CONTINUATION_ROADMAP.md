# M6 continuation roadmap after PR #261

This roadmap continues epic [#13](https://github.com/kelo221/bevyout/issues/13)
after merged PR [#261](https://github.com/kelo221/bevyout/pull/261). It does not
rewrite the original proposal in [WorldPlan.md](WorldPlan.md): PR #261 shipped
parts of its original waves 1 through 8 in one foundation PR, so this document
rebaselines the remaining work into executable waves 2 through 9.

The W2 implementation and the W3-A/B, W4-A/B, W5-A/B, and W6-A/B preparation
lanes are now integrated. The active continuation work is W2 real-data
acceptance plus W6-C numeric-threshold sign-off; W3-C, W4-C, and W5-C remain
sequenced behind their documented runtime dependencies. Waves 7, 8, and 9
retain their existing meanings and manuals: bounded gate #87, LOD/presentation,
and final route gate #14 respectively.

### Current checkpoint — 2026-08-02

The current v21 native preflight validates all 14 frozen selectors with
`0 stale` fingerprints. A supported synthetic input path is now live-validated:
runtime-write BRP `world.write_message` can send a reflected
`bevy_input::keyboard::KeyboardInput` `Pressed` message, sustain the existing
fixed-update controller, and clear it with the matching `Released` message plus
`KeyboardFocusLost`. This is a bridge-input diagnostic lane, not physical
OS-keyboard acceptance; ordinary traversal, reversal, and loop measurements
remain `not_yet_sampled`. A follow-up explicit `worldstream trace 1/0` BRP
diagnostic completed five deterministic `tp` out-and-back loops: every loop
returned to `(4,-5)`, the final counters were `requests=149`, `evictions=143`,
`resident_cells=7`, `peak_resident_cells=11`, and
`failed=0/cancellations=0/stale_completions=0`; the closed trace recorded a
process-memory peak of `1,860,186,112` bytes and ending sample
`1,857,990,656` bytes. This is still synthetic deterministic-input evidence,
not ordinary OS-keyboard acceptance. A separate held-input boundary diagnostic
crossed `(4,-5)` to `(3,-5)` with `KeyA` and returned with `KeyD`, using the
real player physics and ending with `failed=0/cancellations=0/stale_completions=0`.
This is synthetic held-input evidence only. W6-C provenance is recorded in
`M6_WAVE6_PLAN.md`
(commit `9181c691`), including the source of the configured 25-cell, 128 MiB
estimated-package, 64-light, and 16.6667 ms reporting values; the historical M2
`<=33 ms` swap bar is not an M6 threshold. Strict clippy, `cargo test`,
`cargo check-dev`, and `prepare --help` pass; repository-wide `cargo fmt --check`
still reports only the known unrelated baseline drift. No W2, #285, #87, or
final M6 closure claim is made. A current-v21 W8 presentation probe also
verified default-off worldspace LOD, the opt-in `48`-tile bound (`40` terrain,
`8` blocks), `presentation_only=true`, full-land collision, and clean toggle
back to zero active far tiles; the W8 manual records the transcript.

## Execution model recommendation

Use **Sol X-High** in the Codex runtime. M6 still crosses asynchronous Bevy
lifecycle state, BoxDDD collision ownership, actor/navigation authority,
persistence, rendering, and real-data performance judgment. If execution runs
in the Claude runtime, use an **Opus orchestrator** with Opus executors for
runtime integration and Sonnet executors only for isolated policy, parser,
diagnostic, fixture, or documentation issues.

Each row below is a proposed child issue under #13 and therefore an independent
executor brief. Parallel execution is allowed only where both the file set and
the semantic authority are disjoint. The wave integrator owns shared plugin
wiring, `tests/features.rs`, manuals, conflict resolution, and final gates.

## Post-#261 baseline

PR #261 established the contracts and runtime surfaces; it did not prove the
milestone gates.

| Area | Shipped foundation | Still required |
| --- | --- | --- |
| Preparation | Native-only WRLD/CELL, LAND, objects, water, collision, NAVM, environment, and presentation packages | Route-wide clean/warm rebuild evidence and final cache/rebuild budgets |
| Streaming | Generation-aware bounded residency, stale-completion rejection, ordered eviction, collision teardown | Ordinary bidirectional traversal, rapid reversals, repeated-loop invariants, and real process-memory measurements |
| Navigation/actors | Resident NAVM merge-link surfaces and focused test-agent handoff | Gameplay actor binding, cross-resident-cell pathing, unload/reload continuity, and #10 dependency closure |
| Travel/persistence | Exterior location save shape, exterior deltas, and travel surfaces | Exact exterior/interior return anchors, save/reload at route points, and one canonical ownership path |
| Player physics | Water contact, swimming, breath timer, and landing state surfaces | Entry/exit acceptance, breath consequences/recovery, and fall behavior on real terrain |
| Environment | Prepared weather/time/ImageSpace and dynamic day/night/light surfaces | Transition precedence, local-light budget evidence, and interior ImageSpace/irradiance isolation |
| Presentation | Terrain LOD policy, optional worldspace LOD, distance culling, presentation diagnostics | Hysteresis/neighbour acceptance, VWD duplicate suppression, conservative occlusion, and visible pop-in gate |
| Diagnostics | Deterministic package estimates and lifecycle counters | Stable process working-set/RSS samples and frozen numeric budgets; estimates must remain separately labelled |

## Dependency sequence

```diagram
┌──────────────────────────────────────────────┐
│ W2 Streaming reliability + real memory      │
└──────────────────────┬───────────────────────┘
                       │       ┌──────────────────────┐
                       ├──────▶│ #10 actor gate closes│
                       │       └──────────┬───────────┘
                       ▼                  ▼
┌──────────────────────────────────────────────┐
│ W3 Resident gameplay actors + navigation     │
└──────────────────────┬───────────────────────┘
                       ▼
┌──────────────────────────────────────────────┐
│ W4 Travel + persistence + player physics     │
└──────────────────────┬───────────────────────┘
                       ▼
┌──────────────────────────────────────────────┐
│ W5 Environment + lighting isolation          │
└──────────────────────┬───────────────────────┘
                       ▼
┌──────────────────────────────────────────────┐
│ W6 Bounded-route convergence + frozen budgets│
└──────────────────────┬───────────────────────┘
                       ▼
┌──────────────────────────────────────────────┐
│ W7 Gate #87                                  │
└──────────────────────┬───────────────────────┘
                       ▼
┌──────────────────────────────────────────────┐
│ W8 LOD + VWD + occlusion + pop-in            │
└──────────────────────┬───────────────────────┘
                       ▼
              ┌─────────────────┐
              │ Close epic #13  │
              └────────┬────────┘
                       ▼
┌──────────────────────────────────────────────┐
│ W9 Final route gate #14                      │
└──────────────────────────────────────────────┘
```

Pure policy or preparation work may start before its preceding wave finishes.
Runtime integration and acceptance may not skip the dependency sequence.

## Wave 2 — Streaming reliability and measured memory

**Goal:** prove that the existing lifecycle remains bounded under ordinary
movement, cancellation, reversal, and repeated load/evict loops. Use a short,
fixed exterior cell set rather than the complete Megaton route.

### Parallel child issues

| Executor issue | Authority and owned files | Must not touch | Exit evidence |
| --- | --- | --- | --- |
| **W2-A: Measure exterior process memory** | Measurement policy and platform adapter; `src/viewer/world/exterior/diagnostics.rs`, `src/viewer/console/world_commands.rs`, focused tests and any narrowly required dependency declaration | Residency transitions, collision teardown, package-byte estimator semantics | `worldstream summary/trace` reports sampled current, peak, and ending process memory with method/platform metadata; package estimates remain separate; unsupported platforms say unsupported rather than fabricate values |
| **W2-B: Stress exterior lifecycle and reversal** | Residency/lifecycle authority; `crates/bevyout-core/src/manifest/exterior.rs`, `src/viewer/world/exterior/{policy.rs,lifecycle.rs,loading.rs,mod.rs}` and their dedicated tests | Process-memory implementation, environment, actors, LOD | Ordinary two-way boundary crossing and rapid reversal cannot duplicate roots, resurrect stale generations, tear collision down early, or leak lifecycle/accounting state |

W2-A and W2-B may run in parallel worktrees. The integrator alone edits shared
console registration/tests, feature steps, and the wave manual after both land.

### Tests first

1. Pure planner scenarios for reversal while requested, loading, spawned but
   collision-pending, resident, and evicting.
2. Minimal-App tests for stale completion, root uniqueness, collision-ledger
   teardown, and zero residual cell-owned entities after eviction.
3. A bounded live loop using normal keyboard movement in both directions,
   followed by rapid reversal and at least ten repeated crossings.
4. Memory samples before the loop, at peak, after eviction, and after a fixed
   cool-down. Record the plateau rule before judging the result.

### Exit criterion

The short route completes in both directions; `stale_completions=0`; every
grid has at most one root; evicted cells retain no collider ownership; and real
process memory reaches an agreed post-loop plateau. This wave validates the
measurement path but does not freeze the final route budget.

## Wave 3 — Resident gameplay actors and navigation

**Blocker:** runtime integration starts only after gate
[#10](https://github.com/kelo221/bevyout/issues/10) closes. Policy and fixture
work may proceed earlier, but it must not guess a replacement actor authority.

**Goal:** a real gameplay actor—not only `tna`'s disposable test agent—binds to
resident exterior navigation, crosses a cell border, unloads, and restores
without duplicate ECS or persistence authority.

### Child issues and merge order

| Order | Executor issue | Authority and owned files | Exit evidence |
| --- | --- | --- | --- |
| Parallel 1 | **W3-A: Define actor residency and canonical handoff policy** | Pure resident-cell/actor ownership policy plus dedicated tests; actor catalog/state contracts only | Deterministic bind, retain, handoff, unload, and restore decisions; exactly one canonical actor record owns state |
| Parallel 1 | **W3-B: Make resident NAVM topology lifecycle-safe** | `src/viewer/nav/landmass_graph.rs`, focused nav tests, and prepare-side portal fixtures if required | Links appear only when both resident sides are valid and disappear without stale links when either side evicts |
| Sequential 2 | **W3-C / #278: Integrate gameplay actors with exterior residency** | `src/viewer/ai/autonomous.rs`, `src/viewer/nav/actor_binding.rs`, the required portions of `src/viewer/nav/agent.rs`, exterior activation/eviction adapter, and dedicated tests | One representative actor crosses a resident boundary, survives source eviction/reload, keeps package/animation state, and never exists twice |

W3-C consumes W3-A and W3-B. It is the only executor permitted to modify the
runtime actor/exterior lifecycle seam. Any required `world::swap` change is
merged sequentially by the wave integrator.

### Exit criterion

The fixed bounded route has one human or creature actor that autonomously
binds, paths across resident cells, crosses a border, unloads/restores, and
retains its canonical state. Navigation diagnostics show no stale archipelago
or link after eviction.

## Wave 4 — Exact travel, persistence, and player physics

**Goal:** establish one canonical exterior/interior location and return-anchor
contract, then prove save/reload, dynamic state, water, breath, and fall
behavior against it.

### Child issues and merge order

| Order | Executor issue | Authority and owned files | Exit evidence |
| --- | --- | --- | --- |
| Sequential 1 | **W4-A: Canonicalize travel anchors and save location** | `crates/bevyout-core` location/door contracts, `src/save/`, `src/viewer/world/mod.rs`, and focused migration/round-trip tests | Exterior→interior→exterior returns to the exact authored destination transform; save format has one location authority and deterministic migration |
| Parallel with W4-A policy work | **W4-B: Complete water, breath, and fall policy** | Pure movement/contact/landing policy and dedicated tests under `src/viewer/openmw_player/` and player movement tests | Water entry/exit, swim/ground transition, breath depletion/recovery consequence, and fall thresholds are deterministic and frame-rate independent |
| Sequential 2 | **W4-C: Integrate travel, streamed persistence, and player physics** | `src/viewer/world/{swap.rs,persist.rs}`, `src/viewer/interaction/activation.rs`, the exterior lifecycle adapter, `src/viewer/player/movement.rs`, and dedicated tests | Exact round trip and save/reload work from exterior, interior, and water-adjacent points; dynamic references survive eviction and save reload |

W4-A owns the contract; W4-C owns runtime integration. No parallel executor may
add a second return-anchor resource or independently serialize travel state.

### Exit criterion

One exterior door round trip restores exact position and rotation; saves from
both sides and from the exterior route reload correctly; moved/disabled dynamic
references survive; and water/breath/fall acceptance passes on real terrain.

## Wave 5 — Weather, ImageSpace, and lighting isolation

**Goal:** prove one deterministic environment authority across streamed cells
and interior transitions, with bounded cell-owned local lights.

### Child issues and merge order

| Order | Executor issue | Authority and owned files | Exit evidence |
| --- | --- | --- | --- |
| Parallel 1 | **W5-A: Complete environment resolution and blending** | Prepare-side climate/weather/ImageSpace resolver, `src/viewer/day_night.rs`, and dedicated pure/minimal-App tests | Worldspace→climate→weather→time precedence and fallback are deterministic; blends are bounded and reproducible |
| Parallel 1 | **W5-B: Specify local-light ownership and budget policy** | Pure ranking/ownership policy, core exterior tests, and focused scene-light fixtures | Stable distance/FormID ranking, fixed budget, cell-owned teardown, and no gameplay effect from presentation culling |
| Sequential 2 | **W5-C: Integrate exterior environment with travel and streaming** | `src/viewer/scene.rs`, exterior presentation/light systems, post-processing integration, and dedicated viewer tests | Exterior sun/sky/fog/water/ImageSpace restore after travel; interiors reapply authored ImageSpace and retain baked irradiance; exterior lights do not leak |

The environment resolver may run while wave 4 finishes. W5-C merges only after
W4's transition contract is stable and is the single owner of scene/travel
environment integration.

### Exit criterion

A timed weather transition is visible and deterministic outside; entering an
interior restores its authored ImageSpace and lighting isolation; leaving
restores exterior state; streamed local lights stay within budget and disappear
with their owning cell.

## Wave 6 — Bounded-route convergence and frozen budgets

**Goal:** make gate #87 executable. This is not a feature bucket and does not
claim the gate.

### Parallel preparation tasks

| Executor issue | Owned artifacts | Exit evidence |
| --- | --- | --- |
| **W6-A: Consolidate M6 diagnostics and measurement export** | Existing diagnostics/report modules and focused tests; no lifecycle behavior changes | One deterministic report joins traversal, lifecycle, actor/nav, travel/save, environment, presentation, cache, timing, and process-memory evidence with no mandatory `null` metric |
| **W6-B: Freeze bounded-route fixtures and acceptance protocol** | `docs/plans/M6_WAVE7_MANUAL.md`, synthetic test fixtures, command transcript shape | Exact route/cells, actor, door, water point, save points, weather IDs, build mode, hardware metadata, clean/warm matrix, loop count, and expected outputs are fixed |
| **W6-C: Freeze numeric route acceptance thresholds** (#285) | Orchestrator-owned threshold matrix, provenance, and dependency-held fields; no lifecycle implementation | Numeric limits are recorded for the W7 fields, with configured/reporting defaults kept distinct from measured acceptance and blocked gameplay fields named explicitly |

After W6-A and W6-B land, the orchestrator runs one preflight. W6-C records the
threshold decision before W7. Every nontrivial failure becomes a narrowly
scoped child bug under #13 assigned to one owner. Bugs that touch the exterior
lifecycle, travel/save seam, or scene integration merge sequentially; do not
hide implementation work inside the gate issue.

### Budget protocol

Freeze numeric limits before W7 for:

- steady frame time, 1% low, and worst transition frame;
- package request-to-ready and collision-ready transition time;
- resident root/collider/light limits and stale completion count;
- process-memory peak and post-loop plateau;
- clean and warm prepare time, cache size, and no-op rebuild time;
- actor path latency, stuck/blocked status, and return-anchor error.

Each value records machine, build profile, sample window, route, and whether the
run was clean or warm. W8 may require a later final-route rebaseline.

The current tree already exposes configured or reporting values for the
exterior resident window (25 cells), estimated package-byte bound
(134,217,728 bytes), active streamed-light default (64), and convergence frame
report budget (16.6667 ms). These are provenance for W6-C, not automatic
measured gate results. Process RSS/working-set, package estimates, and runtime
transition/path measurements remain separate fields.

### Exit criterion

The W7 manual covers every checkbox in #87, one preflight completes far enough
to validate the protocol, all discovered defects have assigned child issues,
and the acceptance thresholds are recorded before the gate run.

## Wave 7 — Pass bounded exterior gate #87

**Goal:** execute the frozen acceptance matrix in
[M6_WAVE7_MANUAL.md](M6_WAVE7_MANUAL.md). W7 is acceptance and focused
hardening, not open-ended implementation.

Required runs include clean and warm preparation; ordinary bidirectional
movement and reversal; actor crossing; exterior/interior/exterior return;
save/reload; water, fall, weather, ImageSpace, and light isolation; repeated
eviction loops; and all frozen budgets. Any nontrivial defect gets a child bug
under #13 and is rerun after its focused fix.

**Exit criterion:** every #87 acceptance checkbox has linked measured evidence,
repository gates pass, the manual is complete, and #87 closes.

## Wave 8 — LOD, VWD, occlusion, and controlled pop-in

**Goal:** finish required default-route presentation after the bounded gameplay
gate. Archive far-worldspace tiles remain an optional layer unless explicitly
promoted by a recorded amendment.

### Parallel child issues

| Executor issue | Authority and owned files | Exit evidence |
| --- | --- | --- |
| **W8-A: Finish seam-safe terrain LOD policy** | `crates/bevyout-core/src/manifest/exterior.rs`, dedicated core tests, and synthetic terrain fixtures | Hysteresis is stable; cardinal neighbours differ by at most one level; geometry and skirts cannot expose cracks |
| **W8-B: Prepare VWD/distant representations** | `src/vsa/exterior/{package.rs,index.rs}` and `src/vsa/prepare/worldspace_lod.rs` with dedicated tests | Deterministic near/far identity, cache fingerprinting, bounded assets, and no duplicate representation contract |
| **W8-C: Measure conservative culling and pop-in** | Presentation diagnostics and capture/report tests only | Frustum/occlusion state is measured rather than inferred; pop events and duplicate near/far identities are reportable |

### Sequential runtime integration

**W8-D: Integrate exterior presentation selection** is the sole owner of the
runtime presentation path in `src/viewer/world/exterior/mod.rs`. It consumes
W8-A through W8-C, preserves resident collision/navigation/gameplay regardless
of presentation selection, applies duplicate suppression, and keeps occlusion
conservative. It also completes [M6_WAVE8_MANUAL.md](M6_WAVE8_MANUAL.md).

### Required versus optional

- **Required by default:** per-cell terrain/object LOD, neighbour clamping,
  hysteresis, no cracks, no duplicate near/far objects, conservative culling,
  bounded visible transitions, and diagnostics.
- **Optional unless amended:** archive-based far-worldspace horizon tiles. The
  opt-in layer must still be bounded and must not alter gameplay ownership.

### Exit criterion

The default route passes visual crack, duplicate, occlusion, and pop-in checks;
presentation remains separate from collision/nav ownership; affected frame and
memory budgets are remeasured; #13's checklist and measured evidence are
complete; and epic #13 closes. Gate #14 remains downstream.

## Wave 9 — Pass final route gate #14

**Prerequisites:** #10, #87, and #13 are closed. Assign #14 to the human user
before kickoff to satisfy the wave assignment invariant.

Execute [M6_WAVE9_MANUAL.md](M6_WAVE9_MANUAL.md) using ordinary input from
Super-Duper Mart to Megaton and back. Include rapid reversal, at least one
gameplay actor route segment, an interior visit with exact return, saves at
multiple route points, repeated complete loops, and final post-W8 frame,
memory, transition, nav, cache, and rebuild budgets.

**Exit criterion:** all #14 acceptance criteria and repository gates pass with
linked diagnostics and real-data evidence; #14 closes.

## Subagent and integration rules

1. Create every executor task as a child issue under #13, assign it to the
   authenticated human, add milestone 6 and the appropriate area/priority
   labels, and link it through the sub-issue API before execution.
2. Create `M6_WAVE<n>_PROMPT.md`, `M6_WAVE<n>_PLAN.md`, and the manual before
   each implementation PR. This roadmap is not a substitute for the fixed
   per-wave feature list.
3. Each executor gets one worktree and one issue. Its brief names both files
   and semantic authority, tests to write first, forbidden files, validation,
   and the commit expected for integration.
4. Parallel agents do not edit `tests/features.rs`, plugin composition,
   shared console registration, the wave manual, or another agent's semantic
   authority. The integrator appends feature steps and performs shared wiring.
5. Runtime edits to exterior lifecycle, actor persistence, travel/save,
   water/movement scheduling, scene environment, or presentation selection are
   sequential even if individual hunks appear disjoint.
6. Prepared serialized shape changes bump the corresponding asset revision,
   including serde-defaulted fields.
7. Acceptance discoveries become new child bugs; amend the wave plan's
   `Shipped amendments` section rather than rewriting the original plan.

## Per-wave integration gates

Executors run focused tests. After all wave commits are integrated, the
orchestrator runs:

```powershell
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
```

Then run the wave's representative `cargo run-dev` command and live BRP/MCP
manual. Record measured output on each child issue and on #13. No wave closes
from unit/feature coverage alone when its exit criterion requires real data.

## Continuation issue housekeeping

The post-#261 housekeeping is complete and must remain synchronized with the
live issue state:

- #13 identifies merged PR #261 and this continuation sequence; it closes only
  after W8 and #87, while #14 remains the downstream final milestone gate.
- #10 remains the explicit blocker for W3 runtime actor integration; W7 also
  waits for the W2/W4/W5 evidence and W6-C threshold matrix.
- `M6_WAVE7_MANUAL.md` is a frozen acceptance protocol, not gate-complete
  evidence; W6-C/#285 owns the pre-W7 numeric decision.
- #13's distant/VWD wording keeps the required/default seam-safe route split
  separate from optional far-worldspace presentation work in W8.
