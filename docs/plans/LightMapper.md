# Recommended Direction

The pasted plan is reasonable for a standalone generic lightmapper, but it should be adjusted substantially for **bevyout**.

The repository already has:

* A native Rust scene-composition pipeline.
* A CPU BVH-based irradiance baker.
* Deterministic static batching.
* KTX output support.
* An explicit goal of removing Blender from the supported asset path.

The better design is therefore to **extend the current Rust baker into a shared surface-lightmap and irradiance-volume renderer**, rather than building a separate GPU-first subsystem beside it. ([GitHub][1])

The Solari 7 review adds an important constraint to that direction: Solari should
replace only the proposed custom GPU ray-tracing machinery. It must not replace
the backend-independent lightmap frontend, the CPU reference backend, or the
offline UV-texel accumulation model.

```text
Bevyout owns: UVs, atlas/texel maps, Fallout material and light policy,
             offline accumulation, denoising, dilation, caching, KTX2,
             manifest bindings, and CPU fallback.
Solari owns:  optional GPU BLAS/TLAS construction, ray queries, hit/material
             resolution, and reusable BRDF/light-sampling infrastructure.
```

```text
Prepared Fallout Scene
        │
        ▼
Static/Immutable Placement Classification
        │
        ▼
Native Rust Scene Composition and Batching
        │
        ├──► Lightmap UV Generation and Atlas Packing
        │          │
        │          ▼
        │    Conservative UV Texel Rasterization
        │          │
        │          ▼
        ├──► Shared CPU BVH and Material/Light Transport
        │          │
        │          ├──► Surface Lightmaps
        │          │      Direct + Multi-bounce Diffuse
        │          │
        │          └──► Irradiance Volumes
        │                 Dynamic actors and movable objects
        │
        ▼
Feature-Guided Denoising
        │
        ▼
Chart-Aware Dilation
        │
        ▼
Linear HDR KTX2
        │
        ▼
GLB TEXCOORD_1 + PreparedBake Lightmap Bindings
        │
        ▼
Bevy Lightmap Components
```


## Solari 7 review and impact on the current code

Solari v7, as described by the [v7 change set][12] and its [tracking issue][13],
is now the preferred source of GPU ray-tracing infrastructure when
the optional backend is implemented: BLAS/TLAS construction, ray queries,
scene/material binding, hit resolution, and reusable BRDF helpers. Its unified
real-time ReSTIR lighting path is camera- and history-dependent, so it is not
the lightmap baker. The camera-based path-tracing plugin is also a reference
integrator to adapt, not a plugin to run unchanged for UV-atlas baking.

The code already written does **not** need a Solari-driven rewrite. M0, the CPU
transport module, UV1 generation, final GLB emission, CPU surface pages,
manifest bindings, and Bevy `Lightmap` attachment are backend-independent work
and remain valid. The opt-in adapter now has a tested headless ray-query
session, but the default build still does not enable `bevy_solari`, final
UV1-bearing meshes are not used as Solari proxies, and the CPU BVH remains the
reference backend.

The required future seam is instead:

1. Define one backend contract consumed by the same bake scene, texel map,
   material/light interpretation, deterministic seed convention, accumulation
   format, and atlas layout.
2. Keep the current CPU implementation behind that contract as the correctness
   and cross-platform fallback.
3. Add a Solari-only bake proxy representation. The final lightmapped mesh may
   contain `TEXCOORD_1`; the Solari proxy must preserve the same triangles and
   transforms while using Solari's currently supported
   `POSITION/NORMAL/UV_0/TANGENT` vertex layout and no UV1.
4. Dispatch a custom bevyout bake node over UV texels/surfels, using Solari's
   scene bindings for rays and hits rather than a second custom GPU BVH.

That seam belongs to Milestone 6. The current CPU implementation still has
known planned limitations, including no partial light invalidation and no
cross-adapter GPU/CPU statistical suite. A feature-gated Solari bake adapter now exists
as a bounded zero-to-four-bounce dispatch/readback prototype and is wired into
the CLI, tile cache, atlas, and manifest path; it is not yet the authoritative
bake backend.
Surface-lightmap recursion is now configurable from zero through eight
secondary bounces, with two as the compatibility default. Adaptive convergence,
an initial feature-guided denoiser, resumable raw transport tiles, explicit
emissive-area sampling with MIS, and Russian roulette are now part of the CPU
pipeline; they remain independent of Solari.

---

# 1. Define the Lighting Contract First

Before implementing UVs or path tracing, define exactly which lighting belongs where.

## Recommended baked-only contract

### Immutable static geometry

Bake:

* Static point-light diffuse.
* Static spot-light diffuse.
* Static directional-light diffuse.
* Emissive-material lighting.
* Cell/world ambient contribution.
* Multi-bounce diffuse global illumination.
* Static occlusion and shadows.

Do not apply at runtime:

* Static direct diffuse from the same lights.
* Global ambient on lightmapped meshes.
* Irradiance-volume diffuse on lightmapped meshes.

### Dynamic actors and movable objects

Use:

* The existing irradiance volume, upgraded to use the shared transport implementation.
* Reflection probes for specular.
* Optionally dynamic lights for genuinely dynamic effects.

### Static specular

Keep reflection probes or environment maps. Bevy lightmaps represent diffuse illumination; they do not replace view-dependent specular reflections. Bevy’s PBR shader prioritizes a lightmap as the diffuse indirect source while still allowing environment-map specular. ([GitHub][2])

This gives the desired Fallout-style arrangement:

```text
Static architecture:
    Surface lightmap

NPCs, creatures, movable clutter:
    Irradiance volume

Metallic/specular response:
    Reflection probes

Muzzle flashes, explosions, animated lights:
    Runtime dynamic lights
```

---

# 2. Correct the Current Bake Accuracy Problems

The original baker was to be refactored before surface lightmaps were added;
that shared transport seam is now partly shipped and should remain the
authority for both outputs.

Historically, the six-lobe volume sampled visible surface radiance containing
only direct point/directional light plus emissive contribution. The shared
transport fixes below are now partly shipped: the CPU surface path has
deterministic recursive diffuse transport with a configurable surface-bake
depth, both bake paths receive cell ambient, point lights use the shared
explicit-intensity/fallback conversion, emissive areas use explicit next-event
sampling with MIS, and paths use Russian roulette after bounce three.
([GitHub][3])

Address these in the shared transport layer:

1. Replace the hard-coded lighting scale with a common photometric conversion used by both the baker and runtime.
2. Pass the prepared scalar ambient into the integrator, and support an
   optional authored environment-radiance map as an additive escape source.
3. Support point, spot, directional, and emissive-area light sampling.
4. Implement recursive diffuse bounces.
5. Use deterministic sampling and adaptive convergence.
6. Use the same material interpretation for lightmaps and irradiance volumes.

This avoids producing surface lightmaps that disagree visibly with dynamic-object lighting.

---

# 3. Static Geometry Eligibility

The current static-bake filter primarily checks whether a placement is marked static and initially enabled. The prepared scene model already has a more detailed mutability classification: `Immutable`, `EnableGroup`, `ScriptAddressable`, and `Unknown`. Surface lightmaps should use that classification explicitly. ([GitHub][4])

## Default policy

| Mutability          | Lightmap receiver | Static occluder | Lighting method           |
| ------------------- | ----------------: | --------------: | ------------------------- |
| `Immutable`         |               Yes |             Yes | Surface lightmap          |
| `EnableGroup`       |      No initially |    No initially | Irradiance volume/runtime |
| `ScriptAddressable` |                No |              No | Irradiance volume/runtime |
| `Unknown`           |                No |              No | Irradiance volume/runtime |

This conservative default prevents:

* Shadows remaining after a door or object disappears.
* Baked darkening where an enable-parent object was disabled.
* Movable objects carrying lighting tied to their original position.
* Scripted geometry leaving ghost illumination.

A later extension can bake separate lighting states for important enable groups, but that should not block the first implementation.

---

# 4. Native Lightmap UV Generation

## Do not depend on Blender

UV generation should happen directly inside the existing Rust preparation pipeline after world transforms and static composition have been resolved.

The upstream xatlas library is well suited to this: it is a small C++ library without external runtime dependencies and explicitly supports unique UV generation for lightmaps. It can duplicate seam vertices, so the integration must remap every vertex attribute, not merely append UV coordinates. The newer Rust wrapper exists because the original wrapper became unmaintained; for a long-lived engine pipeline, vendoring the upstream `xatlas.cpp` and `xatlas.h` with a thin Rust FFI is safer than depending blindly on either wrapper. ([GitHub][5])

## Build arrangement

```text
crates/bevyout-xatlas/
├── build.rs
├── vendor/
│   └── xatlas/
│       ├── xatlas.cpp
│       ├── xatlas.h
│       └── LICENSE
└── src/
    ├── lib.rs
    └── ffi.rs
```

Use the Rust `cc` crate in `build.rs`. This requires only the normal native compiler toolchain used by Rust crates with C/C++ dependencies:

* MSVC on Windows.
* Apple Clang on macOS.
* GCC or Clang on Linux.

No Blender, Python, CUDA, Vulkan ray-tracing extensions, or OS-specific graphics SDK should be required.

## Perform unwrapping after composition

Generate UV1 after:

* Placement transforms have been applied.
* Static batching has been decided.
* Geometry has been split by material.
* Mutable objects have been excluded.
* Degenerate and unsupported triangles have been removed.

This matters because two instances of the same source mesh generally receive different lighting and therefore cannot share the same final surface-lightmap coordinates.

## Preserve all vertex data during seam duplication

When xatlas produces a new vertex mapping, duplicate/remap:

* Position.
* Normal.
* Tangent.
* Primary UV.
* Vertex color.
* Skin data, if any unexpectedly reaches the static path.
* Material-specific transport data.
* Generated UV1.

The current `ComposedPrimitive` has positions, normals, primary UVs, colors, transport colors, and indices, but no secondary UV set. The GLB writer similarly does not currently emit `TEXCOORD_1`. Both structures need to be extended. ([GitHub][6])

---

# 5. Atlas Packing Model

Do not allow a single Bevy mesh primitive to span multiple lightmap images. A Bevy entity receives one `Lightmap` component and one atlas rectangle.

A robust arrangement is:

1. Generate local UV1 coordinates for each composed primitive.
2. Determine that primitive’s required lightmap tile size.
3. Rectangle-pack primitive tiles into one or more atlas pages.
4. Keep UV1 local to the tile.
5. Store the tile location through `Lightmap::uv_rect`.
6. Split a primitive spatially when its required tile is larger than the maximum page size.

Bevy explicitly recommends atlasing lightmaps to avoid breaking batching, and exposes `uv_rect` for that purpose. ([Docs.rs][7])

## Texel-density controls

Use texels per world-space metre rather than estimating one arbitrary square texture per mesh.

Suggested presets:

| Preset     |            Texels/metre |             Samples | Bounces | Intended use                       |
| ---------- | ----------------------: | ------------------: | ------: | ---------------------------------- |
| Preview    |                    8–12 |               32–64 |       2 | Iteration and UV validation        |
| Medium     |                   16–24 |           Up to 256 |       4 | Routine development bakes          |
| Production |                   32–48 | Up to 1024 adaptive |       6 | Final interior cells               |
| Hero       | Configurable per object | Up to 2048 adaptive |     6–8 | Small visually important locations |

These should be defaults, not hard-coded constraints.

## Atlas defaults

```text
Default page size:       4096 × 4096
Allowed page sizes:      1024–8192
Default chart padding:   12 texels
Minimum chart padding:   8 texels
Tile alignment:          4 texels
```

A 4096 default is a safer baseline across desktop adapters than assuming every machine should receive 8192 textures.

The current CLI exposes a global default plus keyed per-placement overrides:

```text
--lightmap-texels-per-meter 16
--lightmap-density 000151e3=32
--lightmap-environment-map lighting/interior.hdr
```

`--lightmap-environment-map` is an optional 2:1 equirectangular HDR radiance
map. Its pixels are sampled in linear RGB with horizontal wrapping and
vertical clamping. Receiver irradiance combines cosine-BSDF and
luminance/solid-angle environment sampling with deterministic MIS, and the
map is returned as radiance when an indirect ray escapes; prepared scalar cell
ambient remains additive and authoritative when the option is omitted.

The density range is 1–128 texels per metre. Overrides are keyed by the
prepared reference FormID; duplicate keys are rejected. The selected density
flows through scene composition, static batching, xatlas, binding metadata,
and the bake fingerprint, so different densities cannot accidentally share one
composed primitive or stale accumulation cache.

The longer-term policy can still grow to include explicit exclusion and fixed
tile overrides:

```rust
pub enum LightmapResolutionOverride {
    Exclude,
    Scale(f32),
    TexelsPerMeter(f32),
    FixedTile { width: u32, height: u32 },
}
```

---

# 6. Replace the UV-Space G-Buffer With a Texel Map

The proposed world-position and world-normal render targets are unnecessary for the canonical CPU implementation.

Instead, rasterize triangles directly in UV space on the CPU and produce:

```rust
struct LightmapTexel {
    primitive_index: u32,
    triangle_index: u32,
    barycentric: [f32; 3],
    chart_id: u32,
    material_id: u32,
    coverage: f32,
    flags: u32,
}
```

World-space attributes can then be reconstructed from the triangle and barycentric coordinates when sampling.

## Advantages

* Deterministic across graphics backends.
* No GPU readback.
* No `Rgba32Float` position texture.
* No loss of precision for large world coordinates.
* Lower memory use.
* Easy chart-edge detection.
* Easy incremental tile caching.
* The same texel map can later be consumed by either the CPU backend or an
  optional Solari-backed GPU integrator.

## Rasterization requirements

Implement:

* Top-left edge convention.
* Conservative triangle coverage.
* Degenerate UV-triangle rejection.
* Four-sample or 2×2 edge supersampling.
* Per-chart ownership.
* A validity mask.
* Geometric and shading normal reconstruction.
* An offset position based on geometric normal.

A texel partly covered by a triangle should not be treated identically to a fully covered interior texel. Coverage data improves both edge antialiasing and dilation.

---

# 7. Shared Transport Contracts and Backend Boundary

Refactor the reusable portions of `rust_irradiance.rs` into a shared CPU
transport module, then keep backend selection above it. The CPU BVH is the
reference implementation; it is not the GPU representation to upload once
Solari is used.

```text
src/vsa/bake/
├── transport/
│   ├── mod.rs
│   ├── scene.rs
│   ├── bvh.rs
│   ├── intersection.rs
│   ├── material.rs
│   ├── lights.rs
│   ├── sampling.rs
│   └── integrator.rs
├── irradiance_volume/
│   └── ...
└── lightmap/
    └── ...
```

The transport scene should contain immutable bake-ready data only:

```rust
struct BakeTriangle {
    positions: [[f32; 3]; 3],
    geometric_normal: [f32; 3],
    shading_normals: [[f32; 3]; 3],
    uv0: [[f32; 2]; 3],
    material_id: u32,
    primitive_id: u32,
}

struct BakeMaterial {
    base_color: [f32; 3],
    emissive: [f32; 3],
    metallic: f32,
    roughness: f32,
    alpha_mode: BakeAlphaMode,
    double_sided: bool,
    base_color_texture: Option<TextureId>,
    emissive_texture: Option<TextureId>,
}
```

The future backend boundary should be narrow and backend-neutral:

```rust
pub enum LightmapBackendPreference {
    Auto,
    Solari,
    Cpu,
}

pub trait LightmapTraceBackend {
    fn name(&self) -> &'static str;
    fn is_available(&self) -> bool;
    fn prepare_scene(&mut self, scene: &BakeScene) -> anyhow::Result<()>;
    fn accumulate_tile(
        &mut self,
        tile: &BakeTile,
        settings: &IntegratorSettings,
        accumulation: &mut TileAccumulation,
    ) -> anyhow::Result<()>;
}
```

Both implementations must consume the same `BakeScene`, material sampling,
Fallout light definitions, texel map, seed convention, output convention,
atlas layout, and accumulation-cache format. `Auto` selects Solari only when
its adapter reports the required hardware capabilities; otherwise it selects
the CPU backend. An explicit `Solari` request must fail with a clear capability
error rather than silently changing quality or falling back.

## Solari reuse boundary

The optional GPU adapter should reuse Solari's scene plugin/bindings (the
current 0.19 public example uses `SolariPlugins`), BLAS/TLAS construction,
scene bind group, ray-query helpers, hit resolution, material/texture lookup,
and selectively its BRDF and light-sampling routines. Resolve the exact API at
the adapter spike because the v7 branch and Bevy 0.19 do not necessarily expose
identical symbol names.
Bevyout must retain its own Fallout light metadata, ambient policy, and
alpha/transparency policy until the optional adapter supports its remaining
texture-varying emitter and richer material cases, along with
UV/atlas/rasterization logic,
offline accumulation, denoising, dilation, cache, encoding, and manifest
binding.

Do not use `SolariLightingPlugin`/unified ReSTIR as the authoritative bake
integrator. Do not run the camera-space path tracer unchanged or project
hundreds of camera views into UV space. The custom node's primary input is a
valid lightmap texel with world position and normals, not a camera ray.

## Intersection improvements

Add:

* Scale-aware ray offsets rather than relying only on a fixed `0.002`.
* Separate geometric and shading normals.
* Watertight or near-watertight triangle intersection.
* Backface handling based on `double_sided`.
* Alpha-mask testing at the actual hit UV.
* Consistent handling of blended materials.
* NaN and invalid-normal rejection.
* Optional self-intersection primitive exclusion for the first ray segment.

---

# 8. Physically Consistent Surface Integrator

The most important output convention is:

> The lightmap must not contain the receiving surface’s base color.

Bevy reads the lightmap through UV1 and multiplies it by the material’s runtime diffuse color. Therefore, for a Lambertian receiver, store the incident diffuse-light factor—approximately `irradiance / π` under the chosen convention—not `receiver_albedo × irradiance / π`. ([Docs.rs][7])

For indirect bounces:

* Exclude the original receiver’s albedo.
* Include the albedo of surfaces that bounce light along the path.
* Include emissive radiance from emissive hits.
* Include environment/cell ambient when a ray escapes.

Conceptually:

```text
Lightmap texel =
    direct diffuse factor at receiver
  + one-bounce diffuse factor
  + two-bounce diffuse factor
  + ...
```

not:

```text
Lightmap texel =
    receiver base color × lighting
```

## Sampling strategy

Use:

* Next-event estimation for point, spot, directional, and emissive lights.
* Power-weighted light selection.
* Cosine-weighted hemisphere sampling.
* Multiple-importance sampling for emissive triangles and environment lighting.
* Russian roulette after bounce 3.
* Adaptive sampling based on variance.
* Deterministic low-discrepancy sequences.

A suitable production configuration:

```text
Minimum samples:     128
Maximum samples:     1024
Maximum bounces:     6
Russian roulette:    from bounce 3
Variance threshold:  configurable
Tile size:           16 × 16 or 32 × 32 texels
```

## Determinism

Seed each sample from stable inputs:

```text
scene fingerprint
atlas page
texel coordinate
sample index
integrator revision
```

Do not seed from thread IDs, execution order, system time, or GPU wave order.

Exact floating-point output may still differ slightly between architectures, but scene topology, atlas layout, sample sequences, and convergence decisions should remain deterministic.

## Light-unit agreement

Create one shared conversion path from prepared Fallout light records into bake/runtime light parameters.

Do not let the baker use radius-derived brightness while runtime uses a separate intensity interpretation. At minimum, the following must be shared:

* Lumens or engine intensity.
* Radius/range.
* Attenuation curve.
* Spot cone angles.
* Light color space.
* Directional-light illuminance convention.
* Fade threshold.

---

# 9. Denoising and Dilation

## Denoise before dilation

The order should be:

```text
Raw sampled chart texels
        ▼
Coverage resolve
        ▼
Feature-guided denoise
        ▼
Chart-aware dilation
        ▼
Encoding
```

Denoising after dilation can pull padded values back across chart boundaries.

## In-tree denoiser

Implement an edge-avoiding À-Trous filter guided by:

* Geometric normal.
* Shading normal.
* Position or depth.
* Chart ID.
* Material ID.
* Sample variance.
* Coverage.
* Luminance difference.

The chart ID must be an absolute barrier. Two UV islands that are adjacent in atlas space must never exchange denoising samples.

OIDN can be retained as:

```toml
[features]
oidn-denoiser = ["dep:oidn"]
```

It should not be required for ordinary builds or CI because that would add another native cross-platform dependency.

## Dilation

Use nearest-valid chart-aware propagation rather than repeatedly averaging arbitrary neighbours.

Good implementations include:

* Jump-flood propagation.
* Multi-source breadth-first propagation.
* Exact nearest-valid search within a limited padding radius.

The propagated texel should retain the color of the closest valid texel belonging to the appropriate chart or tile.

---

# 10. KTX2 Output

The default production output should be a linear HDR format:

```text
Preferred:
    R16G16B16A16_SFLOAT + Zstd

Compact option:
    E5B9G9R9_UFLOAT_PACK32 + Zstd

Debug:
    R32G32B32A32_SFLOAT EXR or raw dump
```

KTX supports uncompressed GPU formats and Zstd supercompression separately from Basis Universal. Basis ETC1S/UASTC should not be the default for HDR lightmaps because it targets a different compression trade-off and can introduce visible block or range artifacts. ([GitHub][8])

The surface-lightmap and irradiance-volume paths now write and validate their
required KTX2 containers in Rust. The current outputs are uncompressed; Zstd
is still an optional production-compression step rather than a bake-time
dependency. ([GitHub][9])

This removes the external encoder from the `bake` command. Reflection-probe
and prepared static-shadow tooling still expose their separate legacy KTX
paths; those are not part of this surface/volume encoder slice.

## Cross-platform encoding plan

### Phase 1

Support:

```text
Internal raw HDR buffer
    ├── In-tree uncompressed RGBA16F KTX2 lightmap output
    ├── In-tree uncompressed RGB9E5 3D KTX2 irradiance-volume output
    └── Raw/debug output for tests
```

### Final architecture

Implement a small pure-Rust writer for only the required combinations:

* KTX2 container.
* `R16G16B16A16_SFLOAT`.
* `E5B9G9R9_UFLOAT_PACK32`.
* Optional Zstd supercompression.
* One mip level initially.
* Required DFD and level index metadata.

Avoid embedding the whole KTX-Software build as a mandatory dependency.

## Mipmaps

Bevy’s current lightmap shader explicitly samples mip level zero, and its source notes that conventional mipmapping commonly causes UV-island leakage. Therefore, the first implementation should write one mip level and rely on adequate padding/dilation. ([GitHub][10])

Atlas dimensions do not need to be powers of two on the supported Bevy/wgpu
path. The reduced real-cell bake produced a `3996 × 3980` single-mip atlas,
which loaded as `VK_FORMAT_R16G16B16A16_SFLOAT` with no supercompression.
The relevant constraints are the device's maximum 2D texture dimensions and
the gutter/`uv_rect` contract, not power-of-two sizing. If mipmaps are added
later, they must be generated chart-by-chart with per-mip dilation; enabling
ordinary atlas-wide mip generation would reintroduce island leakage.

A later custom shader path can support chart-aware mip generation, where every mip level is generated separately within chart boundaries.

---

# 11. GLB and Manifest Changes

## Extend composed primitives

```rust
pub struct ComposedPrimitive {
    pub positions: Vec<[f32; 3]>,
    pub normals: Vec<[f32; 3]>,
    pub uv0: Vec<[f32; 2]>,
    pub uv1: Vec<[f32; 2]>,
    pub colors: Vec<[f32; 4]>,
    pub transport_colors: Vec<[f32; 4]>,
    pub indices: Vec<u32>,

    pub primitive_key: String,
    pub lightmap_binding_id: Option<u32>,
}
```

Emit UV1 as:

```text
TEXCOORD_1
```

Add a stable generated identifier to GLTF node or primitive extras:

```json
{
  "bevyout": {
    "primitive_key": "cell:00015208/chunk:4/material:17/primitive:2",
    "lightmap_binding": 42
  }
}
```

Do not rely only on display names or GLTF array indices, as those can change when batching is adjusted.

## Extend `PreparedBake`

The current manifest records the scene and optional irradiance volume but no surface-lightmap pages or primitive bindings. ([GitHub][4])

Suggested model:

```rust
pub struct PreparedBake {
    pub scene_path: String,
    pub lightmaps: Vec<PreparedLightmapAtlas>,
    pub lightmap_variance_pages: Vec<PreparedLightmapVariancePage>,
    pub lightmap_bindings: Vec<PreparedLightmapBinding>,
    pub irradiance_volume: Option<PreparedIrradianceVolume>,
    pub bake_settings: PreparedBakeSettings,
}

pub struct PreparedLightmapVariancePage {
    pub primitive_key: String,
    pub asset_path: String,
    pub width: u32,
    pub height: u32,
    pub format: PreparedLightmapVarianceFormat,
    pub content_hash: String,
    pub covered_texels: u32,
}

pub enum PreparedLightmapVarianceFormat {
    R32FloatRaw,
}

pub struct PreparedLightmapAtlas {
    pub asset_path: String,
    pub width: u32,
    pub height: u32,
    pub format: PreparedLightmapFormat,
    pub content_hash: String,
}

pub struct PreparedLightmapBinding {
    pub binding_id: u32,
    pub primitive_key: String,
    pub atlas_index: u16,
    pub uv_rect: [f32; 4],
    pub texels_per_meter: f32,
}

pub enum PreparedLightmapFormat {
    Rgba16Float,
    Rgb9e5,
}
```

Also store:

* Integrator version.
* xatlas version or source revision.
* UV-layout fingerprint.
* Material fingerprint.
* Light fingerprint.
* Sample and bounce settings.
* Denoiser version.
* Encoder version.

Increment `CURRENT_BAKE_REVISION` when the interpretation changes.

---

# 12. Runtime Attachment and Double-Counting Prevention

After a GLTF scene is spawned:

1. Read the stable binding ID from GLTF extras.
2. Resolve its atlas and rectangle from `PreparedBake`.
3. Insert the Bevy `Lightmap` component.
4. Ensure the mesh contains `ATTRIBUTE_UV_1`.
5. Keep bicubic sampling disabled initially.
6. Keep material lightmap exposure at a calibrated global value, preferably `1.0`.

```rust
commands.entity(entity).insert(Lightmap {
    image: atlas_handle.clone(),
    uv_rect: Rect::new(
        binding.uv_rect[0],
        binding.uv_rect[1],
        binding.uv_rect[2],
        binding.uv_rect[3],
    ),
    bicubic_sampling: false,
});
```

## Required lighting changes

For the baked-only path:

```rust
GlobalAmbientLight {
    affects_lightmapped_meshes: false,
    ..
}
```

```rust
IrradianceVolume {
    affects_lightmapped_meshes: false,
    ..
}
```

For static point, spot, and directional lights:

```rust
affects_lightmapped_mesh_diffuse: false
```

or do not spawn those static lights at runtime at all.

The current viewer creates prepared point/spot/directional lighting, but the
static direct diffuse path now sets `affects_lightmapped_mesh_diffuse: false`
when lightmap pages exist and disables ambient/volume diffuse on those meshes.
This prevents static surfaces from being lit more than once. ([GitHub][11])

Reflection probes can remain active for specular. The current reflection-probe configuration already avoids applying its diffuse contribution to lightmapped meshes. ([GitHub][11])

---

# 13. Proposed Source Layout

```text
src/vsa/bake/
├── mod.rs
├── settings.rs
├── fingerprint.rs
├── backend.rs                 # backend-neutral selection and contracts
│
├── transport/
│   ├── mod.rs
│   ├── scene.rs
│   ├── bvh.rs
│   ├── intersection.rs
│   ├── material.rs
│   ├── texture.rs
│   ├── lights.rs
│   ├── sampling.rs
│   └── integrator.rs
│
├── lightmap/
│   ├── mod.rs
│   ├── eligibility.rs
│   ├── unwrap.rs
│   ├── atlas.rs
│   ├── texel_map.rs
│   ├── bake.rs
│   ├── denoise.rs
│   ├── dilate.rs
│   ├── encode.rs
│   └── cache.rs
│
└── irradiance_volume/
    ├── mod.rs
    ├── layout.rs
    ├── bake.rs
    └── encode.rs
```

Optional GPU implementation (behind `lightmap-gpu-solari`):

```text
src/vsa/bake/backend/
├── cpu.rs
└── solari.rs                  # Bevy render adapter, not core transport
```

Repository-specific edits:

| File or area                          | Change                                                             |
| ------------------------------------- | ------------------------------------------------------------------ |
| `src/vsa/bake/rust_irradiance.rs`     | Split shared tracing and keep only volume-specific integration     |
| `src/vsa/bake/rust_scene.rs`          | Add UV1, vertex remapping, stable primitive keys, GLB `TEXCOORD_1` |
| `crates/bevyout-core/src/manifest.rs` | Add atlas and binding structures                                   |
| `src/viewer/scene.rs`                 | Attach lightmaps and disable duplicate diffuse sources             |
| `src/cli.rs`                          | Add lightmap settings and quality presets                          |
| `src/vsa/bake/backend.rs`             | Select CPU or optional Solari backend through one contract          |
| `src/vsa/bake/backend/solari.rs`      | Build UV1-free Solari bake proxies and texel accumulation           |
| Static placement classification       | Require immutable geometry for surface baking                      |
| Bake fingerprinting                   | Include UV, integrator, denoiser, atlas, and encoder revisions     |

---

# 14. CLI Design

```bash
bevyout bake SuperDuperMart \
    --lightmaps \
    --lightmap-quality production \
    --lightmap-texels-per-meter 32 \
    --lightmap-page-size 4096 \
    --lightmap-padding 12 \
    --lightmap-min-samples 128 \
    --lightmap-max-samples 1024 \
    --lightmap-bounces 6 \
    --lightmap-denoiser atrous \
    --lightmap-format rgba16f \
    --bake-backend cpu
```

Additional useful controls:

```text
--lightmap-direct-only
--lightmap-no-emissive
--lightmap-no-ambient
--lightmap-variance-threshold <value>
--lightmap-max-pages <count>
--lightmap-density <FORM_ID=TEXELS_PER_METER>
--lightmap-debug-uv
--lightmap-debug-samples
--lightmap-debug-variance
--lightmap-resume
--lightmap-tile-size <power-of-two>
--lightmap-denoise-iterations <0..5>
--lightmap-force-retrace
--lightmap-force-repack
```

Keep the CPU backend as the default and reference implementation:

```text
--bake-backend cpu
```

The optional Solari backend is explicit during its prototype phase:

```text
--bake-backend solari
```

Expose it only when the build includes:

```toml
[features]
lightmap-gpu-solari = ["bevy/bevy_solari"]
```

The feature must remain off in the default game build. `Auto` still resolves to
the CPU reference path; a feature-enabled explicit `Solari` request attempts
the adapter and returns a clear capability or unsupported-input error. A build
without the feature rejects it before baking. Bevy 0.19 already exposes the
feature, but Solari remains experimental, so the adapter should stay narrow and
disposable when bevyout migrates to Bevy 0.20.

Both backends must consume the same:

* Bake scene.
* UV atlas.
* Texel map.
* Material representation.
* Light representation.
* Sampling seeds.
* Output convention.

---

# 15. Incremental and Resumable Baking

Production lightmaps can be expensive. Structure the cache so that a changed light does not force UV regeneration.

```text
cache/
├── scene_geometry.bin
├── uv_layout.bin
├── atlas_layout.bin
├── texel_maps/
│   └── page_000.bin
├── bvh.bin
└── accumulation/
    ├── page_000_tile_0000.bin
    ├── page_000_tile_0001.bin
    └── ...
```

Separate fingerprints:

| Fingerprint     | Invalidated by                              |
| --------------- | ------------------------------------------- |
| Geometry        | Mesh, transform, static eligibility         |
| UV layout       | Geometry, density, padding, xatlas revision |
| Transport scene | Geometry, materials, texture interpretation |
| Lighting        | Lights, ambient, emissive values            |
| Sampling        | Samples, bounces, seed, integrator revision |
| Denoise         | Denoiser settings                           |
| Encoding        | KTX format and compression                  |

This permits:

* Continuing an interrupted bake.
* Re-encoding without retracing.
* Re-denoising without rebaking.
* Reusing UV layouts after lighting changes.
* Rebaking only affected atlas pages or spatial tiles.

### Shipped amendments — 2026-08-05

* The CPU bake now writes sparse raw transport tiles to
  `baked/lightmap-accumulation/` and validates each tile with a cache
  fingerprint, dimensions, identity, and checksum. Publication is temporary
  file plus rename, so an interrupted write is never treated as complete.
* `--lightmap-tile-size` controls the deterministic tile edge and
  `--lightmap-force-retrace` clears the accumulation root. Re-running the same
  bake resumes completed tiles before denoising, atlas packing, and encoding.
* The cache now separates the shared scene/transport identity from each
  primitive's relevant point/spot-light set. A light edit only invalidates
  primitive pages whose bounds intersect the changed light range; distant
  primitive pages retain their tile payloads. Corrupt or stale tile payloads
  remain misses, while unrelated valid tiles are preserved.
* CI now runs a focused miniature CPU bake on every Windows, macOS, and Linux
  test job. The synthetic triangle exercises UV1 rasterization, composed GLB
  `TEXCOORD_1` emission, accumulation tile publication and resume hits, atlas
  packing, the production primitive-to-atlas binding projection, RGBA16F KTX2
  writing/reading, and finite texel validation without requiring Fallout data,
  Blender, or an external KTX executable. The focused check is deliberately a
  small cross-platform bake smoke test rather than a real-cell acceptance bake.

---

# 16. Cross-Platform Requirements

Treat these as supported release targets:

```text
x86_64-pc-windows-msvc
aarch64-apple-darwin
x86_64-apple-darwin
x86_64-unknown-linux-gnu
```

The mandatory path should depend only on:

* Rust.
* Rayon.
* Existing CPU BVH implementation.
* Vendored xatlas compiled by `cc`.
* Pure-Rust image, Zstd, and KTX2 writing.
* Bevy’s existing runtime texture loading.

The optional Solari backend additionally requires hardware ray queries,
buffer/texture binding arrays, non-uniform indexing, and partially bound
binding arrays. Its availability is therefore capability- and driver-based,
not merely OS-based:

```text
Supported Windows/Linux adapter       → optional Solari backend
Supported Apple Silicon / Bevy 0.20  → optional Solari backend when available
Unsupported adapter, CI, or headless  → CPU backend
```

The CPU path remains mandatory for all supported platforms and is the numeric
reference for GPU comparisons. Solari's remaining texture-varying emitter,
production-scale emitter lookup, and richer custom-material gaps must be
covered by bevyout-side policies or remain on the CPU path until the adapter
handles them explicitly.

It should not require:

* Blender.
* Python.
* CUDA.
* OptiX.
* DirectX Raytracing.
* Vulkan ray-tracing extensions.
* Metal ray tracing.
* OIDN.
* A separately installed KTX CLI in the final version.
* Platform-specific shell scripts.

## CI matrix

Run on:

```yaml
strategy:
  matrix:
    os:
      - windows-latest
      - macos-latest
      - ubuntu-latest
```

Each CI job should:

1. Compile the xatlas wrapper.
2. Build `bevyout`.
3. Run UV-layout tests.
4. Bake a tiny fixed scene.
5. Validate the GLB contains `TEXCOORD_1`.
6. Validate the manifest bindings.
7. Load the KTX2.
8. Check every valid texel is finite.
9. Compare lighting against reference tolerances.
10. Confirm no Blender executable is queried or invoked.

The current matrix implements the miniature-bake portion with
`cargo test --locked --lib miniature_surface_bake_produces_finite_ktx2` and
also runs the production binding projection with
`cargo test --locked --lib lightmap_binding_projection_preserves_primitive_identity_and_uv_rect`
after the full locked test run. These fixtures are synthetic and CPU-only by
design; hardware-dependent Solari parity remains an ignored local acceptance
probe.

Use exact hashes for:

* Atlas topology.
* Primitive-to-page assignments.
* Chart IDs.
* Sample seeds.

Use numeric tolerances, not byte-for-byte texture equality, for final floating-point radiance.

---

# 17. Accuracy Test Scenes

Create small programmatic fixtures rather than relying only on Fallout cells.

## UV tests

* Cube with hard seams.
* Long narrow wall.
* Mixed triangle sizes.
* Mirrored source UV0.
* Degenerate triangles.
* Large composed batch.
* Object exceeding one atlas page.
* Two charts placed near each other.

Validate:

* No UV1 overlap.
* Required padding.
* Every valid triangle mapped.
* No chart outside its tile.
* Correct seam-vertex attribute remapping.
* Stable packing across repeated runs.

## Lighting tests

### White Lambertian test

A white diffuse plane under a known light verifies the `irradiance / π` convention.

Then change the material to red without rebaking. The output should become red because Bevy multiplies the lightmap by runtime diffuse color. This detects accidental receiver-albedo baking.

### Point-light attenuation

Measure texels at known distances and compare them with the shared runtime attenuation function.

### Cornell-box test

Validate:

* Multi-bounce illumination.
* Red/green color bleeding.
* Dark corners becoming brighter with additional bounces.
* Convergence as sample count increases.

### Emissive-panel test

A rectangular emissive surface should illuminate nearby geometry and produce a soft penumbra.

### Alpha-cutout test

A fence or leaf card should cast the correct masked shadow.

### Dynamic-object test

A movable neutral sphere should:

* Receive no surface lightmap.
* Receive the irradiance volume.
* Visually agree with the nearby baked wall.
* Continue to receive reflection-probe specular.

### Double-counting test

Render once with the static runtime lights removed and once with the baked-only configuration. The lightmapped static diffuse should match.

---

# 18. Implementation Milestones

## Milestone 0 — Lighting Contract and Shared Tests

* Establish the baked-only lighting contract.
* Add white-Lambertian, point-light, and Cornell-box fixtures.
* Refactor light-unit conversion.
* Pass ambient into the bake path.
* Add deterministic seeds.
* Add shared material sampling.

**Result:** Existing volume baking becomes more physically consistent before lightmaps are introduced.

### Shipped amendments — 2026-08-04

* Added the Bevy-free `bevyout-core::lighting` contract for point-light
  intensity conversion, sRGB-to-linear conversion, ambient irradiance, and
  cell directional-light validation.
* Threaded prepared ambient lighting into Rust irradiance and reflection-probe
  tracing, with deterministic scene/sample seeds independent of Rayon order.
* Extracted shared CPU material sampling and added white-Lambertian, escaped
  ambient, material, and deterministic-sampling fixtures.
* Added numeric explicit/fallback point-light parity and a colored-wall
  Cornell-box fixture for direct-transport regression coverage.
* Surface and volume ray origins now use bounded scale-aware offsets derived
  from coordinate precision plus local triangle extent, with NaN-safe fallback
  and a translated close-blocker regression test.
* Viewer point lights now preserve prepared intensity values when the runtime
  lighting scale changes. Bake and reflection-probe revisions were bumped for
  the changed lighting meaning.

### Shipped amendments — 2026-08-05

* Added a small in-tree KTX2 writer for linear RGBA16F surface atlases and
  RGB9E5 3D irradiance volumes. Both outputs write one explicit level and are
  parser-validated without invoking KTX-Software; the bake revision records
  the new container semantics.
* Point-light visibility now measures its shadow-ray limit from the shifted
  ray origin, with an oblique blocker regression covering the scale-aware
  bias path.
* The Bevy irradiance-volume shader now imports the lightmapped-diffuse flag
  it consumes. A bounded real-cell viewer load completed without the prior
  shader compilation error.

## Milestone 1 — UV1 and Runtime Binding

* Vendor xatlas.
* Add UV1 to `ComposedPrimitive`.
* Remap duplicated vertices.
* Emit GLB `TEXCOORD_1`.
* Add primitive binding IDs.
* Extend `PreparedBake`.
* Attach prepared lightmap pages in Bevy through stable primitive extras.

**Result:** Correct native UV and manifest pipeline on all three operating systems.

### Shipped amendments — 2026-08-04

* Vendored upstream xatlas behind `crates/bevyout-xatlas`, with deterministic
  seam-vertex remapping for every composed vertex attribute.
* Composed primitives now emit normalized UV1, generated page dimensions,
  stable binding IDs, and GLB primitive/node extras; the prepared bake manifest
  records page and binding metadata.
* The viewer resolves each binding to its prepared KTX2 atlas page and attaches
  Bevy `Lightmap` components after the GLTF scene creates mesh entities.

## Milestone 2 — Direct Surface Lighting

* Generate the CPU texel map.
* Trace direct visibility.
* Bake point, spot, and directional diffuse.
* Encode HDR KTX2.
* Disable runtime duplicate diffuse.
* Add chart-aware dilation.

### Shipped amendments — 2026-08-04

* Added deterministic UV-space rasterization with barycentric position and
  shading-normal reconstruction, conservative mask handling, and per-primitive
  RGBA16F pages.
* Interior texels use one center evaluation while UV-boundary texels resolve
  deterministic 2x2 quarter-pixel coverage, keeping transport cost focused on
  actual chart edges.
* Surface texels store shared direct incident irradiance divided by PI, so
  runtime material albedo remains authoritative; point and directional
  visibility uses the existing BVH transport path.
* xatlas chart IDs now survive UV remapping into the bake-only primitive data;
  rasterization rejects cross-chart triangles/overlap and performs conservative
  chart-aware dilation within the configured padding radius without bridging
  neighboring chart fronts.
* Deterministic shelf packing now combines the primitive pages into one or more
  RGBA16F atlas pages with gutter texels; `PreparedLightmapBinding::uv_rect`
  records each primitive's atlas region while UV1 remains local to that region.
* Pages are encoded and parser-validated in Rust as linear
  `R16G16B16A16_SFLOAT` single-mip KTX2 files; the in-tree writer is valid for
  NPOT atlas dimensions and recorded with content hashes and one-to-one
  bindings. The irradiance-volume RGB9E5 export uses the same in-tree KTX2
  writer with explicit 3D level metadata.
* Runtime global ambient, irradiance-volume diffuse, and static point/
  directional diffuse are excluded from lightmapped meshes when pages exist;
  dynamic/non-lightmapped meshes retain those sources.
* Prepared `LIGH` records now retain the FO3 spot flag (`0x200`), falloff
  exponent, and full-cone FOV (degrees in the source record, radians in the
  prepared manifest). CPU transport applies the oriented cone and falloff;
  the viewer creates Bevy `SpotLight` entities with the same `-Z` orientation
  and cone limits. Static prepared/native cubemap shadows remain explicitly
  point-only until a separate cone-shadow artifact exists.

* The surface path now adds four deterministic cosine-weighted samples and two
  secondary diffuse bounces by default. Each covered texel now uses a deterministic
  Welford RGB estimator with configurable minimum/maximum samples and a
  relative-variance stopping rule; the reduced development preset is 4..32
  samples and records adaptive telemetry in the bake log.

### Shipped amendments — 2026-08-05

* Covered texels now pass through a deterministic feature-guided À-Trous
  denoiser before dilation. Chart ownership and invalid texels are hard
  barriers; position, normal, material, relative variance, coverage, and
  luminance guide the filter. `--lightmap-denoise-iterations 0` disables it and
  the default is one pass; the selected revision is recorded in prepared bake
  metadata.
* The current surface output remains one-mip uncompressed RGBA16F KTX2. The
  raw pre-denoise accumulation cache can resume completed tiles, and
  primitive-scoped light fingerprints preserve unaffected pages when distant
  point/spot lights change. Optional Zstd compression remains subsequent work.
* Surface transport depth is now controlled by `--lightmap-bounces 0..8`,
  defaults to two, and is recorded in both the bake job and prepared bake
  settings. The recursive transport and two-bounce emissive fixture remain
  deterministic at the default.

**Result:** Usable direct plus configurable recursive baked lighting and static
shadows.

## Milestone 3 — Multi-Bounce GI

### Shipped amendments — 2026-08-04

* Surface pages call the shared CPU transport with a deterministic scene seed,
  four cosine-weighted samples, and the configured number of secondary diffuse
  bounces (two by default).
* Secondary hits use the same material/light policy as direct transport and
  include emissive material contribution; escaped indirect rays do not add the
  ambient environment a second time.
* A two-bounce emissive-panel fixture proves indirect transfer through a
  diffuse intermediate surface.

### Shipped amendments — 2026-08-05

* The CPU transport now builds a deterministic emitted-power distribution over
  emissive triangles and performs one explicit area-light next-event sample at
  each diffuse transport vertex. The estimator accounts for triangle area,
  selection probability, exact solid-angle PDFs, both-sided emission, alpha
  policy, and shadow visibility. A power-heuristic MIS weight combines this
  sample with cosine-path emitter hits, so small emitters do not depend on a
  cosine ray landing on them and overlapping paths are not double-counted.
* Russian roulette starts after the third transport bounce, uses a stable path
  variate, and divides surviving paths by their survival probability. The
  transport/integrator revisions were bumped so old accumulation tiles cannot
  be reused under the new estimator.
* Every primitive now persists a pre-denoise
  `lightmap-variance-XXXX.r32f.raw` page. Covered texels contain relative
  variance of the adaptive mean; uncovered texels contain NaN. These pages are
  now first-class `PreparedBake.lightmap_variance_pages` artifacts with stable
  primitive keys, dimensions, coverage counts, and content hashes. Authored
  environment-map sampling is now additive to the scalar cell-ambient
  contract: escaped surface/volume rays read HDR radiance, while receivers
  use deterministic cosine-weighted environment irradiance.
* Authored environment maps now also build a deterministic
  luminance/solid-angle distribution. Direct receiver irradiance combines
  cosine-BSDF and environment samples with a power-heuristic MIS weight, and
  both strategies use the existing alpha-aware visibility path. The
  integrator and bake revisions were bumped so prior accumulation tiles cannot
  be reused under the changed estimator.

**Result:** Actual global illumination and color bleeding rather than only direct surface radiance.

## Milestone 4 — Production Denoising and Caching

### Shipped amendments — 2026-08-05

* The first in-tree feature-guided À-Trous denoiser is shipped and runs before
  chart-aware dilation, with deterministic iteration control and chart,
  position, normal, material, variance, coverage, and luminance guidance.
* Raw pre-denoise surface transport is now cached as sparse, atomically
  published per-primitive tiles under each baked scene. The shared cache
  identity covers prepared geometry, content-addressed GLB bytes,
  transport/sampling settings, seed, and tile layout; each primitive adds the
  signatures of its relevant point/spot lights. Denoiser and KTX encoding stay
  outside the cache identity. A real-cell bake wrote 223 tiles in about 75.7
  seconds; the identical repeat reused all 223 in about 7.7 seconds.
* Global `--lightmap-texels-per-meter` and repeated
  `--lightmap-density FORM_ID=TEXELS_PER_METER` controls are shipped. Density
  participates in primitive batching, xatlas chart generation, binding metadata,
  and cache identity; duplicate or unknown FormID overrides fail validation.
* Primitive-scoped light fingerprints now provide partial invalidation for
  distant point/spot-light edits; the bake revision and cache format were
  bumped so existing whole-scene caches are rebuilt once under the new model.
* `--lightmap-debug-uv`, `--lightmap-debug-samples`, and
  `--lightmap-debug-variance` now write deterministic per-primitive PNGs under
  `baked/lightmaps/debug/`. They visualize chart ownership/coverage, adaptive
  sample counts, and normalized relative variance before denoising and dilation.
* The raw variance pages are retained as published bake artifacts rather than
  treated as untracked debug output; stale manifests are rejected by the bumped
  bake and manifest revisions.

**Result:** Production-quality and practical rebake iteration.

## Milestone 5 — Irradiance-Volume Unification

### Shipped amendments — 2026-08-04

* Irradiance-volume probe rays now use the same surface transport entry point
  as lightmap texels, including the shared material policy, scale-aware ray
  offsets, deterministic seed convention, and fixed two-bounce diffuse path.
* The volume retains its own probe sampling density and ambient escape behavior.

### Shipped amendments — 2026-08-05

* The volume output is now produced by the in-tree RGB9E5 KTX2 writer rather
  than an external executable. Runtime loading was exercised on the real
  `000151e3` scene after the shader import fix.
* A single runtime lightmap policy now drives prepared-scene startup, active
  cell refresh, day/night refresh, and prepared point/spot lights. When a bake
  contains surface pages, ambient, irradiance-volume, and static direct-light
  diffuse are excluded from lightmapped meshes while dynamic/non-lightmapped
  meshes retain those sources; the CPU BVH still keeps the geometry as
  transport geometry.
* A focused transport fixture compares the surface-lightmap incident
  irradiance with the irradiance-volume ray-hit path after the same sRGB
  material conversion and Lambertian division.
* Live acceptance completed on the real `000151e3` viewer scene: the bridge
  reported `GlobalAmbientLight.affects_lightmapped_meshes = false`, loaded 24
  lightmap atlas entities, and rendered dynamic `HD00MrHandyWadsworth`
  alongside the baked interior architecture in the captured scene view. The
  remaining limitation is visual/manual comparison rather than an unverified
  runtime binding.

**Result:** Static architecture and dynamic actors occupy the same lighting solution.

## Milestone 6 — Optional GPU Backend

Only after the CPU version, atlas layout, cache format, and material/light
contract are correct. The GPU backend is now Solari-backed rather than a
custom BVH/traversal implementation:

### Shipped amendments — 2026-08-05

* Added the backend-selection seam and `--bake-backend auto|cpu|solari` CLI
  contract. `cpu` remains the default and `auto` currently resolves to it.
  Feature-enabled explicit `solari` requests now enter the adapter; builds
  without `lightmap-gpu-solari` reject them deterministically. The default
  build still does not enable `bevy_solari`.
* Added the opt-in `lightmap-gpu-solari` feature and compiled it against the
  pinned Bevy 0.19 API. The prototype boundary uses
  `SolariPlugins::required_wgpu_features()`, exposes the public
  `RaytracingScenePlugin` BLAS/TLAS seam, and builds UV1-free proxy meshes with
  Solari's exact `POSITION/NORMAL/UV_0/TANGENT` contract. No realtime ReSTIR
  plugin is enabled.
* Added a bake-only `SolariBakePlugin` and `solari_bake.wgsl` compute node.
  It dispatches over explicit world-position/normal UV texels, consumes
  bevyout-owned point, spot, ambient, and directional lighting records, uses Solari
  ray queries for visibility, and schedules a `map_buffer_on_submit` readback
  into a shared result slot. The headless session reuses the scene across
  bounded lightmap tiles; cache hits bypass dispatch and misses write the same
  pre-denoise tile payload consumed by the CPU path.
* The ignored hardware acceptance test was run successfully on the current
  compatible adapter: BLAS/TLAS creation, shader compilation, ray-query
  dispatch, and GPU readback all completed for a direct-light triangle fixture.
  The test remains ignored for cross-machine CI because ray-query support is
  hardware-dependent.
* Added a second ignored hardware fixture comparing five deterministic texels
  on the same point-light triangle against the CPU direct-irradiance reference,
  then repeating the comparison for the matching spotlight cone. The fixture
  records maximum and mean RGB error under a documented `1e-3` maximum
  tolerance; the current adapter passed. It remains a parity probe, not yet a
  release gate for every GPU family.
* Solari proxy materials now come from the composed scene rather than an
  unconditional white placeholder. Opaque base-color factors and authored
  base-color textures are projected into Bevy's Solari material table, and the
  bake shader resolves the first hit through Solari's material bindings.
  Explicit `--lightmap-bounces 1` now evaluates one deterministic cosine
  diffuse bounce and applies the secondary hit's diffuse albedo while keeping
  the receiver albedo out of the stored lightmap. An ignored hardware fixture
  compares that path against the CPU reference on a two-surface material
  fixture; the current adapter passed.
* Tightened baked receiver and prepared point-shadow eligibility: a placement
  must be both `PreparedPhysicsClassification::Static` and
  `PreparedRuntimeMutability::Immutable`; kinematic, enable-group,
  script-addressable, and unknown placements cannot leave baked lighting or
  shadow silhouettes behind.
* Added Solari alpha-mask traversal. The adapter now uploads a primitive-ordered
  alpha side table plus bilinearly sampled base-color alpha texels, skips masked
  ray hits below the authored cutoff for direct and one-bounce visibility, and
  keeps the conservative opaque fallback when the side table is absent. A
  factor-mask hardware fixture is included.
* Added blended alpha transport to the Solari side table. Blend materials now
  upload factor and base-color texture alpha, accumulate scalar transmittance
  through bounded visibility layers, and continue the same path through a
  partially covered hit instead of treating it as opaque. A hardware fixture
  compares alpha `0`, `0.5`, and `1` against the CPU reference.
* Extended the deterministic cosine path from one to four bounded secondary
  diffuse bounces. Each path carries the resolved hit albedo into the next
  bounce, skips alpha-masked layers at every depth, and preserves the existing
  zero-bounce direct result. A three-plane ambient fixture compares one and two
  bounces against the CPU linear-albedo reference on the current adapter.
* Added a bevyout-owned emissive-triangle table for explicit Solari bounces.
  Triangles are selected from an emitted-power CDF, sampled uniformly in area,
  converted to a solid-angle PDF, visibility-tested through the same alpha
  transmittance path, and combined with the cosine estimator using a power
  heuristic. Hit-emission MIS uses the actual path origin and BSDF PDF rather
  than Solari's area-measure helper. The current adapter stores a center sample
  per emitter; textured or strongly vertex-varying emitters still need a more
  detailed texture-aware sampler.
* Added authored equirectangular HDR environment transport. Environment pixels
  are copied into a render-owned storage buffer; the shader preserves horizontal
  wrap and vertical clamp, evaluates a deterministic cosine irradiance estimate
  with alpha-aware visibility, and adds environment radiance when indirect paths
  escape. A constant-map hardware oracle matches the CPU `radiance × PI`
  contract for direct and one-bounce escape lighting.
* A real-cell GPU acceptance bake completed for `000151e3` with the explicit
  Solari backend (`--lightmap-min-samples 1 --lightmap-max-samples 1
  --lightmap-variance-threshold 0 --lightmap-bounces 1
  --lightmap-denoise-iterations 0`). It composed 65 static placements into 23
  primitive pages, packed one `3996 × 3980` single-mip RGBA16F atlas, covered
  772,053 texels, and completed in 28.91 seconds. The bake reported 746 GPU
  tile misses/writes and no cache hits, proving dispatch, readback, cache
  publication, atlas encoding, and manifest publication on a real prepared
  cell rather than only on the ignored synthetic fixtures. The same command
  also produced the existing shared CPU irradiance-volume artifact; that volume
  is not being misreported as Solari output.
* The live viewer loaded that prepared result through the local MCP/agent
  bridge. A viewport capture shows the baked interior architecture and dynamic
  `HD00MrHandyWadsworth`; the persistent local capture is
  `.bevyout/screenshots/gpu_bake_solari.png`. This is visual runtime evidence
  for the integrated path, not a cross-adapter parity gate or a production
  quality bake.
* Solari now has an explicit fast default profile when density, tile size, and
  static batch size are omitted: `4` texels/metre, `512`-texel accumulation
  tiles, and `32 m` static batches. CPU/Auto defaults remain `16`, `128`, and
  `64 m`. Higher Solari density remains an explicit quality choice.
* Atlas-size validation now runs immediately after composition, before any
  backend dispatch. It reports the maximum fitting density instead of spending
  minutes tracing an unrepresentable page. Solari readback also waits
  indefinitely for the submitted GPU work rather than treating a normal 50 ms
  wgpu poll timeout as a bake failure.
* The exact fast Solari command completed on real `Tenpenny01` (`00017f34`) in
  `68.04 s` after compilation with `--lightmap-tile-size 512`: `192` primitive
  pages, `4` atlases, `379` GPU tile misses/writes, and the existing CPU
  irradiance volume. This is a successful real-cell acceptance result, not a
  claim that all adapters have the same timing.

### Review correction — 2026-08-05

The Solari 7 review found that the preceding bullets described a prototype as
more physically complete than the code justified. The following corrections are
now part of the implementation, and the older hardware-pass statements above
are historical evidence from the pre-correction shader, not current acceptance
evidence:

* glTF `baseColorFactor`, `emissiveFactor`, and `COLOR_0` are now treated as
  linear values in both CPU and Solari transport. Only sampled color textures
  receive sRGB decoding. The material contract tests were updated accordingly.
* All custom Solari transport and alpha-peel rays use closest-hit semantics
  (`RAY_FLAG_NONE`). Solari proxy identity is carried explicitly through the
  pinned 0.19 `ResolvedMaterial.reflectance` field; alpha, vertex-color, and
  geometric-position side tables no longer assume compacted Solari instance
  order. Geometric normals and an authored double-sided flag control face
  acceptance and ray offsets.
* The upstream Solari emissive sampler/MIS path remains disabled for this bake
  prototype. Its 0.19 PDF is area-measure and cannot be compared directly with
  the shader's solid-angle BSDF PDF; it also cannot apply bevyout alpha masks
  to NEE emitter visibility. The adapter instead owns its solid-angle emitter
  table and MIS, while the CPU reference remains authoritative for richer
  texture-varying emitter sampling.
* Environment radiance is evaluated at the current surface's direct vertex;
  an indirect miss no longer adds it a second time. Constant maps therefore
  use `radiance × PI`, not `radiance × 2PI`. Environment-CDF vertical jitter
  uses the selected CDF interval residual, and cosine-path sampling uses a
  radial jitter plus a global page/primitive/texel/sample seed identity.
* Solari now accepts fixed sample counts greater than one, but still rejects
  adaptive variance stopping. Its dispatch waits for two consecutive prepared
  Solari scene frames before submitting work. The cache fingerprint includes
  the backend identity and the exact `solari_bake.wgsl` hash; the unused
  serialized `BakeJob.emission_scale` field was removed.
* The source-verified lightmap container remains intentionally one-mip and
  accepts NPOT dimensions such as `3996×3980`; power-of-two padding is not a
  requirement. Cross-OS runtime verification of that NPOT one-mip asset is
  still outstanding.
* After these corrections, the ignored Solari matrix passes eight fixtures on
  the current compatible adapter: direct/session, CPU parity, alpha mask,
  blended alpha, constant environment, one and two authored diffuse bounces,
  and emissive-mesh transport. This is adapter-local evidence, not a
  representative-GPU release gate.

This leaves the optional backend an experimental transport prototype. The
feature build, source-contract tests, CPU analytical oracles, and the current
compatible-adapter hardware fixtures are evidence for the implemented slice;
cross-adapter parity and texture-varying emitter coverage remain open.

Still required for the complete optional backend:

* Run the deterministic multi-texel parity fixture across representative GPU
  families and promote it to a release gate once adapter coverage is stable.
* Rerun and then broaden hardware validation: direct, alpha-mask, linear
  material/vertex-color, geometric-sidedness, multi-sample, environment, and
  bounded-bounce fixtures must pass on representative adapters. The current
  two-frame readiness fence is conservative but does not yet read back Solari's
  compact instance counts.
* Complete the remaining full-lighting-contract parity work. The current GPU
  path supports zero through four bounded diffuse bounces, opaque/mask/blend
  alpha transport, a bevyout-owned solid-angle emissive sample with MIS, and
  direct authored-environment transport. It does not yet claim CPU-equivalent
  texture-varying emitter sampling, production-scale emitter lookup cost, or
  adaptive convergence.
* Add stronger analytical fixtures for constant environments, emissive panels,
  sidedness, texture/factor color spaces, and non-ring sample distributions;
  keep the CPU path authoritative until those fixtures are hardware-verified.
* Add capability-aware `Auto` selection only after GPU/CPU parity is proven;
  until then `Auto` stays on CPU and explicit `Solari` remains opt-in.
* Keep the Solari adapter's material/light policy narrow and bevyout-owned; do
  not reuse camera-space path tracing, exposure, temporal reservoirs, or the
  real-time ReSTIR path as bake authority.

**Result:** Faster baking on capable adapters while preserving deterministic,
cross-platform CPU baking and without maintaining a custom GPU acceleration
structure.

---

# Definition of Done

The implementation is complete when:

* A cell can be prepared and baked without Blender or Python.
* The same command works on Windows, macOS, and Linux.
* Generated GLB meshes contain valid `TEXCOORD_1`.
* Only immutable static receivers enter surface atlases.
* Lightmaps store linear HDR diffuse-light factors without receiver albedo.
* Static point, spot, directional, ambient, and volume diffuse are not double-counted.
* Dynamic objects use the retained irradiance volume.
* Reflection probes continue to provide static and dynamic specular.
* Direct shadows, emissive lighting, and multi-bounce color bleeding pass reference tests.
* UV packing is deterministic.
* Interrupted bakes can resume by tile.
* The final production path does not require an external KTX executable.
* CI runs a real miniature bake on all three operating systems.
* If the optional GPU backend is enabled, it uses Solari scene infrastructure,
  never a second custom BVH or camera-projected lightmap path.

The central architectural decision should be:

> **Make the existing deterministic Rust baker the authoritative transport system. Add surface lightmaps as a second output of that system, retain irradiance volumes for dynamic objects, and treat the GPU implementation as an optional acceleration backend rather than a platform requirement.**

The Solari-specific form of that decision is:

> **Reuse Solari for GPU scene/ray infrastructure, but keep bevyout's UV-texel
> offline integrator, Fallout lighting policy, postprocessing, cache, output,
> runtime binding, and CPU fallback. Do not use Solari's real-time ReSTIR path
> or camera-space path tracer unchanged as the lightmap baker.**

[1]: https://github.com/kelo221/bevyout "https://github.com/kelo221/bevyout"
[2]: https://raw.githubusercontent.com/bevyengine/bevy/v0.19.0/crates/bevy_pbr/src/render/pbr_functions.wgsl "https://raw.githubusercontent.com/bevyengine/bevy/v0.19.0/crates/bevy_pbr/src/render/pbr_functions.wgsl"
[3]: https://raw.githubusercontent.com/kelo221/bevyout/master/src/vsa/bake/rust_irradiance.rs "https://raw.githubusercontent.com/kelo221/bevyout/master/src/vsa/bake/rust_irradiance.rs"
[4]: https://raw.githubusercontent.com/kelo221/bevyout/master/crates/bevyout-core/src/manifest.rs "https://raw.githubusercontent.com/kelo221/bevyout/master/crates/bevyout-core/src/manifest.rs"
[5]: https://github.com/jpcy/xatlas "https://github.com/jpcy/xatlas"
[6]: https://raw.githubusercontent.com/kelo221/bevyout/master/src/vsa/bake/rust_scene.rs "https://raw.githubusercontent.com/kelo221/bevyout/master/src/vsa/bake/rust_scene.rs"
[7]: https://docs.rs/bevy_pbr/0.19.0/src/bevy_pbr/lightmap/mod.rs.html "https://docs.rs/bevy_pbr/0.19.0/src/bevy_pbr/lightmap/mod.rs.html"
[8]: https://github.com/KhronosGroup/KTX-Software "https://github.com/KhronosGroup/KTX-Software"
[9]: https://raw.githubusercontent.com/kelo221/bevyout/master/src/vsa/bake/mod.rs "https://raw.githubusercontent.com/kelo221/bevyout/master/src/vsa/bake/mod.rs"
[10]: https://raw.githubusercontent.com/bevyengine/bevy/v0.19.0/crates/bevy_pbr/src/lightmap/lightmap.wgsl "https://raw.githubusercontent.com/bevyengine/bevy/v0.19.0/crates/bevy_pbr/src/lightmap/lightmap.wgsl"
[11]: https://raw.githubusercontent.com/kelo221/bevyout/master/src/viewer/scene.rs "https://raw.githubusercontent.com/kelo221/bevyout/master/src/viewer/scene.rs"
[12]: https://github.com/bevyengine/bevy/pull/24767 "Solari v7 by JMS55"
[13]: https://github.com/bevyengine/bevy/issues/20203 "Solari tracking issue"
