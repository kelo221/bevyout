[bevy](../../index.html)::[render](../index.html)

# Module occlusion\_culling 

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/lib.rs.html#53)

GPU occlusion culling.

See [`OcclusionCulling`](struct.OcclusionCulling.html "struct bevy::render::occlusion_culling::OcclusionCulling") for a detailed description of occlusion culling in Bevy.

## Structs

[OcclusionCulling](struct.OcclusionCulling.html "struct bevy::render::occlusion_culling::OcclusionCulling")

Add this component to a view in order to enable GPU occlusion culling.

[OcclusionCullingPlugin](struct.OcclusionCullingPlugin.html "struct bevy::render::occlusion_culling::OcclusionCullingPlugin")

Enables GPU occlusion culling.

[OcclusionCullingSubview](struct.OcclusionCullingSubview.html "struct bevy::render::occlusion_culling::OcclusionCullingSubview")

A render-world component that contains resources necessary to perform occlusion culling on any view other than a camera.

[OcclusionCullingSubviewEntities](struct.OcclusionCullingSubviewEntities.html "struct bevy::render::occlusion_culling::OcclusionCullingSubviewEntities")

A render-world component placed on each camera that stores references to all entities other than cameras that need occlusion culling.