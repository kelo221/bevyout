[bevy](../../../index.html)::[pbr](../../index.html)::[decal](../index.html)::[clustered](index.html)

# Function extract\_decals 

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/decal/clustered.rs.html#237-271)

```rust
pub fn extract_decals(
    decals: Extract<'_, '_, Query<'_, '_, (RenderEntity, &ClusteredDecal, &GlobalTransform, &ViewVisibility)>>,
    spot_light_textures: Extract<'_, '_, Query<'_, '_, (RenderEntity, &SpotLightTexture, &GlobalTransform, &ViewVisibility)>>,
    point_light_textures: Extract<'_, '_, Query<'_, '_, (RenderEntity, &PointLightTexture, &GlobalTransform, &ViewVisibility)>>,
    directional_light_textures: Extract<'_, '_, Query<'_, '_, (RenderEntity, &DirectionalLightTexture, &GlobalTransform, &ViewVisibility)>>,
    render_decals: ResMut<'_, RenderClusteredDecals>,
)
```

Extracts decals from the main world into the render world.