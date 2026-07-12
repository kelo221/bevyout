[bevy](../../index.html)::[pbr](../index.html)

# Module irradiance\_volume 

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/light_probe/mod.rs.html#46)

Irradiance volumes, also known as voxel global illumination.

An _irradiance volume_ is a cuboid voxel region consisting of regularly-spaced precomputed samples of diffuse indirect light. They’re ideal if you have a dynamic object such as a character that can move about static non-moving geometry such as a level in a game, and you want that dynamic object to be affected by the light bouncing off that static geometry.

To use irradiance volumes, you need to precompute, or _bake_, the indirect light in your scene. Bevy doesn’t currently come with a way to do this. Fortunately, [Blender](http://blender.org/) provides a [baking tool](https://docs.blender.org/manual/en/latest/render/eevee/light_probes/volume.html) as part of the Eevee renderer, and its irradiance volumes are compatible with those used by Bevy. The [`bevy-baked-gi`](https://github.com/pcwalton/bevy-baked-gi) project provides a tool, `export-blender-gi`, that can extract the baked irradiance volumes from the Blender `.blend` file and package them up into a `.ktx2` texture for use by the engine. See the documentation in the `bevy-baked-gi` project for more details on this workflow.

Like all light probes in Bevy, irradiance volumes are 1×1×1 cubes, centered on the origin, that can be arbitrarily scaled, rotated, and positioned in a scene with the [`bevy_transform::components::Transform`](../../prelude/struct.Transform.html "struct bevy::prelude::Transform") component. The 3D voxel grid will be stretched to fill the interior of the cube, with linear interpolation, and the illumination from the irradiance volume will apply to all fragments within that bounding region.

Bevy’s irradiance volumes are based on Valve’s [_ambient cubes_](https://advances.realtimerendering.com/s2006/Mitchell-ShadingInValvesSourceEngine.pdf) as used in _Half-Life 2_ ([Mitchell 2006, slide 27](https://advances.realtimerendering.com/s2006/Mitchell-ShadingInValvesSourceEngine.pdf#page=27)). These encode a single color of light from the six 3D cardinal directions and blend the sides together according to the surface normal. For an explanation of why ambient cubes were chosen over spherical harmonics, see [Why ambient cubes?](#why-ambient-cubes) below.

If you wish to use a tool other than `export-blender-gi` to produce the irradiance volumes, you’ll need to pack the irradiance volumes in the following format. The irradiance volume of resolution _(Rx, Ry, Rz)_ is expected to be a 3D texture of dimensions _(Rx, 2Ry, 3Rz)_. The unnormalized texture coordinate _(s, t, p)_ of the voxel at coordinate _(x, y, z)_ with side _S_ ∈ _{-X, +X, -Y, +Y, -Z, +Z}_ is as follows:

```
s = x

t = y + ⎰  0 if S ∈ {-X, -Y, -Z}
        ⎱ Ry if S ∈ {+X, +Y, +Z}

        ⎧   0 if S ∈ {-X, +X}
p = z + ⎨  Rz if S ∈ {-Y, +Y}
        ⎩ 2Rz if S ∈ {-Z, +Z}
```

Visually, in a left-handed coordinate system with Y up, viewed from the right, the 3D texture looks like a stacked series of voxel grids, one for each cube side, in this order:

| **+X** | **+Y** | **+Z** |
| --- | --- | --- |
| **\-X** | **\-Y** | **\-Z** |

A terminology note: Other engines may refer to irradiance volumes as _voxel global illumination_, _VXGI_, or simply as _light probes_. Sometimes _light probe_ refers to what Bevy calls a reflection probe. In Bevy, _light probe_ is a generic term that encompasses all cuboid bounding regions that capture indirect illumination, whether based on voxels or not.

Note that, if binding arrays aren’t supported (e.g. on WebGPU or WebGL 2), then only the closest irradiance volume to the view will be taken into account during rendering. The required `wgpu` features are [`bevy_render::settings::WgpuFeatures::TEXTURE_BINDING_ARRAY`](../../render/render_resource/struct.WgpuFeatures.html#associatedconstant.TEXTURE_BINDING_ARRAY "associated constant bevy::render::render_resource::WgpuFeatures::TEXTURE_BINDING_ARRAY") and [`bevy_render::settings::WgpuFeatures::SAMPLED_TEXTURE_AND_STORAGE_BUFFER_ARRAY_NON_UNIFORM_INDEXING`](../../render/render_resource/struct.WgpuFeatures.html#associatedconstant.SAMPLED_TEXTURE_AND_STORAGE_BUFFER_ARRAY_NON_UNIFORM_INDEXING "associated constant bevy::render::render_resource::WgpuFeatures::SAMPLED_TEXTURE_AND_STORAGE_BUFFER_ARRAY_NON_UNIFORM_INDEXING").

### Why ambient cubes?

This section describes the motivation behind the decision to use ambient cubes in Bevy. It’s not needed to use the feature; feel free to skip it unless you’re interested in its internal design.

Bevy uses _Half-Life 2_\-style ambient cubes (usually abbreviated as _HL2_) as the representation of irradiance for light probes instead of the more-popular spherical harmonics (_SH_). This might seem to be a surprising choice, but it turns out to work well for the specific case of voxel sampling on the GPU. Spherical harmonics have two problems that make them less ideal for this use case:

1.  The level 1 spherical harmonic coefficients can be negative. That prevents the use of the efficient [RGB9E5 texture format](https://www.khronos.org/opengl/wiki/Small_Float_Formats#RGB9_E5), which only encodes unsigned floating point numbers, and forces the use of the less-efficient [RGBA16F format](https://www.khronos.org/opengl/wiki/Small_Float_Formats#Low-bitdepth_floats) if hardware interpolation is desired.
    
2.  As an alternative to RGBA16F, level 1 spherical harmonics can be normalized and scaled to the SH0 base color, as [Frostbite](https://media.contentapi.ea.com/content/dam/eacom/frostbite/files/gdc2018-precomputedgiobalilluminationinfrostbite.pdf#page=53) does. This allows them to be packed in standard LDR RGBA8 textures. However, this prevents the use of hardware trilinear filtering, as the nonuniform scale factor means that hardware interpolation no longer produces correct results. The 8 texture fetches needed to interpolate between voxels can be upwards of twice as slow as the hardware interpolation.
    

The following chart summarizes the costs and benefits of ambient cubes, level 1 spherical harmonics, and level 2 spherical harmonics:

| Technique | HW-interpolated samples | Texel fetches | Bytes per voxel | Quality |
| --- | --- | --- | --- | --- |
| Ambient cubes | 3 | 0 | 24 | Medium |
| Level 1 SH, compressed | 0 | 36 | 16 | Low |
| Level 1 SH, uncompressed | 4 | 0 | 24 | Low |
| Level 2 SH, compressed | 0 | 72 | 28 | High |
| Level 2 SH, uncompressed | 9 | 0 | 54 | High |

(Note that the number of bytes per voxel can be reduced using various texture compression methods, but the overall ratios remain similar.)

From these data, we can see that ambient cubes balance fast lookups (from leveraging hardware interpolation) with relatively-small storage requirements and acceptable quality. Hence, they were chosen for irradiance volumes in Bevy.