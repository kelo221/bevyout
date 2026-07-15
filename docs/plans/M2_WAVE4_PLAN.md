# M2 Wave 4 — Failure Recovery, State Persistence, Reveal Pre-warm

Epic: [#5 — M2 Bulk preparation and connected interiors](https://github.com/kelo221/bevyout/issues/5)
Kickoff: [M2_WAVE4_PROMPT.md](M2_WAVE4_PROMPT.md)

Goal: close gate #6. A failed fallback load returns the player safely to the
source cell with a visible notice (#59); dynamic, inventory, and
enable-parent state survive revisits and a viewer restart through the
`ActiveSaveState` seam (#60 + #61); the first reveal of a large preloaded
cell meets the ≤33 ms frame bar via render pre-warm (#55). #62 lands as an
isolated parallel P2; #63 is scoped after the gate items are green.

| Issue | Scope | Executor |
|-------|-------|----------|
| [#59](https://github.com/kelo221/bevyout/issues/59) fades, cancellation, failure recovery | fallback path only; instant swaps stay fade-free | agent A (Sonnet, worktree) |
| [#60](https://github.com/kelo221/bevyout/issues/60)+[#61](https://github.com/kelo221/bevyout/issues/61) state persistence | one agent, the save-layer seam is its internal boundary | agent B (Sonnet, worktree) |
| [#62](https://github.com/kelo221/bevyout/issues/62) resumable `bake --all-interiors` | isolated in `src/vsa/bake/**` | agent C (Sonnet, worktree) |
| [#55](https://github.com/kelo221/bevyout/issues/55) render pre-warm | profile-first against wave-3 reveal telemetry; needs the real corpus + live viewer | orchestrator, main session |
| [#63](https://github.com/kelo221/bevyout/issues/63) ownership/asset barriers/lifecycle | scoped minimally after A+B merge (their unload paths come first) | orchestrator, after 1–3 green |

**Ordering rule inside every issue (repo convention): feature list fixed below →
Cucumber feature + tests written first → implementation makes them green.**

File-ownership boundaries (shared seam: `tests/features.rs`, append-only
World fields + delimited step sections; one new `features/*.feature` per
issue). #59 and #60/#61 both touch `swap.rs` — split by region: agent A owns
`evaluate_door_travel_requests`, the fallback/resolution systems, and the
loading-overlay UI; agent B owns `apply_save_state_to_cell` (may move it out
into its module) and adds capture calls in `activate_resident_cell`. Neither
touches the other's regions.

- **A (#59):** `src/viewer/world/swap.rs` (its regions), `swap_policy.rs`,
  `src/app_state/**` (only if a fade state is needed),
  `src/viewer/interaction.rs` (only to expose the notice API),
  `features/loading_fallback.feature`.
- **B (#60/#61):** new `src/viewer/world/persist.rs` + pure
  `persist_policy.rs`, `src/save/mod.rs` (player-inventory record),
  `src/viewer/console.rs` (`save` command), `src/cli.rs` (`--save-slot` on
  view), `src/viewer/world/preload.rs` (one evict-hook call), its `swap.rs`
  regions, `features/state_persistence.feature`.
- **C (#62):** `src/vsa/bake/**`, `src/cli.rs` (bake args only),
  `features/resumable_bake.feature`.
- **Orchestrator (#55):** `src/viewer/world/reveal.rs`, `reveal_policy.rs`,
  `preload.rs`, any new pre-warm module.

## Issue #59 — Loading-fallback fades, cancellation, failure recovery

Today (`swap.rs`): a fallback shows a hard-cut black overlay; a failed parse
already resolves `ReturnToSource` but only logs a `warn!`; a second travel
request mid-swap is ignored with a `warn!`; there is no player-facing notice
and no way to back out.

### Feature list

- **F59.1** Pure fallback-lifecycle policy (`swap_policy.rs`): given an
  in-flight fallback and an input (destination ready, parse failed, player
  cancel, superseding travel request), decide the outcome (proceed, return
  to source + notice, cancel + notice, supersede = cancel old then start
  new). Instant swaps never enter this lifecycle.
- **F59.2** Overlay fades: the loading overlay's background alpha animates
  in (~0.25 s) on `GameplayModal::Loading` enter and out on exit, driven by
  a pure easing/progress function. Instant swaps never show it.
- **F59.3** Cancellation: Esc while the loading overlay is up cancels the
  in-flight fallback cleanly — pending state cleared, modal back to `None`,
  player untouched at the source door, notice shown. A superseding
  `DoorTravelRequested` replaces the pending fallback instead of being
  dropped.
- **F59.4** Failure recovery: `ReturnToSource` shows a visible notice
  ("Loading failed — returned to <source>") through the existing
  interaction-notice HUD (expose a `pub(crate)` API or message from
  `interaction.rs`), modal returns to `None`, never a stuck loading screen.
- **F59.5** `features/loading_fallback.feature`: the pure lifecycle policy
  (F59.1) and fade-progress function (F59.2).

Non-goal: a load-timeout watchdog — the parse task is a bounded file read;
failure surfaces as `PreloadParseFailed` already.

### Tests before code

- **T59.1** Lifecycle policy: each input in each state maps to exactly the
  outcome above; superseding cancels then restarts.
- **T59.2** Fade progress function: monotonic 0→1 over the duration,
  clamped, symmetric out-fade.
- **T59.3** Bevy-side: a failed fallback leaves the player entity's
  transform untouched, modal `None`, and queues a notice; Esc during
  `Loading` does the same minus the failure wording.
- **T59.4** Cucumber scenarios for T59.1–T59.2 shapes.

## Issues #60 + #61 — State persistence through the save layer

Today: `src/save/` is a complete, tested std-only format
(`PersistentReferenceDelta` already carries enabled/deleted/activated/lock/
enable-root/transform/inventory/body; `SaveStore` writes atomically with a
backup slot). `ActiveSaveState` is a viewer resource nothing inserts;
`apply_save_state_to_cell` applies only enabled/deleted/transform, only on
swap activation (not at startup). Nothing captures.

### Feature list

- **F60.1** Pure `persist_policy.rs`: (a) effective-enabled resolution —
  manifest `initially_enabled` + `enable_parent` chains + save deltas
  (including `enable_root_form_id`) → per-reference visibility; (b) apply
  planning — merged delta application (visibility, transform, open/activated
  state, body state); (c) capture diffing — manifest baseline vs current
  runtime snapshot (pose, velocities, open set, taken pickups) → minimal
  delta set (unchanged references produce no delta).
- **F60.2** Apply on every load path: swap activation (both kinds), preload
  reveal (via the same swap-activation call site), and viewer startup for
  the launch cell. Replaces the enabled/deleted/transform-only
  `apply_save_state_to_cell`. Restored dynamic bodies get their saved
  pose/velocity when their staggered collider build runs (not a pristine
  respawn).
- **F61.1** Capture on the way out: swapping away and preload eviction both
  snapshot the departing cell (dynamic body poses/velocities from
  `CollisionWorld`, door/container open state from `InteractionState`,
  picked-up pickups as deleted) into `ActiveSaveState`.
- **F60.3** Save/load flow: console `save <slot>` captures the active cell
  then writes the slot via `SaveStore` (content fingerprint + plugins from
  the manifest); `view --save-slot <slot>` loads it at startup, inserts
  `ActiveSaveState`, and applies it to the launch cell. Deterministic
  console response naming the path.
- **F60.4** Player inventory survives restart (picked-up keys must still
  open locked doors after a reload): a new optional player record in the
  save format (unknown-record forward compatibility is already tested).
- **F60.5** `features/state_persistence.feature`: the pure policy (F60.1)
  on synthetic manifests/deltas.

### Tests before code

- **T60.1** Enable-parent chains: parent disabled ⇒ child hidden; opposite
  flag honored if the manifest models it; delta overrides baseline.
- **T60.2** Capture diff: moved dynamic body ⇒ transform+body delta; opened
  container ⇒ activated delta; taken pickup ⇒ deleted; untouched reference ⇒
  no delta.
- **T60.3** Round trip: capture → encode → decode → apply reproduces the
  runtime snapshot (pure level).
- **T60.4** Bevy-side: swap away and back restores a moved placement's
  transform and hides a taken pickup; startup with `--save-slot` applies to
  the launch cell.
- **T60.5** Cucumber scenarios for T60.1–T60.3 shapes.

## Issue #62 — Resumable `bake --all-interiors`

- **F62.1** `bake --all-interiors` walks the cell catalogue exactly like
  `prepare --all-interiors`, reusing the wave-2 job-manifest machinery
  (resume after interruption, `--retry-failed`, failure summary lines) and
  #49's fingerprint records to skip already-baked, still-valid cells.
- **F62.2** `features/resumable_bake.feature`: pure job-selection/skip logic
  on synthetic manifests.
- **T62.1–T62.3** resume skips completed work; retry-failed requeues exactly
  the failures; stale fingerprint re-bakes exactly that cell.

## Issue #55 — Render pre-warm (orchestrator, profile-first)

Wave-3 baseline (do not re-derive): Vault101d first reveal ~36–39 ms warm
(bar ≤33), `visflip_ms` ~0.1 so the cost is render preparation, not
visibility count; smaller chunks measure worse (A9); below-world stashing is
disproven (A4); first run after a corpus rebuild is cold-cache garbage —
measure the second run.

1. Profile the reveal frame (`--trace-seconds` capture during an a→b→d hop)
   to split extraction / specialization / batching cost.
2. Design the smallest pre-warm that moves the dominant cost off the reveal
   frame (candidate: make preloaded-but-hidden content reach the render
   world before the swap — e.g. a bounded per-frame warm pass while the cell
   is resident-hidden; exact mechanism decided by the profile).
3. Measure against the same `reveal`/`swap max_frame_ms` lines, second run,
   cool machine (11 ms cook canary). If the bar still isn't met, report the
   profile data on #55 and stop rather than stacking speculative mechanisms.

## Issue #63 — scoped after A+B merge

The gate does not require it; #60/#61 land the unload-half capture hooks.
After they merge, scope the minimum: formalize the lifecycle states the
swap/preload/reveal systems imply and add an asset-barrier regression test
(eviction must not unload assets a still-resident cell shares — Bevy
handle refcounting is expected to already guarantee this; the test pins it).
If that exceeds the wave's budget, re-defer with a comment on #63.

## Shipped amendments (found during implementation/acceptance)

- **A12 — #55 was a wave-3 regression, not a missing pre-warm** (commit
  `d48a77a`): the reveal-frame profile (10 GB `bevy/trace_chrome` capture)
  showed the two animated vault-tileset GLBs re-running their glTF loader
  16,217 times in ~10 s. `resolve_pending_animation_discovery` dropped its
  root `Gltf` handle after building the clip graph; the freed root made
  every later rediscovery `load::<Gltf>` re-run the loader, firing
  `Modified` for every subasset → respawning every scene instance (wave 3's
  A8 hazard) → re-`Added` players → more discoveries, while the render
  thread re-uploaded the same meshes/images (~15 ms nearly every frame).
  Pipeline/shader compilation was ruled out. Also explains A9's
  smaller-chunks-measure-worse paradox. Fix: retain the handle in
  `AnimatedPlacement`. No pre-warm mechanism was built (F-plan step 2
  became unnecessary); the pure chunked reveal from wave 3 stays as-is.
- **A13 — console `activate` toggles containers** (`interaction::
  scripted_container_toggle`): the #60/#61 gate walk-through runs over the
  agent bridge, but `activate` was door-only, so container open-state
  persistence could not be driven on real data. Containers now route
  through the same open-state/clip/sound/notice path as player activation;
  the console responds `{opened: bool}`.
- **A14 — the superseded #52 save-application seam was deleted at merge**
  (`swap_policy::apply_persistent_cell_state`, its delta types, its tests,
  and the four `instant_swap.feature` scenarios that exercised it):
  `persist_policy.rs` owns application now and
  `state_persistence.feature` covers every removed scenario shape.
- **A15 — post-fix frame-bar tuning, and what was rejected**: with the
  reload churn gone (A12), the remaining >33 ms frames split into two
  phenomena. (1) Reveal/collider overlap on the largest cell:
  `PRELOAD_SPAWN_BUDGET_PER_FRAME` 128→64 and
  `COLLIDER_BUILD_BUDGET_PER_FRAME` 64→48 bring every transition/reveal
  frame of the a→b→d→b→a chain to ≤32 ms on a cool machine (Vault101d
  first reveal 31.6 ms, was 84 in wave 2 and ~36 in wave 3).
  (2) An intermittent 90–163 ms frame while a large neighbor preloads in
  the background: an atomic GPU upload of big uncompressed textures.
  `RenderAssetBytesPerFrame` (16 MB) was tried against it and REVERTED —
  deferring uploads starved the next reveal instead (118–126 ms reveals,
  3/3 runs). No runtime knob splits an atomic upload; the fix is
  GPU-compressed textures at prepare time (BC7/KTX2, like the
  point-shadow artifacts). Follow-up to be filed; not gate-blocking —
  transition frames themselves meet the bar.
- **Flagged for #63 during #60/#61** (agent finding, confirmed by reading
  `queue_collider_build`): collider construction has no per-cell ownership
  or teardown — every swap re-queues the destination's full placement list,
  so revisits duplicate static shapes and keyframed bindings on the shared
  static body, and a departed cell's colliders persist indefinitely. The
  #60/#61 dynamic-body guard fixed only the dynamic case. This is #63's
  "explicit ownership / unload releases" criterion; see that issue.

## Orchestrator: gates and real-data acceptance

1. Merge A/B/C branches into `m2-wave4`; resolve `tests/features.rs` and
   `swap.rs`/`cli.rs` seams; `cargo fmt --check`, `clippy --all-targets --
   -D warnings`, `cargo test`, representative `cargo run-dev` commands.
2. Gate walk-through on real data (Fallout3.esm GOTY, Vault 101): push/move
   a dynamic object and open a container in Vault101b, run a→b→d→b→a,
   revisit b — state persists; `save` + restart with `--save-slot` — still
   holds (#60/#61).
3. Rename a destination `scene.ron`, activate its door: fade in, notice,
   player back at the source door, modal closed; restore the file (#59).
   Esc mid-fallback cancels cleanly.
4. Frame bar: all preloaded hops ≤33 ms including first visits, second run,
   cool machine (#55).
5. `bake --all-interiors` resume/retry on the prepared corpus (#62).
6. Comment measured results on each issue; amend this plan (never rewrite);
   one PR per convention with `Closes #NN` footers; update
   `docs/plans/README.md`; tick epic #5 only for criteria that hold on real
   data; update session memory.
