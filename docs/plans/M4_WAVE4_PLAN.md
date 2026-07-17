# M4 wave 4 — travel-door navigation and intercell agents (#113, #134)

Two-issue wave under epic #9 on branch `m4-wave4` off master. Because both
issues rework the same runtime seam (`src/viewer/nav/agent.rs`), the
executors run **sequentially on the wave branch** — #113 first, #134 on
top — instead of parallel worktrees; there is no merge seam to arbitrate
beyond `tests/features.rs`'s usual append-only convention. Model routing
per AGENTS.md: executors write all production and test code; the
orchestrator owns this plan, review, gates, and real-data acceptance.

Wave 3 (#112, PR #129) left three documented gaps this wave closes:

1. Every real door triangle is single-sided; the other side lives in
   another cell's NAVM, linked via NAVI door-merge data. The shipped
   pause/open/traverse state machine never fires on real data.
2. Intra-cell multi-mesh routing is not connected: FranklinMetro02's two
   NAVMs validate as separate landmass islands and a cross-mesh target
   deterministically reports `NoPath`. NAVI merge decode is required.
3. Nothing owns an agent's continuity across a travel-door cell swap
   (new sub-issue #134).

## Issue #113 — Fallout nav adapter: NAVI merges, travel-door links, repath

### 1. NVMI tail decode (prepare side)

- `src/vsa/openmw_esm4/navmesh.rs`: decode the retained `NaviInfoEntry`
  tail per the fopdoc NVMI layout, cross-checked against OpenMW's
  `loadnavi` source: the merged-navmesh FormID array, the
  preferred-merge array, and the linked-door array (door FormID +
  connected navmesh). Unknown trailing bytes stay retained and
  diagnosed, never panic; truncated arrays produce the module's usual
  pre-formatted diagnostics.
- `src/vsa/prepare/nav_graph.rs`: extend `PreparedNavGraph` with
  deterministic cross-mesh connections derived from those arrays, split
  by kind: same-cell merges (both meshes in this cell's graph) and
  door links (door reference FormID + this-cell triangle + other-cell
  navmesh/cell identity). Connections referencing meshes outside the
  prepared cell are kept (they are the travel-door destinations), ones
  referencing unknown FormIDs are diagnosed. Graph fingerprint/summary
  counts updated.

### 2. Cross-mesh routing within a cell (pure + runtime)

- `src/viewer/nav/landmass_graph.rs`: consume same-cell merge
  connections. First try landmass's native island boundary linking; if
  real data does not link (wave 3 evidence suggests boundary vertices
  are not shared), emit deterministic walk-through animation links
  across the matched boundary edges. Document which path real data
  takes. Diagnostics keep their severity-tagged deterministic ordering.
- Gate: a FranklinMetro02 cross-mesh `tna goto` resolves a route (the
  wave 3 `NoPath` repro becomes the acceptance case).

### 3. Travel-door off-mesh links (runtime)

- Door triangles whose reference resolves to a travel door (destination
  cell via the existing manifest/world-transition door metadata, #51/#52)
  become terminal travel links: route to the door triangle, then the
  existing `door_link.rs` lifecycle (pause → scripted open request via
  the existing interaction boundary → wait → traverse). No teleporting
  through closed doors; locked/never-opening doors resolve to the
  existing deterministic `Failed`/unreachable status.
- Closed/locked/unavailable door links are excluded from route planning
  as blocked until usable; a door state change triggers repath.

### 4. Repath policy (pure)

- Pure repath decision module (cucumber-testable, no Bevy imports):
  inputs = door/link state change, target moved beyond tolerance,
  destination cell unloaded, agent off-link; output = keep route /
  repath / fail with the existing `NavAgentStatus` mapping. The Bevy
  system only feeds it observations.

### 5. Console surface

- `tna goto` unchanged but now resolves cross-mesh routes; `tna status`
  additionally reports the active link kind (`merge`/`door <formid>`)
  when traversing. Stable tracing lines keep the wave 3 prefixes
  (`nav agent door wait/resume <formid>`, `nav agent unreachable`).

## Issue #134 — intercell agent continuity

### 1. Ledger policy (pure)

- New std/serde-only module (pattern: `src/viewer/world/policy.rs`):
  ledger entries {agent id, destination cell FormID, destination door
  reference, remaining target}; operations record, claim-on-activate
  (only entries matching the newly active cell, deterministic order),
  stale-entry diagnosis (destination door absent from the active cell).

### 2. Runtime handoff

- When #113's travel-door traversal completes, the agent entity
  despawns from the active cell and is recorded in the ledger resource
  (which survives the existing cell-swap teardown). Stable line:
  `nav agent handoff <formid> -> cell <cellid>`.
- After a cell swap, ledgered agents for the new cell spawn at the
  destination door's marker position and resume toward any remaining
  target, else idle. Stable line: `nav agent restore <formid> cell
  <cellid>`.

### 3. Console surface

- `tna travel <door-formid>`: route the test agent through that travel
  door end-to-end. `tna status` reports `handed off to cell <cellid>`
  while ledgered. Errors reuse existing console error wording patterns.

### Non-goals (both issues)

No offscreen simulation of unloaded-cell agents, no save/load of the
`tna` agent or ledger, no multi-hop cross-cell door-sequence planning,
no exterior stitching (M6), no grounded movement/local avoidance (#114),
no AI packages (#115).

## Testing (feature-first, per issue)

Mandatory order: fix this feature list → write `features/*.feature` +
unit tests → implement until green.

- #113 cucumber: NVMI tail decode (merged/preferred/door arrays, truncated
  input), cross-mesh connection derivation, repath decision table,
  blocked-door exclusion. Pure modules only (`#[path]`, no Bevy).
- #113 minimal-App: travel-door link routes to the door and drives the
  existing `DoorLinkState` lifecycle; a door state change triggers one
  repath; never two concurrent travel requests.
- #134 cucumber: ledger record/claim/stale rules, deterministic ordering.
- #134 minimal-App: traversal despawns agent + populates ledger; matching
  cell activation spawns exactly one agent.
- `tests/features.rs`: each issue appends World fields at the end of the
  struct and a delimited step section at the end of the file.

## Gates and acceptance (orchestrator)

`cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`,
`cargo test`, representative `cargo run-dev -- prepare` on the acceptance
cells. Real-data acceptance over the agent bridge: FranklinMetro02
cross-mesh route (gap 2), `tna travel` through a real travel door with
player follow-through (gap 1 + 3, #134), route/path-latency and repath
evidence commented on each issue. Manual script `M4_WAVE4_MANUAL.md`
written before the wave PR; PR closes #113 and #134.

## Shipped amendments

(amended during acceptance, not rewritten)
