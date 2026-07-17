# Isolated DynamicLighting port

This directory is the complete boundary for the port of Henry de Jongh's
Unity DynamicLighting package. `mod.rs` exposes only the Bevy plugin, custom
light component, authoring enums, view marker, settings, and the shared default
bounce multiplier used by the existing irradiance baker.

- `core/`: Bevy-free authoring values, Unity-compatible random/Perlin/fixed
  timestep behavior, all 15 temporal effects, and CPU spatial references.
- `bevy_bridge/`: main-world custom-light ECS state. It does not create or
  mutate gameplay-facing Bevy `PointLight` components. The isolated
  `shadow_proxy.rs` boundary may synchronize transform/range to an explicitly
  authored black proxy; its intensity never follows an effect.
- `render/` and `shaders/`: the 112-byte upstream GPU ABI and HDR deferred
  fullscreen pass containing all eight spatial light functions.
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
proxy; F freezes temporal effects; Space pauses the moving caster. The HUD also
reports the render-world extracted custom-light count.
