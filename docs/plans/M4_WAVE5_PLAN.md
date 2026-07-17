# M4 wave 5 — grounded movement, mid-route door gating, tnm path (#114, #137, #138; #136 measured)

Wave under epic #9 on branch `m4-wave5` off master. #114 and #137 rework
the same runtime seam (`src/viewer/nav/agent.rs` / `door_link.rs`), so
their executors run **sequentially on the wave branch** — #114 first,
#137 on top (wave 4 precedent). #138 owns a disjoint file
(`src/viewer/nav_overlay.rs`) and runs in a **parallel worktree**. #136
is a measure-first decision at acceptance time, not an implementation
task. Model routing per AGENTS.md: executors (Sonnet) write all
production and test code; the orchestrator owns this plan, merges,
review, gates, and real-data acceptance.

## Issue #114 — physics-authoritative grounded agent movement

Wave 4's agent is kinematic by design: landmass desired velocity is
added to `Transform` each frame and fed straight back as the agent's own
velocity, with `Archipelago3d::sample_point` snapping Y to the navmesh.
This issue inverts movement authority: navigation proposes, physics
disposes.

### 1. Agent physics proxy

- Capsule proxy per nav agent via `bevy_boxddd` (the crate the player
  KCC already uses): radius 0.35 m / height matching the current test
  agent constants, kinematic character-controller policy mirroring
  `src/viewer/player/movement.rs` (`KccState` pattern: collision filter
  vs `WORLD_STATIC | WORLD_DYNAMIC`, step-support probes, gravity,
  grounded state), gated on `CellPhysicsReadiness` like the player.
- Feet/root pivot stays aligned with the visual capsule; no duplicate
  collision ownership (the proxy is the only agent collider).

### 2. Movement authority inversion

- Landmass desired velocity becomes the KCC's *input*; the KCC resolves
  collision/steps/slopes/gravity and moves the `Transform`; the actual
  post-collision velocity is what gets written back to the landmass
  `Velocity3d` (replaces `sync_velocity_from_desired` +
  `apply_kinematic_velocity`).
- The navmesh `sample_point` Y-snap stops being the ground authority;
  it remains only as an off-navmesh diagnostic input. A flat nav polygon
  is not assumed to be a physical floor.
- Door-traversal lerp (`DoorTraversal`) and ledger handoff/restore
  (#134) keep their existing semantics on top of the new movement.

### 3. Pure movement policy module

- New std-only module (cucumber-includable via `#[path]`, pattern:
  `src/viewer/world/policy.rs`): deterministic decision tables for
  slope limit, step height, grounded/airborne transitions,
  collision-rejection clamping (desired vs achievable velocity), and
  stuck detection/recovery (no progress toward waypoint over a window →
  recovery, then deterministic `Stuck` failure). The Bevy system only
  feeds it observations.

### 4. Local avoidance

- Enable landmass local avoidance among same-cell agents with a bounded
  budget. `tna spawn` gains bounded multi-agent support (indexed agents,
  small fixed cap) so avoidance is observable; `tna goto`/`status`
  address agents by index with the current single-agent forms defaulting
  to agent 0. Behavior around paused (door-waiting) agents documented.

### 5. Diagnostics and console

- Stable tracing lines (grep-able prefixes): `nav agent stuck <id>`,
  `nav agent off-navmesh <id>`, `nav agent collision-blocked <id>`.
- `tna status` additionally reports grounded state and stuck/blocked
  condition.

### Non-goals

No player-controller rewrite, no ragdoll, no combat steering, no AI
packages (#115), no exterior terrain/streaming (M6). Animation
locomotion sync is limited to the capsule test agent (real actor
skeletons are the #104–#108 track).

## Issue #137 — closed doors gate nav agents mid-route

Scope per the issue: route/lifecycle gating only; physics blocking of
agents by door colliders arrives with #114 but the route behavior must
be correct standalone.

1. Door-flagged triangles that are *not* already link endpoints (the
   #113 travel/merge links) gate routes: a closed door standing on the
   path triggers the existing `door_link.rs` pause → scripted-open →
   resume lifecycle — same boundary, no new door logic. Decide at
   implementation whether that is modeled as off-mesh links across the
   door triangle or as a route-crossing check; either way the lifecycle
   and its stable lines (`nav agent door wait/resume <formid>`) are
   reused, not duplicated.
2. Locked/never-opening doors: the gated triangle is excluded/blocked in
   planning, yielding the existing deterministic `Failed`/unreachable
   outcome instead of clipping through; a door state change (unlock,
   open) triggers repath via the existing pure repath policy
   (`repath.rs` extends its decision table if a new input is needed).

## Issue #138 — tnm agent path + overlay brightness (parallel worktree)

1. Draw the active agent's current route (waypoint polyline from the
   backend, updated on repath) as part of the `tnm` overlay, visually
   distinct from the mesh triangles. Multi-agent (#114) may land in
   parallel: drawing agent 0's path is sufficient for this wave.
2. Dim/de-emphasize the overlay so it stops driving eye adaptation:
   reduce the unlit material's brightness/alpha, or exclude the overlay
   from auto-exposure metering if the existing `AutoExposure` setup in
   `scene.rs` already supports it (metering mask/compensation). No new
   render systems for a debug overlay.

**File ownership:** `src/viewer/nav_overlay.rs` plus its own feature
file/steps section. If reading the active path requires a hook in
`agent.rs`, it must be a single additive resource written by the agent
systems, in a clearly delimited block — nothing else in that file; the
orchestrator resolves the merge against #114's rewrite.

## Issue #136 — corner clearance (measure, then decide)

After #114 lands, the orchestrator measures corner clearance on the
acceptance routes (below). If capsule-vs-wall collision plus avoidance
already keeps visible daylight at corners, #136 closes with that
evidence. Otherwise the scoped fix (navmesh erosion at
`landmass_graph` conversion vs radius-aware smoothing upstream options)
becomes its own executor task — not silently folded into #114.

## Testing (feature-first, per issue)

Mandatory order: fix this feature list → write `features/*.feature` +
unit tests → implement until green.

- #114 cucumber: movement-policy decision tables (slope/step/grounded,
  clamping, stuck/recovery). Pure module only (`#[path]`, no Bevy).
- #114 minimal-App/World: desired vs actual velocity feedback (a blocked
  agent reports its real, near-zero velocity to landmass); grounded
  gating on `CellPhysicsReadiness`; multi-agent spawn cap.
- #137 cucumber: gating decision table (door-flagged triangle × door
  state × lock state → pass/wait/blocked), repath-on-state-change rows
  appended to the existing repath table.
- #137 minimal-App: closed unlocked door on the path drives the
  `DoorLinkState` lifecycle exactly once; locked door yields
  deterministic unreachable; unlock triggers one repath.
- #138: overlay path entity exists/updates on repath, brightness change
  is asserted on the material constants; console test for unchanged
  `tnm` toggling.
- `tests/features.rs`: each issue appends World fields at the end of the
  struct and a delimited step section at the end of the file.

## Gates and acceptance (orchestrator)

`cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`,
`cargo test`, representative `cargo run-dev -- prepare` on the
acceptance cells. Real-data acceptance over the agent bridge on
FranklinMetro02 (0001a273) and Vault 101 Entrance (00024512):

- #114: agent follows a route with stairs/slope while grounded (no
  hover/tunnel), a wall stops it with `collision-blocked` rather than
  clipping, two agents cross paths without jitter; frame-time cost of
  the agent budget recorded (cool-machine canary per AGENTS.md).
- #137: `tna goto` through a closed unlocked door shows
  `nav agent door wait/resume` and the door visibly opens; locked door
  → deterministic unreachable.
- #136: corner-clearance measurement on the metro corner route,
  commented on the issue with the close/fix decision.
- #138: path polyline visible across the FranklinMetro02 mesh seam;
  `tnm` in a dark corridor no longer blacks out surroundings.

Manual script `M4_WAVE5_MANUAL.md` before the PR; PR closes #114, #137,
#138 (and #136 only if the measurement closes it).

## Shipped amendments

- **#137's "non-travel door" class is empty on real FO3 data.** An
  orchestrator scan of every prepared cell's `navgraph.ron` (12+ cells)
  found that *every* door-flagged NAVM triangle resolves to a travel
  door — the issue's "ordinary interior door that is not a link
  endpoint" has no witness. The wave-4 clip-through was a **travel door
  crossed mid-route**: travel-door triangles with 2–3 walkable
  neighbors are ordinary ground to a route passing through rather than
  terminating there. The crossing gate therefore candidates *every*
  single-sided door, excluding only the one door the agent's own
  `travel_intent` currently targets (that door stays owned by the
  travel-arrival handoff lifecycle); a mid-route crossing resolves
  pause → open → resume with an `IntraCell` destination and no handoff.
- **#114 avoidance budget:** landmass's `ArchipelagoOptions` defaults
  already provide bounded local avoidance once ≥2 agents share the
  archipelago; no new configuration was added.
- **#114 stuck handling is diagnostic + one forced repath,** then a
  latched `stuck` status; it does not halt movement (door-link `Failed`
  owns hard-stop semantics).
