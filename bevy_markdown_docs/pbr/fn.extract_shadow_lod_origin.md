[bevy](../index.html)::[pbr](index.html)

# Function extract\_shadow\_lod\_origin 

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/light.rs.html#2924-2930)

```rust
pub fn extract_shadow_lod_origin(
    global_transform_query: Extract<'_, '_, Query<'_, '_, &GlobalTransform>>,
    camera_query: Extract<'_, '_, Query<'_, '_, (Entity, &RenderTarget), With<Camera>>>,
    shadow_lod_origin_query: Extract<'_, '_, Query<'_, '_, Entity, With<ShadowLodOrigin>>>,
    lights_query: Extract<'_, '_, Query<'_, '_, Entity, Or<(With<PointLight>, With<SpotLight>)>>>,
    render_shadow_lod_origin: ResMut<'_, RenderShadowLodOrigin>,
)
```

An extraction system that determines the origin for LOD computation for point and spot light shadow maps and updates the [`RenderShadowLodOrigin`](../render/view/struct.RenderShadowLodOrigin.html "struct bevy::render::view::RenderShadowLodOrigin") with the result.

See [`ShadowLodOrigin`](../camera/struct.ShadowLodOrigin.html "struct bevy::camera::ShadowLodOrigin") for more details on the algorithm that this system uses.