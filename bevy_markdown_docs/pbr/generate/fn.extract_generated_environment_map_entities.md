[bevy](../../index.html)::[pbr](../index.html)::[generate](index.html)

# Function extract\_generated\_environment\_map\_entities 

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/light_probe/generate.rs.html#435-445)

```rust
pub fn extract_generated_environment_map_entities(
    query: Extract<'_, '_, Query<'_, '_, (RenderEntity, &GeneratedEnvironmentMapLight, &EnvironmentMapLight)>>,
    commands: Commands<'_, '_>,
    render_images: Res<'_, RenderAssets<GpuImage>>,
)
```