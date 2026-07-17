# Isolated DynamicLighting port

This directory is the complete boundary for the port of Henry de Jongh's
Unity DynamicLighting package. `mod.rs` exposes only the Bevy plugin, custom
light component, authoring enums, light-layer component, view marker, settings,
and diagnostics.

- `core/`: Bevy-free authoring values, Unity-compatible random/Perlin/fixed
  timestep behavior, all 15 temporal effects, CPU spatial references, and
  source-compatible volumetric authoring/packing policy.
- `bevy_bridge/`: main-world custom-light ECS state. It does not create or
  mutate gameplay-facing Bevy `PointLight` components. The isolated
  `shadow_proxy.rs` boundary may synchronize transform/range to an explicitly
  authored black proxy; its intensity never follows an effect.
- `baker.rs`: the Bevy-free static triangle baker, deterministic artifact
  codec, 32-channel light association, raytraced visibility, and compressed
  single-bounce samples.
- `render/` and `shaders/`: the 112-byte upstream GPU ABI, packed artifact
  upload, and depth-aware volumetric pass for Sphere, Box, ConeZ, and ConeY
  fog. Static surface lighting is evaluated in the local Bevy forward PBR
  fragment path by `MeshTag`, primitive index, and `TEXCOORD_1`.
- `reference_unity/` and `tests/golden/`: reproducible Unity 6000.3 reference
  exporter and generated parity fixtures.
- `upstream/`: ignored frozen source checkout at the commit recorded in
  `NOTICE.md`.

The hermetic proof scene is launched from the repository root with:

```powershell
cargo run-dev -- lighting-test
```

Controls are shown in the scene. Keys 1 and 2 isolate prepared-static and
realtime moving-object shadows; 3 toggles the custom GPU pass; 4 hides ordinary
Bevy lights; 5 removes custom shadow visibility by hiding the shadow-only
proxy; 6 toggles Henry-style volumetric fog; B switches direct-only versus
single-bounce illumination; F freezes temporal effects; Space pauses the
moving caster. The HUD reports both render-world custom-light and
active volumetric counts, plus any sources clipped by the 1,024-light safety
limit. Marking a camera with `DynamicLightingView` automatically installs the
required depth prepass.

Volumetric fog stays inside this slice. It does not attach Bevy
`VolumetricLight` to the black shadow proxy or translate sources into Bevy
`FogVolume`; those components implement a different scattering model. The
custom fullscreen node executes after PBR surface lighting and distance fog,
before bloom/tonemapping/UI, and skips its draw when no active volumes are
present.

## Deliberate compatibility boundary

The standalone source-compatible defaults remain direct illumination with
raytraced shadows. Prepared Fallout lights opt into raytraced visibility plus
single bounce, with color, intensity, enable state, and temporal effects left
live at runtime. Cookie textures and shimmer remain follow-ups; transparency
modes are represented in the bake contract and fingerprint but do not yet
sample material alpha textures in the CPU tracer.
`DynamicLightLayerMask` is a light-side authoring filter, not a per-camera view
mask, so every marked camera currently consumes the same extracted light list.
The same boundary applies to volumetric sources: each view performs its own
depth-limited shape rejection, but the compact GPU source buffer is global.
Per-view CPU compaction is a deferred multi-camera performance optimization.

On Windows, `cargo test-dev --test dynamic_lighting_gpu -- --nocapture`
launches a deterministic production-render target scene with no ordinary Bevy
lights or shadow proxies. It waits for both custom passes to draw, freezes the
scene, then captures enabled/control images in one DX12 process. Region checks
cover every spatial mode and clear-background Sphere/ConeZ fog under both
perspective and orthographic cameras. The custom shadow pass uses Bevy's
finalized cubemap allocation, per-proxy bias values, near plane, and deferred
`NotShadowReceiver` flag.
