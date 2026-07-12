[bevy](../index.html)::[pbr](index.html)

# Function set\_mesh\_motion\_vector\_flags 

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#2366-2370)

```rust
pub fn set_mesh_motion_vector_flags(
    render_mesh_instances: ResMut<'_, RenderMeshInstances>,
    skin_uniforms: Res<'_, SkinUniforms>,
    morph_indices: Res<'_, MorphIndices>,
)
```

A system that sets the [`RenderMeshInstanceFlags`](struct.RenderMeshInstanceFlags.html "struct bevy::pbr::RenderMeshInstanceFlags") for each mesh based on whether the previous frame had skins and/or morph targets.

Ordinarily, [`RenderMeshInstanceFlags`](struct.RenderMeshInstanceFlags.html "struct bevy::pbr::RenderMeshInstanceFlags") are set during the extraction phase. However, we can’t do that for the flags related to skins and morph targets because the previous frame’s skin and morph targets are the responsibility of [`extract_skins`](fn.extract_skins.html "fn bevy::pbr::extract_skins") and [`extract_morphs`](fn.extract_morphs.html "fn bevy::pbr::extract_morphs") respectively. We want to run those systems in parallel with mesh extraction for performance, so we need to defer setting of these mesh instance flags to after extraction, which this system does. An alternative to having skin- and morph-target-related data in [`RenderMeshInstanceFlags`](struct.RenderMeshInstanceFlags.html "struct bevy::pbr::RenderMeshInstanceFlags") would be to have [`crate::material::queue_material_meshes`](fn.queue_material_meshes.html "fn bevy::pbr::queue_material_meshes") check the skin and morph target tables for each mesh, but that would be too slow in the hot mesh queuing loop.