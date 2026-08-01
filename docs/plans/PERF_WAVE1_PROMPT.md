# PERF wave 1 prompt — verified idle-frame quick wins

The user pasted a detailed external static review of the repository
(architecture assessment + ranked performance/modularity recommendations) and
asked, implicitly, for the orchestrator verdict on what to do with it. The
orchestrator verified every major claim against the code on `master` before
acting, per house convention.

Verified findings and chosen scope (user picked "quick-wins wave first",
filed as four standalone issues — no epic — per user decision):

- **#267** — Realtime-shadow disabled path (`src/viewer/lighting.rs:91–95`)
  unconditionally writes `shadow_maps_enabled = false` on every candidate and
  clears the selected-light resource every frame. Disabled is the default
  (`--realtime-shadows` opt-in), so this is default-path change-detection spam.
- **#268** — `update_debug_info_hud` (`src/viewer/diagnostics.rs:369`) is an
  exclusive `&mut World` system that builds and writes its `Text` every frame
  even while off; three player HUD systems
  (`src/viewer/player/mod.rs:469–471`) follow the same rewrite-every-frame
  pattern.
- **#270** — `apply_ao_strength` (`src/viewer/controls.rs:496`) counts the
  full mesh query every frame on a count-based sentinel that misses
  remove+add pairs; `configure_glow_cards` (`src/viewer/scene.rs:1404`) has
  the same sentinel plus a stale-prone `Local<HashSet<Entity>>` and a
  per-candidate lowercase allocation.
- **#269** — the metallic/dielectric-specular/roughness clamps
  (`src/viewer/controls.rs:692–820`) each own a baseline map, lack change
  gating, rescan all of `Assets<StandardMaterial>` while engaged, and
  serialize against each other via `ResMut`.

Explicitly **out of scope** for this wave (verified but deferred — each is a
later-wave candidate):

- Manifest/placement authority redesign (review item 4): `PlacementRoot` owns
  full `PreparedPlacement` clones, `LoadedSceneManifest` is an owned copy, and
  `ResidentCell` deep-clones per cell. This re-cuts the documented #163
  `setlock` in-place-mutation seam and needs its own design wave.
- Prepare staging-lock concurrency (item 5): `asset_stage_lock` serialization
  is self-documented (orchestrator.rs F48.4 comment); per-job staging +
  atomic publish is a prepare-side wave with resume-semantics testing.
- Runtime profile split (item 6): `WinitSettings::continuous()` is the
  deliberate #180 fix; only profile-shaped changes are compatible.
- Module splits (scene.rs 1492, orchestrator.rs 3611 lines, plugin bundles):
  follow after ownership boundaries are fixed by the deferred items.

## Review claims that did **not** survive verification (do not implement these)

- "The metallic/dielectric/roughness systems are the clearest runtime CPU
  issue": in their default states they early-return cheaply; the per-frame
  rescans only occur while a clamp is engaged. The consolidation (#269)
  stands as architecture + engaged-path cost, not as default-path cost.
- "The HUD updater": there were actually **four** such systems, not one.
- Item 6's generic "vsync + reactive" suggestion would regress issue #180's
  deliberate continuous-mode fix.
