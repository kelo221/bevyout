# DynamicLighting volumetric fog follow-up plan

Status: implemented and validated on the `DynamicLightsPort` working tree.

Traceability: assign a milestone issue before implementation. This document is
the fixed feature list and tests-first execution order for that issue.

## 1. Current-state verdict

Henry-style volumetric fog does not work in the current port.

- `DynamicLightConfig` has no volumetric type, radius, thickness, intensity, or
  visibility authoring fields.
- The source-compatible 112-byte `GpuDynamicLight` retains
  `volumetric_intensity` and `volumetric_visibility`, but extraction writes both
  fields as `0.0`.
- No DynamicLighting WGSL function consumes those fields and there is no
  volumetric render-graph node.
- `lighting-test` contains no volumetric source, toggle, diagnostic, or visual
  acceptance case.
- Bevy 0.19's built-in `VolumetricFog`, `FogVolume`, and `VolumetricLight`
  pipeline is available, but it only lights fog from Bevy light entities. It
  cannot see standalone `DynamicLight` entities and its physically based fog
  model is not source-compatible with Henry's post-processing shader.

The existing black `PointLight` shadow proxy must remain shadow-only. Adding
`VolumetricLight` to it would still produce the wrong fog model and would couple
visible DynamicLighting behavior back to Bevy lights.

## 2. Fixed source-compatible feature list

Port the five numeric `DynamicLightVolumetricType` values exactly:

| Value | Type | Required behavior |
|---:|---|---|
| 0 | `None` | No volumetric contribution and no volumetric-buffer entry. |
| 1 | `Sphere` | Camera-to-fragment segment proximity to a spherical fog volume. |
| 2 | `Box` | Depth-limited ray/box segment length using the light transform scale. |
| 3 | `ConeZ` | Depth-limited ray/cone segment using the source's forward-axis convention. |
| 4 | `ConeY` | Depth-limited ray/cone segment using the source's up-axis convention. |

Preserve these authored defaults and meanings from the frozen Unity source:

- volumetric type: `None`;
- radius: `4.0`;
- thickness: `1.0`;
- intensity: `0.75`;
- visibility distance: `2.0` metres;
- box size: volumetric radius multiplied by transform scale;
- cone angle: derived from the existing outer cutoff using the upstream
  expression;
- temporal modulation: the same runtime effect multiplier that drives direct
  illumination also drives volumetric intensity;
- composition: smooth the shape intersection, apply thickness, limit opacity
  by camera-to-geometry visibility, apply volumetric intensity, screen-blend
  colors, and retain the greatest encountered opacity.

Do not add volumetric shadowing, noise, physically based scattering, temporal
reprojection, or Bevy fog-volume conversion unless a separate follow-up is
approved. Those behaviors are not present in the frozen Henry implementation.

## 3. Target architecture

Keep the feature inside `src/vsa/dynamic_lighting/`:

```text
DynamicLightConfig + DynamicLightRuntime + GlobalTransform
    -> extraction of source-compatible volumetric parameters
    -> per-view frustum filtering of active volumes
    -> filtered 112-byte GpuDynamicLight buffer
    -> DynamicLighting volumetric fullscreen node
    -> depth reconstruction and ray/shape intersection in WGSL
    -> linear HDR composition
    -> tonemapping
    -> UI and bottom-right FPS counter
```

Use a second filtered buffer because the Unity implementation recycles fields
in `ShaderDynamicLight` for the volumetric pass:

- `radius_sqr` stores volumetric radius, not squared direct-light radius;
- `channel` stores the volumetric type;
- `gp_float_1` stores thickness;
- `gp_float_2`, `gp_float_3`, and `shimmer_scale` store box scale or cone data;
- `volumetric_intensity` stores authored intensity multiplied by temporal state;
- `volumetric_visibility` stores the reciprocal visibility distance.

The ordinary surface-light buffer and ABI remain unchanged. The volumetric
buffer uses the same exact 112-byte layout with different source-defined field
meanings.

Add a dedicated render-graph node after the existing DynamicLighting surface
pass and before tonemapping. It reads the current HDR target, view/depth data,
and the filtered volumetric buffer, then writes through
`ViewTarget::post_process_write`. It must skip the draw when the per-view count
is zero or volumetrics are disabled.

## 4. Tests first

### 4.1 Unity reference fixtures

Extend `reference_unity/ExportDynamicLightingGoldens.cs` and use the frozen
Unity package to export, rather than hand-calculate, reference values for:

- all five numeric volumetric types;
- default and non-default radius, thickness, intensity, visibility, and outer
  cutoff;
- non-uniform box scale;
- rotated `ConeZ` and `ConeY` sources;
- camera outside and inside each volume;
- geometry before, inside, and beyond each volume;
- zero radius, zero intensity, and `None` no-op cases;
- strobe active/inactive phases and one continuously varying effect;
- two overlapping differently colored volumes and the source screen blend.

Render small deterministic Unity targets in linear color space and export the
sampled RGBA results plus camera, transform, depth, and authoring inputs to
`tests/golden/unity_volumetric_v1.json`.

### 4.2 Cucumber contract

Add scenarios to `features/dynamic_lighting.feature` before production code:

1. The five volumetric discriminants match upstream.
2. `None`, zero radius, and zero intensity produce no active volume.
3. Defaults and serde round trips preserve every volumetric parameter.
4. Temporal effects modulate fog intensity from the shared runtime state.
5. Visibility distance preserves nearby geometry inside dense fog.
6. Non-uniform box scale and both cone orientations map correctly.
7. Multiple volumes retain the source blend and maximum-opacity rule.

Keep pure eligibility, packing, cone-parameter, and culling decisions in a
`std`/`serde` core module so Cucumber can include it without Bevy.

### 4.3 Rust and GPU tests

- Assert the existing surface-light ABI remains 112 bytes with unchanged
  offsets.
- Assert every recycled volumetric field and sentinel maps exactly.
- Assert inactive and off-frustum volumes are excluded per view.
- Assert entity removal and config changes update the filtered count.
- Assert a zero-volume view skips its fullscreen draw.
- Parse every new WGSL import and require the pipeline to reach `Ready`.
- Compare shader sample pixels for sphere, box, both cones, depth clipping,
  visibility, thickness, temporal modulation, and overlap against Unity
  fixtures.
- Prove the image changes with every Bevy `PointLight`, `VolumetricLight`, and
  `FogVolume` absent.

## 5. Implementation order

1. Add `DynamicLightVolumetricType` and
   `DynamicLightVolumetricParameters` to the pure core with upstream numeric
   values and defaults.
2. Generate and commit `unity_volumetric_v1.json`; make the new feature and
   unit tests fail against the missing port.
3. Add source-compatible volumetric packing without changing the direct-light
   GPU ABI or shadow-proxy behavior.
4. Extract active volumes and build stable, per-view, frustum-filtered lists.
5. Add `dynamic_lighting_volumetric.wgsl` with the upstream finite-line,
   ray/box, ray/cone, smoothing, thickness, visibility, and blend operations.
6. Add the isolated volumetric pipeline, bind group, diagnostics, and explicit
   render-graph ordering after surface accumulation and before tonemapping.
7. Add `DynamicLightingSettings.volumetric_enabled` and extracted/visible
   volume counts for BRP inspection. Do not add a global viewer-owned fog
   subsystem.
8. Upgrade `lighting-test` with a sphere, scaled box, rotated `ConeZ`, rotated
   `ConeY`, and a prominent strobing fog source. Add key `6` as the volumetric
   toggle and show `fog ON|OFF | volumes N` in the existing status UI.
9. Run parity, full-suite, and live visual acceptance, then document the
   shipped behavior and measured GPU cost.

## 6. Expected file ownership

Normal changes should stay within:

```text
src/vsa/dynamic_lighting/core/config.rs
src/vsa/dynamic_lighting/core/types.rs
src/vsa/dynamic_lighting/core/volumetric.rs
src/vsa/dynamic_lighting/render/gpu.rs
src/vsa/dynamic_lighting/render/extract.rs
src/vsa/dynamic_lighting/render/pipeline.rs
src/vsa/dynamic_lighting/render/volumetric.rs
src/vsa/dynamic_lighting/shaders/dynamic_lighting_volumetric.wgsl
src/vsa/dynamic_lighting/reference_unity/ExportDynamicLightingGoldens.cs
src/vsa/dynamic_lighting/tests/golden/unity_volumetric_v1.json
src/viewer/lighting_test.rs
features/dynamic_lighting.feature
tests/features.rs
```

No DynamicLighting-specific fog logic belongs in general viewer lighting,
camera, or world modules. A local `bevy_pbr` patch is not expected; if the
dedicated pass cannot obtain the required depth/view resources, stop and
document the exact missing Bevy seam before modifying the patch.

## 7. Live acceptance scene

`cargo run-dev -- lighting-test --agent-bridge --shadow-resolution 256` must
demonstrate all of the following in one scene:

- sphere, box, `ConeZ`, and `ConeY` fog shapes are visually distinguishable;
- geometry terminates fog rays correctly instead of receiving fog through
  walls;
- the strobe's direct light and fog turn on and off from the same temporal
  state;
- static baked shadows and the moving realtime caster still work;
- toggling key `6` changes only volumetric fog;
- custom fog remains with all Bevy lights and fog components absent;
- the FPS counter remains fixed to the bottom-right and is not composited into
  the fog;
- status reports the extracted/visible volume count;
- zero-volume mode issues no volumetric draw;
- logs contain no WGSL, pipeline, bind-group, or backend validation errors.

Capture BRP screenshots for volumetrics on/off, strobe active/inactive, and
custom-only mode. Record a bounded frame-time comparison for zero volumes and
the four-shape test rack; zero volumes must be indistinguishable from the
disabled path within measurement noise.

## 8. Validation gate

Run, in order:

```powershell
cargo fmt --all -- --check
cargo clippy --all-targets --features bevy/dynamic_linking -- -D warnings
cargo test-dev --lib dynamic_lighting -- --nocapture
cargo test-dev
cargo test
cargo run-dev -- lighting-test --agent-bridge --shadow-resolution 256
git diff --check
```

The feature is complete only when all Unity volumetric fixtures pass, all four
visible shapes work in the production GPU pass, effect modulation is shared
with direct light, the custom-only proof passes, existing static/realtime
shadow behavior is unchanged, and the live pass is error-free.

## 9. Shipped amendments

- A1: The batch reference project exports deterministic shader-equation sample
  values through Unity 6000.3 `Mathf`/`Vector3`, matching the established
  spatial-fixture method. Production WGSL behavior is additionally proven by
  live fog-on/off and custom-only captures; the batch project does not create a
  second renderer-specific camera stack.
- A2: The render world compacts active volumetric sources into a separate
  stable-order buffer and runs once per eligible view. Shape/depth rejection is
  performed in that view's fullscreen pass. CPU frustum compaction is deferred
  until a multi-camera workload demonstrates that the extra buffer
  indirection is worthwhile.
- A3: The four-shape scene uses one strobing Sphere plus volume-only Box,
  ConeZ, and ConeY sources. This keeps fog shapes readable without adding
  unrelated direct light, while still proving shared temporal state on the
  strobe.
- A4: Review remediation added a global Unity-compatible animation clock,
  corrected type-specific axes and invalid cutoff handling, restored source
  direct/shadow defaults, labeled the test rack's bounce approximation, and
  added production-WGSL readback under perspective and orthographic cameras.
  The marked view now receives its required prepasses automatically, the
  1,024-light cap is diagnostic, and shadow sampling consumes Bevy's finalized
  cubemap allocation and per-proxy bias/near metadata.
