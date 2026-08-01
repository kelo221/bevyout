# PERF wave 1 plan — verified idle-frame quick wins

Four standalone issues (no epic, per user decision): #267, #268, #269, #270.
Integration branch: `perf-wave1`.

## Fixed feature lists

### #267 — realtime-shadow disabled path is zero-write

1. `apply_realtime_shadow_light` early-returns while disabled *and* already
   clean (`!settings.is_changed() && selected.0.is_none()`); the disable
   transition clears candidates with conditional writes only.
2. Enabled path preserves exactly-one-camera-relevant-startup-light selection
   semantics; rescanning may stay per-frame only if a test documents it as
   required (light movement/intensity changes while enabled).

### #268 — diagnostics HUDs are change-driven

1. `update_debug_info_hud` is no longer an exclusive `&mut World` system.
2. Disabled `Text` is written once per toggle transition, not per frame; the
   enabled live-data refresh runs on a bounded timer (5–10 Hz); unchanged
   composed strings never replace `Text`.
3. Same no-op-frame guarantee for `update_collider_debug_hud`,
   `update_step_debug_hud`, `update_player_debug_hud`.
4. `tdi`/`tdt` console behavior unchanged (#151 pattern preserved).

### #270 — scene classification is revision/event-driven

1. `apply_ao_strength` stops counting the full mesh query per frame;
   eligibility is tracked from entity/asset events (or computed at scene
   load); strength changes reprocess only the cached eligible set; baselines
   are dropped for removed assets.
2. `configure_glow_cards` drops the count sentinel and the
   `Local<HashSet<Entity>>`: **every** inspected entity gets a marker
   component (markers despawn with entities); the name check is
   allocation-free (prefix slice + `eq_ignore_ascii_case`).
3. `apply_reflection_probe_settings` and `apply_horizontal_fov` gain
   settings-change/`Added` gates (writes are already conditional).
4. The remove+add count-coincidence blind spot is closed and regression-
   covered (new glow card still hidden; new AO mesh still processed).

### #269 — one incremental material-clamp policy owner

1. A single clamp-policy resource owns metallic gate, dielectric-specular
   gate, and roughness scale, with one baseline map and one mutation per
   material per settings change.
2. Settings-unchanged frames consume `AssetEvent<StandardMaterial>` only
   (Added/Modified processed, Removed drops baselines); full passes happen
   only on settings revision changes.
3. `setrender metallic|dielectric_specular|roughness_scale` semantics are
   unchanged: restores are value-exact, including for materials loaded while
   a clamp was engaged.
4. No parallel `ResMut<Assets<StandardMaterial>>` clamp systems remain
   (emission/unlit keep their existing gated implementations).

## Tests-first list

- shadow policy: disabled steady frame mutates nothing; enable/disable
  transitions are conditional-write only; selection still yields exactly one
  candidate (existing tests updated, not weakened);
- HUD: off-frame composes nothing, toggle writes once, enabled timer cadence,
  unchanged-string no-op (deterministic formatting tests stay pure);
- AO/glow: remove+add coincidence regression, eligible-set reprocessing on
  strength change, baseline cleanup on asset removal, allocation-free name
  check unit test;
- clamps: toggle/restore value-exact round trips, load-while-clamped,
  removed-asset baseline cleanup, steady engaged frame performs no full
  store iteration (assert via instrumented counter or asset-event-only path).

## Execution model recommendation

Current runtime is **pi on moonshotai/Kimi-K3**, which the AGENTS.md routing
table does not cover. Closest discipline is the Claude runtime: this
orchestrating session plans, merges, and evaluates; executor subagents write
all production and test code. Executors run as pi subagents on the session
model (Kimi-K3). Portability note for reruns: Claude runtime — Sonnet for
# 267/#268, Fable for #269/#270; Codex runtime — Sol High throughout.

## Sequencing

- **Parallel (isolated worktrees off `perf-wave1`):** #267
  (`src/viewer/lighting.rs` + its tests) and #268
  (`src/viewer/diagnostics.rs`, `src/viewer/player/mod.rs` + HUD tests) —
  disjoint file ownership.
- **Sequential on `perf-wave1`** (shared `controls.rs` seam, M4-wave-4
  precedent): #270 first (AO/glow + small gates; also `scene.rs`), then
  #269 (clamp consolidation) on top of it.
- Shared merge seam: `tests/features.rs` registrations are appended per
  issue in delimited sections; orchestrator resolves.

## Gates and acceptance

- `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`,
  `cargo test`, `cargo check-dev`.
- Real-data: `cargo run-dev -- view --manifest .bevyout/cache/scenes/<formid>/scene.ron --agent-bridge`
  with console `setrender` toggles and `tdi` exercised per
  `docs/plans/PERF_WAVE1_MANUAL.md` (written before the PR).

## Shipped amendments

(none yet)
