[bevy](../index.html)::[light](index.html)

# Function get\_shadow\_lod\_origin 

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/lib.rs.html#747-751)

```rust
pub fn get_shadow_lod_origin(
    camera_query: QueryLens<'_, (Entity, &RenderTarget), With<Camera>>,
    shadow_lod_origin_query: QueryLens<'_, Entity, With<ShadowLodOrigin>>,
    lights_query: QueryLens<'_, Entity, Or<(With<PointLight>, With<SpotLight>)>>,
) -> Option<Entity>
```

Determines the LOD origin for spot and point light shadow maps.

The selection priority is, from highest to lowest:

1.  An entity explicitly marked with the [`ShadowLodOrigin`](../camera/struct.ShadowLodOrigin.html "struct bevy::camera::ShadowLodOrigin") component.
    
2.  A camera that renders to a window.
    
3.  Any camera.