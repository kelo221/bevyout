[bevy](../../index.html)::[anti\_alias](../index.html)

# Module smaa 

[Source](https://docs.rs/bevy_anti_alias/0.19.0/x86_64-unknown-linux-gnu/src/bevy_anti_alias/lib.rs.html#18)

Subpixel morphological antialiasing (SMAA).

[SMAA](https://www.iryoku.com/smaa/) is a 2011 antialiasing technique that takes an aliased image and smooths out the _jaggies_, making edges smoother. It’s been used in numerous games and has become a staple postprocessing technique. Compared to MSAA, SMAA has the advantage of compatibility with deferred rendering and reduction of GPU memory bandwidth. Compared to FXAA, SMAA has the advantage of improved quality, but the disadvantage of reduced performance. Compared to TAA, SMAA has the advantage of stability and lack of _ghosting_ artifacts, but has the disadvantage of not supporting temporal accumulation, which have made SMAA less popular when advanced photorealistic rendering features are used in recent years.

To use SMAA, add [`Smaa`](struct.Smaa.html "struct bevy::anti_alias::smaa::Smaa") to a [`bevy_camera::Camera`](../../prelude/struct.Camera.html "struct bevy::prelude::Camera"). In a pinch, you can simply use the default settings (via the [`Default`](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") trait) for a high-quality, high-performance appearance. When using SMAA, you will likely want set [`bevy_render::view::Msaa`](../../prelude/enum.Msaa.html "enum bevy::prelude::Msaa") to [`bevy_render::view::Msaa::Off`](../../prelude/enum.Msaa.html#variant.Off "variant bevy::prelude::Msaa::Off") for every camera using SMAA.

Those who have used SMAA in other engines should be aware that Bevy doesn’t yet support the following more advanced features of SMAA:

*   The temporal variant.
    
*   Depth- and chroma-based edge detection.
    
*   Predicated thresholding.
    
*   Compatibility with SSAA and MSAA.
    

## Structs

[Smaa](struct.Smaa.html "struct bevy::anti_alias::smaa::Smaa")

A component for enabling Subpixel Morphological Anti-Aliasing (SMAA) for a [`bevy_camera::Camera`](../../prelude/struct.Camera.html "struct bevy::prelude::Camera").

[SmaaBindGroups](struct.SmaaBindGroups.html "struct bevy::anti_alias::smaa::SmaaBindGroups")

A render world component that stores the bind groups necessary to perform SMAA.

[SmaaInfoUniform](struct.SmaaInfoUniform.html "struct bevy::anti_alias::smaa::SmaaInfoUniform")

Values supplied to the GPU for SMAA.

[SmaaInfoUniformBuffer](struct.SmaaInfoUniformBuffer.html "struct bevy::anti_alias::smaa::SmaaInfoUniformBuffer")

The GPU buffer that holds all [`SmaaInfoUniform`](struct.SmaaInfoUniform.html "struct bevy::anti_alias::smaa::SmaaInfoUniform")s for all views.

[SmaaInfoUniformOffset](struct.SmaaInfoUniformOffset.html "struct bevy::anti_alias::smaa::SmaaInfoUniformOffset")

A render world component that stores the offset of each [`SmaaInfoUniform`](struct.SmaaInfoUniform.html "struct bevy::anti_alias::smaa::SmaaInfoUniform") within the [`SmaaInfoUniformBuffer`](struct.SmaaInfoUniformBuffer.html "struct bevy::anti_alias::smaa::SmaaInfoUniformBuffer") for each view.

[SmaaNeighborhoodBlendingPipelineKey](struct.SmaaNeighborhoodBlendingPipelineKey.html "struct bevy::anti_alias::smaa::SmaaNeighborhoodBlendingPipelineKey")

A unique identifier for a set of SMAA pipelines.

[SmaaPipelines](struct.SmaaPipelines.html "struct bevy::anti_alias::smaa::SmaaPipelines")

A render world resource that holds all render pipeline data needed for SMAA.

[SmaaPlugin](struct.SmaaPlugin.html "struct bevy::anti_alias::smaa::SmaaPlugin")

Adds support for subpixel morphological antialiasing, or SMAA.

[SmaaSpecializedRenderPipelines](struct.SmaaSpecializedRenderPipelines.html "struct bevy::anti_alias::smaa::SmaaSpecializedRenderPipelines")

Stores the specialized render pipelines for SMAA.

[SmaaTextures](struct.SmaaTextures.html "struct bevy::anti_alias::smaa::SmaaTextures")

A render world component that holds the intermediate textures necessary to perform SMAA.

[ViewSmaaPipelines](struct.ViewSmaaPipelines.html "struct bevy::anti_alias::smaa::ViewSmaaPipelines")

A render world component that holds the pipeline IDs for the SMAA passes.

## Enums

[SmaaPreset](enum.SmaaPreset.html "enum bevy::anti_alias::smaa::SmaaPreset")

A preset quality level for SMAA.

## Functions

[init\_smaa\_pipelines](fn.init_smaa_pipelines.html "fn bevy::anti_alias::smaa::init_smaa_pipelines")

[smaa](fn.smaa.html "fn bevy::anti_alias::smaa::smaa")