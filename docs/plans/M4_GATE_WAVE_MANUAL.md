# M4 gate wave — manual acceptance for #222, #231, #242, and #10

This wave makes authored AI package distances use metres, prevents short
patrol chains from advancing or reversing every frame, and records the final
M4 actor/package/navigation/persistence gate evidence. It does not claim that
unrelated M4 fidelity or tooling follow-ups are complete.

## 0. One-time setup

From the repository root, prepare the three real cells with the current native
pipeline. Derived output stays under `.bevyout/`.

```sh
cargo run-dev -- prepare --cell 00017f37   # Super-Duper Mart
cargo run-dev -- prepare --cell 00024511   # Vault 101 Atrium
cargo run-dev -- prepare --cell 00024512   # Vault 101 Entrance
```

Record the deterministic package/actor/animation summary lines. The package
catalog must report unsupported data explicitly; known script/idle payloads
may be reported as deferred. The prepared manifests must be current for the
revision under test.

## 1. Repository and feature gates

Run:

```sh
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
```

Expected: all checks pass, including the package-family, follow/sandbox,
perception, autonomous-driver, actor-state, navigation, and architecture
coverage. Save the command output as the gate evidence.

## 2. Patrol — Super-Duper Mart `00041600`

Launch:

```sh
cargo run-dev -- view --manifest .bevyout/cache/scenes/00017f37/scene.ron \
  --agent-bridge --agent-port 15702 --unfocused --trace-seconds 45
```

Use the console/agent bridge:

```text
showpackages 0002f6e2
tna bind 0 00041600
runpackage 00041600
runpackage 00041600 status
```

The current viewer may auto-bind live actors during startup. If `tna bind`
returns `already_spawned`, keep the existing autonomous binding and continue
with `runpackage ... status`; the command is still useful on a run where the
actor was not auto-bound.

Expected:

- `showpackages` selects the authored Patrol package and resolves the first
  linked marker instead of reporting a missing linked reference.
- The patrol starts at marker `0/2`, reaches marker `00041601`, and later
  reports marker `1/2` targeting `00041602`.
- The actor stops and requests idle at a marker for the configured dwell; it
  does not emit `patrol advance` at frame rate or reverse before departing.
- `00041600` changes position between two `getpos`/`actorinspect` observations
  while the package is running. Stop it with `runpackage 00041600 stop`.

Retain the viewer log and the two status observations as #231/#242 evidence.

## 3. Sandbox — Vault 101 Atrium `0005443b`

Stop the first viewer and launch:

```sh
cargo run-dev -- view --manifest .bevyout/cache/scenes/00024511/scene.ron \
  --agent-bridge --agent-port 15702 --unfocused --trace-seconds 45
```

Run:

```text
showpackages 000522db
```

If the initial player position is `(0,0,0)`, use this atomic teleport before
binding so the nav build sees prepared terrain before physics can apply
gravity:

```text
tp -47.6438 120.6857 12.3696
```

Then run:

```text
tna bind 0 0005443b
runpackage 0005443b
runpackage 0005443b status
```

If the actor was already bound, omit the duplicate `tna bind` after noting
the `already_spawned` response.

Expected:

- The selected Sandbox package resolves its editor-location center to the
  Radroach's authored position `(-47.64,120.69,12.37)`.
- The displayed radius is approximately `14.63` metres for the authored
  native value `1024`, not `1024` metres.
- The package remains in a bounded running/idle cycle inside the cell instead
  of failing after routing outside the interior. Record at least two status
  lines and the actor position.

## 4. Travel door, NAVM route, and cell handoff

Stop the Sandbox viewer and launch the Entrance:

```sh
cargo run-dev -- view --manifest .bevyout/cache/scenes/00024512/scene.ron \
  --agent-bridge --agent-port 15702 --unfocused --trace-seconds 60
```

Run:

```text
tna spawn
tna goto 154 40 -90
tna status
tna travel 00028579
tna status
```

Expected:

- The short grounded route crosses multiple NAVM polygons and reaches the
  target with `stuck=false` and no collision-block failure.
- Travel door `00028579` opens, the agent hands off to cell `00024511`, and
  `tna status` reports the destination cell. The handoff is an off-mesh/cell
  transition, not a teleport-only test.
- Follow the same door with `activate 00028579` and verify the destination
  manifest loads. Do not use the known closed-door spawn inside
  `MetroGateLoad` as a purported successful route.

## 5. Perception and deterministic authored package behavior

Use the Super-Duper Mart viewer from step 2 or relaunch it, then run:

```text
perception 0005cf10 player
showpackages 0005cf10
```

Expected: the output includes a deterministic disposition/hostility rule,
distance/LOS/awareness values, and authored package priority. A genuine
unresolved package point is reported with a stable diagnostic rather than
silently converted to `(0,0,0)`; use `00041600` for the real patrol execution
surface.

The pure feature suite must also pass the deterministic Follow, Eat, Sleep,
Travel, Idle, and Sandbox scenarios. Their real data need not all be the
highest-priority package in one cell; the gate evidence must identify which
families were exercised live and which were exercised through the pure
runtime family harness.

## 6. Actor and package persistence

In the Super-Duper Mart viewer, inspect and mutate Raider `00041600`:

```text
actorstate 00041600
setactorvalue 00041600 health -12
setactorpackage 00041600 0002c6f1 3 4.5
save m4-gate-wave
```

Stop and relaunch the same manifest:

```sh
cargo run-dev -- view --manifest .bevyout/cache/scenes/00017f37/scene.ron \
  --save-slot m4-gate-wave --agent-bridge --agent-port 15702 --unfocused
```

Run:

```text
actorstate 00041600
```

Expected: the health mutation, package checkpoint, lifecycle/reference
identity, and canonical equipped item-instance identity survive restart with
no duplicate actor inventory. The existing Vault 101 cell-swap acceptance
must also report the nav agent frozen/restored at the same position.

## 7. Runtime budget evidence

After each representative viewer has completed its collision cook, capture
three comparable 120-sample windows:

```sh
curl -sS -X POST http://127.0.0.1:15702/ \
  -H 'Content-Type: application/json' \
  -d '{"jsonrpc":"2.0","id":1,"method":"bevyout.performance_snapshot","params":{"latest_limit":120,"budget_ms":16.667,"include_samples":false}}'
curl -sS -X POST http://127.0.0.1:15702/ \
  -H 'Content-Type: application/json' \
  -d '{"jsonrpc":"2.0","id":2,"method":"bevyout.schedule_snapshot","params":{"schedule_contains":"Update","include_systems":false,"conflict_limit":10}}'
ps -axo pid,rss,vsz,etime,command | rg 'target/debug/bevyout.*--agent-port 15702'
```

Record average, p50, p95, p99, max, over-budget count, sample count, entity/
mesh counts, path-latency logs, load-to-`InGame` time, and RSS. Record the
build profile, resolution, cache state, and collision-cook canary. Retry a
thermally degraded or `DeviceLost` run rather than mixing it into the gate
numbers.

## 8. Closure evidence

The wave is complete only when the plan contains the measured results, the
four issue records contain their evidence, the implementation PR closes
#222/#231/#242, and #10's full acceptance checklist is ticked and closed.
Leave unrelated M4 follow-ups open and explicitly list them as non-blocking.
