[bevy](../../index.html)::[gltf](../index.html)::[vertex\_attributes](index.html)

# Function convert\_attribute 

[Source](https://docs.rs/bevy_gltf/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gltf/vertex_attributes.rs.html#280-286)

```rust
pub fn convert_attribute(
    semantic: Semantic,
    accessor: Accessor<'_>,
    buffer_data: &Vec<Vec<u8>>,
    custom_vertex_attributes: &HashMap<Box<str>, MeshVertexAttribute>,
    convert_coordinates: bool,
) -> Result<(MeshVertexAttribute, VertexAttributeValues), ConvertAttributeError>
```

map glTF vertex attributes into their `MeshVertexAttribute` forms, optionally converting values if necessary.