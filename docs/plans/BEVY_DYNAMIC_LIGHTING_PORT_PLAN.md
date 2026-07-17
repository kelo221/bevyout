# Architecture & Implementation Plan: Porting AlpacaIT.DynamicLighting to Bevy 0.19

## 1. Overview & Isolation Philosophy (The OpenMW Model)

`AlpacaIT.DynamicLighting` brings Tim Sweeney's classic Unreal Engine 1 (Unreal Gold / UT '99) lighting architecture into modern 3D rendering. Unlike standard lightmaps (which bake static light colors directly onto textures) or dynamic shadowmaps (which incur high per-frame depth rendering costs), this system **precomputes static visibility/shadow channels per light source** while allowing **colors, intensities, spot angles, animated cookies, wave patterns, flickering, and bounce multipliers** to be mutated freely at runtime in GPU shaders.

### 1.1 Isolated Provenance Architecture
Following the OpenMW isolation precedent in `bevyout` (`src/vsa/openmw_esm4/`), **all ported DynamicLighting code must reside in a strictly isolated, self-contained directory with clear provenance attribution.** No Unity C# ported code or internal dynamic lighting data structures may pollute general engine/viewer code.

#### Directory Layout: `src/vsa/dynamic_lighting/`
```
src/vsa/dynamic_lighting/
├── NOTICE.md            # License attribution, copyright notice (Henry00IS), source commit, port log
├── README.md            # Module isolation boundary description and API bridge summary
├── mod.rs               # Narrow public API exposed to bevyout
│
├── core/                # ISOLATED CORE ENGINE (std + serde only, NO Bevy dependencies)
│   ├── mod.rs           # Core module exports
│   ├── types.rs         # Light parameters, LightType, LightEffect, LightChannel enums & structs
│   ├── channel_alloc.rs # Spatial graph-coloring channel allocator (0..=31 channels)
│   ├── effects.rs       # Deterministic Tim Sweeney procedural light animation state curves
│   └── packing.rs       # Bitmask compression, R8/R32Uint packing, GZip stream handlers
│
├── baker/               # ISOLATED OFFLINE RAYTRACING BAKER (std + serde + rayon + bvh)
│   ├── mod.rs           # Baker entry point
│   ├── bvh.rs           # Bounding Volume Hierarchy raytracing acceleration structure
│   ├── tracer.rs        # Multi-threaded raycasting kernel (Direct visibility & Bounce GI)
│   ├── padding.rs       # Texel seam dilation and edge padding filter
│   └── manifest.rs      # Serialized RON & binary payload schemas (.bevyout/cache/)
│
└── bevy_bridge/         # THIN BEVY 0.19 PRESENTATION & RENDER BRIDGE
    ├── mod.rs           # DynamicLightingPlugin for Bevy 0.19 App
    ├── components.rs    # Bevy Components: DynamicLight, DynamicLightManager
    ├── pipeline.rs      # ExtendedMaterial<StandardMaterial, DynamicLightingExtension>
    ├── gpu_structs.rs   # GPU POD repr(C) light array structs (WGSL compatible)
    └── dynamic_lighting.wgsl # Bevy 0.19 custom WGSL shader
```

---

## 2. Provenance, License & Attribution Requirements

The ported library will include a mandatory `NOTICE.md` within its isolated directory:

```markdown
# Provenance and License Notice

This directory (`src/vsa/dynamic_lighting/`) contains Rust code ported from 
AlpacaIT.DynamicLighting (Unity Package by Henry00IS / Henry de Jongh).

- **Original Project**: https://github.com/Henry00IS/DynamicLighting
- **Original Author**: Henry de Jongh (Alpaca IT)
- **License**: MIT License
- **Ported By**: bevyout Project Team

All ported algorithms (spatial channel allocation, raytracing kernel, Tim Sweeney procedural light curves, seam padding, and lightmap compression) are isolated within `src/vsa/dynamic_lighting/core/` and `src/vsa/dynamic_lighting/baker/`.
```

---

## 3. Strict Layer Boundaries & Isolation Contracts

To maintain complete isolation similar to `openmw_esm4`:

1. **Pure Rust Core (`src/vsa/dynamic_lighting/core/`)**:
   - Zero Bevy dependencies (`std` + `serde` only).
   - Driven via Cucumber feature tests without requiring a Bevy App harness.
   - Holds all pure logic: channel allocation graphs, Tim Sweeney light curve calculations, bitwise visibility packing.

2. **Isolated Baker (`src/vsa/dynamic_lighting/baker/`)**:
   - Consumes raw 3D mesh data (triangles, lightmap UVs) and produces `PreparedDynamicLightingManifest`.
   - Uses CPU BVH (`bvh` crate) and parallel raytracing (`rayon`).
   - Does not touch Bevy `World`, `App`, or `RenderApp`.

3. **Narrow Bevy 0.19 Bridge (`src/vsa/dynamic_lighting/bevy_bridge/`)**:
   - The *only* file/submodule that imports `bevy` types.
   - Exposes a single `DynamicLightingPlugin` struct and `DynamicLight` component to `bevyout`.
   - Extracts light parameters to GPU buffers in Bevy 0.19 render app.

---

## 4. Technical Specifications & Porting Roadmap

### 4.1 Channel Allocation Engine (`core/channel_alloc.rs`)
In any given scene, static lights whose influence radii overlap must be assigned distinct **Channel IDs (0..31)**. Realtime unbaked moving lights are assigned **Channel 32**.

1. **Light Influence Bounding Spheres**: Each light $L_i$ has origin $P_i$ and radius $R_i$.
2. **Overlap Graph**: Construct an undirected graph $G = (V, E)$ where an edge $(i, j) \in E$ exists if $\|P_i - P_j\| < R_i + R_j$.
3. **Greedy Graph Coloring**: Assign the lowest unused channel $c \in [0, 31]$ such that no neighboring light shares $c$.
4. **Validation**: If any clique or local region exceeds 32 overlapping lights, trigger an explicit baker error identifying the overlapping light FormIDs/positions.

### 4.2 Tim Sweeney Light Effects (`core/effects.rs`)
- Implement 19 framerate-independent procedural light animation curves (`Steady`, `Pulse`, `Pulsar`, `Candle`, `Fire`, `Flicker`, `Generator`, `Lightning`, `FluorescentStarter`, `FluorescentClicker`, `FluorescentRandom`, `Strobe`, `Overcast`, `Cloudy`, `Rotor`, `Wave`, `Interference`, `Shock`, `Disco`).
- Fixed timestep accumulator for decoupling VR/high-refresh frame rates from 30Hz light effect ticking.

### 4.3 GPU Parameter Memory Layout (`bevy_bridge/gpu_structs.rs`)
Packed into GPU Uniform/Storage Buffers (`GpuArrayBuffer<DynamicLightGpu>`):

```rust
#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
pub struct DynamicLightGpu {
    pub color_intensity: [f32; 4],   // [r, g, b, intensity]
    pub position_radius: [f32; 4],    // [x, y, z, radius]
    pub direction_falloff: [f32; 4],  // [dir_x, dir_y, dir_z, falloff]
    pub spot_wave_params: [f32; 4],   // [inner_cutoff, outer_cutoff, wave_freq, wave_phase]
    pub effect_params: [f32; 4],      // [effect_type, pulse_mod, shimmer_scale, shimmer_mod]
    pub bounce_params: [f32; 4],      // [bounce_r, bounce_g, bounce_b, bounce_modifier]
}
```

### 4.4 Phase-by-Phase Execution Roadmap

- **Phase 1: Isolated Core Setup**: Create `src/vsa/dynamic_lighting/` with `NOTICE.md`, `README.md`, and `core/` types.
- **Phase 2: Core Procedural Effects & Graph Coloring**: Implement `core/effects.rs` and `core/channel_alloc.rs` with std unit tests.
- **Phase 3: Isolated BVH Baker Engine**: Implement `baker/` raytracer (Direct visibility + Bounce GI) using `rayon` and `bvh`.
- **Phase 4: Serialization Contract**: Define `.bevyout/cache/scenes/<formid>/dynamic_lighting.ron` and binary payload schemas.
- **Phase 5: Bevy 0.19 Render Bridge & WGSL Shader**: Build `bevy_bridge/` with `ExtendedMaterial` and `dynamic_lighting.wgsl`.
- **Phase 6: Runtime Systems & Budgeting**: Build Bevy ECS systems for effect updates, light registration, and Channel 32 realtime light culling.
- **Phase 7: CLI & BRP Verification**: Integrate `--dynamic-lighting` flag into `bevyout` CLI dispatcher and BRP inspection agent bridge.

---

## 5. Cucumber Feature Specifications (BDD)

```gherkin
Feature: Dynamic Lighting Isolated Core Engine

  Scenario: Spatial light overlap graph coloring assigns distinct channels in isolated core
    Given a static scene with 3 overlapping lights at positions:
      | name   | x   | y   | z   | radius |
      | Light1 | 0.0 | 0.0 | 0.0 | 5.0    |
      | Light2 | 2.0 | 0.0 | 0.0 | 5.0    |
      | Light3 | 4.0 | 0.0 | 0.0 | 5.0    |
    When the isolated dynamic lighting core assigns channels
    Then Light1, Light2, and Light3 must all be assigned distinct channels between 0 and 31

  Scenario: Tim Sweeney flicker effect produces deterministic intensity
    Given a dynamic light with effect "Flicker" and base intensity 2.0
    When 0.1 seconds elapse in the isolated effects simulation
    Then the calculated current_intensity varies between 0.0 and 2.0 deterministically
```

---

## 6. Execution Guardrails

- **Strict Isolation**: No Unity C# or DynamicLighting internal code allowed outside `src/vsa/dynamic_lighting/`.
- **Build Rule Active**: DO NOT execute `cargo build`, `cargo check`, `cargo test`, or `cargo run` until explicit prompt confirmation ("cheese") is received from the user.
