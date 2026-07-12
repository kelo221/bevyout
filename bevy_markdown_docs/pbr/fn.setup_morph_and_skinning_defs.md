[bevy](../index.html)::[pbr](index.html)

# Function setup\_morph\_and\_skinning\_defs 

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#3221-3229)

```rust
pub fn setup_morph_and_skinning_defs(
    mesh_layouts: &MeshLayouts,
    layout: &MeshVertexBufferLayoutRef,
    offset: u32,
    key: &MeshPipelineKey,
    shader_defs: &mut Vec<ShaderDefVal>,
    vertex_attributes: &mut Vec<VertexAttributeDescriptor>,
    skins_use_uniform_buffers: bool,
) -> BindGroupLayoutDescriptor
```