# M4 wave 10 — actor KF compatibility and animation zoo (#104)

Single-issue wave under epic #9 on the existing `Animations` branch. The
branch is merged with master after PR #166 before continuation changes.

**Execution model recommendation (Codex runtime): Sol X-High.** The wave
crosses ESM4 record decoding, archive discovery, Blender/NIFTools compatibility,
content-addressed prepared assets, Bevy animation target binding, cache
revisions, and real Fallout data. The higher-reasoning execution model is
appropriate for keeping those contracts aligned.

## Recovered prototype baseline

- NPC_ and CREA KFFZ data decode through a pure `bevyout-core` contract and
  participate in model-animation template inheritance.
- Preparation discovers loose/BSA KF files, deduplicates skeleton-compatible
  source sets, fingerprints source bytes, and serializes a versioned per-cell
  actor-animation catalog.
- A Blender 5/NIFTools compatibility script imports external KF files one at a
  time, records per-clip failures and missing targets, and exports reusable
  animation-only GLB packs.
- `animation-zoo` loads one prepared actor plus its clip pack, restores bind
  pose between clips, and exposes deterministic keyboard and agent-bridge
  controls.
- The recovered branch passed its 43-feature/342-scenario Cucumber baseline,
  but it had not passed all-target clippy, been reconciled with current master,
  or run against real humanoid/creature assets.

## Fixed feature list

### 1. Deterministic source and compatibility catalog

- Keep KFFZ decoding, template inheritance, relative-path resolution,
  case-insensitive sort/deduplication, skeleton/source fingerprints, stable
  clip aliases, and animation-set reuse in the pure core contract.
- Preserve missing, malformed, incompatible, missing-target, and conversion
  failure diagnostics. No source clip may disappear silently.
- Keep the actor-animation catalog content-addressed and linked from
  `PreparedSceneManifest`. Bump every affected prepared revision whenever its
  serialized shape or meaning changes.

### 2. Explicit clip-pack preparation boundary

- Native NIF/actor assembly remains the default production path and ordinary
  `prepare` must not resolve Blender.
- External-KF clip-pack generation is a separate explicit compatibility
  opt-in. It may invoke Blender/NIFTools while the scene and actor appearance
  continue to use native conversion; it must not force the Windows-only
  PyNifly whole-scene comparison backend.
- Cache identity includes converter policy, exact skeleton/KF bytes, normalized
  source paths, and stable clip names. Warm preparation validates and reuses
  both report and GLB; rebuild affects only the requested asset family.
- A disabled, unavailable, or failed compatibility backend produces a stable
  diagnostic and accurate counts rather than labelling unconverted clips as a
  successful pack.

### 3. Isolated animation-zoo acceptance surface

- Select one NPC or creature reference from an exact prepared manifest, verify
  the catalog hash/revision and pack hash, bind pack target paths to the
  appearance hierarchy, and fail clearly on missing/mismatched targets.
- Cycle every successfully converted clip; restore bind pose on clip changes;
  support pause/resume, previous/next, restart, looping, and bounded speed.
- Keep the deterministic `bevyout.animation_zoo_probe` and
  `bevyout.animation_zoo_control` bridge methods so acceptance can prove state
  changes without relying on an occluded macOS viewport.

### 4. Compatibility matrix and parser decision

- Record real conversion/playback results for at least one humanoid and one
  creature clip set, including source counts, successes/failures, missing
  targets, output size, conversion time, cache reuse, and viewer playback.
- Reuse M2 #57 as the door-controller baseline and rerun a representative door
  only if current converter changes could regress it.
- Exercise a representative weapon/controller asset through the existing
  conversion/audit surface and record whether its controllers and attachment
  nodes survive. Do not invent a weapon gameplay animation system in this
  spike.
- State the pass/fail threshold and whether conditional native-parser issue
  #105 is required. Failed classes remain explicit evidence, not hidden
  fallbacks.

## Tests first

1. Preserve `features/actor_animation_catalog.feature`; add scenarios before
   behavior changes for explicit clip-pack opt-in, disabled-backend reporting,
   and warm-cache decisions.
2. Add CLI/config tests that prove native scene conversion and external-KF
   compatibility conversion are independent selections.
3. Keep unit coverage for pack fingerprints, GLB names/channels/targets/times,
   catalog serialization, playback controls, bind-pose restoration, and a
   minimal-App animated target.
4. Add a focused regression test for every failure discovered during real-data
   preparation or live zoo playback before fixing it.

## Gates and real-data acceptance

- `cargo fmt --check`
- `cargo clippy --all-targets -- -D warnings`
- `cargo test`
- `cargo run-dev -- prepare SuperDuperMart --converter native` as the ordinary
  no-Blender path, followed by the explicit actor-animation compatibility
  opt-in for the same cell.
- Run `animation-zoo` with the agent bridge for raider `00041600` or another
  catalog-backed humanoid and Protectron `0006d921` or another catalog-backed
  creature. Probe initial playback, next/restart, pause/resume, loop, and speed.
- Re-run preparation warm and require deterministic catalog/pack reuse.
- Record exact FormIDs, clip coverage, diagnostics, timing/output-size/cache
  counts, and any unverified visual behavior in shipped amendments, the manual
  script, and issue #104 before a PR.

## Non-goals

- The gameplay animation state machine and idle/walk/run/turn/equip/unequip
  transition integration (#106).
- Native attributed KF parsing unless the measured matrix triggers #105.
- Combat, hit, death, reload, animation-event audio, or script dispatch.
- Exact FaceGen reconstruction, body assembly, or a second actor item authority.

## Shipped amendments

- **A1 — Recovered branch reconciled with current master.** The stopped
  prototype was based before M4 waves 6–8. It was merged with current master,
  retaining schema 18's corpse/nested-actor fields and the append-only
  `tests/features.rs` seam. The actor appearance converter revisions were
  restored to master's v12/v31 identities; external-KF cache identity now
  lives only in its own catalog/converter revisions.
- **A2 — Clip-pack conversion is an independent opt-in.** The prototype tied
  KF conversion to `--converter blender`, which also selected the Windows-only
  PyNifly whole-scene path and made ordinary native preparation report every
  unconverted KF as failed. Shipped `--actor-animation-converter
  disabled|blender` (disabled by default), a distinct job fingerprint, an
  informational `NotConverted` state, and a pure warm-cache decision. Disabled
  cells neither resolve Blender nor acquire the shared Blender lock.
- **A3 — Normalized runtime contract completed from source metadata.** The
  v3 catalog / v6 converter report now retains authored sequence name,
  start/end/frequency/phase, clamp/loop/reverse mode, the explicit
  `PreserveAuthored` root-motion policy and accumulation root, required versus
  animated/missing targets, controller/interpolator classes, and every
  `NiTextKeyExtraData` key. The animation-zoo HUD and bridge probe expose the
  same fields. Real `1hpequip.kf` proves `Equip`, 0.0–0.3667 s, clamp,
  accumulation root `Bip01`, transform/material/visibility controllers,
  transform/bool/point interpolators, and five keys including `Attach`.
- **A4 — Creature path policy corrected and a playable creature proved.** The
  Super-Duper Mart Protectron's 20 `KFFZ` values are filenames such as
  `1HPIdle.kf`. OpenMW's ESM4 contract identifies these as filenames whose
  directory comes from the actor model. None exist beside the Protectron
  model, so the catalog honestly retains 20 `missing_kf` diagnostics and no
  creature pack. A tempting global-basename fallback to humanoid `_male` KFs
  was rejected because it would bind the wrong rig and contradict that source
  contract. Vault 101's Radroach supplies a present, matching creature set:
  all 16 KFs converted into a 720,424-byte pack, the zoo loaded all 16 with no
  skips, and bridge-driven pause/loop/speed/restart/next controls passed.
- **A5 — Parser decision.** Conditional native-parser issue #105 is **not
  triggered by this matrix**. NIFTools preserved every present Radroach KF and
  1,385 humanoid KFs. The 27 humanoid failures are imported actions with zero
  usable channels; the Protectron failure is absent source content. A second
  parser cannot recover missing files, and neither class currently blocks the
  later gameplay state-machine slice. #105 stays explicitly deferred unless a
  later representative actor supplies a present KF whose gate-required
  controller data NIFTools cannot preserve.
- **A6 — Model routing (recorded per AGENTS.md).** Codex runtime, direct
  execution with no subagents; recommended and used `Sol X-High` because the
  recovery crossed ESM4 semantics, Blender/NIFTools, prepared revisions,
  Bevy animation binding, and live real-data acceptance.
- **A7 — Measured matrix and pass threshold.** The spike passes when at least
  one humanoid and one non-humanoid skeleton produce playable, metadata-complete
  packs; every discovered source is ready, not requested, missing, or failed
  with a stable reason; a representative weapon preserves controller targets
  and loads through Bevy; the shipped door baseline remains green; and a warm
  prepare validates rather than rebuilds its pack. Visual weapon-controller
  triggering is explicitly outside the current pickup/gameplay surface and is
  not represented as verified playback.

  | Class | Conversion evidence | Bevy/runtime evidence | Result |
  |---|---|---|---|
  | Humanoid (`00041600`) | 1,412 source KFs; 1,385 ready; 27 explicit zero-channel failures; 154,503,936-byte pack; about 17 minutes and 6.65 GiB peak RSS for the cold v6 build | Zoo loaded 1,385, skipped 27, and exposed sequence/range/loop/root/target/controller/interpolator/text-key metadata; controls passed | Pass with bounded failures |
  | Radroach (`0005443b`) | 16/16 KFs ready; 720,424-byte pack; no conversion failures | Zoo loaded 16, skipped 0; normalized metadata and all bridge controls passed | Pass |
  | Door | Existing M2 #57 Open/Close NIF-controller path is unchanged by this actor-only slice and remains covered by the full feature/unit suite | Existing placement animation graph/playback tests remain green | Baseline pass |
  | Laser pistol | Native conversion produced a 528,512-byte GLB with 3 named direct clips, 3 channels, 10 keyframes, attachment/controller nodes, and no lossy issues | Both prepared instances loaded in Super-Duper Mart and exposed `AnimationPlayer` plus a discovered graph; no pickup weapon-animation trigger exists in this spike | Structural/load pass; trigger not in scope |

  The identical explicit Super-Duper Mart prepare then reported `packs built
  0, reused 1`; the default native prepare reported `0 ready clips, packs
  built 0, reused 0, failed clips 0` and never resolved Blender. The 147 MiB
  cold humanoid pack is therefore paid once and validated on reuse.
- **A8 — Variant coverage is recorded, including negative cases.** The real
  humanoid report contains 1,143 clamp, 267 loop, and two mixed-loop sequences;
  all 1,412 sources retain text keys (6,304 total), and 1,408 name an
  accumulation root. Observed controller families are tread-transform, float
  extra-data, geometry morpher, material color, texture transform, transform,
  and visibility. Observed interpolators are tread-transform, compressed
  B-spline float/point/transform, bool, float, point, and transform. Live
  humanoid and Radroach motion proves prepared skin/skeleton hierarchy binding.
  The synthetic report fixture separately preserves a non-zero 0.25–0.75
  source range, missing targets, and unsupported/failure diagnostics because
  every real source in this corpus starts at zero.

- **A9 — Visual playback regression and correction.** The first live humanoid
  run proved the catalog and Bevy control surface but exposed a real visual
  defect: NIFTools clip packs use Blender-space transforms and `Bip01 Calf.L`
  naming, while the native actor GLB uses Fallout-space transforms and
  `Bip01 L Calf` names. Binding those clips directly made the actor visibly
  collapse even though metadata and state probes were green. The zoo now
  canonicalizes the known side-name spelling, records source and target rest
  hierarchies, and transfers sampled source deltas through global rest space
  before writing native local transforms. The compatibility backend remains
  explicit and Blender is still not resolved by ordinary native preparation.
  Live probes and viewport captures now pass for humanoid `00041600`
  (`bound_targets=66`, 1,385 clips) and Radroach `0005443b`
  (`bound_targets=48`, 16 clips, no missing targets); the bridge probe exposes
  the bound-target count so this regression cannot hide behind catalog
  metadata again.
- **A10 — Native Nifty KF bridge (continuation).** Added an explicit
  `--actor-animation-converter native` backend. It reuses the pinned
  `native-fo3-glb` Nifty parser/GLB encoder for scene and skeleton assets, then
  supplies the external-KF skeleton hierarchy and sequence metadata that KF
  files do not carry as scene roots. The adapter also tolerates the four-byte
  Bethesda sequence tail that the current Nifty typed decoder rejects, while
  retaining authored targets, loop mode, accumulation root, and text keys.
  A real Super-Duper Mart prepare with the native backend produced 1,380 ready
  clips and 32 explicit failures (27 zero-channel clips also fail in the
  Blender comparison; five are legacy 20.0.0.4 KFs outside Nifty's FO3
  20.2.0.7 contract). The adapter decodes compact B-spline transform control
  points into deterministic GLB channels, so those clips no longer disappear.
  Native live-zoo startup loaded all 1,380 ready clips with
  `bound_targets=67`; viewport captures show a coherent humanoid pose. Native
  KF conversion remains explicit and does not silently invoke Blender; no
  external repository was modified or published by this continuation.
