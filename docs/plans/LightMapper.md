# Recommended Direction

The pasted plan is reasonable for a standalone generic lightmapper, but it should be adjusted substantially for **bevyout**.

The repository already has:

* A native Rust scene-composition pipeline.
* A CPU BVH-based irradiance baker.
* Deterministic static batching.
* KTX output support.
* An explicit goal of removing Blender from the supported asset path.

The better design is therefore to **extend the current Rust baker into a shared surface-lightmap and irradiance-volume renderer**, rather than building a separate GPU-first subsystem beside it. ([GitHub][1])

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

## Important Changes From the Original Plan

| Original proposal                                   | Recommended replacement                                                          |
| --------------------------------------------------- | -------------------------------------------------------------------------------- |
| GPU compute path tracer as the primary baker        | Deterministic CPU reference backend first; optional GPU backend later            |
| UV-space position and normal G-buffer               | CPU-generated compact texel map with triangle IDs and barycentrics               |
| One generic lightmap texture attached to every mesh | Multiple atlases with stable per-primitive bindings and `uv_rect`                |
| Bake final material-colored radiance                | Bake the diffuse illumination factor without the receiver’s albedo               |
| Surface lightmaps replacing the current baker       | Surface lightmaps plus the existing irradiance-volume system                     |
| Old `xatlas-rs` dependency                          | Vendored upstream xatlas with a thin maintained Rust FFI                         |
| Basis Universal as the default HDR format           | `RGBA16F + Zstd`, with RGB9E5 as an optional compact format                      |
| OIDN as the production requirement                  | In-tree cross-platform denoiser; OIDN only as an optional feature                |
| Required external `toktx` process                   | Initially supported fallback, eventually replaced by a small in-tree KTX2 writer |

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

The existing baker should be refactored before surface lightmaps are added.

At present, the six-lobe volume samples visible surface radiance, but that surface radiance contains only direct point/directional light plus emissive contribution. There is no recursive multi-bounce transport. The bake job also carries `intensity_lumens`, while the current point-light calculation derives strength from radius and a hard-coded scale. Cell ambient is present in the bake job but is not passed into the current irradiance-bake call. ([GitHub][3])

Address these in the shared transport layer:

1. Replace the hard-coded lighting scale with a common photometric conversion used by both the baker and runtime.
2. Pass ambient/environment data into the integrator.
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

Add per-placement or per-model overrides:

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
* The same texel map can later be uploaded to an optional GPU integrator.

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

# 7. Shared Ray-Tracing and Material Layer

Refactor the reusable portions of `rust_irradiance.rs` into a shared transport module.

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
Feature-guided denoise
        ▼
Coverage resolve
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

The existing pipeline already invokes the unified Khronos `ktx` tool and emits an RGB9E5-style HDR texture. That can remain as a transitional fallback while the baker gains an in-tree KTX2 writer. ([GitHub][9])

## Cross-platform encoding plan

### Phase 1

Support:

```text
Internal raw HDR buffer
    ├── External Khronos ktx executable, when configured
    └── Uncompressed/debug output for tests
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
    pub lightmap_bindings: Vec<PreparedLightmapBinding>,
    pub irradiance_volume: Option<PreparedIrradianceVolume>,
    pub bake_settings: PreparedBakeSettings,
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

The current viewer creates static point/directional lighting that affects lightmapped diffuse and enables ambient/volume participation on lightmapped meshes. Those settings must change once direct and indirect diffuse are stored in the surface texture, or static surfaces will be lit more than once. ([GitHub][11])

Reflection probes can remain active for specular. The current reflection-probe configuration already avoids applying its diffuse contribution to lightmapped meshes. ([GitHub][11])

---

# 13. Proposed Source Layout

```text
src/vsa/bake/
├── mod.rs
├── settings.rs
├── fingerprint.rs
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

Repository-specific edits:

| File or area                          | Change                                                             |
| ------------------------------------- | ------------------------------------------------------------------ |
| `src/vsa/bake/rust_irradiance.rs`     | Split shared tracing and keep only volume-specific integration     |
| `src/vsa/bake/rust_scene.rs`          | Add UV1, vertex remapping, stable primitive keys, GLB `TEXCOORD_1` |
| `crates/bevyout-core/src/manifest.rs` | Add atlas and binding structures                                   |
| `src/viewer/scene.rs`                 | Attach lightmaps and disable duplicate diffuse sources             |
| `src/cli.rs`                          | Add lightmap settings and quality presets                          |
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
--lightmap-debug-uv
--lightmap-debug-samples
--lightmap-debug-variance
--lightmap-resume
--lightmap-force-repack
--lightmap-force-retrace
```

Keep the CPU backend as the default and reference implementation:

```text
--bake-backend cpu
```

An optional later backend can use:

```text
--bake-backend gpu
```

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
* Increasing sample count without discarding previous accumulation.
* Re-encoding without retracing.
* Re-denoising without rebaking.
* Reusing UV layouts after lighting changes.
* Rebaking only affected atlas pages or spatial tiles.

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

## Milestone 1 — UV1 and Runtime Binding

* Vendor xatlas.
* Add UV1 to `ComposedPrimitive`.
* Remap duplicated vertices.
* Emit GLB `TEXCOORD_1`.
* Add primitive binding IDs.
* Extend `PreparedBake`.
* Attach a diagnostic checkerboard lightmap in Bevy.

**Result:** Correct native UV and manifest pipeline on all three operating systems.

## Milestone 2 — Direct Surface Lighting

* Generate the CPU texel map.
* Trace direct visibility.
* Bake point, spot, and directional diffuse.
* Encode HDR KTX2.
* Disable runtime duplicate diffuse.
* Add chart-aware dilation.

**Result:** Usable noise-free direct baked lighting and static shadows.

## Milestone 3 — Multi-Bounce GI

* Add recursive diffuse transport.
* Add emissive-triangle sampling.
* Add environment/cell ambient.
* Add Russian roulette.
* Add adaptive sampling.
* Add variance output.

**Result:** Actual global illumination and color bleeding rather than only direct surface radiance.

## Milestone 4 — Production Denoising and Caching

* Implement guided À-Trous denoising.
* Add tile accumulation cache.
* Add resume support.
* Add partial invalidation.
* Add debug visualizations.
* Add per-model density overrides.

**Result:** Production-quality and practical rebake iteration.

## Milestone 5 — Irradiance-Volume Unification

* Move volume baking to the shared integrator.
* Match material and light interpretation.
* Exclude lightmapped geometry as volume receivers while retaining it as transport geometry.
* Validate dynamic/static agreement.

**Result:** Static architecture and dynamic actors occupy the same lighting solution.

## Milestone 6 — Optional GPU Backend

Only after the CPU version is correct:

* Upload the same texel map and flattened BVH to wgpu.
* Implement compute traversal and accumulation.
* Keep the CPU backend as fallback and correctness reference.
* Compare GPU output statistically against CPU fixtures.
* Avoid hardware ray-tracing APIs.

**Result:** Faster baking where compute support is suitable, without sacrificing Windows/macOS/Linux availability.

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

The central architectural decision should be:

> **Make the existing deterministic Rust baker the authoritative transport system. Add surface lightmaps as a second output of that system, retain irradiance volumes for dynamic objects, and treat the GPU implementation as an optional acceleration backend rather than a platform requirement.**

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
