# M4 wave 3 — bevy_landmass navigation backend spike (#112)

Single-issue wave under epic #9 on branch `m4-wave3` off master. One
executor works directly on the wave branch per AGENTS.md model routing;
the orchestrator owns GitHub housekeeping, diff review, gates, and
real-data acceptance.

Decision under test: adopt bevy_landmass 0.12.0 (landmass 0.9.0, Bevy
0.19) as the navigation backend. The spike either adopts it with a pinned
release and documented configuration, or rejects it with measured
evidence. No generic navigation trait over a single backend.

## Fixed feature list

### 1. Dependency

- `bevy_landmass = "=0.12.0"` (pinned exact). Add `landmass` directly
  only if the pure conversion module needs types bevy_landmass does not
  re-export. No other new dependencies.

### 2. Pure graph conversion (`src/viewer/nav/landmass_graph.rs`)

- New `src/viewer/nav/` module: the runtime navigation slice seam that
  #113 will grow into.
- Pure conversion, cucumber-testable via `#[path]` (no `bevy` imports;
  `landmass`/`glam` are fine): `PreparedNavGraph` →
  - a validated landmass 3D navigation mesh (vertices are already Bevy
    metres from #111 — no second coordinate conversion; polygon vertex
    order/winding fixed here once, verified against landmass validation),
  - non-walkable polygons excluded (or mapped to a distinct type index —
    whichever landmass validation favors; document the choice),
  - a deterministic door-link descriptor list from the graph's door
    associations (door FormID, polygon, edge midpoints) for feature 4,
  - severity-tagged conversion diagnostics with deterministic ordering
    (degenerate/invalid polygons skipped, never panics).
- Unreachable/invalid-target and agent-state mapping policy
  (`landmass state → project NavAgentStatus`) as pure functions.

### 3. Spike runtime (`src/viewer/nav/agent.rs` + plugin)

- `NavBackendPlugin` (Bevy `Plugin` value, registered from the viewer
  module tree, not `main.rs`): owns one `Archipelago3d` + one island per
  active cell, built lazily from `PreparedSceneManifest::nav_graph` the
  same way `nav_overlay.rs` reads `navgraph.ron`; torn down on cell swap
  (reuse the existing swap teardown pattern).
- Crude test actor: capsule mesh + `Agent3d` bundle sized for a humanoid
  (radius ~0.35 m, height ~1.8 m, feet pivot — verify against landmass
  FAQ vertical-sampling guidance and set `PointSampleDistance3d`
  accordingly for stairs/slopes).
- Movement is kinematic for the spike: apply landmass desired velocity to
  the transform each frame and feed the applied velocity back to the
  agent (upstream FAQ: stale velocity causes corner slowing). No
  bevy_boxddd physics, no stepping/slopes — #114 owns grounded movement.
- Console command family `tna` (test nav agent), the wave's visible
  surface per AGENTS.md, driven via `bevyout.console.exec`:
  - `tna spawn` — spawn the test agent at the player's position (error if
    no nav graph, reusing the `no_nav_graph_error` wording).
  - `tna goto <x> <y> <z>` and `tna goto player` — set the agent target.
  - `tna status` — deterministic one-line state (position, status,
    target).
  - `tna despawn` — remove agent (and `tna` with no args prints usage).
- Stable grep-able `tracing` lines for evidence: `nav agent spawn`,
  `nav agent path latency_ms=<x>`, `nav agent reached`,
  `nav agent unreachable`, `nav agent door wait <formid>` /
  `nav agent door resume <formid>`.

### 4. Door / animation link exercise

- Intra-cell doors only: door-link descriptors from feature 2 become
  landmass animation links (off-mesh links) when both sides resolve to
  polygons in the loaded cell's graph. Travel doors / cell swaps stay in
  #113 with the existing world transition policy (#51/#52) — the spike
  must never teleport an actor across a cell boundary.
- Link lifecycle as a pure state machine (unit-tested): reach link →
  pause agent → request door activation through the existing door
  interaction boundary (same code path as the `activate` console
  command; no new door logic) → wait for the door's open state → traverse
  → resume. Failure (door stays closed/locked) → deterministic
  unreachable-style status, agent stops at the link.

### 5. Measurements (spike evidence, not permanent instrumentation)

- Path latency (target set → first non-empty path) logged once per
  request; per-frame landmass update cost behind the existing
  `--trace-seconds` diagnostics path. No per-frame logging in release
  behavior.

## Tests (feature-first, before implementation)

- `features/nav_backend.feature` + steps appended to `tests/features.rs`
  (World fields at the end of the struct, delimited step section at the
  end of the file), all on synthetic in-memory `PreparedNavGraph` values:
  conversion vertex/polygon mapping, winding, non-walkable exclusion,
  degenerate-polygon diagnostic, door-link descriptor extraction and
  determinism, landmass validation success on a known-good graph,
  agent-state mapping, unreachable mapping, door-link state machine
  transitions (pause, resume, failure).
- Minimal-`App`/`World` unit tests (console harness pattern from
  `console.rs` tests): `tna` command parsing/errors, spawn/despawn
  round-trip, archipelago teardown on cell swap.
- Gates: `cargo fmt --check`, `cargo clippy --all-targets -- -D
  warnings`, `cargo test`, representative `cargo run-dev -- view` smoke.

## Real-data acceptance (orchestrator)

- `000151e3` (MegatonPlayerHouse): spawn agent, `tna goto` across the
  room; verify `reached`, record path latency and frame cost over the
  agent bridge.
- `0001a273` (FranklinMetro02, 2 meshes, 3 doors): route requiring a
  door; verify pause → door opens → resume on the far side, or document
  precisely why real FO3 door links cannot be exercised intra-cell (that
  finding feeds #113's design).
- Comment measured results and the adopt/reject decision on #112.

## Out of scope

Full Fallout adapter: travel doors/cell handoff, flag/cost mapping,
repathing policy (#113); grounded physics movement, steps/slopes,
avoidance tuning (#114); AI packages (#115); exterior stitching (M6).

## Manual acceptance

`docs/plans/M4_WAVE3_MANUAL.md`, written before the wave PR and linked
from its body.

## Shipped amendments

- **No intra-cell two-sided door links exist in real FO3 interior data.**
  Every door triangle across the prepared cells (000151e3, 00003a35,
  0001a273) is single-sided: the other side lives in another cell's NAVM,
  linked via NAVI door-merge data. The animation-link runtime, the
  `scripted_door_open` boundary, and the pause/open/resume state machine
  shipped fully unit- and cucumber-tested, but cannot fire on real data;
  wiring them to travel doors (and NAVI-based cross-NAVM merges) is
  #113's job, per this plan's own "or document precisely why" branch.
- **Per-frame ground snap added to the kinematic agent.** Real-data
  acceptance on FranklinMetro02's sloped corridor exposed y frozen at
  spawn height until the agent left the sampling envelope
  (`AgentNotOnNavMesh` mid-route). `apply_kinematic_velocity` now snaps y
  to `Archipelago3d::sample_point`'s surface point using the same
  envelope as the archipelago options; a miss leaves y unchanged. This is
  surface tracking for the spike agent, not #114's grounded movement.
- **`glam` pinned to bevy_math 0.19's exact version (`=0.32.1`)** so the
  pure conversion module's `Vec3` is type-identical to
  `bevy_landmass`'s coordinates without importing `bevy`; the plain
  `landmass` crate was not needed.
- **`PointSampleDistance3d::from_agent_radius` defaults rejected on real
  data** (0.07 m horizontal / 0.35 m below reports `AgentNotOnNavMesh`
  for an agent standing on the MegatonPlayerHouse mesh); explicit
  humanoid-scale distances (1.0/1.0/2.0) shipped instead.
- **Real FO3 NAVM winding requires landmass's reversed attempt** on every
  mesh tested; the conversion tries authored order first, then reversed,
  once per mesh, with a warning diagnostic.
- **Intra-cell multi-mesh routing is not connected**: FranklinMetro02's
  two NAVMs validate as separate landmass islands and a cross-mesh target
  deterministically reports `NoPath`. Connecting them needs NAVI merge
  decode in the adapter (#113).
