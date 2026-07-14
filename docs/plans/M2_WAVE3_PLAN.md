# M2 Wave 3 — Door Animations, Fingerprints, Reveal Amortization

Epic: [#5 — M2 Bulk preparation and connected interiors](https://github.com/kelo221/bevyout/issues/5)
Kickoff: [M2_WAVE3_PROMPT.md](M2_WAVE3_PROMPT.md)

Goal: activating any door visibly opens it (travel doors animate into the
already-instant swap), fingerprint changes invalidate exactly the affected
prepared cells, and the first reveal of a large preloaded cell stays inside
the 33 ms frame budget. Then the remaining gate #6 state-persistence work
lands as wave 3b.

| Issue | Scope | Wave |
|-------|-------|------|
| [#57](https://github.com/kelo221/bevyout/issues/57) Door/activator controller animations | GLB animation export + viewer playback synced to activation | **3a** |
| [#49](https://github.com/kelo221/bevyout/issues/49) Fingerprint validation | plugin/converter/physics/prep fingerprints, targeted invalidation, report | **3a** |
| [#55](https://github.com/kelo221/bevyout/issues/55) First-reveal amortization | instrument the reveal frame, bounded chunked reveal | **3a** |
| Loading-fallback fades, cancellation, failure recovery | gate: "Failed loading returns safely to the source cell" | **3b** (issue pending) |
| Persist dynamic/inventory/enable-parent state across revisits and restart | gate: "Dynamic, inventory, and enable-parent state survive revisits and restart" | **3b** (issue pending) |

Deferred past this wave (in epic #5 but not required by gate #6 acceptance
criteria): resumable `bake --all-interiors`; the cell
ownership/asset-barrier/unload-lifecycle item beyond what state capture in
wave 3b needs.

**Ordering rule inside every issue (repo convention): feature list fixed below →
Cucumber feature + tests written first → implementation makes them green.**

Execution: one Sonnet agent per issue in an isolated git worktree, branches
`m2-57-door-anim`, `m2-49-fingerprints`, `m2-55-first-reveal`, merged into the
`m2-wave3` integration branch. Orchestrator merges, runs `cargo fmt --check`,
`clippy --all-targets -- -D warnings`, `cargo test`, re-prepares the Vault 101
corpus (converter revision bump invalidates every cached GLB), and verifies
the live viewer over BRP.

File-ownership boundaries (shared seam: `tests/features.rs`, append-only World
fields + delimited step sections; one new `features/*.feature` per issue):

- #57: `src/vsa/assets/blender_script.py`, `src/vsa/assets/mod.rs`
  (revision const only), `src/viewer/animation.rs` (new),
  `src/viewer/interaction.rs`, `src/viewer/mod.rs` (install line),
  `features/door_animation.feature`.
- #49: `src/vsa/prepare/**`, `src/cli.rs`, `features/fingerprints.feature`.
- #55: `src/viewer/world/**`, `features/first_reveal.feature`.

## Spike decision — animations ride in the GLB (no sidecar)

Spiked on `vdoorsliding01.nif` with Blender 5.1.2 + niftools v0.1.1 before
this plan was written (wave-3 caution in the prompt). Result: **enable
`animation=True` on NIF import, regroup the imported per-sequence actions
onto NLA tracks named by their NIF controller sequence, and export with
`export_animations=True, export_animation_mode='NLA_TRACKS'`.** The GLB then
carries one named animation per sequence (`Open`, `Close`), each animating
every door node (verified: both `VDoorTop01`/`VDoorBottom01`, 1.33 s / 1.30 s
at 30 fps). No sidecar format is needed; Bevy's glTF loader exposes named
animations directly.

Findings the implementation relies on:

- niftools' animation import is broken on Blender 5.x (slotted actions
  removed `Action.fcurves`). Three shims fix it, same pattern as the existing
  `patch_niftools_blender52`: `Animation.create_action` binds the action's
  slot to the object, `Animation.create_fcurves` creates curves through
  `layer/strip/channelbag`, and a read-only `bpy.types.Action.fcurves`
  property serves the addon's two remaining read sites.
- niftools names imported actions `<Sequence>_<NodeName>` (e.g.
  `Open_VDoorBottom01`); the sequence name is recovered by stripping the
  `_<object name>` suffix. Without NLA regrouping, the glTF exporter only
  exports each object's active action.
- NIF text keys arrive as action pose markers (`start`, `end`,
  `sound: DRSVaultVertical01Open`) — available later for frame-accurate
  sound sync; wave 3 keeps the existing activation-time sound playback.
- `NIF_CONVERTER_REVISION` bumps to a `…-anim-v4` value: every cached GLB is
  invalidated, so acceptance includes one full corpus re-prepare (#49 makes
  exactly this situation detectable).

## Issue #57 — Door and activator controller animations

### Feature list

- **F57.1** `blender_script.py`: import with `animation=True`; add the three
  slotted-action shims to the patch function; after collision-object removal,
  clear active actions and push every imported action onto an NLA track named
  by its sequence; export with `export_animations=True,
  export_animation_mode='NLA_TRACKS'`. Bump `NIF_CONVERTER_REVISION`.
- **F57.2** New `src/viewer/animation.rs`: when a placement scene finishes
  spawning, index its `AnimationPlayer` and named clips. On door/container
  open-state change and on activator activation, play the matching clip
  (`Open`/`Close`; an activator with clips but no `Open` plays its first
  clip), holding the final pose. Assets without animations behave exactly as
  today. Log line: `door anim <reference formid> <clip> lead_ms=<x>`.
- **F57.3** Travel doors: the `DoorTravelRequested` write is deferred by an
  open-lead computed by a pure policy function (clip duration capped — the
  swap must still feel immediate; cap chosen by test, ~600 ms). The message
  still goes through the same `DoorActivationSet` path, so `world::swap`
  is untouched (that file belongs to #55).
- **F57.4** Sync rule: `InteractionState::open` stays the single source of
  truth; animation is presentation only. Repeated activation mid-animation
  reverses cleanly (play the opposite clip from the current pose).
- **F57.5** `features/door_animation.feature`: pure clip-selection and
  open-lead policy (which clip for which transition, lead capping, missing
  clip ⇒ zero lead, mid-animation reversal decision).

### Tests before code

- **T57.1** Clip selection: opening picks `Open`, closing picks `Close`,
  activator without `Open` picks first clip, asset without clips picks none.
- **T57.2** Open-lead: lead = min(open-clip seconds, cap); no destination ⇒
  no deferral; no clip ⇒ zero lead (travel fires same frame, wave-2
  behavior preserved).
- **T57.3** Mid-animation reversal decision is pure and covered.
- **T57.4** Bevy-side unit test (`World`/minimal `App`): toggling a door
  entity's open state queues exactly one play request for the right clip;
  deferred travel request is written after the lead elapses.
- **T57.5** Cucumber scenarios for T57.1–T57.2 shapes.

The agent must not run Blender (spike output above is authoritative; the
orchestrator re-prepares the corpus during acceptance).

## Issue #49 — Fingerprint validation

### Feature list

- **F49.1** Every prepared cell's job-manifest entry records four
  fingerprints at completion: plugin content-set (the batch session's chain
  fingerprint), converter (`NIF_CONVERTER_REVISION` + Blender executable
  identity), physics (physics/collision pipeline revision), and preparation
  pipeline (prepare-code revision constant).
- **F49.2** Batch runs (fresh, resume, `--retry-failed`) validate recorded
  fingerprints before skipping a cell as already-prepared: any stale
  fingerprint re-prepares exactly that cell. Deterministic lines:
  `fingerprint: cell <formid> stale (<component>)` /
  `fingerprint: <n> cells valid, <m> stale`.
- **F49.3** Report mode (`prepare --check-fingerprints`, no work performed)
  lists per-cell fingerprint status against the current toolchain, exit code
  reflects staleness — the "report command" from the issue.
- **F49.4** Legacy manifest entries without fingerprints count as stale, not
  as errors.
- **F49.5** `features/fingerprints.feature`: pure fingerprint
  record/compare/invalidate logic on synthetic manifests.

### Tests before code

- **T49.1** Round-trip: completed cell records all four fingerprints.
- **T49.2** Each fingerprint component, changed alone, invalidates the cell;
  unchanged set skips it (four cases).
- **T49.3** Mixed manifest: exactly the stale subset is re-queued; the
  summary line counts match.
- **T49.4** Legacy (fingerprint-less) entry ⇒ stale.
- **T49.5** Cucumber scenarios for T49.2–T49.4 shapes.

## Issue #55 — First-reveal amortization

Wave-2 acceptance measured the first-ever reveal of Vault101d
(1,371 placements) at 84 ms while revisits hold 22–33 ms; A2 staggered
spawning already removed the 130 ms preload-spawn spike, and A4 ruled out
below-world pre-warm stashing. The remaining cost is first-render work when
1,200+ entities flip visible in one frame.

### Feature list

- **F55.1** Instrument the reveal: one log line per swap that splits the
  reveal frame — `reveal <formid> entities=<n> chunks=<c> visflip_ms=<a>
  frame_ms=<b>` — so visibility/propagation cost is separable from
  render-prep cost in acceptance data.
- **F55.2** Bounded chunked reveal (pure policy + thin system, the A2
  pattern): a preloaded cell's entities flip visible in bounded chunks over
  at most a few frames instead of all at once; budget is a constant with the
  same `..._PER_FRAME` convention. The chunk containing the arrival door /
  player position reveals first.
- **F55.3** The strict same-frame swap contract relaxes only for the tail:
  the swap (active-cell switch, teleport, telemetry line) still happens on
  the activation frame; remaining chunks complete within the next 2–3
  frames. Fallback (non-preloaded) path unchanged.
- **F55.4** If measurement shows render/pipeline prep (not visibility count)
  dominates, stop and report on the issue before building anything further —
  the next candidate (pipeline pre-warm) is a different design.
- **F55.5** `features/first_reveal.feature`: chunk planning is pure
  (budget partitioning, door-proximity ordering, already-small cells reveal
  in one chunk).

### Tests before code

- **T55.1** Chunk plan: n entities, budget b ⇒ ceil(n/b) chunks, stable
  order, door-nearest chunk first.
- **T55.2** Small cell (n ≤ b) reveals in exactly one chunk (wave-2 behavior
  preserved bit-for-bit).
- **T55.3** Bevy-side test: revealing across frames leaves no entity
  permanently hidden; interrupt (immediate second swap) doesn't strand
  hidden entities.
- **T55.4** Cucumber scenarios for T55.1–T55.2 shapes.

### Acceptance bar

No frame > 33 ms on any preloaded hop of the Vault101a→b→d→b→a chain,
**first visits included**, measured on a cool machine (collision-cook canary
~10 ms) against the same `swap … max_frame_ms=` telemetry as wave 2.

## Orchestrator: real-data acceptance (after 3a merges)

1. `cargo run-dev -- prepare Vault101a Vault101b Vault101d 00017f37` — the
   revision bump makes #49 report every cell stale; the full re-prepare
   exercises F49.2 on real data. Rerun immediately: all cells valid/skipped.
2. `prepare --check-fingerprints` shows a clean report; flip a recorded
   fingerprint by hand and see exactly one cell go stale.
3. Launch the viewer with the agent bridge; drive the Vault101a→b→d→b→a
   chain over BRP. Evidence per hop: `scene_snapshot` before/after, swap +
   reveal telemetry lines, `door anim` lines showing Open playback and the
   deferred travel write; non-travel door (e.g. a container/plain door in
   Vault101a) visibly animates — `capture_viewport` is black when occluded,
   so ask the user to watch a run for visual confirmation.
4. Frame bar: no frame > 33 ms on any preloaded hop, first visits included.
5. Comment measured results on #57/#49/#55, tick epic #5 checklist items,
   PR `m2-wave3` → master with `Closes` footers, update the
   `docs/plans/README.md` table row, then kick off wave 3b.
