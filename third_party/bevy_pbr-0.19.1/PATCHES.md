# bevy_pbr 0.19.1 local patch manifest

This directory is a source fork of the crates.io `bevy_pbr` 0.19.1 crate used
by the workspace's `[patch.crates-io]` override.

- Upstream crate: `bevy_pbr` 0.19.1
- Upstream `.crate` SHA-256:
  `244ae7d618b51a59c913c36b0564a295cd85ea91a5e777b542bb7bdb846a23d6`
- Local scope: prepared point-light shadows plus Fallout material and lighting
  behavior. Release builds intentionally compile this fork.

Compare this directory with a clean extraction of that exact crate before an
upgrade. Excluding this manifest, the current fork has 17 source/shader files
that differ from upstream:

| File | Intentional local change |
| --- | --- |
| `src/cluster/cluster_raster.wgsl` | Decode the packed point/spot-light field as the spot tangent angle during cluster rasterization. |
| `src/cluster/mod.rs` | Pack a point light's shadow indices or a spot light's tangent angle into the existing 64-byte GPU light layout. |
| `src/lib.rs` | Export `BakedPointShadowReceiver` and initialize prepared-shadow resources. |
| `src/pbr_material.rs` | Add Fallout surface kind, authored glossiness exponent, and Chan diffuse strength to `StandardMaterial` and its GPU uniform. |
| `src/render/light.rs` | Define, validate, extract, upload, bind, and diagnose prepared D32 point-shadow cubemap arrays; preserve separate prepared and realtime indices and flags. |
| `src/render/mesh.rs` | Project `BakedPointShadowReceiver` into the extracted mesh flags. |
| `src/render/mesh_types.wgsl` | Reserve the baked point-shadow receiver mesh bit. |
| `src/render/mesh_view_bindings.rs` | Add the prepared point-shadow cubemap-array view to the mesh view bind group. |
| `src/render/mesh_view_bindings.wgsl` | Declare the prepared point-shadow depth cube/cube-array binding. |
| `src/render/mesh_view_types.wgsl` | Describe the packed light field and expose distinct prepared and realtime shadow flags. |
| `src/render/pbr_fragment.wgsl` | Transfer Fallout material properties from bind-group data into the PBR input. |
| `src/render/pbr_functions.wgsl` | Select one dominant shadow-capable point light, combine its prepared and realtime visibility with `min`, and keep lighting from other point lights unshadowed. |
| `src/render/pbr_lighting.wgsl` | Keep GGX while adding source-driven hair, eye, skin, and legacy-world direct-light behavior; legacy diffuse uses Chan plus the approved wrapped visibility. |
| `src/render/pbr_types.wgsl` | Extend the shader material structure and defaults with the Fallout properties. |
| `src/render/shadow_sampling.wgsl` | Route every hard/PCF/PCSS cubemap sampling path to either the prepared or realtime depth source. |
| `src/render/shadows.wgsl` | Unpack prepared/realtime cubemap indices and pass the selected source through point-shadow sampling. |
| `src/volumetric_fog/volumetric_fog.wgsl` | Read the packed point/spot-light field while retaining realtime shadow sampling for volumetric fog. |

`src/material.rs` is also a preservation checkpoint, but it is byte-identical
to upstream 0.19.1 rather than an eighteenth local patch. In particular,
`MaterialPropertiesExt::prepass_reads_material` is upstream behavior required
by this fork and must survive rebases unchanged unless the corresponding Bevy
pipeline contract is deliberately updated.

After rebasing the fork, verify the file set with:

```text
git diff --no-index --name-only <clean-bevy_pbr-0.19.1> third_party/bevy_pbr-0.19.1
```
