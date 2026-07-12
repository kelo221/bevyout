[bevy](../../index.html)::[pbr](../index.html)

# Module environment\_map 

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/light_probe/mod.rs.html#44)

Environment maps and reflection probes.

An _environment map_ consists of a pair of diffuse and specular cubemaps that together reflect the static surrounding area of a region in space. When available, the PBR shader uses these to apply diffuse light and calculate specular reflections.

Environment maps come in two flavors, depending on what other components the entities they’re attached to have:

1.  If attached to a view, they represent the objects located a very far distance from the view, in a similar manner to a skybox. Essentially, these _view environment maps_ represent a higher-quality replacement for [`AmbientLight`](../../prelude/struct.AmbientLight.html "struct bevy::prelude::AmbientLight") for outdoor scenes. The indirect light from such environment maps are added to every point of the scene, including interior enclosed areas.
    
2.  If attached to a [`bevy_light::LightProbe`](../../prelude/struct.LightProbe.html "struct bevy::prelude::LightProbe"), environment maps represent the immediate surroundings of a specific location in the scene. These types of environment maps are known as _reflection probes_.
    

Typically, environment maps are static (i.e. “baked”, calculated ahead of time) and so only reflect fixed static geometry. The environment maps must be pre-filtered into a pair of cubemaps, one for the diffuse component and one for the specular component, according to the [split-sum approximation](https://cdn2.unrealengine.com/Resources/files/2013SiggraphPresentationsNotes-26915738.pdf). To pre-filter your environment map, you can use the [glTF IBL Sampler](https://github.com/KhronosGroup/glTF-IBL-Sampler) or its [artist-friendly UI](https://github.com/pcwalton/gltf-ibl-sampler-egui). The diffuse map uses the Lambertian distribution, while the specular map uses the GGX distribution.

The Khronos Group has [several pre-filtered environment maps](https://github.com/KhronosGroup/glTF-Sample-Environments) available for you to use.

Currently, reflection probes (i.e. environment maps attached to light probes) use binding arrays (also known as bindless textures) and consequently aren’t supported on WebGL2 or WebGPU. Reflection probes are also unsupported if GLSL is in use, due to `naga` limitations. Environment maps attached to views are, however, supported on all platforms.

## Structs

[EnvironmentMapIds](struct.EnvironmentMapIds.html "struct bevy::pbr::environment_map::EnvironmentMapIds")

Like [`EnvironmentMapLight`](../../prelude/struct.EnvironmentMapLight.html "struct bevy::prelude::EnvironmentMapLight"), but contains asset IDs instead of handles.

[EnvironmentMapViewLightProbeInfo](struct.EnvironmentMapViewLightProbeInfo.html "struct bevy::pbr::environment_map::EnvironmentMapViewLightProbeInfo")

Information about the environment map attached to the view, if any. This is a global environment map that lights everything visible in the view, as opposed to a light probe which affects only a specific area.