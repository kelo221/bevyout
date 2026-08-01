# M5 gate-support wave — ImageSpace/IMAD screen feedback + M4A evidence

Tracks [#96](https://github.com/kelo221/bevyout/issues/96) and
[#86](https://github.com/kelo221/bevyout/issues/86). This wave is deliberately
separate from combat-architecture Wave 3 in
`M5_COMBAT_ARCHITECTURE_ROADMAP.md`.

## Execution model recommendation

Use **Sol X-High** in the Codex runtime. #96 crosses Fallout record decoding,
prepared serialized catalogs, pure time/keyframe and composition policy,
Bevy 0.19 post-processing, console/agent observability, and gameplay-state
teardown. #86 requires controlled real-data measurement across actor assembly,
animation, physics, rendering, and persistence. The higher reasoning setting
is warranted by the number of boundaries that must remain aligned.

## Baseline, ownership, and dependency map

The integration branch is `m5-gates-wave1`, based on the current
`origin/master` at kickoff. #96 and #86 are separate production/evidence
surfaces and can be developed in parallel conceptually:

| Workstream | Issue | Owns | Must not expand into |
| --- | --- | --- | --- |
| A | #96 | IMAD data, screen-FX policy, camera presentation, request/inspection boundary | combat damage, scripts, weather, V.A.T.S., or M6 exterior precedence |
| B | #86 | actor gate mapping, real-data runs, performance/cache evidence, manual steps | new actor behavior unless a reproducible gate failure requires it |
| C | #96/#86 | final manual, evidence bundle, issue comments, gate checklist | closing #88 or #12 before their own criteria pass |

The functional actor work for #86 is already present on `origin/master`. Its
remaining acceptance checkboxes are real-data frame-time, memory, load-time,
and cache measurements plus repository and real-data gate evidence. Its
predecessors #8 and #6 are closed. #88 remains downstream of both #96 and
#86; final M5 gate #12 also waits for the M4 gate.

## Fixed feature list — #96

### A. Decode and prepare IMAD records

- Add an engine-independent prepared IMAD model without changing the existing
  `ImageSpaceInfo`/IMGS parser. The model must carry the stable FormID,
  optional EDID, DNAM flags/duration, scalar base values, color values, and
  typed time/value keyframes needed by the screen-FX policy.
- Decode the Fallout 3 IMAD subrecords documented by the format reference:
  `BNAM`, `VNAM`, `TNAM`, `NAM3`, `RNAM`, `SNAM`, `UNAM`, `NAM1`, `NAM2`,
  `WNAM`, `XNAM`, `YNAM`, `NAM4`, the recognized `*IAD` multiplier/additive
  curves, and optional `RDSD`/`RDSI` sound FormIDs. Unknown `*IAD` fields are
  retained as diagnostics or ignored explicitly; they must not be silently
  interpreted as a different property.
- Validate finite values, monotonic/duplicate keyframe behavior, payload
  lengths, flags, and durations. Malformed optional curves produce stable
  diagnostics and do not panic or poison unrelated records.
- Store parsed IMAD records in the content-set state and expose the records to
  preparation through a sorted, deterministic collection.
- Write a content-addressed prepared IMAD catalog linked from the scene
  manifest. Do not put a large content-set-wide modifier map directly into
  `CellInfo`; the current base ImageSpace remains the cell's resolved base.
- Add a dedicated `IMAGE_SPACE_MODIFIER_CATALOG_REVISION` (name may follow
  local naming conventions), bump the manifest schema and prepare revision for
  the new link/meaning, and pin those revisions with stale-cache tests.

### B. Pure transient screen-FX policy

Add a Bevy-free policy module, preferably under a feature-local
`src/viewer/screen_fx/policy.rs` included by the existing test harness, with:

- `BaseImageSpaceState` for the active cell base and a separate ordered set of
  active transient modifiers.
- A typed `ScreenFxRequest` boundary carrying source, modifier identity,
  start/stop operation, priority, explicit integer time, and optional
  intensity/settings scope. It must not carry Bevy `World`, `Entity`, timers,
  assets, or rendering types.
- Deterministic IMAD keyframe sampling by elapsed integer milliseconds,
  including duration, non-animatable/static values, endpoint clamping, and
  malformed/empty curve fallback.
- Deterministic composition. Apply modifiers in the total order
  `(priority, stable FormID, request sequence)`; use the authored operation
  type for multiplier/additive curves, ordered alpha composition for colors,
  and explicit clamping for post-process strengths. The policy must document
  every field's combine rule rather than relying on hash-map iteration order.
- Idempotent start, stop, replacement, cancellation, and expiry. Repeating a
  start/end or loading the same saved state must not duplicate a modifier.
- Explicit clear/retain policies for death, save/load, camera mode changes,
  cell transitions, and viewer teardown. Cell base replacement must not leave
  a transient modifier attached to the old camera or old resident cell.
- Settings scaling and disable switches for overall intensity, screen blood,
  flashes, and motion/distortion-heavy effects.

### C. Bevy presentation boundary

- Add a typed `ScreenFxPlugin` to `ViewerPlugins` with narrow systems in the
  existing `ViewerSet` phases. The plugin owns only presentation state and
  consumes `ScreenFxRequest`/policy outputs.
- Apply the base ImageSpace to the active camera separately from transient
  modifiers. Preserve the current IMGS color grading, bloom, and auto-exposure
  behavior as the neutral/base path.
- Map exact Bevy 0.19 capabilities to built-in camera components where they
  fit (color grading, depth of field, and motion blur). Implement remaining
  blur, double vision, radial blur, fade, and hit/screen-blood presentation in
  one bounded screen-space path rather than an unbounded per-effect pass
  stack. The chosen render path must be validated against the local Bevy 0.19
  docs and Metal/WebGPU behavior.
- Make screen blood and hit-shader feedback explicit presentation channels,
  with deterministic fade/clear state and no dependency on gameplay health.
- Emit a representative request from the existing first-person weapon hit
  adapter without moving damage authority into screen FX. Add deterministic
  developer/agent inspection commands for starting, stopping, clearing, and
  inspecting representative effects so a human can see #96 before the later
  combat/effects/script integrations land.
- Refresh the base effect when the active camera/cell changes and clear
  transient state on teardown. Keep M6 exterior ImageSpace precedence and
  M7/M9 gameplay/script integrations explicitly out of this wave.

### D. Tests-first list for #96

1. Add `features/image_space.feature` covering IMAD curve sampling, malformed
   records, deterministic overlapping composition, replacement/cancellation,
   expiry, screen-blood fade/clear, and restoration of the active base.
2. Append the feature world's fields and step section to `tests/features.rs`;
   keep the driven policy module std/serde-only.
3. Add synthetic parser fixtures/tests for fixed DNAM offsets, all supported
   curve/color structures, short/malformed payloads, duplicate timestamps,
   duration, flags, and optional sound FormIDs.
4. Add prepared-catalog, manifest-link, fingerprint, and stale-revision tests.
5. Add pure policy tests for total ordering, field composition, idempotent
   repeated requests, replacement, expiry, all clear reasons, and settings
   scaling.
6. Add minimal Bevy `World`/`App` tests for request-to-camera application,
   built-in component mapping, screen-blood fade/clear, camera/cell teardown,
   and no duplicate application after save/load.
7. Add console/agent golden tests for structured screen-FX status and the
   deterministic developer request surface.
8. Implement until focused tests are green, then run the full repository and
   real-data gates.

## Fixed feature list — #86

### A. Evidence mapping, not actor redesign

- Map every #86 acceptance checkbox to an existing feature/unit test or to a
  named real-data observation. Existing actor catalog, assembly/fallback,
  animation game-flow, actor state, autonomous-actor, and console features
  remain the regression suite.
- Add only missing test coverage discovered by that mapping. Do not add fake
  Cucumber steps that claim a live actor rendered correctly; live visual and
  performance claims belong in the manual and evidence record.
- If an actor defect is found, isolate it as a confirmed production issue and
  amend the plan before changing runtime code. Do not silently widen #86 into
  combat death, ragdoll, corpse, or respawn behavior.

### B. Controlled real-data protocol

Use the native prepared path and exact known records:

- Super-Duper Mart cell `00017f37`, humanoid Raider `00041600` (and the
  autonomous raider set already documented by M4).
- Vault 101 Atrium cell `00024511`, creature Radroach `0005443b`.
- Use the existing `actorinspect`, `actorstate`, `actoranim`, `getpos`, and
  autonomous package inspection surfaces. Verify body/gear assembly, idle/
  walk/run/turn/equip behavior, canonical holder identity, lifecycle mutation,
  save/reload restoration, and no repeated attachment/seed logs.
- Prepare once cold and once warm. Record deterministic prepare summaries,
  actor catalog revision/hash, cache reuse/build counts, load time, and any
  unsupported-data diagnostics.
- Launch the same manifests with the agent bridge at a fixed resolution,
  present mode, physics setting, camera, and render configuration. Capture
  at least three warm steady-state performance windows per representative
  actor cell, with explicit warmup/duration/latest-limit/budget parameters.
  Record average, p50, p95, p99, max, over-budget count, sample count, entity/
   mesh counts, collider/physics counts, and relevant schedule conflicts.
- Record process memory using the platform-appropriate command and label it
  with build profile, dynamic-linking feature, OS, resolution, and cache state.
  Memory and frame results are comparable only when those controls match.
- Treat the collision-cook startup line as the thermal canary. Retry a run
  after a degraded/thermally noisy startup or transient Metal `DeviceLost`.

### C. Tests/gates for #86

1. Run the existing actor feature/unit suites first and add only any missing
   regression scenario found by the acceptance matrix.
2. Run the static Bevy scanner and inspect actor/animation/nav systems with
   their registrations. Record hypotheses separately from measured findings.
3. Inspect `Update`, `PostUpdate`, and actor/navigation schedule snapshots
   with bounded conflict output. Preserve correctness-critical chains; do not
   optimize from conflict count alone.
4. Run the controlled real-data protocol and write the evidence into the
   manual/issue comment. #86 closes only when both missing GitHub checkboxes
   can be ticked with actual evidence.

## Integration and acceptance order

1. Fix the feature lists in this plan and the two GitHub issue descriptions if
   acceptance clarifies a boundary; do not mark checkboxes complete early.
2. Land the #96 parser/catalog/policy tests and the Cucumber feature before
   implementing the Bevy adapter.
3. Implement and test #96's presentation boundary, then connect the existing
   weapon hit adapter and inspection surface.
4. In parallel, run the #86 actor regression matrix and controlled measurements
   against the current `origin/master` actor runtime. Any failure becomes a
   confirmed follow-up or a plan amendment before a fix.
5. Write `M5_SCREENFX_M4A_MANUAL.md` before the integration PR. It must contain
   exact prepared cells, exact actor and IMAD FormIDs discovered from the real
   prepared catalog, exact console/BRP commands, keys, expected output/visual
   result, reset/cleanup, and evidence paths. No placeholders or screenshots
   alone satisfy the gate.
6. Run the repository gates and representative native preparation:

   ```text
   cargo fmt --check
   cargo clippy --all-targets -- -D warnings
   cargo test
   cargo run-dev -- prepare --cell 00017f37
   cargo run-dev -- prepare --cell 00024511
   ```

7. Add measured results to this plan's amendments and comment evidence on
   #96 and #86. Only then tick the relevant epic/gate checkboxes and prepare
   the integration PR with `Closes #96` and the appropriate gate issue
   reference. Do not close #88 or #12 from this wave.

## Revision checklist

Any implementation that adds or changes prepared serialized data must review
and test all applicable revisions together:

- `ImageSpaceModifier`/prepared IMAD catalog revision;
- scene manifest schema revision for new catalog links or meaning;
- prepare pipeline revision for source-to-prepared meaning;
- any save/inspection schema revision if transient state is persisted or its
  wire shape changes.

The transient screen-FX runtime should not be saved in this wave unless the
existing save contract is deliberately extended with a version bump. The
default policy is to clear transient effects on reload and restore only the
base ImageSpace.

## Shipped amendments

### A1 — IMAD fixed-prefix and keyframe interpretation corrected during real-data validation

The initial synthetic parser shape was too eager about the fixed DNAM prefix.
Validation against the Fallout 3 IMAD format and current FO3 editor schema
showed that the HDR/Bloom/Cinematic slots are multiplier/additive keyframe
counts, while the later packed fields contain direct effect values and flags.
The parser now preserves the direct values, radial/DoF flags, optional sound
FormIDs, named timed curves, and diagnostics for unsupported fields. The
recognized xEdit ordering for `*IAD` curves is used (`0x11` saturation,
`0x12` contrast, `0x14` brightness); `0x13` is retained as an explicit
unsupported contrast-average-luminance diagnostic. Normalized authored times
are converted to duration-relative integer milliseconds, so real records such
as `GetHit` (400 ms) and `BloodISFXd` (2,000 ms) do not run for seconds by
accident.

Affected issue: #96. Added fixed-offset parser tests, malformed-payload tests,
curve/color tests, duration/endpoint assertions, and the image-space Cucumber
scenarios. The prepared revision is `openmw-imad-v3`; the scene manifest is
schema 24 and the prepare pipeline is `prepare-v12-imad-screen-fx`.

### A2 — Old KTX-Software input-list compatibility kept inside preparation

The real Vault 101 prepare exposed that the installed KTX-Software 4.4.2
does not support the KTX 5 `@file` input-list convention. Reflection probe
preparation now selects the response-file form only when the tool advertises
it and otherwise passes the validated input paths explicitly. This keeps the
runtime/viewer free of tool dependencies and retains the existing cache
fingerprint behavior.

Affected issue: #96 real-data gate. Both representative cells now prepare
successfully warm with the local tool; no derived game data was added to the
repository.

### A3 — Real preparation and live actor/screen-feedback evidence

The native prepared runs completed as follows:

| Cell | Prepared result | Warm elapsed | Cache/result evidence |
| --- | --- | ---: | --- |
| Super-Duper Mart `00017f37` | 1,693 placements; 12 unresolved; 32 visual issues | 74.69 s | 605 assets reused; 11 actor mappings; 1,380 ready clips; 112 IMAD records |
| Vault 101 Atrium `00024511` | 706 placements; 7 unresolved; 43 visual issues | 17.31 s | 589 assets reused; 17 actor mappings; 1,396 ready clips; 112 IMAD records |

The real screen catalog fingerprint was
`24efdfcef26d1ebb3d347c976da6c85cd8a17e313b8a22c2709ff90b180941d0`.
Live bridge checks confirmed Raider `00041600` assembled with 67 bound
animation targets and its prepared weapon, Radroach `0005443b` assembled as
an authored creature with 48 bound targets, both accepted walk/run/turn/idle
requests, and both actor states retained stable canonical-holder identities.
`GetHit` (`00000162`) and `BloodISFXd` (`00019482`) activated from the real
catalog; settings scaling, timed expiry, and `clear death` all restored the
neutral transient output. Vault's authored base contrast of 1.5 was preserved.

### A4 — M4A performance gate measured on the two representative cells

Each result below is three warm 120-sample windows from the agent bridge,
`budget_ms=16.667`, optimized dynamic-linking development profile, default
1,920x1,080 window, physics enabled, after collision preparation:

| Cell | Average ms (three windows) | p95 range | Max range | Over budget | Warm world | RSS |
| --- | --- | --- | --- | --- | --- | --- |
| Super-Duper Mart `00017f37` | 9.74 / 9.72 / 9.60 | 10.06–10.35 | 10.34–11.29 | 0/120 in every window | 16,384 entities; 3,070 meshes; 47 point lights | ~2.39 GB |
| Vault 101 Atrium `00024511` | 7.35 / 7.50 / 7.37 | 7.65–7.86 | 7.97–8.10 | 0/120 in every window | 8,192 entities; 1,689 meshes; 42 point lights | ~1.63 GB |

The live schedule snapshot was 456 systems, 48 exclusive systems, and 1,725
conflict pairs; `Update` contained 175 systems, 17 exclusive systems, and
1,530 conflict pairs. These counts are recorded for investigation and are
not treated as bottleneck proof without frame-time evidence. The static audit
reported 2,057 candidates (189 reported after the per-category cap), led by
collection allocation (897), entity churn (286), broad mutable resources
(251), exclusive-world access (233), and filesystem I/O (211). No new
screen-FX-specific bottleneck was inferred from the scanner alone.

Affected issue: #86. Functional actor criteria were verified with the existing
actor/animation/state features and the two live manifests; the measured
results above close the remaining frame-time, memory, load, cache, and
schedule-evidence gap. The exact repeatable procedure is in
`M5_SCREENFX_M4A_MANUAL.md`.

### A5 — Overlapping-effect cost measured; clean budget verdict remains open for #96

The required live overlap check started the real `GetHit` (`00000162`),
`ImageSpaceConcussion` (`00000164`), and `ExplosionInFace` (`00000166`)
records together. The bridge reported all three active and deterministic
composed values (`blur=1.0`, `brightness=1.507`, `contrast=0.690`,
`radial_blur=0.030`, `double_vision=0.0031`). After changing the camera
adapter so MotionBlur and DepthOfField components/prepasses are attached only
while their channels are active, a warm 120-sample overlap window measured
18.21 ms average, p95 25.31 ms, max 29.23 ms, and 58/120 over the 16.667 ms
budget; a second noisy window measured 23.20 ms average and 120/120 over.
The same run's neutral window was already 18.13 ms average with 58/120 over,
while the process was using about 195% CPU and had a degraded 4.16 s
collision-cook line. The viewer then lost its monitor/window before a third
overlap window could be collected.

This is valid evidence that the overlap path is exercised, but not a clean
pass/fail comparison for the screen-FX cost because the machine was thermally
noisy. The component lifetime fix prevents effects from leaving the expensive
MotionVector/DoF prepasses enabled after clear. #96 therefore remains open
until a cool-machine three-window overlap run can establish the budget result;
the exact command sequence is retained in the manual rather than hiding this
finding in the wave scope. No serialized revision changed in A5.

### A6 — Three-window overlap retry completed

The follow-up run used the same Super-Duper Mart manifest after warmup and kept
the real `GetHit` (`00000162`), `ImageSpaceConcussion` (`00000164`), and
`ExplosionInFace` (`00000166`) modifiers active together for each window. The
bridge reported `active_count=3` with modifier IDs `354`, `356`, and `358` and
non-neutral composed values. Three consecutive 120-sample windows measured:

| Window | Average ms | p50 ms | p95 ms | p99 ms | Max ms | Over budget |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | 11.72 | 11.46 | 14.12 | 14.87 | 15.45 | 0/120 |
| 2 | 6.89 | 6.77 | 7.74 | 7.86 | 7.92 | 0/120 |
| 3 | 6.79 | 6.75 | 7.25 | 7.53 | 7.70 | 0/120 |

The world snapshot was 16,384 entities, 3,057 mesh entities, and 47 point
lights. This establishes the representative overlapping-effect budget gate for
#96. The neutral windows from the same viewer launch were noisier, so these
results establish the absolute overlap budget but are not used to claim a
precise neutral-to-effect delta. No serialized revision changed in A6.
