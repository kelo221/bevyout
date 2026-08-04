# M4 static NPC FaceGen reconstruction (#109)

## Execution model

**Codex runtime recommendation: Luna X-High.** This slice crosses pure core
contracts, ESM4 inheritance, bounded binary parsing, native NIF scene
deformation, texture staging, cache identity, structured console output, and
real-data visual verification. Keep the work on one `m4-facegen` branch.

The branch is based directly on current `master`. Its first commit,
`dc865792` (`Init`), is pre-existing and contains the skin/hair shader changes
that must remain part of the final PR.

## Fixed feature list

### 1. Pure FaceGen contract and coefficient resolution

- Add an engine-independent contract for symmetric geometry, asymmetric
  geometry, and symmetric texture coefficients.
- Decode only exactly 200/120/200-byte little-endian finite-f32 payloads.
  Missing data is `NotAuthored`; any other length or non-finite value is a
  typed unsupported-layout diagnostic.
- Resolve race/sex defaults plus NPC values through the existing `USE_TRAITS`
  inheritance seam. Preserve raw bytes and resolved coefficients in the
  catalog/blueprint rather than re-reading records during native conversion.
- Make `FaceGenAvailability::Compatible` reachable only after the complete
  coefficient and asset contract is verified, and report
  `FaceGenPolicy::Authored` for that path.

### 2. Clean-room EGM/EGT preparation

- Add a dedicated preparation module with strict, bounded parsers for
  `FREGM002` and `FREGT003`.
- Resolve companions case-insensitively from loose data before BSA entries.
- Validate magic, ASCII version, reserved/header bounds, finite scales,
  positive bounded counts, exact payload length, basis counts, and selected
  NIF vertex/UV compatibility.
- Keep all extracted EGM/EGT/DDS/GLB files beneath `.bevyout`; commit only
  synthetic fixtures.

### 3. Native head reconstruction

- Before merging the selected head anchor, apply verified EGM modes using the
  resolved race/NPC geometry coefficients.
- Preserve mesh topology, UVs, colors, joints, and weights. Recompute smooth
  normals and tangent handedness from the unchanged indexed topology.
- Reject non-finite or mismatched output without mutating the actor; retain the
  existing race/sex rest-pose fallback and `missing_facegen` diagnostic.
- Synthesize only the selected head diffuse from the race/sex base texture and
  verified EGT modes, preserve alpha, clamp deterministically, and stage it
  through the existing UASTC KTX2 path. Body, hair, eyes, mouth, teeth, and
  apparel diffuse inputs are unchanged.

### 4. Cache and inspection surface

- Include FaceGen asset paths, raw coefficient bytes, race/base identity,
  EGM/EGT hashes, and an algorithm revision in actor cache identity.
- Bump `ACTOR_CATALOG_REVISION`, `NATIVE_ACTOR_CONVERTER_REVISION`, and the
  composite prepared converter revision. Ordinary `prepare` must invalidate
  stale actors without rebuilding unrelated static assets.
- Extend `actorinspect` with FaceGen policy, reconstruction fingerprint,
  geometry/texture status, and typed diagnostics.

## Tests-first order

1. Extend `features/actor_fallback.feature` with authored/compatible,
   unsupported-layout, inheritance, and safe-fallback scenarios; add every
   Cucumber step before production code.
2. Add pure decoder tests for canonical coefficients, invalid lengths,
   non-finite values, deterministic fingerprints, and race/NPC resolution.
3. Add EGM/EGT parser tests for headers, bounds, exact lengths, and rejection
   of unsupported layouts.
4. Add deformation/texture tests for weighted vertex displacement, unchanged
   topology/UVs/weights, smooth normal and tangent reconstruction, alpha
   preservation, deterministic clamping, and byte-identical repeated output.
5. Add actor-assembly tests proving success removes `MissingFaceGen`, malformed
   or missing inputs preserve it, unrelated actor parts remain unchanged, and
   revision changes invalidate actor caches only.

## Verification gates

- `cargo fmt --check`
- `cargo clippy --all-targets -- -D warnings`
- `cargo test`
- focused dynamic-link checks during implementation
- ordinary native prepares twice for both exact cell/actor pairs in
  `M4_FACEGEN_PROMPT.md`
- agent-bridge snapshots, `actorinspect` output, and comparable close-ups for
  all four reference actors
- ready PR to `master` with `Closes #109`, followed by hosted Linux/macOS/
  Windows/fmt/clippy verification and repair until green

## Stop conditions

- Do not invent a heuristic tint when head/body color mismatch is visible.
- Do not claim `FaceGenPolicy::Authored` unless at least one female and one
  male vanilla NPC pass geometry, texture, cache, structured-output, and
  visual gates.
- If real assets use a layout not covered by the canonical contract, retain
  the safe fallback, report the exact unsupported layout/evidence, and do not
  open a ready PR.

## Shipped amendments

### Review follow-up: head-only texture scope and diagnostic fallback

- Removed the original diffuse-path alias and post-assembly source-path rewrite
  from native actor conversion. FaceGen now registers only the generated head
  texture key, so another actor material sharing the authored path remains
  unchanged.
- Native conversion now emits a stable warning when a compatible FaceGen
  descriptor has no head anchor or its expected head visual was not decoded.
- Bumped the native actor converter to v18 and the composite prepared converter
  to the matching `actor-assembly-v18-facegen-head-only` segment.
- Added focused regression tests covering shared diffuse paths and both missing
  FaceGen head prerequisites.

- The real FO3 companion layout uses TRI base vertices plus expression vertices:
  `headfemale.tri` reports 1,211 base vertices and 1,078 expression vertices,
  while its EGM reports 2,289 vertices. The bounded TRI header validation now
  checks both values rather than treating the NIF vertex count as the EGM
  count.
- Real EGT files are 256x256 while the head diffuse is 1024x1024. The native
  path accepts an integer-compatible lower-resolution EGT and bilinearly
  expands each signed RGB mode during deterministic synthesis; it still
  rejects incompatible dimensions.
- `CURRENT_PREPARE_REVISION` is now `prepare-v25-m4-static-facegen-reconstruction`
  and `PREPARE_PIPELINE_REVISION` is v9 so the new serialized FaceGen fields
  cannot be read from stale scene manifests.
- Final real-data prepares completed for both selectors. The warm second
  prepares reused all 507 Megaton assets and all 605 Super-Duper Mart assets
  with zero conversion jobs. Target GLBs contain
  `__bevyout_facegen/*.png` plus unchanged body, hair, eyes, mouth/teeth, and
  apparel texture identities.
- Live bridge evidence passed for Super-Duper Mart `00041600`, Megaton Moira
  `0002d2bc`, and Megaton male mercenary `0001ff18`: each reports
  `facegen.status=Applied`, geometry/texture `Applied`, empty FaceGen
  diagnostics, and an idle animation; the mercenary also reports an attached
  right-hand weapon. Super-Duper Mart `00041610` is an initially disabled
  enable-parent reference and is not instantiated in the live placement set,
  so `actorinspect 00041610` returns the expected `reference_not_found`.
  Its prepared descendant assemblies and FaceGen assets remain validated;
  the Megaton mercenary supplies the live male geometry/texture/animation/
  weapon gate.

### Visual acceptance follow-up: native FaceGen texture row origin

- A close-up audit found the synthesized EGT deltas vertically inverted on the
  native Fallout texture boundary, producing a bright mouth-band artifact even
  though the FaceGen status fields were `Applied`.
- Native EGT sampling now maps decoded image rows through the bottom-origin
  Fallout texture V convention before bilinear expansion. The actor converter
  is v19 and the composite prepared converter carries the matching revision.
- Added a distinct-row regression test and re-prepared Megaton. Moira's
  regenerated actor asset is `AuthoredExact` with FaceGen geometry and texture
  both `Applied`, empty diagnostics, and preserved idle animation.

### Visual acceptance follow-up: FaceGen hair fit

- The close-up hair audit confirmed that FaceGen moved the head surface toward
  an unchanged hair cap by up to roughly 9 mm in actor space; the hair vertex
  buffers themselves were unchanged, and the hair texture retains intentional
  alpha cutouts.
- Native actor preparation now builds a shared-skeleton head displacement field
  and applies only outward, scalp-facing corrections to hair parts, with a
  small deterministic clearance. Head, eyes, mouth, teeth, apparel, topology,
  and UVs remain unchanged.
- Bumped the native actor converter to v20 and added synthetic inward/outward
  hair-fit regression coverage. Real-data regeneration and live close-up
  verification remain required before the PR handoff.
