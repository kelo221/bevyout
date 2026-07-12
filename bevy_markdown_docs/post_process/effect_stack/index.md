[bevy](../../index.html)::[post\_process](../index.html)

# Module effect\_stack 

[Source](https://docs.rs/bevy_post_process/0.19.0/x86_64-unknown-linux-gnu/src/bevy_post_process/lib.rs.html#12)

Miscellaneous built-in postprocessing effects.

Includes:

*   Chromatic Aberration
*   Lens Distortion
*   Vignette

## Structs

[ChromaticAberration](struct.ChromaticAberration.html "struct bevy::post_process::effect_stack::ChromaticAberration")

Adds colored fringes to the edges of objects in the scene.

[ChromaticAberrationUniform](struct.ChromaticAberrationUniform.html "struct bevy::post_process::effect_stack::ChromaticAberrationUniform")

The on-GPU version of the [`ChromaticAberration`](struct.ChromaticAberration.html "struct bevy::post_process::effect_stack::ChromaticAberration") settings.

[EffectStackPlugin](struct.EffectStackPlugin.html "struct bevy::post_process::effect_stack::EffectStackPlugin")

A plugin that implements a built-in postprocessing stack with some common effects.

[LensDistortion](struct.LensDistortion.html "struct bevy::post_process::effect_stack::LensDistortion")

Simulates the warping of the image caused by real-world camera lenses.

[LensDistortionUniform](struct.LensDistortionUniform.html "struct bevy::post_process::effect_stack::LensDistortionUniform")

The on-GPU version of the [`LensDistortion`](struct.LensDistortion.html "struct bevy::post_process::effect_stack::LensDistortion") settings.

[PostProcessingPipeline](struct.PostProcessingPipeline.html "struct bevy::post_process::effect_stack::PostProcessingPipeline")

GPU pipeline data for the built-in postprocessing stack.

[PostProcessingPipelineId](struct.PostProcessingPipelineId.html "struct bevy::post_process::effect_stack::PostProcessingPipelineId")

A component attached to cameras in the render world that stores the specialized pipeline ID for the built-in postprocessing stack.

[PostProcessingPipelineKey](struct.PostProcessingPipelineKey.html "struct bevy::post_process::effect_stack::PostProcessingPipelineKey")

A key that uniquely identifies a built-in postprocessing pipeline.

[PostProcessingUniformBufferOffsets](struct.PostProcessingUniformBufferOffsets.html "struct bevy::post_process::effect_stack::PostProcessingUniformBufferOffsets")

A component, part of the render world, that stores the appropriate byte offset within the [`PostProcessingUniformBuffers`](struct.PostProcessingUniformBuffers.html "struct bevy::post_process::effect_stack::PostProcessingUniformBuffers") for the camera it’s attached to.

[PostProcessingUniformBuffers](struct.PostProcessingUniformBuffers.html "struct bevy::post_process::effect_stack::PostProcessingUniformBuffers")

A resource, part of the render world, that stores the uniform buffers for post-processing effects.

[Vignette](struct.Vignette.html "struct bevy::post_process::effect_stack::Vignette")

Adds a gradual shading effect to the edges of the screen, drawing focus towards the center.

[VignetteUniform](struct.VignetteUniform.html "struct bevy::post_process::effect_stack::VignetteUniform")

The on-GPU version of the [`Vignette`](struct.Vignette.html "struct bevy::post_process::effect_stack::Vignette") settings.