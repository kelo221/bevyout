# M4 gate wave — Sandbox/patrol unblockers and final gate #10

Epic: [M4 #9](https://github.com/kelo221/bevyout/issues/9). Gate:
[M4 #10](https://github.com/kelo221/bevyout/issues/10).

## Execution recommendation

Use **Sol X-High** in the Codex runtime. The implementation is small but sits
on a previously failure-prone AI resolution/family seam and ends with a
multi-cell real-data acceptance matrix, persistence checks, and measured
runtime evidence.

## Why this is the next executable wave

The direct M5 dependency is not ready to advance: #11 is blocked by #10, and
#10's final player-facing behavior criteria are still unchecked. The earlier
prerequisites are now satisfied (#6, #8, and #86 are closed), so the next work
is to make the remaining package behavior pass on the current integration
head and execute the gate.

The live audit found two kinds of stale state that must be kept distinct:

- The real Sandbox failure in [#222](https://github.com/kelo221/bevyout/issues/222)
  is still present in `origin/master` and must be fixed before a real Sandbox
  schedule can be accepted.
- The patrol arrival/dwell repairs for [#231](https://github.com/kelo221/bevyout/issues/231)
  and [#242](https://github.com/kelo221/bevyout/issues/242) exist as commits
  `1c9613b` and `675b374` on the unmerged autonomous-actors follow-up branch;
  they are not in the current `origin/master` and must be re-applied/reviewed
  against this head rather than assumed to have landed.

## Issue order and ownership

| Order | Issue | Work | Dependency |
| --- | --- | --- | --- |
| 1 | [#222](https://github.com/kelo221/bevyout/issues/222) | Convert PACK location radius and target distance from Fallout units to metres exactly once at the pure runtime resolution boundary. | Current M4 package resolver |
| 2 | [#231](https://github.com/kelo221/bevyout/issues/231) | Require a patrol actor to depart its previous marker before accepting the next arrival. | #213 marker-chain resolution |
| 3 | [#242](https://github.com/kelo221/bevyout/issues/242) | Add marker dwell and a tolerance compatible with short authored patrol legs; retain authored marker orientation. | #231 |
| 4 | [#10](https://github.com/kelo221/bevyout/issues/10) | Run the complete M4 gate matrix, record evidence, and close only if every criterion passes. | #222, #231, #242 and merged current-head gates |

All four issues are assigned to the human user before execution. The three
implementation issues share `src/viewer/ai` and run sequentially. The gate is
evaluation-only unless acceptance discovers a new confirmed defect; such a
defect gets its own child issue and plan amendment instead of being silently
added here.

## Fixed feature list

### F0 — Freeze the gate contract and baseline

- Use the exact #10 acceptance criteria: actor/package preparation, actor
  movement and animation, six package families (Patrol, Travel, Follow, Eat,
  Sleep, Sandbox), NAVM/local-obstacle/off-mesh traversal, deterministic
  perception/disposition/faction behavior, cell/save restoration, diagnostic
  reporting, and measured frame/memory/path/load evidence.
- Run the existing AI, actor, navigation, persistence, and architecture tests
  on the current head before changing code. Preserve failures as baseline
  evidence rather than changing the gate wording to fit the implementation.
- Do not close or tick unrelated M4 follow-ups such as #148, #178, #179,
  #190, #216, #217, #219, #220, #221, #226, #227, #230, #232, #240, or #250.
  They are only pulled into this wave if a reproducible failure directly
  violates a #10 criterion.

### F1 — #222: one Fallout-unit conversion at the resolver boundary

- Keep prepared PACK/catalog values in their decoded Fallout units.
- In the pure `viewer::ai::resolution` policy, convert `PLDT.radius` and
  `PTDT.count_or_distance` to Bevy metres exactly once using the established
  `FO3_SCALE` (`1.0 / 70.0`) convention. The returned `ResolvedPoint.radius`
  must be metres for every location/target type.
- Add feature-first coverage for a native radius/distance of `1024` resolving
  to `1024 / 70` metres, plus non-positive input behavior and a regression that
  ordinary reference positions are not scaled a second time.
- Keep this runtime-only change free of prepared serialized shape changes; no
  `*_REVISION` bump is expected. If the implementation moves scaling into a
  prepared catalog instead, it must add the required serialized revision and
  stale-cache tests before acceptance.

### F2 — #231: patrol departure gate

- Add the feature scenario and pure unit coverage before implementation: an
  actor that arrives at marker A must not advance repeatedly while still at A,
  and a short leg must advance after the actor has genuinely departed.
- Port/review the existing `departed_marker` direction against the current
  `origin/master`; do not copy a stale branch wholesale. The KCC/nav agent
  remains the movement authority and the family only emits route/animation
  requests.
- Preserve deterministic marker ordering, lifecycle state, and no per-tick
  `patrol advance` spam.

### F3 — #242: marker dwell and short-leg tolerance

- Add feature/unit coverage for arrival → stop → idle/dwell → next route, and
  for the real short second leg in the `00041600` chain.
- Build patrol waypoints through one patrol-marker constructor that preserves
  authored heading. Until `XPRD` is decoded, use the existing explicit default
  dwell policy and document it; do not invent per-reference authored wait data.
- Use a tolerance compatible with both the authored short leg and the nav
  arrival threshold. Keep the policy pure and deterministic.

### F4 — #10 gate evidence

Map each criterion to automated coverage and a real-data observation before
claiming closure:

| #10 criterion | Evidence seam |
| --- | --- |
| Actor assembly and valid fallback | Closed #86 evidence plus current actor/animation regression suite. |
| Idle/walk/run/turn/equip | `actor_animation_gameflow`, actor-state/animation tests, and live `actoranim`/`actorinspect` on Raider `00041600` and Radroach `0005443b`. |
| Patrol/Travel/Follow/Eat/Sleep/Sandbox schedules | `package_families`, `ai_follow_sandbox`, lifecycle, autonomous-driver features; live Patrol `00041600`, Sandbox `0005443b`, and the existing travel route. Other families must have deterministic automated execution evidence even where no real cell selects them first. |
| NAVM polygons, local obstacles, off-mesh door/cell link | Vault 101 short grounded route, travel door `00028579`, cell handoff to `00024511`, and existing nav portal/door features. Do not use the known invalid closed-door spawn as a false success. |
| Detection/disposition/faction reactions | `perception_awareness` and `faction_hostility` features plus live `perception 0005cf10 player` output. |
| Cell/save restoration | M4 actor-state save/reload path for Raider `00041600`, canonical item identity, and the Vault 101 agent/cell restore path. |
| Unsupported data is reported | Prepare package/actor/animation diagnostics; deferred data must be labeled deferred and unsupported data must not disappear silently. |
| Frame, memory, path, and load budgets | Three comparable bridge performance windows, process RSS, path-latency logs, and load-to-`InGame` timing on the named cells, with the collision-cook thermal canary recorded. |
| Repository and real-data gates | `fmt`, strict Clippy, full tests, representative native prepare, and the manual script. |

The gate issue remains open if any criterion is only inferred from a unit test
when its wording requires real-data evidence, or if a known failure is merely
listed as a follow-up without deciding whether it violates #10.

## Tests-first order

1. Amend this plan/feature list and the issue descriptions only where the
   acceptance boundary is clarified.
2. Add the #222 Cucumber scenario and pure tests; add the #231/#242 patrol
   scenarios and unit tests. Append shared `World` fields and step sections at
   the end of `tests/features.rs`, each with a delimiter.
3. Implement #222, then #231, then #242 until focused tests are green. Keep
   pure policy modules free of Bevy imports and keep unit tests in dedicated
   test modules.
4. Run the full repository gates and reprepare the exact real cells.
5. Run `M4_GATE_WAVE_MANUAL.md`, record measured evidence in this plan and on
   each issue, then close #222/#231/#242 through the implementation PR and
   close #10 only after its complete matrix passes.

## File ownership and architecture boundaries

- #222: `src/viewer/ai/resolution.rs` and its dedicated tests/feature seam.
- #231/#242: `src/viewer/ai/families.rs`, the narrow runtime waypoint adapter,
  and their dedicated tests/feature seam. Keep execution sequential because
  both change the same patrol state machine.
- No changes to `src/main.rs`, no new global AI authority, and no combat/M5
  policy in this wave.
- No prepared asset shape changes unless the implementation proves they are
  necessary; any such change must bump every affected prepared revision.
- Do not commit Bethesda-derived RON, GLB, DDS, WAV, NIF, or cache outputs.

## Required repository gates

```text
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
cargo run-dev -- prepare --cell 00017f37
cargo run-dev -- prepare --cell 00024511
cargo run-dev -- prepare --cell 00024512
```

The final PR must link the manual and include `Closes #222`, `Closes #231`,
`Closes #242`, and `Closes #10` only if the corresponding evidence is real and
complete. M5 Wave 4 remains downstream of this gate.

## Shipped amendments

- A1 (implementation, 2026-08-02): re-applied and reviewed the #222/#231/#242
  seam on the current head. PACK location radii and target distances now
  convert native Fallout units to metres once at pure runtime resolution;
  patrol drivers require departure from the previous marker, dwell for an
  explicit 3.0 seconds, and retain linked-marker authored yaw. No prepared
  serialized shape changed, so no revision bump was needed.
- A2 (automated evidence, 2026-08-02): focused location/family features passed;
  `cargo test` passed 1,619 unit tests plus 633 Cucumber scenarios / 3,116
  steps; `cargo fmt --check` and strict Clippy passed. Native preparation
  completed for `00017f37`, `00024511`, and `00024512`. `git diff --check`
  passed.
- A3 (live evidence, 2026-08-02): the prepared Super-Duper Mart bridge loaded
  1,693 placements and started real Raider `00041600` patrol execution. Its
  linked marker route produced navigation path latencies and `run`/`idle`
  transitions; `patrol advance` events were separated by the configured dwell
  rather than emitted every frame. The Vault bridge selected real
  `CG04RadroachSandbox1024`, resolved authored radius `1024` to
  `14.6285715103` metres at `(-47.6438,120.6857,12.3696)`, and accepted an
  atomic `tp` plus `tna bind` at that position. Stable Sandbox movement/status
  evidence was not captured before the short viewer trace ended.
- A4 (gate status): #222/#231/#242 are implementation-ready but #10 remains
  open. Travel/cell handoff, perception, save restoration, and the three
  comparable performance windows have not been completed here, so this wave
  does not claim the M4 final gate or tick its checklist.
